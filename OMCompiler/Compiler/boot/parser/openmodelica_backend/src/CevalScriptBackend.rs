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

use crate::AbsynToJulia;
use crate::BackendDAE;
use crate::BackendDAECreate;
use crate::BackendDAEOptimize;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::Binding;
use crate::CevalScript;
use crate::CodegenFMU;
use crate::Conversion;
use crate::DAEQuery;
use crate::FindZeroCrossings;
use crate::Interactive::Access;
use crate::Interactive;
use crate::InteractiveUtil;
use crate::LexerModelicaDiff;
use crate::NFApi;
use crate::Obfuscate;
use crate::Refactor;
use crate::ReverseLookup;
use crate::RewriteRules;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeFunctionUtil;
use crate::SimCodeMain;
use crate::SimCodeUtil;
use crate::SimpleModelicaParser;
use crate::StaticScript;
use crate::SymbolTable;
use crate::SymbolicJacobian;
use crate::TotalModelDebug;
use crate::Uncertainties;
use crate::XMLDump;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_dump_extra::AbsynJLDumpTpl;
use openmodelica_dump_extra::BlockCallRewrite;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::FInst;
use openmodelica_frontend::Figaro;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::NFSCodeEnv;
use openmodelica_frontend::NFSCodeFlatten;
use openmodelica_frontend::NFSCodeLookup;
use openmodelica_frontend::Parser;
use openmodelica_frontend::Types;
use openmodelica_frontend::UnitAbsyn;
use openmodelica_frontend::UnitAbsynBuilder;
use openmodelica_frontend::ValuesUtil;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_nf_frontend::NFConvertDAE;
use openmodelica_nf_frontend::NFFlatModel;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFlatten;
use openmodelica_nf_frontend::NFInst;
use openmodelica_script_util::PackageManagement;
use openmodelica_script_util::SimulationResults;
use openmodelica_script_util::UnitParserExt;
use openmodelica_susan::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::DiffAlgorithm;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::FMI;
use openmodelica_util::FMIExt;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Graph;
use openmodelica_util::Print;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::TaskGraphResults;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
thread_local! { static __simulationResultType_rtest_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("SimulationResult")).clone() }) }, varLst: list![Arc::new(DAE::Var { name: (literal!("resultFile")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("simulationOptions")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("messages")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub fn simulationResultType_rtest() -> Arc<DAE::Type> { __simulationResultType_rtest_TLS.with(|__t| __t.clone()) }

thread_local! { static __simulationResultType_full_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("SimulationResult")).clone() }) }, varLst: list![Arc::new(DAE::Var { name: (literal!("resultFile")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("simulationOptions")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("messages")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("timeFrontend")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("timeBackend")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("timeSimCode")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("timeTemplates")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("timeCompile")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("timeSimulation")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("timeTotal")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub fn simulationResultType_full() -> Arc<DAE::Type> { __simulationResultType_full_TLS.with(|__t| __t.clone()) }

thread_local! { static __simulationResultType_drModelica_TLS: Arc<DAE::Type> = Arc::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: Arc::new(Absyn::Path::IDENT { name: (literal!("SimulationResult")).clone() }) }, varLst: list![Arc::new(DAE::Var { name: (literal!("messages")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("flatteningTime")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None }), Arc::new(DAE::Var { name: (literal!("simulationTime")).clone(), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_REAL_DEFAULT().clone(), binding: Arc::new(openmodelica_frontend_types::DAE::Binding::UNBOUND), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false }); }
pub fn simulationResultType_drModelica() -> Arc<DAE::Type> { __simulationResultType_drModelica_TLS.with(|__t| __t.clone()) }

//these are in reversed order than above
pub static zeroAdditionalSimulationResultValues: std::sync::LazyLock<Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>> = std::sync::LazyLock::new(|| { list![(literal!("timeTotal"), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })), (literal!("timeSimulation"), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })), (literal!("timeCompile"), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })), (literal!("timeTemplates"), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })), (literal!("timeSimCode"), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })), (literal!("timeBackend"), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })), (literal!("timeFrontend"), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }))] });

thread_local! { static __defaultStartTime_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }); }
pub fn defaultStartTime() -> Arc<DAE::Exp> { __defaultStartTime_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultStopTime_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }); }
pub fn defaultStopTime() -> Arc<DAE::Exp> { __defaultStopTime_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultNumberOfIntervals_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::ICONST { integer: 500 }); }
pub fn defaultNumberOfIntervals() -> Arc<DAE::Exp> { __defaultNumberOfIntervals_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultStepSize_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.002_f64) }); }
pub fn defaultStepSize() -> Arc<DAE::Exp> { __defaultStepSize_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultTolerance_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1e-6_f64) }); }
pub fn defaultTolerance() -> Arc<DAE::Exp> { __defaultTolerance_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultMethod_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::SCONST { string: (literal!("dassl")).clone() }); }
pub fn defaultMethod() -> Arc<DAE::Exp> { __defaultMethod_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultFileNamePrefix_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }); }
pub fn defaultFileNamePrefix() -> Arc<DAE::Exp> { __defaultFileNamePrefix_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultOptions_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }); }
pub fn defaultOptions() -> Arc<DAE::Exp> { __defaultOptions_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultOutputFormat_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::SCONST { string: (literal!("mat")).clone() }); }
pub fn defaultOutputFormat() -> Arc<DAE::Exp> { __defaultOutputFormat_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultVariableFilter_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::SCONST { string: (literal!(".*")).clone() }); }
pub fn defaultVariableFilter() -> Arc<DAE::Exp> { __defaultVariableFilter_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultCflags_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }); }
pub fn defaultCflags() -> Arc<DAE::Exp> { __defaultCflags_TLS.with(|__t| __t.clone()) }

thread_local! { static __defaultSimflags_TLS: Arc<DAE::Exp> = Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }); }
pub fn defaultSimflags() -> Arc<DAE::Exp> { __defaultSimflags_TLS.with(|__t| __t.clone()) }

pub static defaultSimulationOptions: std::sync::LazyLock</* ? */> = std::sync::LazyLock::new(|| { InteractiveTypes::SIMULATION_OPTIONS(defaultStartTime().clone(), defaultStopTime().clone(), defaultNumberOfIntervals().clone(), defaultStepSize().clone(), defaultTolerance().clone(), defaultMethod().clone(), defaultFileNamePrefix().clone(), defaultOptions().clone(), defaultOutputFormat().clone(), defaultVariableFilter().clone(), defaultCflags().clone(), defaultSimflags().clone()).unwrap() });

pub static simulationOptionsNames: std::sync::LazyLock<Arc<metamodelica::List<ArcStr>>> = std::sync::LazyLock::new(|| { list![(literal!("startTime")).clone(), (literal!("stopTime")).clone(), (literal!("numberOfIntervals")).clone(), (literal!("tolerance")).clone(), (literal!("method")).clone(), (literal!("fileNamePrefix")).clone(), (literal!("options")).clone(), (literal!("outputFormat")).clone(), (literal!("variableFilter")).clone(), (literal!("cflags")).clone(), (literal!("simflags")).clone()] });

pub fn getSimulationResultType() -> Result<Arc<DAE::Type>> {
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    t = if (Testsuite::isRunning()?) {simulationResultType_rtest().clone()} else {simulationResultType_full().clone()};
    Ok(t)
}

pub fn getDrModelicaSimulationResultType() -> Result<Arc<DAE::Type>> {
    let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    t = if (Testsuite::isRunning()?) {simulationResultType_rtest().clone()} else {simulationResultType_drModelica().clone()};
    Ok(t)
}

pub fn createSimulationResult(mut resultFile: ArcStr, mut options: ArcStr, mut message: ArcStr, mut inAddResultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut fields: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut isTestType: bool = false;
    let mut notest: bool = false;
    resultValues = inAddResultValues.clone().reverse();
    notest = !(Testsuite::isRunning()?);
    fields = if (notest.clone()) {List::map(resultValues.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)))} else {metamodelica::nil()};
    vals = if (notest.clone()) {List::map(resultValues.clone(), std::sync::Arc::new(fnptr!(Util::tuple22, _)))} else {metamodelica::nil()};
    res = Arc::new(Values::Value::RECORD { record_: Arc::new(Absyn::Path::IDENT { name: (literal!("SimulationResult")).clone() }), orderd: cons(Arc::new(Values::Value::STRING { string: (resultFile.clone()).clone() }), cons(Arc::new(Values::Value::STRING { string: (options.clone()).clone() }), cons(Arc::new(Values::Value::STRING { string: (message.clone()).clone() }), vals.clone()))), comp: cons((literal!("resultFile")).clone(), cons((literal!("simulationOptions")).clone(), cons((literal!("messages")).clone(), fields.clone()))), index: -1 });
    Ok(res)
}

pub fn createSimulationResultFailure(mut message: ArcStr, mut options: ArcStr) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut fields: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    res = createSimulationResult((literal!("")).clone(), (options.clone()).clone(), (message.clone()).clone(), zeroAdditionalSimulationResultValues.clone())?;
    Ok(res)
}

fn buildCurrentSimulationResultExp() -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    let mut cref: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
    cref = ComponentReferenceBasics::makeCrefIdent((literal!("currentSimulationResult")).clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil());
    outExp = Expression::makeCrefExp(cref.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
    Ok(outExp)
}

fn cevalCurrentSimulationResultExp(mut inCache: FCore::Cache, mut env: FCore::Graph, mut inputFilename: ArcStr, mut msg: Absyn::Msg) -> Result<(FCore::Cache, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut filename: ArcStr = arcstr::literal!("");
    (outCache, filename) = (::match_deref::match_deref! { match &((inCache.clone(), env.clone(), inputFilename.clone(), msg.clone())) {
        (cache, _, Deref @ "<default>", _) => {
            let mut cache = (*cache).clone();
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(cache.clone(), env.clone(), buildCurrentSimulationResultExp()?, true, msg.clone(), 0)?) {
                (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cache = __pa0.clone();
            filename = __pa1.clone();
            (cache.clone(), filename.clone())
        },
        _ => {
            (inCache.clone(), inputFilename.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, filename))
}

fn getConst(mut inAbsynExp: Arc<Absyn::Exp>, mut inExpType: Arc<DAE::Type>) -> Result<Arc<DAE::Exp>> {
    let mut outExp: Arc<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inAbsynExp.clone(), inExpType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS, exp }, _) => {
                    let mut i: i32 = 0;
                    let __pa0 = ::match_deref::match_deref! { match &(getConst(exp.clone(), inExpType.clone())?) {
                        Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    i = __pa0.clone();
                    i = intNeg(i.clone());
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UMINUS, exp }, _) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let __pa0 = ::match_deref::match_deref! { match &(getConst(exp.clone(), inExpType.clone())?) {
                        Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r = __pa0.clone();
                    r = -(r.clone());
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { value: i }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(Arc::new(DAE::Exp::ICONST { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::REAL { value: r#str }, Deref @ DAE::Type::T_REAL { .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = stringReal((r#str.clone()).clone())?;
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::INTEGER { value: i }, Deref @ DAE::Type::T_REAL { .. }) => {
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    r = intReal(i.clone());
                    Ok(Arc::new(DAE::Exp::RCONST { real: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.getConst: experiment annotation contains unsupported expression: ")); __mm_s.push_str(&*Dump::printExpStr(inAbsynExp.clone())?); __mm_s.push_str(&*literal!(" of type ")); __mm_s.push_str(&*TypesDump::unparseType(inExpType.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone();
                    Error::addCompilerError((r#str.clone()).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outExp)
}

fn simOptionsAsString(mut vals: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = vals.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: lst } => {
                    let mut simOptsValues: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = r#str.clone();
                    simOptsValues = List::map(lst.clone(), (std::sync::Arc::new(ValuesDump::valString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    simOptsValues = List::map2(simOptsValues.clone(), (std::sync::Arc::new(System::stringReplace) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("\"")).clone(), (literal!("'")).clone());
                    r#str = (Util::buildMapStr(simulationOptionsNames.clone(), simOptsValues.clone(), (literal!(" = ")).clone(), (literal!(", ")).clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: lst } => {
                    let mut simOptsValues: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = r#str.clone();
                    simOptsValues = List::map(lst.clone(), (std::sync::Arc::new(ValuesDump::valString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    simOptsValues = List::map2(simOptsValues.clone(), (std::sync::Arc::new(System::stringReplace) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("\"")).clone(), (literal!("'")).clone());
                    r#str = stringDelimitList(simOptsValues.clone(), (literal!(", ")).clone());
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn diffSanityCheckEqual(mut s1: ArcStr, mut s2: ArcStr) -> Result<bool> {
    use crate::LexerModelicaDiff::Token;
    use crate::LexerModelicaDiff::blockCommentCanonical;
    use crate::LexerModelicaDiff::isBlockComment;
    use crate::LexerModelicaDiff::isLineComment;
    use crate::LexerModelicaDiff::modelicaDiffTokenWhitespace;
    use crate::LexerModelicaDiff::scanString;
    use crate::LexerModelicaDiff::tokenContent;
    let mut b: bool = false;
    let mut ts1: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut ts2: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut comments1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut comments2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (ts1, _) = scanString((s1.clone()).clone())?;
    (ts2, _) = scanString((s2.clone()).clone())?;
    if stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (ts1.clone()).into_iter().cloned() {
            if !(!(modelicaDiffTokenWhitespace(t.clone())?)) { continue; }
            let __x = tokenContent(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) != stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (ts2.clone()).into_iter().cloned() {
            if !(!(modelicaDiffTokenWhitespace(t.clone())?)) { continue; }
            let __x = tokenContent(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) {
        b = false;
        return Ok(b);
    }
    comments1 = List::sort({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (ts1.clone()).into_iter().cloned() {
            if !(isLineComment(t.clone())? || isBlockComment(t.clone())?) { continue; }
            let __x = diffSanityCheckCommentStr(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
    comments2 = List::sort({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (ts2.clone()).into_iter().cloned() {
            if !(isLineComment(t.clone())? || isBlockComment(t.clone())?) { continue; }
            let __x = diffSanityCheckCommentStr(t.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
    b = List::isEqualOnTrue(comments1.clone(), comments2.clone(), (std::sync::Arc::new(stringEq) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>));
    Ok(b)
}

fn diffSanityCheckCommentStr(mut t: LexerModelicaDiff::Token) -> Result<ArcStr> {
    use crate::LexerModelicaDiff::blockCommentCanonical;
    use crate::LexerModelicaDiff::isBlockComment;
    use crate::LexerModelicaDiff::tokenContent;
    let mut s: ArcStr = arcstr::literal!("");
    s = (if (isBlockComment(t.clone())?) {stringDelimitList(blockCommentCanonical(t.clone())?, (literal!("\n")).clone())} else {tokenContent(t.clone())?}).clone();
    Ok(s)
}

pub fn cevalInteractiveFunctions3(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFunctionName: ArcStr, mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>, mut msg: Absyn::Msg) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    use openmodelica_util::DiffAlgorithm::Diff;
    use openmodelica_util::DiffAlgorithm::diff;
    use openmodelica_util::DiffAlgorithm::printActual;
    use openmodelica_util::DiffAlgorithm::printDiffTerminalColor;
    use openmodelica_util::DiffAlgorithm::printDiffXml;
    use crate::LexerModelicaDiff::Token;
    use crate::LexerModelicaDiff::TokenId;
    use crate::LexerModelicaDiff::filterModelicaDiff;
    use crate::LexerModelicaDiff::modelicaDiffTokenEq;
    use crate::LexerModelicaDiff::modelicaDiffTokenWhitespace;
    use crate::LexerModelicaDiff::reportErrors;
    use crate::LexerModelicaDiff::scanString;
    use crate::LexerModelicaDiff::tokenContent;
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inFunctionName.clone(), inVals.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScriptParallel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: true }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut blst: Arc<metamodelica::List<bool>> = metamodelica::nil();
                    let mut forkedSymbolTable: Arc<SymbolTable::SymbolTable>;
                    strs = List::map(vals.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    forkedSymbolTable = SymbolTable::get();
                    blst = System::launchParallelTasks(i.clone(), List::map1(strs.clone(), std::sync::Arc::new(fnptr!(Util::makeTuple, _, _)), forkedSymbolTable.clone()), (std::sync::Arc::new(Interactive::evaluateFork) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<SymbolTable::SymbolTable>)) -> Result<bool> + 'static>))?;
                    v = ValuesMake::makeArray(List::map(blst.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeBoolean, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<Arc<Values::Value>> + 'static>)))?;
                    SymbolTable::update(forkedSymbolTable.clone());
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScriptParallel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut is: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = List::map(vals.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    strs = List::map1r(strs.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (stringAppend((Settings::getInstallationDirectoryPath()?).clone(), (literal!("/bin/omc ")).clone())).clone());
                    is = System::systemCallParallel(strs.clone(), i.clone());
                    Ok(ValuesMake::makeArray(List::map(List::map1(is.clone(), (std::sync::Arc::new(fnptr!(intEq, i32, i32)) as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>), 0), (std::sync::Arc::new(fnptr!(ValuesMake::makeBoolean, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScriptParallel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(ValuesMake::makeArray(List::fill(Arc::new(Values::Value::BOOL { boolean: false }), (vals.clone().len() as i32)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setClassComment", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::setClassComment(path.clone(), (r#str.clone()).clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isShortDefinition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = isShortDefinition(path.clone(), SymbolTable::getAbsyn())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getUsedClassNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    sp = SymbolTable::getSCode()?;
                    (sp, _) = NFSCodeFlatten::flattenClassInProgram(path.clone(), sp.clone())?;
                    sp = SCodeUtil::removeBuiltinsFromTopScope(sp.clone());
                    paths = Interactive::getSCodeClassNamesRecursive(sp.clone());
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getUsedClassNames", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassComment", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut elem: Arc<Absyn::Element>;
                    elem = InteractiveUtil::getPathedElementInProgram(path.clone(), SymbolTable::getAbsyn())?;
                    r#str = (System::unescapedString((getClassElementComment(elem.clone())).clone())).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassComment", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getPackages", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AllLoadedClasses" } } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    paths = Interactive::getTopPackages(SymbolTable::getAbsyn())?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getPackages", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    paths = Interactive::getPackagesInPath(path.clone(), SymbolTable::getAbsyn())?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "convertUnits", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut offset: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut offset1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut offset2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut scaleFactor: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut scaleFactor1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut scaleFactor2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut b: bool = false;
                    let mut u1: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    let mut u2: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    Error::clearMessages();
                    UnitParserExt::initSIUnits();
                    (u1, scaleFactor1, offset1) = UnitAbsynBuilder::str2unitWithScaleFactor((str1.clone()).clone(), None)?;
                    (u2, scaleFactor2, offset2) = UnitAbsynBuilder::str2unitWithScaleFactor((str2.clone()).clone(), None)?;
                    b = u1.clone() == u2.clone();
                    scaleFactor = realDiv(scaleFactor2.clone(), scaleFactor1.clone());
                    offset = realDiv((offset2.clone()) - (offset1.clone()), scaleFactor1.clone());
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: b.clone() }), Arc::new(Values::Value::REAL { real: scaleFactor.clone() }), Arc::new(Values::Value::REAL { real: offset.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "convertUnits", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(1.0_f64) }), Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedUnits", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut u1: UnitAbsyn::Unit = UnitAbsyn::Unit::UNSPECIFIED;
                    Error::clearMessages();
                    UnitParserExt::initSIUnits();
                    u1 = UnitAbsynBuilder::str2unit((str1.clone()).clone(), None)?;
                    strs = UnitAbsynBuilder::getDerivedUnits(u1.clone(), (str1.clone()).clone())?;
                    Ok(ValuesMake::makeArray(List::map(strs.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedUnits", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassInformation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(getClassInformation(className.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassInformation", _) => {
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::INTEGER { integer: 0 }), Arc::new(Values::Value::INTEGER { integer: 0 }), Arc::new(Values::Value::INTEGER { integer: 0 }), Arc::new(Values::Value::INTEGER { integer: 0 }), Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTransitions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(className.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTransitions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(getTransitions(className.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTransitions", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, .. } } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = Interactive::addTransition(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), cons(Arc::new(Absyn::NamedArg { argName: (literal!("annotate")).clone(), argValue: aexp.clone() }), metamodelica::nil()), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: eltargs } } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = Interactive::addTransitionWithAnnotation(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), Arc::new(Absyn::Annotation { elementArgs: eltargs.clone() }), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addTransition", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } } } } } } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Nil } } } } } } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil } } } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = Interactive::deleteTransition(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteTransition", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } } } } } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, .. } } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b5 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = Interactive::deleteTransition(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = Interactive::addTransition(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), (str4.clone()).clone(), b3.clone(), b4.clone(), b5.clone(), i1.clone(), cons(Arc::new(Absyn::NamedArg { argName: (literal!("annotate")).clone(), argValue: aexp.clone() }), metamodelica::nil()), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b5 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: eltargs } } }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = Interactive::deleteTransition(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), b.clone(), b1.clone(), b2.clone(), i.clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = Interactive::addTransitionWithAnnotation(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), (str4.clone()).clone(), b3.clone(), b4.clone(), b5.clone(), i1.clone(), Arc::new(Absyn::Annotation { elementArgs: eltargs.clone() }), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateTransition", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialStates", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(className.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialStates", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(getInitialStates(className.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialStates", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, .. } } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = addInitialState(classpath.clone(), (str1.clone()).clone(), cons(Arc::new(Absyn::NamedArg { argName: (literal!("annotate")).clone(), argValue: aexp.clone() }), metamodelica::nil()), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: eltargs } } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = addInitialStateWithAnnotation(classpath.clone(), (str1.clone()).clone(), Arc::new(Absyn::Annotation { elementArgs: eltargs.clone() }), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addInitialState", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = deleteInitialState(classpath.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteInitialState", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: _ } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, .. } } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    r#str = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = deleteInitialState(classpath.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = addInitialState(classpath.clone(), (str1.clone()).clone(), cons(Arc::new(Absyn::NamedArg { argName: (literal!("annotate")).clone(), argValue: aexp.clone() }), metamodelica::nil()), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: eltargs } } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut bval: bool = false;
                    (bval, p) = deleteInitialState(classpath.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn())?;
                    (bval, p) = addInitialStateWithAnnotation(classpath.clone(), (str1.clone()).clone(), Arc::new(Absyn::Annotation { elementArgs: eltargs.clone() }), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: bval.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateInitialState", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffModelicaFileListings", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { name: path, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut s4: ArcStr = arcstr::literal!("");
                    let mut s5: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut bom: ArcStr = arcstr::literal!("");
                    let mut sanityCheckFailed: bool = false;
                    let mut tokens1: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let mut tokens2: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let mut errorTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let mut parseTree1: Arc<metamodelica::List<Arc<SimpleModelicaParser::ParseTree>>> = metamodelica::nil();
                    let mut parseTree2: Arc<metamodelica::List<Arc<SimpleModelicaParser::ParseTree>>> = metamodelica::nil();
                    let mut treeDiffs: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Arc<SimpleModelicaParser::ParseTree>>>)>> = metamodelica::nil();
                    let mut s1 = (*s1).clone();
                    let mut s2 = (*s2).clone();
                    ExecStat::execStatReset()?;
                    (s1, bom) = StringUtil::stripBOM((s1.clone()).clone())?;
                    (tokens1, errorTokens) = scanString((s1.clone()).clone())?;
                    reportErrors(errorTokens.clone())?;
                    if false && s1.clone() != stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (tokens1.clone()).into_iter().cloned() {
                    let __x = tokenContent(t.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }) {
                        System::writeFile((literal!("string.before")).clone(), (s1.clone()).clone())?;
                        System::writeFile((literal!("string.after")).clone(), stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut t in (tokens1.clone()).into_iter().cloned() {
                    let __x = tokenContent(t.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
                        Error::assertion(false, (literal!("Lexed string does not match the original. See files string.before and string.after")).clone(), metamodelica::sourceInfo!())?;
                        bail!("fail");
                    }
                    ExecStat::execStat((literal!("diffModelicaFileListings scan string 1")).clone())?;
                    (_, parseTree1) = SimpleModelicaParser::stored_definition(tokens1.clone(), metamodelica::nil())?;
                    ExecStat::execStat((literal!("diffModelicaFileListings parse string 1")).clone())?;
                    if false && s1.clone() != SimpleModelicaParser::parseTreeStr(parseTree1.clone())? {
                        System::writeFile((literal!("string.before")).clone(), (s1.clone()).clone())?;
                        System::writeFile((literal!("string.after")).clone(), (SimpleModelicaParser::parseTreeStr(parseTree1.clone())?).clone())?;
                        Error::assertion(false, (literal!("Parsed string does not match the original. See files string.before and string.after")).clone(), metamodelica::sourceInfo!())?;
                        bail!("fail");
                    }
                    (s2, bom) = StringUtil::stripBOM((s2.clone()).clone())?;
                    (tokens2, errorTokens) = scanString((s2.clone()).clone())?;
                    reportErrors(errorTokens.clone())?;
                    ExecStat::execStat((literal!("diffModelicaFileListings scan string 2")).clone())?;
                    (_, parseTree2) = SimpleModelicaParser::stored_definition(tokens2.clone(), metamodelica::nil())?;
                    ExecStat::execStat((literal!("diffModelicaFileListings parse string 2")).clone())?;
                    if false && s2.clone() != SimpleModelicaParser::parseTreeStr(parseTree2.clone())? {
                        System::writeFile((literal!("string.before")).clone(), (s2.clone()).clone())?;
                        System::writeFile((literal!("string.after")).clone(), (SimpleModelicaParser::parseTreeStr(parseTree2.clone())?).clone())?;
                        Error::assertion(false, (literal!("Parsed string does not match the original. See files string.before and string.after")).clone(), metamodelica::sourceInfo!())?;
                        bail!("fail");
                    }
                    treeDiffs = SimpleModelicaParser::treeDiff(parseTree1.clone(), parseTree2.clone(), std::cmp::max((tokens1.clone().len() as i32), (tokens2.clone().len() as i32)))?;
                    ExecStat::execStat((literal!("treeDiff")).clone())?;
                    sanityCheckFailed = false;
                    if true {
                        s3 = (Dump::unparseStr(Parser::parsestring((s2.clone()).clone(), (literal!("<interactive>")).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?, false, Dump::defaultDumpOptions.clone())?).clone();
                        ExecStat::execStat((literal!("sanity parsestring(s2)")).clone())?;
                        s5 = (printActual(treeDiffs.clone(), (std::sync::Arc::new(SimpleModelicaParser::parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?).clone();
                        match '__try0: {
                            s4 = (unwrap_break_err!(Dump::unparseStr(Parser::parsestring((s5.clone()).clone(), (literal!("<interactive>")).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?, false, Dump::defaultDumpOptions.clone()), '__try0)).clone();
                            unwrap_break_err!(ExecStat::execStat((literal!("sanity parsestring(s5)")).clone()), '__try0);
                            Ok::<_, anyhow::Error>((s4.clone(),))
                        } {
                            Ok((__try0_o0,)) => {
                                        s4 = __try0_o0;
                            }
                            Err(_) => {
                                        System::writeFile((literal!("SanityCheckFail.mo")).clone(), (s5.clone()).clone())?;
                                        Error::addInternalError((literal!("Failed to parse merged string (see generated file SanityCheckFail.mo)\n")).clone(), metamodelica::sourceInfo!())?;
                                        bail!("fail");
                            }
                        }
                        if !(diffSanityCheckEqual((s3.clone()).clone(), (s4.clone()).clone())?) {
                            System::writeFile((literal!("SanityCheckFailBefore.mo")).clone(), (s3.clone()).clone())?;
                            System::writeFile((literal!("SanityCheckFailAfter.mo")).clone(), (s4.clone()).clone())?;
                            if b.clone() {
                                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("After merging the strings, the semantics changed for some reason (see generated files SanityCheckFailBefore.mo SanityCheckFailAfter.mo). Will return the empty string:\ns1:\n")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\ns2:\n")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\ns3:\n")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("\ns4:\n")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("\ns5:\n")); __mm_s.push_str(&*s5.clone()); __mm_s.push_str(&*literal!("\nparseTree2:")); __mm_s.push_str(&*SimpleModelicaParser::parseTreeStr(parseTree2.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                                        bail!("fail");
                            } else {
                                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("After merging the strings, the semantics changed for some reason (see generated files SanityCheckFailBefore.mo SanityCheckFailAfter.mo). Will return s2:\ns1:\n")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*literal!("\ns2:\n")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\ns3:\n")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("\ns4:\n")); __mm_s.push_str(&*s4.clone()); __mm_s.push_str(&*literal!("\ns5:\n")); __mm_s.push_str(&*s5.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                            }
                            sanityCheckFailed = true;
                        }
                    }
                    r#str = (if (sanityCheckFailed.clone()) {s2.clone()} else {'mc: {
        let __mc_input = AbsynUtil::pathLastIdent(path.clone())?;
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ "plain" => {
                            Ok(printActual(treeDiffs.clone(), (std::sync::Arc::new(SimpleModelicaParser::parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?)
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ "color" => {
                            Ok(printDiffTerminalColor(treeDiffs.clone(), (std::sync::Arc::new(SimpleModelicaParser::parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?)
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ "xml" => {
                            Ok(printDiffXml(treeDiffs.clone(), (std::sync::Arc::new(SimpleModelicaParser::parseTreeNodeStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SimpleModelicaParser::ParseTree>) -> Result<ArcStr> + 'static>))?)
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            Error::addInternalError((literal!("Unknown diffModelicaFileListings choice")).clone(), metamodelica::sourceInfo!())?;
                            Ok(bail!("fail"))
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }}).clone();
                    Ok(Arc::new(Values::Value::STRING { string: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*bom.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffModelicaFileListings", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exportToFigaro", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Nil } } } } } }) => {
                    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    sp = SymbolTable::getSCode()?;
                    Figaro::run(sp.clone(), path.clone(), (s1.clone()).clone(), (r#str.clone()).clone(), (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exportToFigaro", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "inferBindings", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    pnew = Binding::inferBindings(classpath.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(pnew.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "inferBindings", _) => {
                    println!("{}", (literal!("failed inferBindings\n")).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateVerificationScenarios", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    pnew = Binding::generateVerificationScenarios(classpath.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(pnew.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateVerificationScenarios", _) => {
                    println!("{}", (literal!("failed to generateVerificationScenarios\n")).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "rewriteBlockCall", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
                    let mut within_: Absyn::Within = Absyn::Within::TOP;
                    let mut outCache: FCore::Cache = outCache.clone();
                    p = SymbolTable::getAbsyn();
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    classes = list![absynClass.clone()];
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), p.clone(), false, false)?;
                    within_ = InteractiveUtil::buildWithin(classpath.clone())?;
                    pnew = BlockCallRewrite::rewriteBlockCall(Absyn::Program { classes: list![absynClass.clone()], within_: within_.clone() }, Absyn::Program { classes: classes.clone(), within_: within_.clone() })?;
                    pnew = InteractiveUtil::updateProgram(pnew.clone(), p.clone(), false)?;
                    SymbolTable::setAbsyn(pnew.clone())?;
                    outCache = FCore::emptyCache();
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "rewriteBlockCall", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "jacobian", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut filenameprefix: ArcStr = arcstr::literal!("");
                    let mut description: ArcStr = arcstr::literal!("");
                    let mut env: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut daelow: Arc<BackendDAE::BackendDAE>;
                    let mut vars: BackendDAE::Variables;
                    let mut eqnarr: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
                    let mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>;
                    let mut jac: Option<Arc<metamodelica::List<(i32, i32, Arc<BackendDAE::Equation>)>>> = None;
                    let mut syst: Arc<BackendDAE::EqSystem>;
                    let mut shared: Arc<BackendDAE::Shared>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let (__pa0, __pa1, Some(__pa2), _) = (runFrontEnd(outCache.clone(), inEnv.clone(), path.clone(), true, false, true)?) else { bail!("pattern mismatch") };
                    outCache = __pa0.clone();
                    env = __pa1.clone();
                    dae = __pa2.clone();
                    filenameprefix = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    description = (DAEUtil::daeDescription(dae.clone())).clone();
                    daelow = BackendDAECreate::lower(dae.clone(), outCache.clone(), env.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (filenameprefix.clone()).clone(), simSettingsOption: None })?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(BackendDAEUtil::preOptimizeBackendDAE(daelow.clone(), None)?) {
                        Deref @ DAE { UNIQUEIO: metamodelica::List::Cons { head: __pa3, tail: Deref @ metamodelica::List::Nil }, derivativeNamePrefix: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    syst = __pa3.clone();
                    shared = __pa4.clone();
                    (syst, m, _) = BackendDAEUtil::getAdjacencyMatrixfromOption(syst.clone(), crate::BackendDAE::IndexType::NORMAL, None, BackendDAEUtil::isInitializationDAE(shared.clone()))?;
                    vars = BackendVariable::daeVars(syst.clone());
                    eqnarr = BackendEquation::getEqnsFromEqSystem(syst.clone());
                    (jac, _) = SymbolicJacobian::calculateJacobian(vars.clone(), eqnarr.clone(), m.clone(), false, shared.clone())?;
                    res = (BackendDump::dumpJacobianStr(jac.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (res.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModel", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } }) => {
                    let mut b: bool = false;
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                    (b, outCache) = translateModel(outCache.clone(), inEnv.clone(), className.clone(), (filenameprefix.clone()).clone(), true, true, Some(simSettings.clone()))?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModel", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelEquationsUC", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: outputFile }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpExtractionSteps }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = Uncertainties::modelEquationsUC(outCache.clone(), inEnv.clone(), className.clone(), (outputFile.clone()).clone(), dumpExtractionSteps.clone())?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelEquationsUC", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("There were errors during extraction of uncertainty equations. Use getErrorString() to see them.")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModelFMU", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: _ } } } } }) => {
                    let mut b: bool = false;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (b, outCache, _) = translateModelFMU(outCache.clone(), inEnv.clone(), className.clone(), (str1.clone()).clone(), (str2.clone()).clone(), (filenameprefix.clone()).clone(), true, {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut vv in (cvars.clone()).into_iter().cloned() {
                    let __x = ValuesUtil::extractValueString(vv.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, None)?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModelFMU", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildModelFMU", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: _ } } } } }) => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = buildModelFMU(outCache.clone(), inEnv.clone(), className.clone(), (str1.clone()).clone(), (str2.clone()).clone(), (filenameprefix.clone()).clone(), true, {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut vv in (cvars.clone()).into_iter().cloned() {
                    let __x = ValuesUtil::extractValueString(vv.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, None)?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildModelFMU", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildEncryptedPackage", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b1: bool = false;
                    p = SymbolTable::getAbsyn();
                    b1 = buildEncryptedPackage(className.clone(), b.clone(), p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b1.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildEncryptedPackage", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateModelXML", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    filenameprefix = (Util::stringReplaceChar((filenameprefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
                    (outCache, ret_val) = translateModelXML(outCache.clone(), inEnv.clone(), className.clone(), (filenameprefix.clone()).clone(), true, None)?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exportDAEtoMatlab", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val, _) = getAdjacencyMatrix(outCache.clone(), inEnv.clone(), className.clone(), msg.clone(), (filenameprefix.clone()).clone())?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), true)?;
                    (outCache, ret_val) = checkModel(outCache.clone(), inEnv.clone(), className.clone(), msg.clone())?;
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), false)?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkAllModelsRecursive", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: showProtected }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = checkAllModelsRecursive(outCache.clone(), inEnv.clone(), className.clone(), showProtected.clone(), msg.clone())?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateGraphics", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(translateGraphics(className.clone(), msg.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLoadedLibraries", Deref @ metamodelica::List::Nil) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    p = SymbolTable::getAbsyn();
                    Ok(ValuesMake::makeArray(List::fold(p.classes.clone(), (std::sync::Arc::new(makeLoadLibrariesEntryAbsyn) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>, Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> + 'static>), metamodelica::nil()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "OpenModelica_uriToFilename", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut res: ArcStr = arcstr::literal!("");
                    res = uriToFilename((s1.clone()).clone());
                    if Flags::getConfigBool(Flags::BUILDING_FMU.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("The following path is a loaded resource... ")); __mm_s.push_str(&*res.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        bail!("fail");
                    }
                    Ok(Arc::new(Values::Value::STRING { string: (res.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "OpenModelica_uriToFilename", _) => {
                    if !((!(Flags::getConfigBool(Flags::BUILDING_MODEL.clone())?))) { bail!("guard") }
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationVersion", Deref @ metamodelica::List::Nil) => {
                    let mut res: ArcStr = arcstr::literal!("");
                    res = (Config::getAnnotationVersion()?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (res.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNoSimplify", Deref @ metamodelica::List::Nil) => {
                    let mut b: bool = false;
                    b = Config::getNoSimplify()?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setNoSimplify", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil }) => {
                    Config::setNoSimplify(b.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getShowAnnotations", Deref @ metamodelica::List::Nil) => {
                    let mut b: bool = false;
                    b = Config::showAnnotations()?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setShowAnnotations", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil }) => {
                    Config::setShowAnnotations(b.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getVectorizationLimit", Deref @ metamodelica::List::Nil) => {
                    let mut i: i32 = 0;
                    i = Config::vectorizationLimit()?;
                    Ok(Arc::new(Values::Value::INTEGER { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getOrderConnections", Deref @ metamodelica::List::Nil) => {
                    let mut b: bool = false;
                    b = Config::orderConnections()?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildModel", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut executable: ArcStr = arcstr::literal!("");
                    let mut initfilename: ArcStr = arcstr::literal!("");
                    let mut filenameprefix: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    List::map_0(ClockIndexes::buildModelClocks.clone(), (std::sync::Arc::new(System::realtimeClear) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>));
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    if !(Config::simCodeTarget()? == literal!("omsic")) {
                        (b, outCache, compileDir, executable, _, _, initfilename, _, _, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    } else {
                        filenameprefix = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                        match '__try0: {
                            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(buildModelFMU(outCache.clone(), inEnv.clone(), className.clone(), (literal!("2.0")).clone(), (literal!("me")).clone(), (literal!("<default>")).clone(), true, list![(literal!("static")).clone()], None), '__try0)) {
                                        (__pa1, Deref @ Values::Value::STRING { string: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                                        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                            } };
                            outCache = __pa1.clone();
                            r#str = __pa2.clone();
                            if stringEmpty((r#str.clone()).clone()) {
                                        break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
                            }
                            b = true;
                            Ok::<_, anyhow::Error>((b.clone(),))
                        } {
                            Ok((__try0_o0,)) => {
                                        b = __try0_o0;
                            }
                            Err(_) => {
                                        b = false;
                            }
                        }
                        compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
                        executable = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*filenameprefix.clone()); __mm_s.push_str(&*literal!("_me_FMU")); ArcStr::from(__mm_s) }).clone();
                        initfilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*filenameprefix.clone()); __mm_s.push_str(&*literal!("_init_xml")); ArcStr::from(__mm_s) }).clone();
                    }
                    executable = (if (!(Testsuite::isRunning()?)) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*compileDir.clone()); __mm_s.push_str(&*executable.clone()); ArcStr::from(__mm_s) }} else {executable.clone()}).clone();
                    Ok(ValuesMake::makeArray(if (b.clone()) {list![Arc::new(Values::Value::STRING { string: (executable.clone()).clone() }), Arc::new(Values::Value::STRING { string: (initfilename.clone()).clone() })]} else {list![Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })]})?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildModel", _) => {
                    Ok(ValuesMake::makeArray(list![Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })])?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "buildLabel", vals) => {
                    let mut executable: ArcStr = arcstr::literal!("");
                    let mut initfilename: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    FlagsUtil::setConfigBool(Flags::GENERATE_LABELED_SIMCODE.clone(), true)?;
                    List::map_0(ClockIndexes::buildModelClocks.clone(), (std::sync::Arc::new(System::realtimeClear) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>));
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    (b, outCache, _, executable, _, _, initfilename, _, _, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    Ok(ValuesMake::makeArray(if (b.clone()) {list![Arc::new(Values::Value::STRING { string: (executable.clone()).clone() }), Arc::new(Values::Value::STRING { string: (initfilename.clone()).clone() })]} else {list![Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })]})?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reduceTerms", vals) => {
                    let mut executable: ArcStr = arcstr::literal!("");
                    let mut initfilename: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    FlagsUtil::setConfigBool(Flags::REDUCE_TERMS.clone(), true)?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_LABELED_SIMCODE.clone(), false)?;
                    FlagsUtil::disableDebug(Flags::WRITE_TO_BUFFER.clone())?;
                    List::map_0(ClockIndexes::buildModelClocks.clone(), (std::sync::Arc::new(System::realtimeClear) as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<()> + 'static>));
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    if (vals.clone().len() as i32) != 13 {
                        Error::addInternalError((literal!("reduceTerms expected 13 arguments")).clone(), metamodelica::sourceInfo!())?;
                    }
                    let _ = (vals.clone()).get(13)?;
                    vals = listDelete(vals.clone(), 13)?;
                    (b, outCache, _, executable, _, _, initfilename, _, _, _, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    Ok(ValuesMake::makeArray(if (b.clone()) {list![Arc::new(Values::Value::STRING { string: (executable.clone()).clone() }), Arc::new(Values::Value::STRING { string: (initfilename.clone()).clone() })]} else {list![Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })]})?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "simulate", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut simflags: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut executable: ArcStr = arcstr::literal!("");
                    let mut outputFormat_str: ArcStr = arcstr::literal!("");
                    let mut executableSuffixedExe: ArcStr = arcstr::literal!("");
                    let mut sim_call: ArcStr = arcstr::literal!("");
                    let mut result_file: ArcStr = arcstr::literal!("");
                    let mut filenameprefix: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut exeDir: ArcStr = arcstr::literal!("");
                    let mut logFile: ArcStr = arcstr::literal!("");
                    let mut simValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut resI: i32 = 0;
                    let mut timeTotal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut timeSimulation: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut b: bool = false;
                    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    if Config::simCodeTarget()? == literal!("omsicpp") {
                        filenameprefix = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                        (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                        match '__try0: {
                            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(buildModelFMU(outCache.clone(), inEnv.clone(), className.clone(), (literal!("2.0")).clone(), (literal!("me")).clone(), (literal!("<default>")).clone(), true, list![(literal!("static")).clone()], Some(simSettings.clone())), '__try0)) {
                                        (__pa1, Deref @ Values::Value::STRING { string: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                                        _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                            } };
                            outCache = __pa1.clone();
                            r#str = __pa2.clone();
                            if stringEmpty((r#str.clone()).clone()) {
                                        break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
                            }
                            b = true;
                            Ok::<_, anyhow::Error>((b.clone(),))
                        } {
                            Ok((__try0_o0,)) => {
                                        b = __try0_o0;
                            }
                            Err(_) => {
                                        b = false;
                            }
                        }
                        compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
                        executable = (filenameprefix.clone()).clone();
                        simflags = (literal!("")).clone();
                        resultValues = metamodelica::nil();
                    } else if !(Config::simCodeTarget()? == literal!("omsic")) {
                        (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    } else {
                        Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![(literal!("Can't simulate for SimCodeTarget=omsic!\n")).clone()])?;
                        bail!("fail");
                    }
                    if b.clone() {
                        exeDir = (compileDir.clone()).clone();
                        (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                        let SimCode::SIMULATION_SETTINGS { outputFormat: __pa3, .. } = (simSettings.clone()) else { bail!("pattern mismatch") };
                        outputFormat_str = __pa3.clone();
                        result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), (compileDir.clone()).clone(), list![(executable.clone()).clone(), (literal!("_res.")).clone(), (outputFormat_str.clone()).clone()]));
                        result_file = (selectResultFile((result_file.clone()).clone(), (simflags.clone()).clone())?).clone();
                        executableSuffixedExe = (stringAppend((executable.clone()).clone(), (getSimulationExtension((Config::simCodeTarget()?).clone(), (arcstr::literal!(Autoconf::platform)).clone())).clone())).clone();
                        logFile = (stringAppend((executable.clone()).clone(), (literal!(".log")).clone())).clone();
                        if System::regularFileExists((logFile.clone()).clone()) {
                            let 0 = (System::removeFile((logFile.clone()).clone())) else { bail!("pattern mismatch") };
                        }
                        sim_call = stringAppendList(list![(literal!("\"")).clone(), (exeDir.clone()).clone(), (executableSuffixedExe.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (simflags.clone()).clone()]);
                        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                        SimulationResults::close();
                        resI = System::systemCallRestrictedEnv((sim_call.clone()).clone(), (logFile.clone()).clone())?;
                        timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                    } else {
                        result_file = (literal!("")).clone();
                        resI = 1;
                        timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                    }
                    timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    (outCache, simValue) = createSimulationResultFromcallModelExecutable(b.clone(), resI.clone(), timeTotal.clone(), timeSimulation.clone(), resultValues.clone(), outCache.clone(), className.clone(), vals.clone(), (result_file.clone()).clone(), (logFile.clone()).clone())?;
                    Ok(simValue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "simulate", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    Settings::getInstallationDirectoryPath()?;
                    r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to build model: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Ok(createSimulationResultFailure((res.clone()).clone(), (simOptionsAsString(vals.clone())?).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "simulate", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Ok(createSimulationResultFailure(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Simulation failed for model: ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\nEnvironment variable OPENMODELICAHOME not set.")); ArcStr::from(__mm_s) }).clone(), (simOptionsAsString(vals.clone())?).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: direction }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = moveClass(className.clone(), direction.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClass", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToTop", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = moveClassToTop(className.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToTop", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToBottom", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = moveClassToBottom(className.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moveClassToBottom", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "copyClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    p = SymbolTable::getAbsyn();
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), p.clone(), false, false)?;
                    p = copyClass(absynClass.clone(), (name.clone()).clone(), InteractiveUtil::parseWithinPath(path.clone()), classpath.clone(), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "copyClass", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "linearize", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut errMsg: ArcStr = arcstr::literal!("");
                    let false = (Interactive::existClass(className.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
                    errMsg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Linearization Failed. Model: ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" does not exist! Please load it first before linearization.")); ArcStr::from(__mm_s) }).clone();
                    Ok(createSimulationResultFailure((errMsg.clone()).clone(), (simOptionsAsString(vals.clone())?).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "linearize", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut simflags: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut executable: ArcStr = arcstr::literal!("");
                    let mut outputFormat_str: ArcStr = arcstr::literal!("");
                    let mut executableSuffixedExe: ArcStr = arcstr::literal!("");
                    let mut sim_call: ArcStr = arcstr::literal!("");
                    let mut result_file: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut logFile: ArcStr = arcstr::literal!("");
                    let mut strlinearizeTime: ArcStr = arcstr::literal!("");
                    let mut simValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut timeTotal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut timeSimulation: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut linearizeTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut b: bool = false;
                    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    r#str = (Flags::getConfigString(Flags::LINEARIZATION_DUMP_LANGUAGE.clone())?).clone();
                    if stringEq((r#str.clone()).clone(), (literal!("none")).clone())? {
                        FlagsUtil::setConfigString(Flags::LINEARIZATION_DUMP_LANGUAGE.clone(), (literal!("modelica")).clone())?;
                    }
                    (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    if b.clone() {
                        let __pa0 = ::match_deref::match_deref! { match &(getListNthShowError(vals.clone(), (literal!("try to get stop time")).clone(), 0, 2)?) {
                            Deref @ Values::Value::REAL { real: __pa0 } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        linearizeTime = __pa0.clone();
                        executableSuffixedExe = (stringAppend((executable.clone()).clone(), (getSimulationExtension((Config::simCodeTarget()?).clone(), (arcstr::literal!(Autoconf::platform)).clone())).clone())).clone();
                        logFile = (stringAppend((executable.clone()).clone(), (literal!(".log")).clone())).clone();
                        if System::regularFileExists((logFile.clone()).clone()) {
                            let 0 = (System::removeFile((logFile.clone()).clone())) else { bail!("pattern mismatch") };
                        }
                        strlinearizeTime = (realString(linearizeTime.clone())).clone();
                        sim_call = stringAppendList(list![(literal!("\"")).clone(), (compileDir.clone()).clone(), (executableSuffixedExe.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (literal!("-l=")).clone(), (strlinearizeTime.clone()).clone(), (literal!(" ")).clone(), (simflags.clone()).clone()]);
                        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                        SimulationResults::close();
                        if 0 == System::systemCallRestrictedEnv((sim_call.clone()).clone(), (logFile.clone()).clone())? {
                            result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), (compileDir.clone()).clone(), list![(executable.clone()).clone(), (literal!("_res.")).clone(), (outputFormat_str.clone()).clone()]));
                            timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                            timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                            simValue = createSimulationResult((result_file.clone()).clone(), (simOptionsAsString(vals.clone())?).clone(), (System::readFile((logFile.clone()).clone())?).clone(), cons((literal!("timeTotal"), Arc::new(Values::Value::REAL { real: timeTotal.clone() })), cons((literal!("timeSimulation"), Arc::new(Values::Value::REAL { real: timeSimulation.clone() })), resultValues.clone())))?;
                            SymbolTable::addVar(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("currentSimulationResult")).clone(), identType: DAE::T_STRING_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), Arc::new(Values::Value::STRING { string: (result_file.clone()).clone() }), FGraph::empty())?;
                        } else {
                            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Succeeding building the linearized executable, but failed to run the linearize command: ")); __mm_s.push_str(&*sim_call.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logFile.clone()).clone())?); ArcStr::from(__mm_s) }).clone();
                            simValue = createSimulationResultFailure((res.clone()).clone(), (simOptionsAsString(vals.clone())?).clone())?;
                        }
                    } else {
                        timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                        timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                        simValue = createSimulationResult((literal!("")).clone(), (simOptionsAsString(vals.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the linearize command: ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), cons((literal!("timeTotal"), Arc::new(Values::Value::REAL { real: timeTotal.clone() })), cons((literal!("timeSimulation"), Arc::new(Values::Value::REAL { real: timeSimulation.clone() })), resultValues.clone())))?;
                    }
                    Ok(simValue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "linearize", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the linearize command: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Ok(createSimulationResultFailure((res.clone()).clone(), (simOptionsAsString(vals.clone())?).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "optimize", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut simflags: ArcStr = arcstr::literal!("");
                    let mut executable: ArcStr = arcstr::literal!("");
                    let mut outputFormat_str: ArcStr = arcstr::literal!("");
                    let mut executableSuffixedExe: ArcStr = arcstr::literal!("");
                    let mut sim_call: ArcStr = arcstr::literal!("");
                    let mut result_file: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut exeDir: ArcStr = arcstr::literal!("");
                    let mut logFile: ArcStr = arcstr::literal!("");
                    let mut simValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut resI: i32 = 0;
                    let mut timeTotal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut timeSimulation: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut b: bool = false;
                    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(), true)?;
                    FlagsUtil::setConfigEnum(Flags::GRAMMAR.clone(), Flags::OPTIMICA.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone(), true)?;
                    (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    if b.clone() {
                        exeDir = (compileDir.clone()).clone();
                        (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                        let SimCode::SIMULATION_SETTINGS { outputFormat: __pa0, .. } = (simSettings.clone()) else { bail!("pattern mismatch") };
                        outputFormat_str = __pa0.clone();
                        result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), (compileDir.clone()).clone(), list![(executable.clone()).clone(), (literal!("_res.")).clone(), (outputFormat_str.clone()).clone()]));
                        executableSuffixedExe = (stringAppend((executable.clone()).clone(), (getSimulationExtension((Config::simCodeTarget()?).clone(), (arcstr::literal!(Autoconf::platform)).clone())).clone())).clone();
                        logFile = (stringAppend((executable.clone()).clone(), (literal!(".log")).clone())).clone();
                        if System::regularFileExists((logFile.clone()).clone()) {
                            let 0 = (System::removeFile((logFile.clone()).clone())) else { bail!("pattern mismatch") };
                        }
                        sim_call = stringAppendList(list![(literal!("\"")).clone(), (exeDir.clone()).clone(), (executableSuffixedExe.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (simflags.clone()).clone()]);
                        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                        SimulationResults::close();
                        resI = System::systemCallRestrictedEnv((sim_call.clone()).clone(), (logFile.clone()).clone())?;
                        timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                    } else {
                        result_file = (literal!("")).clone();
                        timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                        resI = 1;
                    }
                    timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    (outCache, simValue) = createSimulationResultFromcallModelExecutable(b.clone(), resI.clone(), timeTotal.clone(), timeSimulation.clone(), resultValues.clone(), outCache.clone(), className.clone(), vals.clone(), (result_file.clone()).clone(), (logFile.clone()).clone())?;
                    Ok(simValue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "optimize", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the optimize command: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Ok(createSimulationResultFailure((res.clone()).clone(), (simOptionsAsString(vals.clone())?).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "instantiateModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = instantiateModel(outCache.clone(), inEnv.clone(), className.clone())?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moo", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut simflags: ArcStr = arcstr::literal!("");
                    let mut executable: ArcStr = arcstr::literal!("");
                    let mut outputFormat_str: ArcStr = arcstr::literal!("");
                    let mut executableSuffixedExe: ArcStr = arcstr::literal!("");
                    let mut sim_call: ArcStr = arcstr::literal!("");
                    let mut result_file: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut exeDir: ArcStr = arcstr::literal!("");
                    let mut logFile: ArcStr = arcstr::literal!("");
                    let mut simValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut resI: i32 = 0;
                    let mut timeTotal: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut timeSimulation: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut b: bool = false;
                    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut vals = (*vals).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_SYMBOLIC_LINEARIZATION.clone(), true)?;
                    FlagsUtil::setConfigEnum(Flags::GRAMMAR.clone(), Flags::OPTIMICA.clone())?;
                    FlagsUtil::setConfigBool(Flags::GENERATE_DYN_OPTIMIZATION_PROBLEM.clone(), true)?;
                    (b, outCache, compileDir, executable, _, outputFormat_str, _, simflags, resultValues, vals, _) = buildModel(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    simflags = (stringAppend((simflags.clone()).clone(), (literal!(" -moo")).clone())).clone();
                    if b.clone() {
                        exeDir = (compileDir.clone()).clone();
                        (outCache, simSettings) = calculateSimulationSettings(outCache.clone(), vals.clone())?;
                        let SimCode::SIMULATION_SETTINGS { outputFormat: __pa0, .. } = (simSettings.clone()) else { bail!("pattern mismatch") };
                        outputFormat_str = __pa0.clone();
                        result_file = stringAppendList(List::consOnTrue(!(Testsuite::isRunning()?), (compileDir.clone()).clone(), list![(executable.clone()).clone(), (literal!("_res.")).clone(), (outputFormat_str.clone()).clone()]));
                        executableSuffixedExe = (stringAppend((executable.clone()).clone(), (getSimulationExtension((Config::simCodeTarget()?).clone(), (arcstr::literal!(Autoconf::platform)).clone())).clone())).clone();
                        logFile = (stringAppend((executable.clone()).clone(), (literal!(".log")).clone())).clone();
                        if System::regularFileExists((logFile.clone()).clone()) {
                            let 0 = (System::removeFile((logFile.clone()).clone())) else { bail!("pattern mismatch") };
                        }
                        sim_call = stringAppendList(list![(literal!("\"")).clone(), (exeDir.clone()).clone(), (executableSuffixedExe.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (simflags.clone()).clone()]);
                        System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                        SimulationResults::close();
                        resI = System::systemCallRestrictedEnv((sim_call.clone()).clone(), (logFile.clone()).clone())?;
                        timeSimulation = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_SIMULATION.clone())?;
                    } else {
                        result_file = (literal!("")).clone();
                        timeSimulation = metamodelica::OrderedFloat(0.0_f64);
                        resI = 1;
                    }
                    timeTotal = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
                    (outCache, simValue) = createSimulationResultFromcallModelExecutable(b.clone(), resI.clone(), timeTotal.clone(), timeSimulation.clone(), resultValues.clone(), outCache.clone(), className.clone(), vals.clone(), (result_file.clone()).clone(), (logFile.clone()).clone())?;
                    Ok(simValue.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "moo", vals @ Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to run the moo command: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Ok(createSimulationResultFailure((res.clone()).clone(), (simOptionsAsString(vals.clone())?).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMU", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: fmiLogLevel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: inputConnectors }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: outputConnectors }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } } } } } } } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut filename_1: ArcStr = arcstr::literal!("");
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut outputFile: ArcStr = arcstr::literal!("");
                    let mut fmiContext: Option<i32> = None;
                    let mut fmiInstance: Option<i32> = None;
                    let mut fmiModelVariablesInstance: Option<i32> = None;
                    let mut fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>> = metamodelica::nil();
                    let mut fmiModelVariablesList: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
                    let mut fmiExperimentAnnotation: FMI::ExperimentAnnotation;
                    let mut fmiInfo: FMI::Info;
                    let mut b: bool = false;
                    let mut workdir = (*workdir).clone();
                    Error::clearMessages();
                    let true = (System::regularFileExists((filename.clone()).clone())) else { bail!("pattern mismatch") };
                    workdir = (if (System::directoryExists((workdir.clone()).clone())) {workdir.clone()} else {System::pwd()}).clone();
                    (b, fmiContext, fmiInstance, fmiInfo, fmiTypeDefinitionsList, fmiExperimentAnnotation, fmiModelVariablesInstance, fmiModelVariablesList) = FMIExt::initializeFMIImport((filename.clone()).clone(), (workdir.clone()).clone(), fmiLogLevel.clone(), inputConnectors.clone(), outputConnectors.clone(), false)?;
                    let true = (b.clone()) else { bail!("pattern mismatch") };
                    fmiTypeDefinitionsList = fmiTypeDefinitionsList.clone().reverse();
                    fmiModelVariablesList = fmiModelVariablesList.clone().reverse();
                    s1 = (System::tolower((arcstr::literal!(Autoconf::platform)).clone())).clone();
                    name = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    name = (if (stringEq((name.clone()).clone(), (literal!("Default")).clone())? || stringEq((name.clone()).clone(), (literal!("default")).clone())?) {literal!("")} else {name.clone()}).clone();
                    r#str = (Tpl::tplString2((std::sync::Arc::new(CodegenFMU::importFMUModelica) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, FMI::FmiImport, ArcStr) -> Result<Tpl::Text> + 'static>), FMI::FmiImport { platform: (s1.clone()).clone(), fmuFileName: (filename.clone()).clone(), fmuWorkingDirectory: (workdir.clone()).clone(), fmiLogLevel: fmiLogLevel.clone(), fmiDebugOutput: b2.clone(), fmiContext: fmiContext.clone(), fmiInstance: fmiInstance.clone(), fmiInfo: fmiInfo.clone(), fmiTypeDefinitionsList: fmiTypeDefinitionsList.clone(), fmiExperimentAnnotation: fmiExperimentAnnotation.clone(), fmiModelVariablesInstance: fmiModelVariablesInstance.clone(), fmiModelVariablesList: fmiModelVariablesList.clone(), generateInputConnectors: inputConnectors.clone(), generateOutputConnectors: outputConnectors.clone() }, (name.clone()).clone())?).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    str1 = (FMI::getFMIModelIdentifier(fmiInfo.clone())?).clone();
                    str2 = (FMI::getFMIType(fmiInfo.clone())?).clone();
                    str3 = (FMI::getFMIVersion(fmiInfo.clone())?).clone();
                    outputFile = if (stringEmpty((name.clone()).clone())) {stringAppendList(list![(str1.clone()).clone(), (literal!("_")).clone(), (str2.clone()).clone(), (literal!("_FMU.mo")).clone()])} else {stringAppendList(list![(name.clone()).clone(), (literal!(".mo")).clone()])};
                    filename_1 = (if (b1.clone()) {stringAppendList(list![(workdir.clone()).clone(), (pd.clone()).clone(), (outputFile.clone()).clone()])} else {outputFile.clone()}).clone();
                    System::writeFile(stringAppendList(list![(workdir.clone()).clone(), (pd.clone()).clone(), (outputFile.clone()).clone()]), (r#str.clone()).clone())?;
                    FMIExt::releaseFMIImport(fmiModelVariablesInstance.clone(), fmiInstance.clone(), fmiContext.clone(), (str3.clone()).clone())?;
                    Ok(Arc::new(Values::Value::STRING { string: (filename_1.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMU", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: _ }, tail: Deref @ metamodelica::List::Nil } } } } } } } }) => {
                    let false = (System::regularFileExists((filename.clone()).clone())) else { bail!("pattern mismatch") };
                    Error::clearMessages();
                    Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![(filename.clone()).clone()])?;
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMU", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: _ }, tail: Deref @ metamodelica::List::Nil } } } } } } } }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMUModelDescription", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: fmiLogLevel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: inputConnectors }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: outputConnectors }, tail: Deref @ metamodelica::List::Nil } } } } } } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut filename_1: ArcStr = arcstr::literal!("");
                    let mut outputFile: ArcStr = arcstr::literal!("");
                    let mut modeldescriptionfilename: ArcStr = arcstr::literal!("");
                    let mut tmpDir: ArcStr = arcstr::literal!("");
                    let mut tmpFile: ArcStr = arcstr::literal!("");
                    let mut fmiContext: Option<i32> = None;
                    let mut fmiInstance: Option<i32> = None;
                    let mut fmiModelVariablesInstance: Option<i32> = None;
                    let mut fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>> = metamodelica::nil();
                    let mut fmiModelVariablesList: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
                    let mut fmiExperimentAnnotation: FMI::ExperimentAnnotation;
                    let mut fmiInfo: FMI::Info;
                    let mut b: bool = false;
                    let mut workdir = (*workdir).clone();
                    Error::clearMessages();
                    let true = (System::regularFileExists((filename.clone()).clone())) else { bail!("pattern mismatch") };
                    workdir = (if (System::directoryExists((workdir.clone()).clone())) {workdir.clone()} else {System::pwd()}).clone();
                    tmpDir = (System::createTemporaryDirectory(({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getTempDirectoryPath()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*literal!("fmuTmp")); __mm_s.push_str(&*intString(System::intRand(1000))); ArcStr::from(__mm_s) }).clone())?).clone();
                    tmpFile = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpDir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*literal!("modelDescription.xml")); ArcStr::from(__mm_s) }).clone();
                    System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cp -f ")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*tmpFile.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                    modeldescriptionfilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*tmpDir.clone()); __mm_s.push_str(&*literal!("/modelDescription.fmu")); ArcStr::from(__mm_s) }).clone();
                    System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("zip -j ")); __mm_s.push_str(&*modeldescriptionfilename.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*tmpFile.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                    let true = (System::regularFileExists((modeldescriptionfilename.clone()).clone())) else { bail!("pattern mismatch") };
                    (b, fmiContext, fmiInstance, fmiInfo, fmiTypeDefinitionsList, fmiExperimentAnnotation, fmiModelVariablesInstance, fmiModelVariablesList) = FMIExt::initializeFMIImport((modeldescriptionfilename.clone()).clone(), (tmpDir.clone()).clone(), fmiLogLevel.clone(), inputConnectors.clone(), outputConnectors.clone(), true)?;
                    let true = (b.clone()) else { bail!("pattern mismatch") };
                    fmiTypeDefinitionsList = fmiTypeDefinitionsList.clone().reverse();
                    fmiModelVariablesList = fmiModelVariablesList.clone().reverse();
                    s1 = (System::tolower((arcstr::literal!(Autoconf::platform)).clone())).clone();
                    r#str = (Tpl::tplString((std::sync::Arc::new(CodegenFMU::importFMUModelDescription) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, FMI::FmiImport) -> Result<Tpl::Text> + 'static>), FMI::FmiImport { platform: (s1.clone()).clone(), fmuFileName: (modeldescriptionfilename.clone()).clone(), fmuWorkingDirectory: (workdir.clone()).clone(), fmiLogLevel: fmiLogLevel.clone(), fmiDebugOutput: b2.clone(), fmiContext: fmiContext.clone(), fmiInstance: fmiInstance.clone(), fmiInfo: fmiInfo.clone(), fmiTypeDefinitionsList: fmiTypeDefinitionsList.clone(), fmiExperimentAnnotation: fmiExperimentAnnotation.clone(), fmiModelVariablesInstance: fmiModelVariablesInstance.clone(), fmiModelVariablesList: fmiModelVariablesList.clone(), generateInputConnectors: inputConnectors.clone(), generateOutputConnectors: outputConnectors.clone() })?).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    str1 = (FMI::getFMIModelIdentifier(fmiInfo.clone())?).clone();
                    str3 = (FMI::getFMIVersion(fmiInfo.clone())?).clone();
                    outputFile = stringAppendList(list![(workdir.clone()).clone(), (pd.clone()).clone(), (str1.clone()).clone(), (literal!("_Input_Output_FMU.mo")).clone()]);
                    filename_1 = if (b1.clone()) {stringAppendList(list![(workdir.clone()).clone(), (pd.clone()).clone(), (str1.clone()).clone(), (literal!("_Input_Output_FMU.mo")).clone()])} else {stringAppendList(list![(str1.clone()).clone(), (literal!("_Input_Output_FMU.mo")).clone()])};
                    System::writeFile((outputFile.clone()).clone(), (r#str.clone()).clone())?;
                    FMIExt::releaseFMIImport(fmiModelVariablesInstance.clone(), fmiInstance.clone(), fmiContext.clone(), (str3.clone()).clone())?;
                    System::removeDirectory((tmpDir.clone()).clone());
                    Ok(Arc::new(Values::Value::STRING { string: (filename_1.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMUModelDescription", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Nil } } } } } } }) => {
                    if !(System::regularFileExists((filename.clone()).clone())) {
                        Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![(filename.clone()).clone()])?;
                    }
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "importFMUModelDescription", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Nil } } } } } } }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getIndexReductionMethod", _) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Config::getIndexReductionMethod()?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableIndexReductionMethods", _) => {
                    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut strs1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(Flags::INDEX_REDUCTION_METHOD.clone())?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut ret_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, ret_val) = cevalInteractiveFunctions4(inCache.clone(), inEnv.clone(), (inFunctionName.clone()).clone(), inVals.clone(), msg.clone())?;
                    Ok(ret_val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn cevalInteractiveFunctions4(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFunctionName: ArcStr, mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>, mut msg: Absyn::Msg) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    use openmodelica_util::DiffAlgorithm::Diff;
    use openmodelica_util::DiffAlgorithm::diff;
    use openmodelica_util::DiffAlgorithm::printActual;
    use openmodelica_util::DiffAlgorithm::printDiffTerminalColor;
    use openmodelica_util::DiffAlgorithm::printDiffXml;
    use crate::LexerModelicaDiff::Token;
    use crate::LexerModelicaDiff::TokenId;
    use crate::LexerModelicaDiff::filterModelicaDiff;
    use crate::LexerModelicaDiff::modelicaDiffTokenEq;
    use crate::LexerModelicaDiff::modelicaDiffTokenWhitespace;
    use crate::LexerModelicaDiff::reportErrors;
    use crate::LexerModelicaDiff::scanString;
    use crate::LexerModelicaDiff::tokenContent;
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inFunctionName.clone(), inVals.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableIndexReductionMethods", _) => {
                    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut strs1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(Flags::INDEX_REDUCTION_METHOD.clone())?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMatchingAlgorithm", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Config::getMatchingAlgorithm()?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableMatchingAlgorithms", _) => {
                    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut strs1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(Flags::MATCHING_ALGORITHM.clone())?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTearingMethod", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Config::getTearingMethod()?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableTearingMethods", _) => {
                    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut v2: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut strs1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    (strs1, strs2) = FlagsUtil::getConfigOptionsStringList(Flags::TEARING_METHOD.clone())?;
                    v1 = ValuesMake::makeArray(List::map(strs1.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    v2 = ValuesMake::makeArray(List::map(strs2.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![v1.clone(), v2.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut access: Access = Access::hide;
                    let mut b: bool = false;
                    b = false;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::all.clone() {
                        absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?;
                        r#str = (Dump::unparseStr(Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, true, Dump::defaultDumpOptions.clone())?).clone();
                        if '__try0: {
                            unwrap_break_err!(System::writeFile((filename.clone()).clone(), (r#str.clone()).clone()), '__try0);
                            b = true;
                            Ok::<(), anyhow::Error>(())
                        }.is_err() {
                            Error::addMessage(Error::WRITING_FILE_ERROR.clone(), list![(filename.clone()).clone()])?;
                        }
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "save", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut filename: ArcStr = arcstr::literal!("");
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut access: Access = Access::hide;
                    let mut b: bool = false;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::all.clone() {
                        (newp, filename) = Interactive::getContainedClassAndFile(classpath.clone(), SymbolTable::getAbsyn())?;
                        r#str = (Dump::unparseStr(newp.clone(), false, Dump::defaultDumpOptions.clone())?).clone();
                        System::writeFile((filename.clone()).clone(), (r#str.clone()).clone())?;
                        b = true;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "save", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveAll", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Dump::unparseStr(SymbolTable::getAbsyn(), true, Dump::defaultDumpOptions.clone())?).clone();
                    System::writeFile((filename.clone()).clone(), (r#str.clone()).clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut cname: ArcStr = arcstr::literal!("");
                    cname = (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(cname.clone()).clone(), (literal!("global")).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTotalModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut access: Access = Access::hide;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::all.clone() {
                        (s1, _) = getTotalModel(classpath.clone(), b1.clone(), b2.clone(), b3.clone())?;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                    }
                    Ok(Arc::new(Values::Value::STRING { string: (s1.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTotalModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    let mut access: Access = Access::hide;
                    let mut b: bool = false;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::all.clone() {
                        saveTotalModel((filename.clone()).clone(), classpath.clone(), b1.clone(), b2.clone(), b3.clone())?;
                        b = true;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModelDebug", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    let mut access: Access = Access::hide;
                    let mut b: bool = false;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::all.clone() {
                        saveTotalModelDebug((filename.clone()).clone(), classpath.clone(), b1.clone(), b2.clone(), b3.clone())?;
                        b = true;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "saveTotalModelDebug", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDocumentationAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut access: Access = Access::hide;
                    access = Interactive::checkAccessAnnotationAndEncryption(classpath.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::documentation.clone() {
                        (str1, str2, str3) = Interactive::getNamedAnnotationExp(classpath.clone(), SymbolTable::getAbsyn(), Arc::new(Absyn::Path::IDENT { name: (literal!("Documentation")).clone() }), Some((literal!(""), literal!(""), literal!(""))), (std::sync::Arc::new(Interactive::getDocumentationAnnotationString) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<(ArcStr, ArcStr, ArcStr)> + 'static>))?;
                    } else {
                        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                        (str1, str2, str3) = (literal!(""), literal!(""), literal!(""));
                    }
                    Ok(ValuesMake::makeArray(list![Arc::new(Values::Value::STRING { string: (str1.clone()).clone() }), Arc::new(Values::Value::STRING { string: (str2.clone()).clone() }), Arc::new(Values::Value::STRING { string: (str3.clone()).clone() })])?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addClassAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    p = Interactive::addClassAnnotation(AbsynUtil::pathToCref(classpath.clone())?, cons(Arc::new(Absyn::NamedArg { argName: (literal!("annotate")).clone(), argValue: aexp.clone() }), metamodelica::nil()), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addClassAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: annlst } } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    p = SymbolTable::getAbsyn();
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), p.clone(), false, false)?;
                    absynClass = Interactive::addClassAnnotationToClass(absynClass.clone(), Arc::new(Absyn::Annotation { elementArgs: annlst.clone() }))?;
                    p = InteractiveUtil::updateProgram(Absyn::Program { classes: list![absynClass.clone()], within_: if (AbsynUtil::pathIsIdent(classpath.clone())) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(classpath.clone())? }} }, p.clone(), false)?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addClassAnnotation", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setDocumentationAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    p = SymbolTable::getAbsyn();
                    nargs = List::consOnTrue(!(stringEq((str1.clone()).clone(), (literal!("")).clone())?), Arc::new(Absyn::NamedArg { argName: (literal!("info")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (System::escapedString((str1.clone()).clone(), false)).clone() }) }), metamodelica::nil());
                    nargs = List::consOnTrue(!(stringEq((str2.clone()).clone(), (literal!("")).clone())?), Arc::new(Absyn::NamedArg { argName: (literal!("revisions")).clone(), argValue: Arc::new(Absyn::Exp::STRING { value: (System::escapedString((str2.clone()).clone(), false)).clone() }) }), nargs.clone());
                    aexp = Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("Documentation")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: nargs.clone() }), typeVars: metamodelica::nil() });
                    p = Interactive::addClassAnnotation(AbsynUtil::pathToCref(classpath.clone())?, cons(Arc::new(Absyn::NamedArg { argName: (literal!("annotate")).clone(), argValue: aexp.clone() }), metamodelica::nil()), p.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setDocumentationAnnotation", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stat", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut b: bool = false;
                    (b, r1, r2, _) = System::stat((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: b.clone() }), Arc::new(Values::Value::REAL { real: r1.clone() }), Arc::new(Values::Value::REAL { real: r2.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "regularFileExists", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut statFileType: System::StatFileType = System::StatFileType::NoFile;
                    (_, _, _, statFileType) = System::stat((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: statFileType.clone() == System::StatFileType::RegularFile.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "directoryExists", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut statFileType: System::StatFileType = System::StatFileType::NoFile;
                    (_, _, _, statFileType) = System::stat((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: statFileType.clone() == System::StatFileType::Directory.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "OpenModelicaInternal_fullPathName", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::realpath((r#str.clone()).clone())?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isType", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isType(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isPackage", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isPackage(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isClass(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isRecord", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isRecord(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isBlock", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isBlock(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isFunction", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isFunction(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isPartial", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isPartial(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isReplaceable", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isReplaceable(path.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isRedeclare", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isRedeclare(path.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isModel(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isConnector", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isConnector(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOptimization", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isOptimization(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isEnumeration", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isEnumeration(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOperator", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isOperator(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOperatorRecord", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isOperatorRecord(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isOperatorFunction", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut b: bool = false;
                    b = Interactive::isOperatorFunction(classpath.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isProtectedClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut b: bool = false;
                    b = Interactive::isProtectedClass(classpath.clone(), (name.clone()).clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBuiltinType", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    (_, tp, _) = Lookup::lookupType(outCache.clone(), inEnv.clone(), classpath.clone(), Some(Absyn::dummyInfo.clone()))?;
                    r#str = (TypesDump::unparseType(tp.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBuiltinType", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "extendsFrom", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut b: bool = false;
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    paths = Interactive::getAllInheritedClasses(classpath.clone(), SymbolTable::getAbsyn())?;
                    b = List::applyAndFold1(paths.clone(), (std::sync::Arc::new(fnptr!(boolOr, bool, bool)) as std::sync::Arc<dyn ::std::ops::Fn(bool, bool) -> Result<bool> + 'static>), (std::sync::Arc::new(AbsynUtil::pathSuffixOfr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>), baseClassPath.clone(), false);
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "extendsFrom", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isExperiment", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: isExperiment(classpath.clone(), SymbolTable::getAbsyn()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInheritedClasses", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    paths = Interactive::getInheritedClasses(classpath.clone())?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInheritedClasses", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentsTest", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut genv: Interactive::GraphicEnvCache;
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut valsLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?;
                    genv = Interactive::getClassEnv(SymbolTable::getAbsyn(), classpath.clone())?;
                    valsLst = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
        for mut c in (InteractiveUtil::getPublicComponentsInClass(absynClass.clone())).into_iter().cloned() {
                    let __x = getComponentInfo(c.clone(), genv.clone(), false)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    valsLst = listAppend({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Values::Value>>>>> = metamodelica::nil();
        for mut c in (InteractiveUtil::getProtectedComponentsInClass(absynClass.clone())).into_iter().cloned() {
                    let __x = getComponentInfo(c.clone(), genv.clone(), true)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, valsLst.clone());
                    Ok(ValuesMake::makeArray(List::flatten(valsLst.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentsTest", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getSimulationOptions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: startTime }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: stopTime }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: tolerance }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: numberOfIntervals }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: interval }, tail: Deref @ metamodelica::List::Nil } } } } } }) => {
                    let mut simOpt; // TODO: local with unresolved type
                    let mut startTimeExp: Arc<DAE::Exp>;
                    let mut stopTimeExp: Arc<DAE::Exp>;
                    let mut toleranceExp: Arc<DAE::Exp>;
                    let mut intervalExp: Arc<DAE::Exp>;
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    let mut startTime = (*startTime).clone();
                    let mut stopTime = (*stopTime).clone();
                    let mut tolerance = (*tolerance).clone();
                    let mut numberOfIntervals = (*numberOfIntervals).clone();
                    let mut interval = (*interval).clone();
                    cr = AbsynUtil::pathToCref(classpath.clone())?;
                    ErrorExt::setCheckpoint((literal!("getSimulationOptions")).clone());
                    simOpt = InteractiveTypes::SIMULATION_OPTIONS(Arc::new(DAE::Exp::RCONST { real: startTime.clone() }), Arc::new(DAE::Exp::RCONST { real: stopTime.clone() }), Arc::new(DAE::Exp::ICONST { integer: numberOfIntervals.clone() }), Arc::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), Arc::new(DAE::Exp::RCONST { real: tolerance.clone() }), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }), Arc::new(DAE::Exp::SCONST { string: (literal!("")).clone() }))?;
                    ErrorExt::rollBack((literal!("getSimulationOptions")).clone());
                    let (_, metamodelica::List::Cons { head: _, tail: __t4 }) = (StaticScript::getSimulationArguments(FCore::emptyCache(), FGraph::empty(), list![Arc::new(Absyn::Exp::CREF { componentRef: cr.clone() })], metamodelica::nil(), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, (literal!("getSimulationOptions")).clone(), Absyn::dummyInfo.clone(), Some(simOpt.clone()))?) else { bail!("pattern mismatch") };
                    let metamodelica::List::Cons { head: __pa0, tail: __t5 } = (__t4.clone()) else { bail!("pattern mismatch") };
                    let metamodelica::List::Cons { head: __pa1, tail: __t6 } = (__t5.clone()) else { bail!("pattern mismatch") };
                    let metamodelica::List::Cons { head: __pa2, tail: __t7 } = (__t6.clone()) else { bail!("pattern mismatch") };
                    let metamodelica::List::Cons { head: __pa3, tail: _ } = (__t7.clone()) else { bail!("pattern mismatch") };
                    startTimeExp = __pa0.clone();
                    stopTimeExp = __pa1.clone();
                    intervalExp = __pa2.clone();
                    toleranceExp = __pa3.clone();
                    startTime = ValuesUtil::valueReal(Util::makeValueOrDefault((std::sync::Arc::new(Ceval::cevalSimple) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Values::Value>> + 'static>), startTimeExp.clone(), Arc::new(Values::Value::REAL { real: startTime.clone() })))?;
                    stopTime = ValuesUtil::valueReal(Util::makeValueOrDefault((std::sync::Arc::new(Ceval::cevalSimple) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Values::Value>> + 'static>), stopTimeExp.clone(), Arc::new(Values::Value::REAL { real: stopTime.clone() })))?;
                    tolerance = ValuesUtil::valueReal(Util::makeValueOrDefault((std::sync::Arc::new(Ceval::cevalSimple) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Values::Value>> + 'static>), toleranceExp.clone(), Arc::new(Values::Value::REAL { real: tolerance.clone() })))?;
                    let __pa8 = ::match_deref::match_deref! { match &(Util::makeValueOrDefault((std::sync::Arc::new(Ceval::cevalSimple) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>) -> Result<Arc<Values::Value>> + 'static>), intervalExp.clone(), Arc::new(Values::Value::INTEGER { integer: numberOfIntervals.clone() }))) {
                        Deref @ Values::Value::INTEGER { integer: __pa8 } => __pa8.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    numberOfIntervals = __pa8.clone();
                    if numberOfIntervals.clone() == 0 {
                        numberOfIntervals = if (interval.clone() > metamodelica::OrderedFloat(0.0_f64)) {((((stopTime.clone() - startTime.clone()) / interval.clone()).ceil()).0.floor() as i32)} else {0};
                    } else {
                        interval = (stopTime.clone() - startTime.clone()) / std::cmp::max(numberOfIntervals.clone(), 1);
                    }
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: startTime.clone() }), Arc::new(Values::Value::REAL { real: stopTime.clone() }), Arc::new(Values::Value::REAL { real: tolerance.clone() }), Arc::new(Values::Value::INTEGER { integer: numberOfIntervals.clone() }), Arc::new(Values::Value::REAL { real: interval.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationNamedModifiers", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: annotationname }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(getAnnotationNamedModifiers(classpath.clone(), (annotationname.clone()).clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationModifierValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: annotationname }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: modifiername }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(getAnnotationModifierValue(classpath.clone(), (annotationname.clone()).clone(), (modifiername.clone()).clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "searchClassNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    (_, paths) = InteractiveUtil::getClassNamesRecursive(None, SymbolTable::getAbsyn(), false, false, metamodelica::nil())?;
                    paths = paths.clone().reverse();
                    vals = List::map(paths.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeCodeTypeName, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<Arc<Values::Value>> + 'static>));
                    vals = searchClassNames(vals.clone(), (r#str.clone()).clone(), b.clone(), SymbolTable::getAbsyn())?;
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableLibraries", Deref @ metamodelica::List::Nil) => {
                    let mut files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    PackageManagement::installCachedPackages()?;
                    files = PackageManagement::AvailableLibraries::listKeys(PackageManagement::getInstalledLibraries()?, metamodelica::nil());
                    Ok(ValuesMake::makeArray(List::map(files.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailableLibraryVersions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    PackageManagement::installCachedPackages()?;
                    files = PackageManagement::getInstalledLibraryVersions((str1.clone()).clone())?;
                    Ok(ValuesMake::makeArray(List::map(files.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "installPackage", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: PackageManagement::installPackage((str1.clone()).clone(), (str2.clone()).clone(), b.clone(), false)? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "installPackage", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: path @ Deref @ Absyn::Path::QUALIFIED { .. } } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Error::addMessage(Error::ERROR_PKG_NOT_IDENT.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "installPackage", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updatePackageIndex", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: PackageManagement::updateIndex()? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "upgradeInstalledPackages", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: PackageManagement::upgradeInstalledPackages(b.clone())? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageVersions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut s in (PackageManagement::versionsThatProvideTheWanted((str1.clone()).clone(), (str2.clone()).clone(), true)).into_iter().cloned() {
            let __x = ValuesMake::makeString((s.clone()).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageVersions", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsFrom", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeStringArray(PackageManagement::versionsThatConvertFromTheWanted((str1.clone()).clone(), (str2.clone()).clone(), true))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsFrom", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsTo", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: str1 } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeStringArray(PackageManagement::versionsThatConvertToTheWanted((str1.clone()).clone(), (str2.clone()).clone(), true))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAvailablePackageConversionsTo", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getUses", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut uses: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?) {
                        __pa0 @ Deref @ Absyn::Class { .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    absynClass = __pa0.clone();
                    uses = Interactive::getUsesAnnotation(Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP })?;
                    Ok(ValuesMake::makeArray(List::map(uses.clone(), (std::sync::Arc::new(makeUsesArray) as std::sync::Arc<dyn ::std::ops::Fn((Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConversionsFromVersions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut withoutConversion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut withConversion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?) {
                        __pa0 @ Deref @ Absyn::Class { .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    absynClass = __pa0.clone();
                    (withoutConversion, withConversion) = Interactive::getConversionAnnotation(absynClass.clone())?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(List::map(withoutConversion.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?, ValuesMake::makeArray(List::map(withConversion.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedClassModifierNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?;
                    args = Interactive::getDerivedClassModifierNames(absynClass.clone())?;
                    vals = List::map(args.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDerivedClassModifierValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (Interactive::getDerivedClassModifierValue(absynClass.clone(), path.clone())).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAstAsCorbaString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "<interactive>" }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    Print::clearBuf();
                    Dump::getAstAsCorbaString(SymbolTable::getAbsyn())?;
                    r#str = (Print::getString()?).clone();
                    Print::clearBuf();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAstAsCorbaString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str = (*r#str).clone();
                    Print::clearBuf();
                    Dump::getAstAsCorbaString(SymbolTable::getAbsyn())?;
                    Print::writeBuf((r#str.clone()).clone())?;
                    Print::clearBuf();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Wrote result to file: ")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAstAsCorbaString", _) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("getAstAsCorbaString failed")).clone()])?;
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResult", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: size }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut vars_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut filename = (*filename).clone();
                    vars_1 = List::map(cvars.clone(), (std::sync::Arc::new(ValuesUtil::printCodeVariableName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    filename = (Util::absoluteOrRelative((filename.clone()).clone())).clone();
                    Ok(SimulationResults::readDataset((filename.clone()).clone(), vars_1.clone(), size.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResult", _) => {
                    Error::addMessage(Error::SCRIPT_READ_SIM_RES_ERROR.clone(), metamodelica::nil())?;
                    Ok(Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResultSize", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut i: i32 = 0;
                    let mut filename = (*filename).clone();
                    filename = (Util::absoluteOrRelative((filename.clone()).clone())).clone();
                    i = SimulationResults::readSimulationResultSize((filename.clone()).clone())?;
                    Ok(Arc::new(Values::Value::INTEGER { integer: i.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readSimulationResultVars", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut filename = (*filename).clone();
                    filename = (Util::absoluteOrRelative((filename.clone()).clone())).clone();
                    args = SimulationResults::readVariables((filename.clone()).clone(), b1.clone(), b2.clone())?;
                    vals = List::map(args.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareSimulationResults", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::List::Nil } } } } } }) => {
                    let mut vars_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    let mut filename2 = (*filename2).clone();
                    let mut cvars = (*cvars).clone();
                    Error::addMessage(Error::DEPRECATED_API_CALL.clone(), list![(literal!("compareSimulationResults")).clone(), (literal!("diffSimulationResults")).clone()])?;
                    filename = (Util::absoluteOrRelative((filename.clone()).clone())).clone();
                    filename_1 = (Testsuite::friendlyPath((filename_1.clone()).clone())?).clone();
                    filename_1 = (Util::absoluteOrRelative((filename_1.clone()).clone())).clone();
                    filename2 = (Util::absoluteOrRelative((filename2.clone()).clone())).clone();
                    vars_1 = List::map(cvars.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    strings = SimulationResults::cmpSimulationResults(Testsuite::isRunning()?, (filename.clone()).clone(), (filename_1.clone()).clone(), (filename2.clone()).clone(), x1.clone(), x2.clone(), vars_1.clone())?;
                    cvars = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(cvars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareSimulationResults", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("Error in compareSimulationResults")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deltaSimulationResults", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: method_str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut vars_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    filename = (Util::absoluteOrRelative((filename.clone()).clone())).clone();
                    filename_1 = (Testsuite::friendlyPath((filename_1.clone()).clone())?).clone();
                    filename_1 = (Util::absoluteOrRelative((filename_1.clone()).clone())).clone();
                    vars_1 = List::map(cvars.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    val = SimulationResults::deltaSimulationResults((filename.clone()).clone(), (filename_1.clone()).clone(), (method_str.clone()).clone(), vars_1.clone())?;
                    Ok(Arc::new(Values::Value::REAL { real: val.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deltaSimulationResults", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("Error in deltaSimulationResults")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "filterSimulationResults", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: numberOfIntervals }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: hintReadAllVars }, tail: Deref @ metamodelica::List::Nil } } } } } }) => {
                    let mut vars_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut b = (*b).clone();
                    vars_1 = List::map(cvars.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    b = SimulationResults::filterSimulationResults((filename.clone()).clone(), (filename_1.clone()).clone(), vars_1.clone(), numberOfIntervals.clone(), b.clone(), hintReadAllVars.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "filterSimulationResults", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResults", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: reltol }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: reltolDiffMinMax }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rangeDelta }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } } } } } } } }) => {
                    let mut v1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut vars_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    let mut filename2 = (*filename2).clone();
                    let mut cvars = (*cvars).clone();
                    let mut b = (*b).clone();
                    filename = (Util::absoluteOrRelative((filename.clone()).clone())).clone();
                    filename_1 = (Testsuite::friendlyPath((filename_1.clone()).clone())?).clone();
                    filename_1 = (Util::absoluteOrRelative((filename_1.clone()).clone())).clone();
                    filename2 = (Util::absoluteOrRelative((filename2.clone()).clone())).clone();
                    vars_1 = List::map(cvars.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    (b, strings) = SimulationResults::diffSimulationResults(Testsuite::isRunning()?, (filename.clone()).clone(), (filename_1.clone()).clone(), (filename2.clone()).clone(), reltol.clone(), reltolDiffMinMax.clone(), rangeDelta.clone(), vars_1.clone(), b.clone())?;
                    cvars = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    v1 = ValuesMake::makeArray(cvars.clone())?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: b.clone() }), v1.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResults", _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    v = ValuesMake::makeArray(metamodelica::nil())?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: false }), v.clone()] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResultsHtml", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: reltol }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: reltolDiffMinMax }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: rangeDelta }, tail: Deref @ metamodelica::List::Nil } } } } } }) => {
                    let mut r#str = (*r#str).clone();
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    filename = (Util::absoluteOrRelative((filename.clone()).clone())).clone();
                    filename_1 = (Testsuite::friendlyPath((filename_1.clone()).clone())?).clone();
                    filename_1 = (Util::absoluteOrRelative((filename_1.clone()).clone())).clone();
                    r#str = (SimulationResults::diffSimulationResultsHtml(Testsuite::isRunning()?, (filename.clone()).clone(), (filename_1.clone()).clone(), reltol.clone(), reltolDiffMinMax.clone(), rangeDelta.clone(), (r#str.clone()).clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "diffSimulationResultsHtml", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkTaskGraph", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut pwd: ArcStr = arcstr::literal!("");
                    let mut cvars: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    pwd = (System::pwd()).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    filename = (if (StringUtil::startsWith((filename.clone()).clone(), (literal!("/")).clone())) {filename.clone()} else {stringAppendList(list![(pwd.clone()).clone(), (pd.clone()).clone(), (filename.clone()).clone()])}).clone();
                    filename_1 = (if (StringUtil::startsWith((filename_1.clone()).clone(), (literal!("/")).clone())) {filename_1.clone()} else {stringAppendList(list![(pwd.clone()).clone(), (pd.clone()).clone(), (filename_1.clone()).clone()])}).clone();
                    strings = TaskGraphResults::checkTaskGraph((filename.clone()).clone(), (filename_1.clone()).clone())?;
                    cvars = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(cvars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkTaskGraph", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("Error in checkTaskGraph")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkCodeGraph", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename_1 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut pwd: ArcStr = arcstr::literal!("");
                    let mut cvars: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut filename = (*filename).clone();
                    let mut filename_1 = (*filename_1).clone();
                    pwd = (System::pwd()).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    filename = (if (StringUtil::startsWith((filename.clone()).clone(), (literal!("/")).clone())) {filename.clone()} else {stringAppendList(list![(pwd.clone()).clone(), (pd.clone()).clone(), (filename.clone()).clone()])}).clone();
                    filename_1 = (if (StringUtil::startsWith((filename_1.clone()).clone(), (literal!("/")).clone())) {filename_1.clone()} else {stringAppendList(list![(pwd.clone()).clone(), (pd.clone()).clone(), (filename_1.clone()).clone()])}).clone();
                    strings = TaskGraphResults::checkCodeGraph((filename.clone()).clone(), (filename_1.clone()).clone())?;
                    cvars = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(cvars.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkCodeGraph", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("Error in checkCodeGraph")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotAll", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: externalWindow }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: title }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: gridStr }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: logX }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: logY }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: xLabel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yLabel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: curveWidth }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: curveStyle }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: legendPosition }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: footer }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: autoScale }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: forceOMPlot }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yAxis }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yLabelRight }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y1R }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y2R }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } } } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut call: ArcStr = arcstr::literal!("");
                    let mut omhome: ArcStr = arcstr::literal!("");
                    let mut logXStr: ArcStr = arcstr::literal!("");
                    let mut logYStr: ArcStr = arcstr::literal!("");
                    let mut x1Str: ArcStr = arcstr::literal!("");
                    let mut x2Str: ArcStr = arcstr::literal!("");
                    let mut y1Str: ArcStr = arcstr::literal!("");
                    let mut y2Str: ArcStr = arcstr::literal!("");
                    let mut curveWidthStr: ArcStr = arcstr::literal!("");
                    let mut curveStyleStr: ArcStr = arcstr::literal!("");
                    let mut autoScaleStr: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut filename = (*filename).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    omhome = (Settings::getInstallationDirectoryPath()?).clone();
                    (outCache, filename) = cevalCurrentSimulationResultExp(outCache.clone(), inEnv.clone(), (filename.clone()).clone(), msg.clone())?;
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*filename.clone()); ArcStr::from(__mm_s) }).clone();
                    s1 = (if (arcstr::literal!(Autoconf::os) == literal!("Windows_NT")) {literal!(".exe")} else {literal!("")}).clone();
                    filename = (if (System::regularFileExists((str1.clone()).clone())) {str1.clone()} else {filename.clone()}).clone();
                    b = System::plotCallBackDefined();
                    if boolOr(forceOMPlot.clone(), boolNot(b.clone())) {
                        str2 = stringAppendList(list![(omhome.clone()).clone(), (pd.clone()).clone(), (literal!("bin")).clone(), (pd.clone()).clone(), (literal!("OMPlot")).clone(), (s1.clone()).clone()]);
                        str3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--filename=\"")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!("\" --title=\"")); __mm_s.push_str(&*title.clone()); __mm_s.push_str(&*literal!("\" --grid=")); __mm_s.push_str(&*gridStr.clone()); __mm_s.push_str(&*literal!(" --plotAll --logx=")); __mm_s.push_str(&*boolString(logX.clone())); __mm_s.push_str(&*literal!(" --logy=")); __mm_s.push_str(&*boolString(logY.clone())); __mm_s.push_str(&*literal!(" --yaxis=\"")); __mm_s.push_str(&*yAxis.clone()); __mm_s.push_str(&*literal!("\" --xlabel=\"")); __mm_s.push_str(&*xLabel.clone()); __mm_s.push_str(&*literal!("\" --ylabel=\"")); __mm_s.push_str(&*yLabel.clone()); __mm_s.push_str(&*literal!("\" --ylabel-right=\"")); __mm_s.push_str(&*yLabelRight.clone()); __mm_s.push_str(&*literal!("\" --xrange=")); __mm_s.push_str(&*realString(x1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(x2.clone())); __mm_s.push_str(&*literal!(" --yrange=")); __mm_s.push_str(&*realString(y1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2.clone())); __mm_s.push_str(&*literal!(" --yrange-right=")); __mm_s.push_str(&*realString(y1R.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2R.clone())); __mm_s.push_str(&*literal!(" --new-window=")); __mm_s.push_str(&*boolString(externalWindow.clone())); __mm_s.push_str(&*literal!(" --curve-width=")); __mm_s.push_str(&*realString(curveWidth.clone())); __mm_s.push_str(&*literal!(" --curve-style=")); __mm_s.push_str(&*intString(curveStyle.clone())); __mm_s.push_str(&*literal!(" --legend-position=\"")); __mm_s.push_str(&*legendPosition.clone()); __mm_s.push_str(&*literal!("\" --footer=\"")); __mm_s.push_str(&*footer.clone()); __mm_s.push_str(&*literal!("\" --auto-scale=")); __mm_s.push_str(&*boolString(autoScale.clone())); ArcStr::from(__mm_s) }).clone();
                        call = stringAppendList(list![(literal!("\"")).clone(), (str2.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (str3.clone()).clone()]);
                        let 0 = (System::spawnCall((str2.clone()).clone(), (call.clone()).clone())) else { bail!("pattern mismatch") };
                    } else if b.clone() {
                        logXStr = (boolString(logX.clone())).clone();
                        logYStr = (boolString(logY.clone())).clone();
                        x1Str = (realString(x1.clone())).clone();
                        x2Str = (realString(x2.clone())).clone();
                        y1Str = (realString(y1.clone())).clone();
                        y2Str = (realString(y2.clone())).clone();
                        curveWidthStr = (realString(curveWidth.clone())).clone();
                        curveStyleStr = (intString(curveStyle.clone())).clone();
                        autoScaleStr = (boolString(autoScale.clone())).clone();
                        System::plotCallBack(externalWindow.clone(), (filename.clone()).clone(), (title.clone()).clone(), (gridStr.clone()).clone(), (literal!("plotall")).clone(), (logXStr.clone()).clone(), (logYStr.clone()).clone(), (xLabel.clone()).clone(), (yLabel.clone()).clone(), (x1Str.clone()).clone(), (x2Str.clone()).clone(), (y1Str.clone()).clone(), (y2Str.clone()).clone(), (curveWidthStr.clone()).clone(), (curveStyleStr.clone()).clone(), (legendPosition.clone()).clone(), (footer.clone()).clone(), (autoScaleStr.clone()).clone(), (literal!("")).clone());
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotAll", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plot", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: externalWindow }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: title }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: gridStr }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: logX }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: logY }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: xLabel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yLabel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: curveWidth }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: curveStyle }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: legendPosition }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: footer }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: autoScale }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: forceOMPlot }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yAxis }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yLabelRight }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y1R }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y2R }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } } } } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut call: ArcStr = arcstr::literal!("");
                    let mut omhome: ArcStr = arcstr::literal!("");
                    let mut logXStr: ArcStr = arcstr::literal!("");
                    let mut logYStr: ArcStr = arcstr::literal!("");
                    let mut x1Str: ArcStr = arcstr::literal!("");
                    let mut x2Str: ArcStr = arcstr::literal!("");
                    let mut y1Str: ArcStr = arcstr::literal!("");
                    let mut y2Str: ArcStr = arcstr::literal!("");
                    let mut curveWidthStr: ArcStr = arcstr::literal!("");
                    let mut curveStyleStr: ArcStr = arcstr::literal!("");
                    let mut autoScaleStr: ArcStr = arcstr::literal!("");
                    let mut vars_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut filename = (*filename).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    vars_1 = List::map(cvars.clone(), (std::sync::Arc::new(ValuesUtil::printCodeVariableName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    omhome = (Settings::getInstallationDirectoryPath()?).clone();
                    (outCache, filename) = cevalCurrentSimulationResultExp(outCache.clone(), inEnv.clone(), (filename.clone()).clone(), msg.clone())?;
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*filename.clone()); ArcStr::from(__mm_s) }).clone();
                    s1 = (if (arcstr::literal!(Autoconf::os) == literal!("Windows_NT")) {literal!(".exe")} else {literal!("")}).clone();
                    filename = (if (System::regularFileExists((str1.clone()).clone())) {str1.clone()} else {filename.clone()}).clone();
                    b = System::plotCallBackDefined();
                    if boolOr(forceOMPlot.clone(), boolNot(b.clone())) {
                        r#str = stringDelimitList(vars_1.clone(), (literal!("\" \"")).clone());
                        str2 = stringAppendList(list![(omhome.clone()).clone(), (pd.clone()).clone(), (literal!("bin")).clone(), (pd.clone()).clone(), (literal!("OMPlot")).clone(), (s1.clone()).clone()]);
                        str3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--filename=\"")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!("\" --title=\"")); __mm_s.push_str(&*title.clone()); __mm_s.push_str(&*literal!("\" --grid=")); __mm_s.push_str(&*gridStr.clone()); __mm_s.push_str(&*literal!(" --plot --logx=")); __mm_s.push_str(&*boolString(logX.clone())); __mm_s.push_str(&*literal!(" --logy=")); __mm_s.push_str(&*boolString(logY.clone())); __mm_s.push_str(&*literal!(" --yaxis=\"")); __mm_s.push_str(&*yAxis.clone()); __mm_s.push_str(&*literal!("\" --xlabel=\"")); __mm_s.push_str(&*xLabel.clone()); __mm_s.push_str(&*literal!("\" --ylabel=\"")); __mm_s.push_str(&*yLabel.clone()); __mm_s.push_str(&*literal!("\" --ylabel-right=\"")); __mm_s.push_str(&*yLabelRight.clone()); __mm_s.push_str(&*literal!("\" --xrange=")); __mm_s.push_str(&*realString(x1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(x2.clone())); __mm_s.push_str(&*literal!(" --yrange=")); __mm_s.push_str(&*realString(y1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2.clone())); __mm_s.push_str(&*literal!(" --yrange-right=")); __mm_s.push_str(&*realString(y1R.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2R.clone())); __mm_s.push_str(&*literal!(" --new-window=")); __mm_s.push_str(&*boolString(externalWindow.clone())); __mm_s.push_str(&*literal!(" --curve-width=")); __mm_s.push_str(&*realString(curveWidth.clone())); __mm_s.push_str(&*literal!(" --curve-style=")); __mm_s.push_str(&*intString(curveStyle.clone())); __mm_s.push_str(&*literal!(" --legend-position=\"")); __mm_s.push_str(&*legendPosition.clone()); __mm_s.push_str(&*literal!("\" --footer=\"")); __mm_s.push_str(&*footer.clone()); __mm_s.push_str(&*literal!("\" --auto-scale=")); __mm_s.push_str(&*boolString(autoScale.clone())); __mm_s.push_str(&*literal!(" \"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                        call = stringAppendList(list![(literal!("\"")).clone(), (str2.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (str3.clone()).clone()]);
                        let 0 = (System::spawnCall((str2.clone()).clone(), (call.clone()).clone())) else { bail!("pattern mismatch") };
                    } else if b.clone() {
                        logXStr = (boolString(logX.clone())).clone();
                        logYStr = (boolString(logY.clone())).clone();
                        x1Str = (realString(x1.clone())).clone();
                        x2Str = (realString(x2.clone())).clone();
                        y1Str = (realString(y1.clone())).clone();
                        y2Str = (realString(y2.clone())).clone();
                        curveWidthStr = (realString(curveWidth.clone())).clone();
                        curveStyleStr = (intString(curveStyle.clone())).clone();
                        autoScaleStr = (boolString(autoScale.clone())).clone();
                        r#str = stringDelimitList(vars_1.clone(), (literal!(" ")).clone());
                        System::plotCallBack(externalWindow.clone(), (filename.clone()).clone(), (title.clone()).clone(), (gridStr.clone()).clone(), (literal!("plot")).clone(), (logXStr.clone()).clone(), (logYStr.clone()).clone(), (xLabel.clone()).clone(), (yLabel.clone()).clone(), (x1Str.clone()).clone(), (x2Str.clone()).clone(), (y1Str.clone()).clone(), (y2Str.clone()).clone(), (curveWidthStr.clone()).clone(), (curveStyleStr.clone()).clone(), (legendPosition.clone()).clone(), (footer.clone()).clone(), (autoScaleStr.clone()).clone(), (r#str.clone()).clone());
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plot", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "val", Deref @ metamodelica::List::Cons { head: cvar, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: timeStamp }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "<default>" }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut filename: ArcStr = arcstr::literal!("");
                    let mut varNameStr: ArcStr = arcstr::literal!("");
                    let mut val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut outCache: FCore::Cache = outCache.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(outCache.clone(), inEnv.clone(), buildCurrentSimulationResultExp()?, true, msg.clone(), 0)?) {
                        (__pa0, Deref @ Values::Value::STRING { string: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    outCache = __pa0.clone();
                    filename = __pa1.clone();
                    varNameStr = (ValuesUtil::printCodeVariableName(cvar.clone())?).clone();
                    val = SimulationResults::val((filename.clone()).clone(), (varNameStr.clone()).clone(), timeStamp.clone())?;
                    Ok(Arc::new(Values::Value::REAL { real: val.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "val", Deref @ metamodelica::List::Cons { head: cvar, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: timeStamp }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut varNameStr: ArcStr = arcstr::literal!("");
                    let mut val: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let false = (stringEq((filename.clone()).clone(), (literal!("<default>")).clone())?) else { bail!("pattern mismatch") };
                    varNameStr = (ValuesUtil::printCodeVariableName(cvar.clone())?).clone();
                    val = SimulationResults::val((filename.clone()).clone(), (varNameStr.clone()).clone(), timeStamp.clone())?;
                    Ok(Arc::new(Values::Value::REAL { real: val.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "closeSimulationResultFile", _) => {
                    SimulationResults::close();
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getParameterNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strings = Interactive::getParameterNames(path.clone(), SymbolTable::getAbsyn())?;
                    vals = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getParameterValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut str2: ArcStr = arcstr::literal!("");
                    str2 = (Interactive::getComponentBinding(path.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn())).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (str2.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setParameterValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = InteractiveUtil::setElementModifier(classpath.clone(), path.clone(), Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: aexp.clone(), info: Absyn::dummyInfo.clone() }) }), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentModifierNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strings = Interactive::getComponentModifierNames(path.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn())?;
                    vals = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentModifierValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cr = AbsynUtil::pathToCref(path.clone())?;
                    if AbsynUtil::crefIsIdent(cr.clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        s1 = __pa0.clone();
                        r#str = (Interactive::getComponentBinding(classpath.clone(), (s1.clone()).clone(), SymbolTable::getAbsyn())).clone();
                    } else {
                        s1 = (AbsynUtil::crefFirstIdent(cr.clone())?).clone();
                        cr = AbsynUtil::crefStripFirst(cr.clone())?;
                        r#str = (Interactive::getComponentModifierValue(AbsynUtil::pathToCref(classpath.clone())?, Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (s1.clone()).clone(), subscripts: metamodelica::nil() }), cr.clone(), SymbolTable::getAbsyn())).clone();
                    }
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentModifierValues", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cr = AbsynUtil::pathToCref(path.clone())?;
                    if AbsynUtil::crefIsIdent(cr.clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        s1 = __pa0.clone();
                        r#str = (Interactive::getComponentBinding(classpath.clone(), (s1.clone()).clone(), SymbolTable::getAbsyn())).clone();
                    } else {
                        s1 = (AbsynUtil::crefFirstIdent(cr.clone())?).clone();
                        cr = AbsynUtil::crefStripFirst(cr.clone())?;
                        r#str = (Interactive::getComponentModifierValues(AbsynUtil::pathToCref(classpath.clone())?, Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (s1.clone()).clone(), subscripts: metamodelica::nil() }), cr.clone(), SymbolTable::getAbsyn())?).clone();
                    }
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setElementModifierValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = InteractiveUtil::setElementModifier(classpath.clone(), path.clone(), r#mod.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getExtendsModifierValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(Interactive::getExtendsModifierValue(classpath.clone(), baseClassPath.clone(), path.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setExtendsModifierValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = InteractiveUtil::setExtendsModifier(classpath.clone(), baseClassPath.clone(), path.clone(), r#mod.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setExtendsModifier", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = InteractiveUtil::setExtendsModifier(classpath.clone(), baseClassPath.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("_")).clone() }), r#mod.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isExtendsModifierFinal", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(Interactive::isExtendsModifierFinal(classpath.clone(), baseClassPath.clone(), path.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "removeComponentModifiers", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: keepRedeclares }, tail: _ } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::removeComponentModifiers(path.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn(), keepRedeclares.clone());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementModifierNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strings = InteractiveUtil::getElementModifierNames(path.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn())?;
                    vals = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementModifierValue", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cr = AbsynUtil::pathToCref(path.clone())?;
                    if AbsynUtil::crefIsIdent(cr.clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        s1 = __pa0.clone();
                        r#str = (InteractiveUtil::getElementBinding(classpath.clone(), (s1.clone()).clone(), SymbolTable::getAbsyn())).clone();
                    } else {
                        s1 = (AbsynUtil::crefFirstIdent(cr.clone())?).clone();
                        cr = AbsynUtil::crefStripFirst(cr.clone())?;
                        r#str = (InteractiveUtil::getElementModifierValue(AbsynUtil::pathToCref(classpath.clone())?, Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (s1.clone()).clone(), subscripts: metamodelica::nil() }), cr.clone(), SymbolTable::getAbsyn())).clone();
                    }
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementModifierValues", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut cr: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    cr = AbsynUtil::pathToCref(path.clone())?;
                    if AbsynUtil::crefIsIdent(cr.clone()) {
                        let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
                            Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, .. } => __pa0.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        s1 = __pa0.clone();
                        r#str = (InteractiveUtil::getElementBinding(classpath.clone(), (s1.clone()).clone(), SymbolTable::getAbsyn())).clone();
                    } else {
                        s1 = (AbsynUtil::crefFirstIdent(cr.clone())?).clone();
                        cr = AbsynUtil::crefStripFirst(cr.clone())?;
                        r#str = (InteractiveUtil::getElementModifierValues(AbsynUtil::pathToCref(classpath.clone())?, Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (s1.clone()).clone(), subscripts: metamodelica::nil() }), cr.clone(), SymbolTable::getAbsyn())?).clone();
                    }
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "removeElementModifiers", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: keepRedeclares }, tail: _ } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = InteractiveUtil::removeElementModifiers(path.clone(), (str1.clone()).clone(), SymbolTable::getAbsyn(), keepRedeclares.clone());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "removeExtendsModifiers", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: baseClassPath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: keepRedeclares }, tail: _ } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::removeExtendsModifiers(classpath.clone(), baseClassPath.clone(), SymbolTable::getAbsyn(), keepRedeclares.clone())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInstantiatedParametersAndValues", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut odae: Option<DAE::DAElist> = None;
                    let mut strings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, _, odae, _) = runFrontEnd(outCache.clone(), inEnv.clone(), classpath.clone(), true, false, false)?;
                    strings = Interactive::getInstantiatedParametersAndValues(odae.clone())?;
                    vals = List::map(strings.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>));
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInstantiatedParametersAndValues", _) => {
                    Error::addCompilerWarning((literal!("getInstantiatedParametersAndValues failed to instantiate the model.")).clone())?;
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    p = InteractiveUtil::updateConnectionAnnotation(AbsynUtil::pathToCref(classpath.clone())?, (str1.clone()).clone(), (str2.clone()).clone(), cons(Arc::new(Absyn::NamedArg { argName: (literal!("annotate")).clone(), argValue: aexp.clone() }), metamodelica::nil()), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: annlst } } }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    p = SymbolTable::getAbsyn();
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), p.clone(), false, false)?;
                    absynClass = InteractiveUtil::updateConnectionAnnotationInClass(absynClass.clone(), (str1.clone()).clone(), (str2.clone()).clone(), Arc::new(Absyn::Annotation { elementArgs: annlst.clone() }))?;
                    p = InteractiveUtil::updateProgram(Absyn::Program { classes: list![absynClass.clone()], within_: if (AbsynUtil::pathIsIdent(classpath.clone())) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(classpath.clone())? }} }, p.clone(), false)?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnection", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: annStr }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
                    let mut istmts: GlobalScript::Statements;
                    let mut nargs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
                    let mut annlst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    istmts = Parser::parsestringexp(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("__dummy(")); __mm_s.push_str(&*annStr.clone()); __mm_s.push_str(&*literal!(");")); ArcStr::from(__mm_s) }).clone(), (literal!("<interactive>")).clone())?;
                    let GlobalScript::ISTMTS { interactiveStmtLst: metamodelica::List::Cons { head: GlobalScript::IEXP { exp: __pa0, .. }, tail: __t1 }, .. } = (istmts.clone()) else { bail!("pattern mismatch") };
                    ::match_deref::match_deref! { match &(__t1.clone()) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    aexp = __pa0.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(aexp.clone()) {
                        Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: __pa2, .. }, .. } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    nargs = __pa2.clone();
                    let __pa4 = ::match_deref::match_deref! { match &(listHead(nargs.clone())?) {
                        Deref @ Absyn::NamedArg { argValue: Deref @ Absyn::Exp::CODE { code: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::NOMOD, elementArgLst: __pa4 } } }, .. } => __pa4.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    annlst = __pa4.clone();
                    p = SymbolTable::getAbsyn();
                    absynClass = InteractiveUtil::getPathedClassInProgram(classpath.clone(), p.clone(), false, false)?;
                    absynClass = InteractiveUtil::updateConnectionAnnotationInClass(absynClass.clone(), (str1.clone()).clone(), (str2.clone()).clone(), Arc::new(Absyn::Annotation { elementArgs: annlst.clone() }))?;
                    p = InteractiveUtil::updateProgram(Absyn::Program { classes: list![absynClass.clone()], within_: if (AbsynUtil::pathIsIdent(classpath.clone())) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(classpath.clone())? }} }, p.clone(), false)?;
                    SymbolTable::setAbsynClass(p.clone(), absynClass.clone(), classpath.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionAnnotation", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str4 }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (b, p) = InteractiveUtil::updateConnectionNames(classpath.clone(), (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), (str4.clone()).clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateConnectionNames", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectionCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    let mut access: Access = Access::hide;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::diagram.clone() {
                        n = (Interactive::getConnections(absynClass.clone())?.len() as i32);
                    } else {
                        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                        n = 0;
                    }
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectionCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut access: Access = Access::hide;
                    access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::diagram.clone() {
                        vals = Interactive::getNthConnection(AbsynUtil::pathToCref(path.clone())?, SymbolTable::getAbsyn(), n.clone())?;
                    } else {
                        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                        vals = metamodelica::nil();
                    }
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnection", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectionList", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(getConnectionList(path.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = (getAlgorithms(absynClass.clone())?.len() as i32);
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithm", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthAlgorithm(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithm", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = (getInitialAlgorithms(absynClass.clone())?.len() as i32);
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithm", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthInitialAlgorithm(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithm", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmItemsCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = getAlgorithmItemsCount(absynClass.clone())?;
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAlgorithmItemsCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithmItem", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthAlgorithmItem(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAlgorithmItem", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmItemsCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = getInitialAlgorithmItemsCount(absynClass.clone())?;
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialAlgorithmItemsCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithmItem", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthInitialAlgorithmItem(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialAlgorithmItem", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = (getEquations(absynClass.clone())?.len() as i32);
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthEquation(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquation", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = (getInitialEquations(absynClass.clone())?.len() as i32);
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthInitialEquation(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquation", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationItemsCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = getEquationItemsCount(absynClass.clone())?;
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEquationItemsCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquationItem", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthEquationItem(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthEquationItem", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationItemsCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = getInitialEquationItemsCount(absynClass.clone())?;
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInitialEquationItemsCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquationItem", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthInitialEquationItem(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInitialEquationItem", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = getAnnotationCount(absynClass.clone())?;
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAnnotationCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAnnotationString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    r#str = (getNthAnnotationString(absynClass.clone(), n.clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthAnnotationString", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut n: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    n = getImportCount(absynClass.clone());
                    Ok(Arc::new(Values::Value::INTEGER { integer: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportCount", _) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthImport", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    absynClass = InteractiveUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?;
                    vals = getNthImport(absynClass.clone(), n.clone())?;
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthImport", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotParametric", Deref @ metamodelica::List::Cons { head: cvar, tail: Deref @ metamodelica::List::Cons { head: cvar2, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: externalWindow }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: title }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: gridStr }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: logX }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: logY }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: xLabel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yLabel }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y2 }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: curveWidth }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: curveStyle }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: legendPosition }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: footer }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: autoScale }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: forceOMPlot }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yAxis }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: yLabelRight }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y1R }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::REAL { real: y2R }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } } } } } } } } } } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut call: ArcStr = arcstr::literal!("");
                    let mut omhome: ArcStr = arcstr::literal!("");
                    let mut logXStr: ArcStr = arcstr::literal!("");
                    let mut logYStr: ArcStr = arcstr::literal!("");
                    let mut x1Str: ArcStr = arcstr::literal!("");
                    let mut x2Str: ArcStr = arcstr::literal!("");
                    let mut y1Str: ArcStr = arcstr::literal!("");
                    let mut y2Str: ArcStr = arcstr::literal!("");
                    let mut curveWidthStr: ArcStr = arcstr::literal!("");
                    let mut curveStyleStr: ArcStr = arcstr::literal!("");
                    let mut autoScaleStr: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut filename = (*filename).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    omhome = (Settings::getInstallationDirectoryPath()?).clone();
                    (outCache, filename) = cevalCurrentSimulationResultExp(outCache.clone(), inEnv.clone(), (filename.clone()).clone(), msg.clone())?;
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*filename.clone()); ArcStr::from(__mm_s) }).clone();
                    s1 = (if (arcstr::literal!(Autoconf::os) == literal!("Windows_NT")) {literal!(".exe")} else {literal!("")}).clone();
                    filename = (if (System::regularFileExists((str1.clone()).clone())) {str1.clone()} else {filename.clone()}).clone();
                    b = System::plotCallBackDefined();
                    if boolOr(forceOMPlot.clone(), boolNot(b.clone())) {
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(cvar.clone())?); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(cvar2.clone())?); ArcStr::from(__mm_s) }).clone();
                        str2 = stringAppendList(list![(omhome.clone()).clone(), (pd.clone()).clone(), (literal!("bin")).clone(), (pd.clone()).clone(), (literal!("OMPlot")).clone(), (s1.clone()).clone()]);
                        str3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--filename=\"")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!("\" --title=\"")); __mm_s.push_str(&*title.clone()); __mm_s.push_str(&*literal!("\" --grid=")); __mm_s.push_str(&*gridStr.clone()); __mm_s.push_str(&*literal!(" --plotParametric --logx=")); __mm_s.push_str(&*boolString(logX.clone())); __mm_s.push_str(&*literal!(" --logy=")); __mm_s.push_str(&*boolString(logY.clone())); __mm_s.push_str(&*literal!(" --yaxis=\"")); __mm_s.push_str(&*yAxis.clone()); __mm_s.push_str(&*literal!("\" --xlabel=\"")); __mm_s.push_str(&*xLabel.clone()); __mm_s.push_str(&*literal!("\" --ylabel=\"")); __mm_s.push_str(&*yLabel.clone()); __mm_s.push_str(&*literal!("\" --ylabel-right=\"")); __mm_s.push_str(&*yLabelRight.clone()); __mm_s.push_str(&*literal!("\" --xrange=")); __mm_s.push_str(&*realString(x1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(x2.clone())); __mm_s.push_str(&*literal!(" --yrange=")); __mm_s.push_str(&*realString(y1.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2.clone())); __mm_s.push_str(&*literal!(" --yrange-right=")); __mm_s.push_str(&*realString(y1R.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*realString(y2R.clone())); __mm_s.push_str(&*literal!(" --new-window=")); __mm_s.push_str(&*boolString(externalWindow.clone())); __mm_s.push_str(&*literal!(" --curve-width=")); __mm_s.push_str(&*realString(curveWidth.clone())); __mm_s.push_str(&*literal!(" --curve-style=")); __mm_s.push_str(&*intString(curveStyle.clone())); __mm_s.push_str(&*literal!(" --legend-position=\"")); __mm_s.push_str(&*legendPosition.clone()); __mm_s.push_str(&*literal!("\" --footer=\"")); __mm_s.push_str(&*footer.clone()); __mm_s.push_str(&*literal!("\" --auto-scale=")); __mm_s.push_str(&*boolString(autoScale.clone())); __mm_s.push_str(&*literal!(" \"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                        call = stringAppendList(list![(literal!("\"")).clone(), (str2.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (str3.clone()).clone()]);
                        let 0 = (System::spawnCall((str2.clone()).clone(), (call.clone()).clone())) else { bail!("pattern mismatch") };
                    } else if b.clone() {
                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(cvar.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*ValuesUtil::printCodeVariableName(cvar2.clone())?); ArcStr::from(__mm_s) }).clone();
                        logXStr = (boolString(logX.clone())).clone();
                        logYStr = (boolString(logY.clone())).clone();
                        x1Str = (realString(x1.clone())).clone();
                        x2Str = (realString(x2.clone())).clone();
                        y1Str = (realString(y1.clone())).clone();
                        y2Str = (realString(y2.clone())).clone();
                        curveWidthStr = (realString(curveWidth.clone())).clone();
                        curveStyleStr = (intString(curveStyle.clone())).clone();
                        autoScaleStr = (boolString(autoScale.clone())).clone();
                        System::plotCallBack(externalWindow.clone(), (filename.clone()).clone(), (title.clone()).clone(), (gridStr.clone()).clone(), (literal!("plotparametric")).clone(), (logXStr.clone()).clone(), (logYStr.clone()).clone(), (xLabel.clone()).clone(), (yLabel.clone()).clone(), (x1Str.clone()).clone(), (x2Str.clone()).clone(), (y1Str.clone()).clone(), (y2Str.clone()).clone(), (curveWidthStr.clone()).clone(), (curveStyleStr.clone()).clone(), (legendPosition.clone()).clone(), (footer.clone()).clone(), (autoScaleStr.clone()).clone(), (r#str.clone()).clone());
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "plotParametric", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "dumpXMLDAE", vals) => {
                    let mut xml_filename: ArcStr = arcstr::literal!("");
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, xml_filename) = dumpXMLDAE(outCache.clone(), inEnv.clone(), vals.clone(), msg.clone())?;
                    Ok(ValuesMake::makeTuple(list![Arc::new(Values::Value::BOOL { boolean: true }), Arc::new(Values::Value::STRING { string: (xml_filename.clone()).clone() })]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "dumpXMLDAE", _) => {
                    Ok(ValuesMake::makeTuple(list![Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "solveLinearSystem", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Cons { head: v, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut i: i32 = 0;
                    let mut realVals: Arc<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                    let mut v = (*v).clone();
                    (realVals, i) = System::dgesv(List::map(vals.clone(), (std::sync::Arc::new(ValuesUtil::arrayValueReals) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<Arc<metamodelica::List<metamodelica::Real>>> + 'static>)), ValuesUtil::arrayValueReals(v.clone())?)?;
                    v = ValuesMake::makeArray(List::map(realVals.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeReal, metamodelica::Real)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Real) -> Result<Arc<Values::Value>> + 'static>)))?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![v.clone(), Arc::new(Values::Value::INTEGER { integer: i.clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "solveLinearSystem", Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: v, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Unknown input to solveLinearSystem scripting function")).clone()])?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![v.clone(), Arc::new(Values::Value::INTEGER { integer: -1 })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "relocateFunctions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: v @ Deref @ Values::Value::ARRAY { .. }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut b: bool = false;
                    let mut relocatableFunctionsTuple: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
                    relocatableFunctionsTuple = metamodelica::nil();
                    for mut varr in &*var_field!((**v).valueLst, Values::Value::ARRAY).clone() {
                        let mut varr = varr.clone();
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(varr.clone()) {
                            Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Nil } }, .. } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        s1 = __pa0.clone();
                        s2 = __pa1.clone();
                        relocatableFunctionsTuple = cons((s1.clone(), s2.clone()), relocatableFunctionsTuple.clone());
                    }
                    b = System::relocateFunctions((r#str.clone()).clone(), relocatableFunctionsTuple.clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "toJulia", Deref @ metamodelica::List::Nil) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Tpl::tplString((std::sync::Arc::new(AbsynToJulia::dumpProgram) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Program) -> Result<Tpl::Text> + 'static>), SymbolTable::getAbsyn())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "interactiveDumpAbsynToJL", Deref @ metamodelica::List::Nil) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Tpl::tplString((std::sync::Arc::new(AbsynJLDumpTpl::dump) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Program) -> Result<Tpl::Text> + 'static>), SymbolTable::getAbsyn())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "relocateFunctions", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runConversionScript", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(runConversionScript(path.clone(), (r#str.clone()).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "convertPackageToLibrary", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(convertPackageToLibrary(classpath.clone(), path.clone(), (r#str.clone()).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelInstance", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    Ok(NFApi::getModelInstance(classpath.clone(), path.clone(), (r#str.clone()).clone(), b.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelInstanceAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: v @ Deref @ Values::Value::ARRAY { .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(NFApi::getModelInstanceAnnotation(classpath.clone(), ValuesUtil::arrayValueStrings(v.clone())?, b.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modifierToJSON", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(NFApi::modifierToJSON((r#str.clone()).clone(), b.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "storeAST", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: SymbolTable::storeAST()? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "restoreAST", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: SymbolTable::restoreAST(n.clone())? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "qualifyPath", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeCodeTypeName(NFApi::mkFullyQual(SymbolTable::getAbsyn(), classpath.clone(), path.clone(), false)?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (InteractiveUtil::getElementAnnotation(path.clone(), SymbolTable::getAbsyn())).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setElementAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut b: bool = false;
                    (_, b) = InteractiveUtil::setElementAnnotation(path.clone(), r#mod.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadClassContentString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: x }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: y }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = InteractiveUtil::loadClassContentString((r#str.clone()).clone(), classpath.clone(), x.clone(), y.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setElementType", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut b: bool = false;
                    (_, b) = InteractiveUtil::setElementType(path.clone(), cr.clone(), SymbolTable::getAbsyn());
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getExtendsModifierNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(InteractiveUtil::getExtendsModifierNames(classpath.clone(), path.clone(), b.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isPrimitive", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isPrimitive(classpath.clone(), SymbolTable::getAbsyn())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isParameter", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isParameter(path.clone(), classpath.clone(), SymbolTable::getAbsyn())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isConstant", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isConstant(path.clone(), classpath.clone(), SymbolTable::getAbsyn())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "isProtected", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::isProtected(path.clone(), classpath.clone(), SymbolTable::getAbsyn())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setComponentDimensions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp @ Deref @ Absyn::Exp::ARRAY { .. } } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::setComponentDimensions(classpath.clone(), path.clone(), var_field!((**aexp).arrayExp, Absyn::Exp::ARRAY).clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setComponentProperties", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Nil } }, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: Deref @ metamodelica::List::Nil }, .. }, tail: Deref @ metamodelica::List::Nil } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (p, v) = Interactive::setComponentProperties(classpath.clone(), (name.clone()).clone(), {
        let mut __acc: Arc<metamodelica::List<bool>> = metamodelica::nil();
        for mut va in (vals.clone()).into_iter().cloned() {
                    let __x = ValuesUtil::valueBool(va.clone())?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, (s1.clone()).clone(), b1.clone(), b2.clone(), (s2.clone()).clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "createModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    p = Interactive::createModel(classpath.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "newModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    p = Interactive::newModel(classpath.clone(), path.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(true))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (b, p) = Interactive::deleteClass(classpath.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addComponent", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp2 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp3 } }, tail: Deref @ metamodelica::List::Nil } } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::addComponent((name.clone()).clone(), path.clone(), classpath.clone(), aexp.clone(), r#mod.clone(), aexp2.clone(), aexp3.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateComponent", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_MODIFICATION { modification: r#mod } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp2 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp3 } }, tail: Deref @ metamodelica::List::Nil } } } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::updateComponent((name.clone()).clone(), path.clone(), classpath.clone(), aexp.clone(), r#mod.clone(), aexp2.clone(), aexp3.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteComponent", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::deleteComponent((name.clone()).clone(), classpath.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(ValuesMake::makeInteger(Interactive::getComponentCount(classpath.clone(), SymbolTable::getAbsyn())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponent", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthComponent(classpath.clone(), SymbolTable::getAbsyn(), n.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponents", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getComponents(classpath.clone(), b.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElements", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getElements(classpath.clone(), b.clone(), SymbolTable::getAbsyn(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementsInfo", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getElementsInfo(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentAnnotations", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getComponentAnnotations(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getElementAnnotations", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getElementAnnotations(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponentAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthComponentAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponentModification", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthComponentModification(classpath.clone(), n.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthComponentCondition", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthComponentCondition(classpath.clone(), n.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInheritanceCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getInheritanceCount(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInheritedClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(NFApi::getNthInheritedClass(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setConnectionComment", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::setConnectionComment(classpath.clone(), cr.clone(), cr2.clone(), (r#str.clone()).clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_EXPRESSION { exp: aexp2 } }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::addConnection(classpath.clone(), cr.clone(), cr2.clone(), aexp.clone(), aexp2.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteConnection", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::deleteConnection(classpath.clone(), cr.clone(), cr2.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnectionAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthConnectionAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConnectorCount", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getConnectorCount(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnector", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthConnector(classpath.clone(), n.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthConnectorIconAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthConnectorIconAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getIconAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getIconAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDiagramAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getDiagramAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "refactorIconAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::refactorIconAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "refactorDiagramAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::refactorDiagramAnnotation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "refactorClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::refactorClass(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInheritedClassIconMapAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthInheritedClassIconMapAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNthInheritedClassDiagramMapAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: n }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNthInheritedClassDiagramMapAnnotation(classpath.clone(), n.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getNamedAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getNamedAnnotation(classpath.clone(), path.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getShortDefinitionBaseClassInformation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getShortDefinitionBaseClassInformation(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getExternalFunctionSpecification", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getExternalFunctionSpecification(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEnumerationLiterals", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getEnumerationLiterals(classpath.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "existClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::existClass(classpath.clone(), SymbolTable::getAbsyn())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getComponentComment", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Interactive::getComponentComment(classpath.clone(), path.clone(), SymbolTable::getAbsyn())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setComponentComment", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut b: bool = false;
                    (p, b) = Interactive::setComponentComment(classpath.clone(), path.clone(), (r#str.clone()).clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(ValuesMake::makeBoolean(b.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "renameClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (p, v) = Interactive::renameClass(classpath.clone(), path.clone(), SymbolTable::getAbsyn())?;
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "renameComponent", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (p, v) = Interactive::renameComponent(classpath.clone(), cr.clone(), cr2.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "renameComponentInClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: cr2 } }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (p, v) = Interactive::renameComponentOnlyInClass(classpath.clone(), cr.clone(), cr2.clone(), SymbolTable::getAbsyn());
                    SymbolTable::setAbsyn(p.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCrefInfo", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getCrefInfo(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefaultComponentName", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getDefaultComponentName(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefaultComponentPrefixes", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getDefaultComponentPrefixes(classpath.clone(), SymbolTable::getAbsyn()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefinitions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Interactive::getDefinitions(SymbolTable::getAbsyn(), b.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getDefaultOpenCLDevice", Deref @ metamodelica::List::Nil) => {
                    Ok(ValuesMake::makeInteger(Config::getDefaultOpenCLDevice()?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reverseLookup", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    Ok(ValuesMake::makeString((ReverseLookup::lookup(path.clone(), classpath.clone(), SymbolTable::getAbsyn(), b1.clone(), b2.clone())?).clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "translateResidualsDAE", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(ValuesMake::makeBoolean(NFApi::translateResidualsDAE(path.clone(), (s1.clone()).clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "addEquation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::addEquation(path.clone(), (s1.clone()).clone(), b1.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "updateEquation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: s2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b3 }, tail: Deref @ metamodelica::List::Nil } } } } } } }) => {
                    Ok(ValuesMake::makeBoolean(Interactive::updateEquation(path.clone(), (s1.clone()).clone(), (s2.clone()).clone(), b.clone(), b1.clone(), b2.clone(), b3.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn getSimulationExtension(mut inString: ArcStr, mut inString2: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inString.clone(), inString2.clone())) {
        (Deref @ "C", Deref @ "WIN64") => literal!(".bat"),
        (Deref @ "C", Deref @ "WIN32") => literal!(".bat"),
        (Deref @ "Cpp", Deref @ "WIN32") => literal!(".bat"),
        (Deref @ "Cpp", Deref @ "WIN64") => literal!(".bat"),
        (Deref @ "Cpp", Deref @ "Unix") => literal!(".sh"),
        (Deref @ "omsicpp", Deref @ "WIN64") => literal!(".bat"),
        (Deref @ "omsicpp", Deref @ "WIN32") => literal!(".bat"),
        (Deref @ "omsicpp", Deref @ "Unix") => literal!(".sh"),
        _ => arcstr::literal!(Autoconf::exeExt),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn getAdjacencyMatrix(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut className: Arc<Absyn::Path>, mut inMsg: Absyn::Msg, mut filenameprefix: ArcStr) -> Result<(FCore::Cache, Arc<Values::Value>, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut outString: ArcStr = arcstr::literal!("");
    (outCache, outValue, outString) = (::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), className.clone(), inMsg.clone(), filenameprefix.clone())) {
        (cache, env, _, _, _) => {
            let mut filename: ArcStr = arcstr::literal!("");
            let mut file_dir: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut dae: DAE::DAElist;
            let mut dlow: Arc<BackendDAE::BackendDAE>;
            let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            let mut flatModelicaStr: ArcStr = arcstr::literal!("");
            let mut description: ArcStr = arcstr::literal!("");
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let (__pa0, __pa1, Some(__pa2), _) = (runFrontEnd(cache.clone(), env.clone(), className.clone(), true, false, true)?) else { bail!("pattern mismatch") };
            cache = __pa0.clone();
            env = __pa1.clone();
            dae = __pa2.clone();
            description = (DAEUtil::daeDescription(dae.clone())).clone();
            a_cref = AbsynUtil::pathToCref(className.clone())?;
            file_dir = (getFileDir(a_cref.clone(), SymbolTable::getAbsyn())?).clone();
            dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (filenameprefix.clone()).clone(), simSettingsOption: None })?;
            dlow = FindZeroCrossings::findZeroCrossings(dlow.clone())?;
            flatModelicaStr = DAEDump::dumpStr(dae.clone(), FCore::getFunctionTree(cache.clone()))?;
            flatModelicaStr = (stringAppend((literal!("OldEqStr={'")).clone(), (flatModelicaStr.clone()).clone())).clone();
            flatModelicaStr = (System::stringReplace((flatModelicaStr.clone()).clone(), (literal!("\n")).clone(), (literal!("%##%")).clone())?).clone();
            flatModelicaStr = (System::stringReplace((flatModelicaStr.clone()).clone(), (literal!("%##%")).clone(), (literal!("','")).clone())?).clone();
            flatModelicaStr = (stringAppend((flatModelicaStr.clone()).clone(), (literal!("'};")).clone())).clone();
            filename = (DAEQuery::writeAdjacencyMatrix(dlow.clone(), (filenameprefix.clone()).clone(), (flatModelicaStr.clone()).clone())?).clone();
            r#str = (stringAppend((literal!("The equation system was dumped to Matlab file:")).clone(), (filename.clone()).clone())).clone();
            (cache.clone(), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }), file_dir.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outValue, outString))
}

/* -------------------------------------------------------------------
                         RUN FRONTEND
   ------------------------------------------------------------------- */
pub fn runFrontEnd(mut cache: FCore::Cache, mut env: FCore::Graph, mut className: Arc<Absyn::Path>, mut relaxedFrontEnd: bool, mut dumpFlat: bool, mut transform: bool) -> Result<(FCore::Cache, FCore::Graph, Option<DAE::DAElist>, ArcStr)> {
    let mut cache: FCore::Cache = cache;
    let mut env: FCore::Graph = env;
    let mut odae: Option<DAE::DAElist> = None;
    let mut flatString: ArcStr = literal!("");
    let mut dae: DAE::DAElist;
    let mut b: bool = false;
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    match '__try0: {
        b = unwrap_break_err!(loadProgram(className.clone()), '__try0);
        let true = (b.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        if unwrap_break_err!(Flags::isSet(Flags::GC_PROF.clone()), '__try0) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(GCExt::profStatsStr(GCExt::getProfStats(), (literal!("GC stats before front-end:")).clone(), (literal!("\n  ")).clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        unwrap_break_err!(ExecStat::execStat((literal!("FrontEnd - loaded program")).clone()), '__try0);
        (cache, env, dae, flatString) = unwrap_break_err!(runFrontEndWork(cache.clone(), env.clone(), className.clone(), relaxedFrontEnd.clone(), dumpFlat.clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::GC_PROF.clone()), '__try0) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(GCExt::profStatsStr(GCExt::getProfStats(), (literal!("GC stats after front-end:")).clone(), (literal!("\n  ")).clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        unwrap_break_err!(ExecStat::execStat((literal!("FrontEnd - DAE generated")).clone()), '__try0);
        if transform.clone() {
            dae = unwrap_break_err!(DAEUtil::transformationsBeforeBackend(cache.clone(), env.clone(), dae.clone()), '__try0);
        }
        odae = Some(dae.clone());
        Ok::<_, anyhow::Error>((b.clone(), cache.clone(), dae.clone(), env.clone(), flatString.clone(), odae.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            b = __try0_o0;
            cache = __try0_o1;
            dae = __try0_o2;
            env = __try0_o3;
            flatString = __try0_o4;
            odae = __try0_o5;
        }
        Err(_) => {
            bail!("try/else: outputs not set in else branch");
        }
    }
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), false)?;
    Ok((cache, env, odae, flatString))
}

pub fn runFrontEndNF(mut className: Arc<Absyn::Path>, mut relaxedFrontEnd: bool, mut dumpFlat: bool) -> Result<(Arc<NFFlatModel::NFFlatModel>, Arc<NFFlatten::FunctionTreeImpl::Tree>, ArcStr)> {
    let mut flatModel: Arc<NFFlatModel::NFFlatModel>;
    let mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree> = Arc::new(NFFlatten::FunctionTreeImpl::Tree::EMPTY);
    let mut flatString: ArcStr = arcstr::literal!("");
    let true = (loadProgram(className.clone())?) else { bail!("pattern mismatch") };
    (flatModel, functions, flatString) = runFrontEndWorkNF(className.clone(), relaxedFrontEnd.clone(), dumpFlat.clone())?;
    Ok((flatModel, functions, flatString))
}

pub fn runFrontEndWorkNF(mut className: Arc<Absyn::Path>, mut relaxedFrontend: bool, mut dumpFlat: bool) -> Result<(Arc<NFFlatModel::NFFlatModel>, Arc<NFFlatten::FunctionTreeImpl::Tree>, ArcStr)> {
    let mut flatModel: Arc<NFFlatModel::NFFlatModel>;
    let mut functions: Arc<NFFlatten::FunctionTreeImpl::Tree> = Arc::new(NFFlatten::FunctionTreeImpl::Tree::EMPTY);
    let mut flatString: ArcStr = arcstr::literal!("");
    let mut builtin_p: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut scode_p: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut annotation_p: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut nf_api: bool = false;
    let mut inst_failed: bool = false;
    let mut cls_name: Arc<Absyn::Path> = className.clone();
    let mut obfuscate_map: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>;
    let mut obfuscate_mode: ArcStr = arcstr::literal!("");
    (_, builtin_p) = FBuiltin::getInitialFunctions()?;
    scode_p = SymbolTable::getSCode()?;
    obfuscate_mode = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
    if obfuscate_mode.clone() == literal!("none") && Interactive::astContainsEncryptedClass(SymbolTable::getAbsyn())? {
        FlagsUtil::setConfigString(Flags::OBFUSCATE.clone(), (literal!("encrypted")).clone())?;
    }
    if obfuscate_mode.clone() == literal!("full") {
        (scode_p, cls_name, _, _, obfuscate_map) = Obfuscate::obfuscateProgram(scode_p.clone(), cls_name.clone(), SCode::noComment.clone())?;
    }
    scode_p = listAppend(builtin_p.clone(), scode_p.clone());
    ExecStat::execStat((literal!("FrontEnd - Absyn->SCode")).clone())?;
    annotation_p = AbsynToSCode::translateAbsyn2SCode(InteractiveUtil::modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?)?;
    nf_api = FlagsUtil::set(Flags::NF_API.clone(), false)?;
    inst_failed = false;
    match '__try0: {
        (flatModel, functions, flatString) = unwrap_break_err!(NFInst::instClassInProgram(cls_name.clone(), scode_p.clone(), annotation_p.clone(), relaxedFrontend.clone(), dumpFlat.clone()), '__try0);
        Ok::<_, anyhow::Error>((flatModel.clone(), flatString.clone(), functions.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            flatModel = __try0_o0;
            flatString = __try0_o1;
            functions = __try0_o2;
        }
        Err(_) => {
            inst_failed = true;
            NFInst::clearCaches()?;
            bail!("try/else: outputs not set in else branch");
        }
    }
    FlagsUtil::set(Flags::NF_API.clone(), nf_api.clone())?;
    if inst_failed.clone() {
        bail!("fail");
    }
    Ok((flatModel, functions, flatString))
}

fn getProcsStr(mut isMake: bool) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut n: i32 = 0;
    let mut sn: ArcStr = arcstr::literal!("");
    n = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    sn = (intString(n.clone())).clone();
    s = (if (n.clone() == 0) {literal!("")} else {if (isMake.clone()) {sn.clone()} else {stringAppend((literal!("-j")).clone(), (sn.clone()).clone())}}).clone();
    Ok(s)
}

fn configureFMU_cmake(mut platform: ArcStr, mut fmutmp: ArcStr, mut fmuTargetName: ArcStr, mut logfile: ArcStr, mut externalLibLocations: Arc<metamodelica::List<ArcStr>>, mut isWindows: bool) -> Result<()> {
    let mut fmuSourceDir: ArcStr = arcstr::literal!("");
    let mut CMAKE_GENERATOR: ArcStr = literal!("");
    let mut CMAKE_BUILD_TYPE: ArcStr = arcstr::literal!("");
    let mut quote: ArcStr = arcstr::literal!("");
    let mut dquote: ArcStr = arcstr::literal!("");
    let mut defaultFmiIncludeDirectoy: ArcStr = arcstr::literal!("");
    let mut CC: ArcStr = arcstr::literal!("");
    let mut makefileParams: SimCodeFunction::MakefileParams;
    makefileParams = SimCodeFunctionUtil::createMakefileParams(metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false, true)?;
    fmuSourceDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/")); ArcStr::from(__mm_s) }).clone();
    quote = (literal!("'")).clone();
    dquote = (if (isWindows.clone()) {literal!("\"")} else {literal!("'")}).clone();
    CC = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-DCMAKE_C_COMPILER=")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*System::basename((makefileParams.ccompiler.clone()).clone())); __mm_s.push_str(&*dquote.clone()); ArcStr::from(__mm_s) }).clone();
    defaultFmiIncludeDirectoy = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/c/fmi")); __mm_s.push_str(&*dquote.clone()); ArcStr::from(__mm_s) }).clone();
    if Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_BLACKBOX.clone() || Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_PROTECTED.clone() {
        CMAKE_BUILD_TYPE = (literal!("-DCMAKE_BUILD_TYPE=Release")).clone();
    } else if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
        CMAKE_BUILD_TYPE = (literal!("-DCMAKE_BUILD_TYPE=Debug")).clone();
    } else {
        CMAKE_BUILD_TYPE = (literal!("-DCMAKE_BUILD_TYPE=RelWithDebInfo")).clone();
    }
    if System::regularFileExists((logfile.clone()).clone()) {
        System::removeFile((logfile.clone()).clone());
    }
    let _ = (::match_deref::match_deref! { match &(Util::stringSplitAtChar((platform.clone()).clone(), (literal!(" ")).clone())?) {
        Deref @ metamodelica::List::Cons { head: Deref @ "dynamic", tail: Deref @ metamodelica::List::Nil } => {
            let mut cmd: ArcStr = arcstr::literal!("");
            let mut cmakeCall: ArcStr = arcstr::literal!("");
            let mut buildDir: ArcStr = arcstr::literal!("");
            if isWindows.clone() {
                CMAKE_GENERATOR = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-G ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!("MSYS Makefiles")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            }
            buildDir = (literal!("build_cmake_dynamic")).clone();
            cmakeCall = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CMAKE_GENERATOR.clone()); __mm_s.push_str(&*CMAKE_BUILD_TYPE.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CC.clone()); __mm_s.push_str(&*literal!(" ..")); ArcStr::from(__mm_s) }).clone();
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*fmuSourceDir.clone()); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("mkdir ")); __mm_s.push_str(&*buildDir.clone()); __mm_s.push_str(&*literal!(" && cd ")); __mm_s.push_str(&*buildDir.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*cmakeCall.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" --build . --parallel ")); __mm_s.push_str(&*getProcsStr(false)?); __mm_s.push_str(&*literal!(" --target install && ")); __mm_s.push_str(&*literal!("cd .. && rm -rf ")); __mm_s.push_str(&*buildDir.clone()); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCallRestrictedEnv((cmd.clone()).clone(), (logfile.clone()).clone())? {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cmd: ")); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            ()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ "static", tail: Deref @ metamodelica::List::Nil } => {
            let mut cmd: ArcStr = arcstr::literal!("");
            let mut cmakeCall: ArcStr = arcstr::literal!("");
            let mut buildDir: ArcStr = arcstr::literal!("");
            if isWindows.clone() {
                CMAKE_GENERATOR = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-G ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!("MSYS Makefiles")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
            }
            buildDir = (literal!("build_cmake_static")).clone();
            cmakeCall = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CMAKE_GENERATOR.clone()); __mm_s.push_str(&*CMAKE_BUILD_TYPE.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*CC.clone()); __mm_s.push_str(&*literal!(" ..")); ArcStr::from(__mm_s) }).clone();
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*fmuSourceDir.clone()); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("mkdir ")); __mm_s.push_str(&*buildDir.clone()); __mm_s.push_str(&*literal!(" && cd ")); __mm_s.push_str(&*buildDir.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*cmakeCall.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*arcstr::literal!(Autoconf::cmake)); __mm_s.push_str(&*literal!(" --build . --parallel ")); __mm_s.push_str(&*getProcsStr(false)?); __mm_s.push_str(&*literal!(" --target install && ")); __mm_s.push_str(&*literal!("cd .. && rm -rf ")); __mm_s.push_str(&*buildDir.clone()); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCallRestrictedEnv((cmd.clone()).clone(), (logfile.clone()).clone())? {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cmd: ")); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            }
            ()
        },
        Deref @ metamodelica::List::Cons { head: crossTriple, tail: Deref @ metamodelica::List::Cons { head: Deref @ "docker", tail: Deref @ metamodelica::List::Cons { head: Deref @ "run", tail: dockerImgArgs } } } => {
            let mut cmd: ArcStr = arcstr::literal!("");
            let mut cmakeCall: ArcStr = arcstr::literal!("");
            let mut buildDir: ArcStr = arcstr::literal!("");
            let mut fmiTarget: ArcStr = arcstr::literal!("");
            let mut uid: i32 = 0;
            let mut cidFile: ArcStr = arcstr::literal!("");
            let mut volumeID: ArcStr = arcstr::literal!("");
            let mut containerID: ArcStr = arcstr::literal!("");
            let mut userID: ArcStr = arcstr::literal!("");
            let mut dockerLogFile: ArcStr = arcstr::literal!("");
            let mut locations: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            uid = System::getuid();
            cidFile = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!(".cidfile")); ArcStr::from(__mm_s) }).clone();
            dockerLogFile = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*crossTriple.clone()); __mm_s.push_str(&*literal!(".tmp.log")); ArcStr::from(__mm_s) }).clone();
            if System::regularFileExists((dockerLogFile.clone()).clone()) {
                System::removeFile((dockerLogFile.clone()).clone());
            }
            cmd = (literal!("docker volume create")).clone();
            runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), false, (literal!("")).clone(), (literal!("")).clone())?;
            volumeID = (List::last(System::strtok((System::readFile((dockerLogFile.clone()).clone())?).clone(), (literal!("\n")).clone()))?).clone();
            if System::regularFileExists((cidFile.clone()).clone()) {
                System::removeFile((cidFile.clone()).clone());
            }
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run --cidfile ")); __mm_s.push_str(&*cidFile.clone()); __mm_s.push_str(&*literal!(" -v ")); __mm_s.push_str(&*volumeID.clone()); __mm_s.push_str(&*literal!(":/data busybox true")); ArcStr::from(__mm_s) }).clone();
            runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (literal!("")).clone())?;
            containerID = (System::trim((System::readFile((cidFile.clone()).clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone();
            System::removeFile((cidFile.clone()).clone());
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(":/data")); ArcStr::from(__mm_s) }).clone();
            runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*defaultFmiIncludeDirectoy.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(":/data/fmiInclude")); ArcStr::from(__mm_s) }).clone();
            runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
            (locations, _) = SimCodeUtil::getDirectoriesForDLLsFromLinkLibs(externalLibLocations.clone());
            for mut loc in &*locations.clone() {
                let mut loc = loc.clone();
                if System::directoryExists((loc.clone()).clone()) {
                    cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run --rm --hostname=")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(" --volume=")); __mm_s.push_str(&*volumeID.clone()); __mm_s.push_str(&*literal!(":/data busybox mkdir -p ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!("/data")); __mm_s.push_str(&*loc.clone()); __mm_s.push_str(&*dquote.clone()); ArcStr::from(__mm_s) }).clone();
                    runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
                    cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp -a -L ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*loc.clone()); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(":/data")); __mm_s.push_str(&*System::dirname((loc.clone()).clone())); __mm_s.push_str(&*dquote.clone()); ArcStr::from(__mm_s) }).clone();
                    runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
                }
            }
            userID = (if (uid.clone() != 0) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--user ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", uid.clone()))); ArcStr::from(__mm_s) }} else {literal!("")}).clone();
            buildDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("build_cmake_")); __mm_s.push_str(&*crossTriple.clone()); ArcStr::from(__mm_s) }).clone();
            if 0 != (System::regex((crossTriple.clone()).clone(), (literal!("mingw")).clone(), 1, false, false)).0 {
                fmiTarget = (literal!(" -DCMAKE_SYSTEM_NAME=Windows ")).clone();
            } else if 0 != (System::regex((crossTriple.clone()).clone(), (literal!("apple")).clone(), 1, false, false)).0 {
                fmiTarget = (literal!(" -DCMAKE_SYSTEM_NAME=Darwin ")).clone();
            } else {
                fmiTarget = (literal!("")).clone();
            }
            cmakeCall = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cmake -DFMI_INTERFACE_HEADER_FILES_DIRECTORY=/fmu/fmiInclude ")); __mm_s.push_str(&*literal!("-DDOCKER_VOL_DIR=/fmu ")); __mm_s.push_str(&*fmiTarget.clone()); __mm_s.push_str(&*CMAKE_BUILD_TYPE.clone()); __mm_s.push_str(&*literal!(" ..")); ArcStr::from(__mm_s) }).clone();
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run ")); __mm_s.push_str(&*userID.clone()); __mm_s.push_str(&*literal!(" --rm -w /fmu -v ")); __mm_s.push_str(&*volumeID.clone()); __mm_s.push_str(&*literal!(":/fmu -e CROSS_TRIPLE=")); __mm_s.push_str(&*crossTriple.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*stringDelimitList(dockerImgArgs.clone(), (literal!(" ")).clone())); __mm_s.push_str(&*literal!(" sh -c ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!("/fmu/")); __mm_s.push_str(&*fmuSourceDir.clone()); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("mkdir ")); __mm_s.push_str(&*buildDir.clone()); __mm_s.push_str(&*literal!(" && cd ")); __mm_s.push_str(&*buildDir.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*cmakeCall.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("cmake --build . &&  make ")); __mm_s.push_str(&*getProcsStr(true)?); __mm_s.push_str(&*literal!(" install && ")); __mm_s.push_str(&*literal!("cd .. && rm -rf ")); __mm_s.push_str(&*buildDir.clone()); __mm_s.push_str(&*dquote.clone()); ArcStr::from(__mm_s) }).clone();
            runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
            if isWindows.clone() {
                cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run ")); __mm_s.push_str(&*userID.clone()); __mm_s.push_str(&*literal!(" --rm -w /fmu -v ")); __mm_s.push_str(&*volumeID.clone()); __mm_s.push_str(&*literal!(":/fmu ")); __mm_s.push_str(&*stringDelimitList(dockerImgArgs.clone(), (literal!(" ")).clone())); __mm_s.push_str(&*literal!(" tar -zcf comp-fmutmp.tar.gz ")); __mm_s.push_str(&*fmutmp.clone()); ArcStr::from(__mm_s) }).clone();
                runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
                cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(":/data/comp-fmutmp.tar.gz .")); ArcStr::from(__mm_s) }).clone();
                runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), true, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
                System::systemCall((literal!("tar zxf comp-fmutmp.tar.gz && rm comp-fmutmp.tar.gz")).clone(), (literal!("")).clone());
            } else {
                cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(":/data/")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/ .")); ArcStr::from(__mm_s) }).clone();
                runDockerCmd((cmd.clone()).clone(), (dockerLogFile.clone()).clone(), false, (volumeID.clone()).clone(), (containerID.clone()).clone())?;
            }
            System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker rm ")); __mm_s.push_str(&*containerID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
            System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
            System::copyFile((dockerLogFile.clone()).clone(), (logfile.clone()).clone());
            System::removeFile((dockerLogFile.clone()).clone());
            ()
        },
        _ => {
            Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown/unsupported platform \"")); __mm_s.push_str(&*platform.clone()); __mm_s.push_str(&*literal!(" \" for CMake FMU build. ")); __mm_s.push_str(&*literal!("Use platforms={\"dynamic\"} for the default case.")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn runDockerCmd(mut cmd: ArcStr, mut logfile: ArcStr, mut cleanup: bool, mut volumeID: ArcStr, mut containerID: ArcStr) -> Result<()> {
    let mut verbose: bool = false;
    System::appendFile((logfile.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
    if 0 != System::systemCall((cmd.clone()).clone(), (logfile.clone()).clone()) {
        Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(" failed:\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
        if cleanup.clone() {
            if !(stringEqual((containerID.clone()).clone(), (literal!("")).clone())) {
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker rm ")); __mm_s.push_str(&*containerID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
            }
            if !(stringEqual((volumeID.clone()).clone(), (literal!("")).clone())) {
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
            }
        }
        bail!("fail");
    } else if verbose.clone() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    Ok(())
}

fn configureFMU(mut platform: ArcStr, mut fmutmp: ArcStr, mut logfile: ArcStr, mut isWindows: bool, mut needs3rdPartyLibs: bool) -> Result<()> {
    let mut CC: ArcStr = arcstr::literal!("");
    let mut CFLAGS: ArcStr = arcstr::literal!("");
    let mut CPPFLAGS: ArcStr = arcstr::literal!("");
    let mut LDFLAGS: ArcStr = arcstr::literal!("");
    let mut SUNDIALS: ArcStr = arcstr::literal!("");
    let mut makefileStr: ArcStr = arcstr::literal!("");
    let mut container: ArcStr = arcstr::literal!("");
    let mut host: ArcStr = arcstr::literal!("");
    let mut nozip: ArcStr = arcstr::literal!("");
    let mut path1: ArcStr = arcstr::literal!("");
    let mut path2: ArcStr = arcstr::literal!("");
    let mut dir: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/")); ArcStr::from(__mm_s) };
    let mut cmd: ArcStr = literal!("");
    let mut quote: ArcStr = literal!("'");
    let mut dquote: ArcStr = if (isWindows.clone()) {literal!("\"")} else {literal!("'")};
    let mut includeDefaultFmi: ArcStr = arcstr::literal!("");
    let mut volumeID: ArcStr = arcstr::literal!("");
    let mut cidFile: ArcStr = arcstr::literal!("");
    let mut containerID: ArcStr = arcstr::literal!("");
    let mut rest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut finishedBuild: bool = false;
    let mut uid: i32 = 0;
    let mut status: i32 = 0;
    let mut verbose: bool = false;
    includeDefaultFmi = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/c/fmi")); __mm_s.push_str(&*dquote.clone()); ArcStr::from(__mm_s) }).clone();
    CC = (System::getCCompiler()).clone();
    if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
        CFLAGS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-O0 -g ")); __mm_s.push_str(&*System::stringReplace((System::getCFlags()).clone(), (literal!("${MODELICAUSERCFLAGS}")).clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone();
    } else {
        CFLAGS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-Os ")); __mm_s.push_str(&*System::stringReplace((System::getCFlags()).clone(), (literal!("${MODELICAUSERCFLAGS}")).clone(), (literal!("")).clone())?); ArcStr::from(__mm_s) }).clone();
    }
    LDFLAGS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-L")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*literal!("-Wl,-rpath,")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*System::getLDFlags()); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone();
    CPPFLAGS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-I. -I")); __mm_s.push_str(&*includeDefaultFmi.clone()); __mm_s.push_str(&*literal!(" -DOMC_FMI_RUNTIME=1")); ArcStr::from(__mm_s) }).clone();
    if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
        CPPFLAGS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*CPPFLAGS.clone()); __mm_s.push_str(&*literal!(" -O0 -g ")); ArcStr::from(__mm_s) }).clone();
    }
    if needs3rdPartyLibs.clone() {
        SUNDIALS = (literal!("1")).clone();
        CPPFLAGS = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*CPPFLAGS.clone()); __mm_s.push_str(&*literal!(" -DWITH_SUNDIALS=1 -DLINK_SUNDIALS_STATIC")); __mm_s.push_str(&*literal!(" -Isundials")); ArcStr::from(__mm_s) }).clone();
    } else {
        SUNDIALS = (literal!("")).clone();
    }
    if System::regularFileExists((logfile.clone()).clone()) {
        System::removeFile((logfile.clone()).clone());
    }
    nozip = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*arcstr::literal!(Autoconf::make)); __mm_s.push_str(&*literal!(" -j")); __mm_s.push_str(&*intString(Config::noProc()?)); __mm_s.push_str(&*literal!(" nozip")); ArcStr::from(__mm_s) }).clone();
    finishedBuild = (::match_deref::match_deref! { match &(Util::stringSplitAtChar((platform.clone()).clone(), (literal!(" ")).clone())?) {
        Deref @ metamodelica::List::Cons { head: Deref @ "dynamic", tail: Deref @ metamodelica::List::Nil } => {
            makefileStr = (System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("Makefile.in")); ArcStr::from(__mm_s) }).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@CC@")).clone(), (CC.clone()).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@CFLAGS@")).clone(), (CFLAGS.clone()).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@LDFLAGS@")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*LDFLAGS.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::ldflags_runtime_sim)); ArcStr::from(__mm_s) }).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@LIBS@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@DLLEXT@")).clone(), (arcstr::literal!(Autoconf::dllExt)).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_RUNTIME@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_DGESV@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_CMINPACK@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_SUNDIALS@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@FMIPLATFORM@")).clone(), (System::modelicaPlatform()).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@CPPFLAGS@")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*CPPFLAGS.clone()); __mm_s.push_str(&*literal!(" -DOMC_SIM_SETTINGS_CMDLINE")); ArcStr::from(__mm_s) }).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@LIBTYPE_DYNAMIC@")).clone(), (literal!("1")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@BSTATIC@")).clone(), (arcstr::literal!(Autoconf::bstatic)).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@BDYNAMIC@")).clone(), (arcstr::literal!(Autoconf::bdynamic)).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("\r\n")).clone(), (literal!("\n")).clone())?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("Makefile")); ArcStr::from(__mm_s) }).clone(), (makefileStr.clone()).clone())?;
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("config.log")); ArcStr::from(__mm_s) }).clone(), (literal!("Using cached values for dynamic platform")).clone())?;
            cmd = (literal!("cached values")).clone();
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ "static", tail: Deref @ metamodelica::List::Nil } => {
            makefileStr = (System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("Makefile.in")); ArcStr::from(__mm_s) }).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@CC@")).clone(), (CC.clone()).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@CFLAGS@")).clone(), (CFLAGS.clone()).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@LDFLAGS@")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*LDFLAGS.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::ldflags_runtime_fmu_static)); ArcStr::from(__mm_s) }).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@LIBS@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@DLLEXT@")).clone(), (arcstr::literal!(Autoconf::dllExt)).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_RUNTIME@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_DGESV@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_CMINPACK@")).clone(), (literal!("")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@NEED_SUNDIALS@")).clone(), (SUNDIALS.clone()).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@FMIPLATFORM@")).clone(), (System::modelicaPlatform()).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@CPPFLAGS@")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*CPPFLAGS.clone()); __mm_s.push_str(&*literal!(" -DCMINPACK_NO_DLL=1")); ArcStr::from(__mm_s) }).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@LIBTYPE_DYNAMIC@")).clone(), (literal!("1")).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@BSTATIC@")).clone(), (arcstr::literal!(Autoconf::bstatic)).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("@BDYNAMIC@")).clone(), (arcstr::literal!(Autoconf::bdynamic)).clone())?).clone();
            makefileStr = (System::stringReplace((makefileStr.clone()).clone(), (literal!("\r\n")).clone(), (literal!("\n")).clone())?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("Makefile")); ArcStr::from(__mm_s) }).clone(), (makefileStr.clone()).clone())?;
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("config.log")); ArcStr::from(__mm_s) }).clone(), (literal!("Using cached values for static platform")).clone())?;
            cmd = (literal!("cached values")).clone();
            false
        },
        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } => {
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd \"")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources\" && ./configure --host=")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*platform.clone()); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!(" CFLAGS=")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!("-Os")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!(" CPPFLAGS=")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*CPPFLAGS.clone()); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!(" LDFLAGS= && ")); __mm_s.push_str(&*nozip.clone()); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCallRestrictedEnv((cmd.clone()).clone(), (logfile.clone()).clone())? {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![(System::readFile((logfile.clone()).clone())?).clone()])?;
                System::removeFile((logfile.clone()).clone());
                bail!("fail");
            }
            true
        },
        Deref @ metamodelica::List::Cons { head: host, tail: Deref @ metamodelica::List::Cons { head: Deref @ "docker", tail: Deref @ metamodelica::List::Cons { head: Deref @ "run", tail: rest } } } => {
            uid = System::getuid();
            cmd = (literal!("docker volume create")).clone();
            if 0 != System::systemCall((cmd.clone()).clone(), (logfile.clone()).clone()) {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(" failed:\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            } else if verbose.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            cidFile = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!(".cidfile")); ArcStr::from(__mm_s) }).clone();
            if System::regularFileExists((cidFile.clone()).clone()) {
                System::removeFile((cidFile.clone()).clone());
            }
            volumeID = (System::trim((System::readFile((logfile.clone()).clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone();
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run --cidfile ")); __mm_s.push_str(&*cidFile.clone()); __mm_s.push_str(&*literal!(" -v ")); __mm_s.push_str(&*volumeID.clone()); __mm_s.push_str(&*literal!(":/data busybox true")); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCall((cmd.clone()).clone(), (logfile.clone()).clone()) {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(" failed:\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                bail!("fail");
            } else if verbose.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            containerID = (System::trim((System::readFile((cidFile.clone()).clone())?).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone())).clone();
            System::removeFile((cidFile.clone()).clone());
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(":/data")); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCall((cmd.clone()).clone(), (logfile.clone()).clone()) {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(" failed:\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker rm ")); __mm_s.push_str(&*containerID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                bail!("fail");
            } else if verbose.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*includeDefaultFmi.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(":/data/fmiInclude")); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCall((cmd.clone()).clone(), (logfile.clone()).clone()) {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(" failed:\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker rm ")); __mm_s.push_str(&*containerID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                bail!("fail");
            } else if verbose.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker run ")); __mm_s.push_str(&*if (uid.clone() != 0) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("--user ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", uid.clone()))); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*literal!(" --rm -w /fmu -v ")); __mm_s.push_str(&*volumeID.clone()); __mm_s.push_str(&*literal!(":/fmu ")); __mm_s.push_str(&*stringDelimitList(rest.clone(), (literal!(" ")).clone())); __mm_s.push_str(&*literal!(" sh -c ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!("/fmu/")); __mm_s.push_str(&*System::basename((fmutmp.clone()).clone())); __mm_s.push_str(&*literal!("/sources")); __mm_s.push_str(&*dquote.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*literal!("./configure --host=")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*host.clone()); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!(" CFLAGS=")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!("-Os")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!(" CPPFLAGS=-I/fmu/fmiInclude LDFLAGS= && ")); __mm_s.push_str(&*nozip.clone()); __mm_s.push_str(&*dquote.clone()); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCall((cmd.clone()).clone(), (logfile.clone()).clone()) {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(":\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                System::removeFile((logfile.clone()).clone());
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker rm ")); __mm_s.push_str(&*containerID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
                bail!("fail");
            } else if verbose.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker cp ")); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*containerID.clone()); __mm_s.push_str(&*literal!(":/data/")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*quote.clone()); __mm_s.push_str(&*literal!(" .")); ArcStr::from(__mm_s) }).clone();
            if 0 != System::systemCall((cmd.clone()).clone(), (logfile.clone()).clone()) {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!(":\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); ArcStr::from(__mm_s) }).clone()])?;
                bail!("fail");
            } else if verbose.clone() {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmd.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*System::readFile((logfile.clone()).clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker rm ")); __mm_s.push_str(&*containerID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
            System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("docker volume rm ")); __mm_s.push_str(&*volumeID.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
            true
        },
        _ => {
            Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![(literal!("Unknown platform (contains spaces but does does not conform to \"platform docker run [args] container\"")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ExecStat::execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildModelFMU: configured platform ")); __mm_s.push_str(&*platform.clone()); __mm_s.push_str(&*literal!(" using ")); __mm_s.push_str(&*cmd.clone()); ArcStr::from(__mm_s) }).clone())?;
    if !(finishedBuild.clone()) {
        if !(isWindows.clone()) {
            if 0 != System::systemCallRestrictedEnv(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd ")); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!(" && ")); __mm_s.push_str(&*arcstr::literal!(Autoconf::make)); __mm_s.push_str(&*literal!(" clean > /dev/null 2>&1")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone())? {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![(literal!("Failed to make clean")).clone()])?;
                bail!("fail");
            }
        }
        if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd \"")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources\" && ")); __mm_s.push_str(&*nozip.clone()); ArcStr::from(__mm_s) }).clone(), (logfile.clone()).clone()) {
            Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![(System::readFile((logfile.clone()).clone())?).clone()])?;
            System::removeFile((logfile.clone()).clone());
            bail!("fail");
        }
    }
    Ok(())
}

fn translateModelFMU(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut className: Arc<Absyn::Path>, mut FMUVersion: ArcStr, mut inFMUType: ArcStr, mut inFileNamePrefix: ArcStr, mut addDummy: bool, mut platforms: Arc<metamodelica::List<ArcStr>>, mut inSimSettings: Option<SimCode::SimulationSettings>) -> Result<(bool, FCore::Cache, Arc<Values::Value>)> {
    let mut success: bool = false;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    if isProtectedContentAccess(className.clone())? {
        cache = inCache.clone();
        outValue = Arc::new(Values::Value::STRING { string: (literal!("")).clone() });
    } else {
        flags = loadCommandLineOptionsFromModel(className.clone())?;
        match '__try0: {
            (success, cache, outValue) = unwrap_break_err!(callTranslateModelFMU(inCache.clone(), inEnv.clone(), className.clone(), (FMUVersion.clone()).clone(), (inFMUType.clone()).clone(), (inFileNamePrefix.clone()).clone(), addDummy.clone(), platforms.clone(), inSimSettings.clone()), '__try0);
            FlagsUtil::saveFlags(flags.clone());
            Ok::<_, anyhow::Error>((cache.clone(), outValue.clone(), success.clone()))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                cache = __try0_o0;
                outValue = __try0_o1;
                success = __try0_o2;
            }
            Err(_) => {
                FlagsUtil::saveFlags(flags.clone());
                bail!("fail");
            }
        }
    }
    Ok((success, cache, outValue))
}

fn buildModelFMU(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut className: Arc<Absyn::Path>, mut FMUVersion: ArcStr, mut inFMUType: ArcStr, mut inFileNamePrefix: ArcStr, mut addDummy: bool, mut platforms: Arc<metamodelica::List<ArcStr>>, mut inSimSettings: Option<SimCode::SimulationSettings>) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    if isProtectedContentAccess(className.clone())? {
        cache = inCache.clone();
        outValue = Arc::new(Values::Value::STRING { string: (literal!("")).clone() });
    } else {
        flags = loadCommandLineOptionsFromModel(className.clone())?;
        match '__try0: {
            (cache, outValue) = unwrap_break_err!(callBuildModelFMU(inCache.clone(), inEnv.clone(), className.clone(), (FMUVersion.clone()).clone(), (inFMUType.clone()).clone(), (inFileNamePrefix.clone()).clone(), addDummy.clone(), platforms.clone(), inSimSettings.clone()), '__try0);
            FlagsUtil::saveFlags(flags.clone());
            Ok::<_, anyhow::Error>((cache.clone(), outValue.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                cache = __try0_o0;
                outValue = __try0_o1;
            }
            Err(_) => {
                FlagsUtil::saveFlags(flags.clone());
                bail!("fail");
            }
        }
    }
    Ok((cache, outValue))
}

fn buildEncryptedPackage(mut className: Arc<Absyn::Path>, mut encrypt: bool, mut inProgram: Absyn::Program) -> Result<bool> {
    let mut success: bool = false;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut logFile: ArcStr = arcstr::literal!("");
    let mut omhome: ArcStr = arcstr::literal!("");
    let mut pd: ArcStr = arcstr::literal!("");
    let mut ext: ArcStr = arcstr::literal!("");
    let mut packageTool: ArcStr = arcstr::literal!("");
    let mut packageToolArgs: ArcStr = arcstr::literal!("");
    let mut command: ArcStr = arcstr::literal!("");
    let mut runCommand: bool = false;
    let mut molName: ArcStr = arcstr::literal!("");
    let mut dirPath: ArcStr = arcstr::literal!("");
    let mut rmCommand: ArcStr = arcstr::literal!("");
    let mut cdCommand: ArcStr = arcstr::literal!("");
    let mut mvCommand: ArcStr = arcstr::literal!("");
    let mut dirOrFileName: ArcStr = arcstr::literal!("");
    let mut zipCommand: ArcStr = arcstr::literal!("");
    cls = InteractiveUtil::getPathedClassInProgram(className.clone(), inProgram.clone(), false, false)?;
    fileName = (AbsynUtil::classFilename(cls.clone())?).clone();
    logFile = (literal!("buildEncryptedPackage.log")).clone();
    runCommand = true;
    if System::regularFileExists((fileName.clone()).clone()) {
        omhome = (Settings::getInstallationDirectoryPath()?).clone();
        pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
        ext = (if (arcstr::literal!(Autoconf::os) == literal!("Windows_NT")) {literal!(".exe")} else {literal!("")}).clone();
        if encrypt.clone() {
            packageTool = stringAppendList(list![(omhome.clone()).clone(), (pd.clone()).clone(), (literal!("bin")).clone(), (pd.clone()).clone(), (literal!("omc-semla")).clone(), (pd.clone()).clone(), (literal!("packagetool")).clone(), (ext.clone()).clone()]);
            if System::regularFileExists((packageTool.clone()).clone()) {
                packageToolArgs = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-librarypath \"")); __mm_s.push_str(&*System::dirname((fileName.clone()).clone())); __mm_s.push_str(&*literal!("\" -version \"1.0\" -language \"3.2\" -encrypt \"")); __mm_s.push_str(&*boolString(encrypt.clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                command = stringAppendList(list![(literal!("\"")).clone(), (packageTool.clone()).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (packageToolArgs.clone()).clone()]);
            } else {
                Error::addMessage(Error::ENCRYPTION_NOT_SUPPORTED.clone(), list![(packageTool.clone()).clone()])?;
                success = false;
                runCommand = false;
            }
        } else {
            molName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(".mol")); ArcStr::from(__mm_s) }).clone();
            dirPath = (System::dirname((fileName.clone()).clone())).clone();
            rmCommand = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rm -f \"")); __mm_s.push_str(&*molName.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
            cdCommand = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd \"")); __mm_s.push_str(&*dirPath.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
            mvCommand = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mv \"")); __mm_s.push_str(&*molName.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
            if StringUtil::endsWith((fileName.clone()).clone(), (literal!("package.mo")).clone()) {
                dirOrFileName = (System::basename((dirPath.clone()).clone())).clone();
                zipCommand = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("zip -r \"")); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*molName.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*dirOrFileName.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                command = stringAppendList(list![(rmCommand.clone()).clone(), (literal!(" && ")).clone(), (cdCommand.clone()).clone(), (literal!(" && cd .. && ")).clone(), (zipCommand.clone()).clone()]);
            } else {
                dirOrFileName = (System::basename((fileName.clone()).clone())).clone();
                zipCommand = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("zip -r \"")); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*molName.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*dirOrFileName.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
                command = stringAppendList(list![(rmCommand.clone()).clone(), (literal!(" && ")).clone(), (cdCommand.clone()).clone(), (literal!(" && ")).clone(), (zipCommand.clone()).clone()]);
            }
        }
        if runCommand.clone() {
            if System::regularFileExists((logFile.clone()).clone()) {
                System::removeFile((logFile.clone()).clone());
            }
            success = 0 == System::systemCall((command.clone()).clone(), (logFile.clone()).clone());
            if !(success.clone()) {
                Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Command failed: ")); __mm_s.push_str(&*command.clone()); ArcStr::from(__mm_s) }).clone())?;
            }
        }
    } else {
        Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![(fileName.clone()).clone()])?;
        success = false;
    }
    Ok(success)
}

fn translateModelXML(mut cache: FCore::Cache, mut env: FCore::Graph, mut className: Arc<Absyn::Path>, mut fileNamePrefix: ArcStr, mut addDummy: bool, mut inSimSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut success: bool = false;
    if isProtectedContentAccess(className.clone())? {
        outValue = Arc::new(Values::Value::STRING { string: (literal!("")).clone() });
    } else {
        (success, cache) = SimCodeMain::translateModel(crate::SimCodeMain::TranslateModelKind::XML, cache.clone(), env.clone(), className.clone(), (fileNamePrefix.clone()).clone(), true, false, true, inSimSettingsOpt.clone(), Absyn::emptyFunctionArgs.clone())?;
        outValue = Arc::new(Values::Value::STRING { string: (if (success.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*if (!(Testsuite::isRunning()?)) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".xml")); ArcStr::from(__mm_s) }} else {literal!("")}).clone() });
    }
    Ok((cache, outValue))
}

pub fn translateGraphics(mut className: Arc<Absyn::Path>, mut inMsg: Absyn::Msg) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (className.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut retStr: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut refactoredClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut within_: Absyn::Within = Absyn::Within::TOP;
                    p = SymbolTable::getAbsyn();
                    cls = InteractiveUtil::getPathedClassInProgram(className.clone(), p.clone(), false, false)?;
                    refactoredClass = Refactor::refactorGraphicalAnnotation(p.clone(), cls.clone())?;
                    within_ = InteractiveUtil::buildWithin(className.clone())?;
                    SymbolTable::setAbsyn(InteractiveUtil::updateProgram(Absyn::Program { classes: list![refactoredClass.clone()], within_: within_.clone() }, p.clone(), false)?)?;
                    s1 = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
                    retStr = stringAppendList(list![(literal!("Translation of ")).clone(), (s1.clone()).clone(), (literal!(" successful.\n")).clone()]);
                    Ok(Arc::new(Values::Value::STRING { string: (retStr.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut errorMsg: ArcStr = arcstr::literal!("");
                    let mut strEmpty: bool = false;
                    errorMsg = (Error::printMessagesStr(false)).clone();
                    strEmpty = stringCompare((literal!("")).clone(), (errorMsg.clone()).clone()) == 0;
                    errorMsg = (if (strEmpty.clone()) {literal!("Internal error, translating graphics to new version")} else {errorMsg.clone()}).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (errorMsg.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn calculateSimulationSettings(mut inCache: FCore::Cache, mut vals: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<(FCore::Cache, SimCode::SimulationSettings)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outSimSettings: SimCode::SimulationSettings;
    (outCache, outSimSettings) = (::match_deref::match_deref! { match &((inCache.clone(), vals.clone())) {
        (cache, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: _ } }, tail: Deref @ metamodelica::List::Cons { head: starttime_v, tail: Deref @ metamodelica::List::Cons { head: stoptime_v, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: interval_i }, tail: Deref @ metamodelica::List::Cons { head: tolerance_v, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: method_str }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: options_str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: outputFormat_str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: variableFilter_str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cflags }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: simflags }, tail: Deref @ metamodelica::List::Nil } } } } } } } } } } } }) => {
            let mut starttime_r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut stoptime_r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut tolerance_r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            starttime_r = ValuesUtil::valueReal(starttime_v.clone())?;
            stoptime_r = ValuesUtil::valueReal(stoptime_v.clone())?;
            tolerance_r = ValuesUtil::valueReal(tolerance_v.clone())?;
            outSimSettings = SimCodeMain::createSimulationSettings(starttime_r.clone(), stoptime_r.clone(), interval_i.clone(), tolerance_r.clone(), (method_str.clone()).clone(), (options_str.clone()).clone(), (outputFormat_str.clone()).clone(), (variableFilter_str.clone()).clone(), (cflags.clone()).clone(), (simflags.clone()).clone());
            (cache.clone(), outSimSettings.clone())
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.calculateSimulationSettings failed: ")); __mm_s.push_str(&*ValuesDump::valString(Arc::new(Values::Value::TUPLE { valueLst: vals.clone() }))?); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outSimSettings))
}

fn getListFirstShowError(mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut errorMessage: ArcStr) -> Result<(Arc<Values::Value>, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut restValues: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    (outValue, restValues) = (::match_deref::match_deref! { match &((inValues.clone(), errorMessage.clone())) {
        (Deref @ metamodelica::List::Cons { head: v, tail: rest }, _) => {
            (v.clone(), rest.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(errorMessage.clone()).clone()])?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outValue, restValues))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getListNthShowError(mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut errorMessage: ArcStr, mut currentElement: i32, mut nthElement: i32) -> Result<Arc<Values::Value>> {
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (inValues.clone(), errorMessage.clone(), currentElement.clone(), nthElement.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, _, i, n) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut rest: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let true = (i.clone() < n.clone()) else { bail!("pattern mismatch") };
                    (_, rest) = getListFirstShowError(lst.clone(), (errorMessage.clone()).clone())?;
                    v = getListNthShowError(rest.clone(), (errorMessage.clone()).clone(), i.clone() + 1, n.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lst, _, _, _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (v, _) = getListFirstShowError(lst.clone(), (errorMessage.clone()).clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outValue)
}

fn moveClass(mut inClassName: Arc<Absyn::Path>, mut inOffset: i32, mut inProgram: Absyn::Program) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut outSuccess: bool = false;
    let mut parent_cls: Arc<Absyn::Path>;
    let mut cls_name: ArcStr = arcstr::literal!("");
    if inOffset.clone() == 0 {
        outProgram = inProgram.clone();
        outSuccess = true;
        return (outProgram, outSuccess);
    }
    match '__try0: {
        if AbsynUtil::pathIsIdent(inClassName.clone()) {
            outProgram = unwrap_break_err!(moveClassInProgram((AbsynUtil::pathFirstIdent(inClassName.clone()).unwrap()).clone(), inOffset.clone(), inProgram.clone()), '__try0);
        } else {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::splitQualAndIdentPath(inClassName.clone()), '__try0)) {
                (__pa1, Deref @ Absyn::Path::IDENT { name: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            parent_cls = __pa1.clone();
            cls_name = __pa2.clone();
            outProgram = unwrap_break_err!(Interactive::transformPathedClassInProgram(parent_cls.clone(), inProgram.clone(), Arc::new({ let __pe_b0 = (cls_name.clone()).clone(); let __pe_b1 = inOffset.clone(); move |__pe_a2| moveClassInClass(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) })), '__try0);
        }
        outSuccess = true;
        Ok::<_, anyhow::Error>((outProgram.clone(), outSuccess.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outSuccess = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outSuccess = false;
        }
    }
    (outProgram, outSuccess)
}

fn moveClassToTop(mut inClassName: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program = inProgram.clone();
    let mut outSuccess: bool = false;
    let mut parent_cls: Arc<Absyn::Path>;
    let mut cls_name: ArcStr = arcstr::literal!("");
    match '__try0: {
        if AbsynUtil::pathIsIdent(inClassName.clone()) {
            outProgram = (match outProgram.clone() {
        Absyn::Program { .. } => {
            let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
            let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue((AbsynUtil::pathFirstIdent(inClassName.clone()).unwrap()).clone(), outProgram.classes.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isClassNamed, ArcStr, Arc<Absyn::Class>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::Class>) -> Result<bool> + 'static>)), '__try0)) {
                (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            classes = __pa0.clone();
            cls = __pa1.clone();
            outProgram.classes = cons(cls.clone(), classes.clone());
            outProgram.clone()
        },
        _ => bail!("match: no arm matched"),
    });
        } else {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::splitQualAndIdentPath(inClassName.clone()), '__try0)) {
                (__pa1, Deref @ Absyn::Path::IDENT { name: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            parent_cls = __pa1.clone();
            cls_name = __pa2.clone();
            outProgram = unwrap_break_err!(Interactive::transformPathedClassInProgram(parent_cls.clone(), inProgram.clone(), Arc::new({ let __pe_b0 = (cls_name.clone()).clone(); move |__pe_a1| moveClassToTopInClass(__pe_b0.clone(), __pe_a1) })), '__try0);
        }
        outSuccess = true;
        Ok::<_, anyhow::Error>((outSuccess.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outSuccess = __try0_o0;
        }
        Err(_) => {
            outSuccess = false;
        }
    }
    (outProgram, outSuccess)
}

fn moveClassToBottom(mut inClassName: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program = inProgram.clone();
    let mut outSuccess: bool = false;
    let mut parent_cls: Arc<Absyn::Path>;
    let mut cls_name: ArcStr = arcstr::literal!("");
    match '__try0: {
        if AbsynUtil::pathIsIdent(inClassName.clone()) {
            outProgram = (match outProgram.clone() {
        Absyn::Program { .. } => {
            let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
            let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(unwrap_break_err!(List::deleteMemberOnTrue((AbsynUtil::pathFirstIdent(inClassName.clone()).unwrap()).clone(), outProgram.classes.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isClassNamed, ArcStr, Arc<Absyn::Class>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::Class>) -> Result<bool> + 'static>)), '__try0)) {
                (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            classes = __pa0.clone();
            cls = __pa1.clone();
            outProgram.classes = listAppend(classes.clone(), list![cls.clone()]);
            outProgram.clone()
        },
        _ => bail!("match: no arm matched"),
    });
        } else {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::splitQualAndIdentPath(inClassName.clone()), '__try0)) {
                (__pa1, Deref @ Absyn::Path::IDENT { name: __pa2 }) => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            parent_cls = __pa1.clone();
            cls_name = __pa2.clone();
            outProgram = unwrap_break_err!(Interactive::transformPathedClassInProgram(parent_cls.clone(), inProgram.clone(), Arc::new({ let __pe_b0 = (cls_name.clone()).clone(); move |__pe_a1| moveClassToBottomInClass(__pe_b0.clone(), __pe_a1) })), '__try0);
        }
        outSuccess = true;
        Ok::<_, anyhow::Error>((outSuccess.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outSuccess = __try0_o0;
        }
        Err(_) => {
            outSuccess = false;
        }
    }
    (outProgram, outSuccess)
}

fn moveClassInProgram(mut inName: ArcStr, mut inOffset: i32, mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = inProgram.clone();
    outProgram = (match outProgram.clone() {
        Absyn::Program { .. } => {
            outProgram.classes = moveClassInClassList((inName.clone()).clone(), inOffset.clone(), outProgram.classes.clone())?;
            outProgram.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outProgram)
}

fn moveClassInClassList(mut inName: ArcStr, mut inOffset: i32, mut inClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Class>>>> {
    let mut outClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut acc: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Absyn::Class>>> = inClasses.clone();
    let mut name: ArcStr = arcstr::literal!("");
    let mut offset: i32 = 0;
    loop {
        let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa1 @ Deref @ Absyn::Class { name: __pa0, .. }, tail: __pa2 } => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
            _ => bail!("pattern mismatch"),
        } };
        name = __pa0.clone();
        cls = __pa1.clone();
        rest = __pa2.clone();
        if name.clone() == inName.clone() {
            break;
        } else {
            acc = cons(cls.clone(), acc.clone());
        }
    }
    if inOffset.clone() > 0 {
        offset = std::cmp::min(inOffset.clone(), (rest.clone().len() as i32));
        for mut i in 1..=offset.clone() {
            acc = cons(listHead(rest.clone())?, acc.clone());
            rest = listRest(rest.clone())?;
        }
    } else {
        offset = std::cmp::max(inOffset.clone(), -((acc.clone().len() as i32)));
        for mut i in offset.clone()..=-1 {
            rest = cons(listHead(acc.clone())?, rest.clone());
            acc = listRest(acc.clone())?;
        }
    }
    outClasses = List::append_reverse(acc.clone(), cons(cls.clone(), rest.clone()));
    Ok(outClasses)
}

fn moveClassInClass(mut inName: ArcStr, mut inOffset: i32, mut inClass: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut body: Arc<Absyn::ClassDef>;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    body = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = moveClassInClassParts((inName.clone()).clone(), inOffset.clone(), var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone())?);
            body.clone()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = moveClassInClassParts((inName.clone()).clone(), inOffset.clone(), var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?);
            body.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    outClass = AbsynUtil::setClassBody(inClass.clone(), body.clone())?;
    Ok(outClass)
}

fn moveClassInClassParts(mut inName: ArcStr, mut inOffset: i32, mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = inClassParts.clone();
    let mut part: Arc<Absyn::ClassPart>;
    let mut acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = inClassParts.clone();
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut cls: Option<Arc<Absyn::ElementItem>> = None;
    let mut offset: i32 = 0;
    let mut is_public: bool = false;
    let mut is_empty: bool = false;
    loop {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa0.clone();
        rest = __pa1.clone();
        (part, cls, offset, is_public) = moveClassInClassPart((inName.clone()).clone(), inOffset.clone(), part.clone())?;
        if isSome(cls.clone()) {
            break;
        } else {
            acc = cons(part.clone(), acc.clone());
        }
    }
    is_empty = AbsynUtil::isEmptyClassPart(part.clone());
    parts = if (offset.clone() > 0) {rest.clone()} else {acc.clone()};
    if parts.clone().is_empty() && offset.clone() != 0 {
        parts = moveClassInClassParts3(Util::getOption(cls.clone())?, offset.clone() < 0, is_public.clone(), part.clone(), parts.clone())?;
    } else {
        parts = moveClassInClassParts2(Util::getOption(cls.clone())?, offset.clone(), is_public.clone(), parts.clone())?;
        if !(is_empty.clone()) {
            parts = cons(part.clone(), parts.clone());
        }
    }
    if offset.clone() > 0 {
        rest = parts.clone();
    } else {
        acc = parts.clone();
    }
    if is_empty.clone() && !(rest.clone().is_empty()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa2.clone();
        rest = __pa3.clone();
        acc = mergeClassPartWithList(part.clone(), acc.clone());
    }
    outClassParts = List::append_reverse(acc.clone(), rest.clone());
    Ok(outClassParts)
}

fn mergeClassPartWithList(mut inClassPart: Arc<Absyn::ClassPart>, mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut part: Arc<Absyn::ClassPart>;
    let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outClassParts = (::match_deref::match_deref! { match &((inClassPart.clone(), inClassParts.clone())) {
        (Deref @ Absyn::ClassPart::PUBLIC { .. }, Deref @ metamodelica::List::Cons { head: part @ Deref @ Absyn::ClassPart::PUBLIC { .. }, tail: rest }) => cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: listAppend(var_field!((**part).contents, Absyn::ClassPart::PUBLIC).clone(), var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone()) }), rest.clone()),
        (Deref @ Absyn::ClassPart::PROTECTED { .. }, Deref @ metamodelica::List::Cons { head: part @ Deref @ Absyn::ClassPart::PROTECTED { .. }, tail: rest }) => cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: listAppend(var_field!((**part).contents, Absyn::ClassPart::PROTECTED).clone(), var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone()) }), rest.clone()),
        _ => cons(inClassPart.clone(), inClassParts.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outClassParts
}

fn moveClassInClassParts2(mut inClass: Arc<Absyn::ElementItem>, mut inOffset: i32, mut inIsPublic: bool, mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut part: Arc<Absyn::ClassPart>;
    let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = inClassParts.clone();
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut offset: i32 = inOffset.clone();
    let mut moved: bool = false;
    while offset.clone() != 0 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa0.clone();
        rest = __pa1.clone();
        (parts, offset, moved) = moveClassInClassPart3(inClass.clone(), offset.clone(), inIsPublic.clone(), part.clone())?;
        if rest.clone().is_empty() && !(moved.clone()) {
            acc = moveClassInClassParts3(inClass.clone(), inOffset.clone() > 0, inIsPublic.clone(), part.clone(), acc.clone())?;
            break;
        } else if offset.clone() == 0 && !(moved.clone()) {
            acc = listAppend(parts.clone(), acc.clone());
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            part = __pa2.clone();
            rest = __pa3.clone();
            acc = moveClassInClassParts3(inClass.clone(), inOffset.clone() > 0, inIsPublic.clone(), part.clone(), acc.clone())?;
            break;
        }
        acc = listAppend(if (inOffset.clone() > 0) {parts.clone()} else {parts.clone().reverse()}, acc.clone());
    }
    outClassParts = List::append_reverse(acc.clone(), rest.clone());
    Ok(outClassParts)
}

fn moveClassInClassParts3(mut inClass: Arc<Absyn::ElementItem>, mut inPositiveOffset: bool, mut inIsPublic: bool, mut inClassPart: Arc<Absyn::ClassPart>, mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outClassParts = (::match_deref::match_deref! { match &((inPositiveOffset.clone(), inIsPublic.clone(), inClassPart.clone())) {
        (true, true, Deref @ Absyn::ClassPart::PUBLIC { .. }) => cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: cons(inClass.clone(), var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone()) }), inClassParts.clone()),
        (true, false, Deref @ Absyn::ClassPart::PROTECTED { .. }) => cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: cons(inClass.clone(), var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone()) }), inClassParts.clone()),
        (false, true, Deref @ Absyn::ClassPart::PUBLIC { .. }) => cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: listAppend(var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone(), list![inClass.clone()]) }), inClassParts.clone()),
        (false, false, Deref @ Absyn::ClassPart::PROTECTED { .. }) => cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: listAppend(var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone(), list![inClass.clone()]) }), inClassParts.clone()),
        (_, true, _) => cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![inClass.clone()] }), cons(inClassPart.clone(), inClassParts.clone())),
        (_, false, _) => cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: list![inClass.clone()] }), cons(inClassPart.clone(), inClassParts.clone())),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClassParts)
}

fn moveClassInClassPart(mut inName: ArcStr, mut inOffset: i32, mut inClassPart: Arc<Absyn::ClassPart>) -> Result<(Arc<Absyn::ClassPart>, Option<Arc<Absyn::ElementItem>>, i32, bool)> {
    let mut outClassPart: Arc<Absyn::ClassPart> = inClassPart.clone();
    let mut outClass: Option<Arc<Absyn::ElementItem>> = None;
    let mut outRemainingOffset: i32 = 0;
    let mut outIsPublic: bool = false;
    let mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    (outClassPart, outClass, outRemainingOffset, outIsPublic) = (::match_deref::match_deref! { match &(outClassPart.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            (elements, outClass, outRemainingOffset) = moveClassInClassPart2((inName.clone()).clone(), inOffset.clone(), var_field!((*outClassPart).contents, Absyn::ClassPart::PUBLIC).clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = elements.clone());
            (outClassPart.clone(), outClass.clone(), outRemainingOffset.clone(), true)
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            (elements, outClass, outRemainingOffset) = moveClassInClassPart2((inName.clone()).clone(), inOffset.clone(), var_field!((*outClassPart).contents, Absyn::ClassPart::PROTECTED).clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = elements.clone());
            (outClassPart.clone(), outClass.clone(), outRemainingOffset.clone(), false)
        },
        _ => (outClassPart.clone(), None, inOffset.clone(), false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassPart, outClass, outRemainingOffset, outIsPublic))
}

fn moveClassInClassPart2(mut inName: ArcStr, mut inOffset: i32, mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, Option<Arc<Absyn::ElementItem>>, i32)> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut outClass: Option<Arc<Absyn::ElementItem>> = None;
    let mut outRemainingOffset: i32 = 0;
    let mut e: Arc<Absyn::ElementItem>;
    let mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = inElements.clone();
    let mut acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    while !(elements.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elements.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        elements = __pa1.clone();
        if AbsynUtil::isElementItemClassNamed((inName.clone()).clone(), e.clone()) {
            outClass = Some(e.clone());
            break;
        } else {
            acc = cons(e.clone(), acc.clone());
        }
    }
    if isNone(outClass.clone()) {
        outElements = inElements.clone();
        outRemainingOffset = inOffset.clone();
        return Ok((outElements, outClass, outRemainingOffset));
    }
    (acc, elements, outRemainingOffset, _) = moveClassInSplitClassPart(inOffset.clone(), acc.clone(), elements.clone())?;
    if outRemainingOffset.clone() == 0 {
        elements = cons(e.clone(), elements.clone());
    }
    outElements = List::append_reverse(acc.clone(), elements.clone());
    Ok((outElements, outClass, outRemainingOffset))
}

fn makeClassPart(mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inPublic: bool) -> Arc<Absyn::ClassPart> {
    let mut outPart: Arc<Absyn::ClassPart> = if (inPublic.clone()) {Arc::new(Absyn::ClassPart::PUBLIC { contents: inElements.clone() })} else {Arc::new(Absyn::ClassPart::PROTECTED { contents: inElements.clone() })};
    outPart
}

fn moveClassInClassPart3(mut inClass: Arc<Absyn::ElementItem>, mut inOffset: i32, mut inIsPublic: bool, mut inClassPart: Arc<Absyn::ClassPart>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, i32, bool)> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut outRemainingOffset: i32 = 0;
    let mut outMoved: bool = false;
    let mut same_part_type: bool = false;
    let mut reached_end: bool = false;
    let mut elems_before: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut elems_after: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    (elems, same_part_type) = (::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => (var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone(), inIsPublic.clone()),
        Deref @ Absyn::ClassPart::PROTECTED { .. } => (var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone(), !(inIsPublic.clone())),
        _ => bail!("match: no arm matched"),
    } });
    if inOffset.clone() > 0 {
        elems_before = metamodelica::nil();
        elems_after = elems.clone();
    } else {
        elems_before = elems.clone();
        elems_after = metamodelica::nil();
    }
    (elems_before, elems_after, outRemainingOffset, reached_end) = moveClassInSplitClassPart(inOffset.clone(), elems_before.clone().reverse(), elems_after.clone())?;
    if outRemainingOffset.clone() == 0 {
        if same_part_type.clone() {
            elems = List::append_reverse(elems_before.clone(), cons(inClass.clone(), elems_after.clone()));
            outClassParts = list![makeClassPart(elems.clone(), inIsPublic.clone())];
            outMoved = true;
        } else if !(reached_end.clone()) {
            outClassParts = if (elems_before.clone().is_empty()) {metamodelica::nil()} else {list![makeClassPart(elems_before.clone().reverse(), !(inIsPublic.clone()))]};
            outClassParts = cons(makeClassPart(list![inClass.clone()], inIsPublic.clone()), outClassParts.clone());
            if !(elems_after.clone().is_empty()) {
                outClassParts = cons(makeClassPart(elems_after.clone(), !(inIsPublic.clone())), outClassParts.clone());
            }
            outMoved = true;
        } else {
            outClassParts = list![inClassPart.clone()];
        }
    } else {
        outClassParts = list![inClassPart.clone()];
    }
    Ok((outClassParts, outRemainingOffset, outMoved))
}

fn moveClassInSplitClassPart(mut inOffset: i32, mut inElementsBefore: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inElementsAfter: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, i32, bool)> {
    let mut outElementsBefore: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = inElementsBefore.clone();
    let mut outElementsAfter: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = inElementsAfter.clone();
    let mut outRemainingOffset: i32 = inOffset.clone();
    let mut outReachedEnd: bool = false;
    let mut e: Arc<Absyn::ElementItem>;
    if inOffset.clone() > 0 {
        while outRemainingOffset.clone() > 0 {
            if outElementsAfter.clone().is_empty() {
                break;
            } else {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(outElementsAfter.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa0.clone();
                outElementsAfter = __pa1.clone();
                outElementsBefore = cons(e.clone(), outElementsBefore.clone());
                if AbsynUtil::isElementItemClass(e.clone()) {
                    outRemainingOffset = outRemainingOffset.clone() - 1;
                }
            }
        }
        outReachedEnd = outElementsAfter.clone().is_empty();
    } else {
        while outRemainingOffset.clone() < 0 {
            if outElementsBefore.clone().is_empty() {
                break;
            } else {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(outElementsBefore.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                e = __pa2.clone();
                outElementsBefore = __pa3.clone();
                outElementsAfter = cons(e.clone(), outElementsAfter.clone());
                if AbsynUtil::isElementItemClass(e.clone()) {
                    outRemainingOffset = outRemainingOffset.clone() + 1;
                }
            }
        }
        outReachedEnd = outElementsBefore.clone().is_empty();
    }
    Ok((outElementsBefore, outElementsAfter, outRemainingOffset, outReachedEnd))
}

fn deleteClassInClassPart(mut inName: ArcStr, mut inClassPart: Arc<Absyn::ClassPart>) -> Result<(Arc<Absyn::ClassPart>, Option<Arc<Absyn::ElementItem>>)> {
    let mut outClassPart: Arc<Absyn::ClassPart> = inClassPart.clone();
    let mut outClass: Option<Arc<Absyn::ElementItem>> = None;
    let mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    (outClassPart, outClass) = (::match_deref::match_deref! { match &(outClassPart.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            (elements, outClass) = List::deleteMemberOnTrue((inName.clone()).clone(), var_field!((*outClassPart).contents, Absyn::ClassPart::PUBLIC).clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isElementItemClassNamed, ArcStr, Arc<Absyn::ElementItem>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::ElementItem>) -> Result<bool> + 'static>))?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = elements.clone());
            (outClassPart.clone(), outClass.clone())
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            (elements, outClass) = List::deleteMemberOnTrue((inName.clone()).clone(), var_field!((*outClassPart).contents, Absyn::ClassPart::PROTECTED).clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::isElementItemClassNamed, ArcStr, Arc<Absyn::ElementItem>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::ElementItem>) -> Result<bool> + 'static>))?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = elements.clone());
            (outClassPart.clone(), outClass.clone())
        },
        _ => (outClassPart.clone(), None),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassPart, outClass))
}

fn moveClassToTopInClass(mut inName: ArcStr, mut inClass: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut body: Arc<Absyn::ClassDef>;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    body = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = moveClassToTopInClassParts((inName.clone()).clone(), var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone())?);
            body.clone()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = moveClassToTopInClassParts((inName.clone()).clone(), var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?);
            body.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    outClass = AbsynUtil::setClassBody(inClass.clone(), body.clone())?;
    Ok(outClass)
}

fn moveClassToTopInClassParts(mut inName: ArcStr, mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut part: Arc<Absyn::ClassPart>;
    let mut first: Arc<Absyn::ClassPart>;
    let mut acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = inClassParts.clone();
    let mut ocls: Option<Arc<Absyn::ElementItem>> = None;
    let mut cls: Arc<Absyn::ElementItem>;
    let mut is_public: bool = false;
    loop {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa0.clone();
        rest = __pa1.clone();
        (part, ocls) = deleteClassInClassPart((inName.clone()).clone(), part.clone())?;
        if isSome(ocls.clone()) {
            if !(AbsynUtil::isEmptyClassPart(part.clone())) || acc.clone().is_empty() || rest.clone().is_empty() {
                rest = cons(part.clone(), rest.clone());
            }
            outClassParts = List::append_reverse(acc.clone(), rest.clone());
            break;
        } else {
            acc = cons(part.clone(), acc.clone());
        }
    }
    let __pa2 = ::match_deref::match_deref! { match &(ocls.clone()) {
        Some(__pa2) => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls = __pa2.clone();
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(outClassParts.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    first = __pa3.clone();
    rest = __pa4.clone();
    outClassParts = (::match_deref::match_deref! { match &((first.clone(), part.clone())) {
        (Deref @ Absyn::ClassPart::PUBLIC { .. }, Deref @ Absyn::ClassPart::PUBLIC { .. }) => {
            assign_variant_field!(first => Absyn::ClassPart::PUBLIC; contents = cons(cls.clone(), var_field!((*first).contents, Absyn::ClassPart::PUBLIC).clone()));
            cons(first.clone(), rest.clone())
        },
        (Deref @ Absyn::ClassPart::PROTECTED { .. }, Deref @ Absyn::ClassPart::PROTECTED { .. }) => {
            assign_variant_field!(first => Absyn::ClassPart::PROTECTED; contents = cons(cls.clone(), var_field!((*first).contents, Absyn::ClassPart::PROTECTED).clone()));
            cons(first.clone(), rest.clone())
        },
        (_, Deref @ Absyn::ClassPart::PUBLIC { .. }) => cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![cls.clone()] }), cons(first.clone(), rest.clone())),
        (_, Deref @ Absyn::ClassPart::PROTECTED { .. }) => cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: list![cls.clone()] }), cons(first.clone(), rest.clone())),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClassParts)
}

fn moveClassToBottomInClass(mut inName: ArcStr, mut inClass: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut body: Arc<Absyn::ClassDef>;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    body = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = moveClassToBottomInClassParts((inName.clone()).clone(), var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone())?);
            body.clone()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = moveClassToBottomInClassParts((inName.clone()).clone(), var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?);
            body.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    outClass = AbsynUtil::setClassBody(inClass.clone(), body.clone())?;
    Ok(outClass)
}

fn moveClassToBottomInClassParts(mut inName: ArcStr, mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut part: Arc<Absyn::ClassPart>;
    let mut last: Arc<Absyn::ClassPart>;
    let mut acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = inClassParts.clone();
    let mut ocls: Option<Arc<Absyn::ElementItem>> = None;
    let mut cls: Arc<Absyn::ElementItem>;
    let mut is_public: bool = false;
    loop {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        part = __pa0.clone();
        rest = __pa1.clone();
        (part, ocls) = deleteClassInClassPart((inName.clone()).clone(), part.clone())?;
        if isSome(ocls.clone()) {
            break;
        } else {
            acc = cons(part.clone(), acc.clone());
        }
    }
    let __pa2 = ::match_deref::match_deref! { match &(ocls.clone()) {
        Some(__pa2) => __pa2.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls = __pa2.clone();
    if !(AbsynUtil::isEmptyClassPart(part.clone())) || rest.clone().is_empty() {
        rest = cons(part.clone(), rest.clone());
    }
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rest.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    last = __pa3.clone();
    rest = __pa4.clone();
    rest = (::match_deref::match_deref! { match &((last.clone(), part.clone())) {
        (Deref @ Absyn::ClassPart::PUBLIC { .. }, Deref @ Absyn::ClassPart::PUBLIC { .. }) => {
            assign_variant_field!(last => Absyn::ClassPart::PUBLIC; contents = listAppend(var_field!((*last).contents, Absyn::ClassPart::PUBLIC).clone(), list![cls.clone()]));
            cons(last.clone(), rest.clone())
        },
        (Deref @ Absyn::ClassPart::PROTECTED { .. }, Deref @ Absyn::ClassPart::PROTECTED { .. }) => {
            assign_variant_field!(last => Absyn::ClassPart::PROTECTED; contents = listAppend(var_field!((*last).contents, Absyn::ClassPart::PROTECTED).clone(), list![cls.clone()]));
            cons(last.clone(), rest.clone())
        },
        (_, Deref @ Absyn::ClassPart::PUBLIC { .. }) => cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![cls.clone()] }), cons(last.clone(), rest.clone())),
        (_, Deref @ Absyn::ClassPart::PROTECTED { .. }) => cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: list![cls.clone()] }), cons(last.clone(), rest.clone())),
        _ => bail!("match: no arm matched"),
    } });
    outClassParts = List::append_reverse(acc.clone(), rest.clone().reverse());
    Ok(outClassParts)
}

fn copyClass(mut inClass: Arc<Absyn::Class>, mut inName: ArcStr, mut inWithin: Absyn::Within, mut inClassPath: Arc<Absyn::Path>, mut inProg: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProg: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut orig_file: ArcStr = arcstr::literal!("");
    let mut dst_path: ArcStr = arcstr::literal!("");
    let mut cls_path: Arc<Absyn::Path> = inClassPath.clone();
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    orig_file = __pa0.clone();
    dst_path = ((match inWithin.clone() {
        Absyn::Within::TOP => literal!("<interactive>"),
        Absyn::Within::WITHIN { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedClassInProgram(var_field!(inWithin.path, Absyn::Within::WITHIN).clone(), inProg.clone(), false, false)?) {
                Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            dst_path = __pa0.clone();
            dst_path.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    cls = NFApi::updateMovedClassPaths(inClass.clone(), inClassPath.clone(), inWithin.clone())?;
    cls = moveClassInfo(cls.clone(), (dst_path.clone()).clone())?;
    cls = AbsynUtil::setClassName(cls.clone(), (inName.clone()).clone())?;
    outProg = InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: inWithin.clone() }, inProg.clone(), false)?;
    Ok(outProg)
}

fn moveSourceInfo(mut inInfo: SourceInfo, mut dstPath: ArcStr) -> Result<SourceInfo> {
    let mut outInfo: SourceInfo = inInfo.clone();
    let _ = (match outInfo.clone() {
        SourceInfo { .. } => {
            outInfo.fileName = dstPath.clone();
            outInfo.isReadOnly = false;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outInfo)
}

fn moveClassInfo(mut inClass: Arc<Absyn::Class>, mut dstPath: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let _ = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::Class { info: info @ SourceInfo { .. }, .. } => {
            assign_field!(
                outClass.body = moveClassDefInfo(outClass.body.clone(), (dstPath.clone()).clone())?,
                outClass.info = moveSourceInfo(info.clone(), (dstPath.clone()).clone())?
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

fn moveClassDefInfo(mut inClassDef: Arc<Absyn::ClassDef>, mut dstPath: ArcStr) -> Result<Arc<Absyn::ClassDef>> {
    let mut outClassDef: Arc<Absyn::ClassDef> = inClassDef.clone();
    let _ = (::match_deref::match_deref! { match &(outClassDef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::PARTS;
                classParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut cp in (var_field!((*outClassDef).classParts, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = moveClassPartInfo(cp.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*outClassDef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = moveAnnotationInfo(a.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::DERIVED;
                arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut e in (var_field!((*outClassDef).arguments, Absyn::ClassDef::DERIVED).clone()).into_iter().cloned() {
            let __x = moveElementArgInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = moveCommentInfo(var_field!((*outClassDef).comment, Absyn::ClassDef::DERIVED).clone(), (dstPath.clone()).clone())?
            );
            ()
        },
        Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::ENUMERATION; comment = moveCommentInfo(var_field!((*outClassDef).comment, Absyn::ClassDef::ENUMERATION).clone(), (dstPath.clone()).clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::OVERLOAD { .. } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::OVERLOAD; comment = moveCommentInfo(var_field!((*outClassDef).comment, Absyn::ClassDef::OVERLOAD).clone(), (dstPath.clone()).clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::CLASS_EXTENDS;
                modifications = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut e in (var_field!((*outClassDef).modifications, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = moveElementArgInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                parts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut cp in (var_field!((*outClassDef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = moveClassPartInfo(cp.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*outClassDef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = moveAnnotationInfo(a.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::ClassDef::PDER { .. } => {
            assign_variant_field!(outClassDef => Absyn::ClassDef::PDER; comment = moveCommentInfo(var_field!((*outClassDef).comment, Absyn::ClassDef::PDER).clone(), (dstPath.clone()).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outClassDef)
}

fn moveClassPartInfo(mut inPart: Arc<Absyn::ClassPart>, mut dstPath: ArcStr) -> Result<Arc<Absyn::ClassPart>> {
    let mut outPart: Arc<Absyn::ClassPart>;
    outPart = (::match_deref::match_deref! { match &(inPart.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { contents: el } => {
            Arc::new(Absyn::ClassPart::PUBLIC { contents: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = moveElementItemInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ Absyn::ClassPart::PROTECTED { contents: el } => {
            Arc::new(Absyn::ClassPart::PROTECTED { contents: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = moveElementItemInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ Absyn::ClassPart::EQUATIONS { contents: eq } => {
            Arc::new(Absyn::ClassPart::EQUATIONS { contents: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
        for mut e in (eq.clone()).into_iter().cloned() {
            let __x = moveEquationItemInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eq } => {
            Arc::new(Absyn::ClassPart::INITIALEQUATIONS { contents: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
        for mut e in (eq.clone()).into_iter().cloned() {
            let __x = moveEquationItemInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ Absyn::ClassPart::ALGORITHMS { contents: alg } => {
            Arc::new(Absyn::ClassPart::ALGORITHMS { contents: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
        for mut e in (alg.clone()).into_iter().cloned() {
            let __x = moveAlgorithmItemInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: alg } => {
            Arc::new(Absyn::ClassPart::INITIALALGORITHMS { contents: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
        for mut e in (alg.clone()).into_iter().cloned() {
            let __x = moveAlgorithmItemInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } })
        },
        Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: ext, annotation_: ann } => {
            let mut ext = (*ext).clone();
            let mut ann = (*ann).clone();
            ext = moveExternalDeclInfo(ext.clone(), (dstPath.clone()).clone())?;
            ann = moveAnnotationOptInfo(ann.clone(), (dstPath.clone()).clone())?;
            Arc::new(Absyn::ClassPart::EXTERNAL { externalDecl: ext.clone(), annotation_: ann.clone() })
        },
        _ => {
            inPart.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPart)
}

fn moveAnnotationOptInfo(mut inAnnotation: Option<Arc<Absyn::Annotation>>, mut dstPath: ArcStr) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut outAnnotation: Option<Arc<Absyn::Annotation>> = None;
    outAnnotation = (::match_deref::match_deref! { match &(inAnnotation.clone()) {
        Some(a) => {
            Some(moveAnnotationInfo(a.clone(), (dstPath.clone()).clone())?)
        },
        _ => {
            inAnnotation.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

fn moveAnnotationInfo(mut inAnnotation: Arc<Absyn::Annotation>, mut dstPath: ArcStr) -> Result<Arc<Absyn::Annotation>> {
    let mut outAnnotation: Arc<Absyn::Annotation> = inAnnotation.clone();
    assign_field!(outAnnotation.elementArgs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut e in (outAnnotation.elementArgs.clone()).into_iter().cloned() {
            let __x = moveElementArgInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outAnnotation)
}

fn moveElementItemInfo(mut inElement: Arc<Absyn::ElementItem>, mut dstPath: ArcStr) -> Result<Arc<Absyn::ElementItem>> {
    let mut outElement: Arc<Absyn::ElementItem>;
    outElement = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => Arc::new(Absyn::ElementItem::ELEMENTITEM { element: moveElementInfo(var_field!((*inElement).element, Absyn::ElementItem::ELEMENTITEM).clone(), (dstPath.clone()).clone())? }),
        _ => inElement.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn moveElementInfo(mut inElement: Arc<Absyn::Element>, mut dstPath: ArcStr) -> Result<Arc<Absyn::Element>> {
    let mut outElement: Arc<Absyn::Element> = inElement.clone();
    let _ = (::match_deref::match_deref! { match &(outElement.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(outElement => Absyn::Element::ELEMENT;
                specification = moveElementSpecInfo(var_field!((*outElement).specification, Absyn::Element::ELEMENT).clone(), (dstPath.clone()).clone())?,
                constrainClass = moveConstrainClassInfo(var_field!((*outElement).constrainClass, Absyn::Element::ELEMENT).clone(), (dstPath.clone()).clone())?,
                info = moveSourceInfo(var_field!((*outElement).info, Absyn::Element::ELEMENT).clone(), (dstPath.clone()).clone())?
            );
            ()
        },
        Deref @ Absyn::Element::TEXT { .. } => {
            assign_variant_field!(outElement => Absyn::Element::TEXT; info = moveSourceInfo(var_field!((*outElement).info, Absyn::Element::TEXT).clone(), (dstPath.clone()).clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

fn moveElementArgInfo(mut inArg: Arc<Absyn::ElementArg>, mut dstPath: ArcStr) -> Result<Arc<Absyn::ElementArg>> {
    let mut outArg: Arc<Absyn::ElementArg> = inArg.clone();
    let _ = (::match_deref::match_deref! { match &(outArg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            assign_variant_field!(outArg => Absyn::ElementArg::MODIFICATION;
                modification = moveModificationInfo(var_field!((*outArg).modification, Absyn::ElementArg::MODIFICATION).clone(), (dstPath.clone()).clone())?,
                info = moveSourceInfo(var_field!((*outArg).info, Absyn::ElementArg::MODIFICATION).clone(), (dstPath.clone()).clone())?
            );
            ()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            assign_variant_field!(outArg => Absyn::ElementArg::REDECLARATION;
                elementSpec = moveElementSpecInfo(var_field!((*outArg).elementSpec, Absyn::ElementArg::REDECLARATION).clone(), (dstPath.clone()).clone())?,
                constrainClass = moveConstrainClassInfo(var_field!((*outArg).constrainClass, Absyn::ElementArg::REDECLARATION).clone(), (dstPath.clone()).clone())?,
                info = moveSourceInfo(var_field!((*outArg).info, Absyn::ElementArg::REDECLARATION).clone(), (dstPath.clone()).clone())?
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

fn moveModificationInfo(mut inMod: Option<Arc<Absyn::Modification>>, mut dstPath: ArcStr) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>> = None;
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: el, eqMod: eq }) => {
            let mut el = (*el).clone();
            let mut eq = (*eq).clone();
            el = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut e in (el.clone()).into_iter().cloned() {
            let __x = moveElementArgInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            eq = moveEqModInfo(eq.clone(), (dstPath.clone()).clone())?;
            Some(Arc::new(Absyn::Modification { elementArgLst: el.clone(), eqMod: eq.clone() }))
        },
        _ => {
            inMod.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

fn moveEqModInfo(mut inEqMod: Arc<Absyn::EqMod>, mut dstPath: ArcStr) -> Result<Arc<Absyn::EqMod>> {
    let mut outEqMod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    outEqMod = (::match_deref::match_deref! { match &(inEqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => Arc::new(Absyn::EqMod::EQMOD { exp: var_field!((*inEqMod).exp, Absyn::EqMod::EQMOD).clone(), info: moveSourceInfo(var_field!((*inEqMod).info, Absyn::EqMod::EQMOD).clone(), (dstPath.clone()).clone())? }),
        _ => inEqMod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqMod)
}

fn moveConstrainClassInfo(mut inCC: Option<Arc<Absyn::ConstrainClass>>, mut dstPath: ArcStr) -> Result<Option<Arc<Absyn::ConstrainClass>>> {
    let mut outCC: Option<Arc<Absyn::ConstrainClass>> = None;
    outCC = (::match_deref::match_deref! { match &(inCC.clone()) {
        Some(Deref @ Absyn::ConstrainClass { elementSpec: spec, comment: cmt }) => {
            let mut spec = (*spec).clone();
            let mut cmt = (*cmt).clone();
            spec = moveElementSpecInfo(spec.clone(), (dstPath.clone()).clone())?;
            cmt = moveCommentInfo(cmt.clone(), (dstPath.clone()).clone())?;
            Some(Arc::new(Absyn::ConstrainClass { elementSpec: spec.clone(), comment: cmt.clone() }))
        },
        _ => {
            inCC.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCC)
}

fn moveCommentInfo(mut inComment: Option<Arc<Absyn::Comment>>, mut dstPath: ArcStr) -> Result<Option<Arc<Absyn::Comment>>> {
    let mut outComment: Option<Arc<Absyn::Comment>> = None;
    outComment = (::match_deref::match_deref! { match &(inComment.clone()) {
        Some(Deref @ Absyn::Comment { annotation_: Some(a), comment: c }) => {
            let mut a = (*a).clone();
            a = moveAnnotationInfo(a.clone(), (dstPath.clone()).clone())?;
            Some(Arc::new(Absyn::Comment { annotation_: Some(a.clone()), comment: c.clone() }))
        },
        _ => {
            inComment.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComment)
}

fn moveEquationItemInfo(mut inEquation: Arc<Absyn::EquationItem>, mut dstPath: ArcStr) -> Result<Arc<Absyn::EquationItem>> {
    let mut outEquation: Arc<Absyn::EquationItem>;
    outEquation = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq, comment: cmt, info } => {
            let mut cmt = (*cmt).clone();
            let mut info = (*info).clone();
            cmt = moveCommentInfo(cmt.clone(), (dstPath.clone()).clone())?;
            info = moveSourceInfo(info.clone(), (dstPath.clone()).clone())?;
            Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: eq.clone(), comment: cmt.clone(), info: info.clone() })
        },
        _ => {
            inEquation.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEquation)
}

fn moveAlgorithmItemInfo(mut inAlgorithm: Arc<Absyn::AlgorithmItem>, mut dstPath: ArcStr) -> Result<Arc<Absyn::AlgorithmItem>> {
    let mut outAlgorithm: Arc<Absyn::AlgorithmItem>;
    outAlgorithm = (::match_deref::match_deref! { match &(inAlgorithm.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg, comment: cmt, info } => {
            let mut cmt = (*cmt).clone();
            let mut info = (*info).clone();
            cmt = moveCommentInfo(cmt.clone(), (dstPath.clone()).clone())?;
            info = moveSourceInfo(info.clone(), (dstPath.clone()).clone())?;
            Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg.clone(), comment: cmt.clone(), info: info.clone() })
        },
        _ => {
            inAlgorithm.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAlgorithm)
}

fn moveElementSpecInfo(mut inSpec: Arc<Absyn::ElementSpec>, mut dstPath: ArcStr) -> Result<Arc<Absyn::ElementSpec>> {
    let mut outSpec: Arc<Absyn::ElementSpec> = inSpec.clone();
    let _ = (::match_deref::match_deref! { match &(outSpec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::CLASSDEF; class_ = moveClassInfo(var_field!((*outSpec).class_, Absyn::ElementSpec::CLASSDEF).clone(), (dstPath.clone()).clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::EXTENDS;
                elementArg = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut e in (var_field!((*outSpec).elementArg, Absyn::ElementSpec::EXTENDS).clone()).into_iter().cloned() {
            let __x = moveElementArgInfo(e.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                annotationOpt = moveAnnotationOptInfo(var_field!((*outSpec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone(), (dstPath.clone()).clone())?
            );
            ()
        },
        Deref @ Absyn::ElementSpec::IMPORT { .. } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::IMPORT;
                comment = moveCommentInfo(var_field!((*outSpec).comment, Absyn::ElementSpec::IMPORT).clone(), (dstPath.clone()).clone())?,
                info = moveSourceInfo(var_field!((*outSpec).info, Absyn::ElementSpec::IMPORT).clone(), (dstPath.clone()).clone())?
            );
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            assign_variant_field!(outSpec => Absyn::ElementSpec::COMPONENTS; components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (var_field!((*outSpec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
            let __x = moveComponentItemInfo(c.clone(), (dstPath.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSpec)
}

fn moveComponentItemInfo(mut inComponent: Arc<Absyn::ComponentItem>, mut dstPath: ArcStr) -> Result<Arc<Absyn::ComponentItem>> {
    let mut outComponent: Arc<Absyn::ComponentItem>;
    let mut comp: Absyn::Component;
    let mut cond: Option<Arc<Absyn::Exp>> = None;
    let mut cmt: Option<Arc<Absyn::Comment>> = None;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inComponent.clone()) {
        Deref @ Absyn::ComponentItem { component: __pa0, condition: __pa1, comment: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    comp = __pa0.clone();
    cond = __pa1.clone();
    cmt = __pa2.clone();
    comp = moveComponentInfo(comp.clone(), (dstPath.clone()).clone())?;
    cmt = moveCommentInfo(cmt.clone(), (dstPath.clone()).clone())?;
    outComponent = Arc::new(Absyn::ComponentItem { component: comp.clone(), condition: cond.clone(), comment: cmt.clone() });
    Ok(outComponent)
}

fn moveComponentInfo(mut inComponent: Absyn::Component, mut dstPath: ArcStr) -> Result<Absyn::Component> {
    let mut outComponent: Absyn::Component = inComponent.clone();
    outComponent.modification = moveModificationInfo(outComponent.modification.clone(), (dstPath.clone()).clone())?;
    Ok(outComponent)
}

fn moveExternalDeclInfo(mut inExtDecl: Arc<Absyn::ExternalDecl>, mut dstPath: ArcStr) -> Result<Arc<Absyn::ExternalDecl>> {
    let mut outExtDecl: Arc<Absyn::ExternalDecl> = inExtDecl.clone();
    assign_field!(outExtDecl.annotation_ = moveAnnotationOptInfo(outExtDecl.annotation_.clone(), (dstPath.clone()).clone())?);
    Ok(outExtDecl)
}

fn buildModel(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inValues: Arc<metamodelica::List<Arc<Values::Value>>>, mut inMsg: Absyn::Msg) -> Result<(bool, FCore::Cache, ArcStr, ArcStr, ArcStr, ArcStr, ArcStr, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>, Arc<metamodelica::List<Arc<Values::Value>>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut success: bool = false;
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut compileDir: ArcStr = arcstr::literal!("");
    let mut outString1: ArcStr = arcstr::literal!("");
    let mut outString2: ArcStr = arcstr::literal!("");
    let mut outputFormat_str: ArcStr = arcstr::literal!("");
    let mut outInitFileName: ArcStr = arcstr::literal!("");
    let mut outSimFlags: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut outArgs: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut outLibsAndLibDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outCache, compileDir, outString1, outString2, outputFormat_str, outInitFileName, outSimFlags, resultValues, outArgs, outLibsAndLibDirs) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inValues.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, vals, _) => {
                    let mut libsAndLibDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut file_dir: ArcStr = arcstr::literal!("");
                    let mut init_filename: ArcStr = arcstr::literal!("");
                    let mut method_str: ArcStr = arcstr::literal!("");
                    let mut filenameprefix: ArcStr = arcstr::literal!("");
                    let mut simflags: ArcStr = arcstr::literal!("");
                    let mut classname: Arc<Absyn::Path>;
                    let mut timeCompile: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut simSettings: SimCode::SimulationSettings;
                    let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut simflags_mod: Option<Arc<Absyn::Modification>> = None;
                    let mut cache = (*cache).clone();
                    let mut vals = (*vals).clone();
                    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = resultValues.clone();
                    let mut success: bool = success.clone();
                    let mut outputFormat_str: ArcStr = outputFormat_str.clone();
                    let mut compileDir: ArcStr = compileDir.clone();
                    values = vals.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getListFirstShowError(vals.clone(), (literal!("while retrieving the className (1 arg) from the buildModel arguments")).clone())?) {
                        (Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    classname = __pa0.clone();
                    vals = __pa1.clone();
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the startTime (2 arg) from the buildModel arguments")).clone())?;
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the stopTime (3 arg) from the buildModel arguments")).clone())?;
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the numberOfIntervals (4 arg) from the buildModel arguments")).clone())?;
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the tolerance (5 arg) from the buildModel arguments")).clone())?;
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the method (6 arg) from the buildModel arguments")).clone())?;
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(getListFirstShowError(vals.clone(), (literal!("while retreaving the fileNamePrefix (7 arg) from the buildModel arguments")).clone())?) {
                        (Deref @ Values::Value::STRING { string: __pa3 }, __pa4) => (__pa3.clone(), __pa4.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    filenameprefix = __pa3.clone();
                    vals = __pa4.clone();
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the options (8 arg) from the buildModel arguments")).clone())?;
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the outputFormat (9 arg) from the buildModel arguments")).clone())?;
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the variableFilter (10 arg) from the buildModel arguments")).clone())?;
                    (_, vals) = getListFirstShowError(vals.clone(), (literal!("while retrieving the cflags (11 arg) from the buildModel arguments")).clone())?;
                    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(getListFirstShowError(vals.clone(), (literal!("while retrieving the simflags (12 arg) from the buildModel arguments")).clone())?) {
                        (Deref @ Values::Value::STRING { string: __pa5 }, __pa6) => (__pa5.clone(), __pa6.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    simflags = __pa5.clone();
                    vals = __pa6.clone();
                    Error::clearMessages();
                    if stringEmpty((simflags.clone()).clone()) && !(Flags::getConfigBool(Flags::IGNORE_SIMULATION_FLAGS_ANNOTATION.clone())?) {
                        loadProgram(classname.clone())?;
                        simflags_mod = Interactive::getNamedAnnotationExp(classname.clone(), SymbolTable::getAbsyn(), Arc::new(Absyn::Path::IDENT { name: (literal!("__OpenModelica_simulationFlags")).clone() }), Some(None), std::sync::Arc::new(fnptr!(Util::id, _)))?;
                        simflags = (formatSimulationFlagsString(simflags_mod.clone())?).clone();
                        if !(stringEmpty((simflags.clone()).clone())) {
                            values = List::replaceAt(Arc::new(Values::Value::STRING { string: (simflags.clone()).clone() }), 12, values.clone())?;
                        }
                    }
                    compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
                    (cache, simSettings) = calculateSimulationSettings(cache.clone(), values.clone())?;
                    let SimCode::SIMULATION_SETTINGS { outputFormat: __pa7, method: __pa8, .. } = (simSettings.clone()) else { bail!("pattern mismatch") };
                    outputFormat_str = __pa7.clone();
                    method_str = __pa8.clone();
                    (success, cache, libsAndLibDirs, file_dir, resultValues) = translateModel(cache.clone(), env.clone(), classname.clone(), (filenameprefix.clone()).clone(), true, true, Some(simSettings.clone()))?;
                    System::realtimeTick(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
                    init_filename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*filenameprefix.clone()); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) }).clone();
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildModel: about to compile model ")); __mm_s.push_str(&*filenameprefix.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*file_dir.clone()); ArcStr::from(__mm_s) }).clone())?;
                    }
                    if success.clone() {
                        if '__try9: {
                            unwrap_break_err!(CevalScript::compileModel((filenameprefix.clone()).clone(), libsAndLibDirs.clone(), (literal!("")).clone(), metamodelica::nil()), '__try9);
                            Ok::<(), anyhow::Error>(())
                        }.is_err() {
                            success = false;
                        }
                        timeCompile = System::realtimeTock(ClockIndexes::RT_CLOCK_BUILD_MODEL.clone())?;
                    } else {
                        timeCompile = metamodelica::OrderedFloat(0.0_f64);
                    }
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::trace((literal!("buildModel: Compiling done.\n")).clone())?;
                    }
                    resultValues = cons((literal!("timeCompile"), Arc::new(Values::Value::REAL { real: timeCompile.clone() })), resultValues.clone());
                    Ok((cache.clone(), compileDir.clone(), filenameprefix.clone(), method_str.clone(), outputFormat_str.clone(), init_filename.clone(), simflags.clone(), resultValues.clone(), values.clone(), libsAndLibDirs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::assertion((inValues.clone().len() as i32) == 12, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("buildModel failure, length = ")); __mm_s.push_str(&*intString((inValues.clone().len() as i32))); ArcStr::from(__mm_s) }).clone(), Absyn::dummyInfo.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((success, outCache, compileDir, outString1, outString2, outputFormat_str, outInitFileName, outSimFlags, resultValues, outArgs, outLibsAndLibDirs))
}

fn formatSimulationFlagsString(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: args, .. }) => {
            List::toString(args.clone(), (std::sync::Arc::new(formatSimulationFlagString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<ArcStr> + 'static>), (literal!("")).clone(), (literal!("-")).clone(), (literal!(" -")).clone(), (literal!("")).clone(), false, 0)?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(r#str)
}

fn formatSimulationFlagString(mut arg: Arc<Absyn::ElementArg>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp, .. }, .. }), .. } => {
            (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::STRING { value: Deref @ "()" } => AbsynUtil::pathString(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), (literal!(".")).clone(), true, false)?,
        _ => { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("=")); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); ArcStr::from(__mm_s) },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            AbsynUtil::pathString(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), (literal!(".")).clone(), true, false)?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

fn createSimulationResultFromcallModelExecutable(mut buildSuccess: bool, mut callRet: i32, mut timeTotal: metamodelica::Real, mut timeSimulation: metamodelica::Real, mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>, mut inCache: FCore::Cache, mut className: Arc<Absyn::Path>, mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>, mut result_file: ArcStr, mut logFile: ArcStr) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (buildSuccess.clone(), callRet.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (false, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut simValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            simValue = createSimulationResult((result_file.clone()).clone(), (simOptionsAsString(inVals.clone())?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to build model: ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone(), cons((literal!("timeTotal"), Arc::new(Values::Value::REAL { real: timeTotal.clone() })), cons((literal!("timeSimulation"), Arc::new(Values::Value::REAL { real: timeSimulation.clone() })), resultValues.clone())))?;
            Ok((inCache.clone(), simValue.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, 0) = __mc_input.clone() else { bail!("nomatch") };
            let mut simValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            simValue = createSimulationResult((result_file.clone()).clone(), (simOptionsAsString(inVals.clone())?).clone(), (System::readFile((logFile.clone()).clone())?).clone(), cons((literal!("timeTotal"), Arc::new(Values::Value::REAL { real: timeTotal.clone() })), cons((literal!("timeSimulation"), Arc::new(Values::Value::REAL { real: timeSimulation.clone() })), resultValues.clone())))?;
            SymbolTable::addVar(Arc::new(DAE::ComponentRef::CREF_IDENT { ident: (literal!("currentSimulationResult")).clone(), identType: DAE::T_STRING_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), Arc::new(Values::Value::STRING { string: (result_file.clone()).clone() }), FGraph::empty())?;
            Ok((inCache.clone(), simValue.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut res: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut simValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            res = (if (System::regularFileExists((logFile.clone()).clone())) {System::readFile((logFile.clone()).clone())?} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*logFile.clone()); __mm_s.push_str(&*literal!(" does not exist")); ArcStr::from(__mm_s) }}).clone();
            r#str = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
            res = stringAppendList(list![(literal!("Simulation execution failed for model: ")).clone(), (r#str.clone()).clone(), (literal!("\n")).clone(), (res.clone()).clone()]);
            simValue = createSimulationResult((literal!("")).clone(), (simOptionsAsString(inVals.clone())?).clone(), (res.clone()).clone(), cons((literal!("timeTotal"), Arc::new(Values::Value::REAL { real: timeTotal.clone() })), cons((literal!("timeSimulation"), Arc::new(Values::Value::REAL { real: timeSimulation.clone() })), resultValues.clone())))?;
            Ok((inCache.clone(), simValue.clone()))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn getFileDir(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inProgram: Absyn::Program) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inComponentRef.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, p) => {
                    let mut p_class: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut filename: ArcStr = arcstr::literal!("");
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut dir_1: ArcStr = arcstr::literal!("");
                    let mut pd_1: ArcStr = arcstr::literal!("");
                    let mut filename_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut dir: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    p_class = AbsynUtil::crefToPath(class_.clone())?;
                    cdef = InteractiveUtil::getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    filename = (AbsynUtil::classFilename(cdef.clone())?).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((pd.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    pd_1 = __pa0.clone();
                    filename_1 = Util::stringSplitAtChar((filename.clone()).clone(), (pd_1.clone()).clone())?;
                    dir = List::stripLast(filename_1.clone())?;
                    dir_1 = stringDelimitList(dir.clone(), (pd.clone()).clone());
                    Ok(dir_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut dir_1: ArcStr = arcstr::literal!("");
                    let mut omhome: ArcStr = arcstr::literal!("");
                    let mut omhome_1: ArcStr = arcstr::literal!("");
                    omhome = (Settings::getInstallationDirectoryPath()?).clone();
                    omhome_1 = (System::trim((omhome.clone()).clone(), (literal!("\"")).clone())).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    dir_1 = stringAppendList(list![(literal!("\"")).clone(), (omhome_1.clone()).clone(), (pd.clone()).clone(), (literal!("work")).clone(), (literal!("\"")).clone()]);
                    Ok(dir_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn checkModel(mut cache: FCore::Cache, mut env: FCore::Graph, mut className: Arc<Absyn::Path>, mut inMsg: Absyn::Msg) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut odae: Option<DAE::DAElist> = None;
            let mut dae: DAE::DAElist;
            let mut eqnSize: i32 = 0;
            let mut varSize: i32 = 0;
            let mut simpleEqnSize: i32 = 0;
            let mut retStr: ArcStr = arcstr::literal!("");
            let mut classNameStr: ArcStr = arcstr::literal!("");
            let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
            let mut cache: FCore::Cache = cache.clone();
            flags = loadCommandLineOptionsFromModel(className.clone())?;
            match '__try0: {
                (cache, _, odae, _) = unwrap_break_err!(runFrontEnd(cache.clone(), env.clone(), className.clone(), false, false, false), '__try0);
                let Some(__pa1) = (odae.clone()) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                dae = __pa1.clone();
                (varSize, eqnSize, simpleEqnSize) = unwrap_break_err!(CheckModel::checkModel(dae.clone()), '__try0);
                FlagsUtil::saveFlags(flags.clone());
                Ok::<_, anyhow::Error>((cache.clone(), dae.clone(), eqnSize.clone(), odae.clone(), simpleEqnSize.clone(), varSize.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
                    cache = __try0_o0;
                    dae = __try0_o1;
                    eqnSize = __try0_o2;
                    odae = __try0_o3;
                    simpleEqnSize = __try0_o4;
                    varSize = __try0_o5;
                }
                Err(_) => {
                    FlagsUtil::saveFlags(flags.clone());
                    bail!("fail");
                }
            }
            classNameStr = (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone();
            retStr = stringAppendList(list![(literal!("Check of ")).clone(), (classNameStr.clone()).clone(), (literal!(" completed successfully.\nClass ")).clone(), (classNameStr.clone()).clone(), (literal!(" has ")).clone(), ArcStr::from(::std::format!("{}", eqnSize.clone())), (literal!(" equation(s) and ")).clone(), ArcStr::from(::std::format!("{}", varSize.clone())), (literal!(" variable(s).\n")).clone(), ArcStr::from(::std::format!("{}", simpleEqnSize.clone())), (literal!(" of these are trivial equation(s).")).clone()]);
            Ok(Arc::new(Values::Value::STRING { string: (retStr.clone()).clone() }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let false = (Interactive::existClass(className.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
            Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone(), (literal!("<TOP>")).clone()])?;
            Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if Error::getNumMessages() == 0 {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Check of ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" failed with no error message")); ArcStr::from(__mm_s) }).clone(), (literal!("<TOP>")).clone()])?;
            }
            Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, outValue))
}

fn getWithinStatement(mut ip: Arc<Absyn::Path>) -> Result<Absyn::Within> {
    let mut op: Absyn::Within = Absyn::Within::TOP;
    op = 'mc: {
        let __mc_input = ip.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                path => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::stripLast(path.clone())?;
                    Ok(Absyn::Within::WITHIN { path: path.clone() })
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_ast::Absyn::Within::TOP)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(op)
}

fn dumpXMLDAE(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut vals: Arc<metamodelica::List<Arc<Values::Value>>>, mut inMsg: Absyn::Msg) -> Result<(FCore::Cache, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut xml_filename: ArcStr = arcstr::literal!("");
    (outCache, xml_filename) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), vals.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "flat" }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::List::Nil } } } } } } } }, _) => {
                    let mut cname_str: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut description: ArcStr = arcstr::literal!("");
                    let mut dlow: Arc<BackendDAE::BackendDAE>;
                    let mut dlow_1: Arc<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (rewriteRulesFile.clone()).clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, Some(__pa2), _) = (runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    env = __pa1.clone();
                    dae = __pa2.clone();
                    description = (DAEUtil::daeDescription(dae.clone())).clone();
                    compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
                    cname_str = (AbsynUtil::pathString(classname.clone(), (literal!(".")).clone(), true, false)?).clone();
                    filenameprefix = (if (filenameprefix.clone() == literal!("<default>")) {cname_str.clone()} else {filenameprefix.clone()}).clone();
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (filenameprefix.clone()).clone(), simSettingsOption: None })?;
                    dlow_1 = BackendDAEUtil::preOptimizeBackendDAE(dlow.clone(), None)?;
                    dlow_1 = FindZeroCrossings::findZeroCrossings(dlow_1.clone())?;
                    xml_filename = stringAppendList(list![(filenameprefix.clone()).clone(), (literal!(".xml")).clone()]);
                    dlow_1 = applyRewriteRulesOnBackend(dlow_1.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(dlow_1.clone(), addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), false)?;
                    Print::writeBuf((xml_filename.clone()).clone())?;
                    Print::clearBuf();
                    compileDir = (if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()}).clone();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (literal!("")).clone())?;
                    RewriteRules::clearRules();
                    Ok((cache.clone(), stringAppendList(list![(compileDir.clone()).clone(), (xml_filename.clone()).clone()])))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "optimiser" }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::List::Nil } } } } } } } }, _) => {
                    let mut cname_str: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut description: ArcStr = arcstr::literal!("");
                    let mut dlow: Arc<BackendDAE::BackendDAE>;
                    let mut dlow_1: Arc<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (rewriteRulesFile.clone()).clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, Some(__pa2), _) = (runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    env = __pa1.clone();
                    dae = __pa2.clone();
                    description = (DAEUtil::daeDescription(dae.clone())).clone();
                    compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
                    cname_str = (AbsynUtil::pathString(classname.clone(), (literal!(".")).clone(), true, false)?).clone();
                    filenameprefix = (if (filenameprefix.clone() == literal!("<default>")) {cname_str.clone()} else {filenameprefix.clone()}).clone();
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (filenameprefix.clone()).clone(), simSettingsOption: None })?;
                    dlow_1 = BackendDAEUtil::preOptimizeBackendDAE(dlow.clone(), None)?;
                    dlow_1 = BackendDAEUtil::transformBackendDAE(dlow_1.clone(), None, None, None)?;
                    dlow_1 = FindZeroCrossings::findZeroCrossings(dlow_1.clone())?;
                    xml_filename = stringAppendList(list![(filenameprefix.clone()).clone(), (literal!(".xml")).clone()]);
                    dlow_1 = applyRewriteRulesOnBackend(dlow_1.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(dlow_1.clone(), addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), false)?;
                    Print::writeBuf((xml_filename.clone()).clone())?;
                    Print::clearBuf();
                    compileDir = (if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()}).clone();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (literal!("")).clone())?;
                    RewriteRules::clearRules();
                    Ok((cache.clone(), stringAppendList(list![(compileDir.clone()).clone(), (xml_filename.clone()).clone()])))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "backEnd" }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::List::Nil } } } } } } } }, _) => {
                    let mut cname_str: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut description: ArcStr = arcstr::literal!("");
                    let mut dlow: Arc<BackendDAE::BackendDAE>;
                    let mut indexed_dlow: Arc<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (rewriteRulesFile.clone()).clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, Some(__pa2), _) = (runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    env = __pa1.clone();
                    dae = __pa2.clone();
                    description = (DAEUtil::daeDescription(dae.clone())).clone();
                    compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
                    cname_str = (AbsynUtil::pathString(classname.clone(), (literal!(".")).clone(), true, false)?).clone();
                    filenameprefix = (if (filenameprefix.clone() == literal!("<default>")) {cname_str.clone()} else {filenameprefix.clone()}).clone();
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (filenameprefix.clone()).clone(), simSettingsOption: None })?;
                    indexed_dlow = BackendDAEUtil::getSolvedSystem(dlow.clone(), (literal!("")).clone(), None, None, None, None)?;
                    xml_filename = stringAppendList(list![(filenameprefix.clone()).clone(), (literal!(".xml")).clone()]);
                    indexed_dlow = applyRewriteRulesOnBackend(indexed_dlow.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(indexed_dlow.clone(), addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), false)?;
                    Print::writeBuf((xml_filename.clone()).clone())?;
                    Print::clearBuf();
                    compileDir = (if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()}).clone();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (literal!("")).clone())?;
                    RewriteRules::clearRules();
                    Ok((cache.clone(), stringAppendList(list![(compileDir.clone()).clone(), (xml_filename.clone()).clone()])))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classname } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "stateSpace" }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addSolvingInfo }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: addMathMLCode }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: dumpResiduals }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filenameprefix }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: rewriteRulesFile }, tail: Deref @ metamodelica::List::Nil } } } } } } } }, _) => {
                    let mut cname_str: ArcStr = arcstr::literal!("");
                    let mut compileDir: ArcStr = arcstr::literal!("");
                    let mut description: ArcStr = arcstr::literal!("");
                    let mut dlow: Arc<BackendDAE::BackendDAE>;
                    let mut indexed_dlow: Arc<BackendDAE::BackendDAE>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut filenameprefix = (*filenameprefix).clone();
                    let mut xml_filename: ArcStr = xml_filename.clone();
                    Error::clearMessages();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (rewriteRulesFile.clone()).clone())?;
                    RewriteRules::loadRules()?;
                    let (__pa0, __pa1, Some(__pa2), _) = (runFrontEnd(cache.clone(), env.clone(), classname.clone(), true, false, true)?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    env = __pa1.clone();
                    dae = __pa2.clone();
                    description = (DAEUtil::daeDescription(dae.clone())).clone();
                    compileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); ArcStr::from(__mm_s) }).clone();
                    cname_str = (AbsynUtil::pathString(classname.clone(), (literal!(".")).clone(), true, false)?).clone();
                    filenameprefix = (if (filenameprefix.clone() == literal!("<default>")) {cname_str.clone()} else {filenameprefix.clone()}).clone();
                    dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), env.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (filenameprefix.clone()).clone(), simSettingsOption: None })?;
                    indexed_dlow = BackendDAEUtil::getSolvedSystem(dlow.clone(), (literal!("")).clone(), None, None, None, None)?;
                    xml_filename = stringAppendList(list![(filenameprefix.clone()).clone(), (literal!(".xml")).clone()]);
                    indexed_dlow = applyRewriteRulesOnBackend(indexed_dlow.clone())?;
                    Print::clearBuf();
                    XMLDump::dumpBackendDAE(indexed_dlow.clone(), addOriginalAdjacencyMatrix.clone(), addSolvingInfo.clone(), addMathMLCode.clone(), dumpResiduals.clone(), true)?;
                    Print::writeBuf((xml_filename.clone()).clone())?;
                    Print::clearBuf();
                    compileDir = (if (Testsuite::isRunning()?) {literal!("")} else {compileDir.clone()}).clone();
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (literal!("")).clone())?;
                    RewriteRules::clearRules();
                    Ok((cache.clone(), stringAppendList(list![(compileDir.clone()).clone(), (xml_filename.clone()).clone()])))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    FlagsUtil::setConfigString(Flags::REWRITE_RULES_FILE.clone(), (literal!("")).clone())?;
                    RewriteRules::clearRules();
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, xml_filename))
}

fn applyRewriteRulesOnBackend(mut inBackendDAE: Arc<BackendDAE::BackendDAE>) -> Result<Arc<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
    outBackendDAE = 'mc: {
        let __mc_input = inBackendDAE.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (RewriteRules::noRewriteRulesBackEnd()?) else { bail!("pattern mismatch") };
                    Ok(inBackendDAE.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut outBackendDAE: Arc<BackendDAE::BackendDAE>;
                    let false = (RewriteRules::noRewriteRulesBackEnd()?) else { bail!("pattern mismatch") };
                    outBackendDAE = BackendDAEOptimize::applyRewriteRulesBackend(inBackendDAE.clone())?;
                    Ok(outBackendDAE.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBackendDAE)
}

fn getClassnamesInClassList(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClass: Arc<Absyn::Class>, mut inShowProtected: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStrings = (::match_deref::match_deref! { match &((inPath.clone(), inProgram.clone(), inClass.clone(), inShowProtected.clone())) {
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, b) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = InteractiveUtil::getClassnamesInParts(parts.clone(), b.clone(), false)?;
            strlist.clone()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. }, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { functionNames: _, comment: _ }, .. }, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals: _, comment: _ }, .. }, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, b) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = InteractiveUtil::getClassnamesInParts(parts.clone(), b.clone(), false)?;
            strlist.clone()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PDER { functionName: _, vars: _, comment: _ }, .. }, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStrings)
}

fn joinPaths(mut child: ArcStr, mut parent: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((child.clone(), parent.clone())) {
        (c, r) => {
            let mut res: Arc<Absyn::Path>;
            res = AbsynUtil::joinPaths(r.clone(), Arc::new(Absyn::Path::IDENT { name: (c.clone()).clone() }))?;
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

fn getAllClassPathsRecursive(mut inPath: Arc<Absyn::Path>, mut inCheckProtected: bool, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    outPaths = 'mc: {
        let __mc_input = (inPath.clone(), inCheckProtected.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, b, p) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut strlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut result_path_lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut result: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    cdef = InteractiveUtil::getPathedClassInProgram(inPath.clone(), p.clone(), false, false)?;
                    strlst = getClassnamesInClassList(inPath.clone(), p.clone(), cdef.clone(), b.clone())?;
                    result_path_lst = List::map1(strlst.clone(), (std::sync::Arc::new(joinPaths) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>), inPath.clone());
                    result = List::flatten(List::map2(result_path_lst.clone(), (std::sync::Arc::new(getAllClassPathsRecursive) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, bool, Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> + 'static>), b.clone(), p.clone()));
                    Ok(cons(inPath.clone(), result.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut parent_string: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    parent_string = (AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?).clone();
                    s = (Error::printMessagesStr(false)).clone();
                    s = stringAppendList(list![(parent_string.clone()).clone(), (literal!("->")).clone(), (literal!("PROBLEM GETTING CLASS PATHS: ")).clone(), (s.clone()).clone(), (literal!("\n")).clone()]);
                    println!("{}", (s.clone()).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPaths)
}

pub fn checkAllModelsRecursive(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut className: Arc<Absyn::Path>, mut inCheckProtected: bool, mut inMsg: Absyn::Msg) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), className.clone(), inCheckProtected.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, b, msg) => {
                    let mut allClassPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut ret: ArcStr = arcstr::literal!("");
                    let mut failed: i32 = 0;
                    allClassPaths = getAllClassPathsRecursive(className.clone(), b.clone(), SymbolTable::getAbsyn())?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of classes to check: ")); __mm_s.push_str(&*intString((allClassPaths.clone().len() as i32))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    failed = checkAll(cache.clone(), env.clone(), allClassPaths.clone(), msg.clone(), !(Testsuite::isRunning()?), 0)?;
                    ret = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Number of classes checked / failed: ")); __mm_s.push_str(&*intString((allClassPaths.clone().len() as i32))); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*intString(failed.clone())); ArcStr::from(__mm_s) }).clone();
                    Ok((cache.clone(), Arc::new(Values::Value::STRING { string: (ret.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _) => {
                    let mut ret: ArcStr = arcstr::literal!("");
                    ret = (stringAppend((literal!("Error checking: ")).clone(), (AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?).clone())).clone();
                    Ok((cache.clone(), Arc::new(Values::Value::STRING { string: (ret.clone()).clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn failOrSuccess(mut inStr: ArcStr) -> Result<(ArcStr, bool)> {
    let mut outStr: ArcStr = arcstr::literal!("");
    let mut failed: bool = false;
    outStr = ('mc: {
        let __mc_input = inStr.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut res: i32 = 0;
            let mut failed: bool = failed.clone();
            res = System::stringFind((inStr.clone()).clone(), (literal!("successfully")).clone())?;
            let true = (res.clone() >= 0) else { bail!("pattern mismatch") };
            failed = false;
            Ok(literal!("OK"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut failed: bool = failed.clone();
            failed = true;
            Ok(literal!("FAILED!"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok((outStr, failed))
}

pub fn checkAll(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut allClasses: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut inMsg: Absyn::Msg, mut reportTimes: bool, mut failed: i32) -> Result<i32> {
    let mut failed: i32 = failed;
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    p = SymbolTable::getAbsyn();
    let _ = ({
        let mut f: bool = false;
        'mc: {
        let __mc_input = (inEnv.clone(), allClasses.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ metamodelica::List::Cons { head: className, tail: rest }, msg) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut smsg: ArcStr = arcstr::literal!("");
                    let mut t1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut t2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut elapsedTime: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut failed: i32 = failed.clone();
                    c = InteractiveUtil::getPathedClassInProgram(className.clone(), p.clone(), false, false)?;
                    let false = (Interactive::isPackage(className.clone(), p.clone())) else { bail!("pattern mismatch") };
                    let false = (Interactive::isType(className.clone(), p.clone())) else { bail!("pattern mismatch") };
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Checking: ")); __mm_s.push_str(&*Dump::unparseClassAttributesStr(c.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("... ")); ArcStr::from(__mm_s) }).clone());
                    t1 = clock();
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), true)?;
                    let __pa0 = ::match_deref::match_deref! { match &(checkModel(FCore::emptyCache(), env.clone(), className.clone(), msg.clone())?) {
                        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), false)?;
                    t2 = clock();
                    elapsedTime = t2.clone() - t1.clone();
                    s = (realString(elapsedTime.clone())).clone();
                    (smsg, f) = failOrSuccess((r#str.clone()).clone())?;
                    failed = if (f.clone()) {failed.clone() + 1} else {failed.clone()};
                    if reportTimes.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(" seconds -> ")); __mm_s.push_str(&*smsg.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    } else {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*smsg.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if !(stringEmpty((r#str.clone()).clone())) {
                        println!("{}", (literal!("\t")).clone());
                    }
                    println!("{}", (System::stringReplace((r#str.clone()).clone(), (literal!("\n")).clone(), (literal!("\n\t")).clone())?).clone());
                    println!("{}", (literal!("\n")).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error String:\n")); __mm_s.push_str(&*Print::getErrorString()?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error Buffer:\n")); __mm_s.push_str(&*ErrorExt::printMessagesStr(false)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#")); __mm_s.push_str(&*if (f.clone()) {literal!("[-]")} else {literal!("[+]")}); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*if (reportTimes.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*realString(elapsedTime.clone())); __mm_s.push_str(&*literal!(", ")); ArcStr::from(__mm_s) }} else {literal!("")}); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", (literal!("-------------------------------------------------------------------------\n")).clone());
                    failed = checkAll(inCache.clone(), env.clone(), rest.clone(), msg.clone(), reportTimes.clone(), failed.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, Deref @ metamodelica::List::Cons { head: className, tail: rest }, msg) => {
                    let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut failed: i32 = failed.clone();
                    c = InteractiveUtil::getPathedClassInProgram(className.clone(), p.clone(), false, false)?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Checking skipped: ")); __mm_s.push_str(&*Dump::unparseClassAttributesStr(c.clone())?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("...\n")); ArcStr::from(__mm_s) }).clone());
                    failed = checkAll(inCache.clone(), env.clone(), rest.clone(), msg.clone(), reportTimes.clone(), failed.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }
    });
    Ok(failed)
}

fn getAlgorithms(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            algsList = getAlgorithmsInClassParts(parts.clone())?;
            algsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            algsList = getAlgorithmsInClassParts(parts.clone())?;
            algsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAlgorithmsInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: cp @ Deref @ Absyn::ClassPart::ALGORITHMS { .. }, tail: xs } => {
                    let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    algsList = getAlgorithmsInClassParts(xs.clone())?;
                    Ok(cons(cp.clone(), algsList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    algsList = getAlgorithmsInClassParts(xs.clone())?;
                    Ok(algsList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

fn getNthAlgorithm(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    algsList = getAlgorithms(inClass.clone())?;
    outString = (getNthAlgorithmInClass((algsList.clone()).get(inInteger.clone())?)?).clone();
    Ok(outString)
}

fn getNthAlgorithmInClass(mut inClassPart: Arc<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (Dump::unparseAlgorithmStrLst(algs.clone(), (literal!("\n")).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getInitialAlgorithms(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            algsList = getInitialAlgorithmsInClassParts(parts.clone())?;
            algsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            algsList = getInitialAlgorithmsInClassParts(parts.clone())?;
            algsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getInitialAlgorithmsInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: cp @ Deref @ Absyn::ClassPart::INITIALALGORITHMS { .. }, tail: xs } => {
                    let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    algsList = getInitialAlgorithmsInClassParts(xs.clone())?;
                    Ok(cons(cp.clone(), algsList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    algsList = getInitialAlgorithmsInClassParts(xs.clone())?;
                    Ok(algsList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

fn getNthInitialAlgorithm(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut algsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    algsList = getInitialAlgorithms(inClass.clone())?;
    outString = (getNthInitialAlgorithmInClass((algsList.clone()).get(inInteger.clone())?)?).clone();
    Ok(outString)
}

fn getNthInitialAlgorithmInClass(mut inClassPart: Arc<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (Dump::unparseAlgorithmStrLst(algs.clone(), (literal!("\n")).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getAlgorithmItemsCount(mut inClass: Arc<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getAlgorithmItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getAlgorithmItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAlgorithmItemsCountInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs }, tail: xs } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    c1 = getAlgorithmItemsCountInAlgorithmItems(algs.clone())?;
                    c2 = getAlgorithmItemsCountInClassParts(xs.clone())?;
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: i32 = 0;
                    res = getAlgorithmItemsCountInClassParts(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAlgorithmItemsCountInAlgorithmItems(mut inAbsynAlgorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inAbsynAlgorithmItemLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. }, tail: xs } => {
                    let mut c1: i32 = 0;
                    c1 = getAlgorithmItemsCountInAlgorithmItems(xs.clone())?;
                    Ok(c1.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: i32 = 0;
                    res = getAlgorithmItemsCountInAlgorithmItems(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

fn getNthAlgorithmItem(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthAlgorithmItemInClassParts(parts.clone(), inInteger.clone())?).clone();
            r#str.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthAlgorithmItemInClassParts(parts.clone(), inInteger.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getNthAlgorithmItemInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inAbsynClassPartLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs }, tail: _ }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthAlgorithmItemInAlgorithms(algs.clone(), n.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::ALGORITHMS { contents: algs }, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut c1: i32 = 0;
                    let mut newn: i32 = 0;
                    c1 = getAlgorithmItemsCountInAlgorithmItems(algs.clone())?;
                    newn = n.clone() - c1.clone();
                    r#str = (getNthAlgorithmItemInClassParts(xs.clone(), newn.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthAlgorithmItemInClassParts(xs.clone(), n.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getNthAlgorithmItemInAlgorithms(mut inAbsynAlgorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inAbsynAlgorithmItemLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { info: inf, comment: cmt, algorithm_: alg }, tail: _ }, 1) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Dump::unparseAlgorithmStr(Arc::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: alg.clone(), comment: cmt.clone(), info: inf.clone() }))?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut newn: i32 = 0;
                    newn = n.clone() - 1;
                    r#str = (getNthAlgorithmItemInAlgorithms(xs.clone(), newn.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn getInitialAlgorithmItemsCount(mut inClass: Arc<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getInitialAlgorithmItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getInitialAlgorithmItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getInitialAlgorithmItemsCountInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs }, tail: xs } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    c1 = getAlgorithmItemsCountInAlgorithmItems(algs.clone())?;
                    c2 = getInitialAlgorithmItemsCountInClassParts(xs.clone())?;
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: i32 = 0;
                    res = getInitialAlgorithmItemsCountInClassParts(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

fn getNthInitialAlgorithmItem(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inClass.clone(), inInteger.clone())) {
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, n) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthInitialAlgorithmItemInClassParts(parts.clone(), n.clone())?).clone();
            r#str.clone()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, n) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthInitialAlgorithmItemInClassParts(parts.clone(), n.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getNthInitialAlgorithmItemInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inAbsynClassPartLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs }, tail: _ }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthAlgorithmItemInAlgorithms(algs.clone(), n.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algs }, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut c1: i32 = 0;
                    let mut newn: i32 = 0;
                    c1 = getAlgorithmItemsCountInAlgorithmItems(algs.clone())?;
                    newn = n.clone() - c1.clone();
                    r#str = (getNthInitialAlgorithmItemInClassParts(xs.clone(), newn.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthInitialAlgorithmItemInClassParts(xs.clone(), n.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn getEquations(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            eqsList = getEquationsInClassParts(parts.clone())?;
            eqsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            eqsList = getEquationsInClassParts(parts.clone())?;
            eqsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getEquationsInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: cp @ Deref @ Absyn::ClassPart::EQUATIONS { .. }, tail: xs } => {
                    let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    eqsList = getEquationsInClassParts(xs.clone())?;
                    Ok(cons(cp.clone(), eqsList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    eqsList = getEquationsInClassParts(xs.clone())?;
                    Ok(eqsList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

fn getNthEquation(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    eqsList = getEquations(inClass.clone())?;
    outString = (getNthEquationInClass((eqsList.clone()).get(inInteger.clone())?)?).clone();
    Ok(outString)
}

fn getNthEquationInClass(mut inClassPart: Arc<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (Dump::unparseEquationItemStrLst(eqs.clone(), (literal!("\n")).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getInitialEquations(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            eqsList = getInitialEquationsInClassParts(parts.clone())?;
            eqsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            eqsList = getInitialEquationsInClassParts(parts.clone())?;
            eqsList.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getInitialEquationsInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: cp @ Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. }, tail: xs } => {
                    let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    eqsList = getInitialEquationsInClassParts(xs.clone())?;
                    Ok(cons(cp.clone(), eqsList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    eqsList = getInitialEquationsInClassParts(xs.clone())?;
                    Ok(eqsList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

fn getNthInitialEquation(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut eqsList: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    eqsList = getInitialEquations(inClass.clone())?;
    outString = (getNthInitialEquationInClass((eqsList.clone()).get(inInteger.clone())?)?).clone();
    Ok(outString)
}

fn getNthInitialEquationInClass(mut inClassPart: Arc<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (Dump::unparseEquationItemStrLst(eqs.clone(), (literal!("\n")).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getEquationItemsCount(mut inClass: Arc<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getEquationItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getEquationItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getEquationItemsCountInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs }, tail: xs } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    c1 = getEquationItemsCountInEquationItems(eqs.clone())?;
                    c2 = getEquationItemsCountInClassParts(xs.clone())?;
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: i32 = 0;
                    res = getEquationItemsCountInClassParts(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getEquationItemsCountInEquationItems(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inAbsynEquationItemLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { .. }, tail: xs } => {
                    let mut c1: i32 = 0;
                    c1 = getEquationItemsCountInEquationItems(xs.clone())?;
                    Ok(c1.clone() + 1)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: i32 = 0;
                    res = getEquationItemsCountInEquationItems(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

fn getNthEquationItem(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inClass.clone(), inInteger.clone())) {
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, n) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthEquationItemInClassParts(parts.clone(), n.clone())?).clone();
            r#str.clone()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, n) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthEquationItemInClassParts(parts.clone(), n.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getNthEquationItemInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inAbsynClassPartLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs }, tail: _ }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthEquationItemInEquations(eqs.clone(), n.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqs }, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut c1: i32 = 0;
                    let mut newn: i32 = 0;
                    c1 = getEquationItemsCountInEquationItems(eqs.clone())?;
                    newn = n.clone() - c1.clone();
                    r#str = (getNthEquationItemInClassParts(xs.clone(), newn.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthEquationItemInClassParts(xs.clone(), n.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getNthEquationItemInEquations(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inAbsynEquationItemLst.clone(), inInteger.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq, .. }, tail: _ }, 1) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Dump::unparseEquationStr(eq.clone())?).clone();
                    r#str = (stringAppend((r#str.clone()).clone(), (literal!(";")).clone())).clone();
                    r#str = (System::trim((r#str.clone()).clone(), (literal!(" ")).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, n) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut newn: i32 = 0;
                    newn = n.clone() - 1;
                    r#str = (getNthEquationItemInEquations(xs.clone(), newn.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn getInitialEquationItemsCount(mut inClass: Arc<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getInitialEquationItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut count: i32 = 0;
            count = getInitialEquationItemsCountInClassParts(parts.clone())?;
            count.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getInitialEquationItemsCountInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs }, tail: xs } => {
                    let mut c1: i32 = 0;
                    let mut c2: i32 = 0;
                    c1 = getEquationItemsCountInEquationItems(eqs.clone())?;
                    c2 = getInitialEquationItemsCountInClassParts(xs.clone())?;
                    Ok(c1.clone() + c2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut res: i32 = 0;
                    res = getInitialEquationItemsCountInClassParts(xs.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(0)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInteger)
}

fn getNthInitialEquationItem(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthInitialEquationItemInClassParts(parts.clone(), inInteger.clone())?).clone();
            r#str.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (getNthInitialEquationItemInClassParts(parts.clone(), inInteger.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getNthInitialEquationItemInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs }, tail: _ } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthEquationItemInEquations(eqs.clone(), inInteger.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqs }, tail: xs } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut c1: i32 = 0;
                    let mut newn: i32 = 0;
                    c1 = getEquationItemsCountInEquationItems(eqs.clone())?;
                    newn = inInteger.clone() - c1.clone();
                    r#str = (getNthInitialEquationItemInClassParts(xs.clone(), newn.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (getNthInitialEquationItemInClassParts(xs.clone(), inInteger.clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn getAnnotationCount(mut inClass: Arc<Absyn::Class>) -> Result<i32> {
    let mut outInteger: i32 = 0;
    outInteger = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { ann, .. }, .. } => {
            (ann.clone().len() as i32)
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { ann, .. }, .. } => {
            (ann.clone().len() as i32)
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            0
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInteger)
}

fn getNthAnnotationString(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inClass.clone(), inInteger.clone())) {
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { ann: anns, .. }, .. }, n) => {
            let mut ann: Arc<Absyn::Annotation> = Arc::new(<Absyn::Annotation as ::std::default::Default>::default());
            let mut r#str: ArcStr = arcstr::literal!("");
            ann = (anns.clone()).get(n.clone())?;
            r#str = (Dump::unparseAnnotation(ann.clone())?).clone();
            r#str = (stringAppend((r#str.clone()).clone(), (literal!(";")).clone())).clone();
            r#str = (System::trim((r#str.clone()).clone(), (literal!(" ")).clone())).clone();
            r#str.clone()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { ann: anns, .. }, .. }, n) => {
            let mut ann: Arc<Absyn::Annotation> = Arc::new(<Absyn::Annotation as ::std::default::Default>::default());
            let mut r#str: ArcStr = arcstr::literal!("");
            ann = (anns.clone()).get(n.clone())?;
            r#str = (Dump::unparseAnnotation(ann.clone())?).clone();
            r#str = (stringAppend((r#str.clone()).clone(), (literal!(";")).clone())).clone();
            r#str = (System::trim((r#str.clone()).clone(), (literal!(" ")).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getImportCount(mut inClass: Arc<Absyn::Class>) -> i32 {
    let mut outInteger: i32 = 0;
    let mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    (pub_imports_list, pro_imports_list) = CevalScript::getImportList(inClass.clone(), metamodelica::nil(), metamodelica::nil());
    outInteger = (pub_imports_list.clone().len() as i32) + (pro_imports_list.clone().len() as i32);
    outInteger
}

fn getNthImport(mut inClass: Arc<Absyn::Class>, mut inInteger: i32) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValue: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    (pub_imports_list, pro_imports_list) = CevalScript::getImportList(inClass.clone(), metamodelica::nil(), metamodelica::nil());
    outValue = unparseNthImport((pub_imports_list.clone()).get(inInteger.clone())?)?;
    Ok(outValue)
}

fn unparseNthImport(mut inImport: Absyn::Import) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outValue: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outValue = (match inImport.clone() {
        Absyn::Import::NAMED_IMPORT { path: mut path, name: mut id } => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut path_str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            vals = list![Arc::new(Values::Value::STRING { string: (path_str.clone()).clone() }), Arc::new(Values::Value::STRING { string: (id.clone()).clone() }), Arc::new(Values::Value::STRING { string: (literal!("named")).clone() })];
            vals.clone()
        },
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut path_str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            vals = list![Arc::new(Values::Value::STRING { string: (path_str.clone()).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("qualified")).clone() })];
            vals.clone()
        },
        Absyn::Import::UNQUAL_IMPORT { path: mut path } => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut path_str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            path_str = stringAppendList(list![(path_str.clone()).clone(), (literal!(".*")).clone()]);
            vals = list![Arc::new(Values::Value::STRING { string: (path_str.clone()).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("unqualified")).clone() })];
            vals.clone()
        },
        Absyn::Import::GROUP_IMPORT { groups: ref gi, prefix: ref path } => {
            let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut path_str: ArcStr = arcstr::literal!("");
            let mut id: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            id = stringDelimitList(unparseGroupImport(gi.clone())?, (literal!(",")).clone());
            path_str = stringAppendList(list![(path_str.clone()).clone(), (literal!(".{")).clone(), (id.clone()).clone(), (literal!("}")).clone()]);
            vals = list![Arc::new(Values::Value::STRING { string: (path_str.clone()).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("multiple")).clone() })];
            vals.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outValue)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn unparseGroupImport(mut inAbsynGroupImportLst: Arc<metamodelica::List<Absyn::GroupImport>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = inAbsynGroupImportLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Absyn::GroupImport::GROUP_IMPORT_NAME { name: r#str }, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst = unparseGroupImport(rest.clone())?;
                    Ok(cons((r#str.clone()).clone(), lst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    lst = unparseGroupImport(rest.clone())?;
                    Ok(lst.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

pub fn isShortDefinition(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = 'mc: {
        let __mc_input = inPath.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                path => {
                    ::match_deref::match_deref! { match &(InteractiveUtil::getPathedClassInProgram(path.clone(), inProgram.clone(), false, false)?) {
                        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => (),
                        _ => bail!("pattern mismatch"),
                    } };
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
    Ok(outBoolean)
}

fn isExperiment(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> bool {
    let mut res: bool = false;
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    match '__try0: {
        cdef = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(path.clone(), program.clone(), false, false), '__try0);
        let false = (unwrap_break_err!(AbsynUtil::isPartial(cdef.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        let true = (AbsynUtil::isModel(cdef.clone()) || AbsynUtil::isBlock(cdef.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        let Some(__pa1) = (unwrap_break_err!(AbsynUtil::getNamedAnnotationInClass(cdef.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("experiment")).clone() }), (std::sync::Arc::new(hasStopTime) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<bool> + 'static>)), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        res = __pa1.clone();
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = false;
        }
    }
    res
}

fn hasStopTime(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: arglst, .. }) => {
            List::any(arglst.clone(), (std::sync::Arc::new(fnptr!(hasStopTime2, Arc<Absyn::ElementArg>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<bool> + 'static>))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

fn hasStopTime2(mut arg: Arc<Absyn::ElementArg>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "StopTime" }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn searchClassNames(mut inVals: Arc<metamodelica::List<Arc<Values::Value>>>, mut inSearchText: ArcStr, mut inFindInText: bool, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut outVals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    outVals = 'mc: {
        let __mc_input = (inVals.clone(), inSearchText.clone(), inFindInText.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: val @ Deref @ Values::Value::CODE { A: _ }, tail: xs }, str1, true, p) => {
                    let mut valsList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut p1: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut position: i32 = 0;
                    absynClass = InteractiveUtil::getPathedClassInProgram(ValuesUtil::getPath(val.clone())?, p.clone(), false, false)?;
                    p1 = Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP };
                    let false = (Interactive::isPackage(ValuesUtil::getPath(val.clone())?, inProgram.clone())) else { bail!("pattern mismatch") };
                    r#str = (Dump::unparseStr(p1.clone(), false, Dump::defaultDumpOptions.clone())?).clone();
                    position = System::stringFind((System::tolower((r#str.clone()).clone())).clone(), (System::tolower((str1.clone()).clone())).clone())?;
                    let true = (position.clone() > -1) else { bail!("pattern mismatch") };
                    valsList = searchClassNames(xs.clone(), (str1.clone()).clone(), true, p.clone())?;
                    Ok(cons(val.clone(), valsList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: val @ Deref @ Values::Value::CODE { A: _ }, tail: xs }, str1, b, p) => {
                    let mut valsList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut position: i32 = 0;
                    r#str = (ValuesDump::valString(val.clone())?).clone();
                    position = System::stringFind((System::tolower((r#str.clone()).clone())).clone(), (System::tolower((str1.clone()).clone())).clone())?;
                    let true = (position.clone() > -1) else { bail!("pattern mismatch") };
                    valsList = searchClassNames(xs.clone(), (str1.clone()).clone(), b.clone(), p.clone())?;
                    Ok(cons(val.clone(), valsList.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, str1, b, p) => {
                    let mut valsList: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    valsList = searchClassNames(xs.clone(), (str1.clone()).clone(), b.clone(), p.clone())?;
                    Ok(valsList.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVals)
}

fn makeUsesArray(mut inTpl: (Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)) -> Result<Arc<Values::Value>> {
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    v = (::match_deref::match_deref! { match &(inTpl.clone()) {
        (p, _, Deref @ metamodelica::List::Cons { head: ver, tail: Deref @ metamodelica::List::Nil }, _) => {
            let mut pstr: ArcStr = arcstr::literal!("");
            pstr = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
            ValuesMake::makeArray(list![Arc::new(Values::Value::STRING { string: (pstr.clone()).clone() }), Arc::new(Values::Value::STRING { string: (ver.clone()).clone() })])?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("makeUsesArray failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(v)
}

fn saveTotalModel(mut filename: ArcStr, mut classpath: Arc<Absyn::Path>, mut stripAnnotations: bool, mut stripComments: bool, mut obfuscate: bool) -> Result<()> {
    let mut result: ArcStr = arcstr::literal!("");
    let mut obfuscate_map: ArcStr = arcstr::literal!("");
    (result, obfuscate_map) = getTotalModel(classpath.clone(), stripAnnotations.clone(), stripComments.clone(), obfuscate.clone())?;
    if obfuscate.clone() {
        System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::stripFileExtension((filename.clone()).clone())?); __mm_s.push_str(&*literal!("_mapping.json")); ArcStr::from(__mm_s) }).clone(), (obfuscate_map.clone()).clone())?;
    }
    System::writeFile((filename.clone()).clone(), (result.clone()).clone())?;
    Ok(())
}

fn getTotalModel(mut classpath: Arc<Absyn::Path>, mut stripAnnotations: bool, mut stripComments: bool, mut obfuscate: bool) -> Result<(ArcStr, ArcStr)> {
    let mut result: ArcStr = arcstr::literal!("");
    let mut obfuscate_map: ArcStr = arcstr::literal!("");
    let mut scodeP: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut str1: ArcStr = arcstr::literal!("");
    let mut str2: ArcStr = arcstr::literal!("");
    let mut str3: ArcStr = arcstr::literal!("");
    let mut env: Arc<metamodelica::List<Arc<NFSCodeEnv::Frame>>> = metamodelica::nil();
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    let mut cls_path: Arc<Absyn::Path> = classpath.clone();
    loadProgram(cls_path.clone())?;
    scodeP = SymbolTable::getSCode()?;
    (scodeP, env) = NFSCodeFlatten::flattenClassInProgram(cls_path.clone(), scodeP.clone())?;
    let __pa0 = ::match_deref::match_deref! { match &(NFSCodeLookup::lookupClassName(cls_path.clone(), env.clone(), Absyn::dummyInfo.clone())?) {
        (Deref @ NFSCodeEnv::Item::CLASS { cls: Deref @ SCode::Element::CLASS { cmt: __pa0, .. }, .. }, _, _) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cmt = __pa0.clone();
    scodeP = SCodeUtil::removeBuiltinsFromTopScope(scodeP.clone());
    if stripAnnotations.clone() || stripComments.clone() {
        scodeP = SCodeUtil::stripCommentsFromProgram(scodeP.clone(), stripAnnotations.clone(), stripComments.clone())?;
    }
    if obfuscate.clone() {
        (scodeP, cls_path, cmt, obfuscate_map, _) = Obfuscate::obfuscateProgram(scodeP.clone(), cls_path.clone(), cmt.clone())?;
    }
    r#str = (SCodeDump::programStr(scodeP.clone(), SCodeDump::defaultOptions.clone())?).clone();
    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathLastIdent(cls_path.clone())?); __mm_s.push_str(&*literal!("_total")); ArcStr::from(__mm_s) }).clone();
    str2 = (if (stripComments.clone()) {literal!("")} else {SCodeDump::printCommentStr(cmt.clone(), SCodeDump::defaultOptions.clone())?}).clone();
    str2 = (if (stringEq((str2.clone()).clone(), (literal!("")).clone())?) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*str2.clone()); ArcStr::from(__mm_s) }}).clone();
    str3 = (if (stripAnnotations.clone()) {literal!("")} else {SCodeDump::printAnnotationStr(cmt.clone(), SCodeDump::defaultOptions.clone())?}).clone();
    str3 = (if (stringEq((str3.clone()).clone(), (literal!("")).clone())?) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*str3.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }}).clone();
    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmodel ")); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*str2.clone()); __mm_s.push_str(&*literal!("\n  extends ")); __mm_s.push_str(&*AbsynUtil::pathString(cls_path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(";\n")); __mm_s.push_str(&*str3.clone()); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
    result = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*str1.clone()); ArcStr::from(__mm_s) }).clone();
    Ok((result, obfuscate_map))
}

fn saveTotalModelDebug(mut filename: ArcStr, mut classPath: Arc<Absyn::Path>, mut stripAnnotations: bool, mut stripComments: bool, mut obfuscate: bool) -> Result<()> {
    let mut prog: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut name_str: ArcStr = arcstr::literal!("");
    let mut cls_str: ArcStr = arcstr::literal!("");
    let mut str1: ArcStr = arcstr::literal!("");
    let mut str2: ArcStr = arcstr::literal!("");
    let mut str3: ArcStr = arcstr::literal!("");
    let mut cls_path: Arc<Absyn::Path> = classPath.clone();
    let mut ocmt: Option<Arc<SCode::Comment>> = None;
    let mut cmt: Arc<SCode::Comment> = Arc::new(<SCode::Comment as ::std::default::Default>::default());
    loadProgram(cls_path.clone())?;
    prog = SymbolTable::getSCode()?;
    prog = TotalModelDebug::getTotalModel(prog.clone(), cls_path.clone())?;
    prog = SCodeUtil::removeBuiltinsFromTopScope(prog.clone());
    ocmt = SCodeUtil::getElementComment(InteractiveUtil::getPathedSCodeElementInProgram(cls_path.clone(), prog.clone())?);
    cmt = if (isSome(ocmt.clone())) {Util::getOption(ocmt.clone())?} else {SCode::noComment.clone()};
    if stripAnnotations.clone() || stripComments.clone() {
        prog = SCodeUtil::stripCommentsFromProgram(prog.clone(), stripAnnotations.clone(), stripComments.clone())?;
    }
    if obfuscate.clone() {
        (prog, cls_path, cmt, _, _) = Obfuscate::obfuscateProgram(prog.clone(), cls_path.clone(), cmt.clone())?;
    }
    r#str = (SCodeDump::programStr(prog.clone(), SCodeDump::defaultOptions.clone())?).clone();
    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathLastIdent(cls_path.clone())?); __mm_s.push_str(&*literal!("_total")); ArcStr::from(__mm_s) }).clone();
    str2 = (if (stripComments.clone()) {literal!("")} else {SCodeDump::printCommentStr(cmt.clone(), SCodeDump::defaultOptions.clone())?}).clone();
    str2 = (if (stringEq((str2.clone()).clone(), (literal!("")).clone())?) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*str2.clone()); ArcStr::from(__mm_s) }}).clone();
    str3 = (if (stripAnnotations.clone()) {literal!("")} else {SCodeDump::printAnnotationStr(cmt.clone(), SCodeDump::defaultOptions.clone())?}).clone();
    str3 = (if (stringEq((str3.clone()).clone(), (literal!("")).clone())?) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*str3.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }}).clone();
    str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nmodel ")); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*str2.clone()); __mm_s.push_str(&*literal!("\n  extends ")); __mm_s.push_str(&*AbsynUtil::pathString(cls_path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(";\n")); __mm_s.push_str(&*str3.clone()); __mm_s.push_str(&*literal!("end ")); __mm_s.push_str(&*str1.clone()); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
    System::writeFile((filename.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*str1.clone()); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn getDymolaStateAnnotation(mut className: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<bool> {
    let mut isState: bool = false;
    isState = (::match_deref::match_deref! { match &((className.clone(), p.clone())) {
        (_, _) => {
            let mut stateStr: ArcStr = arcstr::literal!("");
            stateStr = (Interactive::getNamedAnnotationExp(className.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("__Dymola_state")).clone() }), Some((literal!("false")).clone()), (std::sync::Arc::new(getDymolaStateAnnotationModStr) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<ArcStr> + 'static>))?).clone();
            stringEq((stateStr.clone()).clone(), (literal!("true")).clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(isState)
}

fn getDymolaStateAnnotationModStr(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut stateStr: ArcStr = arcstr::literal!("");
    stateStr = ('mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }) => {
                    let mut stateStr: ArcStr = stateStr.clone();
                    stateStr = (Dump::printExpStr(e.clone())?).clone();
                    Ok(stateStr.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("false"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(stateStr)
}

fn getClassInformation(mut path: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut res_1: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut name: ArcStr = arcstr::literal!("");
    let mut file: ArcStr = arcstr::literal!("");
    let mut strPartial: ArcStr = arcstr::literal!("");
    let mut strFinal: ArcStr = arcstr::literal!("");
    let mut strEncapsulated: ArcStr = arcstr::literal!("");
    let mut res: ArcStr = arcstr::literal!("");
    let mut cmt: ArcStr = arcstr::literal!("");
    let mut str_readonly: ArcStr = arcstr::literal!("");
    let mut str_sline: ArcStr = arcstr::literal!("");
    let mut str_scol: ArcStr = arcstr::literal!("");
    let mut str_eline: ArcStr = arcstr::literal!("");
    let mut str_ecol: ArcStr = arcstr::literal!("");
    let mut version: ArcStr = arcstr::literal!("");
    let mut preferredView: ArcStr = arcstr::literal!("");
    let mut access: ArcStr = arcstr::literal!("");
    let mut versionDate: ArcStr = arcstr::literal!("");
    let mut versionBuild: ArcStr = arcstr::literal!("");
    let mut dateModified: ArcStr = arcstr::literal!("");
    let mut revisionId: ArcStr = arcstr::literal!("");
    let mut dim_str: ArcStr = arcstr::literal!("");
    let mut lastIdent: ArcStr = arcstr::literal!("");
    let mut partialPrefix: bool = false;
    let mut finalPrefix: bool = false;
    let mut encapsulatedPrefix: bool = false;
    let mut isReadOnly: bool = false;
    let mut isProtectedClass: bool = false;
    let mut isDocClass: bool = false;
    let mut isState: bool = false;
    let mut restr: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    let mut cdef: Arc<Absyn::ClassDef>;
    let mut sl: i32 = 0;
    let mut sc: i32 = 0;
    let mut el: i32 = 0;
    let mut ec: i32 = 0;
    let mut classPath: Arc<Absyn::Path>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?) {
        Deref @ Absyn::Class { name: __pa0, partialPrefix: __pa1, finalPrefix: __pa2, encapsulatedPrefix: __pa3, restriction: __pa4, body: __pa5, commentsBeforeClass: _, commentsBeforeEnd: _, commentsAfterEnd: _, info: SourceInfo { fileName: __pa6, isReadOnly: __pa7, lineNumberStart: __pa8, columnNumberStart: __pa9, lineNumberEnd: __pa10, columnNumberEnd: __pa11, lastModification: _ } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    partialPrefix = __pa1.clone();
    finalPrefix = __pa2.clone();
    encapsulatedPrefix = __pa3.clone();
    restr = __pa4.clone();
    cdef = __pa5.clone();
    file = __pa6.clone();
    isReadOnly = __pa7.clone();
    sl = __pa8.clone();
    sc = __pa9.clone();
    el = __pa10.clone();
    ec = __pa11.clone();
    res = (Dump::unparseRestrictionStr(restr.clone())?).clone();
    cmt = (getClassDefComment(cdef.clone())).clone();
    file = (Testsuite::friendly((file.clone()).clone())?).clone();
    if AbsynUtil::pathIsIdent(AbsynUtil::makeNotFullyQualified(path.clone())) {
        isProtectedClass = false;
    } else {
        lastIdent = (AbsynUtil::pathLastIdent(AbsynUtil::makeNotFullyQualified(path.clone()))?).clone();
        classPath = AbsynUtil::stripLast(path.clone())?;
        isProtectedClass = Interactive::isProtectedClass(classPath.clone(), (lastIdent.clone()).clone(), p.clone());
    }
    isDocClass = Interactive::getDocumentationClassAnnotation(path.clone(), p.clone())?;
    version = (CevalScript::getPackageVersion(path.clone(), p.clone())?).clone();
    preferredView = (Interactive::getStringNamedAnnotation(path.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("preferredView")).clone() }))).clone();
    isState = getDymolaStateAnnotation(path.clone(), p.clone())?;
    access = (Interactive::getAccessAnnotation(path.clone(), p.clone())?).clone();
    versionDate = (Interactive::getStringNamedAnnotation(path.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("versionDate")).clone() }))).clone();
    versionBuild = (Interactive::getIntegerNamedAnnotation(path.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("versionBuild")).clone() }))).clone();
    dateModified = (Interactive::getStringNamedAnnotation(path.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("dateModified")).clone() }))).clone();
    revisionId = (Interactive::getStringNamedAnnotation(path.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("revisionId")).clone() }))).clone();
    res_1 = Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::STRING { string: (res.clone()).clone() }), Arc::new(Values::Value::STRING { string: (cmt.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: partialPrefix.clone() }), Arc::new(Values::Value::BOOL { boolean: finalPrefix.clone() }), Arc::new(Values::Value::BOOL { boolean: encapsulatedPrefix.clone() }), Arc::new(Values::Value::STRING { string: (file.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: isReadOnly.clone() }), Arc::new(Values::Value::INTEGER { integer: sl.clone() }), Arc::new(Values::Value::INTEGER { integer: sc.clone() }), Arc::new(Values::Value::INTEGER { integer: el.clone() }), Arc::new(Values::Value::INTEGER { integer: ec.clone() }), getClassDimensions(cdef.clone())?, Arc::new(Values::Value::BOOL { boolean: isProtectedClass.clone() }), Arc::new(Values::Value::BOOL { boolean: isDocClass.clone() }), Arc::new(Values::Value::STRING { string: (version.clone()).clone() }), Arc::new(Values::Value::STRING { string: (preferredView.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: isState.clone() }), Arc::new(Values::Value::STRING { string: (access.clone()).clone() }), Arc::new(Values::Value::STRING { string: (versionDate.clone()).clone() }), Arc::new(Values::Value::STRING { string: (versionBuild.clone()).clone() }), Arc::new(Values::Value::STRING { string: (dateModified.clone()).clone() }), Arc::new(Values::Value::STRING { string: (revisionId.clone()).clone() })] });
    Ok(res_1)
}

fn getClassDimensions(mut cdef: Arc<Absyn::ClassDef>) -> Result<Arc<Values::Value>> {
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    v = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: Some(ad), .. }, .. } => {
            ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut d in (ad.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (Dump::printSubscriptStr(d.clone())?).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?
        },
        _ => {
            ValuesMake::makeArray(metamodelica::nil())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(v)
}

fn getClassElementComment(mut element: Arc<Absyn::Element>) -> ArcStr {
    let mut commentStr: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    commentStr = ((::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: cls, .. }, .. } => {
            commentStr = (InteractiveUtil::getConstrainingClassComment(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone())).clone();
            if stringEmpty((commentStr.clone()).clone()) {
                commentStr = (getClassDefComment(cls.body.clone())).clone();
            }
            commentStr.clone()
        },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    commentStr
}

fn getClassDefComment(mut inClassDef: Arc<Absyn::ClassDef>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClassDef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { comment: Some(r#str), .. } => {
            r#str.clone()
        },
        Deref @ Absyn::ClassDef::DERIVED { comment: cmt, .. } => {
            Interactive::getStringComment(cmt.clone())
        },
        Deref @ Absyn::ClassDef::ENUMERATION { comment: cmt, .. } => {
            Interactive::getStringComment(cmt.clone())
        },
        Deref @ Absyn::ClassDef::ENUMERATION { comment: cmt, .. } => {
            Interactive::getStringComment(cmt.clone())
        },
        Deref @ Absyn::ClassDef::OVERLOAD { comment: cmt, .. } => {
            Interactive::getStringComment(cmt.clone())
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: Some(r#str), .. } => {
            r#str.clone()
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

fn getAnnotationInEquation(mut inEquationItem: Arc<Absyn::EquationItem>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inEquationItem.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annotations }), comment: _ }), .. } => {
            let mut annotationStr: ArcStr = arcstr::literal!("");
            let mut annotationList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            annotationList = getAnnotationInEquationElArgs(annotations.clone())?;
            annotationStr = stringDelimitList(annotationList.clone(), (literal!(", ")).clone());
            annotationStr.clone()
        },
        Deref @ Absyn::EquationItem::EQUATIONITEM { comment: None, .. } => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn getAnnotationInEquationElArgs(mut inElArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inElArgLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: r#mod, eqMod: _ }), path: Deref @ Absyn::Path::IDENT { name: annName }, .. }, tail: rest } => {
                    let mut fargs: Arc<Absyn::FunctionArgs>;
                    let mut p_1: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut env: FCore::Graph;
                    let mut newexp: Arc<DAE::Exp>;
                    let mut gexpstr: ArcStr = arcstr::literal!("");
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
                    let mut prop: DAE::Properties;
                    let mut lineProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    lineProgram = InteractiveUtil::modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?;
                    fargs = Interactive::createFuncargsFromElementargs(r#mod.clone())?;
                    p_1 = AbsynToSCode::translateAbsyn2SCode(lineProgram.clone())?;
                    (cache, env) = Inst::makeEnvFromProgram(p_1.clone())?;
                    (_, newexp, prop) = StaticScript::elabGraphicsExp(cache.clone(), env.clone(), Arc::new(Absyn::Exp::CALL { function_: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (annName.clone()).clone(), subscripts: metamodelica::nil() }), functionArgs: fargs.clone(), typeVars: metamodelica::nil() }), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, metamodelica::sourceInfo!())?;
                    (cache, newexp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), newexp.clone(), prop.clone(), false, metamodelica::sourceInfo!())?;
                    Print::clearErrorBuf();
                    gexpstr = (ExpressionBasics::printExpStr(newexp.clone())?).clone();
                    res = getAnnotationInEquationElArgs(rest.clone())?;
                    Ok(cons((gexpstr.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: _, eqMod: Deref @ Absyn::EqMod::NOMOD }), path: Deref @ Absyn::Path::IDENT { name: annName }, .. }, tail: rest } => {
                    let mut gexpstr_1: ArcStr = arcstr::literal!("");
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    gexpstr_1 = stringAppendList(list![(annName.clone()).clone(), (literal!("(error)")).clone()]);
                    res = getAnnotationInEquationElArgs(rest.clone())?;
                    Ok(cons((gexpstr_1.clone()).clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

fn getTransitions(mut path: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut transitions: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    cdef = InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
    transitions = getTransitionsInClass(cdef.clone())?.reverse();
    res = ValuesMake::makeArray(List::map(transitions.clone(), (std::sync::Arc::new(ValuesMake::makeStringArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Values::Value>> + 'static>)))?;
    Ok(res)
}

fn getTransitionsInClass(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outTransitions: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    outTransitions = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut transitions: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            transitions = getTransitionsInClassParts(parts.clone())?;
            transitions.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut transitions: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            transitions = getTransitionsInClassParts(parts.clone())?;
            transitions.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTransitions)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getTransitionsInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outTransitions: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    outTransitions = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqlist }, tail: xs } => {
                    let mut transitions1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut transitions2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    transitions1 = getTransitionsInEquations(eqlist.clone(), metamodelica::nil())?;
                    transitions2 = getTransitionsInClassParts(xs.clone())?;
                    Ok(listAppend(transitions1.clone(), transitions2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut transitions1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    transitions1 = getTransitionsInClassParts(xs.clone())?;
                    Ok(transitions1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTransitions)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getTransitionsInEquations(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut inTransitions: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outTransitions: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    outTransitions = (::match_deref::match_deref! { match &((inAbsynEquationItemLst.clone(), inTransitions.clone())) {
        (Deref @ metamodelica::List::Cons { head: eqItem @ Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq @ Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "transition", .. }, .. }, .. }, tail: xs }, transitions) => {
            let mut transition: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut transitions = (*transitions).clone();
            transition = getTransitionInEquation(eq.clone())?;
            transition = List::insert(transition.clone(), (transition.clone().len() as i32) + 1, (getAnnotationInEquation(eqItem.clone())?).clone())?;
            transitions = listAppend(list![transition.clone()], transitions.clone());
            getTransitionsInEquations(xs.clone(), transitions.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: xs }, _) => {
            getTransitionsInEquations(xs.clone(), inTransitions.clone())?
        },
        (Deref @ metamodelica::List::Nil, _) => {
            inTransitions.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTransitions)
}

fn getTransitionInEquation(mut inEquation: Arc<Absyn::Equation>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outTransition: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outTransition = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ Absyn::Equation::EQ_NORETCALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: namedArgs, args: expArgs }, .. } => {
            let mut transition: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            transition = List::map(expArgs.clone(), (std::sync::Arc::new(Dump::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>));
            transition = Interactive::addOrUpdateNamedArg(namedArgs.clone(), (literal!("immediate")).clone(), (literal!("true")).clone(), transition.clone(), 4)?;
            transition = Interactive::addOrUpdateNamedArg(namedArgs.clone(), (literal!("reset")).clone(), (literal!("true")).clone(), transition.clone(), 5)?;
            transition = Interactive::addOrUpdateNamedArg(namedArgs.clone(), (literal!("synchronize")).clone(), (literal!("false")).clone(), transition.clone(), 6)?;
            transition = Interactive::addOrUpdateNamedArg(namedArgs.clone(), (literal!("priority")).clone(), (literal!("1")).clone(), transition.clone(), 7)?;
            transition.clone()
        },
        _ => {
            list![(literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("true")).clone(), (literal!("true")).clone(), (literal!("false")).clone(), (literal!("1")).clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTransition)
}

fn getInitialStates(mut path: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut initialStates: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    cdef = InteractiveUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
    initialStates = getInitialStatesInClass(cdef.clone())?.reverse();
    res = ValuesMake::makeArray(List::map(initialStates.clone(), (std::sync::Arc::new(ValuesMake::makeStringArray) as std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Values::Value>> + 'static>)))?;
    Ok(res)
}

fn getInitialStatesInClass(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outInitialStates: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    outInitialStates = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
            let mut initialStates: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            initialStates = getInitialStatesInClassParts(parts.clone())?;
            initialStates.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
            let mut initialStates: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
            initialStates = getInitialStatesInClassParts(parts.clone())?;
            initialStates.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. } => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInitialStates)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getInitialStatesInClassParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outInitialStates: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    outInitialStates = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: eqlist }, tail: xs } => {
                    let mut initialStates1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    let mut initialStates2: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    initialStates1 = getInitialStatesInEquations(eqlist.clone(), metamodelica::nil())?;
                    initialStates2 = getInitialStatesInClassParts(xs.clone())?;
                    Ok(listAppend(initialStates1.clone(), initialStates2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
                    let mut initialStates1: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
                    initialStates1 = getInitialStatesInClassParts(xs.clone())?;
                    Ok(initialStates1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outInitialStates)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getInitialStatesInEquations(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut inInitialStates: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>> {
    let mut outInitialStates: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    outInitialStates = (::match_deref::match_deref! { match &((inAbsynEquationItemLst.clone(), inInitialStates.clone())) {
        (Deref @ metamodelica::List::Cons { head: eqItem @ Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq @ Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "initialState", .. }, .. }, .. }, tail: xs }, initialStates) => {
            let mut initialState: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut initialStates = (*initialStates).clone();
            initialState = getInitialStateInEquation(eq.clone());
            initialState = List::insert(initialState.clone(), (initialState.clone().len() as i32) + 1, (getAnnotationInEquation(eqItem.clone())?).clone())?;
            initialStates = listAppend(list![initialState.clone()], initialStates.clone());
            getInitialStatesInEquations(xs.clone(), initialStates.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: xs }, _) => {
            getInitialStatesInEquations(xs.clone(), inInitialStates.clone())?
        },
        (Deref @ metamodelica::List::Nil, _) => {
            inInitialStates.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outInitialStates)
}

fn getInitialStateInEquation(mut inEquation: Arc<Absyn::Equation>) -> Arc<metamodelica::List<ArcStr>> {
    let mut outInitialState: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outInitialState = (::match_deref::match_deref! { match &(inEquation.clone()) {
        Deref @ Absyn::Equation::EQ_NORETCALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expArgs, .. }, .. } => {
            let mut initialState: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            initialState = List::map(expArgs.clone(), (std::sync::Arc::new(Dump::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>));
            initialState.clone()
        },
        _ => {
            list![(literal!("")).clone()]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outInitialState
}

fn addInitialState(mut inPath: Arc<Absyn::Path>, mut state: ArcStr, mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut b: bool = false;
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    (b, outProgram) = addInitialStateWithAnnotation(inPath.clone(), (state.clone()).clone(), InteractiveUtil::annotationListToAbsyn(inAbsynNamedArgLst.clone())?, inProgram.clone())?;
    Ok((b, outProgram))
}

fn addInitialStateWithAnnotation(mut inPath: Arc<Absyn::Path>, mut state: ArcStr, mut inAnnotation: Arc<Absyn::Annotation>, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut b: bool = false;
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    (b, outProgram) = (::match_deref::match_deref! { match &((inPath.clone(), state.clone(), inProgram.clone())) {
        (modelpath, state_, p @ Absyn::Program { .. }) => {
            let mut package_: Arc<Absyn::Path>;
            let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut newcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut cmt: Option<Arc<Absyn::Comment>> = None;
            cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
            cmt = Some(Arc::new(Absyn::Comment { annotation_: Some(inAnnotation.clone()), comment: None }));
            newcdef = InteractiveUtil::addToEquation(cdef.clone(), Arc::new(Absyn::EquationItem::EQUATIONITEM { equation_: Arc::new(Absyn::Equation::EQ_NORETCALL { functionName: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("initialState")).clone(), subscripts: metamodelica::nil() }), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::CREF { componentRef: Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (state_.clone()).clone(), subscripts: metamodelica::nil() }) })], argNames: metamodelica::nil() }) }), comment: cmt.clone(), info: Absyn::dummyInfo.clone() }))?;
            if AbsynUtil::pathIsIdent(AbsynUtil::makeNotFullyQualified(modelpath.clone())) {
                newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: p.within_.clone() }, p.clone(), false)?;
            } else {
                package_ = AbsynUtil::stripLast(modelpath.clone())?;
                newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: package_.clone() } }, p.clone(), false)?;
            }
            (true, newp.clone())
        },
        (_, _, p @ Absyn::Program { .. }) => {
            (false, p.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((b, outProgram))
}

fn deleteInitialState(mut inPath: Arc<Absyn::Path>, mut state: ArcStr, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut b: bool = false;
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    (b, outProgram) = 'mc: {
        let __mc_input = (inPath.clone(), state.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, state_, p @ Absyn::Program { .. }) => {
                    let mut modelwithin: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newcdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    cdef = InteractiveUtil::getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    newcdef = deleteInitialStateInClass(cdef.clone(), (state_.clone()).clone())?;
                    if AbsynUtil::pathIsIdent(AbsynUtil::makeNotFullyQualified(modelpath.clone())) {
                        newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), false)?;
                    } else {
                        modelwithin = AbsynUtil::stripLast(modelpath.clone())?;
                        newp = InteractiveUtil::updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: modelwithin.clone() } }, p.clone(), false)?;
                    }
                    Ok((true, newp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, p @ Absyn::Program { .. }) => {
                    Ok((false, p.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((b, outProgram))
}

fn deleteInitialStateInClass(mut inClass: Arc<Absyn::Class>, mut state: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &(inClass.clone()) {
        outClass @ Deref @ Absyn::Class { info: _, body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. } => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = InteractiveUtil::getEquationList(parts.clone())?;
            eqlst_1 = deleteInitialStateInEqlist(eqlst.clone(), (state.clone()).clone())?;
            parts2 = InteractiveUtil::replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: modif, baseClassName: bcname }, .. } => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = InteractiveUtil::getEquationList(parts.clone())?;
            eqlst_1 = deleteInitialStateInEqlist(eqlst.clone(), (state.clone()).clone())?;
            parts2 = InteractiveUtil::replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deleteInitialStateInEqlist(mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut state: ArcStr) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    outAbsynEquationItemLst = 'mc: {
        let __mc_input = (inAbsynEquationItemLst.clone(), state.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_NORETCALL { functionName: name, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: expArgs, argNames: _ } }, .. }, tail: xs }, state_) => {
                    if !((AbsynUtil::crefEqual(name.clone(), Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (literal!("initialState")).clone(), subscripts: metamodelica::nil() })))) { bail!("guard") }
                    let mut args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    args = List::map(expArgs.clone(), (std::sync::Arc::new(Dump::printExpStr) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<ArcStr> + 'static>));
                    let true = (compareInitialStateFuncArgs(args.clone(), (state_.clone()).clone())?) else { bail!("pattern mismatch") };
                    Ok(deleteInitialStateInEqlist(xs.clone(), (state_.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: x, tail: xs }, state_) => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
                    res = deleteInitialStateInEqlist(xs.clone(), (state_.clone()).clone())?;
                    Ok(cons(x.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynEquationItemLst)
}

fn compareInitialStateFuncArgs(mut args: Arc<metamodelica::List<ArcStr>>, mut state: ArcStr) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = args.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: state1, tail: Deref @ metamodelica::List::Nil } => {
                    if !((stringEq((state1.clone()).clone(), (state.clone()).clone())?)) { bail!("guard") }
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
    Ok(b)
}

fn getComponentInfo(mut comp: Arc<Absyn::Element>, mut inEnv: Interactive::GraphicEnvCache, mut isProtected: bool) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut vs: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    vs = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::COMPONENTS { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: p, arrayDim: _ }, attributes: attr @ _, .. }, .. } => {
            let mut p_1: Arc<Absyn::Path>;
            let mut typename: ArcStr = arcstr::literal!("");
            let mut inout_str: ArcStr = arcstr::literal!("");
            let mut variability_str: ArcStr = arcstr::literal!("");
            let mut dir_str: ArcStr = arcstr::literal!("");
            let mut name: ArcStr = arcstr::literal!("");
            let mut comment: ArcStr = arcstr::literal!("");
            let mut r_1: bool = false;
            let mut dims: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dims1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
            typename = ('mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut p_1: Arc<Absyn::Path>;
            (_, p_1) = Interactive::mkFullyQual(inEnv.clone(), p.clone(), false)?;
            Ok(AbsynUtil::pathString(p_1.clone(), (literal!(".")).clone(), true, false)?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
            vs = metamodelica::nil();
            dims1 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut sub in (attr.arrayDim.clone()).into_iter().cloned() {
            let __x = Dump::printSubscriptStr(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            r_1 = Interactive::keywordReplaceable(var_field!((*comp).redeclareKeywords, Absyn::Element::ELEMENT).clone());
            inout_str = (AbsynUtil::innerOuterStr(var_field!((*comp).innerOuter, Absyn::Element::ELEMENT).clone())?).clone();
            variability_str = (attrVariabilityStr(attr.clone())?).clone();
            dir_str = (attrDirectionStr(attr.clone())?).clone();
            for mut ci in &*var_field!((**spec).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let mut ci = ci.clone();
                (name, comment) = getComponentitemsName(ci.clone())?;
                let __pa0 = ::match_deref::match_deref! { match &(ci.clone()) {
                    Deref @ Absyn::ComponentItem { component: Absyn::Component { arrayDim: __pa0, .. }, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                subs = __pa0.clone();
                dims = listAppend({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = Dump::printSubscriptStr(sub.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, dims1.clone());
                vs = cons(makeGetComponentsRecord((typename.clone()).clone(), (name.clone()).clone(), (comment.clone()).clone(), isProtected.clone(), var_field!((*comp).finalPrefix, Absyn::Element::ELEMENT).clone(), attr.flowPrefix.clone(), attr.streamPrefix.clone(), r_1.clone(), (variability_str.clone()).clone(), (inout_str.clone()).clone(), (dir_str.clone()).clone(), dims.clone())?, vs.clone());
            }
            vs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(vs)
}

fn makeGetComponentsRecord(mut className: ArcStr, mut name: ArcStr, mut comment: ArcStr, mut isProtected: bool, mut isFinal: bool, mut isFlow: bool, mut isStream: bool, mut isReplaceable: bool, mut variability: ArcStr, mut innerOuter: ArcStr, mut inputOutput: ArcStr, mut dimensions: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Values::Value>> {
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    v = Arc::new(Values::Value::RECORD { record_: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("OpenModelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Scripting")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("getComponentsTest")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("Component")).clone() }) }) }) }), orderd: list![Arc::new(Values::Value::STRING { string: (className.clone()).clone() }), Arc::new(Values::Value::STRING { string: (name.clone()).clone() }), Arc::new(Values::Value::STRING { string: (comment.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: isProtected.clone() }), Arc::new(Values::Value::BOOL { boolean: isFinal.clone() }), Arc::new(Values::Value::BOOL { boolean: isFlow.clone() }), Arc::new(Values::Value::BOOL { boolean: isStream.clone() }), Arc::new(Values::Value::BOOL { boolean: isReplaceable.clone() }), Arc::new(Values::Value::STRING { string: (variability.clone()).clone() }), Arc::new(Values::Value::STRING { string: (innerOuter.clone()).clone() }), Arc::new(Values::Value::STRING { string: (inputOutput.clone()).clone() }), ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut s in (dimensions.clone()).into_iter().cloned() {
            let __x = Arc::new(Values::Value::STRING { string: (s.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?], comp: list![(literal!("className")).clone(), (literal!("name")).clone(), (literal!("comment")).clone(), (literal!("isProtected")).clone(), (literal!("isFinal")).clone(), (literal!("isFlow")).clone(), (literal!("isStream")).clone(), (literal!("isReplaceable")).clone(), (literal!("variability")).clone(), (literal!("innerOuter")).clone(), (literal!("inputOutput")).clone(), (literal!("dimensions")).clone()], index: -1 });
    Ok(v)
}

fn attrVariabilityStr(mut inElementAttributes: Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inElementAttributes.clone() {
        Absyn::ElementAttributes { variability: Absyn::Variability::VAR, .. } => literal!(""),
        Absyn::ElementAttributes { variability: Absyn::Variability::DISCRETE, .. } => literal!("discrete"),
        Absyn::ElementAttributes { variability: Absyn::Variability::PARAM, .. } => literal!("parameter"),
        Absyn::ElementAttributes { variability: Absyn::Variability::CONST, .. } => literal!("constant"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn attrDirectionStr(mut inElementAttributes: Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inElementAttributes.clone() {
        Absyn::ElementAttributes { direction: Absyn::Direction::INPUT, .. } => literal!("input"),
        Absyn::ElementAttributes { direction: Absyn::Direction::OUTPUT, .. } => literal!("output"),
        Absyn::ElementAttributes { direction: Absyn::Direction::BIDIR, .. } => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn getComponentitemsName(mut ci: Arc<Absyn::ComponentItem>) -> Result<(ArcStr, ArcStr)> {
    let mut name: ArcStr = arcstr::literal!("");
    let mut comment: ArcStr = arcstr::literal!("");
    (name, comment) = (::match_deref::match_deref! { match &(ci.clone()) {
        Deref @ Absyn::ComponentItem { comment: Some(Deref @ Absyn::Comment { annotation_: _, comment: Some(s2) }), component: Absyn::Component { name: c1, .. }, .. } => {
            (c1.clone(), s2.clone())
        },
        Deref @ Absyn::ComponentItem { comment: Some(Deref @ Absyn::Comment { annotation_: _, comment: _ }), component: Absyn::Component { name: c1, .. }, .. } => {
            (c1.clone(), literal!(""))
        },
        Deref @ Absyn::ComponentItem { comment: None, component: Absyn::Component { name: c1, .. }, .. } => {
            (c1.clone(), literal!(""))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((name, comment))
}

fn getAnnotationNamedModifiers(mut classPath: Arc<Absyn::Path>, mut annotationName: ArcStr, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    fn get_names(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
        let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        let mut m: Arc<Absyn::Modification>;
        let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        names = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(m) => {
            paths = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut a in (m.elementArgLst.clone()).into_iter().cloned() {
            let __x = AbsynUtil::elementArgName(a.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
            if !(AbsynUtil::pathIsIdent(p.clone())) { continue; }
            let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        Ok(names)
    }

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::getNamedAnnotationInClass(cls.clone(), Arc::new(Absyn::Path::IDENT { name: (annotationName.clone()).clone() }), (std::sync::Arc::new(get_names) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>))?) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    names = __pa0.clone();
    result = ValuesMake::makeStringArray(names.clone())?;
    Ok(result)
}

fn getOptModifierValue(mut modifier: Option<Arc<Absyn::Modification>>) -> Result<Arc<Values::Value>> {
    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#mod: Arc<Absyn::Modification>;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let __pa0 = ::match_deref::match_deref! { match &(modifier.clone()) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#mod = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(r#mod.eqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { exp: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    exp = __pa1.clone();
    value = ValuesUtil::absynExpValue(exp.clone())?;
    Ok(value)
}

fn getAnnotationModifierValue(mut classPath: Arc<Absyn::Path>, mut annotationName: ArcStr, mut modifierName: ArcStr, mut program: Absyn::Program) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    cls = InteractiveUtil::getPathedClassInProgram(classPath.clone(), program.clone(), false, false)?;
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::getNamedAnnotationInClass(cls.clone(), Arc::new(Absyn::Path::QUALIFIED { name: (annotationName.clone()).clone(), path: Arc::new(Absyn::Path::IDENT { name: (modifierName.clone()).clone() }) }), (std::sync::Arc::new(getOptModifierValue) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<Values::Value>> + 'static>))?) {
        Some(__pa0) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    result = __pa0.clone();
    Ok(result)
}

fn makeLoadLibrariesEntryAbsyn(mut cl: Arc<Absyn::Class>, mut acc: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut out: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    out = (::match_deref::match_deref! { match &((cl.clone(), acc.clone())) {
        (Deref @ Absyn::Class { info: SourceInfo { fileName: Deref @ "<interactive>", .. }, .. }, _) => {
            acc.clone()
        },
        (Deref @ Absyn::Class { info: SourceInfo { fileName, .. }, name, .. }, _) => {
            let mut dir: ArcStr = arcstr::literal!("");
            let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut b: bool = false;
            let mut fileName = (*fileName).clone();
            dir = (System::dirname((fileName.clone()).clone())).clone();
            fileName = (System::basename((fileName.clone()).clone())).clone();
            v = ValuesMake::makeArray(list![Arc::new(Values::Value::STRING { string: (name.clone()).clone() }), Arc::new(Values::Value::STRING { string: (dir.clone()).clone() })])?;
            b = stringEq((fileName.clone()).clone(), (literal!("ModelicaBuiltin.mo")).clone())? || stringEq((fileName.clone()).clone(), (literal!("MetaModelicaBuiltin.mo")).clone())? || stringEq((dir.clone()).clone(), (literal!(".")).clone())?;
            List::consOnTrue(!(b.clone()), v.clone(), acc.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out)
}

fn selectResultFile(mut resultFile: ArcStr, mut simflags: ArcStr) -> Result<ArcStr> {
    let mut resultFile: ArcStr = resultFile;
    let mut nm: i32 = 0;
    let mut f: ArcStr = literal!("");
    if System::stringFind((simflags.clone()).clone(), (literal!("-r")).clone())? < 0 {
        return Ok(resultFile);
    }
    if '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(System::regex((simflags.clone()).clone(), (literal!("-r=\"(.*?)\"")).clone(), 2, true, false)) {
            (__pa1, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa2, tail: Deref @ metamodelica::List::Nil } }) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        nm = __pa1.clone();
        f = __pa2.clone();
        if nm.clone() == 2 {
            resultFile = (f.clone()).clone();
            return Ok(resultFile);
        }
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(System::regex((simflags.clone()).clone(), (literal!("-r='(.*?)'")).clone(), 2, true, false)) {
            (__pa4, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa5, tail: Deref @ metamodelica::List::Nil } }) => (__pa4.clone(), __pa5.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        nm = __pa4.clone();
        f = __pa5.clone();
        if nm.clone() == 2 {
            resultFile = (f.clone()).clone();
            return Ok(resultFile);
        }
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(System::regex((simflags.clone()).clone(), (literal!("-r[ ]*\"(.*?)\"")).clone(), 2, true, false)) {
            (__pa7, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa8, tail: Deref @ metamodelica::List::Nil } }) => (__pa7.clone(), __pa8.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        nm = __pa7.clone();
        f = __pa8.clone();
        if nm.clone() == 2 {
            resultFile = (f.clone()).clone();
            return Ok(resultFile);
        }
        let (__pa10, __pa11) = ::match_deref::match_deref! { match &(System::regex((simflags.clone()).clone(), (literal!("-r[ ]*'(.*?)'")).clone(), 2, true, false)) {
            (__pa10, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa11, tail: Deref @ metamodelica::List::Nil } }) => (__pa10.clone(), __pa11.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        nm = __pa10.clone();
        f = __pa11.clone();
        if nm.clone() == 2 {
            resultFile = (f.clone()).clone();
            return Ok(resultFile);
        }
        let (__pa13, __pa14) = ::match_deref::match_deref! { match &(System::regex((simflags.clone()).clone(), (literal!("-r=([^ ]*)")).clone(), 2, true, false)) {
            (__pa13, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa14, tail: Deref @ metamodelica::List::Nil } }) => (__pa13.clone(), __pa14.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        nm = __pa13.clone();
        f = __pa14.clone();
        if nm.clone() == 2 {
            resultFile = (f.clone()).clone();
            return Ok(resultFile);
        }
        let (__pa16, __pa17) = ::match_deref::match_deref! { match &(System::regex((simflags.clone()).clone(), (literal!("-r[ ]*([^ ]*)")).clone(), 2, true, false)) {
            (__pa16, Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa17, tail: Deref @ metamodelica::List::Nil } }) => (__pa16.clone(), __pa17.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        nm = __pa16.clone();
        f = __pa17.clone();
        if nm.clone() == 2 {
            resultFile = (f.clone()).clone();
            return Ok(resultFile);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    Ok(resultFile)
}

fn instantiateModel(mut cache: FCore::Cache, mut env: FCore::Graph, mut path: Arc<Absyn::Path>) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut cache: FCore::Cache = cache;
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut odae: Option<DAE::DAElist> = None;
    let mut flat_model: Arc<NFFlatModel::NFFlatModel>;
    let mut funcs: Arc<NFFlatten::FunctionTreeImpl::Tree> = Arc::new(NFFlatten::FunctionTreeImpl::Tree::EMPTY);
    let mut flags: Flags::Flag = Flags::Flag::NO_FLAGS;
    if isProtectedContentAccess(path.clone())? {
        result = Arc::new(Values::Value::STRING { string: (literal!("")).clone() });
        return Ok((cache, result));
    }
    r#str = ('mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = r#str.clone();
            let mut flags: Flags::Flag = flags.clone();
            let mut cache: FCore::Cache = cache.clone();
            let mut odae: Option<DAE::DAElist> = odae.clone();
            ExecStat::execStatReset()?;
            flags = loadCommandLineOptionsFromModel(path.clone())?;
            match '__try0: {
                (cache, _, odae, r#str) = unwrap_break_err!(runFrontEnd(cache.clone(), env.clone(), path.clone(), false, Config::flatModelica()? && !(Config::silent()?), false), '__try0);
                unwrap_break_err!(ExecStat::execStat((literal!("runFrontEnd")).clone()), '__try0);
                if !(stringEmpty((r#str.clone()).clone())) {
                } else if isNone(odae.clone()) {
                    r#str = (literal!("")).clone();
                } else if unwrap_break_err!(Config::silent(), '__try0) {
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("model ")); __mm_s.push_str(&*unwrap_break_err!(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false), '__try0)); __mm_s.push_str(&*literal!("\n  /* Silent mode */\nend")); __mm_s.push_str(&*unwrap_break_err!(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false), '__try0)); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }).clone();
                } else {
                    r#str = unwrap_break_err!(DAEDump::dumpStr(Util::getOption(odae.clone())?, FCore::getFunctionTree(cache.clone())), '__try0);
                    unwrap_break_err!(ExecStat::execStat((literal!("DAEDump.dumpStr")).clone()), '__try0);
                }
                FlagsUtil::saveFlags(flags.clone());
                Ok::<_, anyhow::Error>((cache.clone(), odae.clone(), r#str.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    cache = __try0_o0;
                    odae = __try0_o1;
                    r#str = __try0_o2;
                }
                Err(_) => {
                    FlagsUtil::saveFlags(flags.clone());
                    bail!("fail");
                }
            }
            Ok(r#str.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let false = (Interactive::existClass(path.clone(), SymbolTable::getAbsyn())) else { bail!("pattern mismatch") };
            Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), (literal!("<TOP>")).clone()])?;
            Ok(literal!(""))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = r#str.clone();
            if Error::getNumMessages() == 0 {
                r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Instantiation of ")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" failed with no error message")); ArcStr::from(__mm_s) }).clone();
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone(), (literal!("<TOP>")).clone()])?;
            }
            Ok(literal!(""))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    result = Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() });
    Ok((cache, result))
}

fn getConnectionList(mut className: Arc<Absyn::Path>) -> Result<Arc<Values::Value>> {
    let mut valList: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut annotation_sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut connList: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    annotation_sp = AbsynToSCode::translateAbsyn2SCode(InteractiveUtil::modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?)?;
    (_, sp) = FBuiltin::getInitialFunctions()?;
    sp = listAppend(SymbolTable::getSCode()?, sp.clone());
    connList = NFInst::instClassForConnection(className.clone(), sp.clone(), annotation_sp.clone())?;
    valList = ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut conn in (connList.clone()).into_iter().cloned() {
            let __x = ValuesMake::makeArray(List::map(conn.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
    Ok(valList)
}

fn runConversionScript(mut clsPath: Arc<Absyn::Path>, mut scriptFile: ArcStr) -> Arc<Values::Value> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut wi: Absyn::Within = Absyn::Within::TOP;
    match '__try0: {
        p = SymbolTable::getAbsyn();
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(clsPath.clone(), p.clone(), false, true), '__try0);
        cls = unwrap_break_err!(Conversion::convertPackage(cls.clone(), (scriptFile.clone()).clone()), '__try0);
        wi = unwrap_break_err!(InteractiveUtil::buildWithin(clsPath.clone()), '__try0);
        p = unwrap_break_err!(InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: wi.clone() }, p.clone(), false), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(p.clone()), '__try0);
        res = Arc::new(Values::Value::BOOL { boolean: true });
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = Arc::new(Values::Value::BOOL { boolean: false });
        }
    }
    res
}

fn convertPackageToLibrary(mut clsPath: Arc<Absyn::Path>, mut libPath: Arc<Absyn::Path>, mut libVersion: ArcStr) -> Arc<Values::Value> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut lib_program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut lib_cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut wi: Absyn::Within = Absyn::Within::TOP;
    let mut cls_uses: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut lib_converts_from: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut b: bool = false;
    let mut has_conversion: bool = false;
    let mut uses_version: Option<ArcStr> = None;
    let mut lib_version: SemanticVersion::Version;
    let mut lib_version_used: SemanticVersion::Version;
    let mut conversions: Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>> = metamodelica::nil();
    let mut scripts: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut lib_name: ArcStr = arcstr::literal!("");
    match '__try0: {
        p = SymbolTable::getAbsyn();
        cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(clsPath.clone(), p.clone(), false, true), '__try0);
        uses_version = unwrap_break_err!(Interactive::getUsedVersion(cls.clone(), libPath.clone()), '__try0);
        if isSome(uses_version.clone()) {
            lib_version_used = unwrap_break_err!(SemanticVersion::parse((Util::getOption(uses_version.clone()).unwrap()).clone(), true), '__try0);
        } else {
            unwrap_break_err!(Error::addMessage(Error::CONVERSION_MISSING_USES.clone(), list![(AbsynUtil::pathString(clsPath.clone(), (literal!(".")).clone(), true, false).unwrap()).clone(), (AbsynUtil::pathString(libPath.clone(), (literal!(".")).clone(), true, false).unwrap()).clone()]), '__try0);
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        lib_name = (unwrap_break_err!(AbsynUtil::pathFirstIdent(libPath.clone()), '__try0)).clone();
        lib_version = unwrap_break_err!(SemanticVersion::parse((CevalScript::getPackageVersion(libPath.clone(), p.clone()).unwrap()).clone(), false), '__try0);
        if unwrap_break_err!(SemanticVersion::compare(lib_version.clone(), SemanticVersion::parse((libVersion.clone()).clone(), false).unwrap(), true, false), '__try0) != 0 {
            if lib_name.clone() == literal!("Modelica") {
                unwrap_break_err!(Config::setLanguageStandardFromMSL(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Modelica ")); __mm_s.push_str(&*libVersion.clone()); ArcStr::from(__mm_s) }).clone(), true), '__try0);
            }
            let (__pa1, true) = (unwrap_break_err!(CevalScript::loadModel(list![(libPath.clone(), lib_name.clone(), list![(libVersion.clone()).clone()], false)], (Settings::getModelicaPath(Testsuite::isRunning().unwrap()).unwrap()).clone(), p.clone(), true, true, false, true, false, (literal!("")).clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
            lib_program = __pa1.clone();
            unwrap_break_err!(SymbolTable::setAbsyn(lib_program.clone()), '__try0);
        } else {
            lib_program = p.clone();
        }
        lib_version = unwrap_break_err!(SemanticVersion::parse((CevalScript::getPackageVersion(libPath.clone(), lib_program.clone()).unwrap()).clone(), false), '__try0);
        lib_cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(libPath.clone(), lib_program.clone(), false, true), '__try0);
        conversions = unwrap_break_err!(Interactive::getConversionsInClass(lib_cls.clone()), '__try0);
        scripts = unwrap_break_err!(findConversionPaths(conversions.clone(), lib_version.clone(), lib_version_used.clone(), 0), '__try0);
        if scripts.clone().is_empty() {
            unwrap_break_err!(Error::addMessage(Error::CONVERSION_NO_COMPATIBLE_SCRIPT_FOUND.clone(), list![(AbsynUtil::pathString(libPath.clone(), (literal!(".")).clone(), true, false).unwrap()).clone(), (SemanticVersion::toString(lib_version_used.clone()).unwrap()).clone(), (SemanticVersion::toString(lib_version.clone()).unwrap()).clone()]), '__try0);
            break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
        }
        for mut script in &*scripts.clone() {
            let mut script = script.clone();
            script = uriToFilename((script.clone()).clone());
            cls = unwrap_break_err!(Conversion::convertPackage(cls.clone(), (script.clone()).clone()), '__try0);
        }
        cls = unwrap_break_err!(Interactive::updateUsedVersion(cls.clone(), libPath.clone(), (SemanticVersion::toString(lib_version.clone()).unwrap()).clone()), '__try0);
        wi = unwrap_break_err!(InteractiveUtil::buildWithin(clsPath.clone()), '__try0);
        lib_program = unwrap_break_err!(InteractiveUtil::updateProgram(Absyn::Program { classes: list![cls.clone()], within_: wi.clone() }, lib_program.clone(), false), '__try0);
        unwrap_break_err!(SymbolTable::setAbsyn(lib_program.clone()), '__try0);
        res = Arc::new(Values::Value::BOOL { boolean: true });
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = Arc::new(Values::Value::BOOL { boolean: false });
        }
    }
    res
}

fn findConversionPaths(mut conversions: Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>>, mut libVersion: SemanticVersion::Version, mut libVersionUsed: SemanticVersion::Version, mut depth: i32) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut scripts: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut version: ArcStr = arcstr::literal!("");
    let mut paths: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut path_len: i32 = 0;
    let mut path_min: i32 = 100;
    if depth.clone() > 100 {
        return Ok(scripts);
    }
    for mut c in &*conversions.clone() {
        let mut c = c.clone();
        paths = cons(findConversionPath(c.clone(), libVersion.clone(), libVersionUsed.clone(), conversions.clone(), depth.clone())?, paths.clone());
    }
    for mut p in &*paths.clone() {
        let mut p = p.clone();
        path_len = (p.clone().len() as i32);
        if path_len.clone() > 0 && path_len.clone() < path_min.clone() {
            scripts = p.clone();
            path_min = path_len.clone();
        }
    }
    Ok(scripts)
}

fn findConversionPath(mut conversion: (ArcStr, Option<ArcStr>, Option<ArcStr>), mut libVersion: SemanticVersion::Version, mut libVersionUsed: SemanticVersion::Version, mut conversions: Arc<metamodelica::List<(ArcStr, Option<ArcStr>, Option<ArcStr>)>>, mut depth: i32) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut scripts: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut from: ArcStr = arcstr::literal!("");
    let mut to: Option<ArcStr> = None;
    let mut script: Option<ArcStr> = None;
    let mut from_version: SemanticVersion::Version;
    let mut to_version: SemanticVersion::Version;
    (from, to, script) = conversion.clone();
    if isNone(script.clone()) {
        return Ok(scripts);
    }
    from_version = SemanticVersion::parse((from.clone()).clone(), true)?;
    if SemanticVersion::compare(libVersionUsed.clone(), from_version.clone(), true, false)? == 0 {
        if isSome(to.clone()) {
            to_version = SemanticVersion::parse((Util::getOption(to.clone())?).clone(), true)?;
            if SemanticVersion::compare(libVersion.clone(), to_version.clone(), true, false)? != 0 {
                scripts = findConversionPaths(conversions.clone(), libVersion.clone(), to_version.clone(), depth.clone() + 1)?;
            }
        }
        scripts = cons((Util::getOption(script.clone())?).clone(), scripts.clone());
    }
    Ok(scripts)
}

pub fn loadCommandLineOptionsFromModel(mut className: Arc<Absyn::Path>) -> Result<Flags::Flag> {
    let mut oldFlags: Flags::Flag = Flags::Flag::NO_FLAGS;
    let mut opts: ArcStr = arcstr::literal!("");
    let mut args: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if Config::ignoreCommandLineOptionsAnnotation()? {
        oldFlags = FlagsUtil::loadFlags(true)?;
        return Ok(oldFlags);
    }
    loadProgram(className.clone())?;
    let Absyn::STRING { value: __pa0 } = (Interactive::getNamedAnnotationExp(className.clone(), SymbolTable::getAbsyn(), Arc::new(Absyn::Path::IDENT { name: (literal!("__OpenModelica_commandLineOptions")).clone() }), Some(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })), (std::sync::Arc::new(Interactive::getAnnotationExp) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<Absyn::Exp>> + 'static>))?) else { bail!("pattern mismatch") };
    opts = __pa0.clone();
    if !(stringEmpty((opts.clone()).clone())) {
        oldFlags = FlagsUtil::backupFlags()?;
        args = System::strtok((opts.clone()).clone(), (literal!(" ")).clone());
        FlagsUtil::readArgs(args.clone())?;
    } else {
        oldFlags = FlagsUtil::loadFlags(true)?;
    }
    Ok(oldFlags)
}

pub fn isProtectedContentAccess(mut className: Arc<Absyn::Path>) -> Result<bool> {
    let mut restricted: bool = false;
    loadProgram(className.clone())?;
    restricted = Interactive::astContainsEncryptedClass(SymbolTable::getAbsyn())?;
    if restricted.clone() {
        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
    }
    Ok(restricted)
}

