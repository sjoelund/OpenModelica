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

use crate::BackendDAE;
use crate::BackendDAECreate;
use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::CevalScriptBackend;
use crate::CodegenC;
use crate::CodegenCpp;
use crate::CodegenCppHpcom;
use crate::CodegenEmbeddedC;
use crate::CodegenFMU;
use crate::CodegenFMUCpp;
use crate::CodegenFMUCppHpcom;
use crate::CodegenJS;
use crate::CodegenOMSIC;
use crate::CodegenOMSICpp;
use crate::CodegenOMSI_common;
use crate::CodegenXML;
use crate::DAEMode;
use crate::HashTableCrefSimVar;
use crate::HpcOmSimCode;
use crate::HpcOmSimCodeMain;
use crate::HpcOmTaskGraph;
use crate::Interactive;
use crate::NBackendDAE;
use crate::NSimCode;
use crate::RuntimeSources;
use crate::SerializeInitXML;
use crate::SerializeModelInfo;
use crate::SerializeSparsityPattern;
use crate::SerializeTaskSystemInfo;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeFunctionUtil;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use crate::SymbolTable;
use crate::SymbolicJacobian;
use crate::ZeroCrossings;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Builtin;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::HashTable;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_frontend::HashTableCrILst;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_nf_frontend::NFConvertDAE;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFlatten::FunctionTree;
use openmodelica_nf_frontend::NFFlatten::FunctionTreeImpl;
use openmodelica_nf_frontend::NFFunction;
use openmodelica_susan::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::AvlSetString;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::FMI;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

/* used for new backend */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TranslateModelKind {
    NORMAL,
    XML,
    FMU {
        kind: ArcStr,
        targetName: ArcStr,
    },
}
pub use self::TranslateModelKind::{NORMAL,XML,FMU};

pub fn createSimulationSettings(mut startTime: metamodelica::Real, mut stopTime: metamodelica::Real, mut inumberOfIntervals: i32, mut tolerance: metamodelica::Real, mut method: ArcStr, mut options: ArcStr, mut outputFormat: ArcStr, mut variableFilter: ArcStr, mut cflags: ArcStr, mut simflags: ArcStr) -> SimCode::SimulationSettings {
    let mut simSettings: SimCode::SimulationSettings;
    let mut stepSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut numberOfIntervals: i32 = 0;
    numberOfIntervals = if (inumberOfIntervals.clone() <= 0) {1} else {inumberOfIntervals.clone()};
    stepSize = (stopTime.clone() - startTime.clone()) / intReal(numberOfIntervals.clone());
    simSettings = SimCode::SimulationSettings { startTime: startTime.clone(), stopTime: stopTime.clone(), numberOfIntervals: numberOfIntervals.clone(), stepSize: stepSize.clone(), tolerance: tolerance.clone(), method: (method.clone()).clone(), options: (options.clone()).clone(), outputFormat: (outputFormat.clone()).clone(), variableFilter: (variableFilter.clone()).clone(), cflags: (cflags.clone()).clone(), simflags: (simflags.clone()).clone() };
    simSettings
}

fn generateModelCodeFMU(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inFMIDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut FMUVersion: ArcStr, mut FMUType: ArcStr, mut filenamePrefix: ArcStr, mut fmuTargetName: ArcStr, mut simSettings: Option<SimCode::SimulationSettings>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut filename: ArcStr = arcstr::literal!("");
    let mut funcfilename: ArcStr = arcstr::literal!("");
    let mut simCode: SimCode::SimCode;
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut indexed_dlow: Arc<BackendDAE::BackendDAE>;
    let mut indexed_dlow_1: Arc<BackendDAE::BackendDAE>;
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(className.clone())?;
    if Config::simCodeTarget()? == literal!("omsic") {
        fileDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*listHead(AbsynUtil::pathToStringList(className.clone())?)?); __mm_s.push_str(&*literal!(".tmp")); ArcStr::from(__mm_s) }).clone();
    } else {
        fileDir = (CevalScriptBackend::getFileDir(a_cref.clone(), p.clone())?).clone();
    }
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = SimCodeUtil::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone())?;
    simCode = createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), className.clone(), (filenamePrefix.clone()).clone(), (fileDir.clone()).clone(), functions.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), p.clone(), simSettings.clone(), recordDecls.clone(), literals.clone(), Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }), true, (FMUVersion.clone()).clone(), (fmuTargetName.clone()).clone(), inFMIDer.clone())?;
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat((literal!("SimCode")).clone())?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    if Config::simCodeTarget()? == literal!("omsicpp") {
        callTargetTemplatesFMU(simCode.clone(), (literal!("C")).clone(), (FMUVersion.clone()).clone(), (FMUType.clone()).clone(), p.clone())?;
    } else {
        callTargetTemplatesFMU(simCode.clone(), (Config::simCodeTarget()?).clone(), (FMUVersion.clone()).clone(), (FMUType.clone()).clone(), p.clone())?;
    }
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn generateModelCodeXML(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut simSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut filename: ArcStr = arcstr::literal!("");
    let mut funcfilename: ArcStr = arcstr::literal!("");
    let mut simCode: SimCode::SimCode;
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut indexed_dlow: Arc<BackendDAE::BackendDAE>;
    let mut indexed_dlow_1: Arc<BackendDAE::BackendDAE>;
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    let mut program: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    a_cref = AbsynUtil::pathToCref(className.clone())?;
    fileDir = (CevalScriptBackend::getFileDir(a_cref.clone(), p.clone())?).clone();
    (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = SimCodeUtil::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone())?;
    (simCode, _) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), None, inRemovedInitialEquationLst.clone(), className.clone(), (filenamePrefix.clone()).clone(), (fileDir.clone()).clone(), functions.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), p.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }), false, (literal!("")).clone(), (literal!("")).clone(), metamodelica::nil())?;
    timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
    ExecStat::execStat((literal!("SimCode")).clone())?;
    System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    callTargetTemplatesXML(simCode.clone(), (Config::simCodeTarget()?).clone())?;
    timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

