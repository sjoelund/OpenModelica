// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenCppOMSI;
use crate::CodegenUtil;
use crate::SimCode;
use crate::SimCodeFunction;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::Settings;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;

fn fun_50(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }) => {
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut l_extraFuncsDecl: Tpl::Text;
            let mut l_extraFuncs: Tpl::Text;
            l_extraFuncs = Tpl::emptyTxt.clone();
            l_extraFuncsDecl = Tpl::emptyTxt.clone();
            (txt_2, l_extraFuncs, l_extraFuncsDecl, _) = simulationOMSUCPPMainRunScript(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (literal!("")).clone(), (literal!("")).clone(), (literal!("exec")).clone())?;
            txt_3 = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            (txt_3, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCppOMSI::simulationMainRunScriptSuffix(txt_3.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            Tpl::textFile(txt_2.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn translateModel(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_FMUVersion: ArcStr, mut a_FMUType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_50(txt.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

fn fun_52(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_settings_method: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_settings_method.clone()) {
        (mut txt, SimCode::SimCode { daeModeData: None, .. }, mut a_settings_method) => {
            txt = Tpl::writeStr(txt.clone(), (a_settings_method.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ida")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "i386-pc-linux") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("linux32")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "x86_64-linux") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("linux64")).clone() }))?;
            txt.clone()
        },
        (txt, i_makefileParams_platform) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-O none")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_55(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_path, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_path.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_55(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-u true -p ")).clone() }))?;
            ret_0 = Flags::getConfigInt(Flags::ZEROMQ_PUB_PORT.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -s ")).clone() }))?;
            ret_1 = Flags::getConfigInt(Flags::ZEROMQ_SUB_PORT.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -v ")).clone() }))?;
            ret_2 = (Flags::getConfigString(Flags::ZEROMQ_SERVER_ID.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -c ")).clone() }))?;
            ret_3 = (Flags::getConfigString(Flags::ZEROMQ_CLIENT_ID.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -g ")).clone() }))?;
            ret_4 = (Flags::getConfigString(Flags::ZEROMQ_JOB_ID.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr, mut in_a_preRunCommandWindows: ArcStr, mut in_a_libPaths: Tpl::Text, mut in_a_libFolder: Tpl::Text, mut in_a_outputParameter: Tpl::Text, mut in_a_zermMQParams: Tpl::Text, mut in_a_execParameters: Tpl::Text, mut in_a_binFolder: Tpl::Text, mut in_a_execCommandLinux: ArcStr, mut in_a_preRunCommandLinux: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone(), in_a_preRunCommandWindows.clone(), in_a_libPaths.clone(), in_a_libFolder.clone(), in_a_outputParameter.clone(), in_a_zermMQParams.clone(), in_a_execParameters.clone(), in_a_binFolder.clone(), in_a_execCommandLinux.clone(), in_a_preRunCommandLinux.clone())) {
        (txt, Deref @ "linux32", _, _, _, a_outputParameter, a_zermMQParams, a_execParameters, a_binFolder, a_execCommandLinux, a_preRunCommandLinux) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#!/bin/sh\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_preRunCommandLinux.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_execCommandLinux.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_binFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/OMCppOSUSimulation ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_execParameters.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_zermMQParams.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_outputParameter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" $*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "linux64", _, _, _, a_outputParameter, a_zermMQParams, a_execParameters, a_binFolder, a_execCommandLinux, a_preRunCommandLinux) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#!/bin/sh\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_preRunCommandLinux.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_execCommandLinux.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_binFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/OMCppOSUSimulation ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_execParameters.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_zermMQParams.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_outputParameter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" $*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "win32", a_preRunCommandWindows, a_libPaths, a_libFolder, a_outputParameter, a_zermMQParams, a_execParameters, a_binFolder, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("@echo off\n")).clone(), (literal!("SET PATH=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_binFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_libFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_libPaths.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";%PATH%\n")).clone(), (literal!("REM ::export PATH=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_libFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(":$PATH REPLACE C: with /C/\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_preRunCommandWindows.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCppOSUSimulation.exe ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_execParameters.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_zermMQParams.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_outputParameter.clone())?;
            txt.clone()
        },
        (txt, Deref @ "win64", a_preRunCommandWindows, a_libPaths, a_libFolder, a_outputParameter, a_zermMQParams, a_execParameters, a_binFolder, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("@echo off\n")).clone(), (literal!("SET PATH=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_binFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_libFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_libPaths.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";%PATH%\n")).clone(), (literal!("REM ::export PATH=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_libFolder.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(":$PATH REPLACE C: with /C/\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_preRunCommandWindows.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCppOSUSimulation.exe ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_execParameters.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_zermMQParams.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_outputParameter.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn simulationOMSUCPPMainRunScript(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_preRunCommandLinux: ArcStr, mut in_a_preRunCommandWindows: ArcStr, mut in_a_execCommandLinux: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (match (in_txt.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_preRunCommandLinux.clone(), in_a_preRunCommandWindows.clone(), in_a_execCommandLinux.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { fileNamePrefix: ref i_fileNamePrefix, simulationSettingsOpt: Some(SimCode::SimulationSettings { outputFormat: ref i_settings_outputFormat, method: ref i_settings_method, tolerance: ref i_settings_tolerance, numberOfIntervals: ref i_settings_numberOfIntervals, stepSize: ref i_settings_stepSize, stopTime: ref i_settings_stopTime, startTime: ref i_settings_startTime, .. }), makefileParams: SimCodeFunction::MakefileParams { libPaths: ref i_makefileParams_libPaths, platform: ref i_makefileParams_platform, omhome: ref i_makefileParams_omhome, compileDir: ref i_makefileParams_compileDir, .. }, modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace, mut a_preRunCommandLinux, mut a_preRunCommandWindows, mut a_execCommandLinux) => {
            let mut ret_23: bool = false;
            let mut l_zermMQParams: Tpl::Text;
            let mut l_libPaths: Tpl::Text;
            let mut ret_20: ArcStr = arcstr::literal!("");
            let mut l_binFolder: Tpl::Text;
            let mut ret_18: ArcStr = arcstr::literal!("");
            let mut l_libFolder: Tpl::Text;
            let mut ret_16: bool = false;
            let mut l_outputParameter: Tpl::Text;
            let mut ret_14: bool = false;
            let mut ret_13: ArcStr = arcstr::literal!("");
            let mut l_execParameters: Tpl::Text;
            let mut l_platformstr: Tpl::Text;
            let mut l_fileNamePrefixx: Tpl::Text;
            let mut l_modelName: Tpl::Text;
            let mut l_outputformat: Tpl::Text;
            let mut l_home: Tpl::Text;
            let mut l_moLib: Tpl::Text;
            let mut l_solver: Tpl::Text;
            let mut l_tol: Tpl::Text;
            let mut l_intervals: Tpl::Text;
            let mut l_stepsize: Tpl::Text;
            let mut l_end: Tpl::Text;
            let mut l_start: Tpl::Text;
            l_start = Tpl::writeStr(Tpl::emptyTxt.clone(), (realString(i_settings_startTime.clone())).clone())?;
            l_end = Tpl::writeStr(Tpl::emptyTxt.clone(), (realString(i_settings_stopTime.clone())).clone())?;
            l_stepsize = Tpl::writeStr(Tpl::emptyTxt.clone(), (realString(i_settings_stepSize.clone())).clone())?;
            l_intervals = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_settings_numberOfIntervals.clone())).clone())?;
            l_tol = Tpl::writeStr(Tpl::emptyTxt.clone(), (realString(i_settings_tolerance.clone())).clone())?;
            l_solver = fun_52(Tpl::emptyTxt.clone(), i_simCode.clone(), (i_settings_method.clone()).clone())?;
            l_moLib = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_makefileParams_compileDir.clone()).clone())?;
            l_home = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            l_outputformat = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_settings_outputFormat.clone()).clone())?;
            l_modelName = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_fileNamePrefixx = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            l_platformstr = fun_53(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_execParameters = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-S ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_start.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -E ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_end.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -H ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_stepsize.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -G ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_intervals.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -P ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_outputformat.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -T ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_tol.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -I ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_solver.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -R ")).clone() }))?;
            ret_13 = (Config::simulationCodeTarget()?).clone();
            (l_execParameters, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = CodegenCppOMSI::simulationLibDir(l_execParameters.clone(), (ret_13.clone()).clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -M ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_moLib.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -r ")).clone() }))?;
            ret_14 = Testsuite::isRunning()?;
            (l_execParameters, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = CodegenCppOMSI::simulationResults(l_execParameters.clone(), ret_14.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -a ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_moLib.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -o ")).clone() }))?;
            l_execParameters = Tpl::writeText(l_execParameters.clone(), l_fileNamePrefixx.clone())?;
            l_execParameters = Tpl::writeTok(l_execParameters.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmu")).clone() }))?;
            ret_16 = stringEq((i_settings_outputFormat.clone()).clone(), (literal!("empty")).clone());
            l_outputParameter = fun_54(Tpl::emptyTxt.clone(), ret_16.clone())?;
            ret_18 = (Config::simulationCodeTarget()?).clone();
            (l_libFolder, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = CodegenCppOMSI::simulationLibDir(Tpl::emptyTxt.clone(), (ret_18.clone()).clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            ret_20 = (Config::simulationCodeTarget()?).clone();
            l_binFolder = CodegenCppOMSI::simulationBinDir(Tpl::emptyTxt.clone(), (ret_20.clone()).clone(), i_simCode.clone())?;
            l_libPaths = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_libPaths = lm_55(l_libPaths.clone(), i_makefileParams_libPaths.clone())?;
            l_libPaths = Tpl::popIter(l_libPaths.clone())?;
            ret_23 = Flags::getConfigBool(Flags::USE_ZEROMQ_IN_SIM.clone())?;
            l_zermMQParams = fun_56(Tpl::emptyTxt.clone(), ret_23.clone())?;
            txt = fun_57(txt.clone(), (i_makefileParams_platform.clone()).clone(), (a_preRunCommandWindows.clone()).clone(), l_libPaths.clone(), l_libFolder.clone(), l_outputParameter.clone(), l_zermMQParams.clone(), l_execParameters.clone(), l_binFolder.clone(), (a_execCommandLinux.clone()).clone(), (a_preRunCommandLinux.clone()).clone())?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (mut txt, _, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace, _, _, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
    });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

