// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenCpp;
use crate::CodegenCppCommon;
use crate::CodegenCppInit;
use crate::CodegenFMU;
use crate::CodegenFMUCommon;
use openmodelica_ast::Absyn;
use openmodelica_backend::BackendDAE;
use openmodelica_backend::CodegenUtil;
use openmodelica_backend::SimCode;
use openmodelica_backend::SimCodeFunction;
use openmodelica_backend::SimCodeUtil;
use openmodelica_backend::SimCodeVar;
use openmodelica_frontend::Expression;
use openmodelica_frontend::Types;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_susan::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::FMI;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn fun_55(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_complexStartExpressions: Tpl::Text, mut in_a_numStringVars: Tpl::Text, mut in_a_numBoolVars: Tpl::Text, mut in_a_numIntVars: Tpl::Text, mut in_a_numRealVars: Tpl::Text, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_FMUType: ArcStr, mut in_a_FMUVersion: ArcStr, mut in_a_guid: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_complexStartExpressions: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_complexStartExpressions) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_stateDerVectorName.clone(), in_a_complexStartExpressions.clone(), in_a_numStringVars.clone(), in_a_numBoolVars.clone(), in_a_numIntVars.clone(), in_a_numRealVars.clone(), in_a_sourceFiles.clone(), in_a_FMUType.clone(), in_a_FMUVersion.clone(), in_a_guid.clone(), in_a_simCode.clone())) {
        (txt, false, a_stateDerVectorName, a_complexStartExpressions, _, _, _, _, a_sourceFiles, a_FMUType, a_FMUVersion, a_guid, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = CodegenFMU::fmuModelDescriptionFile(txt.clone(), a_simCode.clone(), (Tpl::textString(a_guid.clone())?).clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone(), a_sourceFiles.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
        (txt, _, a_stateDerVectorName, a_complexStartExpressions, a_numStringVars, a_numBoolVars, a_numIntVars, a_numRealVars, _, a_FMUType, a_FMUVersion, a_guid, a_simCode) => {
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_complexStartExpressions = (*a_complexStartExpressions).clone();
            (txt, a_complexStartExpressions, a_stateDerVectorName) = CodegenCppInit::modelInitXMLFile(txt.clone(), a_simCode.clone(), (Tpl::textString(a_numRealVars.clone())?).clone(), (Tpl::textString(a_numIntVars.clone())?).clone(), (Tpl::textString(a_numBoolVars.clone())?).clone(), (Tpl::textString(a_numStringVars.clone())?).clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone(), (Tpl::textString(a_guid.clone())?).clone(), true, (literal!("cpp-runtime")).clone(), a_complexStartExpressions.clone(), a_stateDerVectorName.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_complexStartExpressions.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_stateDerVectorName, out_a_complexStartExpressions))
}

pub fn translateModel(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUVersion: ArcStr, mut in_a_FMUType: ArcStr, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_FMUVersion.clone(), in_a_FMUType.clone(), in_a_sourceFiles.clone())) {
        (txt, i_simCode @ SimCode::SimCode { fileNamePrefix: i_fileNamePrefix, modelInfo: i_modelInfo @ SimCode::ModelInfo { name: _, .. }, .. }, a_FMUVersion, a_FMUType, a_sourceFiles) => {
            let mut ret_30: bool = false;
            let mut l_0___1: Tpl::Text;
            let mut txt_28: Tpl::Text;
            let mut txt_27: Tpl::Text;
            let mut txt_26: Tpl::Text;
            let mut txt_25: Tpl::Text;
            let mut txt_24: Tpl::Text;
            let mut txt_23: Tpl::Text;
            let mut txt_21: Tpl::Text;
            let mut ret_21: bool = false;
            let mut txt_20: Tpl::Text;
            let mut txt_19: Tpl::Text;
            let mut txt_18: Tpl::Text;
            let mut txt_17: Tpl::Text;
            let mut txt_16: Tpl::Text;
            let mut txt_15: Tpl::Text;
            let mut l_cpp: Tpl::Text;
            let mut ret_13: bool = false;
            let mut l_0__: Tpl::Text;
            let mut l_numStringVars: Tpl::Text;
            let mut l_numBoolVars: Tpl::Text;
            let mut l_numIntVars: Tpl::Text;
            let mut l_numRealVars: Tpl::Text;
            let mut l_complexStartExpressions: Tpl::Text;
            let mut l_extraFuncsDecl: Tpl::Text;
            let mut l_extraFuncs: Tpl::Text;
            let mut l_stateDerVectorName: Tpl::Text;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_target: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_guid: Tpl::Text;
            ret_1 = (System::getUUIDStr()).clone();
            l_guid = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            ret_3 = (Config::simulationCodeTarget()?).clone();
            l_target = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            l_stateDerVectorName = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__zDot")).clone() }))?;
            l_extraFuncs = Tpl::emptyTxt.clone();
            l_extraFuncsDecl = Tpl::emptyTxt.clone();
            l_complexStartExpressions = Tpl::emptyTxt.clone();
            l_numRealVars = CodegenCpp::numRealvars(Tpl::emptyTxt.clone(), i_modelInfo.clone())?;
            l_numIntVars = CodegenCpp::numIntvars(Tpl::emptyTxt.clone(), i_modelInfo.clone())?;
            l_numBoolVars = CodegenCpp::numBoolvars(Tpl::emptyTxt.clone(), i_modelInfo.clone())?;
            l_numStringVars = CodegenCpp::numStringvars(Tpl::emptyTxt.clone(), i_modelInfo.clone())?;
            ret_13 = FlagsUtil::set(Flags::HARDCODED_START_VALUES.clone(), true)?;
            l_0__ = Tpl::writeStr(Tpl::emptyTxt.clone(), (Tpl::booleanString(ret_13.clone())).clone())?;
            l_cpp = CodegenCpp::translateModel(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            (txt_15, l_extraFuncs, l_extraFuncsDecl, _) = fmuWriteOutputHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_16 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_16 = Tpl::writeStr(txt_16.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_16 = Tpl::writeTok(txt_16.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput.h")).clone() }))?;
            Tpl::textFile(txt_15.clone(), (Tpl::textString(txt_16.clone())?).clone())?;
            (txt_17, l_extraFuncs, l_extraFuncsDecl, _) = fmuModelHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(l_guid.clone())?).clone(), (a_FMUVersion.clone()).clone())?;
            txt_18 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_18 = Tpl::writeStr(txt_18.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_18 = Tpl::writeTok(txt_18.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU.h")).clone() }))?;
            Tpl::textFile(txt_17.clone(), (Tpl::textString(txt_18.clone())?).clone())?;
            (txt_19, l_extraFuncs, l_extraFuncsDecl, _) = fmuModelCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(l_guid.clone())?).clone(), (a_FMUVersion.clone()).clone())?;
            txt_20 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_20 = Tpl::writeStr(txt_20.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_20 = Tpl::writeTok(txt_20.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU.cpp")).clone() }))?;
            Tpl::textFile(txt_19.clone(), (Tpl::textString(txt_20.clone())?).clone())?;
            ret_21 = FMI::isFMIVersion10((a_FMUVersion.clone()).clone());
            (txt_21, l_stateDerVectorName, l_complexStartExpressions) = fun_55(Tpl::emptyTxt.clone(), ret_21.clone(), l_stateDerVectorName.clone(), l_complexStartExpressions.clone(), l_numStringVars.clone(), l_numBoolVars.clone(), l_numIntVars.clone(), l_numRealVars.clone(), a_sourceFiles.clone(), (a_FMUType.clone()).clone(), (a_FMUVersion.clone()).clone(), l_guid.clone(), i_simCode.clone())?;
            Tpl::textFile(txt_21.clone(), (literal!("modelDescription.xml")).clone())?;
            txt_23 = CodegenFMU::fmudeffile(Tpl::emptyTxt.clone(), i_simCode.clone(), (a_FMUVersion.clone()).clone())?;
            txt_24 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_24 = Tpl::writeTok(txt_24.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".def")).clone() }))?;
            Tpl::textFile(txt_23.clone(), (Tpl::textString(txt_24.clone())?).clone())?;
            (txt_25, l_extraFuncs, l_extraFuncsDecl, _) = fmuMakefile(Tpl::emptyTxt.clone(), (Tpl::textString(l_target.clone())?).clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (a_FMUVersion.clone()).clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone())?;
            txt_26 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_26 = Tpl::writeTok(txt_26.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.makefile")).clone() }))?;
            Tpl::textFile(txt_25.clone(), (Tpl::textString(txt_26.clone())?).clone())?;
            txt_27 = fmuCalcHelperMainfile(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_28 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_28 = Tpl::writeStr(txt_28.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_28 = Tpl::writeTok(txt_28.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CalcHelperMain.cpp")).clone() }))?;
            Tpl::textFile(txt_27.clone(), (Tpl::textString(txt_28.clone())?).clone())?;
            ret_30 = FlagsUtil::set(Flags::HARDCODED_START_VALUES.clone(), false)?;
            l_0___1 = Tpl::writeStr(Tpl::emptyTxt.clone(), (Tpl::booleanString(ret_30.clone())).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fileNamePrefix.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_fileNamePrefix) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"OMCpp")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("InitializeParameter.cpp\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("InitializeAlgVars.cpp\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmuCalcHelperMainfile(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/*****************************************************************************\n")).clone(), (literal!("*\n")).clone(), (literal!("* Helper file that includes all generated calculation files, except the alg loops.\n")).clone(), (literal!("* This file is generated by the OpenModelica Compiler and produced to speed-up the compile time.\n")).clone(), (literal!("*\n")).clone(), (literal!("*****************************************************************************/\n")).clone(), (literal!("#include <Core/ModelicaDefine.h>\n")).clone(), (literal!("#include <Core/Modelica.h>\n")).clone(), (literal!("#include <Core/System/FactoryExport.h>\n")).clone(), (literal!("#include <Core/DataExchange/SimData.h>\n")).clone(), (literal!("#include <Core/System/SimVars.h>\n")).clone(), (literal!("#include <Core/System/DiscreteEvents.h>\n")).clone(), (literal!("#include <Core/System/EventHandling.h>\n")).clone(), (literal!("#include <Core/Utils/extension/logger.hpp>\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Types.h\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Functions.h\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Jacobian.h\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Mixed.h\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("StateSelection.h\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("WriteOutput.h\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Initialize.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("FMU.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("AlgLoopMain.cpp\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Mixed.cpp\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("Functions.cpp\"\n")).clone() }))?;
            ret_0 = Flags::isSet(Flags::HARDCODED_START_VALUES.clone())?;
            ret_1 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            ret_2 = boolOr(ret_0.clone(), ret_1.clone());
            txt = fun_57(txt.clone(), ret_2.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"OMCpp")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Initialize.cpp\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Jacobian.cpp\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("StateSelection.cpp\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".cpp\"\n")).clone(), (literal!("#include \"OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU.cpp\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_59(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { simulationSettingsOpt: Some(SimCode::SimulationSettings { startTime: _, .. }), modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#pragma once\n")).clone(), (literal!("\n")).clone(), (literal!("// Dummy code for FMU that writes no output file\n")).clone(), (literal!("class ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput  : public IWriteOutput,public ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("StateSelection\n")).clone(), (literal!("{\n")).clone(), (literal!(" public:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput(IGlobalSettings* globalSettings, shared_ptr<ISimObjects> simObjects): ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("StateSelection(globalSettings, simObjects) {}\n")).clone(), (literal!("virtual ~")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("WriteOutput() {}\n")).clone(), (literal!("\n")).clone(), (literal!("virtual void writeOutput(const IWriteOutput::OUTPUT command = IWriteOutput::UNDEF_OUTPUT) {}\n")).clone(), (literal!("virtual IHistory* getHistory() {return NULL;}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" protected:\n")).clone(), (literal!("  void initialize() {}\n")).clone(), (literal!("};")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmuWriteOutputHeaderFile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    out_txt = fun_59(txt.clone(), a_simCode.clone())?;
    out_a_extraFuncs = a_extraFuncs.clone();
    out_a_extraFuncsDecl = a_extraFuncsDecl.clone();
    out_a_extraFuncsNamespace = a_extraFuncsNamespace.clone();
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_61(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }) => {
            let mut l_modelShortName: Tpl::Text;
            l_modelShortName = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// declaration for Cpp FMU target\n")).clone(), (literal!("\n")).clone(), (literal!("class ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU: public ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Initialize {\n")).clone(), (literal!(" public:\n")).clone(), (literal!("  // constructor\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("FMU(IGlobalSettings* globalSettings, shared_ptr<ISimObjects> simObjects);\n")).clone(), (literal!("\n")).clone(), (literal!("// initialization\n")).clone(), (literal!("virtual void initialize();\n")).clone(), (literal!("\n")).clone(), (literal!("// getters for given value references\n")).clone(), (literal!("virtual void getReal(const unsigned int vr[], size_t nvr, double value[]);\n")).clone(), (literal!("virtual void getInteger(const unsigned int vr[], size_t nvr, int value[]);\n")).clone(), (literal!("virtual void getBoolean(const unsigned int vr[], size_t nvr, int value[]);\n")).clone(), (literal!("virtual void getString(const unsigned int vr[], size_t nvr, string value[]);\n")).clone(), (literal!("\n")).clone(), (literal!("// setters for given value references\n")).clone(), (literal!("virtual void setReal(const unsigned int vr[], size_t nvr, const double value[]);\n")).clone(), (literal!("virtual void setInteger(const unsigned int vr[], size_t nvr, const int value[]);\n")).clone(), (literal!("virtual void setBoolean(const unsigned int vr[], size_t nvr, const int value[]);\n")).clone(), (literal!("virtual void setString(const unsigned int vr[], size_t nvr, const string value[]);\n")).clone(), (literal!("\n")).clone(), (literal!("// Jacobian\n")).clone(), (literal!("void getDirectionalDerivative(const unsigned int vrUnknown[], size_t nUnknown,\n")).clone(), (literal!("                              const unsigned int vrKnown[], size_t nKnown,\n")).clone(), (literal!("                              const double dvKnown[], double dvUnknown[]);\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" protected:\n")).clone(), (literal!("  static unsigned int _inputRefs[];  ///< Value references of input variables\n")).clone(), (literal!("  static unsigned int _outputRefs[]; ///< Value references of discrete states and outputs\n")).clone(), (literal!("};\n")).clone(), (literal!("\n")).clone(), (literal!("/// create instance of ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("FMU\n")).clone(), (literal!("static ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU *createSystemFMU(IGlobalSettings *globalSettings);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmuModelHeaderFile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_guid: ArcStr, mut a_FMUVersion: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    out_txt = fun_61(txt.clone(), a_simCode.clone())?;
    out_a_extraFuncs = a_extraFuncs.clone();
    out_a_extraFuncsDecl = a_extraFuncsDecl.clone();
    out_a_extraFuncsNamespace = a_extraFuncsNamespace.clone();
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_63(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_modelShortName: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelShortName.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eqs, tail: rest }, a_modelShortName, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = CodegenCpp::algloopMainfile2(txt.clone(), i_eqs.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), (Tpl::textString(a_modelShortName.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs) = lm_63(txt.clone(), rest.clone(), a_modelShortName.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs))
}

fn fun_64(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("createStaticAlgLoopSolverFactory(globalSettings, PATH(\"\"), PATH(\"\"))")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_65(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"FMU2/FMU2Wrapper.cpp\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <FMU/FMUWrapper.h>")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_66(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"FMU2/FMU2Interface.cpp\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <FMU/FMULibInterface.h>")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_67(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_simCode.clone())) {
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name: i_name, .. }, a_simCode) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut ret_0: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = SimCodeUtil::cref2simvar(i_name.clone(), a_simCode.clone())?;
            ret_1 = SimCodeUtil::getVariableIndex(ret_0.clone());
            ret_2 = intSub(ret_1.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_68(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_67(txt.clone(), i_var.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_68(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_69(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_simCode.clone())) {
        (txt, SimCodeVar::SimVar { varKind: BackendDAE::VarKind::CLOCKED_STATE { previousName: _, .. }, type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name: i_name, .. }, a_simCode) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut ret_0: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = SimCodeUtil::cref2simvar(i_name.clone(), a_simCode.clone())?;
            ret_1 = SimCodeUtil::getVariableIndex(ret_0.clone());
            ret_2 = intSub(ret_1.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { causality: Some(SimCodeVar::Causality::OUTPUT), type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name: i_name, .. }, a_simCode) => {
            let mut ret_5: i32 = 0;
            let mut ret_4: i32 = 0;
            let mut ret_3: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_3 = SimCodeUtil::cref2simvar(i_name.clone(), a_simCode.clone())?;
            ret_4 = SimCodeUtil::getVariableIndex(ret_3.clone());
            ret_5 = intSub(ret_4.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_5.clone())).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_70(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_69(txt.clone(), i_var.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_70(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn fmuModelCppFile(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_guid: ArcStr, mut in_a_FMUVersion: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (match (in_txt.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_guid.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { clockedPartitions: ref i_clockedPartitions, initialEquations: ref i_initialEquations, allEquations: ref i_allEquations, modelStructure: _, modelInfo: ref i_modelInfo @ SimCode::ModelInfo { name: ref i_modelInfo_name, vars: SimCodeVar::SimVars { algVars: ref i_algVars, inputVars: ref i_inputVars, .. }, .. }, .. }, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace, mut a_guid, mut a_FMUVersion) => {
            let mut ret_12: bool = false;
            let mut ret_11: bool = false;
            let mut str_10: ArcStr = arcstr::literal!("");
            let mut l_solverFactory: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut ret_7: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut ret_6: Arc<metamodelica::List<SimCode::SubPartition>> = metamodelica::nil();
            let mut ret_5: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut l_algloopfiles: Tpl::Text;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_modelLongName: Tpl::Text;
            let mut l_modelShortName: Tpl::Text;
            let mut l_modelName: Tpl::Text;
            l_modelName = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_modelShortName = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            ret_3 = (System::stringReplace((Tpl::textString(l_modelName.clone())?).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_modelLongName = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            ret_5 = listAppend(i_allEquations.clone(), i_initialEquations.clone());
            ret_6 = SimCodeUtil::getSubPartitions(i_clockedPartitions.clone());
            ret_7 = SimCodeUtil::getClockedEquations(ret_6.clone());
            ret_8 = listAppend(ret_5.clone(), ret_7.clone());
            l_algloopfiles = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_algloopfiles, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs) = lm_63(l_algloopfiles.clone(), ret_8.clone(), l_modelShortName.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), i_simCode.clone())?;
            l_algloopfiles = Tpl::popIter(l_algloopfiles.clone())?;
            str_10 = (Tpl::textString(l_algloopfiles.clone())?).clone();
            l_solverFactory = fun_64(Tpl::emptyTxt.clone(), (str_10.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// define model identifier and unique id\n")).clone(), (literal!("#define MODEL_IDENTIFIER ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelLongName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define MODEL_IDENTIFIER_SHORT ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define MODEL_CLASS ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("FMU\n")).clone(), (literal!("#define MODEL_GUID \"{")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\"\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = ModelDefineData(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_EVENT_INDICATORS ")).clone() }))?;
            txt = CodegenFMUCommon::getNumberOfEventIndicators(txt.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            ret_11 = FMI::isFMIVersion10((a_FMUVersion.clone()).clone());
            txt = fun_65(txt.clone(), ret_11.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_12 = FMI::isFMIVersion10((a_FMUVersion.clone()).clone());
            txt = fun_66(txt.clone(), ret_12.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("// SimObjects for ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("FMU\n")).clone(), (literal!("shared_ptr<IAlgLoopSolverFactory> createStaticAlgLoopSolverFactory(IGlobalSettings*, PATH, PATH);\n")).clone(), (literal!("\n")).clone(), (literal!("class ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("SimObjects : public ISimObjects {\n")).clone(), (literal!(" public:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("SimObjects(IGlobalSettings *globalSettings) {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algLoopSolverFactory = shared_ptr<IAlgLoopSolverFactory>(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_solverFactory.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SimObjects(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("SimObjects& instance) {\n")).clone(), (literal!("  _algLoopSolverFactory = instance._algLoopSolverFactory;\n")).clone(), (literal!("}\n")).clone(), (literal!("weak_ptr<ISimData> LoadSimData(string modelKey) {\n")).clone(), (literal!("  return shared_ptr<ISimData>();\n")).clone(), (literal!("}\n")).clone(), (literal!("weak_ptr<ISimVars> LoadSimVars(string modelKey, size_t dim_real, size_t dim_int, size_t dim_bool, size_t dim_string, size_t dim_pre_vars, size_t dim_z, size_t z_i) {\n")).clone(), (literal!("  _simVars = shared_ptr<ISimVars>(new SimVars(dim_real, dim_int, dim_bool, dim_string, dim_pre_vars, dim_z, z_i));\n")).clone(), (literal!("  return _simVars;\n")).clone(), (literal!("}\n")).clone(), (literal!("weak_ptr<IHistory> LoadWriter(size_t) {\n")).clone(), (literal!("  return shared_ptr<IHistory>();\n")).clone(), (literal!("}\n")).clone(), (literal!("shared_ptr<ISimData> getSimData(string modelKey) {\n")).clone(), (literal!("  return shared_ptr<ISimData>();\n")).clone(), (literal!("}\n")).clone(), (literal!("shared_ptr<ISimVars> getSimVars(string modelKey) {\n")).clone(), (literal!("  return _simVars;\n")).clone(), (literal!("}\n")).clone(), (literal!("void eraseSimData(string modelKey) {}\n")).clone(), (literal!("void eraseSimVars(string modelKey) {}\n")).clone(), (literal!("shared_ptr<IAlgLoopSolverFactory> getAlgLoopSolverFactory() {\n")).clone(), (literal!("  return _algLoopSolverFactory;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("ISimObjects* clone() {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return new ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("SimObjects(*this);\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" protected:\n")).clone(), (literal!("  shared_ptr<ISimVars> _simVars;\n")).clone(), (literal!("  shared_ptr<IAlgLoopSolverFactory> _algLoopSolverFactory;\n")).clone(), (literal!("};\n")).clone(), (literal!("\n")).clone(), (literal!("// create instance of ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("FMU\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("FMU *createSystemFMU(IGlobalSettings *globalSettings) {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("shared_ptr<ISimObjects> simObjects(new ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("SimObjects(globalSettings));\n")).clone(), (literal!("simObjects->LoadSimVars(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = CodegenCpp::numRealvars(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = CodegenCpp::numIntvars(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = CodegenCpp::numBoolvars(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = CodegenCpp::numStringvars(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = CodegenCpp::getPreVarsCount(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = CodegenCpp::numStatevars(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = CodegenCpp::numStateVarIndex(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("simObjects->LoadSimData(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\");\n")).clone(), (literal!("globalSettings->setOutputFormat(EMPTY);\n")).clone(), (literal!("return new ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("FMU(globalSettings, simObjects);\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("// value references of real inputs\n")).clone(), (literal!("unsigned int ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU::_inputRefs[] = {")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_68(txt.clone(), i_inputVars.clone(), i_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("// value references of real discrete states and outputs\n")).clone(), (literal!("unsigned int ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU::_outputRefs[] = {")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_70(txt.clone(), i_algVars.clone(), i_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("\n")).clone(), (literal!("// constructor\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU::")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("FMU(IGlobalSettings* globalSettings, shared_ptr<ISimObjects> simObjects)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("Initialize(globalSettings, simObjects) {\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("// initialization\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("FMU::initialize() {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("WriteOutput::initialize();\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("Initialize::initializeMemory();\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("Initialize::initializeFreeVariables();\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("Jacobian::initialize();\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("// getters\n")).clone()], lastHasNewLine: true }))?;
            txt = accessFunctions(txt.clone(), i_simCode.clone(), (literal!("get")).clone(), (Tpl::textString(l_modelShortName.clone())?).clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("// setters\n")).clone()], lastHasNewLine: true }))?;
            txt = accessFunctions(txt.clone(), i_simCode.clone(), (literal!("set")).clone(), (Tpl::textString(l_modelShortName.clone())?).clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("// Jacobian\n")).clone()], lastHasNewLine: true }))?;
            txt = directionalDerivativeFunction(txt.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (mut txt, _, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace, _, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
    });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_72(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = defineExternalFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_72(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn ModelDefineData(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { functions: ref i_functions, vars: SimCodeVar::SimVars { stateVars: _, .. }, varInfo: SimCode::VarInfo { numZeroCrossings: _, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* TODO: implement external functions in FMU wrapper for c++ target\n")).clone() }))?;
            System::tmpTickReset(0);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_72(txt.clone(), i_functions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_comment) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_comment.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_75(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_description: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_description.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone())) {
        (txt, false, a_description, a_useFlatArrayNotation, a_name) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define ")).clone() }))?;
            txt = CodegenCppCommon::cref(txt.clone(), a_name.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_ ")).clone() }))?;
            ret_0 = System::tmpTick();
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_description.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_76(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_description: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_description.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone())) {
        (txt, false, a_description, a_useFlatArrayNotation, a_name) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), a_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("der($dummy)")).clone());
            txt = fun_75(txt.clone(), ret_1.clone(), a_description.clone(), a_useFlatArrayNotation.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn DefineVariables(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_useFlatArrayNotation: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_useFlatArrayNotation.clone()) {
        (mut txt, SimCodeVar::SimVar { name: ref i_name, comment: mut i_comment, .. }, mut a_useFlatArrayNotation) => {
            let mut ret_2: bool = false;
            let mut txt_1: Tpl::Text;
            let mut l_description: Tpl::Text;
            l_description = fun_74(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            txt_1 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_2 = stringEq((Tpl::textString(txt_1.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_76(txt.clone(), ret_2.clone(), l_description.clone(), a_useFlatArrayNotation.clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn defineExternalFunction(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: i_language, extName: i_extName, dynamicLoad: true, .. }) => {
            let mut ret_1: i32 = 0;
            let mut l_fname: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fname = CodegenCpp::extFunctionName(Tpl::emptyTxt.clone(), (i_extName.clone()).clone(), (i_language.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define $P")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            ret_1 = System::tmpTick();
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_79(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_79(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_80(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_numStateVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_numStateVars) => {
            let mut txt = (*txt).clone();
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), a_numStateVars.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_80(txt.clone(), rest.clone(), a_numStateVars.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_numStateVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_numStateVars) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = intMul(2, a_numStateVars.clone());
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), ret_0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_81(txt.clone(), rest.clone(), a_numStateVars.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_82(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_numAlgVars: i32, mut in_a_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_numAlgVars.clone(), in_a_numStateVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_numAlgVars, a_numStateVars) => {
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = intMul(2, a_numStateVars.clone());
            ret_1 = intAdd(ret_0.clone(), a_numAlgVars.clone());
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), ret_1.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_82(txt.clone(), rest.clone(), a_numAlgVars.clone(), a_numStateVars.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_83(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("integerVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_83(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_84(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("booleanVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_84(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_85(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("stringVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_85(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_86(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("realParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_86(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_87(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("integerParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_87(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_88(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_88(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_89(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_89(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn setDefaultStartValues(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringParamVars: ref i_vars_stringParamVars, boolParamVars: ref i_vars_boolParamVars, intParamVars: ref i_vars_intParamVars, paramVars: ref i_vars_paramVars, stringAlgVars: ref i_vars_stringAlgVars, boolAlgVars: ref i_vars_boolAlgVars, intAlgVars: ref i_vars_intAlgVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, varInfo: SimCode::VarInfo { numAlgVars: mut i_numAlgVars, numStateVars: mut i_numStateVars, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// Set values for all variables that define a start value\n")).clone(), (literal!("void setDefaultStartValues(ModelInstance *comp) {\n")).clone(), (literal!("/*\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_79(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_80(txt.clone(), i_vars_derivativeVars.clone(), i_numStateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_81(txt.clone(), i_vars_algVars.clone(), i_numStateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_82(txt.clone(), i_vars_discreteAlgVars.clone(), i_numAlgVars.clone(), i_numStateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_83(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_84(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_85(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_86(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_87(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_88(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_89(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("*/\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_91(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_91(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_92(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_numStateVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_numStateVars) => {
            let mut txt = (*txt).clone();
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), a_numStateVars.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_92(txt.clone(), rest.clone(), a_numStateVars.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_93(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_numStateVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_numStateVars) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = intMul(2, a_numStateVars.clone());
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), ret_0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_93(txt.clone(), rest.clone(), a_numStateVars.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_94(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_numAlgVars: i32, mut in_a_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_numAlgVars.clone(), in_a_numStateVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_numAlgVars, a_numStateVars) => {
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = intMul(2, a_numStateVars.clone());
            ret_1 = intAdd(ret_0.clone(), a_numAlgVars.clone());
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone(), ret_1.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_94(txt.clone(), rest.clone(), a_numAlgVars.clone(), a_numStateVars.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_95(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initVals(txt.clone(), i_var.clone(), (literal!("integerVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_95(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_96(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initVals(txt.clone(), i_var.clone(), (literal!("booleanVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_96(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_97(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initVals(txt.clone(), i_var.clone(), (literal!("stringVars")).clone(), 0)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_97(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_98(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParams(txt.clone(), i_var.clone(), (literal!("realParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_98(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_99(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParams(txt.clone(), i_var.clone(), (literal!("integerParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_99(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_100(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParams(txt.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_100(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_101(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParams(txt.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_101(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn setStartValues(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringParamVars: ref i_vars_stringParamVars, boolParamVars: ref i_vars_boolParamVars, intParamVars: ref i_vars_intParamVars, paramVars: ref i_vars_paramVars, stringAlgVars: ref i_vars_stringAlgVars, boolAlgVars: ref i_vars_boolAlgVars, intAlgVars: ref i_vars_intAlgVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, varInfo: SimCode::VarInfo { numAlgVars: mut i_numAlgVars, numStateVars: mut i_numStateVars, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// Set values for all variables that define a start value\n")).clone(), (literal!("void setStartValues(ModelInstance *comp) {\n")).clone(), (literal!("/*\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_91(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_92(txt.clone(), i_vars_derivativeVars.clone(), i_numStateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_93(txt.clone(), i_vars_algVars.clone(), i_numStateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_94(txt.clone(), i_vars_discreteAlgVars.clone(), i_numAlgVars.clone(), i_numStateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_95(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_96(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_97(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_98(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_99(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_100(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_101(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("*/\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_103(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_offset: i32, mut in_a_index: i32, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_offset.clone(), in_a_index.clone(), in_a_arrayName.clone()) {
        (mut txt, false, mut a_offset, mut a_index, mut a_arrayName) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut l_str: Tpl::Text;
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData.")).clone() }))?;
            l_str = Tpl::writeStr(l_str.clone(), (a_arrayName.clone()).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            ret_1 = intAdd(a_index.clone(), a_offset.clone());
            l_str = Tpl::writeStr(l_str.clone(), (intString(ret_1.clone())).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" =  comp->fmuData->localData[0]->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            ret_2 = intAdd(a_index.clone(), a_offset.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_104(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_offset: i32, mut in_a_index: i32, mut in_a_arrayName: ArcStr, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_offset.clone(), in_a_index.clone(), in_a_arrayName.clone(), in_a_name.clone())) {
        (txt, false, a_offset, a_index, a_arrayName, a_name) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), a_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("der($dummy)")).clone());
            txt = fun_103(txt.clone(), ret_1.clone(), a_offset.clone(), a_index.clone(), (a_arrayName.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initVals(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_arrayName: ArcStr, mut in_a_offset: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone(), in_a_offset.clone()) {
        (mut txt, SimCodeVar::SimVar { index: mut i_index, name: ref i_name, .. }, mut a_arrayName, mut a_offset) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_104(txt.clone(), ret_1.clone(), a_offset.clone(), i_index.clone(), (a_arrayName.clone()).clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn initParams(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone()) {
        (mut txt, SimCodeVar::SimVar { index: mut i_index, .. }, mut a_arrayName) => {
            let mut l_str: Tpl::Text;
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData.")).clone() }))?;
            l_str = Tpl::writeStr(l_str.clone(), (a_arrayName.clone()).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            l_str = Tpl::writeStr(l_str.clone(), (intString(i_index.clone())).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = comp->fmuData->simulationInfo.")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_107(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_str.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = 0;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = 0;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = 0;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = 0;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = \"\";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKOWN_TYPE")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_108(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_type__: Arc<DAE::Type>, mut in_a_str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_type__.clone(), in_a_str.clone())) {
        (txt, Some(i_v), _, a_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = initVal(txt.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, None, a_type__, a_str) => {
            let mut txt = (*txt).clone();
            txt = fun_107(txt.clone(), a_type__.clone(), a_str.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initValsDefault(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_arrayName: ArcStr, mut in_a_offset: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone(), in_a_offset.clone()) {
        (mut txt, SimCodeVar::SimVar { initialValue: mut i_initialValue, type_: ref i_type__, index: mut i_index, .. }, mut a_arrayName, mut a_offset) => {
            let mut ret_1: i32 = 0;
            let mut l_str: Tpl::Text;
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData.")).clone() }))?;
            l_str = Tpl::writeStr(l_str.clone(), (a_arrayName.clone()).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            ret_1 = intAdd(i_index.clone(), a_offset.clone());
            l_str = Tpl::writeStr(l_str.clone(), (intString(ret_1.clone())).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start")).clone() }))?;
            txt = fun_108(txt.clone(), i_initialValue.clone(), i_type__.clone(), l_str.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_110(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_str.clone())) {
        (txt, Some(i_v), a_str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = initVal(txt.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initParamsDefault(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone()) {
        (mut txt, SimCodeVar::SimVar { initialValue: mut i_initialValue, index: mut i_index, .. }, mut a_arrayName) => {
            let mut l_str: Tpl::Text;
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData.")).clone() }))?;
            l_str = Tpl::writeStr(l_str.clone(), (a_arrayName.clone()).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            l_str = Tpl::writeStr(l_str.clone(), (intString(i_index.clone())).clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start")).clone() }))?;
            txt = fun_110(txt.clone(), i_initialValue.clone(), l_str.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_112(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_bool.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn initVal(mut in_txt: Tpl::Text, mut in_a_initialValue: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_string.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }) => {
            let mut txt = (*txt).clone();
            txt = fun_112(txt.clone(), i_bool.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: i_name, index: i_index }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*ENUM:")).clone() }))?;
            txt = CodegenUtil::dotPath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*ERROR* initial value of unknown type")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn setExternalFunction(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { functions: ref i_functions, vars: SimCodeVar::SimVars { stateVars: _, .. }, .. }) => {
            let mut l_externalFuncs: Tpl::Text;
            l_externalFuncs = setExternalFunctionsSwitch(Tpl::emptyTxt.clone(), i_functions.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiStatus setExternalFunction(ModelInstance* c, const fmiValueReference vr, const void* value){\n")).clone(), (literal!("  switch (vr) {\n")).clone(), (literal!("  /*\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_externalFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  */\n")).clone(), (literal!("      default:\n")).clone(), (literal!("          return fmiError;\n")).clone(), (literal!("  }\n")).clone(), (literal!("  return fmiOK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_115(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = setExternalFunctionSwitch(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_115(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn setExternalFunctionsSwitch(mut txt: Tpl::Text, mut a_functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_115(out_txt.clone(), a_functions.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn setExternalFunctionSwitch(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: i_language, extName: i_extName, dynamicLoad: true, .. }) => {
            let mut l_fname: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fname = CodegenCpp::extFunctionName(Tpl::emptyTxt.clone(), (i_extName.clone()).clone(), (i_language.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case $P")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : ptr_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("=(ptrT_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")value; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_118(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode, mut in_a_direction: ArcStr, mut in_a_modelShortName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone(), in_a_direction.clone(), in_a_modelShortName.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numParams: mut i_numParams, numDiscreteReal: mut i_numDiscreteReal, numAlgVars: mut i_numAlgVars, numStateVars: mut i_numStateVars, .. }, vars: SimCodeVar::SimVars { stringAliasVars: ref i_vars_stringAliasVars, stringParamVars: ref i_vars_stringParamVars, stringAlgVars: ref i_vars_stringAlgVars, boolAliasVars: ref i_vars_boolAliasVars, boolParamVars: ref i_vars_boolParamVars, boolAlgVars: ref i_vars_boolAlgVars, intAliasVars: ref i_vars_intAliasVars, intParamVars: ref i_vars_intParamVars, intAlgVars: ref i_vars_intAlgVars, aliasVars: ref i_vars_aliasVars, .. }, .. }, mut a_simCode, mut a_direction, mut a_modelShortName) => {
            let mut ret_12: i32 = 0;
            let mut ret_11: i32 = 0;
            let mut ret_10: i32 = 0;
            let mut ret_9: i32 = 0;
            let mut ret_8: i32 = 0;
            let mut ret_7: i32 = 0;
            let mut ret_6: i32 = 0;
            let mut ret_5: i32 = 0;
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            ret_0 = intMul(2, i_numStateVars.clone());
            ret_1 = intAdd(ret_0.clone(), i_numAlgVars.clone());
            ret_2 = intAdd(ret_1.clone(), i_numDiscreteReal.clone());
            ret_3 = intAdd(ret_2.clone(), i_numParams.clone());
            txt = accessVarsFunction(txt.clone(), a_simCode.clone(), (a_direction.clone()).clone(), (a_modelShortName.clone()).clone(), (literal!("Real")).clone(), (literal!("Real")).clone(), (literal!("double")).clone(), ret_3.clone(), i_vars_aliasVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_4 = (i_vars_intAlgVars.clone().len() as i32);
            ret_5 = (i_vars_intParamVars.clone().len() as i32);
            ret_6 = intAdd(ret_4.clone(), ret_5.clone());
            txt = accessVarsFunction(txt.clone(), a_simCode.clone(), (a_direction.clone()).clone(), (a_modelShortName.clone()).clone(), (literal!("Integer")).clone(), (literal!("Int")).clone(), (literal!("int")).clone(), ret_6.clone(), i_vars_intAliasVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_7 = (i_vars_boolAlgVars.clone().len() as i32);
            ret_8 = (i_vars_boolParamVars.clone().len() as i32);
            ret_9 = intAdd(ret_7.clone(), ret_8.clone());
            txt = accessVarsFunction(txt.clone(), a_simCode.clone(), (a_direction.clone()).clone(), (a_modelShortName.clone()).clone(), (literal!("Boolean")).clone(), (literal!("Bool")).clone(), (literal!("int")).clone(), ret_9.clone(), i_vars_boolAliasVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_10 = (i_vars_stringAlgVars.clone().len() as i32);
            ret_11 = (i_vars_stringParamVars.clone().len() as i32);
            ret_12 = intAdd(ret_10.clone(), ret_11.clone());
            txt = accessVarsFunction(txt.clone(), a_simCode.clone(), (a_direction.clone()).clone(), (a_modelShortName.clone()).clone(), (literal!("String")).clone(), (literal!("String")).clone(), (literal!("string")).clone(), ret_12.clone(), i_vars_stringAliasVars.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn accessFunctions(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_direction: ArcStr, mut a_modelShortName: ArcStr, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_118(txt.clone(), a_modelInfo.clone(), a_simCode.clone(), (a_direction.clone()).clone(), (a_modelShortName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_120(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("const")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_121(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_pointerName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_pointerName.clone()) {
        (mut txt, false, mut a_pointerName) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_pointerTo")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_pointerName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Vars[*vr] = *value;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_pointerName) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*value = _pointerTo")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_pointerName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Vars[*vr];")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_122(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_offset: i32, mut in_a_direction: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone(), in_a_offset.clone(), in_a_direction.clone(), in_a_simCode.clone()) {
        (mut txt, mut i_var @ SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::NEGATEDALIAS { varName: _ }, .. }, mut a_offset, mut a_direction, mut a_simCode) => {
            txt = accessVar(txt.clone(), a_simCode.clone(), (a_direction.clone()).clone(), i_var.clone(), a_offset.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_123(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_offset: i32, mut in_a_direction: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_offset.clone(), in_a_direction.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_offset, a_direction, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_122(txt.clone(), i_var.clone(), a_offset.clone(), (a_direction.clone()).clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_123(txt.clone(), rest.clone(), a_offset.clone(), (a_direction.clone()).clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn accessVarsFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_direction: ArcStr, mut a_modelShortName: ArcStr, mut a_typeName: ArcStr, mut a_pointerName: ArcStr, mut a_typeImpl: ArcStr, mut a_offset: i32, mut a_aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_2: bool = false;
    let mut ret_1: bool = false;
    let mut l_qualifier: Tpl::Text;
    ret_1 = stringEq((a_direction.clone()).clone(), (literal!("set")).clone());
    l_qualifier = fun_120(Tpl::emptyTxt.clone(), ret_1.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_modelShortName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU::")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_direction.clone()).clone())?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_typeName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(const unsigned int vr[], size_t nvr, ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_qualifier.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_typeImpl.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" value[]) {\n")).clone(), (literal!("  for (size_t i = 0; i < nvr; i++, vr++, value++) {\n")).clone(), (literal!("    // access variables and aliases in SimVars memory\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (*vr < _dim")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_typeName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    ret_2 = stringEq((a_direction.clone()).clone(), (literal!("get")).clone());
    out_txt = fun_121(out_txt.clone(), ret_2.clone(), (a_pointerName.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// convert negated aliases\n")).clone(), (literal!("else switch (*vr) {\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_123(out_txt.clone(), a_aliasVars.clone(), a_offset.clone(), (a_direction.clone()).clone(), a_simCode.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("default:\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("throw std::invalid_argument(\"")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_direction.clone()).clone())?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_typeName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" with wrong value reference \" + omcpp::to_string(*vr));\n")).clone() }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

fn fun_125(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr, mut in_a_descName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone(), in_a_descName.clone())) {
        (txt, Deref @ "", a_descName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_descName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt.clone()
        },
        (txt, i_comment, a_descName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_descName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_comment.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" */")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_126(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_cppSign: Tpl::Text, mut in_a_cppName: Tpl::Text, mut in_a_description: Tpl::Text, mut in_a_index: i32, mut in_a_offset: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_cppSign.clone(), in_a_cppName.clone(), in_a_description.clone(), in_a_index.clone(), in_a_offset.clone()) {
        (mut txt, false, mut a_cppSign, mut a_cppName, mut a_description, mut a_index, mut a_offset) => {
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = intAdd(a_offset.clone(), a_index.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_description.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_cppName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_cppSign.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*value; break;")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_cppSign, mut a_cppName, mut a_description, mut a_index, mut a_offset) => {
            let mut ret_1: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_1 = intAdd(a_offset.clone(), a_index.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_description.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*value = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_cppSign.clone())?;
            txt = Tpl::writeText(txt.clone(), a_cppName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; break;")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_127(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_direction: ArcStr, mut in_a_offset: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_direction.clone(), in_a_offset.clone()) {
        (mut txt, ref i_simVar @ SimCodeVar::SimVar { index: ref i_index, comment: ref i_comment, name: ref i_name, .. }, mut a_simCode, mut a_direction, mut a_offset) => {
            let mut ret_6: bool = false;
            let mut l_cppSign: Tpl::Text;
            let mut l_cppName: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt_1: Tpl::Text;
            let mut l_descName: Tpl::Text;
            txt_1 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_2 = (System::stringReplace((Tpl::textString(txt_1.clone())?).clone(), (literal!("$")).clone(), (literal!("_D_")).clone())?).clone();
            l_descName = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_2.clone()).clone())?;
            l_description = fun_125(Tpl::emptyTxt.clone(), (i_comment.clone()).clone(), l_descName.clone())?;
            l_cppName = getCppName(Tpl::emptyTxt.clone(), a_simCode.clone(), i_simVar.clone())?;
            l_cppSign = getCppSign(Tpl::emptyTxt.clone(), a_simCode.clone(), i_simVar.clone())?;
            ret_6 = stringEq((a_direction.clone()).clone(), (literal!("get")).clone());
            txt = fun_126(txt.clone(), ret_6.clone(), l_cppSign.clone(), l_cppName.clone(), l_description.clone(), i_index.clone(), a_offset.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn accessVar(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_direction: ArcStr, mut a_simVar: SimCodeVar::SimVar, mut a_offset: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_127(txt.clone(), a_simVar.clone(), a_simCode.clone(), (a_direction.clone()).clone(), a_offset.clone())?;
    Ok(out_txt)
}

fn fun_129(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable, mut in_a_actualName: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone(), in_a_actualName.clone(), in_a_simCode.clone()) {
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }, _, mut a_simCode) => {
            (txt, _, _, _, _, _) = CodegenCppCommon::cref1(txt.clone(), i_varName.clone(), a_simCode.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextOther().clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), false)?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }, _, mut a_simCode) => {
            (txt, _, _, _, _, _) = CodegenCppCommon::cref1(txt.clone(), i_varName.clone(), a_simCode.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextOther().clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), false)?;
            txt.clone()
        },
        (mut txt, _, mut a_actualName, _) => {
            txt = Tpl::writeText(txt.clone(), a_actualName.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_130(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone()) {
        (mut txt, SimCodeVar::SimVar { aliasvar: mut i_aliasvar, name: ref i_name, .. }, mut a_simCode) => {
            let mut l_actualName: Tpl::Text;
            (l_actualName, _, _, _, _, _) = CodegenCppCommon::cref1(Tpl::emptyTxt.clone(), i_name.clone(), a_simCode.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextOther().clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), false)?;
            txt = fun_129(txt.clone(), i_aliasvar.clone(), l_actualName.clone(), a_simCode.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getCppName(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_130(txt.clone(), a_simVar.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

fn fun_132(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone())) {
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_133(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable, mut in_a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_aliasvar.clone(), in_a_type__.clone())) {
        (txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: _ }, a_type__) => {
            let mut txt = (*txt).clone();
            txt = fun_132(txt.clone(), a_type__.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_134(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone()) {
        (mut txt, SimCodeVar::SimVar { aliasvar: mut i_aliasvar, type_: ref i_type__, .. }) => {
            txt = fun_133(txt.clone(), i_aliasvar.clone(), i_type__.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getCppSign(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_134(txt.clone(), a_simVar.clone())?;
    Ok(out_txt)
}

fn fun_136(mut in_txt: Tpl::Text, mut in_a_modelStructure_fmiDiscreteStates: SimCode::FmiDiscreteStates) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelStructure_fmiDiscreteStates.clone()) {
        (mut txt, SimCode::FmiDiscreteStates { fmiUnknownsList: ref i_fmiUnknownsList }) => {
            let mut ret_0: i32 = 0;
            ret_0 = (i_fmiUnknownsList.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_137(mut in_txt: Tpl::Text, mut in_a_fmiModelStructure: Option<SimCode::FmiModelStructure>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiModelStructure.clone()) {
        (mut txt, Some(SimCode::FmiModelStructure { fmiDiscreteStates: mut i_modelStructure_fmiDiscreteStates, .. })) => {
            txt = fun_136(txt.clone(), i_modelStructure_fmiDiscreteStates.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_138(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_dimDiscreteStates: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_dimDiscreteStates.clone()) {
        (mut txt, false, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  throw ModelicaSimulationError(MATH_FUNCTION, \"No derivative code, see flag disableDirectionalDerivatives\");")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_dimDiscreteStates) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  unsigned int idx, *ref_p, ref_1;\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int dimStates = _dimContinuousStates + ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_dimDiscreteStates.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("\n")).clone(), (literal!("_FMIDERjac_x.clear();\n")).clone(), (literal!("ref_p = NULL;\n")).clone(), (literal!("for (size_t j = 0; j < nKnown; j++) {\n")).clone(), (literal!("  idx = vrKnown[j];\n")).clone(), (literal!("  if (idx >= dimStates) {\n")).clone(), (literal!("    // find input reference\n")).clone(), (literal!("    if (ref_p == NULL || idx < ref_1)\n")).clone(), (literal!("      ref_p = _inputRefs; // reset ref_p if vrKnown decreases\n")).clone(), (literal!("    ref_p = std::find(ref_p, _inputRefs + sizeof(_inputRefs)/sizeof(unsigned int), vrKnown[j]);\n")).clone(), (literal!("    ref_1 = idx;\n")).clone(), (literal!("    idx = dimStates + (ref_p - _inputRefs);\n")).clone(), (literal!("  }\n")).clone(), (literal!("  if (idx >= _FMIDERjac_x.size())\n")).clone(), (literal!("    throw std::invalid_argument(\"getDirectionalDerivative with wrong value reference of known \" + omcpp::to_string(vrKnown[j]));\n")).clone(), (literal!("  _FMIDERjac_x(idx) = dvKnown[j];\n")).clone(), (literal!("}\n")).clone(), (literal!("calcFMIDERJacobianColumn();\n")).clone(), (literal!("ref_p = NULL;\n")).clone(), (literal!("for (size_t i = 0; i < nUnknown; i++) {\n")).clone(), (literal!("  idx = vrUnknown[i] - _dimContinuousStates; // derivatives behind states\n")).clone(), (literal!("  if (idx >= _dimContinuousStates) {\n")).clone(), (literal!("    // find output reference\n")).clone(), (literal!("    if (ref_p == NULL || idx < ref_1)\n")).clone(), (literal!("      ref_p = _outputRefs; // reset ref_p if vrUnknown decreases\n")).clone(), (literal!("    ref_p = std::find(ref_p, _outputRefs + sizeof(_outputRefs)/sizeof(unsigned int), vrUnknown[i]);\n")).clone(), (literal!("    ref_1 = idx;\n")).clone(), (literal!("    idx = _dimContinuousStates + (ref_p - _outputRefs);\n")).clone(), (literal!("  }\n")).clone(), (literal!("  if (idx >= _FMIDERjac_y.size())\n")).clone(), (literal!("    throw std::invalid_argument(\"getDirectionalDerivative with wrong value reference of unknown \" + omcpp::to_string(vrUnknown[i]));\n")).clone(), (literal!("  dvUnknown[i] = _FMIDERjac_y(idx);\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn directionalDerivativeFunction(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelStructure: ref i_fmiModelStructure, modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }) => {
            let mut ret_2: bool = false;
            let mut l_dimDiscreteStates: Tpl::Text;
            let mut l_modelShortName: Tpl::Text;
            l_modelShortName = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_dimDiscreteStates = fun_137(Tpl::emptyTxt.clone(), i_fmiModelStructure.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelShortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("FMU::getDirectionalDerivative(\n")).clone(), (literal!("    const unsigned int vrUnknown[], size_t nUnknown,\n")).clone(), (literal!("    const unsigned int vrKnown[], size_t nKnown,\n")).clone(), (literal!("    const double dvKnown[], double dvUnknown[])\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            ret_2 = SimCodeUtil::providesDirectionalDerivative(i_simCode.clone());
            txt = fun_138(txt.clone(), ret_2.clone(), l_dimDiscreteStates.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_140(mut in_txt: Tpl::Text, mut in_a_modelInfo_directory: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelInfo_directory.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_modelInfo_directory) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-L\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_modelInfo_directory.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_141(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lib, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lib.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_141(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_142(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-lOMOCLRuntime -lOpenCL")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_143(mut in_txt: Tpl::Text, mut in_a_s_method: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_s_method.clone())) {
        (txt, Deref @ "dassljac") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-D_OMC_JACOBIAN ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_144(mut in_txt: Tpl::Text, mut in_a_sopt: Option<SimCode::SimulationSettings>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_sopt.clone()) {
        (mut txt, Some(SimCode::SimulationSettings { method: mut i_s_method, .. })) => {
            txt = fun_143(txt.clone(), (i_s_method.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_145(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_makefileParams_omhome: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_makefileParams_omhome.clone()) {
        (mut txt, false, mut a_makefileParams_omhome) => {
            txt = Tpl::writeStr(txt.clone(), (a_makefileParams_omhome.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(OPENMODELICAHOME)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fmuTargetName: mut i_fmuTargetName, fileNamePrefix: mut i_fileNamePrefix, simulationSettingsOpt: mut i_sopt, makefileParams: SimCodeFunction::MakefileParams { platform: mut i_makefileParams_platform, omhome: mut i_makefileParams_omhome, libs: ref i_makefileParams_libs, .. }, modelInfo: SimCode::ModelInfo { directory: mut i_modelInfo_directory, .. }, .. }) => {
            let mut ret_7: bool = false;
            let mut ret_6: bool = false;
            let mut ret_5: bool = false;
            let mut l_extraCflags: Tpl::Text;
            let mut ret_3: bool = false;
            let mut l_ParModelicaLibs: Tpl::Text;
            let mut l_libsExtra: Tpl::Text;
            let mut l_dirExtra: Tpl::Text;
            l_dirExtra = fun_140(Tpl::emptyTxt.clone(), (i_modelInfo_directory.clone()).clone())?;
            l_libsExtra = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_libsExtra = lm_141(l_libsExtra.clone(), i_makefileParams_libs.clone())?;
            l_libsExtra = Tpl::popIter(l_libsExtra.clone())?;
            ret_3 = Config::acceptParModelicaGrammar()?;
            l_ParModelicaLibs = fun_142(Tpl::emptyTxt.clone(), ret_3.clone())?;
            l_extraCflags = fun_144(Tpl::emptyTxt.clone(), i_sopt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# Makefile generated by OpenModelica\n")).clone(), (literal!("# run with nmake from Visual Studio Command Prompt\n")).clone(), (literal!("# FMU packaging requires PATH to ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/mingw/bin\n")).clone(), (literal!("OMHOME=")).clone()], lastHasNewLine: false }))?;
            ret_5 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win32")).clone());
            ret_6 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win64")).clone());
            ret_7 = boolOr(ret_5.clone(), ret_6.clone());
            txt = fun_145(txt.clone(), ret_7.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("include ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/include/omc/cpp/ModelicaConfig_msvc.inc\n")).clone(), (literal!("include ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/include/omc/cpp/ModelicaLibraryConfig_msvc.inc\n")).clone(), (literal!("# Simulations use /Od by default\n")).clone(), (literal!("SIM_OR_DYNLOAD_OPT_LEVEL=\n")).clone(), (literal!("MODELICAUSERCFLAGS=\n")).clone(), (literal!("CXX=cl\n")).clone(), (literal!("EXEEXT=.exe\n")).clone(), (literal!("DLLEXT=.dll\n")).clone(), (literal!("\n")).clone(), (literal!("# /Od - Optimization disabled\n")).clone(), (literal!("# /EHa enable C++ EH (w/ SEH exceptions)\n")).clone(), (literal!("# /fp:except - consider floating-point exceptions when generating code\n")).clone(), (literal!("# /arch:SSE2 - enable use of instructions available with SSE2 enabled CPUs\n")).clone(), (literal!("# /I - Include Directories\n")).clone(), (literal!("# /DNOMINMAX - Define NOMINMAX (does what it says)\n")).clone(), (literal!("# /TP - Use C++ Compiler\n")).clone(), (literal!("\n")).clone(), (literal!("CFLAGS=$(SYSTEM_CFLAGS) /w /I\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/include/omc/cpp/\" /I\"$(BOOST_INCLUDE)\" /I\"$(SUITESPARSE_INCLUDE)\" /I. /TP /DNOMINMAX /DNO_INTERACTIVE_DEPENDENCY /DFMU_BUILD /DRUNTIME_STATIC_LINKING\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("# /MD - link with MSVCRT.LIB\n")).clone(), (literal!("# /link - [linker options and libraries]\n")).clone(), (literal!("# /LIBPATH: - Directories where libs can be found\n")).clone(), (literal!("OMCPP_SOLVER_LIBS=OMCppNewton_static.lib OMCppDgesv_static.lib OMCppDgesvSolver_static.lib -lOMCppSolver_static\n")).clone(), (literal!("MODELICA_UTILITIES_LIB=OMCppModelicaUtilities_static.lib\n")).clone(), (literal!("EXTRA_LIBS=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_dirExtra.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_libsExtra.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("LDFLAGS=/link /DLL /NOENTRY /LIBPATH:\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/lib/omc/cpp/msvc\" /LIBPATH:\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/bin\" OMCppSystem_static.lib OMCppMath_static.lib OMCppExtensionUtilities_static.lib OMCppFMU_static.lib $(OMCPP_SOLVER_LIBS) $(EXTRA_LIBS) $(MODELICA_UTILITIES_LIB)\n")).clone(), (literal!("\n")).clone(), (literal!("PLATFORM=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("\n")).clone(), (literal!("MODELICA_SYSTEM_LIB=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("CALCHELPERMAINFILE=OMCpp$(MODELICA_SYSTEM_LIB)CalcHelperMain.cpp\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmu: $(MODELICA_SYSTEM_LIB)$(DLLEXT)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -rf binaries\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("mkdir -p \"binaries/$(PLATFORM)\"\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("mv $(MODELICA_SYSTEM_LIB)$(DLLEXT) \"binaries/$(PLATFORM)/\"\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -f $(MODELICA_SYSTEM_LIB).fmu\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("zip -r \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmu\" modelDescription.xml binaries\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("rm -rf binaries\n")).clone(), (literal!("\n")).clone(), (literal!("$(MODELICA_SYSTEM_LIB)$(DLLEXT):\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(CXX) /Fe$(MODELICA_SYSTEM_LIB)$(DLLEXT) $(CALCHELPERMAINFILE) $(CFLAGS) $(LDFLAGS)")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_147(mut in_txt: Tpl::Text, mut in_a_modelInfo_directory: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelInfo_directory.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_modelInfo_directory) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-L\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_modelInfo_directory.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_148(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lib, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lib.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_148(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_149(mut in_txt: Tpl::Text, mut in_a_sopt: Option<SimCode::SimulationSettings>) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_sopt.clone()) {
        (mut txt, Some(SimCode::SimulationSettings { startTime: _, .. })) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    out_txt
}

fn fun_150(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
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

fn fun_151(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_omhome: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_omhome.clone())) {
        (txt, Deref @ "win32", a_omhome) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libgcc_s_*.dll\" \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libstdc++-6.dll\" \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libwinpthread-1.dll\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "win64", a_omhome) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libgcc_s_*.dll\" \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libstdc++-6.dll\" \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libwinpthread-1.dll\"")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_152(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_omhome: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_omhome.clone())) {
        (txt, Deref @ "win32", a_omhome) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libopenblas.dll\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "win64", a_omhome) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omhome.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/bin/libopenblas.dll\"")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_153(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"mkdir.exe\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"mkdir.exe\"")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mkdir")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_154(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_makefileParams_omhome: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_makefileParams_omhome.clone()) {
        (mut txt, false, mut a_makefileParams_omhome) => {
            txt = Tpl::writeStr(txt.clone(), (a_makefileParams_omhome.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(OPENMODELICAHOME)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_155(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_155(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_156(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_additionalLinkerFlags__GCC: ArcStr, mut in_a_additionalCFlags__GCC: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_additionalLinkerFlags__GCC.clone(), in_a_additionalCFlags__GCC.clone()) {
        (mut txt, SimCode::SimCode { fmuTargetName: mut i_fmuTargetName, fileNamePrefix: mut i_fileNamePrefix, simulationSettingsOpt: mut i_sopt, makefileParams: SimCodeFunction::MakefileParams { includes: ref i_makefileParams_includes, dllext: mut i_makefileParams_dllext, cxxcompiler: mut i_makefileParams_cxxcompiler, ccompiler: mut i_makefileParams_ccompiler, omhome: mut i_makefileParams_omhome, platform: mut i_makefileParams_platform, libs: ref i_makefileParams_libs, .. }, modelInfo: SimCode::ModelInfo { directory: mut i_modelInfo_directory, .. }, .. }, mut a_additionalLinkerFlags__GCC, mut a_additionalCFlags__GCC) => {
            let mut ret_12: bool = false;
            let mut ret_11: bool = false;
            let mut ret_10: bool = false;
            let mut l_mkdir: Tpl::Text;
            let mut str_8: ArcStr = arcstr::literal!("");
            let mut l_lapackbins: Tpl::Text;
            let mut str_6: ArcStr = arcstr::literal!("");
            let mut l_platformbins: Tpl::Text;
            let mut l_omhome: Tpl::Text;
            let mut l_platformstr: Tpl::Text;
            let mut l_extraCflags: Tpl::Text;
            let mut l_libsExtra: Tpl::Text;
            let mut l_dirExtra: Tpl::Text;
            l_dirExtra = fun_147(Tpl::emptyTxt.clone(), (i_modelInfo_directory.clone()).clone())?;
            l_libsExtra = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_libsExtra = lm_148(l_libsExtra.clone(), i_makefileParams_libs.clone())?;
            l_libsExtra = Tpl::popIter(l_libsExtra.clone())?;
            l_extraCflags = fun_149(Tpl::emptyTxt.clone(), i_sopt.clone());
            l_platformstr = fun_150(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_omhome = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            str_6 = (Tpl::textString(l_platformstr.clone())?).clone();
            l_platformbins = fun_151(Tpl::emptyTxt.clone(), (str_6.clone()).clone(), l_omhome.clone())?;
            str_8 = (Tpl::textString(l_platformstr.clone())?).clone();
            l_lapackbins = fun_152(Tpl::emptyTxt.clone(), (str_8.clone()).clone(), l_omhome.clone())?;
            l_mkdir = fun_153(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# Makefile generated by OpenModelica for native and cross compilation\n")).clone(), (literal!("# How to cross compile:\n")).clone(), (literal!("#  - build OpenModelica (omc) from source code (see github.com/OpenModelica)\n")).clone(), (literal!("#  - install a cross compiler, e.g. apt-get install g++-mingw-w64-i686\n")).clone(), (literal!("#  - fix or work around capitalization of windows.h in MSL, see\n")).clone(), (literal!("#      https://trac.modelica.org/Modelica/ticket/1962\n")).clone(), (literal!("#  - add the target triplet, e.g. i686-w64-mingw32, to\n")).clone(), (literal!("#      OMCompiler/SimulationRuntime/cpp/Makefile\n")).clone(), (literal!("#  - rebuild omc to add the new platform\n")).clone(), (literal!("#  - invoke the omc commands\n")).clone(), (literal!("#      setCommandLineOptions(\"--simCodeTarget=Cpp\");\n")).clone(), (literal!("#      buildModelFMU(MyModel, platforms={\"i686-w64-mingw32\"});\n")).clone(), (literal!("#  - alternatively call this Makefile with\n")).clone(), (literal!("#      make TARGET_TRIPLET=i686-w64-mingw32 -f ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.makefile\n")).clone(), (literal!("\n")).clone(), (literal!("#TARGET_TRIPLET=\n")).clone(), (literal!("# escape spaces for make\n")).clone(), (literal!("empty :=\n")).clone(), (literal!("space := $(empty) $(empty)\n")).clone(), (literal!("escape_path = $(subst $(space),\\$(space),$1)\n")).clone(), (literal!("\n")).clone(), (literal!("OMHOME:=")).clone()], lastHasNewLine: false }))?;
            ret_10 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win32")).clone());
            ret_11 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win64")).clone());
            ret_12 = boolOr(ret_10.clone(), ret_11.clone());
            txt = fun_154(txt.clone(), ret_12.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("OMHOME_ESCAPED:=$(call escape_path,$(OMHOME))\n")).clone(), (literal!("include $(OMHOME_ESCAPED)/include/omc/cpp/ModelicaConfig_gcc.inc\n")).clone(), (literal!("include $(OMHOME_ESCAPED)/include/omc/cpp/ModelicaLibraryConfig_gcc.inc\n")).clone(), (literal!("\n")).clone(), (literal!("# simulations use -O0 by default; can be changed to e.g. -O2 or -Ofast\n")).clone(), (literal!("SIM_OPT_LEVEL=-O0\n")).clone(), (literal!("# the default is ON by default, override the var from commandLine to skip the zipping of fmu  e.g. make -f ZIP_FMU=OFF\n")).clone(), (literal!("ZIP_FMU = ON\n")).clone(), (literal!("# native build or cross compilation\n")).clone(), (literal!("ifeq ($(TARGET_TRIPLET),)\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TRIPLET=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (arcstr::literal!(Autoconf::triple)).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CC=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_ccompiler.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CXX=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_cxxcompiler.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("ABI_CFLAG=\n")).clone(), (literal!("DLLEXT=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_dllext.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PLATFORM=")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_platformstr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("else\n")).clone(), (literal!("  TRIPLET=$(TARGET_TRIPLET)\n")).clone(), (literal!("  CC=$(TRIPLET)-gcc\n")).clone(), (literal!("  CXX=$(TRIPLET)-g++\n")).clone(), (literal!("  ABI_CFLAG=-D_GLIBCXX_USE_CXX11_ABI=0\n")).clone(), (literal!("  DLLEXT=$(if $(findstring mingw,$(TRIPLET)),.dll,.so)\n")).clone(), (literal!("  WORDSIZE=$(if $(findstring x86_64,$(TRIPLET)),64,32)\n")).clone(), (literal!("  PLATFORM=$(if $(findstring darwin,$(TRIPLET)),darwin,$(if $(findstring mingw,$(TRIPLET)),win,linux))$(WORDSIZE)\n")).clone(), (literal!("endif\n")).clone(), (literal!("\n")).clone(), (literal!("CFLAGS_BASED_ON_INIT_FILE=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_extraCflags.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("FMU_CFLAGS=$(subst -DUSE_THREAD,,$(subst -O0,$(SIM_OPT_LEVEL),$(SYSTEM_CFLAGS))) $(ABI_CFLAG)\n")).clone(), (literal!("CFLAGS=$(CFLAGS_BASED_ON_INIT_FILE) -Winvalid-pch $(FMU_CFLAGS) -DFMU_BUILD -DRUNTIME_STATIC_LINKING -I\"$(OMHOME)/include/omc/cpp\" -I\"$(UMFPACK_INCLUDE)\" -I\"$(SUNDIALS_INCLUDE)\" -I\"$(BOOST_INCLUDE)\" ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_155(txt.clone(), i_makefileParams_includes.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_additionalCFlags__GCC.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("ifeq ($(USE_LOGGER),ON)\n")).clone(), (literal!("  $(eval CFLAGS=$(CFLAGS) -DUSE_LOGGER)\n")).clone(), (literal!("endif\n")).clone(), (literal!("\n")).clone(), (literal!("LDFLAGS=-L\"$(OMHOME)/lib/$(TRIPLET)/omc/cpp\" ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_additionalLinkerFlags__GCC.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" -Wl,--no-undefined\n")).clone(), (literal!("\n")).clone(), (literal!("CALCHELPERMAINFILE=OMCpp")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("CalcHelperMain.cpp\n")).clone(), (literal!("\n")).clone(), (literal!("# CVode can be used for Co-Simulation FMUs, Kinsol is available to handle non linear equation systems\n")).clone(), (literal!("OMCPP_SOLVER_LIBS=-lOMCppNewton_static -lOMCppDgesvSolver_static -lOMCppSolver_static\n")).clone(), (literal!("ifeq ($(USE_FMU_SUNDIALS),ON)\n")).clone(), (literal!("$(eval OMCPP_SOLVER_LIBS=$(OMCPP_SOLVER_LIBS) -lOMCppKinsol_static $(SUNDIALS_LIBRARIES))\n")).clone(), (literal!("$(eval CFLAGS=-DENABLE_SUNDIALS_STATIC $(CFLAGS))\n")).clone(), (literal!("endif\n")).clone(), (literal!("\n")).clone(), (literal!("CPPFLAGS=$(CFLAGS)\n")).clone(), (literal!("\n")).clone(), (literal!("BINARIES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("$(DLLEXT)\n")).clone(), (literal!("\n")).clone(), (literal!("OMCPP_LIBS=-lOMCppSystem_static -lOMCppMath_static -lOMCppFMU_static $(OMCPP_SOLVER_LIBS) -lOMCppExtensionUtilities_static\n")).clone(), (literal!("MODELICA_UTILITIES_LIB=-lOMCppModelicaUtilities_static\n")).clone(), (literal!("EXTRA_LIBS=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_dirExtra.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_libsExtra.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("LIBS=$(EXTRA_LIBS) $(OMCPP_LIBS) $(MODELICA_UTILITIES_LIB) $(BASE_LIB)\n")).clone(), (literal!("\n")).clone(), (literal!("# link with simple dgesv or full lapack\n")).clone(), (literal!("ifeq ($(USE_DGESV),ON)\n")).clone(), (literal!("  $(eval LIBS=$(LIBS) -lOMCppDgesv_static)\n")).clone(), (literal!("else\n")).clone(), (literal!("  $(eval LIBS=$(LIBS) -L$(LAPACK_LIBS) $(LAPACK_LIBRARIES))\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(eval BINARIES=$(BINARIES) ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lapackbins.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("endif\n")).clone(), (literal!("\n")).clone(), (literal!("# need boost system lib prior to C++11, forcing also dynamic libs\n")).clone(), (literal!("ifeq ($(findstring USE_CPP_03,$(CFLAGS)),USE_CPP_03)\n")).clone(), (literal!("  $(eval LIBS=$(LIBS) -L\"$(BOOST_LIBS)\")\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(eval BINARIES=$(BINARIES) $(BOOST_LIBS)/lib$(BOOST_SYSTEM_LIB)$(DLLEXT) ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_platformbins.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# link static libs to avoid dependencies; can't link all static under Linux\n")).clone(), (literal!("else ifeq ($(findstring gcc,$(CC)),gcc)\n")).clone(), (literal!("  $(eval LIBS=$(LIBS) $(if $(findstring linux,$(PLATFORM)),-static-libstdc++ -static-libgcc,-static))\n")).clone(), (literal!("else ifeq ($(findstring clang,$(CC)),clang)\n")).clone(), (literal!("  $(eval LIBS=$(LIBS) $(if $(findstring linux,$(PLATFORM)),-static-libstdc++ -static-libgcc,-static))\n")).clone(), (literal!("endif\n")).clone(), (literal!("\n")).clone(), (literal!("CPPFILES=$(CALCHELPERMAINFILE)\n")).clone(), (literal!("OFILES=$(CPPFILES:.cpp=.o)\n")).clone(), (literal!("\n")).clone(), (literal!(".PHONY: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu $(CPPFILES) clean\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmu: $(OFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(CXX) -shared -o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("$(DLLEXT) $(OFILES) $(LDFLAGS) $(LIBS)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" -p \"binaries/$(PLATFORM)\"\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("mv $(BINARIES) \"binaries/$(PLATFORM)/\"\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -rf sources\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" -p sources\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("install -p OMCpp")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*.h OMCpp")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*.cpp ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init.xml ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.makefile sources/\n")).clone(), (literal!("ifeq ($(USE_FMU_SUNDIALS),ON)\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -rf documentation\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" -p \"documentation\"\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("cp $(SUNDIALS_LIBRARIES_KINSOL) \"binaries/$(PLATFORM)/\"\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("cp \"$(OMHOME)/share/omc/runtime/cpp/licenses/sundials.license\" \"documentation/\"\n")).clone(), (literal!("endif\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu\n")).clone(), (literal!("ifeq ($(ZIP_FMU),ON)\n")).clone(), (literal!("ifeq ($(USE_FMU_SUNDIALS),ON)\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("zip -r \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmu\" modelDescription.xml binaries sources documentation\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("rm -rf documentation\n")).clone(), (literal!("else\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("zip -r \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu\" modelDescription.xml binaries sources\n")).clone(), (literal!("endif\n")).clone(), (literal!("endif\n")).clone(), (literal!("\n")).clone(), (literal!("ifeq ($(ZIP_FMU),OFF)\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f OMCpp")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".def ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".sh ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".bat ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".makefile ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init.xml\n")).clone(), (literal!("endif\n")).clone(), (literal!("\n")).clone(), (literal!("clean:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f OMCpp")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".def ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".sh ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".bat ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".makefile ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_init.xml\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("rm -rf modelDescription.xml binaries sources documentation\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_157(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_additionalLinkerFlags__GCC: ArcStr, mut in_a_additionalCFlags__GCC: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_simCode.clone(), in_a_additionalLinkerFlags__GCC.clone(), in_a_additionalCFlags__GCC.clone())) {
        (txt, Deref @ "msvc", a_simCode, _, _) => {
            let mut txt = (*txt).clone();
            txt = fun_146(txt.clone(), a_simCode.clone())?;
            txt.clone()
        },
        (txt, Deref @ "gcc", a_simCode, a_additionalLinkerFlags__GCC, a_additionalCFlags__GCC) => {
            let mut txt = (*txt).clone();
            txt = fun_156(txt.clone(), a_simCode.clone(), (a_additionalLinkerFlags__GCC.clone()).clone(), (a_additionalCFlags__GCC.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn fmuMakefile(mut txt: Tpl::Text, mut a_target: ArcStr, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_FMUVersion: ArcStr, mut a_additionalLinkerFlags__GCC: ArcStr, mut a_additionalLinkerFlags__MSVC: ArcStr, mut a_additionalCFlags__GCC: ArcStr, mut a_additionalCFlags__MSVC: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut str_1: ArcStr = arcstr::literal!("");
    let mut txt_0: Tpl::Text;
    txt_0 = CodegenUtil::getGeneralTarget(Tpl::emptyTxt.clone(), (a_target.clone()).clone())?;
    str_1 = (Tpl::textString(txt_0.clone())?).clone();
    out_txt = fun_157(txt.clone(), (str_1.clone()).clone(), a_simCode.clone(), (a_additionalLinkerFlags__GCC.clone()).clone(), (a_additionalCFlags__GCC.clone()).clone())?;
    out_a_extraFuncs = a_extraFuncs.clone();
    out_a_extraFuncsDecl = a_extraFuncsDecl.clone();
    out_a_extraFuncsNamespace = a_extraFuncsNamespace.clone();
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