pub fn generateModelCode(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inInlineData: Option<BackendDAE::InlineData>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut simSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>, mut inFMIDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut simCode: SimCode::SimCode;
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    let mut program: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut numCheckpoints: i32 = 0;
    let mut fmuVersion: ArcStr = arcstr::literal!("");
    numCheckpoints = ErrorExt::getNumCheckpoints();
    if '__try0: {
        StackOverflow::clearStacktraceMessages();
        if unwrap_break_err!(Flags::isSet(Flags::GRAPHML.clone()), '__try0) {
            unwrap_break_err!(HpcOmTaskGraph::dumpTaskGraph(inBackendDAE.clone(), (filenamePrefix.clone()).clone()), '__try0);
            unwrap_break_err!(BackendDump::dumpBackendDAEBipartiteGraph(inBackendDAE.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BipartiteGraph_CompleteDAE_")); __mm_s.push_str(&*filenamePrefix.clone()); ArcStr::from(__mm_s) }).clone()), '__try0);
        }
        unwrap_break_err!(System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone()), '__try0);
        a_cref = unwrap_break_err!(AbsynUtil::pathToCref(className.clone()), '__try0);
        fileDir = (unwrap_break_err!(CevalScriptBackend::getFileDir(a_cref.clone(), p.clone()), '__try0)).clone();
        (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = unwrap_break_err!(SimCodeUtil::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone()), '__try0);
        simCode = unwrap_break_err!(createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inInlineData.clone(), inRemovedInitialEquationLst.clone(), className.clone(), (filenamePrefix.clone()).clone(), (fileDir.clone()).clone(), functions.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), p.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone(), false, (literal!("")).clone(), (literal!("")).clone(), inFMIDer.clone()), '__try0);
        timeSimCode = unwrap_break_err!(System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone()), '__try0);
        unwrap_break_err!(ExecStat::execStat((literal!("SimCode")).clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::SERIALIZED_SIZE.clone()), '__try0) {
            unwrap_break_err!(serializeNotify(simCode.clone(), (literal!("SimCode")).clone()), '__try0);
            unwrap_break_err!(ExecStat::execStat((literal!("Serialize simCode")).clone()), '__try0);
        }
        unwrap_break_err!(System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone()), '__try0);
        unwrap_break_err!(callTargetTemplates(simCode.clone(), (Config::simCodeTarget()?).clone()), '__try0);
        timeTemplates = unwrap_break_err!(System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone()), '__try0);
        unwrap_break_err!(ExecStat::execStat((literal!("Templates")).clone()), '__try0);
        return Ok((libs, fileDir, timeSimCode, timeTemplates));
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = None);
        ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints.clone());
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow in ")); __mm_s.push_str(&*literal!("SimCodeMain.generateModelCode")); __mm_s.push_str(&*literal!("...\n")); __mm_s.push_str(&*stringDelimitList(StackOverflow::readableStacktraceMessages()?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        StackOverflow::clearStacktraceMessages();
    }
    bail!("fail");
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn createSimCode(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>>, mut inInlineData: Option<BackendDAE::InlineData>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut inClassName: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut inString11: ArcStr, mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut externalFunctionIncludes: Arc<metamodelica::List<ArcStr>>, mut includeDirs: Arc<metamodelica::List<ArcStr>>, mut libs: Arc<metamodelica::List<ArcStr>>, mut libPaths: Arc<metamodelica::List<ArcStr>>, mut program: Absyn::Program, mut simSettingsOpt: Option<SimCode::SimulationSettings>, mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>, mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>), mut args: Arc<Absyn::FunctionArgs>, mut isFMU: bool, mut FMUVersion: ArcStr, mut fmuTargetName: ArcStr, mut inFMIDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>>) -> Result<SimCode::SimCode> {
    let mut simCode: SimCode::SimCode;
    simCode = 'mc: {
        let __mc_input = (inBackendDAE.clone(), inClassName.clone(), filenamePrefix.clone(), inString11.clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::MULTIRATE_PARTITION.clone())?) else { bail!("pattern mismatch") };
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut numProc: i32 = 0;
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { bail!("pattern mismatch") };
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    let true = (numProc.clone() == 0) else { bail!("pattern mismatch") };
                    println!("{}", (literal!("hpcom computes the ideal number of processors. If you want to set the number manually, use the flag +n=_\n")).clone());
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    let mut numProc: i32 = 0;
                    let true = (Flags::isSet(Flags::HPCOM.clone())?) else { bail!("pattern mismatch") };
                    numProc = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
                    let true = (numProc.clone() > 0) else { bail!("pattern mismatch") };
                    Ok(HpcOmSimCodeMain::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tmpSimCode: SimCode::SimCode;
                    (tmpSimCode, _) = SimCodeUtil::createSimCode(inBackendDAE.clone(), inInitDAE.clone(), inInitDAE_lambda0.clone(), inInlineData.clone(), inRemovedInitialEquationLst.clone(), inClassName.clone(), (filenamePrefix.clone()).clone(), (inString11.clone()).clone(), functions.clone(), externalFunctionIncludes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), program.clone(), simSettingsOpt.clone(), recordDecls.clone(), literals.clone(), args.clone(), isFMU.clone(), (FMUVersion.clone()).clone(), (fmuTargetName.clone()).clone(), inFMIDer.clone())?;
                    Ok(tmpSimCode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(simCode)
}

type PartialRunTpl = fn() -> Result<(bool, Arc<metamodelica::List<ArcStr>>)>;

type FuncText = fn(Tpl::Text) -> Result<Tpl::Text>;

fn runTplWriteFile(mut func: FuncText, mut file: ArcStr) -> (bool, Arc<metamodelica::List<ArcStr>>) {
    let mut res: (bool, Arc<metamodelica::List<ArcStr>>);
    let mut nErr: i32 = 0;
    res = (false, metamodelica::nil());
    match '__try0: {
        unwrap_break_err!(SimCodeUtil::resetFunctionIndex(), '__try0);
        SimCodeFunctionUtil::codegenResetTryThrowIndex();
        if unwrap_break_err!(Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone()), '__try0) {
            unwrap_break_err!(Tpl::textFileConvertLines(Tpl::tplCallWithFailErrorNoArg(Arc::new(func), Tpl::emptyTxt.clone()).unwrap(), (file.clone()).clone()), '__try0);
        } else {
            nErr = Error::getNumErrorMessages();
            Tpl::closeFile(Tpl::tplCallWithFailErrorNoArg(Arc::new(func), Tpl::redirectToFile(Tpl::emptyTxt.clone(), (file.clone()).clone()).unwrap()).unwrap());
            unwrap_break_err!(Tpl::failIfTrue(Error::getNumErrorMessages() > nErr.clone()), '__try0);
        }
        res = (true, SimCodeUtil::getFunctionIndex());
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    res
}

fn runTpl(mut func: FuncText) -> (bool, Arc<metamodelica::List<ArcStr>>) {
    let mut res: (bool, Arc<metamodelica::List<ArcStr>>);
    res = (false, metamodelica::nil());
    match '__try0: {
        unwrap_break_err!(SimCodeUtil::resetFunctionIndex(), '__try0);
        SimCodeFunctionUtil::codegenResetTryThrowIndex();
        unwrap_break_err!(Tpl::tplCallWithFailErrorNoArg(Arc::new(func), Tpl::emptyTxt.clone()), '__try0);
        res = (true, SimCodeUtil::getFunctionIndex());
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    res
}

// TODO: use another switch ... later make it first class option like -target or so
fn callTargetTemplates(mut simCode: SimCode::SimCode, mut target: ArcStr) -> Result<()> {
    type Func = fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text>;

    type FuncText = fn(Tpl::Text) -> Result<Tpl::Text>;

    type BoolFunc = fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text>;

    fn runToStr(mut func: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static>) -> (bool, Arc<metamodelica::List<ArcStr>>) {
        pub type Func = fn() -> Result<ArcStr>;

        let mut res: (bool, Arc<metamodelica::List<ArcStr>>);
        res = (false, metamodelica::nil());
        match '__try0: {
            unwrap_break_err!(SimCodeUtil::resetFunctionIndex(), '__try0);
            SimCodeFunctionUtil::codegenResetTryThrowIndex();
            unwrap_break_err!(func(), '__try0);
            res = (true, SimCodeUtil::getFunctionIndex());
            Ok::<_, anyhow::Error>((res.clone(),))
        } {
            Ok((__try0_o0,)) => {
                res = __try0_o0;
            }
            Err(_) => {
                panic!("try/else: outputs not set in else branch");
            }
        }
        res
    }

    fn runCodegenFunc(mut func: PartialRunTpl) -> Result<(bool, Arc<metamodelica::List<ArcStr>>)> {
        let mut res: (bool, Arc<metamodelica::List<ArcStr>>);
        let mut b: bool = false;
        let ref __pa1 @ (ref __pa0, _) = func()?;
        b = __pa0.clone();
        res = __pa1.clone();
        if !(b.clone()) {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::dladdr(func)); __mm_s.push_str(&*literal!(" failed\n")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        if ErrorExt::getNumMessages() > 0 {
            ErrorExt::moveMessagesToParentThread();
        }
        Ok(res)
    }

    fn runToBoolean(mut func: Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static>) -> (bool, Arc<metamodelica::List<ArcStr>>) {
        type Func = fn() -> Result<bool>;

        let mut res: (bool, Arc<metamodelica::List<ArcStr>>);
        res = (func().unwrap(), metamodelica::nil());
        res
    }

    let mut func: Arc<dyn ::std::ops::Fn(Tpl::Text, SimCode::SimCode) -> Result<Tpl::Text> + 'static>;
    let mut txt: Tpl::Text;
    let mut generatedObjects: Arc<AvlSetString::Tree> = Arc::new(openmodelica_util::AvlSetString::Tree::EMPTY);
    crate::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = Some(simCode.clone()));
    let _ = (::match_deref::match_deref! { match &(target.clone()) {
        Deref @ "Cpp" => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
            let mut i: i32 = 0;
            callTargetTemplatesCPP(simCode.clone())?;
            for mut r#str in &*list![(literal!("CalcHelperMain.o\n")).clone(), (literal!(".so\n")).clone()] {
                let mut r#str = r#str.clone();
                generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OMCpp")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
            }
            ()
        },
        Deref @ "C" => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut guid: ArcStr = arcstr::literal!("");
            let mut codegenFuncs: Arc<metamodelica::List<PartialRunTpl>> = metamodelica::nil();
            let mut numThreads: i32 = 0;
            let mut n: i32 = 0;
            let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut tmp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut matches: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut i: i32 = 0;
            guid = (System::getUUIDStr()).clone();
            System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
            codegenFuncs = metamodelica::nil();
            codegenFuncs = cons({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<bool> + 'static> = Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = (guid.clone()).clone(); move || Ok(SerializeInitXML::simulationInitFileReturnBool(__pe_b0.clone(), __pe_b1.clone())) }); move || Ok(runToBoolean(__pe_b0.clone())) }, codegenFuncs.clone());
            codegenFuncs = cons({ let __pe_b0 = { let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenC::translateModel(__pe_a0, __pe_b1.clone()) }; move || Ok(runTpl(__pe_b0.clone())) }, codegenFuncs.clone());
            for mut f in &*list![(CodegenC::simulationFile_exo, literal!("_01exo.c")), (CodegenC::simulationFile_nls, literal!("_02nls.c")), (CodegenC::simulationFile_lsy, literal!("_03lsy.c")), (CodegenC::simulationFile_set, literal!("_04set.c")), (CodegenC::simulationFile_evt, literal!("_05evt.c")), (CodegenC::simulationFile_inz, literal!("_06inz.c")), (CodegenC::simulationFile_dly, literal!("_07dly.c")), (CodegenC::simulationFile_bnd, literal!("_08bnd.c")), (CodegenC::simulationFile_alg, literal!("_09alg.c")), (CodegenC::simulationFile_asr, literal!("_10asr.c")), (CodegenC::simulationFile_jac, literal!("_12jac.c")), (CodegenC::simulationFile_jac_header, literal!("_12jac.h")), (CodegenC::simulationFile_opt, literal!("_13opt.c")), (CodegenC::simulationFile_opt_header, literal!("_13opt.h")), (CodegenC::simulationFile_lnz, literal!("_14lnz.c")), (CodegenC::simulationFile_syn, literal!("_15syn.c")), (CodegenC::simulationFile_dae, literal!("_16dae.c")), (CodegenC::simulationFile_dae_header, literal!("_16dae.h")), (CodegenC::simulationFile_inl, literal!("_17inl.c")), (CodegenC::simulationFile_spd, literal!("_18spd.c")), (CodegenC::simulationHeaderFile, literal!("_model.h"))] {
                let mut f = f.clone();
                (func, r#str) = f.clone();
                codegenFuncs = cons({ let __pe_b0 = { let __pe_b1 = simCode.clone(); move |__pe_a0| func(__pe_a0, __pe_b1.clone()) }; let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }, codegenFuncs.clone());
                (n, matches) = System::regex((r#str.clone()).clone(), (literal!("\\(.*\\)[.]c$")).clone(), 2, false, false);
                if n.clone() == 2 {
                    let __pa0 = ::match_deref::match_deref! { match &(matches.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".o\n")); ArcStr::from(__mm_s) }).clone())?;
                }
            }
            for mut r#str in &*list![(literal!("_11mix.o\n")).clone(), (literal!("_functions.o\n")).clone(), (literal!("_info.json\n")).clone(), (literal!("_init.xml\n")).clone()] {
                let mut r#str = r#str.clone();
                generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
            }
            codegenFuncs = cons({ let __pe_b0 = { let __pe_b1 = simCode.clone(); let __pe_b2 = (simCode.fileNamePrefix.clone()).clone(); move |__pe_a0| CodegenC::simulationFile_mixAndHeader(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }; move || Ok(runTpl(__pe_b0.clone())) }, codegenFuncs.clone());
            codegenFuncs = cons({ let __pe_b0 = { let __pe_b1 = simCode.clone(); let __pe_b2 = (guid.clone()).clone(); let __pe_b3 = (literal!("")).clone(); move |__pe_a0| CodegenC::simulationFile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }; let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".c")); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }, codegenFuncs.clone());
            codegenFuncs = cons({ let __pe_b0 = { let __pe_b1 = (simCode.fileNamePrefix.clone()).clone(); let __pe_b2 = simCode.modelInfo.functions.clone(); let __pe_b3 = simCode.generic_loop_calls.clone(); move |__pe_a0| CodegenC::simulationFunctionsFile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }; let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_functions.c")); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }, codegenFuncs.clone());
            codegenFuncs = cons({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = Arc::new({ let __pe_b0 = simCode.clone(); move || SerializeSparsityPattern::serialize(__pe_b0.clone()) }); move || Ok(runToStr(__pe_b0.clone())) }, codegenFuncs.clone());
            codegenFuncs = cons({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?; move || SerializeModelInfo::serialize(__pe_b0.clone(), __pe_b1.clone()) }); move || Ok(runToStr(__pe_b0.clone())) }, codegenFuncs.clone());
            if Flags::getConfigBool(Flags::PARMODAUTO.clone())? {
                codegenFuncs = cons({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = Arc::new({ let __pe_b0 = simCode.clone(); let __pe_b1 = Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?; move || SerializeTaskSystemInfo::serializeParMod(__pe_b0.clone(), __pe_b1.clone()) }); move || Ok(runToStr(__pe_b0.clone())) }, codegenFuncs.clone());
                generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_ode.json\n")); ArcStr::from(__mm_s) }).clone())?;
            }
            if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
                codegenFuncs = cons({ let __pe_b0: Arc<dyn ::std::ops::Fn() -> Result<ArcStr> + 'static> = Arc::new({ let __pe_b0 = simCode.clone(); move || SimCodeUtil::generateRunnerBatScript(__pe_b0.clone()) }); move || Ok(runToStr(__pe_b0.clone())) }, codegenFuncs.clone());
            }
            numThreads = std::cmp::max(1, if (Testsuite::isRunning()?) {std::cmp::min(2, System::numProcessors())} else {Config::noProc()?});
            if !(Flags::isSet(Flags::PARALLEL_CODEGEN.clone())?) || numThreads.clone() == 1 {
                res = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut codegen_func in (codegenFuncs.clone()).into_iter().cloned() {
            let __x = codegen_func()?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            } else {
                res = System::launchParallelTasks(numThreads.clone(), codegenFuncs.clone(), Arc::new(runCodegenFunc))?;
            }
            strs = metamodelica::nil();
            for mut tpl in &*res.clone() {
                let mut tpl = tpl.clone();
                let __pa2 = ::match_deref::match_deref! { match &(tpl.clone()) {
                    (true, __pa2) => __pa2.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                tmp = __pa2.clone();
                strs = List::append_reverse(tmp.clone(), strs.clone());
            }
            strs = strs.clone().reverse();
            for mut r#str in &*strs.clone() {
                let mut r#str = r#str.clone();
                (n, matches) = System::regex((r#str.clone()).clone(), (literal!("\\(.*\\)[.]c$")).clone(), 2, false, false);
                if n.clone() == 2 {
                    let __pa3 = ::match_deref::match_deref! { match &(matches.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: __pa3, tail: _ } } => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa3.clone();
                    generatedObjects = AvlSetString::add(generatedObjects.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".o\n")); ArcStr::from(__mm_s) }).clone())?;
                }
            }
            Tpl::closeFile(Tpl::tplCallWithFailError3(Arc::new(CodegenC::simulationMakefile), (Config::simulationCodeTarget()?).clone(), simCode.clone(), strs.clone(), Tpl::redirectToFile(Tpl::emptyTxt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!(".makefile")); ArcStr::from(__mm_s) }).clone())?)?);
            ()
        },
        Deref @ "ExperimentalEmbeddedC" => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut codegenFuncs: Arc<metamodelica::List<PartialRunTpl>> = metamodelica::nil();
            let mut numThreads: i32 = 0;
            let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut tmp: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut i: i32 = 0;
            let _ = System::getUUIDStr();
            System::realtimeTick(ClockIndexes::RT_PROFILER0.clone())?;
            codegenFuncs = metamodelica::nil();
            for mut f in &*list![(CodegenEmbeddedC::mainFile, literal!("_main.c"))] {
                let mut f = f.clone();
                (func, r#str) = f.clone();
                codegenFuncs = cons({ let __pe_b0 = { let __pe_b1 = simCode.clone(); move |__pe_a0| func(__pe_a0, __pe_b1.clone()) }; let __pe_b1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone(); move || Ok(runTplWriteFile(__pe_b0.clone(), __pe_b1.clone())) }, codegenFuncs.clone());
            }
            numThreads = std::cmp::max(1, if (Testsuite::isRunning()?) {std::cmp::min(2, System::numProcessors())} else {Config::noProc()?});
            if !(Flags::isSet(Flags::PARALLEL_CODEGEN.clone())?) || numThreads.clone() == 1 {
                res = {
        let mut __acc: Arc<metamodelica::List<Tpl::Text>> = metamodelica::nil();
        for mut func in (codegenFuncs.clone()).into_iter().cloned() {
            let __x = func()?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            } else {
                res = System::launchParallelTasks(numThreads.clone(), codegenFuncs.clone(), Arc::new(runCodegenFunc))?;
            }
            strs = metamodelica::nil();
            for mut tpl in &*res.clone() {
                let mut tpl = tpl.clone();
                let __pa0 = ::match_deref::match_deref! { match &(tpl.clone()) {
                    (true, __pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                tmp = __pa0.clone();
                strs = List::append_reverse(tmp.clone(), strs.clone());
            }
            strs = strs.clone().reverse();
            ()
        },
        Deref @ "JavaScript" => {
            let mut guid: ArcStr = arcstr::literal!("");
            let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
            let mut i: i32 = 0;
            guid = (System::getUUIDStr()).clone();
            Tpl::tplNoret(Arc::new(CodegenC::translateModel), simCode.clone())?;
            SerializeInitXML::simulationInitFile(simCode.clone(), (guid.clone()).clone())?;
            System::covertTextFileToCLiteral(({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_init.xml")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_init.c")); ArcStr::from(__mm_s) }).clone(), (Config::simulationCodeTarget()?).clone());
            SerializeSparsityPattern::serialize(simCode.clone())?;
            SerializeModelInfo::serialize(simCode.clone(), Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
            Tpl::tplNoret(Arc::new(CodegenJS::markdownFile), simCode.clone())?;
            ()
        },
        Deref @ "XML" => {
            let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
            let mut i: i32 = 0;
            Tpl::tplNoret(Arc::new(CodegenXML::translateModel), simCode.clone())?;
            ()
        },
        Deref @ "None" => {
            let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
            let mut i: i32 = 0;
            ()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut res: Arc<metamodelica::List<(bool, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
            let mut i: i32 = 0;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown template target: ")); __mm_s.push_str(&*target.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if Testsuite::isRunning()? {
        System::appendFile((Testsuite::getTempFilesFile()?).clone(), stringAppendList(AvlSetString::listKeys(generatedObjects.clone(), metamodelica::nil())))?;
    }
    crate::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = None);
    Ok(())
}

fn callTargetTemplatesCPP(mut iSimCode: SimCode::SimCode) -> Result<()> {
    if Flags::isSet(Flags::HPCOM.clone())? {
        Tpl::tplNoret(Arc::new(CodegenCppHpcom::translateModel), iSimCode.clone())?;
    } else {
        Tpl::tplNoret(Arc::new(CodegenCpp::translateModel), iSimCode.clone())?;
    }
    Ok(())
}

fn callTargetTemplatesOMSICpp(mut iSimCode: SimCode::SimCode, mut program: Absyn::Program) -> Result<()> {
    let mut fmuVersion: ArcStr = arcstr::literal!("");
    let mut fmuType: ArcStr = arcstr::literal!("");
    fmuVersion = (literal!("2.0")).clone();
    fmuType = (literal!("me")).clone();
    Tpl::tplNoret3(Arc::new(CodegenOMSICpp::translateModel), iSimCode.clone(), (fmuVersion.clone()).clone(), (fmuType.clone()).clone())?;
    callTargetTemplatesFMU(iSimCode.clone(), (literal!("C")).clone(), (fmuVersion.clone()).clone(), (fmuType.clone()).clone(), program.clone())?;
    Ok(())
}

fn callTargetTemplatesFMU(mut simCode: SimCode::SimCode, mut target: ArcStr, mut FMUVersion: ArcStr, mut FMUType: ArcStr, mut program: Absyn::Program) -> Result<()> {
    crate::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = Some(simCode.clone()));
    let _ = (::match_deref::match_deref! { match &((simCode.clone(), target.clone())) {
        (SimCode::SimCode { .. }, Deref @ "C") => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut newdir: ArcStr = arcstr::literal!("");
            let mut newpath: ArcStr = arcstr::literal!("");
            let mut resourcesDir: ArcStr = arcstr::literal!("");
            let mut dirname: ArcStr = arcstr::literal!("");
            let mut htmlFile: ArcStr = arcstr::literal!("");
            let mut fmutmp: ArcStr = arcstr::literal!("");
            let mut b: bool = false;
            let mut exportDocumentation: bool = false;
            let mut needSundials: bool = false;
            let mut fileNamePrefixHash: ArcStr = arcstr::literal!("");
            let mut install_include_omc_dir: ArcStr = arcstr::literal!("");
            let mut install_include_omc_c_dir: ArcStr = arcstr::literal!("");
            let mut install_share_buildproject_dir: ArcStr = arcstr::literal!("");
            let mut install_fmu_sources_dir: ArcStr = arcstr::literal!("");
            let mut fmu_tmp_sources_dir: ArcStr = arcstr::literal!("");
            let mut cmakelistsStr: ArcStr = arcstr::literal!("");
            let mut needCvode: ArcStr = arcstr::literal!("");
            let mut cvodeDirectory: ArcStr = arcstr::literal!("");
            let mut modelDefinesHeaderStr: ArcStr = arcstr::literal!("");
            let mut model_desc_src_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut fmi2HeaderFiles: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut modelica_standard_table_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dgesv_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut cminpack_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_c_sundials_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_linear_solver_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_non_linear_solver_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut simrt_mixed_solver_sources: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut fmi_export_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut model_gen_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut model_all_gen_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut shared_source_files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut varInfo: SimCode::VarInfo;
            fileNamePrefixHash = (Util::hashFileNamePrefix((simCode.fileNamePrefix.clone()).clone())).clone();
            fmutmp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileNamePrefixHash.clone()); __mm_s.push_str(&*literal!(".fmutmp")); ArcStr::from(__mm_s) }).clone();
            if System::directoryExists((fmutmp.clone()).clone()) {
                if !(System::removeDirectory((fmutmp.clone()).clone())) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to remove directory: ")); __mm_s.push_str(&*fmutmp.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    bail!("fail");
                }
            }
            Util::createDirectoryTree(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/include/")); ArcStr::from(__mm_s) }).clone())?;
            resourcesDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/resources/")); ArcStr::from(__mm_s) }).clone();
            Util::createDirectoryTree((resourcesDir.clone()).clone())?;
            for mut path in &*simCode.modelInfo.resourcePaths.clone() {
                let mut path = path.clone();
                dirname = (System::dirname((path.clone()).clone())).clone();
                if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
                    dirname = (System::stringReplace((dirname.clone()).clone(), (literal!(":")).clone(), (literal!("")).clone())?).clone();
                }
                newdir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*dirname.clone()); ArcStr::from(__mm_s) }).clone();
                newpath = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*path.clone()); ArcStr::from(__mm_s) }).clone();
                if System::regularFileExists((newpath.clone()).clone()) || System::directoryExists((newpath.clone()).clone()) {
                    continue;
                }
                Util::createDirectoryTree((newdir.clone()).clone())?;
                if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cp -rf \"")); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*newdir.clone()); __mm_s.push_str(&*literal!("/\"")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy path ")); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*dirname.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                }
            }
            let _ = (match simCode.fmiSimulationFlags.clone() {
        Some(SimCode::FmiSimulationFlags::FMI_SIMULATION_FLAGS_FILE { path: mut pathToFlagsJson }) => {
            needSundials = true;
            if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cp -rf \"")); __mm_s.push_str(&*pathToFlagsJson.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_flags.json\"")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy ")); __mm_s.push_str(&*pathToFlagsJson.clone()); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_flags.json")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            }
            ()
        },
        _ => {
            ()
        },
    });
            SerializeSparsityPattern::serialize(simCode.clone())?;
            for mut jac in &*simCode.jacobianMatrices.clone() {
                let mut jac = jac.clone();
                if !(jac.sparsity.clone().is_empty()) {
                    if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mv '")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Jac")); __mm_s.push_str(&*jac.matrixName.clone()); __mm_s.push_str(&*literal!(".bin")); __mm_s.push_str(&*literal!("' '")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move ")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_Jac")); __mm_s.push_str(&*jac.matrixName.clone()); __mm_s.push_str(&*literal!(".bin file")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    }
                }
            }
            SerializeModelInfo::serialize(simCode.clone(), Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); ArcStr::from(__mm_s) }).clone();
            if FMUVersion.clone() == literal!("1.0") {
                b = System::covertTextFileToCLiteral(({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("_info.c")); ArcStr::from(__mm_s) }).clone(), (Flags::getConfigString(Flags::TARGET.clone())?).clone());
                if !(b.clone()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("System.covertTextFileToCLiteral failed. Could not write ")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("_info.c\n")); ArcStr::from(__mm_s) }).clone()])?;
                    bail!("fail");
                }
            } else {
                if Flags::getConfigEnum(Flags::FMI_FILTER.clone())? != Flags::FMI_BLACKBOX.clone() && Flags::getConfigEnum(Flags::FMI_FILTER.clone())? != Flags::FMI_PROTECTED.clone() {
                    if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mv '")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json")); __mm_s.push_str(&*literal!("' '")); __mm_s.push_str(&*resourcesDir.clone()); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move ")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_info.json file")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    }
                }
            }
            (htmlFile, exportDocumentation) = exportHTMLDocumentation(program.clone(), simCode.clone(), (FMUVersion.clone()).clone())?;
            if exportDocumentation.clone() {
                Util::createDirectoryTree(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/documentation/")); ArcStr::from(__mm_s) }).clone())?;
                if 0 != System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("mv '")); __mm_s.push_str(&*htmlFile.clone()); __mm_s.push_str(&*literal!("' '")); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/documentation/")); __mm_s.push_str(&*literal!("'")); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone()) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to move documentation file ")); __mm_s.push_str(&*htmlFile.clone()); __mm_s.push_str(&*literal!("")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                }
            }
            SimCodeUtil::resetFunctionIndex()?;
            varInfo = simCode.modelInfo.varInfo.clone();
            install_include_omc_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/")); ArcStr::from(__mm_s) }).clone();
            install_include_omc_c_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*install_include_omc_dir.clone()); __mm_s.push_str(&*literal!("c/")); ArcStr::from(__mm_s) }).clone();
            install_share_buildproject_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/share/omc/runtime/c/fmi/buildproject/")); ArcStr::from(__mm_s) }).clone();
            install_fmu_sources_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*arcstr::literal!(RuntimeSources::fmu_sources_dir)); ArcStr::from(__mm_s) }).clone();
            fmu_tmp_sources_dir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/")); ArcStr::from(__mm_s) }).clone();
            copyFiles(RuntimeSources::simrt_c_headers.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::simrt_c_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::dgesv_headers.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::dgesv_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            dgesv_sources = RuntimeSources::dgesv_sources.clone();
            copyFiles(RuntimeSources::cminpack_headers.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::cminpack_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            cminpack_sources = RuntimeSources::cminpack_sources.clone();
            if SimCodeUtil::cvodeFmiFlagIsSet(simCode.fmiSimulationFlags.clone())? {
                copyFiles(RuntimeSources::sundials_headers.clone(), (install_include_omc_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                copyFiles(RuntimeSources::simrt_c_sundials_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                simrt_c_sundials_sources = RuntimeSources::simrt_c_sundials_sources.clone();
            } else {
                simrt_c_sundials_sources = metamodelica::nil();
            }
            simrt_linear_solver_sources = if (varInfo.numLinearSystems.clone() > 0) {RuntimeSources::simrt_linear_solver_sources.clone()} else {metamodelica::nil()};
            copyFiles(simrt_linear_solver_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            simrt_non_linear_solver_sources = if (varInfo.numNonLinearSystems.clone() > 0) {RuntimeSources::simrt_non_linear_solver_sources.clone()} else {metamodelica::nil()};
            copyFiles(simrt_non_linear_solver_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            simrt_mixed_solver_sources = if (varInfo.numMixedSystems.clone() > 0) {RuntimeSources::simrt_mixed_solver_sources.clone()} else {metamodelica::nil()};
            copyFiles(simrt_mixed_solver_sources.clone(), (install_fmu_sources_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            if FMUVersion.clone() == literal!("1.0") {
                copyFiles(RuntimeSources::fmi1Files.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                fmi_export_files = RuntimeSources::fmi1Files.clone();
            } else {
                copyFiles(RuntimeSources::fmi2_sources.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                copyFiles(RuntimeSources::fmi2_headers.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
                fmi_export_files = RuntimeSources::fmi2_sources.clone();
            }
            fmi2HeaderFiles = list![(literal!("fmi/fmi2Functions.h")).clone(), (literal!("fmi/fmi2FunctionTypes.h")).clone(), (literal!("fmi/fmi2TypesPlatform.h")).clone(), (literal!("fmi/fmiModelFunctions.h")).clone(), (literal!("fmi/fmiModelTypes.h")).clone()];
            copyFiles(fmi2HeaderFiles.clone(), (install_include_omc_c_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::modelica_external_c_sources.clone(), (install_include_omc_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            copyFiles(RuntimeSources::modelica_external_c_headers.clone(), (install_include_omc_dir.clone()).clone(), (fmu_tmp_sources_dir.clone()).clone())?;
            modelica_standard_table_sources = RuntimeSources::modelica_external_c_sources.clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/isfmi")); __mm_s.push_str(&*if (FMUVersion.clone() == literal!("1.0")) {literal!("1")} else {literal!("2")}); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone())?;
            model_gen_files = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (RuntimeSources::defaultFileSuffixes.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            shared_source_files = List::flatten(list![fmi_export_files.clone(), RuntimeSources::simrt_c_sources.clone(), simrt_linear_solver_sources.clone(), simrt_non_linear_solver_sources.clone(), simrt_mixed_solver_sources.clone()]);
            if !(Flags::getConfigBool(Flags::FMI_SOURCES.clone())?) || Flags::getConfigEnum(Flags::FMI_FILTER.clone())? == Flags::FMI_BLACKBOX.clone() {
                model_desc_src_files = metamodelica::nil();
            } else {
                model_desc_src_files = List::flatten(list![List::sort(model_gen_files.clone(), Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)))?, List::sort(shared_source_files.clone(), Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)))?, List::sort(dgesv_sources.clone(), Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)))?, List::sort(cminpack_sources.clone(), Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)))?, List::sort(simrt_c_sundials_sources.clone(), Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)))?, List::sort(modelica_standard_table_sources.clone(), Arc::new(fnptr!(Util::strcmpNoCaseBool, ArcStr, ArcStr)))?]);
            }
            Tpl::tplNoret(Arc::new({ let __pe_b2 = (FMUVersion.clone()).clone(); let __pe_b3 = (FMUType.clone()).clone(); let __pe_b4 = model_desc_src_files.clone(); move |__pe_a0, __pe_a1| CodegenFMU::translateModel(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }), simCode.clone())?;
            model_all_gen_files = listAppend(model_gen_files.clone(), SimCodeUtil::getFunctionIndex());
            System::copyFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*install_share_buildproject_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt.in")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) }).clone());
            cmakelistsStr = (System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) }).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_NAME_HASH_IN@")).clone(), (fileNamePrefixHash.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_NAME_IN@")).clone(), (simCode.fileNamePrefix.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_TARGET_NAME@")).clone(), (simCode.fmuTargetName.clone()).clone())?).clone();
            if Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())? {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@CMAKE_BUILD_TYPE@")).clone(), (literal!("Debug")).clone())?).clone();
            } else {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@CMAKE_BUILD_TYPE@")).clone(), (literal!("Release")).clone())?).clone();
            }
            let _ = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::FMU_RUNTIME_DEPENDS.clone())?) {
        Deref @ "default" => {
            let mut cmakeVersion: SemanticVersion::Version;
            let mut minimumVersion: SemanticVersion::Version;
            cmakeVersion = SimCodeUtil::getCMakeVersion((arcstr::literal!(Autoconf::cmake)).clone())?;
            minimumVersion = SemanticVersion::Version::SEMVER { major: 3, minor: 21, patch: 0, prerelease: metamodelica::nil(), meta: metamodelica::nil() };
            if SemanticVersion::compare(minimumVersion.clone(), cmakeVersion.clone(), true, false)? <= 0 {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"modelica\"")).clone())?).clone();
            } else {
                cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"none\"")).clone())?).clone();
            }
            ()
        },
        Deref @ "none" => {
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"none\"")).clone())?).clone();
            ()
        },
        Deref @ "modelica" => {
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"modelica\"")).clone())?).clone();
            ()
        },
        Deref @ "all" => {
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@RUNTIME_DEPENDENCIES_LEVEL@")).clone(), (literal!("\"all\"")).clone())?).clone();
            ()
        },
        _ => {
            Error::addCompilerError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unsupported value ")); __mm_s.push_str(&*Flags::getConfigString(Flags::FMU_RUNTIME_DEPENDS.clone())?); __mm_s.push_str(&*literal!("for compiler flag 'fmuRuntimeDepends'.")); ArcStr::from(__mm_s) }).clone())?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMI_INTERFACE_HEADER_FILES_DIRECTORY@")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/include/omc/c/fmi")); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone())?).clone();
            (needCvode, cvodeDirectory) = SimCodeUtil::getCmakeSundialsLinkCode(simCode.fmiSimulationFlags.clone())?;
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@NEED_CVODE@")).clone(), (needCvode.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@CVODE_DIRECTORY@")).clone(), (cvodeDirectory.clone()).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_ADDITIONAL_LIBS@")).clone(), (SimCodeUtil::getCmakeLinkLibrariesCode(simCode.makefileParams.libs.clone())?).clone())?).clone();
            cmakelistsStr = (System::stringReplace((cmakelistsStr.clone()).clone(), (literal!("@FMU_ADDITIONAL_INCLUDES@")).clone(), (SimCodeUtil::make2CMakeInclude(simCode.makefileParams.includes.clone())).clone())?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("CMakeLists.txt")); ArcStr::from(__mm_s) }).clone(), (cmakelistsStr.clone()).clone())?;
            modelDefinesHeaderStr = (System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("fmi-export/fmu2_model_interface.c")); ArcStr::from(__mm_s) }).clone())?).clone();
            modelDefinesHeaderStr = (System::stringReplace((modelDefinesHeaderStr.clone()).clone(), (literal!("fmu2_dummy_model_defines.h")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("../")); __mm_s.push_str(&*simCode.fileNamePrefix.clone()); __mm_s.push_str(&*literal!("_FMU.h")); ArcStr::from(__mm_s) }).clone())?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmu_tmp_sources_dir.clone()); __mm_s.push_str(&*literal!("fmi-export/fmu2_model_interface.c")); ArcStr::from(__mm_s) }).clone(), (modelDefinesHeaderStr.clone()).clone())?;
            Tpl::closeFile(Tpl::tplCallWithFailErrorNoArg(Arc::new({ let __pe_b1 = (Config::simulationCodeTarget()?).clone(); let __pe_b2 = simCode.clone(); let __pe_b3 = (FMUVersion.clone()).clone(); let __pe_b4 = model_all_gen_files.clone(); let __pe_b5 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (shared_source_files.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }; let __pe_b6 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (dgesv_sources.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }; let __pe_b7 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (cminpack_sources.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }; let __pe_b8 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (simrt_c_sundials_sources.clone()).into_iter().cloned() {
            let __x = System::stringReplace((f.clone()).clone(), (literal!(".c")).clone(), (literal!(".o")).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }; move |__pe_a0| CodegenFMU::fmuMakefile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone(), __pe_b7.clone(), __pe_b8.clone()) }), Tpl::redirectToFile(Tpl::emptyTxt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/Makefile.in")); ArcStr::from(__mm_s) }).clone())?)?);
            Tpl::closeFile(Tpl::tplCallWithFailError(Arc::new(CodegenFMU::settingsfile), simCode.clone(), Tpl::redirectToFile(Tpl::emptyTxt.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fmutmp.clone()); __mm_s.push_str(&*literal!("/sources/omc_simulation_settings.h")); ArcStr::from(__mm_s) }).clone())?)?);
            if Config::simCodeTarget()? == literal!("omsicpp") {
                runTpl({ let __pe_b1 = simCode.clone(); let __pe_b2 = (FMUVersion.clone()).clone(); let __pe_b3 = (FMUType.clone()).clone(); move |__pe_a0| CodegenOMSICpp::translateModel(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) });
            }
            ()
        },
        (_, Deref @ "omsic") => {
            let mut guid: ArcStr = arcstr::literal!("");
            let mut needSundials: bool = false;
            let mut fileprefix: ArcStr = arcstr::literal!("");
            guid = (System::getUUIDStr()).clone();
            fileprefix = (simCode.fileNamePrefix.clone()).clone();
            if System::directoryExists((simCode.fullPathPrefix.clone()).clone()) {
                if !(System::removeDirectory((simCode.fullPathPrefix.clone()).clone())) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to remove directory: ")); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    bail!("fail");
                }
            }
            if !(System::createDirectory((simCode.fullPathPrefix.clone()).clone())) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to create tmp folder ")); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                System::fflush();
                bail!("fail");
            }
            SerializeInitXML::simulationInitFileReturnBool(simCode.clone(), (guid.clone()).clone());
            SerializeSparsityPattern::serialize(simCode.clone())?;
            SerializeModelInfo::serialize(simCode.clone(), Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?)?;
            runTpl({ let __pe_b1 = simCode.clone(); let __pe_b2 = (guid.clone()).clone(); let __pe_b3 = (FMUVersion.clone()).clone(); let __pe_b4 = (FMUType.clone()).clone(); let __pe_b5 = metamodelica::nil(); let __pe_b6 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*literal!("modelDescription.xml")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| CodegenOMSI_common::generateFMUModelDescriptionFile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone(), __pe_b6.clone()) });
            runTplWriteFile({ let __pe_b1 = simCode.clone(); let __pe_b2 = (Config::simulationCodeTarget()?).clone(); let __pe_b3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_FMU.makefile")); ArcStr::from(__mm_s) }).clone(); move |__pe_a0| CodegenOMSIC::createMakefile(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_FMU.makefile")); ArcStr::from(__mm_s) }).clone());
            runTplWriteFile({ let __pe_b1 = simCode.clone(); move |__pe_a0| CodegenOMSIC::generateOMSIC(__pe_a0, __pe_b1.clone()) }, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*simCode.fullPathPrefix.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_omsic.c")); ArcStr::from(__mm_s) }).clone());
            runTpl({ let __pe_b1 = simCode.clone(); let __pe_b2 = (fileprefix.clone()).clone(); move |__pe_a0| CodegenOMSI_common::generateEquationsCode(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) });
            ()
        },
        (_, Deref @ "Cpp") => {
            let mut needSundials: bool = false;
            if Flags::isSet(Flags::HPCOM.clone())? {
                Tpl::tplNoret3(Arc::new(CodegenFMUCppHpcom::translateModel), simCode.clone(), (FMUVersion.clone()).clone(), (FMUType.clone()).clone())?;
            } else {
                Tpl::tplNoret(Arc::new({ let __pe_b2 = (FMUVersion.clone()).clone(); let __pe_b3 = (FMUType.clone()).clone(); let __pe_b4 = metamodelica::nil(); move |__pe_a0, __pe_a1| CodegenFMUCpp::translateModel(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone()) }), simCode.clone())?;
            }
            ()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut needSundials: bool = false;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown FMU template target: ")); __mm_s.push_str(&*target.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    crate::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = None);
    Ok(())
}

