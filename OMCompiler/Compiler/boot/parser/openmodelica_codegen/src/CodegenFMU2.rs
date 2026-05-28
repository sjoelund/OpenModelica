// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenFMUCommon;
use crate::CodegenUtilSimulation;
use openmodelica_ast::Absyn;
use openmodelica_backend::CodegenUtil;
use openmodelica_backend::SimCode;
use openmodelica_backend::SimCodeVar;
use openmodelica_susan::Tpl;
use openmodelica_util::Error;
use openmodelica_util::FMI;
use openmodelica_util::Flags;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;

fn fun_52(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_sourceFiles.clone(), in_a_simCode.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_sourceFiles, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = CodegenFMUCommon::ModelExchange(txt.clone(), a_simCode.clone(), a_sourceFiles.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_sourceFiles.clone(), in_a_simCode.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_sourceFiles, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = CoSimulation(txt.clone(), a_simCode.clone(), a_sourceFiles.clone())?;
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
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<LogCategories>\n")).clone(), (literal!("  <Category name=\"logEvents\" />\n")).clone(), (literal!("  <Category name=\"logSingularLinearSystems\" />\n")).clone(), (literal!("  <Category name=\"logNonlinearSystems\" />\n")).clone(), (literal!("  <Category name=\"logDynamicStateSelection\" />\n")).clone(), (literal!("  <Category name=\"logStatusWarning\" />\n")).clone(), (literal!("  <Category name=\"logStatusDiscard\" />\n")).clone(), (literal!("  <Category name=\"logStatusError\" />\n")).clone(), (literal!("  <Category name=\"logStatusFatal\" />\n")).clone(), (literal!("  <Category name=\"logStatusPending\" />\n")).clone(), (literal!("  <Category name=\"logAll\" />\n")).clone(), (literal!("  <Category name=\"logFmi2Call\" />\n")).clone(), (literal!("</LogCategories>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<LogCategories>\n")).clone(), (literal!("  <Category name=\"logEvents\" description=\"logEvents\" />\n")).clone(), (literal!("  <Category name=\"logSingularLinearSystems\" description=\"logSingularLinearSystems\" />\n")).clone(), (literal!("  <Category name=\"logNonlinearSystems\" description=\"logNonlinearSystems\" />\n")).clone(), (literal!("  <Category name=\"logDynamicStateSelection\" description=\"logDynamicStateSelection\" />\n")).clone(), (literal!("  <Category name=\"logStatusWarning\" description=\"logStatusWarning\" />\n")).clone(), (literal!("  <Category name=\"logStatusDiscard\" description=\"logStatusDiscard\" />\n")).clone(), (literal!("  <Category name=\"logStatusError\" description=\"logStatusError\" />\n")).clone(), (literal!("  <Category name=\"logStatusFatal\" description=\"logStatusFatal\" />\n")).clone(), (literal!("  <Category name=\"logStatusPending\" description=\"logStatusPending\" />\n")).clone(), (literal!("  <Category name=\"logAll\" description=\"logAll\" />\n")).clone(), (literal!("  <Category name=\"logFmi2Call\" description=\"logFmi2Call\" />\n")).clone(), (literal!("</LogCategories>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmiModelDescription(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr, mut in_a_FMUType: ArcStr, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone(), in_a_FMUType.clone(), in_a_sourceFiles.clone())) {
        (txt, i_simCode @ SimCode::SimCode { modelStructure: i_modelStructure, simulationSettingsOpt: i_simulationSettingsOpt, .. }, a_guid, a_FMUType, a_sourceFiles) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fmiModelDescription\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fmiModelDescriptionAttributes(txt.clone(), i_simCode.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            ret_0 = FMI::isFMIMEType((a_FMUType.clone()).clone());
            txt = fun_52(txt.clone(), ret_0.clone(), a_sourceFiles.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_1 = FMI::isFMICSType((a_FMUType.clone()).clone());
            txt = fun_53(txt.clone(), ret_1.clone(), a_sourceFiles.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = CodegenFMUCommon::UnitDefinitions(txt.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = CodegenFMUCommon::fmiTypeDefinitions(txt.clone(), i_simCode.clone(), (literal!("2.0")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_2 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_54(txt.clone(), ret_2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = CodegenFMUCommon::DefaultExperiment(txt.clone(), i_simulationSettingsOpt.clone(), (literal!("2.0")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = CodegenFMUCommon::fmiModelVariables(txt.clone(), i_simCode.clone(), (literal!("2.0")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = CodegenFMUCommon::ModelStructure(txt.clone(), i_modelStructure.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fmiModelDescription>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_author: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_author.clone()) {
        (mut txt, false, mut a_author) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("author=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((Tpl::textString(a_author.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_copyright: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_copyright.clone()) {
        (mut txt, false, mut a_copyright) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("copyright=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((Tpl::textString(a_copyright.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_58(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_license: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_license.clone()) {
        (mut txt, false, mut a_license) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("license=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((Tpl::textString(a_license.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmiModelDescriptionAttributes(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { license: ref i_modelInfo_license, copyright: ref i_modelInfo_copyright, version: ref i_modelInfo_version, author: ref i_modelInfo_author, description: ref i_modelInfo_description, name: ref i_modelInfo_name, vars: SimCodeVar::SimVars { stateVars: _, .. }, varInfo: SimCode::VarInfo { numZeroCrossings: _, .. }, .. }, .. }, mut a_guid) => {
            let mut ret_20: ArcStr = arcstr::literal!("");
            let mut ret_19: ArcStr = arcstr::literal!("");
            let mut ret_18: bool = false;
            let mut ret_17: bool = false;
            let mut ret_16: bool = false;
            let mut ret_15: ArcStr = arcstr::literal!("");
            let mut ret_14: ArcStr = arcstr::literal!("");
            let mut ret_13: ArcStr = arcstr::literal!("");
            let mut l_numberOfEventIndicators: Tpl::Text;
            let mut l_variableNamingConvention: Tpl::Text;
            let mut ret_10: Util::DateTime = <Util::DateTime as ::std::default::Default>::default();
            let mut l_generationDateAndTime: Tpl::Text;
            let mut ret_8: ArcStr = arcstr::literal!("");
            let mut l_generationTool: Tpl::Text;
            let mut l_license: Tpl::Text;
            let mut l_copyright: Tpl::Text;
            let mut l_version: Tpl::Text;
            let mut l_author: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut l_modelName: Tpl::Text;
            let mut l_fmiVersion: Tpl::Text;
            l_fmiVersion = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("2.0")).clone() }))?;
            l_modelName = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_description = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_modelInfo_description.clone()).clone())?;
            l_author = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_modelInfo_author.clone()).clone())?;
            l_version = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_modelInfo_version.clone()).clone())?;
            l_copyright = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_modelInfo_copyright.clone()).clone())?;
            l_license = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_modelInfo_license.clone()).clone())?;
            l_generationTool = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OpenModelica Compiler ")).clone() }))?;
            ret_8 = (Settings::getVersionNr()).clone();
            l_generationTool = Tpl::writeStr(l_generationTool.clone(), (ret_8.clone()).clone())?;
            ret_10 = Util::getCurrentDateTime();
            l_generationDateAndTime = CodegenFMUCommon::xsdateTime(Tpl::emptyTxt.clone(), ret_10.clone())?;
            l_variableNamingConvention = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("structured")).clone() }))?;
            l_numberOfEventIndicators = CodegenFMUCommon::getNumberOfEventIndicators(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmiVersion=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmiVersion.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelName=\"")).clone()], lastHasNewLine: false }))?;
            ret_13 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_modelName.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_13.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("guid=\"{")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\"\n")).clone(), (literal!("description=\"")).clone()], lastHasNewLine: false }))?;
            ret_14 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_description.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_14.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("version=\"")).clone()], lastHasNewLine: false }))?;
            ret_15 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_version.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_15.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"\n")).clone() }))?;
            ret_16 = stringEq((Tpl::textString(l_author.clone())?).clone(), (literal!("")).clone());
            txt = fun_56(txt.clone(), ret_16.clone(), l_author.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_17 = stringEq((Tpl::textString(l_copyright.clone())?).clone(), (literal!("")).clone());
            txt = fun_57(txt.clone(), ret_17.clone(), l_copyright.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_18 = stringEq((Tpl::textString(l_license.clone())?).clone(), (literal!("")).clone());
            txt = fun_58(txt.clone(), ret_18.clone(), l_license.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("generationTool=\"")).clone() }))?;
            ret_19 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_generationTool.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_19.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("generationDateAndTime=\"")).clone()], lastHasNewLine: false }))?;
            ret_20 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_generationDateAndTime.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_20.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("variableNamingConvention=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_variableNamingConvention.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("numberOfEventIndicators=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfEventIndicators.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_60(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canGetAndSetFMUstate=\"false\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canGetAndSetFMUstate=\"true\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_61(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canSerializeFMUstate=\"false\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canSerializeFMUstate=\"true\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_62(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("providesDirectionalDerivative=\"false\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("providesDirectionalDerivative=\"true\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn CoSimulation(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_sourceFiles.clone())) {
        (txt, i_simCode @ SimCode::SimCode { modelInfo: _, .. }, a_sourceFiles) => {
            let mut ret_4: bool = false;
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_modelIdentifier: Tpl::Text;
            let mut txt = (*txt).clone();
            l_modelIdentifier = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<CoSimulation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelIdentifier=\"")).clone() }))?;
            ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_modelIdentifier.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("needsExecutionTool=\"false\"\n")).clone(), (literal!("canHandleVariableCommunicationStepSize=\"true\"\n")).clone(), (literal!("canInterpolateInputs=\"true\"\n")).clone(), (literal!("maxOutputDerivativeOrder=\"1\"\n")).clone(), (literal!("canRunAsynchronuously = \"false\"\n")).clone(), (literal!("canBeInstantiatedOnlyOncePerProcess=\"false\"\n")).clone(), (literal!("canNotUseMemoryManagementFunctions=\"false\"\n")).clone()], lastHasNewLine: true }))?;
            ret_2 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_60(txt.clone(), ret_2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_3 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_61(txt.clone(), ret_3.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_4 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_62(txt.clone(), ret_4.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = CodegenFMUCommon::SourceFiles(txt.clone(), a_sourceFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</CoSimulation>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

