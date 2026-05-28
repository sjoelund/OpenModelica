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
use crate::CodegenUtil;
use crate::CodegenUtilSimulation;
use crate::SimCode;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util::FMI;
use openmodelica_util::Settings;
use openmodelica_util::Util;

fn fun_52(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenFMUCommon::Implementation(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmiModelDescription(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr, mut in_a_FMUType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone(), in_a_FMUType.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { simulationSettingsOpt: ref i_simulationSettingsOpt, .. }, mut a_guid, mut a_FMUType) => {
            let mut ret_0: bool = false;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fmiModelDescription\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fmiModelDescriptionAttributes(txt.clone(), i_simCode.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = CodegenFMUCommon::fmiTypeDefinitions(txt.clone(), i_simCode.clone(), (literal!("1.0")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = CodegenFMUCommon::DefaultExperiment(txt.clone(), i_simulationSettingsOpt.clone(), (literal!("1.0")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = CodegenFMUCommon::fmiModelVariables(txt.clone(), i_simCode.clone(), (literal!("1.0")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_0 = FMI::isFMICSType((a_FMUType.clone()).clone());
            txt = fun_52(txt.clone(), ret_0.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fmiModelDescription>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_listStates: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_vi_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_listStates.clone(), in_a_vi_numStateVars.clone())) {
        (txt, false, _, a_vi_numStateVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(a_vi_numStateVars.clone())).clone())?;
            txt.clone()
        },
        (txt, _, a_listStates, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenFMUCommon::statesnumwithDummy(txt.clone(), a_listStates.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn fmiModelDescriptionAttributes(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { description: ref i_modelInfo_description, name: ref i_modelInfo_name, vars: SimCodeVar::SimVars { stateVars: ref i_listStates, .. }, varInfo: SimCode::VarInfo { numStateVars: ref i_vi_numStateVars, .. }, .. }, .. }, mut a_guid) => {
            let mut ret_15: ArcStr = arcstr::literal!("");
            let mut ret_14: ArcStr = arcstr::literal!("");
            let mut ret_13: ArcStr = arcstr::literal!("");
            let mut ret_12: ArcStr = arcstr::literal!("");
            let mut l_numberOfEventIndicators: Tpl::Text;
            let mut ret_10: bool = false;
            let mut l_numberOfContinuousStates: Tpl::Text;
            let mut l_variableNamingConvention: Tpl::Text;
            let mut ret_7: Util::DateTime = <Util::DateTime as ::std::default::Default>::default();
            let mut l_generationDateAndTime: Tpl::Text;
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut l_generationTool: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut l_modelIdentifier: Tpl::Text;
            let mut l_modelName: Tpl::Text;
            let mut l_fmiVersion: Tpl::Text;
            l_fmiVersion = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1.0")).clone() }))?;
            l_modelName = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_modelIdentifier = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            l_description = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_modelInfo_description.clone()).clone())?;
            l_generationTool = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OpenModelica Compiler ")).clone() }))?;
            ret_5 = (Settings::getVersionNr()).clone();
            l_generationTool = Tpl::writeStr(l_generationTool.clone(), (ret_5.clone()).clone())?;
            ret_7 = Util::getCurrentDateTime();
            l_generationDateAndTime = CodegenFMUCommon::xsdateTime(Tpl::emptyTxt.clone(), ret_7.clone())?;
            l_variableNamingConvention = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("structured")).clone() }))?;
            ret_10 = intEq(i_vi_numStateVars.clone(), 1);
            l_numberOfContinuousStates = fun_54(Tpl::emptyTxt.clone(), ret_10.clone(), i_listStates.clone(), i_vi_numStateVars.clone())?;
            l_numberOfEventIndicators = CodegenFMUCommon::getNumberOfEventIndicators(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmiVersion=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmiVersion.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelName=\"")).clone()], lastHasNewLine: false }))?;
            ret_12 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_modelName.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_12.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelIdentifier=\"")).clone()], lastHasNewLine: false }))?;
            ret_13 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_modelIdentifier.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_13.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("guid=\"{")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\"\n")).clone(), (literal!("description=\"")).clone()], lastHasNewLine: false }))?;
            ret_14 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_description.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_14.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("generationTool=\"")).clone()], lastHasNewLine: false }))?;
            ret_15 = (Util::escapeModelicaStringToXmlString((Tpl::textString(l_generationTool.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_15.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("generationDateAndTime=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_generationDateAndTime.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("variableNamingConvention=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_variableNamingConvention.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("numberOfContinuousStates=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfContinuousStates.clone())?;
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