fn exportHTMLDocumentation(mut program: Absyn::Program, mut simCode: SimCode::SimCode, mut FMUVersion: ArcStr) -> Result<(ArcStr, bool)> {
    let mut fileName: ArcStr = arcstr::literal!("");
    let mut export: bool = true;
    let mut file: File::File;
    let mut info: ArcStr = arcstr::literal!("");
    let mut revisions: ArcStr = arcstr::literal!("");
    let mut infoHeader: ArcStr = arcstr::literal!("");
    (info, revisions, infoHeader) = Interactive::getNamedAnnotationExp(simCode.modelInfo.name.clone(), program.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("Documentation")).clone() }), Some((literal!(""), literal!(""), literal!(""))), Arc::new(Interactive::getDocumentationAnnotationString))?;
    if stringEmpty((info.clone()).clone()) && stringEmpty((revisions.clone()).clone()) && stringEmpty((infoHeader.clone()).clone()) {
        export = false;
    }
    if FMUVersion.clone() == literal!("1.0") {
        fileName = (literal!("_main.html")).clone();
    } else {
        fileName = (literal!("index.html")).clone();
    }
    file = File::File(File::noReference())?;
    File::open(file.clone(), (fileName.clone()).clone(), File::Mode::Write.clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*infoHeader.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<h1>")); __mm_s.push_str(&*AbsynUtil::pathString(simCode.modelInfo.name.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("</h1>\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<p> <i>")); __mm_s.push_str(&*simCode.modelInfo.description.clone()); __mm_s.push_str(&*literal!("</i> </p>\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<h4> <u> Information </u> </h4>")); __mm_s.push_str(&*info.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    File::write(file.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("<h4> <u> Revisions </u> </h4>")); __mm_s.push_str(&*revisions.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok((fileName, export))
}

fn callTargetTemplatesXML(mut simCode: SimCode::SimCode, mut target: ArcStr) -> Result<()> {
    Tpl::tplNoret(Arc::new(CodegenXML::translateModel), simCode.clone())?;
    Ok(())
}

fn translateModelCallBackendOB(mut kind: TranslateModelKind, mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut inDae: DAE::DAElist, mut className: Arc<Absyn::Path>, mut inFileNamePrefix: ArcStr, mut inSimSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>) -> Result<(FCore::Cache, Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>)> {
    let mut cache: FCore::Cache = cache;
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outFileDir: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut generateFunctions: bool = false;
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    FlagsUtil::setConfigBool(Flags::BUILDING_MODEL.clone(), true)?;
    (outLibs, outFileDir) = (match inEnv.clone() {
        mut graph => {
            let mut file_dir: ArcStr = arcstr::literal!("");
            let mut description: ArcStr = arcstr::literal!("");
            let mut fmuType: ArcStr = arcstr::literal!("");
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dae: DAE::DAElist;
            let mut dlow: Arc<BackendDAE::BackendDAE>;
            let mut initDAE: Arc<BackendDAE::BackendDAE>;
            let mut initDAE_lambda0: Option<Arc<BackendDAE::BackendDAE>> = None;
            let mut inlineData: Option<BackendDAE::InlineData> = None;
            let mut removedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut strPreOptModules: Option<Arc<metamodelica::List<ArcStr>>> = None;
            let mut isFMI2: bool = false;
            let mut fmiDer: Arc<metamodelica::List<(Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32))>> = metamodelica::nil();
            let mut funcs; // TODO: local with unresolved type
            System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            dae = DAEUtil::transformationsBeforeBackend(cache.clone(), graph.clone(), inDae.clone())?;
            ExecStat::execStat((literal!("Transformations before backend")).clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dae.clone(), (literal!("FrontEnd DAE after transformations")).clone())?;
                serializeNotify((dae.clone(), inDae.clone()), (literal!("FrontEnd DAE before+after transformations")).clone())?;
                ExecStat::execStat((literal!("Serialize DAE (2)")).clone())?;
            }
            GCExt::free(inDae.clone());
            generateFunctions = FlagsUtil::set(Flags::GEN.clone(), false)?;
            if !(Flags::isSet(Flags::BACKEND_KEEP_ENV_GRAPH.clone())?) {
                (cache, graph) = Builtin::initialGraph(cache.clone())?;
            }
            description = (DAEUtil::daeDescription(dae.clone())).clone();
            dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), graph.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (inFileNamePrefix.clone()).clone(), simSettingsOption: inSimSettingsOpt.clone() })?;
            GCExt::free(dae.clone());
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("BackendDAECreate.lower")).clone())?;
                ExecStat::execStat((literal!("Serialize dlow")).clone())?;
            }
            isFMI2 = (match kind.clone() {
        TranslateModelKind::FMU { kind: mut fmuType, .. } => FMI::isFMIVersion20((FMI::getFMIVersionString()).clone()),
        _ => false,
    });
            strPreOptModules = if (isFMI2.clone()) {Some(cons(literal!("introduceOutputAliases"), BackendDAEUtil::getPreOptModulesString()?))} else {None};
            if isFMI2.clone() && fmuType.clone() == literal!("cs") {
                strPreOptModules = Some(cons(literal!("introduceOutputRealDerivatives"), Util::getOption(strPreOptModules.clone())?));
            }
            (dlow, initDAE, initDAE_lambda0, inlineData, removedInitialEquationLst) = BackendDAEUtil::getSolvedSystem(dlow.clone(), (inFileNamePrefix.clone()).clone(), strPreOptModules.clone(), None, None, None)?;
            if isFMI2.clone() && !(Flags::isSet(Flags::FMI20_DEPENDENCIES.clone())?) {
                (fmiDer, funcs) = SymbolicJacobian::createFMIModelDerivatives(dlow.clone())?;
                dlow = BackendDAEUtil::setFunctionTree(dlow.clone(), funcs.clone())?;
            } else {
                fmiDer = metamodelica::nil();
            }
            timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("BackendDAE (simulation)")).clone())?;
                serializeNotify(initDAE.clone(), (literal!("BackendDAE (initialization)")).clone())?;
                serializeNotify(initDAE_lambda0.clone(), (literal!("BackendDAE (lambda0)")).clone())?;
                serializeNotify((dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), inlineData.clone(), removedInitialEquationLst.clone()), (literal!("BackendDAE (simulation+initialization+lambda0+inlineData+removedInitialEquationLst)")).clone())?;
                ExecStat::execStat((literal!("Serialize solved system")).clone())?;
            }
            (libs, file_dir, timeSimCode, timeTemplates) = (match kind.clone() {
        TranslateModelKind::NORMAL => {
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCode(dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), inlineData.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone(), args.clone(), fmiDer.clone())?;
            (libs.clone(), file_dir.clone(), timeSimCode.clone(), timeTemplates.clone())
        },
        TranslateModelKind::FMU { .. } => {
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeFMU(dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), fmiDer.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (FMI::getFMIVersionString()).clone(), (var_field!(kind.kind, TranslateModelKind::FMU).clone()).clone(), (inFileNamePrefix.clone()).clone(), (var_field!(kind.targetName, TranslateModelKind::FMU).clone()).clone(), inSimSettingsOpt.clone())?;
            (libs.clone(), file_dir.clone(), timeSimCode.clone(), timeTemplates.clone())
        },
        TranslateModelKind::XML => {
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeXML(dlow.clone(), initDAE.clone(), initDAE_lambda0.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone())?;
            (libs.clone(), file_dir.clone(), timeSimCode.clone(), timeTemplates.clone())
        },
        _ => {
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown translateModel kind: ")); __mm_s.push_str(&*anyString(kind.clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
    });
            (libs.clone(), file_dir.clone())
        },
    });
    if generateFunctions.clone() {
        FlagsUtil::set(Flags::GEN.clone(), true)?;
    }
    resultValues = list![(literal!("timeTemplates"), Arc::new(Values::Value::REAL { real: timeTemplates.clone() })), (literal!("timeSimCode"), Arc::new(Values::Value::REAL { real: timeSimCode.clone() })), (literal!("timeBackend"), Arc::new(Values::Value::REAL { real: timeBackend.clone() }))];
    Ok((cache, outLibs, outFileDir, resultValues))
}

pub fn translateModelCallBackendOBDAEMode(mut cache: FCore::Cache, mut inEnv: FCore::Graph, mut inDae: DAE::DAElist, mut className: Arc<Absyn::Path>, mut inFileNamePrefix: ArcStr, mut inSimSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>) -> Result<(FCore::Cache, Arc<metamodelica::List<ArcStr>>, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>>)> {
    let mut cache: FCore::Cache = cache;
    let mut outLibs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outFileDir: ArcStr = arcstr::literal!("");
    let mut resultValues: Arc<metamodelica::List<(ArcStr, Arc<Values::Value>)>> = metamodelica::nil();
    let mut generateFunctions: bool = false;
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeBackend: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (outLibs, outFileDir) = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut graph = __mc_input.clone() else { bail!("nomatch") };
            let mut file_dir: ArcStr = arcstr::literal!("");
            let mut description: ArcStr = arcstr::literal!("");
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut dae: DAE::DAElist;
            let mut dlow: Arc<BackendDAE::BackendDAE>;
            let mut initDAE: Arc<BackendDAE::BackendDAE>;
            let mut initDAE_lambda0_option: Option<Arc<BackendDAE::BackendDAE>> = None;
            let mut removedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut timeBackend: metamodelica::Real = timeBackend.clone();
            let mut timeTemplates: metamodelica::Real = timeTemplates.clone();
            let mut generateFunctions: bool = generateFunctions.clone();
            let mut cache: FCore::Cache = cache.clone();
            let mut timeSimCode: metamodelica::Real = timeSimCode.clone();
            System::realtimeTick(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            dae = DAEUtil::transformationsBeforeBackend(cache.clone(), graph.clone(), inDae.clone())?;
            ExecStat::execStat((literal!("Transformations before backend")).clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dae.clone(), (literal!("dae2")).clone())?;
                ExecStat::execStat((literal!("Serialize DAE (2)")).clone())?;
            }
            GCExt::free(inDae.clone());
            generateFunctions = FlagsUtil::set(Flags::GEN.clone(), false)?;
            if !(Flags::isSet(Flags::BACKEND_KEEP_ENV_GRAPH.clone())?) {
                (cache, graph) = Builtin::initialGraph(cache.clone())?;
            }
            description = (DAEUtil::daeDescription(dae.clone())).clone();
            dlow = BackendDAECreate::lower(dae.clone(), cache.clone(), graph.clone(), BackendDAE::ExtraInfo { description: (description.clone()).clone(), fileNamePrefix: (inFileNamePrefix.clone()).clone(), simSettingsOption: inSimSettingsOpt.clone() })?;
            GCExt::free(dae.clone());
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("dlow")).clone())?;
                ExecStat::execStat((literal!("Serialize dlow")).clone())?;
            }
            (dlow, initDAE, initDAE_lambda0_option, removedInitialEquationLst) = DAEMode::getEqSystemDAEmode(dlow.clone(), (inFileNamePrefix.clone()).clone(), None, None, None, None)?;
            ExecStat::execStat((literal!("Backend")).clone())?;
            timeBackend = System::realtimeTock(ClockIndexes::RT_CLOCK_BACKEND.clone())?;
            if Flags::isSet(Flags::SERIALIZED_SIZE.clone())? {
                serializeNotify(dlow.clone(), (literal!("simDAE")).clone())?;
                serializeNotify(initDAE.clone(), (literal!("initDAE")).clone())?;
                serializeNotify(removedInitialEquationLst.clone(), (literal!("removedInitialEquationLst")).clone())?;
                ExecStat::execStat((literal!("Serialize solved system")).clone())?;
            }
            (libs, file_dir, timeSimCode, timeTemplates) = generateModelCodeDAE(dlow.clone(), initDAE.clone(), initDAE_lambda0_option.clone(), removedInitialEquationLst.clone(), SymbolTable::getAbsyn(), className.clone(), (inFileNamePrefix.clone()).clone(), inSimSettingsOpt.clone(), args.clone())?;
            timeSimCode = System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone())?;
            timeTemplates = System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone())?;
            Ok((libs.clone(), file_dir.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut resstr: ArcStr = arcstr::literal!("");
            resstr = AbsynUtil::pathStringNoQual(className.clone(), (literal!(".")).clone(), true, false)?;
            resstr = stringAppendList(list![(literal!("SimCode DAEmode: The model ")).clone(), (resstr.clone()).clone(), (literal!(" could not be translated")).clone()]);
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(resstr.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    if generateFunctions.clone() {
        FlagsUtil::set(Flags::GEN.clone(), true)?;
    }
    resultValues = list![(literal!("timeTemplates"), Arc::new(Values::Value::REAL { real: timeTemplates.clone() })), (literal!("timeSimCode"), Arc::new(Values::Value::REAL { real: timeSimCode.clone() })), (literal!("timeBackend"), Arc::new(Values::Value::REAL { real: timeBackend.clone() }))];
    Ok((cache, outLibs, outFileDir, resultValues))
}

fn generateModelCodeDAE(mut inBackendDAE: Arc<BackendDAE::BackendDAE>, mut inInitDAE: Arc<BackendDAE::BackendDAE>, mut initDAE_lambda0_option: Option<Arc<BackendDAE::BackendDAE>>, mut inRemovedInitialEquationLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut p: Absyn::Program, mut className: Arc<Absyn::Path>, mut filenamePrefix: ArcStr, mut simSettingsOpt: Option<SimCode::SimulationSettings>, mut args: Arc<Absyn::FunctionArgs>) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr, metamodelica::Real, metamodelica::Real)> {
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileDir: ArcStr = arcstr::literal!("");
    let mut timeSimCode: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut timeTemplates: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let debug: bool = false;
    let mut includes: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    let mut simCode: SimCode::SimCode;
    let mut recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    let mut a_cref: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut literals: (i32, (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::Exp>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), Arc<metamodelica::List<Arc<DAE::Exp>>>);
    let mut lits: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
    let mut program: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut numCheckpoints: i32 = 0;
    let mut tempVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut emptyBDAE: Arc<BackendDAE::BackendDAE>;
    let mut initDAE_lambda0: Arc<BackendDAE::BackendDAE>;
    let mut modelInfo: SimCode::ModelInfo;
    let mut extObjInfo: SimCode::ExtObjInfo;
    let mut crefToSimVarHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, SimCodeVar::SimVar)>>), i32, (HashTableCrefSimVar::FuncHashCref, HashTableCrefSimVar::FuncCrefEqual, HashTableCrefSimVar::FuncCrefStr, HashTableCrefSimVar::FuncExpStr));
    let mut makefileParams: SimCodeFunction::MakefileParams;
    let mut spatialInfo: SimCode::SpatialDistributionInfo;
    let mut delayedExps: Arc<metamodelica::List<(i32, (Arc<DAE::Exp>, Arc<DAE::Exp>, Arc<DAE::Exp>))>> = metamodelica::nil();
    let mut maxDelayedExpIndex: i32 = 0;
    let mut uniqueEqIndex: i32 = 1;
    let mut nStates: i32 = 0;
    let mut numberofEqns: i32 = 0;
    let mut numberofLinearSys: i32 = 0;
    let mut numberofNonLinearSys: i32 = 0;
    let mut numberofMixedSys: i32 = 0;
    let mut numberOfJacobians: i32 = 0;
    let mut numberofFixedParameters: i32 = 0;
    let mut tmpB: bool = false;
    let mut varToArrayIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, (Arc<metamodelica::List<i32>>, metamodelica::Array<i32>))>>), i32, (HashTableCrIListArray::FuncHashCref, HashTableCrIListArray::FuncCrefEqual, HashTableCrIListArray::FuncCrefStr, HashTableCrIListArray::FuncExpStr));
    let mut varToIndexMapping: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<i32>>)>>), i32, (HashTableCrILst::FuncHashCref, HashTableCrILst::FuncCrefEqual, HashTableCrILst::FuncCrefStr, HashTableCrILst::FuncExpStr));
    let mut crefToClockIndexHT: (metamodelica::Array<Arc<metamodelica::List<(Arc<DAE::ComponentRef>, i32)>>>, (i32, i32, metamodelica::Array<Option<(Arc<DAE::ComponentRef>, i32)>>), i32, (HashTable::FuncHashCref, HashTable::FuncCrefEqual, HashTable::FuncCrefStr, HashTable::FuncExpStr));
    let mut discreteModelVars: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>> = metamodelica::nil();
    let mut zeroCrossingsSet: BackendDAE::ZeroCrossingSet;
    let mut sampleZCSet: BackendDAE::ZeroCrossingSet;
    let mut de_relations: DoubleEnded::MutableList<BackendDAE::ZeroCrossing>;
    let mut zeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut sampleZC: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut relations: Arc<metamodelica::List<BackendDAE::ZeroCrossing>> = metamodelica::nil();
    let mut daeVars: BackendDAE::Variables;
    let mut resVars: BackendDAE::Variables;
    let mut algStateVars: BackendDAE::Variables;
    let mut auxVars: BackendDAE::Variables;
    let mut varsLst: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqnsLst: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut daeEqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut localSharedAlgVars: BackendDAE::Variables;
    let mut daeModeSP: Option<Arc<SimCode::JacobianMatrix>> = None;
    let mut daeModeData: Option<SimCode::DaeModeData> = None;
    let mut daeModeConf: SimCode::DaeModeConfig = SimCode::DaeModeConfig::ALL_EQUATIONS;
    let mut matrixnames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut daeEquations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>>> = metamodelica::nil();
    let mut residualVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut algebraicStateVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut auxiliaryVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut daeModeJacobian: (Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32), Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>>, (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32));
    let mut daeModeJac: Option<(Arc<BackendDAE::BackendDAE>, ArcStr, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<BackendDAE::Var>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)> = None;
    let mut jacH: Option<Arc<BackendDAE::Jacobian>> = None;
    let mut daeModeSparsity: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32);
    let mut daeModeColoring: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::ComponentRef>>>>> = metamodelica::nil();
    let mut nonlinearPattern: (Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, Arc<metamodelica::List<(Arc<DAE::ComponentRef>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>)>>, (Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<Arc<DAE::ComponentRef>>>), i32);
    let mut symDAESparsPattern: Arc<SimCode::JacobianMatrix>;
    let mut symJacs: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut SymbolicJacs: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut SymbolicJacsNLS: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut SymbolicJacsTemp: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut SymbolicJacsStateSelect: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
    let mut initialEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut initialEquations_lambda0: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut removedInitialEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut jacobianSimvars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut seedVars: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
    let mut startValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut maxValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut minValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut nominalValueEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut parameterEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut jacobianEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    numCheckpoints = ErrorExt::getNumCheckpoints();
    if '__try0: {
        StackOverflow::clearStacktraceMessages();
        unwrap_break_err!(System::realtimeTick(ClockIndexes::RT_CLOCK_SIMCODE.clone()), '__try0);
        a_cref = unwrap_break_err!(AbsynUtil::pathToCref(className.clone()), '__try0);
        fileDir = (unwrap_break_err!(CevalScriptBackend::getFileDir(a_cref.clone(), p.clone()), '__try0)).clone();
        (libs, libPaths, includes, includeDirs, recordDecls, functions, literals) = unwrap_break_err!(SimCodeUtil::createFunctions(p.clone(), inBackendDAE.shared.functionTree.clone()), '__try0);
        extObjInfo = unwrap_break_err!(SimCodeUtil::createExtObjInfo(inBackendDAE.shared.clone()), '__try0);
        makefileParams = unwrap_break_err!(SimCodeFunctionUtil::createMakefileParams(includeDirs.clone(), libs.clone(), libPaths.clone(), false, false), '__try0);
        (delayedExps, maxDelayedExpIndex) = unwrap_break_err!(SimCodeUtil::extractDelayedExpressions(inBackendDAE.clone()), '__try0);
        spatialInfo = unwrap_break_err!(SimCodeUtil::extractSpatialDistributionInfo(inBackendDAE.clone()), '__try0);
        timeEvents = inBackendDAE.shared.eventInfo.timeEvents.clone();
        (zeroCrossings, relations, sampleZC) = (match inBackendDAE.shared.eventInfo.clone() {
        BackendDAE::EventInfo { samples: mut sampleZCSet, relations: mut de_relations, zeroCrossings: mut zeroCrossingsSet, .. } => (ZeroCrossings::toList(zeroCrossingsSet.clone()), DoubleEnded::toListNoCopyNoClear(de_relations.clone()), ZeroCrossings::toList(sampleZCSet.clone())),
        _ => bail!("match: no arm matched"),
    });
        (initialEquations, uniqueEqIndex, tempVars) = unwrap_break_err!(SimCodeUtil::createInitialEquations(inInitDAE.clone(), uniqueEqIndex.clone(), tempVars.clone()), '__try0);
        if isSome(initDAE_lambda0_option.clone()) {
            let __pa1 = ::match_deref::match_deref! { match &(initDAE_lambda0_option.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            initDAE_lambda0 = __pa1.clone();
            (initialEquations_lambda0, uniqueEqIndex, tempVars) = unwrap_break_err!(SimCodeUtil::createInitialEquations_lambda0(initDAE_lambda0.clone(), uniqueEqIndex.clone(), tempVars.clone()), '__try0);
        } else {
            initialEquations_lambda0 = metamodelica::nil();
        }
        let (__pa2, (__pa3, _), __pa4) = unwrap_break_err!(SimCodeUtil::createNonlinearResidualEquations(inRemovedInitialEquationLst.clone(), (uniqueEqIndex.clone(), 0), tempVars.clone(), inBackendDAE.shared.functionTree.clone()), '__try0);
        removedInitialEquations = __pa2.clone();
        uniqueEqIndex = __pa3.clone();
        tempVars = __pa4.clone();
        unwrap_break_err!(ExecStat::execStat((literal!("simCode: created initialization part")).clone()), '__try0);
        (uniqueEqIndex, startValueEquations, _) = unwrap_break_err!(BackendDAEUtil::foldEqSystem(inInitDAE.clone(), Arc::new(SimCodeUtil::createStartValueEquations), (uniqueEqIndex.clone(), metamodelica::nil(), inBackendDAE.shared.globalKnownVars.clone())), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createStartValueEquations")).clone()), '__try0);
        }
        (uniqueEqIndex, nominalValueEquations) = unwrap_break_err!(SimCodeUtil::createValueEquationsShared(inBackendDAE.shared.clone(), Arc::new(SimCodeUtil::createInitialAssignmentsFromNominal), (uniqueEqIndex.clone(), nominalValueEquations.clone())), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createNominalValueEquationsShared")).clone()), '__try0);
        }
        (uniqueEqIndex, nominalValueEquations) = unwrap_break_err!(BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), Arc::new(SimCodeUtil::createNominalValueEquations), (uniqueEqIndex.clone(), nominalValueEquations.clone())), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createNominalValueEquations")).clone()), '__try0);
        }
        (uniqueEqIndex, minValueEquations) = unwrap_break_err!(SimCodeUtil::createValueEquationsShared(inBackendDAE.shared.clone(), Arc::new(SimCodeUtil::createInitialAssignmentsFromMin), (uniqueEqIndex.clone(), minValueEquations.clone())), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createMinValueEquationsShared")).clone()), '__try0);
        }
        (uniqueEqIndex, minValueEquations) = unwrap_break_err!(BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), Arc::new(SimCodeUtil::createMinValueEquations), (uniqueEqIndex.clone(), minValueEquations.clone())), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createMinValueEquations")).clone()), '__try0);
        }
        (uniqueEqIndex, maxValueEquations) = unwrap_break_err!(SimCodeUtil::createValueEquationsShared(inBackendDAE.shared.clone(), Arc::new(SimCodeUtil::createInitialAssignmentsFromMax), (uniqueEqIndex.clone(), maxValueEquations.clone())), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createMaxValueEquationsShared")).clone()), '__try0);
        }
        (uniqueEqIndex, maxValueEquations) = unwrap_break_err!(BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), Arc::new(SimCodeUtil::createMaxValueEquations), (uniqueEqIndex.clone(), maxValueEquations.clone())), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createMaxValueEquations")).clone()), '__try0);
        }
        (uniqueEqIndex, parameterEquations, _) = unwrap_break_err!(SimCodeUtil::createParameterEquations(uniqueEqIndex.clone(), parameterEquations.clone(), inBackendDAE.shared.globalKnownVars.clone()), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: createParameterEquations")).clone()), '__try0);
        }
        discreteModelVars = unwrap_break_err!(BackendDAEUtil::foldEqSystem(inBackendDAE.clone(), Arc::new(SimCodeUtil::extractDiscreteModelVars), metamodelica::nil()), '__try0);
        (daeEquations, uniqueEqIndex, tempVars) = unwrap_break_err!(SimCodeUtil::createEquationsfromBackendDAE(inBackendDAE.clone(), uniqueEqIndex.clone(), tempVars.clone(), true, true, false, false), '__try0);
        emptyBDAE = unwrap_break_err!(BackendDAE::DAE(cons(BackendDAEUtil::createEqSystem(Util::getOption(inBackendDAE.shared.daeModeData.modelVars.clone())?, BackendEquation::emptyEqns(), metamodelica::nil(), crate::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION, BackendEquation::emptyEqns()), metamodelica::nil()), inBackendDAE.shared.clone()), '__try0);
        if unwrap_break_err!(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone()), '__try0) == literal!("symbolic") {
            (daeModeJac, daeModeSparsity, daeModeColoring, nonlinearPattern) = unwrap_break_err!((inBackendDAE.shared.symjacs.clone()).get(BackendDAE::SymbolicJacobianAIndex.clone()), '__try0);
            if isSome(inBackendDAE.shared.dataReconciliationData.clone()) {
                let BackendDAE::DATA_RECON { symbolicJacobian: _, setcVars: _, datareconinputs: _, setBVars: _, symbolicJacobianH: __pa5, .. } = (unwrap_break_err!(Util::getOption(inBackendDAE.shared.dataReconciliationData.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                jacH = __pa5.clone();
                if isSome(jacH.clone()) {
                    matrixnames = list![(literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("ADJ")).clone()];
                } else {
                    matrixnames = list![(literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
                }
            } else {
                matrixnames = list![(literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("F")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
            }
            (daeModeSP, uniqueEqIndex, tempVars) = unwrap_break_err!(SimCodeUtil::createSymbolicSimulationJacobian(Arc::new(BackendDAE::Jacobian::GENERIC_JACOBIAN { jacobian: daeModeJac.clone(), sparsePattern: daeModeSparsity.clone(), coloring: daeModeColoring.clone(), nonlinearPattern: nonlinearPattern.clone() }), uniqueEqIndex.clone(), tempVars.clone(), false), '__try0);
            tmpB = unwrap_break_err!(FlagsUtil::set(Flags::NO_START_CALC.clone(), true), '__try0);
            modelInfo = unwrap_break_err!(SimCodeUtil::createModelInfo(className.clone(), p.clone(), emptyBDAE.clone(), inInitDAE.clone(), functions.clone(), metamodelica::nil(), 0, spatialInfo.maxIndex.clone(), (fileDir.clone()).clone(), 0, tempVars.clone()), '__try0);
            unwrap_break_err!(FlagsUtil::set(Flags::NO_START_CALC.clone(), tmpB.clone()), '__try0);
            crefToSimVarHT = unwrap_break_err!(SimCodeUtil::createCrefToSimVarHT(modelInfo.clone()), '__try0);
            (symJacs, uniqueEqIndex) = unwrap_break_err!(SimCodeUtil::createSymbolicJacobianssSimCode(metamodelica::nil(), crefToSimVarHT.clone(), uniqueEqIndex.clone(), matrixnames.clone(), metamodelica::nil()), '__try0);
            symJacs = cons(unwrap_break_err!(Util::getOption(daeModeSP.clone()), '__try0), symJacs.clone()).reverse();
        } else {
            tmpB = unwrap_break_err!(FlagsUtil::set(Flags::NO_START_CALC.clone(), true), '__try0);
            modelInfo = unwrap_break_err!(SimCodeUtil::createModelInfo(className.clone(), p.clone(), emptyBDAE.clone(), inInitDAE.clone(), functions.clone(), metamodelica::nil(), 0, spatialInfo.maxIndex.clone(), (fileDir.clone()).clone(), 0, tempVars.clone()), '__try0);
            unwrap_break_err!(FlagsUtil::set(Flags::NO_START_CALC.clone(), tmpB.clone()), '__try0);
            crefToSimVarHT = unwrap_break_err!(SimCodeUtil::createCrefToSimVarHT(modelInfo.clone()), '__try0);
            if isSome(inBackendDAE.shared.dataReconciliationData.clone()) {
                let BackendDAE::DATA_RECON { symbolicJacobian: _, setcVars: _, datareconinputs: _, setBVars: _, symbolicJacobianH: __pa6, .. } = (unwrap_break_err!(Util::getOption(inBackendDAE.shared.dataReconciliationData.clone()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                jacH = __pa6.clone();
                if isSome(jacH.clone()) {
                    matrixnames = list![(literal!("A")).clone(), (literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("ADJ")).clone()];
                } else {
                    matrixnames = list![(literal!("A")).clone(), (literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
                }
            } else {
                matrixnames = list![(literal!("A")).clone(), (literal!("B")).clone(), (literal!("C")).clone(), (literal!("D")).clone(), (literal!("F")).clone(), (literal!("H")).clone(), (literal!("ADJ")).clone()];
            }
            (symJacs, uniqueEqIndex) = unwrap_break_err!(SimCodeUtil::createSymbolicJacobianssSimCode(metamodelica::nil(), crefToSimVarHT.clone(), uniqueEqIndex.clone(), matrixnames.clone(), metamodelica::nil()), '__try0);
        }
        SymbolicJacsNLS = metamodelica::nil();
        (initialEquations, modelInfo, SymbolicJacsTemp) = unwrap_break_err!(SimCodeUtil::addAlgebraicLoopsModelInfo(initialEquations.clone(), modelInfo.clone()), '__try0);
        SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
        (initialEquations_lambda0, modelInfo, SymbolicJacsTemp) = unwrap_break_err!(SimCodeUtil::addAlgebraicLoopsModelInfo(initialEquations_lambda0.clone(), modelInfo.clone()), '__try0);
        SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
        (parameterEquations, modelInfo, SymbolicJacsTemp) = unwrap_break_err!(SimCodeUtil::addAlgebraicLoopsModelInfo(parameterEquations.clone(), modelInfo.clone()), '__try0);
        SymbolicJacsNLS = listAppend(SymbolicJacsTemp.clone(), SymbolicJacsNLS.clone());
        (SymbolicJacs, modelInfo, SymbolicJacsTemp) = SimCodeUtil::addAlgebraicLoopsModelInfoSymJacs(symJacs.clone(), modelInfo.clone());
        jacobianEquations = unwrap_break_err!(SimCodeUtil::collectAllJacobianEquations(SymbolicJacs.clone()), '__try0);
        if debug.clone() {
            unwrap_break_err!(ExecStat::execStat((literal!("simCode: create Jacobian linear code")).clone()), '__try0);
        }
        SymbolicJacs = listAppend(SymbolicJacsNLS.clone().reverse(), listAppend(SymbolicJacs.clone(), SymbolicJacsTemp.clone()));
        jacobianSimvars = unwrap_break_err!(SimCodeUtil::collectAllJacobianVars(SymbolicJacs.clone()), '__try0);
        modelInfo = SimCodeUtil::setJacobianVars(jacobianSimvars.clone(), modelInfo.clone());
        crefToSimVarHT = List::fold(jacobianSimvars.clone(), Arc::new(HashTableCrefSimVar::addSimVarToHashTable), crefToSimVarHT.clone());
        seedVars = unwrap_break_err!(SimCodeUtil::collectAllSeedVars(SymbolicJacs.clone()), '__try0);
        modelInfo = SimCodeUtil::setSeedVars(seedVars.clone(), modelInfo.clone());
        crefToSimVarHT = List::fold(seedVars.clone(), Arc::new(HashTableCrefSimVar::addSimVarToHashTable), crefToSimVarHT.clone());
        varsLst = unwrap_break_err!(BackendVariable::equationSystemsVarsLst(inBackendDAE.eqs.clone()), '__try0);
        daeVars = BackendVariable::listVar(varsLst.clone());
        (_, resVars) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(daeVars.clone(), Arc::new(BackendVariable::collectVarKindVarinVariables), (fnptr!(BackendVariable::isDAEmodeResVar, BackendDAE::Var), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))), '__try0);
        (residualVars, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(resVars.clone(), Arc::new(SimCodeUtil::traversingdlowvarToSimvar), (metamodelica::nil(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))), '__try0);
        (residualVars, _) = SimCodeUtil::rewriteIndex(residualVars.clone(), 0);
        (residualVars, _, _) = SimCodeUtil::setVariableIndexHelper(residualVars.clone(), 0, 0);
        crefToSimVarHT = List::fold(residualVars.clone(), Arc::new(HashTableCrefSimVar::addSimVarToHashTable), crefToSimVarHT.clone());
        (_, auxVars) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(daeVars.clone(), Arc::new(BackendVariable::collectVarKindVarinVariables), (fnptr!(BackendVariable::isDAEmodeAuxVar, BackendDAE::Var), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))), '__try0);
        (auxiliaryVars, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(auxVars.clone(), Arc::new(SimCodeUtil::traversingdlowvarToSimvar), (metamodelica::nil(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))), '__try0);
        auxiliaryVars = unwrap_break_err!(List::sort(auxiliaryVars.clone(), Arc::new(SimCodeUtil::simVarCompareByCrefSubsAtEndlLexical)), '__try0);
        (auxiliaryVars, _) = SimCodeUtil::rewriteIndex(auxiliaryVars.clone(), 0);
        (auxiliaryVars, _, _) = SimCodeUtil::setVariableIndexHelper(auxiliaryVars.clone(), 0, 0);
        crefToSimVarHT = List::fold(auxiliaryVars.clone(), Arc::new(HashTableCrefSimVar::addSimVarToHashTable), crefToSimVarHT.clone());
        algStateVars = BackendVariable::listVar(inBackendDAE.shared.daeModeData.algStateVars.clone());
        (algebraicStateVars, _) = unwrap_break_err!(BackendVariable::traverseBackendDAEVars(algStateVars.clone(), Arc::new(SimCodeUtil::traversingdlowvarToSimvar), (metamodelica::nil(), BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone()))), '__try0);
        algebraicStateVars = unwrap_break_err!(SimCodeUtil::sortSimVarsAndWriteIndex(algebraicStateVars.clone(), crefToSimVarHT.clone()), '__try0);
        daeModeJacobian = unwrap_break_err!((inBackendDAE.shared.symjacs.clone()).get(BackendDAE::SymbolicJacobianAIndex.clone()), '__try0);
        let (__pa7, __pa8) = ::match_deref::match_deref! { match &(unwrap_break_err!(SimCodeUtil::createSymbolicJacobianssSimCode(list![daeModeJacobian.clone()], crefToSimVarHT.clone(), uniqueEqIndex.clone(), list![(literal!("daeMode")).clone()], metamodelica::nil()), '__try0)) {
            (Deref @ metamodelica::List::Cons { head: __pa7, tail: Deref @ metamodelica::List::Nil }, __pa8) => (__pa7.clone(), __pa8.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        symDAESparsPattern = __pa7.clone();
        uniqueEqIndex = __pa8.clone();
        daeModeSP = Some(symDAESparsPattern.clone());
        if unwrap_break_err!(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone()), '__try0) == literal!("symbolic") {
            SymbolicJacs = {
        let mut __acc: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>> = metamodelica::nil();
        for mut symjac in (SymbolicJacs.clone()).into_iter().cloned() {
            let __x = unwrap_break_err!(SimCodeUtil::syncDAEandSimJac(symjac.clone(), symDAESparsPattern.clone()), '__try0);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
        }
        daeModeConf = crate::SimCode::DaeModeConfig::ALL_EQUATIONS;
        daeModeData = Some(SimCode::DaeModeData { daeEquations: daeEquations.clone(), sparsityPattern: daeModeSP.clone(), residualVars: residualVars.clone(), algebraicVars: algebraicStateVars.clone(), auxiliaryVars: auxiliaryVars.clone(), modeCreated: daeModeConf.clone() });
        modelInfo = SimCodeUtil::addNumEqns(modelInfo.clone(), uniqueEqIndex.clone() - (jacobianEquations.clone().len() as i32));
        if stringEqual((Config::simCodeTarget()?).clone(), (literal!("Cpp")).clone()) {
            (varToArrayIndexMapping, varToIndexMapping) = unwrap_break_err!(SimCodeUtil::createVarToArrayIndexMapping(modelInfo.clone()), '__try0);
            (crefToClockIndexHT, _) = List::fold(inBackendDAE.eqs.clone().reverse(), Arc::new(SimCodeUtil::collectClockedVars), (HashTable::emptyHashTable(), 1));
        } else {
            varToArrayIndexMapping = HashTableCrIListArray::emptyHashTable();
            varToIndexMapping = HashTableCrILst::emptyHashTable();
            crefToClockIndexHT = HashTable::emptyHashTable();
        }
        simCode = SimCode::SimCode { scalarized: true, omsiData: None, inlineEquations: metamodelica::nil(), daeModeData: daeModeData.clone(), partitionData: SimCode::emptyPartitionData.clone(), fmiSimulationFlags: None, modelStructure: None, backendMapping: None, crefToClockIndexHT: crefToClockIndexHT.clone(), crefToSimVarHT: crefToSimVarHT.clone(), varToIndexMapping: varToIndexMapping.clone(), varToArrayIndexMapping: varToArrayIndexMapping.clone(), valueReferences: Arc::new(crate::AvlTreeCRToInt::Tree::EMPTY), hpcomData: HpcOmSimCode::emptyHpcomData().clone(), fmuTargetName: (literal!("")).clone(), fullPathPrefix: (literal!("")).clone(), fileNamePrefix: (filenamePrefix.clone()).clone(), simulationSettingsOpt: simSettingsOpt.clone(), jacobianMatrices: SymbolicJacs.clone(), spatialInfo: spatialInfo.clone(), delayedExps: SimCode::DelayedExpression { delayedExps: delayedExps.clone(), maxDelayedIndex: maxDelayedExpIndex.clone() }, makefileParams: makefileParams.clone(), extObjInfo: extObjInfo.clone(), discreteModelVars: discreteModelVars.clone(), timeEvents: timeEvents.clone(), relations: ZeroCrossings::updateIndices(relations.clone()), zeroCrossings: ZeroCrossings::updateIndices(zeroCrossings.clone()), classAttributes: metamodelica::nil(), constraints: metamodelica::nil(), stateSets: metamodelica::nil(), jacobianEquations: jacobianEquations.clone(), equationsForZeroCrossings: metamodelica::nil(), algorithmAndEquationAsserts: metamodelica::nil(), removedEquations: metamodelica::nil(), parameterEquations: parameterEquations.clone(), maxValueEquations: maxValueEquations.clone(), minValueEquations: minValueEquations.clone(), nominalValueEquations: nominalValueEquations.clone(), startValueEquations: startValueEquations.clone(), removedInitialEquations: removedInitialEquations.clone(), initialEquations_lambda0: initialEquations_lambda0.clone(), initialEquations: initialEquations.clone(), clockedPartitions: metamodelica::nil(), algebraicEquations: metamodelica::nil(), odeEquations: metamodelica::nil(), allEquations: metamodelica::nil(), localKnownVars: metamodelica::nil(), generic_loop_calls: metamodelica::nil(), externalFunctionIncludes: includes.clone(), recordDecls: recordDecls.clone(), literals: metamodelica::nil(), modelInfo: modelInfo.clone() };
        let (__pa10, (_, _, __pa11)) = unwrap_break_err!(SimCodeUtil::traverseExpsSimCode(simCode.clone(), Arc::new(SimCodeFunctionUtil::findLiteralsHelper), literals.clone()), '__try0);
        simCode = __pa10.clone();
        lits = __pa11.clone();
        simCode.literals = lits.clone().reverse();
        timeSimCode = unwrap_break_err!(System::realtimeTock(ClockIndexes::RT_CLOCK_SIMCODE.clone()), '__try0);
        unwrap_break_err!(ExecStat::execStat((literal!("SimCode")).clone()), '__try0);
        if unwrap_break_err!(Flags::isSet(Flags::SERIALIZED_SIZE.clone()), '__try0) {
            unwrap_break_err!(serializeNotify(simCode.clone(), (literal!("SimCode")).clone()), '__try0);
            unwrap_break_err!(ExecStat::execStat((literal!("Serialize simCode")).clone()), '__try0);
        }
        if unwrap_break_err!(Flags::isSet(Flags::DUMP_SIMCODE.clone()), '__try0) {
            unwrap_break_err!(SimCodeUtil::dumpSimCodeDebug(simCode.clone()), '__try0);
        }
        unwrap_break_err!(System::realtimeTick(ClockIndexes::RT_CLOCK_TEMPLATES.clone()), '__try0);
        unwrap_break_err!(callTargetTemplates(simCode.clone(), (Config::simCodeTarget()?).clone()), '__try0);
        timeTemplates = unwrap_break_err!(System::realtimeTock(ClockIndexes::RT_CLOCK_TEMPLATES.clone()), '__try0);
        unwrap_break_err!(ExecStat::execStat((literal!("Templates")).clone()), '__try0);
        return Ok((libs, fileDir, timeSimCode, timeTemplates));
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = None);
        ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints.clone());
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow in ")); __mm_s.push_str(&*literal!("SimCodeMain.generateModelCodeDAE")); __mm_s.push_str(&*literal!("...\n")); __mm_s.push_str(&*stringDelimitList(StackOverflow::readableStacktraceMessages()?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        StackOverflow::clearStacktraceMessages();
    }
    bail!("fail");
    Ok((libs, fileDir, timeSimCode, timeTemplates))
}

fn serializeNotify<T: Clone + 'static>(mut data: T, mut name: ArcStr) -> Result<()> {
    let mut sz: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut raw_sz: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    let mut nonSharedStringSize: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
    (sz, raw_sz, nonSharedStringSize) = System::getSizeOfData(data.clone());
    Error::addMessage(Error::SERIALIZED_SIZE.clone(), list![(name.clone()).clone(), (StringUtil::bytesToReadableUnit(sz.clone(), 4, metamodelica::OrderedFloat((500) as f64))).clone(), (StringUtil::bytesToReadableUnit(raw_sz.clone(), 4, metamodelica::OrderedFloat((500) as f64))).clone(), (StringUtil::bytesToReadableUnit(nonSharedStringSize.clone(), 4, metamodelica::OrderedFloat((500) as f64))).clone()])?;
    Ok(())
}

fn copyFiles(mut files: Arc<metamodelica::List<ArcStr>>, mut source: ArcStr, mut destination: ArcStr) -> Result<()> {
    let mut f2: ArcStr = arcstr::literal!("");
    let mut d2: ArcStr = arcstr::literal!("");
    for mut f in &*files.clone() {
        let mut f = f.clone();
        f2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*destination.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) }).clone();
        d2 = (System::dirname((f2.clone()).clone())).clone();
        if !(System::directoryExists((d2.clone()).clone())) {
            Error::assertion(Util::createDirectoryTree((d2.clone()).clone())?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to create directory ")); __mm_s.push_str(&*d2.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        }
        Error::assertion(System::copyFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*source.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) }).clone(), (f2.clone()).clone()), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to copy file ")); __mm_s.push_str(&*f.clone()); __mm_s.push_str(&*literal!(" from ")); __mm_s.push_str(&*source.clone()); __mm_s.push_str(&*literal!(" to ")); __mm_s.push_str(&*destination.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
    }
    Ok(())
}

