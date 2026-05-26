// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::BackendDAE;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeFunctionUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Algorithm;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn translateModel(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = generateXml(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_1 = dotPathXml(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".xml")).clone() }))?;
            Tpl::textFile(txt_0.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn generateXml(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { classAttributes: ref i_classAttributes, recordDecls: ref i_recordDecls, initialEquations: ref i_initialEquations, allEquations: ref i_allEquations, simulationSettingsOpt: ref i_simulationSettingsOpt, modelInfo: ref i_modelInfo @ SimCode::ModelInfo { functions: ref i_modelInfo_functions, .. }, .. }) => {
            let mut l_prefix: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_guid: Tpl::Text;
            ret_1 = (System::getUUIDStr()).clone();
            l_guid = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            l_prefix = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("https://github.com/JModelica/JModelica/tree/master/XML")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")).clone(), (literal!("<OpenModelicaModelDescription\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("xmlns:exp=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeExpressions.xsd\"\n")).clone(), (literal!("xmlns:equ=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeEquations.xsd\"\n")).clone(), (literal!("xmlns:fun=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeFunctions.xsd\"\n")).clone(), (literal!("xmlns:opt=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/daeOptimization.xsd\"\n")).clone(), (literal!("xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n")).clone()], lastHasNewLine: true }))?;
            txt = modelDescriptionXml(txt.clone(), i_simCode.clone(), (Tpl::textString(l_guid.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(">\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = vendorAnnotationsXml(txt.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = defaultExperiment(txt.clone(), i_simulationSettingsOpt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = modelVariablesXml(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = bindingEquationsXml(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = equationsXml(txt.clone(), i_allEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = initialEquationsXml(txt.clone(), i_modelInfo.clone(), i_initialEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = algorithmicEquationsXml(txt.clone(), i_allEquations.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = recordsXml(txt.clone(), i_recordDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = functionsXml(txt.clone(), i_modelInfo_functions.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = objectiveFunctionXml(txt.clone(), i_classAttributes.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</OpenModelicaModelDescription>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn vendorAnnotationsXml(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { varInfo: SimCode::VarInfo { numZeroCrossings: _, .. }, .. }, .. }) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_generationTool: Tpl::Text;
            l_generationTool = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OpenModelica Compiler ")).clone() }))?;
            ret_1 = (Settings::getVersionNr()).clone();
            l_generationTool = Tpl::writeStr(l_generationTool.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<VendorAnnotations>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Tool name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_generationTool.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"> </Tool>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</VendorAnnotations>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn modelDescriptionXml(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, varInfo: SimCode::VarInfo { numZeroCrossings: mut i_modelInfo_varInfo_numZeroCrossings, numStateVars: mut i_modelInfo_varInfo_numStateVars, .. }, .. }, .. }, mut a_guid) => {
            let mut l_numberOfEventIndicators: Tpl::Text;
            let mut l_numberOfContinuousStates: Tpl::Text;
            let mut l_variableNamingConvention: Tpl::Text;
            let mut ret_8: Util::DateTime;
            let mut l_generationDateAndTime: Tpl::Text;
            let mut l_version: Tpl::Text;
            let mut l_author: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_modelIdentifier: Tpl::Text;
            let mut l_modelName: Tpl::Text;
            let mut l_fmiVersion: Tpl::Text;
            l_fmiVersion = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1.0")).clone() }))?;
            l_modelName = dotPathXml(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            ret_3 = (System::stringReplace((i_fileNamePrefix.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_modelIdentifier = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            l_description = Tpl::emptyTxt.clone();
            l_author = Tpl::emptyTxt.clone();
            l_version = Tpl::emptyTxt.clone();
            ret_8 = Util::getCurrentDateTime();
            l_generationDateAndTime = xsdateTimeXml(Tpl::emptyTxt.clone(), ret_8.clone())?;
            l_variableNamingConvention = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("structured")).clone() }))?;
            l_numberOfContinuousStates = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_modelInfo_varInfo_numStateVars.clone())).clone())?;
            l_numberOfEventIndicators = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_modelInfo_varInfo_numZeroCrossings.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmiVersion=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmiVersion.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelName=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("modelIdentifier=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelIdentifier.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("guid=\"{")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\"\n")).clone(), (literal!("generationDateAndTime=\"")).clone()], lastHasNewLine: false }))?;
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

pub fn xsdateTimeXml(mut in_txt: Tpl::Text, mut in_a_dt: Util::DateTime) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_dt.clone()) {
        (mut txt, Util::DateTime { sec: mut i_sec, min: mut i_min, hour: mut i_hour, mday: mut i_mday, mon: mut i_mon, year: mut i_year }) => {
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (intString(i_year.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            ret_0 = (SimCodeFunctionUtil::twodigit(i_mon.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            ret_1 = (SimCodeFunctionUtil::twodigit(i_mday.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("T")).clone() }))?;
            ret_2 = (SimCodeFunctionUtil::twodigit(i_hour.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            ret_3 = (SimCodeFunctionUtil::twodigit(i_min.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            ret_4 = (SimCodeFunctionUtil::twodigit(i_sec.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn defaultExperiment(mut in_txt: Tpl::Text, mut in_a_simulationSettingsOpt: Option<SimCode::SimulationSettings>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simulationSettingsOpt.clone()) {
        (mut txt, Some(SimCode::SimulationSettings { tolerance: mut i_de_tolerance, stopTime: mut i_de_stopTime, startTime: mut i_de_startTime, .. })) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<DefaultExperiment startTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_de_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" stopTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_de_stopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" tolerance=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_de_tolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
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
fn lm_50(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_50(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_51(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_51(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_52(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_52(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_53(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_53(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_54(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_54(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_55(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_55(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_57(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_57(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_58(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_58(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_59(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_59(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_60(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_60(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_61(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_61(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_62(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_62(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_63(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_63(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_64(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_64(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_65(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_65(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_66(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_66(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_67(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_67(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_68(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_68(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_69(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_69(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_70(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_70(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn modelVariablesXml(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringConstVars: ref i_vars_stringConstVars, boolConstVars: ref i_vars_boolConstVars, intConstVars: ref i_vars_intConstVars, constVars: ref i_vars_constVars, extObjVars: ref i_vars_extObjVars, stringAliasVars: ref i_vars_stringAliasVars, stringParamVars: ref i_vars_stringParamVars, stringAlgVars: ref i_vars_stringAlgVars, boolParamVars: ref i_vars_boolParamVars, intParamVars: ref i_vars_intParamVars, paramVars: ref i_vars_paramVars, boolAliasVars: ref i_vars_boolAliasVars, intAliasVars: ref i_vars_intAliasVars, aliasVars: ref i_vars_aliasVars, outputVars: ref i_vars_outputVars, boolAlgVars: ref i_vars_boolAlgVars, intAlgVars: ref i_vars_intAlgVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ModelVariables>\n")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_50(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_51(txt.clone(), i_vars_derivativeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_52(txt.clone(), i_vars_algVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_53(txt.clone(), i_vars_discreteAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_54(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_55(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_56(txt.clone(), i_vars_outputVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_57(txt.clone(), i_vars_aliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_58(txt.clone(), i_vars_intAliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_59(txt.clone(), i_vars_boolAliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_60(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_61(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_62(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_63(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_64(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_65(txt.clone(), i_vars_stringAliasVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_66(txt.clone(), i_vars_extObjVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_67(txt.clone(), i_vars_constVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_68(txt.clone(), i_vars_intConstVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_69(txt.clone(), i_vars_boolConstVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_70(txt.clone(), i_vars_stringConstVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ModelVariables>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn ScalarVariableXml(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone()) {
        (mut txt, mut i_simVar @ SimCodeVar::SimVar { name: _, .. }) => {
            txt = ScalarVariableAttributesXml(txt.clone(), i_simVar.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_73(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_comment) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("description=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_comment.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableAttributesXml(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone()) {
        (mut txt, SimCodeVar::SimVar { isFixed: mut i_isFixed, initialValue: mut i_initialValue, maxValue: mut i_maxValue, minValue: mut i_minValue, displayUnit: mut i_displayUnit, unit: mut i_unit, type_: ref i_type__, name: ref i_name, causality: mut i_causality, aliasvar: mut i_aliasvar, comment: mut i_comment, varKind: mut i_varKind, .. }) => {
            let mut l_variableCategory: Tpl::Text;
            let mut l_caus: Tpl::Text;
            let mut l_alias: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut l_variability: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut l_valueReference: Tpl::Text;
            ret_1 = System::tmpTick();
            l_valueReference = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            l_variability = getVariablityXml(Tpl::emptyTxt.clone(), i_varKind.clone())?;
            l_description = fun_73(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            l_alias = getAliasVarXml(Tpl::emptyTxt.clone(), i_aliasvar.clone())?;
            l_caus = getCausalityXml(Tpl::emptyTxt.clone(), i_causality.clone())?;
            l_variableCategory = variableCategoryXml(Tpl::emptyTxt.clone(), i_varKind.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<ScalarVariable name=\"")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_description.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" valueReference=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_valueReference.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" variability=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_variability.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" causality=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_caus.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" alias=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_alias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ScalarVariableTypeXml(txt.clone(), i_type__.clone(), (i_unit.clone()).clone(), (i_displayUnit.clone()).clone(), i_minValue.clone(), i_maxValue.clone(), i_initialValue.clone(), i_isFixed.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<QualifiedName>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = qualifiedNamePartXml(txt.clone(), i_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</QualifiedName>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<isLinearTimedVariables>\n")).clone(), (literal!("  <TimePoint index=\"0\" isLinear=\"true\"/>\n")).clone(), (literal!("</isLinearTimedVariables>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<VariableCategory>")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_variableCategory.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</VariableCategory>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ScalarVariable>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getCausalityXml(mut in_txt: Tpl::Text, mut in_a_c: Option<SimCodeVar::Causality>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_c.clone()) {
        (mut txt, Some(SimCodeVar::Causality::NONECAUS)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("none")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::OUTPUT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::INPUT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::LOCAL)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("local")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::PARAMETER)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::CALCULATED_PARAMETER)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("calculatedParameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("internal")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getVariablityXml(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_varKind.clone()) {
        (mut txt, BackendDAE::VarKind::DISCRETE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::PARAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::CONST) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continuous")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getAliasVarXml(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone()) {
        (mut txt, SimCodeVar::AliasVariable::NOALIAS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("noAlias")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }) => {
            txt = crefStrXml(txt.clone(), i_varName.clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_varName.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn variableCategoryXml(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_varKind.clone()) {
        (mut txt, BackendDAE::VarKind::VARIABLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::STATE { index: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("state")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::STATE_DER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("derivative")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::DUMMY_DER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::DUMMY_STATE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::DISCRETE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algebraic")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::PARAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("independentParameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::CONST) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("independentConstant")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::EXTOBJ { fullClassName: ref i_fullClassName }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("externalObject_")).clone() }))?;
            txt = dotPathXml(txt.clone(), i_fullClassName.clone())?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::JAC_VAR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jacobianVar")).clone() }))?;
            txt.clone()
        },
        (mut txt, BackendDAE::VarKind::JAC_TMP_VAR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jacobianTmpVar")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 283, 14), (literal!("Unexpected simVarTypeName varKind")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn ScalarVariableTypeXml(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_unit: ArcStr, mut in_a_displayUnit: ArcStr, mut in_a_minValue: Option<Arc<DAE::Exp>>, mut in_a_maxValue: Option<Arc<DAE::Exp>>, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_unit.clone(), in_a_displayUnit.clone(), in_a_minValue.clone(), in_a_maxValue.clone(), in_a_initialValue.clone(), in_a_isFixed.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, _, _, a_minValue, a_maxValue, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Integer ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMinAttribute(txt.clone(), a_minValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMaxAttribute(txt.clone(), a_maxValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_unit, a_displayUnit, a_minValue, a_maxValue, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Real ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMinAttribute(txt.clone(), a_minValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeMaxAttribute(txt.clone(), a_maxValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = ScalarVariableTypeRealAttributeXml(txt.clone(), (a_unit.clone()).clone(), (a_displayUnit.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, _, _, _, _, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Boolean ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, _, _, _, _, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<String ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, _, _, _, _, a_initialValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Real ")).clone() }))?;
            txt = ScalarVariableTypeCommonAttributeXml(txt.clone(), a_initialValue.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKOWN_TYPE")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_isFixed: bool, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_isFixed.clone(), in_a_exp.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_isFixed, a_exp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = initValXml(txt.clone(), a_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" fixed=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_isFixed.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableTypeCommonAttributeXml(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_isFixed.clone())) {
        (txt, Some(i_exp), a_isFixed) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Expression::isEvaluatedConst(i_exp.clone());
            ret_1 = Expression::isCref(i_exp.clone());
            ret_2 = boolOr(ret_0.clone(), ret_1.clone());
            txt = fun_80(txt.clone(), ret_2.clone(), a_isFixed.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableTypeMinAttribute(mut in_txt: Tpl::Text, mut in_a_minValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_minValue.clone())) {
        (txt, Some(i_exp)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("min=\"")).clone() }))?;
            txt = initValXml(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableTypeMaxAttribute(mut in_txt: Tpl::Text, mut in_a_maxValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_maxValue.clone())) {
        (txt, Some(i_exp)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("max=\"")).clone() }))?;
            txt = initValXml(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
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

pub fn initValXml(mut in_txt: Tpl::Text, mut in_a_initialValue: Arc<DAE::Exp>) -> Result<Tpl::Text> {
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
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&quot;")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_string.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&quot;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }) => {
            let mut txt = (*txt).clone();
            txt = fun_84(txt.clone(), i_bool.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_componentRef.clone())?;
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

fn fun_86(mut in_txt: Tpl::Text, mut in_a_unit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_unit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_unit) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_unit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_a_displayUnit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_displayUnit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_displayUnit) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("displayUnit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_displayUnit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableTypeRealAttributeXml(mut txt: Tpl::Text, mut a_unit: ArcStr, mut a_displayUnit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_displayUnit__: Tpl::Text;
    let mut l_unit__: Tpl::Text;
    l_unit__ = fun_86(Tpl::emptyTxt.clone(), (a_unit.clone()).clone())?;
    l_displayUnit__ = fun_87(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_unit__.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_displayUnit__.clone())?;
    Ok(out_txt)
}

fn fun_89(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, a_cr) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = crefStrXml(Tpl::emptyTxt.clone(), a_cr.clone())?;
            ret_1 = (System::unquoteIdentifier((Tpl::textString(txt_0.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_cr) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_cr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn contextCrefXml(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_89(txt.clone(), a_context.clone(), a_cr.clone())?;
    Ok(out_txt)
}

fn fun_91(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_name.clone()) {
        (mut txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn contextIteratorNameXml(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_91(txt.clone(), a_context.clone(), (a_name.clone()).clone())?;
    Ok(out_txt)
}

pub fn crefXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, i_cr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "xloc", .. }) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:Time>time</exp:Time>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::WILD) => {
            txt.clone()
        },
        (txt, i_cr) => {
            let mut txt = (*txt).clone();
            txt = crefToXmlStr(txt.clone(), i_cr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_94(mut in_txt: Tpl::Text, mut in_a_arrayTest: Tpl::Text, mut in_a_subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arrayTest.clone(), in_a_subscriptLst.clone(), in_a_ident.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_subscriptLst, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = arraysubscriptsStrXml(txt.clone(), a_subscriptLst.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:QualifiedNamePart>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_95(mut in_txt: Tpl::Text, mut in_a_arrayTest: Tpl::Text, mut in_a_subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_componentRef: Arc<DAE::ComponentRef>, mut in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arrayTest.clone(), in_a_subscriptLst.clone(), in_a_componentRef.clone(), in_a_ident.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, a_componentRef, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = qualifiedNamePartXml(txt.clone(), a_componentRef.clone())?;
            txt.clone()
        },
        (txt, _, a_subscriptLst, a_componentRef, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = arraysubscriptsStrXml(txt.clone(), a_subscriptLst.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = qualifiedNamePartXml(txt.clone(), a_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:QualifiedNamePart>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn qualifiedNamePartXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, subscriptLst: i_subscriptLst, .. }) => {
            let mut l_arrayTest: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arrayTest = arraysubscriptsStrXml(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            txt = fun_94(txt.clone(), l_arrayTest.clone(), i_subscriptLst.clone(), (i_ident.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: Deref @ "$DER", .. }) => {
            let mut txt = (*txt).clone();
            txt = qualifiedNamePartXml(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: i_ident, subscriptLst: i_subscriptLst, .. }) => {
            let mut l_arrayTest: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arrayTest = arraysubscriptsStrXml(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            txt = fun_95(txt.clone(), l_arrayTest.clone(), i_subscriptLst.clone(), i_componentRef.clone(), (i_ident.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_97(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = arraysubscriptStrXml(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_97(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn arraysubscriptsStrXml(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:ArraySubscripts>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_97(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:ArraySubscripts>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn arraysubscriptStrXml(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<DAE::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:IndexExpression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:IntegerLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</exp:IntegerLiteral>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:IndexExpression>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:IndexExpression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:IntegerLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</exp:IntegerLiteral>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:IndexExpression>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WHOLEDIM")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN_SUBSCRIPT")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn crefToXmlStr(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, i_cr @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = qualifiedNamePartXml(txt.clone(), i_cr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: Deref @ "$DER", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Der>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefToXmlStr(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Der>")).clone() }))?;
            txt.clone()
        },
        (txt, i_cr @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = qualifiedNamePartXml(txt.clone(), i_cr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { instant: i_instant, componentRef: i_componentRef }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:TimedVariable timePointIndex = \"0\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefToXmlStr(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:Instant>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_instant.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</exp:Instant>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:TimedVariable>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::WILD) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefStrXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: i_subscriptLst, ident: i_ident, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = subscriptsStrXml(txt.clone(), i_subscriptLst.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: Deref @ "$DER", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der(")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: Deref @ "$PRE", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pre(")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, subscriptLst: i_subscriptLst, ident: i_ident, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = subscriptsStrXml(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_102(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, a_cr) => {
            let mut txt = (*txt).clone();
            txt = arrayCrefStrXml(txt.clone(), a_cr.clone())?;
            txt.clone()
        },
        (txt, _, a_cr) => {
            let mut txt = (*txt).clone();
            txt = arrayCrefXmlStr(txt.clone(), a_cr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn contextArrayCrefXml(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_102(txt.clone(), a_context.clone(), a_cr.clone())?;
    Ok(out_txt)
}

pub fn arrayCrefXmlStr(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = arrayCrefXmlStr2(txt.clone(), a_cr.clone())?;
    Ok(out_txt)
}

pub fn arrayCrefXmlStr2(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, .. }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:QualifiedName>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            ret_0 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\">")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, subscriptLst: i_subscriptLst, ident: i_ident, .. }) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_1 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = subscriptsStrXml(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$P")).clone() }))?;
            txt = arrayCrefXmlStr2(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("testing array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn arrayCrefStrXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamepart name =\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: i_ident, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = arrayCrefStrXml(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_NOT_IDENT_OR_QUAL")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_107(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = subscriptStrXml(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_107(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn subscriptsStrXml(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_107(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn subscriptStrXml(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<DAE::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ICONST { integer: i_i } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_i.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WHOLEDIM")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN_SUBSCRIPT")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn expCrefXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Der>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Der>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ERROR_NOT_A_CREF")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefFunctionNameXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, .. }) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            ret_1 = (System::stringReplace((ret_0.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: i_ident, .. }) => {
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_2 = (System::unquoteIdentifier((i_ident.clone()).clone())).clone();
            ret_3 = (System::stringReplace((ret_2.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = crefFunctionNameXml(txt.clone(), i_componentRef.clone())?;
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
pub fn dotPathXml(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::QUALIFIED { path: i_path, name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dotPathXml(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn replaceDotAndUnderscoreXml(mut in_txt: Tpl::Text, mut in_a_str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_str.clone()) {
        (mut txt, mut i_name) => {
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_str__underscores: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_str__dots: Tpl::Text;
            ret_1 = (System::stringReplace((i_name.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_str__dots = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            ret_3 = (System::stringReplace((Tpl::textString(l_str__dots.clone())?).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            l_str__underscores = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            ret_4 = (System::unquoteIdentifier((Tpl::textString(l_str__underscores.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn underscorePathXml(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::QUALIFIED { path: i_path, name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = underscorePathXml(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = underscorePathXml(txt.clone(), i_path.clone())?;
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
fn lm_115(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_115(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_116(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_116(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_117(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_117(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_118(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = bindingEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_118(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn bindingEquationsXml(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringParamVars: ref i_vars_stringParamVars, boolParamVars: ref i_vars_boolParamVars, intParamVars: ref i_vars_intParamVars, paramVars: ref i_vars_paramVars, .. }, varInfo: SimCode::VarInfo { numStateVars: _, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<equ:BindingEquations>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_115(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_116(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_117(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_118(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</equ:BindingEquations>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_120(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_varName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_varName.clone())) {
        (txt, Some(i_exp), a_varName) => {
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:BindingEquation>\n")).clone(), (literal!("  <equ:Parameter>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_varName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Parameter>\n")).clone(), (literal!("  <equ:BindingExp>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            (txt, l_preExp, l_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextOther().clone(), l_preExp.clone(), l_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:BindingExp>\n")).clone(), (literal!("</equ:BindingEquation>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn bindingEquationXml(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, SimCodeVar::SimVar { initialValue: mut i_initialValue, name: ref i_name, .. }) => {
            let mut l_varName: Tpl::Text;
            l_varName = qualifiedNamePartXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            txt = fun_120(txt.clone(), i_initialValue.clone(), l_varName.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_122(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_tmp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_tmp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_tmp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_tmp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_tmp, a_varDecls) => {
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_tmp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_tmp = (*a_tmp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_tmp) = equation_Xml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), a_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_tmp, a_varDecls) = lm_122(txt.clone(), rest.clone(), a_tmp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_tmp, out_a_varDecls))
}

pub fn equationsXml(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_eqs: Tpl::Text;
    let mut l_tmp: Tpl::Text;
    let mut l_jens: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    System::tmpTickReset(0);
    l_jens = Tpl::emptyTxt.clone();
    l_tmp = Tpl::emptyTxt.clone();
    l_eqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_eqs, l_tmp, l_varDecls) = lm_122(l_eqs.clone(), a_allEquationsPlusWhen.clone(), l_tmp.clone(), l_varDecls.clone())?;
    l_eqs = Tpl::popIter(l_eqs.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<equ:DynamicEquations>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_tmp.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_eqs.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</equ:DynamicEquations>")).clone() }))?;
    Ok(out_txt)
}

fn lm_124(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationAlgorithmXml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_124(txt.clone(), rest.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algorithmicEquationsXml(mut txt: Tpl::Text, mut a_allEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_algs: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    l_algs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_algs, l_varDecls) = lm_124(l_algs.clone(), a_allEquations.clone(), l_varDecls.clone())?;
    l_algs = Tpl::popIter(l_algs.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Algorithm>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_algs.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Algorithm>")).clone() }))?;
    Ok(out_txt)
}

fn lm_126(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), SimCodeFunction::contextFunction().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_126(txt.clone(), rest.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_127(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: i_statements, .. }, a_varDecls) => {
            let mut l_alg: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_alg = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_alg, a_varDecls) = lm_126(l_alg.clone(), i_statements.clone(), a_varDecls.clone())?;
            l_alg = Tpl::popIter(l_alg.clone())?;
            txt = Tpl::writeText(txt.clone(), l_alg.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn equationAlgorithmXml(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = fun_127(txt.clone(), a_eq.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn lm_129(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_tmp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_tmp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_tmp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_tmp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_tmp, a_varDecls) => {
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_tmp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_tmp = (*a_tmp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_tmp) = equation_Xml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), a_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_tmp, a_varDecls) = lm_129(txt.clone(), rest.clone(), a_tmp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_tmp, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_130(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_130(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_131(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_131(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_132(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_132(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_133(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_133(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_134(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_134(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_135(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_135(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_136(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initialEquationXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_136(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn initialEquationsXml(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_initialEqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelInfo.clone(), in_a_initialEqs.clone())) {
        (txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringAlgVars: i_vars_stringAlgVars, boolAlgVars: i_vars_boolAlgVars, intAlgVars: i_vars_intAlgVars, discreteAlgVars: i_vars_discreteAlgVars, algVars: i_vars_algVars, derivativeVars: i_vars_derivativeVars, stateVars: i_vars_stateVars, .. }, varInfo: SimCode::VarInfo { numStateVars: _, .. }, .. }, a_initialEqs) => {
            let mut l_eqs: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_jens: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            System::tmpTickReset(0);
            l_jens = Tpl::emptyTxt.clone();
            l_tmp = Tpl::emptyTxt.clone();
            l_eqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_eqs, l_tmp, l_varDecls) = lm_129(l_eqs.clone(), a_initialEqs.clone(), l_tmp.clone(), l_varDecls.clone())?;
            l_eqs = Tpl::popIter(l_eqs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<equ:InitialEquations>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_130(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_131(txt.clone(), i_vars_derivativeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_132(txt.clone(), i_vars_algVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_133(txt.clone(), i_vars_discreteAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_134(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_135(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_136(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</equ:InitialEquations>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_138(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_identName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_identName.clone())) {
        (txt, Some(i_exp), a_identName) => {
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:Equation>\n")).clone(), (literal!("  <exp:Sub>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_identName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            (txt, l_preExp, l_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextOther().clone(), l_preExp.clone(), l_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Sub>\n")).clone(), (literal!("</equ:Equation>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initialEquationXml(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, SimCodeVar::SimVar { initialValue: mut i_initialValue, name: ref i_name, .. }) => {
            let mut l_identName: Tpl::Text;
            l_identName = crefXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            txt = fun_138(txt.clone(), i_initialValue.clone(), l_identName.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_140(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_varD: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varD: Tpl::Text;
    (out_txt, out_a_varD) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_varD.clone(), in_a_context.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationArrayCallAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index: _, .. }, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("IfEquation Assign Not implemente yet")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationLinearXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: _, .. }, a_varD, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varD = (*a_varD).clone();
            (txt, a_varD) = equationNonlinearXml(txt.clone(), i_e.clone(), a_context.clone(), a_varD.clone())?;
            (txt.clone(), a_varD.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: _, .. }, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        (txt, _, a_varD, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT IMPLEMENTED EQUATION")).clone() }))?;
            (txt.clone(), a_varD.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varD))
}

pub fn equation_Xml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text, mut in_a_eqs: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_eqs: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_eqs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone(), in_a_eqs.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_MIXED { index: _, .. }, _, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" MIXED EQUATION NOT IMPLEMENTED ")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::List::Nil, .. }, _, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: _, .. }, _, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_context, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationWhenXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, i_eq, a_context, a_varDecls, a_eqs) => {
            let mut l_x: Tpl::Text;
            let mut l_varD: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut l_ix: Tpl::Text;
            let mut a_eqs = (*a_eqs).clone();
            ret_1 = System::tmpTickIndex(10);
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            l_tmp = Tpl::emptyTxt.clone();
            l_varD = Tpl::emptyTxt.clone();
            (l_x, l_varD) = fun_140(Tpl::emptyTxt.clone(), i_eq.clone(), l_varD.clone(), a_context.clone())?;
            a_eqs = Tpl::writeTok(a_eqs.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:Equation>\n")).clone(), (literal!("  <exp:Sub>\n")).clone()], lastHasNewLine: true }))?;
            a_eqs = Tpl::pushBlock(a_eqs.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_eqs = Tpl::writeText(a_eqs.clone(), l_x.clone())?;
            a_eqs = Tpl::softNewLine(a_eqs.clone())?;
            a_eqs = Tpl::popBlock(a_eqs.clone())?;
            a_eqs = Tpl::writeTok(a_eqs.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Sub>\n")).clone(), (literal!("</equ:Equation>")).clone()], lastHasNewLine: false }))?;
            a_eqs = Tpl::writeTok(a_eqs.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_eqs))
}

pub fn old_equation_Xml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_MIXED { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationSimpleAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationArrayCallAssignXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: _, .. }, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: _, .. }, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" equations are not implemented yet")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: _, .. }, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equations are not implemented yet ")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = equationWhenXml(txt.clone(), i_e.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT IMPLEMENTED EQUATION")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_143(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_expPart: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_expPart.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_144(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_expPart: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_expPart.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationSimpleAssignXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: i_cref, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_result: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            l_result = fun_143(Tpl::emptyTxt.clone(), l_preExp.clone(), l_expPart.clone())?;
            txt = crefXml(txt.clone(), i_cref.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_result.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cref: i_cref, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_result: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            l_result = fun_144(Tpl::emptyTxt.clone(), l_preExp.clone(), l_expPart.clone())?;
            txt = crefXml(txt.clone(), i_cref.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_result.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_146(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_eqn_exp: Arc<DAE::Exp>, mut in_a_lhs_componentRef: Arc<DAE::ComponentRef>, mut in_a_expPart: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_eqn_exp.clone(), in_a_lhs_componentRef.clone(), in_a_expPart.clone())) {
        (txt, Deref @ "boolean", _, a_lhs_componentRef, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = crefXml(txt.clone(), a_lhs_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ "integer", _, a_lhs_componentRef, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = crefXml(txt.clone(), a_lhs_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ "real", _, a_lhs_componentRef, a_expPart) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_lhs_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_expPart.clone())?;
            txt.clone()
        },
        (txt, _, a_eqn_exp, _, _) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("No runtime support for this sort of array call: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), a_eqn_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 850, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationArrayCallAssignXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { exp: i_eqn_exp @ i_exp, lhs: Deref @ DAE::Exp::CREF { componentRef: i_lhs_componentRef, .. }, .. }, a_context, a_varDecls) => {
            let mut str_3: ArcStr = arcstr::literal!("");
            let mut txt_2: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt_2 = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), i_eqn_exp.clone())?;
            str_3 = (Tpl::textString(txt_2.clone())?).clone();
            txt = fun_146(txt.clone(), (str_3.clone()).clone(), i_eqn_exp.clone(), i_lhs_componentRef.clone(), l_expPart.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_148(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: (_, _, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_eq_exp, .. }), tail: rest }, a_varDecls, a_context) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_eq_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_148(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = lm_148(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_149(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }, a_varDecls, a_context) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 3 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_149(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn equationLinearXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { beqs: i_ls_beqs, simJac: i_ls_simJac, .. }, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_varDecls) = lm_148(txt.clone(), i_ls_simJac.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_varDecls) = lm_149(txt.clone(), i_ls_beqs.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_151(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_tmp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_tmp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_tmp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_tmp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_tmp, a_varDecls) => {
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq2, tail: rest }, a_tmp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_tmp = (*a_tmp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_tmp) = functionExtraResidualsPreBody(txt.clone(), i_eq2.clone(), a_varDecls.clone(), a_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_tmp, a_varDecls) = lm_151(txt.clone(), rest.clone(), a_tmp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_tmp.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_tmp, out_a_varDecls))
}

fn lm_152(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_eq2_exp, .. }, tail: rest }, a_varDecls) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_eq2_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_152(txt.clone(), rest.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = lm_152(txt.clone(), rest.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_153(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { eqs: i_nls_eqs, .. }, .. }) => {
            let mut l_body: Tpl::Text;
            let mut l_prebody: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_tmp = Tpl::emptyTxt.clone();
            l_prebody = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_prebody, l_tmp, l_varDecls) = lm_151(l_prebody.clone(), i_nls_eqs.clone(), l_tmp.clone(), l_varDecls.clone())?;
            l_prebody = Tpl::popIter(l_prebody.clone())?;
            l_body = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_body, l_varDecls) = lm_152(l_body.clone(), i_nls_eqs.clone(), l_varDecls.clone())?;
            l_body = Tpl::popIter(l_body.clone())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_prebody.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationNonlinearXml(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_153(txt.clone(), a_eq.clone())?;
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_varDecls))
}

pub fn functionExtraResidualsPreBody(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_varDecls: Tpl::Text, mut in_a_eqs: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_eqs: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_eqs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_varDecls.clone(), in_a_eqs.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }, a_varDecls, a_eqs) => {
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        (txt, i_eq, a_varDecls, a_eqs) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_eqs = (*a_eqs).clone();
            (txt, a_varDecls, a_eqs) = equation_Xml(txt.clone(), i_eq.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), a_eqs.clone())?;
            (txt.clone(), a_varDecls.clone(), a_eqs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_eqs))
}

fn lm_156(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut in_a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_helpInits: Tpl::Text;
    (out_txt, out_a_helpInits) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_helpInits.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_helpInits) => {
            (txt.clone(), a_helpInits.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_helpInits) => {
            let mut l_helpInit: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_helpInits) = lm_156(txt.clone(), rest.clone(), a_helpInits.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_helpInits))
}

fn fun_157(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_158(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut in_a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_helpInits: Tpl::Text;
    (out_txt, out_a_helpInits) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_helpInits.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_helpInits) => {
            (txt.clone(), a_helpInits.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_helpInits) => {
            let mut l_helpInit: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_helpInits) = lm_158(txt.clone(), rest.clone(), a_helpInits.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_helpInits))
}

fn fun_159(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationWhenXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { elseWhen: None, conditions: i_conditions, whenStmtLst: i_whenStmtLst, .. }, a_context, a_varDecls) => {
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut l_helpInits: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_helpInits = Tpl::emptyTxt.clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_helpIf, l_helpInits) = lm_156(l_helpIf.clone(), i_conditions.clone(), l_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            l_cond = fun_157(Tpl::emptyTxt.clone(), l_preExp.clone(), l_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:When>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:When>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { elseWhen: Some(i_elseWhenEq), conditions: i_conditions, whenStmtLst: i_whenStmtLst, .. }, a_context, a_varDecls) => {
            let mut l_elseWhen: Tpl::Text;
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut l_helpInits: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_helpInits = Tpl::emptyTxt.clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_helpIf, l_helpInits) = lm_158(l_helpIf.clone(), i_conditions.clone(), l_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            (l_elseWhen, l_preExp, l_helpInits, a_varDecls) = equationElseWhenXml(Tpl::emptyTxt.clone(), i_elseWhenEq.clone(), a_context.clone(), l_preExp.clone(), l_helpInits.clone(), a_varDecls.clone())?;
            l_cond = fun_159(Tpl::emptyTxt.clone(), l_preExp.clone(), l_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:When>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:When>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_elseWhen.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_161(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut in_a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_helpInits: Tpl::Text;
    (out_txt, out_a_helpInits) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_helpInits.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_helpInits) => {
            (txt.clone(), a_helpInits.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_helpInits) => {
            let mut l_helpInit: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_helpInits) = lm_161(txt.clone(), rest.clone(), a_helpInits.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_helpInits))
}

fn fun_162(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_163(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut in_a_helpInits: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_helpInits: Tpl::Text;
    (out_txt, out_a_helpInits) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_helpInits.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_helpInits) => {
            (txt.clone(), a_helpInits.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_helpInits) => {
            let mut l_helpInit: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            l_helpInit = crefToXmlStr(Tpl::emptyTxt.clone(), i_e.clone())?;
            a_helpInits = Tpl::writeText(a_helpInits.clone(), l_helpInit.clone())?;
            a_helpInits = Tpl::writeTok(a_helpInits.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_helpInits) = lm_163(txt.clone(), rest.clone(), a_helpInits.clone())?;
            (txt.clone(), a_helpInits.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_helpInits))
}

fn fun_164(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_helpInits.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_helpInits) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_helpInits.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationElseWhenXml(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_helpInits: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_helpInits: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_helpInits, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_helpInits.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { elseWhen: None, conditions: i_conditions, whenStmtLst: i_whenStmtLst, .. }, a_context, a_preExp, a_helpInits, a_varDecls) => {
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_helpIf, a_helpInits) = lm_161(l_helpIf.clone(), i_conditions.clone(), a_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            l_cond = fun_162(Tpl::emptyTxt.clone(), a_preExp.clone(), a_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:ElseWhen>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:ElseWhen>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { elseWhen: Some(i_elseWhenEq), conditions: i_conditions, whenStmtLst: i_whenStmtLst, .. }, a_context, a_preExp, a_helpInits, a_varDecls) => {
            let mut l_elseWhen: Tpl::Text;
            let mut l_cond: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_helpIf: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_helpInits = (*a_helpInits).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_helpIf = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_helpIf, a_helpInits) = lm_163(l_helpIf.clone(), i_conditions.clone(), a_helpInits.clone())?;
            l_helpIf = Tpl::popIter(l_helpIf.clone())?;
            (l_body, a_varDecls) = whenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone(), a_context.clone(), a_varDecls.clone())?;
            (l_elseWhen, a_preExp, a_helpInits, a_varDecls) = equationElseWhenXml(Tpl::emptyTxt.clone(), i_elseWhenEq.clone(), a_context.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())?;
            l_cond = fun_164(Tpl::emptyTxt.clone(), a_preExp.clone(), a_helpInits.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<equ:ElseWhen>\n")).clone(), (literal!("  <equ:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Condition>\n")).clone(), (literal!("  <equ:Equation>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </equ:Equation>\n")).clone(), (literal!("</equ:ElseWhen>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_elseWhen.clone())?;
            (txt.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_helpInits, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_helpInits.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_helpInits, out_a_varDecls))
}

fn fun_166(mut in_txt: Tpl::Text, mut in_a_whenOp: BackendDAE::WhenOperator, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_whenOp.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, BackendDAE::WhenOperator::ASSIGN { right: i_right, left: Deref @ DAE::Exp::CREF { componentRef: i_cr, .. }, .. }, a_varDecls, a_context) => {
            let mut l_exp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_exp, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_right.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:Sub>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = crefXml(txt.clone(), i_cr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </exp:Sub>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::REINIT { stateVar: i_stateVar, value: i_value, .. }, a_varDecls, _) => {
            let mut l_val: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_val, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_value.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Reinit>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_stateVar.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_val.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Reinit>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::TERMINATE { message: i_message, .. }, a_varDecls, _) => {
            let mut l_msgVar: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_msgVar, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_message.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_msgVar.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::ASSERT { message: i_message, condition: i_condition, source: Deref @ DAE::ElementSource { info: i_info, .. }, .. }, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = assertCommonXml(txt.clone(), i_condition.clone(), i_message.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone(), i_info.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_167(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_whenOp, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = fun_166(txt.clone(), i_whenOp.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_167(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn whenOps(mut txt: Tpl::Text, mut a_whenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_body: Tpl::Text;
    l_body = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_body, out_a_varDecls) = lm_167(l_body.clone(), a_whenOps.clone(), a_varDecls.clone(), a_context.clone())?;
    l_body = Tpl::popIter(l_body.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_body.clone())?;
    Ok((out_txt, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_169(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_rd, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = recordDeclarationXml(txt.clone(), i_rd.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_169(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn recordsXml(mut txt: Tpl::Text, mut a_recordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:RecordsList>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_169(out_txt.clone(), a_recordDecls.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:RecordsList>")).clone() }))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_171(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = recordBodyXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_171(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn recordDeclarationXml(mut in_txt: Tpl::Text, mut in_a_recDecl: SimCodeFunction::RecordDeclaration) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_recDecl.clone()) {
        (mut txt, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { variables: ref i_variables, name: mut i_name, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Record>\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart  name ='")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("'/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Name>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_171(txt.clone(), i_variables.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Record>")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  Record Declaration definition is not yet implemented")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn recordBodyXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Field  type=\"Record\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("  <fun:Record>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Record>\n")).clone(), (literal!("</fun:Field>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_var_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:Field  type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = crefStrXml(txt.clone(), i_var_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:Field>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
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
fn lm_174(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = functionXml(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_174(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn functionsXml(mut txt: Tpl::Text, mut a_functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:FunctionsList>\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_174(out_txt.clone(), a_functions.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:FunctionsList>")).clone() }))?;
    Ok(out_txt)
}

pub fn functionXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, i_fn @ Deref @ SimCodeFunction::Function::FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = regularFunctionXml(txt.clone(), i_fn.clone())?;
            txt.clone()
        },
        (txt, i_fn @ Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = externalFunctionXml(txt.clone(), i_fn.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: _, .. }) => {
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
fn lm_177(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = funOutputVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_177(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_178(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = funArgDefinitionXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_178(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn regularFunctionXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::FUNCTION { functionArguments: i_functionArguments, outVars: i_outVars, body: i_body, name: i_name, .. }) => {
            let mut l_bodyPart: Tpl::Text;
            let mut l_varInits: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut l_fname: Tpl::Text;
            let mut txt = (*txt).clone();
            System::tmpTickReset(1);
            l_fname = underscorePathXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_varDecls = Tpl::emptyTxt.clone();
            l_varInits = Tpl::emptyTxt.clone();
            (l_bodyPart, l_varDecls) = funStatementXml(Tpl::emptyTxt.clone(), i_body.clone(), l_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Function>\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Name>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_177(txt.clone(), i_outVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_178(txt.clone(), i_functionArguments.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_bodyPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Function>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
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
fn lm_180(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = funOutputVariableXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_180(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_181(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Variable::Variable>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = funArgDefinitionXml(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_181(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn externalFunctionXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, i_efn @ Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { funArgs: i_funArgs, outVars: i_outVars, name: i_name, .. }) => {
            let mut l_callPart: Tpl::Text;
            let mut l_fname: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            System::tmpTickReset(1);
            l_preExp = Tpl::emptyTxt.clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_fname = underscorePathXml(Tpl::emptyTxt.clone(), i_name.clone())?;
            (l_callPart, l_preExp, l_varDecls) = extFunCallXml(Tpl::emptyTxt.clone(), i_efn.clone(), l_preExp.clone(), l_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Function>\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Name>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_180(txt.clone(), i_outVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_181(txt.clone(), i_funArgs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_callPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Algorithm>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Function>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn funArgNameXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn funOutputVariableXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:OutputVariable type=\"Record\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("  <fun:Record>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Record>\n")).clone(), (literal!("</fun:OutputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:OutputVariable type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:OutputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn funArgDefinitionXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:InputVariable type=\"Record\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("  <fun:Record>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Record>\n")).clone(), (literal!("</fun:InputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:InputVariable type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:InputVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn funVarDeclarationsXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, i_var @ Deref @ SimCodeFunction::Variable::VARIABLE { name: i_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:protectedVariable type=\"")).clone() }))?;
            txt = varTypeXml(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <fun:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = contextCrefXml(txt.clone(), i_name.clone(), SimCodeFunction::contextFunction().clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Name>\n")).clone(), (literal!("</fun:ProtectedVariable>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: i_name_1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_187(mut in_txt: Tpl::Text, mut in_a_language: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_language.clone(), in_a_name.clone())) {
        (txt, Deref @ "C", a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedName name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "FORTRAN 77", a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedName name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt.clone()
        },
        (txt, i_language, _) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unsupport external language: ")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (i_language.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1250, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extFunctionNameXml(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_language: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_187(txt.clone(), (a_language.clone()).clone(), (a_name.clone()).clone())?;
    Ok(out_txt)
}

fn fun_189(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>, mut in_a_isInput: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone(), in_a_isInput.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("double")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("const char*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }, a_isInput) => {
            let mut txt = (*txt).clone();
            txt = extTypeXml(txt.clone(), i_ty.clone(), a_isInput.clone(), true)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void *")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: i_rname }, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct ")).clone() }))?;
            txt = underscorePathXml(txt.clone(), i_rname.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, i_type, _) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unknown external C type ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_type.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1268, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_190(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "const char*") => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("const ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_191(mut in_txt: Tpl::Text, mut in_a_isArray: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isArray.clone(), in_a_s.clone()) {
        (mut txt, false, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_s) => {
            let mut str_0: ArcStr = arcstr::literal!("");
            str_0 = (Tpl::textString(a_s.clone())?).clone();
            txt = fun_190(txt.clone(), (str_0.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_192(mut in_txt: Tpl::Text, mut in_a_isInput: bool, mut in_a_isArray: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isInput.clone(), in_a_isArray.clone(), in_a_s.clone()) {
        (mut txt, false, _, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_isArray, mut a_s) => {
            txt = fun_191(txt.clone(), a_isArray.clone(), a_s.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_193(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>, mut in_a_isArray: bool, mut in_a_isInput: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone(), in_a_isArray.clone(), in_a_isInput.clone(), in_a_s.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, _, _, a_s) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (txt, _, a_isArray, a_isInput, a_s) => {
            let mut txt = (*txt).clone();
            txt = fun_192(txt.clone(), a_isInput.clone(), a_isArray.clone(), a_s.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extTypeXml(mut txt: Tpl::Text, mut a_type: Arc<DAE::Type>, mut a_isInput: bool, mut a_isArray: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_s: Tpl::Text;
    l_s = fun_189(Tpl::emptyTxt.clone(), a_type.clone(), a_isInput.clone())?;
    out_txt = fun_193(txt.clone(), a_type.clone(), a_isArray.clone(), a_isInput.clone(), l_s.clone())?;
    Ok(out_txt)
}

fn fun_195(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("double")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("char")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = extTypeF77Xml(txt.clone(), i_ty.clone(), true)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: i_rname }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct ")).clone() }))?;
            txt = underscorePathXml(txt.clone(), i_rname.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void*")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void*")).clone() }))?;
            txt.clone()
        },
        (txt, i_type) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unknown external F77 type ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_type.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1287, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_196(mut in_txt: Tpl::Text, mut in_a_isReference: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isReference.clone(), in_a_s.clone()) {
        (mut txt, false, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_s) => {
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_197(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>, mut in_a_isReference: bool, mut in_a_s: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone(), in_a_isReference.clone(), in_a_s.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, _, a_s) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_s.clone())?;
            txt.clone()
        },
        (txt, _, a_isReference, a_s) => {
            let mut txt = (*txt).clone();
            txt = fun_196(txt.clone(), a_isReference.clone(), a_s.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extTypeF77Xml(mut txt: Tpl::Text, mut a_type: Arc<DAE::Type>, mut a_isReference: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_s: Tpl::Text;
    l_s = fun_195(Tpl::emptyTxt.clone(), a_type.clone())?;
    out_txt = fun_197(txt.clone(), a_type.clone(), a_isReference.clone(), l_s.clone())?;
    Ok(out_txt)
}

fn fun_199(mut in_txt: Tpl::Text, mut in_a_dotPath: bool, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dotPath.clone(), in_a_name.clone())) {
        (txt, false, a_name) => {
            let mut txt = (*txt).clone();
            txt = underscorePathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_200(mut in_txt: Tpl::Text, mut in_a_dotPath: bool, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dotPath.clone(), in_a_name.clone())) {
        (txt, false, a_name) => {
            let mut txt = (*txt).clone();
            txt = underscorePathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_201(mut in_txt: Tpl::Text, mut in_a_dotPath: bool, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dotPath.clone(), in_a_name.clone())) {
        (txt, false, a_name) => {
            let mut txt = (*txt).clone();
            txt = underscorePathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn functionNameXml(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>, mut in_a_dotPath: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone(), in_a_dotPath.clone())) {
        (txt, Deref @ SimCodeFunction::Function::FUNCTION { name: i_name, .. }, a_dotPath) => {
            let mut txt = (*txt).clone();
            txt = fun_199(txt.clone(), a_dotPath.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { name: i_name, .. }, a_dotPath) => {
            let mut txt = (*txt).clone();
            txt = fun_200(txt.clone(), a_dotPath.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: i_name, .. }, a_dotPath) => {
            let mut txt = (*txt).clone();
            txt = fun_201(txt.clone(), a_dotPath.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extVarNameXml(mut txt: Tpl::Text, mut a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = crefXml(txt.clone(), a_cr.clone())?;
    Ok(out_txt)
}

fn fun_204(mut in_txt: Tpl::Text, mut in_a_language: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_language.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_fun.clone())) {
        (txt, Deref @ "C", a_varDecls, a_preExp, a_fun) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = extFunCallCXml(txt.clone(), a_fun.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ "FORTRAN 77", a_varDecls, a_preExp, a_fun) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = extFunCallF77Xml(txt.clone(), a_fun.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn extFunCallXml(mut in_txt: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fun.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_fun @ Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: i_language, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_204(txt.clone(), (i_language.clone()).clone(), a_varDecls.clone(), a_preExp.clone(), i_fun.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_206(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }, a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = extArgCXml(txt.clone(), i_arg.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_206(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_207(mut in_txt: Tpl::Text, mut in_a_extReturn: Arc<SimCodeFunction::SimExtArg::SimExtArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extReturn.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, .. }) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extFunCallCXml(mut in_txt: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fun.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { extName: i_extName, extReturn: i_extReturn, extArgs: i_extArgs, .. }, a_preExp, a_varDecls) => {
            let mut l_returnAssign: Tpl::Text;
            let mut l_args: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_args = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_args, a_varDecls, a_preExp) = lm_206(l_args.clone(), i_extArgs.clone(), a_varDecls.clone(), a_preExp.clone())?;
            l_args = Tpl::popIter(l_args.clone())?;
            l_returnAssign = fun_207(Tpl::emptyTxt.clone(), i_extReturn.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_returnAssign.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Expression>\n")).clone(), (literal!("  <exp:FunctionCall>\n")).clone(), (literal!("    <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_extName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\" />\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Name>\n")).clone(), (literal!("    <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_args.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Arguments>\n")).clone(), (literal!("  </exp:FunctionCall>\n")).clone(), (literal!("</fun:Expression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_209(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::SimExtArg::SimExtArg>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }, a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = extArgF77Xml(txt.clone(), i_arg.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_209(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_210(mut in_txt: Tpl::Text, mut in_a_extReturn: Arc<SimCodeFunction::SimExtArg::SimExtArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extReturn.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref: i_c, .. }) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn extFunCallF77Xml(mut in_txt: Tpl::Text, mut in_a_fun: Arc<SimCodeFunction::Function::Function>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fun.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { extName: i_extName, extReturn: i_extReturn, extArgs: i_extArgs, .. }, a_preExp, a_varDecls) => {
            let mut l_returnAssign: Tpl::Text;
            let mut l_args: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_args = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_args, a_varDecls, a_preExp) = lm_209(l_args.clone(), i_extArgs.clone(), a_varDecls.clone(), a_preExp.clone())?;
            l_args = Tpl::popIter(l_args.clone())?;
            l_returnAssign = fun_210(Tpl::emptyTxt.clone(), i_extReturn.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_returnAssign.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Expression>\n")).clone(), (literal!("  <exp:FunctionCall>\n")).clone(), (literal!("    <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_extName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\" />\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Name>\n")).clone(), (literal!("    <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_args.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:Arguments>\n")).clone(), (literal!("  </exp:FunctionCall>\n")).clone(), (literal!("</fun:Expression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn extArgCXml(mut in_txt: Tpl::Text, mut in_a_extArg: Arc<SimCodeFunction::SimExtArg::SimExtArg>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extArg.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { type_: _, isArray: true, outputIndex: _, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { type_: _, outputIndex: 0, isInput: _, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { type_: _, outputIndex: _, isInput: _, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { exp: i_exp, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExternalXmlExp(txt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("test daeexternal xml")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGSIZE { exp: i_exp, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut l_dim: Tpl::Text;
            let mut l_name: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_name = extVarNameXml(Tpl::emptyTxt.clone(), i_c.clone())?;
            (l_dim, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Size>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Size>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn extArgF77Xml(mut in_txt: Tpl::Text, mut in_a_extArg: Arc<SimCodeFunction::SimExtArg::SimExtArg>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extArg.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { type_: _, isArray: true, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { type_: Deref @ DAE::Type::T_INTEGER { varLst: _ }, outputIndex: _, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { type_: Deref @ DAE::Type::T_STRING { varLst: _ }, outputIndex: _, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { type_: _, outputIndex: _, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = extVarNameXml(txt.clone(), i_c.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGEXP { type_: Deref @ DAE::Type::T_STRING { varLst: _ }, exp: i_exp }, a_preExp, a_varDecls) => {
            let mut l_texp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_texp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_texp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ SimCodeFunction::SimExtArg::SIMEXTARGSIZE { exp: i_exp, cref: i_c, .. }, a_preExp, a_varDecls) => {
            let mut l_name: Tpl::Text;
            let mut l_dim: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_dim, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), SimCodeFunction::contextFunction().clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_name = extVarNameXml(Tpl::emptyTxt.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Size>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Size>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_214(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_classAttribute, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = classAttributesXml(txt.clone(), i_classAttribute.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_214(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn objectiveFunctionXml(mut txt: Tpl::Text, mut a_classAttributes: Arc<metamodelica::List<Arc<DAE::ClassAttributes>>>, mut a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_214(out_txt.clone(), a_classAttributes.clone(), a_simCode.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_216(mut in_txt: Tpl::Text, mut in_a_objetiveE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_objetiveE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ObjectiveFunction>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ObjectiveFunction>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_217(mut in_txt: Tpl::Text, mut in_a_objectiveIntegrandE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_objectiveIntegrandE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:IntegrandObjectiveFunction>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:IntegrandObjectiveFunction>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_218(mut in_txt: Tpl::Text, mut in_a_startTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_startTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:IntervalStartTime>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<opt:Value>")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</opt:Value>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:IntervalStartTime>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_219(mut in_txt: Tpl::Text, mut in_a_finalTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_finalTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:IntervalFinalTime>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<opt:Value>")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</opt:Value>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:IntervalFinalTime>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_220(mut in_txt: Tpl::Text, mut in_a_startTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_startTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("index = \"")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_221(mut in_txt: Tpl::Text, mut in_a_finalTimeE: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_finalTimeE.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Some(i_exp), a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("value = \"")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpValueXml(txt.clone(), i_exp.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_222(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { constraints: ref i_constraints, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }) => {
            txt = constraintsXml(txt.clone(), i_constraints.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn classAttributesXml(mut in_txt: Tpl::Text, mut in_a_classAttribute: Arc<DAE::ClassAttributes>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classAttribute.clone(), in_a_simCode.clone())) {
        (txt, Deref @ DAE::ClassAttributes { finalTimeE: i_finalTimeE, startTimeE: i_startTimeE, objectiveIntegrandE: i_objectiveIntegrandE, objetiveE: i_objetiveE }, a_simCode) => {
            let mut l_constraints: Tpl::Text;
            let mut l_timePointValue: Tpl::Text;
            let mut l_timePointIndex: Tpl::Text;
            let mut l_finalTime: Tpl::Text;
            let mut l_startTime: Tpl::Text;
            let mut l_objectiveIntegrand: Tpl::Text;
            let mut l_objectiveFunction: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_objectiveFunction, l_varDecls, l_preExp) = fun_216(Tpl::emptyTxt.clone(), i_objetiveE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_objectiveIntegrand, l_varDecls, l_preExp) = fun_217(Tpl::emptyTxt.clone(), i_objectiveIntegrandE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_startTime, l_varDecls, l_preExp) = fun_218(Tpl::emptyTxt.clone(), i_startTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_finalTime, l_varDecls, l_preExp) = fun_219(Tpl::emptyTxt.clone(), i_finalTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_timePointIndex, l_varDecls, l_preExp) = fun_220(Tpl::emptyTxt.clone(), i_startTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            (l_timePointValue, l_varDecls, l_preExp) = fun_221(Tpl::emptyTxt.clone(), i_finalTimeE.clone(), l_varDecls.clone(), l_preExp.clone())?;
            l_constraints = fun_222(Tpl::emptyTxt.clone(), a_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:Optimization>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_objectiveFunction.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_objectiveIntegrand.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_startTime.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_finalTime.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:TimePoints>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<opt:TimePoint ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_timePointIndex.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_timePointValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(">\n")).clone(), (literal!("</opt:TimePoint>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</opt:TimePoints>\n")).clone(), (literal!("<opt:PathConstraints>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_constraints.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</opt:PathConstraints>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:Optimization>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1509, 16), (literal!("Unknown Optimization attribute")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_224(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_constraint, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = constraintXml(txt.clone(), i_constraint.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_224(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn constraintsXml(mut txt: Tpl::Text, mut a_constraints: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_224(out_txt.clone(), a_constraints.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn lm_226(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_constraint, tail: rest }, a_varDecls, a_preExp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpConstraintXml(txt.clone(), i_constraint.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_226(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn constraintXml(mut in_txt: Tpl::Text, mut in_a_cons: Arc<DAE::Constraint>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cons.clone())) {
        (txt, Deref @ DAE::Constraint::CONSTRAINT_EXPS { constraintLst: i_constraintLst }) => {
            let mut l_constrain: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_constrain = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_constrain, l_varDecls, l_preExp) = lm_226(l_constrain.clone(), i_constraintLst.clone(), l_varDecls.clone(), l_preExp.clone())?;
            l_constrain = Tpl::popIter(l_constrain.clone())?;
            txt = Tpl::writeText(txt.clone(), l_constrain.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1532, 16), (literal!("Unknown Constraint List")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_228(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), SimCodeFunction::contextFunction().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_228(txt.clone(), rest.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn funStatementXml(mut txt: Tpl::Text, mut a_statementLst: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (out_txt, out_a_varDecls) = lm_228(out_txt.clone(), a_statementLst.clone(), a_varDecls.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_230(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, i_s @ Deref @ DAE::Statement::STMT_ASSIGN { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtAssignXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtAssignArrXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtTupleAssignXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_IF { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtIfXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_FOR { type_: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtForXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_WHILE { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtWhileXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_ASSERT { cond: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtAssertXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_TERMINATE { msg: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtTerminateXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_WHEN { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtWhenXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_BREAK { source: _ }, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:Break/>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_RETURN { source: _ }, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:Return/>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_NORETCALL { exp: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtNoretcallXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_REINIT { var: _, .. }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtReinitXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1562, 14), (literal!("ALG_STATEMENT NYI")).clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStatementXml(mut txt: Tpl::Text, mut a_stmt: Arc<DAE::Statement>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_res: Tpl::Text;
    (l_res, out_a_varDecls) = fun_230(Tpl::emptyTxt.clone(), a_stmt.clone(), a_varDecls.clone(), a_context.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_res.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_232(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_val: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_val.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: i_idx, tail: Deref @ metamodelica::List::Nil }, exp: i_arr }, a_val, a_varDecls, a_context) => {
            let mut l_val1: Tpl::Text;
            let mut l_idx1: Tpl::Text;
            let mut l_arr1: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_arr1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_arr.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_idx1, l_preExp, a_varDecls) = daeSubscriptXML(Tpl::emptyTxt.clone(), i_idx.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_val1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_val.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_arr1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_idx1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_val1.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_233(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_exp1: Arc<DAE::Exp>, mut in_a_val: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_exp1.clone(), in_a_val.clone(), in_a_varDecls.clone(), in_a_context.clone(), in_a_exp.clone())) {
        (txt, Deref @ "metatype", _, a_val, a_varDecls, a_context, a_exp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = fun_232(txt.clone(), a_exp.clone(), a_val.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_exp1, a_val, a_varDecls, a_context, _) => {
            let mut l_expPart: Tpl::Text;
            let mut l_varPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = daeExpAsubXml(Tpl::emptyTxt.clone(), a_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_val.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtAssignXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp: i_e, exp1: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, .. }, a_context, a_varDecls) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Assign>\n")).clone(), (literal!("  <fun:Expression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Expression>\n")).clone(), (literal!("</fun:Assign>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp: i_exp, exp1: i_exp1 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }, .. }, .. }, a_context, a_varDecls) => {
            let mut l_varPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp: i_exp, exp1: i_exp1 @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: _, .. }, .. }, .. }, a_context, a_varDecls) => {
            let mut l_varPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp: i_exp, exp1: i_exp1 @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, .. }, a_context, a_varDecls) => {
            let mut l_varPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_varPart, l_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_varPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp: i_exp @ i_val, exp1: i_exp1 @ Deref @ DAE::Exp::ASUB { exp: _, .. }, .. }, a_context, a_varDecls) => {
            let mut str_4: ArcStr = arcstr::literal!("");
            let mut txt_3: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_3 = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), i_exp.clone())?;
            str_4 = (Tpl::textString(txt_3.clone())?).clone();
            (txt, a_varDecls) = fun_233(txt.clone(), (str_4.clone()).clone(), i_exp1.clone(), i_val.clone(), a_varDecls.clone(), a_context.clone(), i_exp.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { exp: i_exp, exp1: i_exp1, .. }, a_context, a_varDecls) => {
            let mut l_expPart2: Tpl::Text;
            let mut l_expPart1: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart2, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_235(mut in_txt: Tpl::Text, mut in_a_ispec: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_expPart: Tpl::Text, mut in_a_t: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ispec.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_expPart.clone(), in_a_t.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_varDecls, a_preExp, a_context, a_cr, a_expPart, a_t) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Assign>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = copyArrayDataXml(txt.clone(), a_t.clone(), (Tpl::textString(a_expPart.clone())?).clone(), a_cr.clone(), a_context.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Expression>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Expression>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Assign>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_ispec, a_varDecls, a_preExp, a_context, a_cr, a_expPart, a_t) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            (txt, a_varDecls) = indexedAssignXml(txt.clone(), a_t.clone(), (Tpl::textString(a_expPart.clone())?).clone(), a_cr.clone(), (Tpl::textString(i_ispec.clone())?).clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtAssignArrXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: i_t, lhs: Deref @ DAE::Exp::CREF { componentRef: i_cr, .. }, exp: i_e, .. }, a_context, a_varDecls) => {
            let mut l_ispec: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_ispec, l_preExp, a_varDecls) = indexSpecFromCrefXml(Tpl::emptyTxt.clone(), i_cr.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls) = fun_235(txt.clone(), l_ispec.clone(), a_varDecls.clone(), l_preExp.clone(), a_context.clone(), i_cr.clone(), l_expPart.clone(), i_t.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_237(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ispec: ArcStr, mut in_a_exp: ArcStr, mut in_a_cref: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_ispec.clone(), in_a_exp.clone(), in_a_cref.clone()) {
        (mut txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, _, mut a_cref) => {
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_ispec, mut a_exp, mut a_cref) => {
            txt = Tpl::writeStr(txt.clone(), (a_exp.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_ispec.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn indexedAssignXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_exp: ArcStr, mut a_cr: Arc<DAE::ComponentRef>, mut a_ispec: ArcStr, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_cref: Tpl::Text;
    let mut l_type: Tpl::Text;
    l_type = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
    l_cref = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
    out_txt = fun_237(txt.clone(), a_context.clone(), (a_ispec.clone()).clone(), (a_exp.clone()).clone(), l_cref.clone())?;
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_varDecls))
}

fn fun_239(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cref: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_cref.clone()) {
        (mut txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, mut a_cref) => {
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_cref) => {
            txt = Tpl::writeText(txt.clone(), a_cref.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn copyArrayDataXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_exp: ArcStr, mut a_cr: Arc<DAE::ComponentRef>, mut a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_cref: Tpl::Text;
    let mut l_type: Tpl::Text;
    l_type = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
    l_cref = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
    out_txt = fun_239(txt.clone(), a_context.clone(), l_cref.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_241(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_e.clone(), (literal!("\"")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_241(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_242(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_retStruct: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_context.clone(), in_a_retStruct.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_afterExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cr, tail: rest }, a_varDecls, a_afterExp, a_context, a_retStruct) => {
            let mut x_i1: i32 = 0;
            let mut l_rhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_rhsStr = Tpl::writeText(Tpl::emptyTxt.clone(), a_retStruct.clone())?;
            l_rhsStr = Tpl::writeTok(l_rhsStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".targ")).clone() }))?;
            l_rhsStr = Tpl::writeStr(l_rhsStr.clone(), (intString(x_i1.clone())).clone())?;
            (txt, a_afterExp, a_varDecls) = writeLhsCrefXml(txt.clone(), i_cr.clone(), (Tpl::textString(l_rhsStr.clone())?).clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_afterExp) = lm_242(txt.clone(), rest.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone(), a_retStruct.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn lm_243(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_prefix: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_context.clone(), in_a_prefix.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_afterExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cr, tail: rest }, a_varDecls, a_afterExp, a_context, a_prefix) => {
            let mut x_i1: i32 = 0;
            let mut l_rhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_rhsStr = Tpl::writeText(Tpl::emptyTxt.clone(), a_prefix.clone())?;
            l_rhsStr = Tpl::writeTok(l_rhsStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_targ")).clone() }))?;
            l_rhsStr = Tpl::writeStr(l_rhsStr.clone(), (intString(x_i1.clone())).clone())?;
            (txt, a_afterExp, a_varDecls) = writeLhsCrefXml(txt.clone(), i_cr.clone(), (Tpl::textString(l_rhsStr.clone())?).clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_afterExp) = lm_243(txt.clone(), rest.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone(), a_prefix.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn fun_244(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "modelica_metatype") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = NULL")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_245(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_rhsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_rhsStr.clone())) {
        (txt, Deref @ "modelica_metatype", a_rhsStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_GC_add_root(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_rhsStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", mmc_GC_local_state, \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_rhsStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_246(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_prefix: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_prefix.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cr, tail: rest }, a_varDecls, a_prefix) => {
            let mut x_i1: i32 = 0;
            let mut str_5: ArcStr = arcstr::literal!("");
            let mut l_addRoot: Tpl::Text;
            let mut str_3: ArcStr = arcstr::literal!("");
            let mut l_initVar: Tpl::Text;
            let mut l_typ: Tpl::Text;
            let mut l_rhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_rhsStr = Tpl::writeText(Tpl::emptyTxt.clone(), a_prefix.clone())?;
            l_rhsStr = Tpl::writeTok(l_rhsStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_targ")).clone() }))?;
            l_rhsStr = Tpl::writeStr(l_rhsStr.clone(), (intString(x_i1.clone())).clone())?;
            l_typ = expTypeFromExpModelicaXml(Tpl::emptyTxt.clone(), i_cr.clone())?;
            str_3 = (Tpl::textString(l_typ.clone())?).clone();
            l_initVar = fun_244(Tpl::emptyTxt.clone(), (str_3.clone()).clone())?;
            str_5 = (Tpl::textString(l_typ.clone())?).clone();
            l_addRoot = fun_245(Tpl::emptyTxt.clone(), (str_5.clone()).clone(), l_rhsStr.clone())?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_typ.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_rhsStr.clone())?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_initVar.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_addRoot.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_246(txt.clone(), rest.clone(), a_varDecls.clone(), a_prefix.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtTupleAssignXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: i_expExpLst, exp: i_exp @ Deref @ DAE::Exp::CALL { path: _, .. }, .. }, a_context, a_varDecls) => {
            let mut l_lhsCrefs: Tpl::Text;
            let mut l_retStruct: Tpl::Text;
            let mut l_marker: Tpl::Text;
            let mut l_crefs: Tpl::Text;
            let mut l_afterExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_afterExp = Tpl::emptyTxt.clone();
            l_crefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_crefs = lm_241(l_crefs.clone(), i_expExpLst.clone())?;
            l_crefs = Tpl::popIter(l_crefs.clone())?;
            l_marker = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            l_marker = Tpl::writeText(l_marker.clone(), l_crefs.clone())?;
            l_marker = Tpl::writeTok(l_marker.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") = ")).clone() }))?;
            l_marker = ExpressionDumpTpl::dumpExp(l_marker.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            l_preExp = Tpl::writeTok(l_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* algStmtTupleAssign: preExp buffer created for ")).clone() }))?;
            l_preExp = Tpl::writeText(l_preExp.clone(), l_marker.clone())?;
            l_preExp = Tpl::writeTok(l_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            l_preExp = Tpl::writeTok(l_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_afterExp = Tpl::writeTok(l_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* algStmtTupleAssign: afterExp buffer created for ")).clone() }))?;
            l_afterExp = Tpl::writeText(l_afterExp.clone(), l_marker.clone())?;
            l_afterExp = Tpl::writeTok(l_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            l_afterExp = Tpl::writeTok(l_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (l_retStruct, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            l_lhsCrefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_lhsCrefs, a_varDecls, l_afterExp) = lm_242(l_lhsCrefs.clone(), i_expExpLst.clone(), a_varDecls.clone(), l_afterExp.clone(), a_context.clone(), l_retStruct.clone())?;
            l_lhsCrefs = Tpl::popIter(l_lhsCrefs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:FunctionCallStatement>\n")).clone(), (literal!("  <fun:OutputArgument>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_lhsCrefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:OutputArgument>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_retStruct.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:FunctionCallStatement>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: i_expExpLst, exp: Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. }, .. }, a_context, a_varDecls) => {
            let mut ret_7: i32 = 0;
            let mut l_prefix: Tpl::Text;
            let mut l_lhsCrefs: Tpl::Text;
            let mut l_afterExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_afterExp = Tpl::emptyTxt.clone();
            l_prefix = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            ret_7 = System::tmpTick();
            l_prefix = Tpl::writeStr(l_prefix.clone(), (intString(ret_7.clone())).clone())?;
            l_lhsCrefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_lhsCrefs, a_varDecls, l_afterExp) = lm_243(l_lhsCrefs.clone(), i_expExpLst.clone(), a_varDecls.clone(), l_afterExp.clone(), a_context.clone(), l_prefix.clone())?;
            l_lhsCrefs = Tpl::popIter(l_lhsCrefs.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_varDecls) = lm_246(txt.clone(), i_expExpLst.clone(), a_varDecls.clone(), l_prefix.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhsCrefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_afterExp.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 1755, 12), (literal!("algStmtTupleAssign failed")).clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_248(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_lhsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_lhsStr.clone()) {
        (mut txt, SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: _ }, mut a_lhsStr) => {
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_lhsStr) => {
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_249(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_lhsStr: Tpl::Text, mut in_a_rhsStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_context.clone(), in_a_lhsStr.clone(), in_a_rhsStr.clone()) {
        (mut txt, SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: _ }, mut a_lhsStr, mut a_rhsStr) => {
            txt = Tpl::writeStr(txt.clone(), (a_rhsStr.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_lhsStr, _) => {
            txt = Tpl::writeText(txt.clone(), a_lhsStr.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn writeLhsCrefXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_rhsStr: ArcStr, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_rhsStr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<fun:EmptyOutputArgument></fun:EmptyOutputArgument>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. }, _, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_248(txt.clone(), a_context.clone(), l_lhsStr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::UNARY { exp: i_e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. }, .. }, a_rhsStr, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_249(txt.clone(), a_context.clone(), l_lhsStr.clone(), (a_rhsStr.clone()).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, _, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhsStr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::UNARY { exp: i_e @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, .. }, _, a_context, a_preExp, a_varDecls) => {
            let mut l_lhsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_lhsStr, a_preExp, a_varDecls) = scalarLhsCrefXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhsStr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_rhsStr, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* SimCodeC.tpl template: writeLhsCref: UNHANDLED LHS\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* ")).clone() }))?;
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_rhsStr.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_251(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_251(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtIfXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_IF { else_: i_else__, statementLst: i_statementLst, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_condExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_condExp, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:If>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_condExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_varDecls) = lm_251(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Statements>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            (txt, a_varDecls) = elseExprXml(txt.clone(), i_else__.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:If>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtForXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, i_s @ Deref @ DAE::Statement::STMT_FOR { range: Deref @ DAE::Exp::RANGE { ty: _, .. }, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtForRangeXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_s @ Deref @ DAE::Statement::STMT_FOR { type_: _, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStmtForGenericXml(txt.clone(), i_s.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_254(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_254(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtForRangeXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_FOR { iter: i_iter, statementLst: i_statementLst, iterIsArray: i_iterIsArray, type_: i_type__, range: i_rng @ Deref @ DAE::Exp::RANGE { ty: _, .. }, .. }, a_context, a_varDecls) => {
            let mut l_stmtStr: Tpl::Text;
            let mut l_identTypeShort: Tpl::Text;
            let mut l_identType: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_identType = expTypeXml(Tpl::emptyTxt.clone(), i_type__.clone(), i_iterIsArray.clone())?;
            l_identTypeShort = expTypeShortXml(Tpl::emptyTxt.clone(), i_type__.clone())?;
            l_stmtStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_stmtStr, a_varDecls) = lm_254(l_stmtStr.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            l_stmtStr = Tpl::popIter(l_stmtStr.clone())?;
            (txt, l_stmtStr, a_varDecls) = algStmtForRange_implXml(txt.clone(), i_rng.clone(), (i_iter.clone()).clone(), (Tpl::textString(l_identType.clone())?).clone(), (Tpl::textString(l_identTypeShort.clone())?).clone(), l_stmtStr.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_256(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_step.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Some(i_eo), a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_eo.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_257(mut in_txt: Tpl::Text, mut in_a_range: Arc<DAE::Exp>, mut in_a_iterator: ArcStr, mut in_a_body: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_range.clone(), in_a_iterator.clone(), in_a_body.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RANGE { stop: i_stop, step: i_step, start: i_start, .. }, a_iterator, a_body, a_context, a_varDecls) => {
            let mut l_stopValue: Tpl::Text;
            let mut l_stepValue: Tpl::Text;
            let mut l_startValue: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_iterName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_iterName = contextIteratorNameXml(Tpl::emptyTxt.clone(), (a_iterator.clone()).clone(), a_context.clone())?;
            l_preExp = Tpl::emptyTxt.clone();
            (l_startValue, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_start.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_stepValue, a_varDecls, l_preExp) = fun_256(Tpl::emptyTxt.clone(), i_step.clone(), a_varDecls.clone(), l_preExp.clone(), a_context.clone())?;
            (l_stopValue, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_stop.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:For>\n")).clone(), (literal!("  <fun:Index>\n")).clone(), (literal!("    <fun:IterationVariable>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iterName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </fun:IterationVariable>\n")).clone(), (literal!("    <fun:IterationSet>\n")).clone(), (literal!("      <exp:Range>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
            txt = Tpl::writeText(txt.clone(), l_startValue.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_stepValue.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_stopValue.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("      </exp:Range>\n")).clone(), (literal!("    </fun:IterationSet>\n")).clone(), (literal!("  </fun:Index>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("</fun:For>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtForRange_implXml(mut txt: Tpl::Text, mut a_range: Arc<DAE::Exp>, mut a_iterator: ArcStr, mut a_type: ArcStr, mut a_shortType: ArcStr, mut a_body: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_body: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = fun_257(txt.clone(), a_range.clone(), (a_iterator.clone()).clone(), a_body.clone(), a_context.clone(), a_varDecls.clone())?;
    out_a_body = a_body.clone();
    Ok((out_txt, out_a_body, out_a_varDecls))
}

fn lm_259(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_259(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtForGenericXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_FOR { iter: i_iter, range: i_range, statementLst: i_statementLst, iterIsArray: i_iterIsArray, type_: i_type__, .. }, a_context, a_varDecls) => {
            let mut l_stmtStr: Tpl::Text;
            let mut l_arrayType: Tpl::Text;
            let mut l_iterType: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_iterType = expTypeXml(Tpl::emptyTxt.clone(), i_type__.clone(), i_iterIsArray.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), i_type__.clone())?;
            l_stmtStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_stmtStr, a_varDecls) = lm_259(l_stmtStr.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            l_stmtStr = Tpl::popIter(l_stmtStr.clone())?;
            (txt, l_stmtStr, a_varDecls) = algStmtForGeneric_implXml(txt.clone(), i_range.clone(), (i_iter.clone()).clone(), (Tpl::textString(l_iterType.clone())?).clone(), (Tpl::textString(l_arrayType.clone())?).clone(), i_iterIsArray.clone(), l_stmtStr.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtForGeneric_implXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_iterator: ArcStr, mut a_type: ArcStr, mut a_arrayType: ArcStr, mut a_iterIsArray: bool, mut a_body: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_body: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_evar: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    let mut l_iterName: Tpl::Text;
    l_iterName = contextIteratorNameXml(Tpl::emptyTxt.clone(), (a_iterator.clone()).clone(), a_context.clone())?;
    l_preExp = Tpl::emptyTxt.clone();
    (l_evar, l_preExp, out_a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:For>\n")).clone(), (literal!("  <fun:Index>\n")).clone(), (literal!("    <fun:IterationVariable>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_iterName.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"/>\n")).clone() }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </fun:IterationVariable>\n")).clone(), (literal!("    <fun:IterationSet>\n")).clone(), (literal!("      <exp:Array>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_preExp.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("      </exp:Array>\n")).clone(), (literal!("    </fun:IterationSet>\n")).clone(), (literal!("  </fun:Index>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), a_body.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("</fun:For>")).clone()], lastHasNewLine: false }))?;
    out_a_body = a_body.clone();
    Ok((out_txt, out_a_body, out_a_varDecls))
}

fn lm_262(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_262(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtWhileXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_WHILE { statementLst: i_statementLst, exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_var: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_var, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:While>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_varDecls) = lm_262(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("</fun:While>")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtAssertXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSERT { msg: i_msg, cond: i_cond, source: Deref @ DAE::ElementSource { info: i_info, .. }, .. }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = assertCommonXml(txt.clone(), i_cond.clone(), i_msg.clone(), a_context.clone(), a_varDecls.clone(), i_info.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtTerminateXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_TERMINATE { msg: i_msg, .. }, a_context, a_varDecls) => {
            let mut l_msgVar: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_msgVar, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_msg.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_msgVar.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtNoretcallXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_NORETCALL { exp: i_exp, .. }, a_context, a_varDecls) => {
            let mut l_expPart: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_267(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = crefToXmlStr(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_267(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_268(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_268(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtWhenXml(mut in_txt: Tpl::Text, mut in_a_when: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_when.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_WHEN { elseWhen: i_elseWhen, statementLst: i_statementLst, conditions: i_conditions, .. }, a_context, a_varDecls) => {
            let mut l_else: Tpl::Text;
            let mut l_statements: Tpl::Text;
            let mut l_cond: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_cond = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_cond = lm_267(l_cond.clone(), i_conditions.clone())?;
            l_cond = Tpl::popIter(l_cond.clone())?;
            l_statements = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_statements, a_varDecls) = lm_268(l_statements.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            l_statements = Tpl::popIter(l_statements.clone())?;
            (l_else, a_varDecls) = algStatementWhenElseXml(Tpl::emptyTxt.clone(), i_elseWhen.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:When>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_cond.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_statements.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Statements>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_else.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_270(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_270(txt.clone(), rest.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_271(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = crefToXmlStr(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_271(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn algStatementWhenElseXml(mut in_txt: Tpl::Text, mut in_a_stmt: Option<Arc<DAE::Statement>>, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_varDecls.clone())) {
        (txt, Some(Deref @ DAE::Statement::STMT_WHEN { conditions: i_when_conditions, elseWhen: i_when_elseWhen, statementLst: i_when_statementLst, .. }), a_varDecls) => {
            let mut l_elseCondStr: Tpl::Text;
            let mut l_else: Tpl::Text;
            let mut l_statements: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_statements = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_statements, a_varDecls) = lm_270(l_statements.clone(), i_when_statementLst.clone(), a_varDecls.clone())?;
            l_statements = Tpl::popIter(l_statements.clone())?;
            (l_else, a_varDecls) = algStatementWhenElseXml(Tpl::emptyTxt.clone(), i_when_elseWhen.clone(), a_varDecls.clone())?;
            l_elseCondStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elseCondStr = lm_271(l_elseCondStr.clone(), i_when_conditions.clone())?;
            l_elseCondStr = Tpl::popIter(l_elseCondStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Condition>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_elseCondStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</fun:Condition>\n")).clone(), (literal!("<fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_statements.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:Statements>\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:When>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn algStmtReinitXml(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Statement::STMT_REINIT { value: i_value, var: i_var, .. }, a_context, a_varDecls) => {
            let mut l_expPart2: Tpl::Text;
            let mut l_expPart1: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_expPart1, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_var.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            (l_expPart2, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_value.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Reinit>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_expPart2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Reinit>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn indexSpecFromCrefXml(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: i_subs @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhsIndexSpecXml(txt.clone(), i_subs.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_275(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_275(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn lm_276(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = algStatementXml(txt.clone(), i_stmt.clone(), a_context.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls) = lm_276(txt.clone(), rest.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn elseExprXml(mut in_txt: Tpl::Text, mut in_a_else__: Arc<DAE::Else>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Else::NOELSE, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Else::ELSEIF { else_: i_else__, statementLst: i_statementLst, exp: i_exp }, a_context, a_varDecls) => {
            let mut l_condExp: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_condExp, l_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:ElseIf>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_condExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  </fun:Condition>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_varDecls) = lm_275(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</fun:ElseIf>\n")).clone() }))?;
            (txt, a_varDecls) = elseExprXml(txt.clone(), i_else__.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Else::ELSE { statementLst: i_statementLst }, a_context, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<fun:Else>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_varDecls) = lm_276(txt.clone(), i_statementLst.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</fun:Else>")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_varDecls) => {
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_278(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr_componentRef.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_ecr.clone())) {
        (txt, false, _, a_varDecls, a_preExp, a_context, a_ecr) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefLhsXml(txt.clone(), a_ecr.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_ecr_componentRef, a_varDecls, a_preExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_ecr_componentRef.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn scalarLhsCrefXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }, componentRef: i_cr }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_ecr @ Deref @ DAE::Exp::CREF { componentRef: i_ecr_componentRef @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, .. }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_0 = SimCodeFunctionUtil::crefNoSub(i_ecr_componentRef.clone())?;
            (txt, a_varDecls, a_preExp) = fun_278(txt.clone(), ret_0.clone(), i_ecr_componentRef.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_ecr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_ecr_componentRef @ Deref @ DAE::ComponentRef::CREF_QUAL { ident: _, .. }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_ecr_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD, .. }, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ONLY_IDENT_OR_QUAL_CREF_SUPPORTED_SLHS")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_280(mut in_txt: Tpl::Text, mut in_a_e: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_e.clone(), in_a_preExp.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_preExp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt.clone()
        },
        (txt, i_e, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_e.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_281(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_preExp: Tpl::Text, mut in_a_eStr1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_preExp.clone(), in_a_eStr1.clone()) {
        (mut txt, false, _, mut a_eStr1) => {
            txt = Tpl::writeText(txt.clone(), a_eStr1.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_preExp, _) => {
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn daeExpXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_4: bool = false;
    let mut ret_3: i32 = 0;
    let mut l_eStr2: Tpl::Text;
    let mut l_eStr1: Tpl::Text;
    let mut l_e: Tpl::Text;
    (l_e, out_a_preExp, out_a_varDecls) = daeExpXml_dispatch(Tpl::emptyTxt.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    l_eStr1 = fun_280(Tpl::emptyTxt.clone(), l_e.clone(), out_a_preExp.clone())?;
    ret_3 = System::stringFind((Tpl::textString(l_eStr1.clone())?).clone(), (literal!("tmp")).clone())?;
    ret_4 = intEq(0, ret_3.clone());
    l_eStr2 = fun_281(Tpl::emptyTxt.clone(), ret_4.clone(), out_a_preExp.clone(), l_eStr1.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_eStr2.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_283(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
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

pub fn daeExpXml_dispatch(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:IntegerLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:IntegerLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:RealLiteral>")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:RealLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:StringLiteral>")).clone() }))?;
            (txt, a_preExp, a_varDecls) = daeExpSconstXml(txt.clone(), (i_string.clone()).clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:StringLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:BooleanLiteral>")).clone() }))?;
            txt = fun_283(txt.clone(), i_bool.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:BooleanLiteral>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::CREF { componentRef: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhsXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::BINARY { exp1: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpBinaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::UNARY { operator: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpUnaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::LBINARY { exp1: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpLbinaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::LUNARY { operator: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpLunaryXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::RELATION { exp1: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpRelationXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::IFEXP { expCond: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpIfXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::CALL { path: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCallXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::ARRAY { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpArrayXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::MATRIX { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpMatrixXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::RANGE { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpRangeXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::CAST { ty: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCastXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::ASUB { exp: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpAsubXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::TSUB { exp: i_exp, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::SIZE { exp: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpSizeXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: _ }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tuple Not yet Implemented")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::BOX { exp: _ }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpBoxXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::UNBOX { exp: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpUnboxXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_e @ Deref @ DAE::Exp::SHARED_LITERAL { index: _, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpSharedLiteralXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone());
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, _, a_preExp, a_varDecls) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unknown expression: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2141, 14), (Tpl::textString(txt_0.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_285(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
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
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn daeExpValueXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_285(txt.clone(), a_exp.clone())?;
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_287(mut in_txt: Tpl::Text, mut in_mArg: Arc<DAE::Type>, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, a_exp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExternalXmlExp(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_0: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    ret_0 = Expression::r#typeof(a_exp.clone())?;
    (out_txt, out_a_preExp, out_a_varDecls) = fun_287(txt.clone(), ret_0.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpSconstXml(mut txt: Tpl::Text, mut a_string: ArcStr, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_0: ArcStr = arcstr::literal!("");
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    ret_0 = (Util::escapeModelicaStringToXmlString((a_string.clone()).clone())?).clone();
    out_txt = Tpl::writeStr(out_txt.clone(), (ret_0.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_290(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_t: Arc<DAE::Type>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_t.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_exp.clone())) {
        (txt, i_context @ SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, _, a_varDecls, a_preExp, a_exp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhs2Xml(txt.clone(), a_exp.clone(), i_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context, a_cr, a_t, a_varDecls, a_preExp, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpRecordCrefRhsXml(txt.clone(), a_t.clone(), a_cr.clone(), i_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpCrefRhsXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_exp @ Deref @ DAE::Exp::CREF { ty: i_t @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: i_cr }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_290(txt.clone(), a_context.clone(), i_cr.clone(), i_t.clone(), a_varDecls.clone(), a_preExp.clone(), i_exp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: _, .. }, componentRef: i_cr }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefFunctionNameXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }, componentRef: i_cr }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCrefRhs2Xml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_292(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: i_exp }, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_292(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = lm_292(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn lm_293(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: i_exp }, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_293(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = lm_293(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_294(mut in_txt: Tpl::Text, mut in_mArg: Arc<DAE::Type>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_arrName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_arrName.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { dims: i_et_dims, .. }, _, a_varDecls, a_preExp, a_context, a_cr, a_arrName) => {
            let mut ret_0: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")[")).clone() }))?;
            ret_0 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            (txt, a_preExp, a_varDecls) = threadDimSubListXml(txt.clone(), i_et_dims.clone(), ret_0.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_ecr, a_varDecls, a_preExp, _, _, _) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Indexing non-array ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(txt_1.clone(), a_ecr.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2256, 28), (Tpl::textString(txt_1.clone())?).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_295(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_arrName: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_ecr.clone(), in_a_arrName.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_cr.clone())) {
        (txt, i_context @ SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, a_arrName, a_varDecls, a_preExp, a_cr) => {
            let mut ret_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut l_dimsValuesStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            ret_1 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_dimsValuesStr, a_varDecls, a_preExp) = lm_293(l_dimsValuesStr.clone(), ret_1.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <exp:ArraySubscripts>\n")).clone(), (literal!("    <exp:IndexExpression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), l_dimsValuesStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:IndexExpression>\n")).clone(), (literal!("  </exp:ArraySubscripts>\n")).clone(), (literal!("</exp:QualifiedNamePart>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context, a_ecr, a_arrName, a_varDecls, a_preExp, a_cr) => {
            let mut ret_2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            ret_2 = ComponentReference::crefLastType(a_cr.clone())?;
            (txt, a_varDecls, a_preExp) = fun_294(txt.clone(), ret_2.clone(), a_ecr.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_cr.clone(), a_arrName.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_296(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_arrName: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr.clone(), in_a_arrName.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Deref @ "metatype_array", _, a_arrName, a_varDecls, a_preExp, a_context, a_cr) => {
            let mut ret_1: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut l_dimsValuesStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            ret_1 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_dimsValuesStr, a_varDecls, a_preExp) = lm_292(l_dimsValuesStr.clone(), ret_1.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /* DAE.CREF */")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_ecr, a_arrName, a_varDecls, a_preExp, a_context, a_cr) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_295(txt.clone(), a_context.clone(), a_ecr.clone(), a_arrName.clone(), a_varDecls.clone(), a_preExp.clone(), a_cr.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_297(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr.clone(), in_a_preExp.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, false, _, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut ret_4: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut l_spec1: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_arrName = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayType.clone())?).clone(), a_varDecls.clone())?;
            ret_4 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            (l_spec1, a_preExp, a_varDecls) = daeExpCrefRhsIndexSpecXml(Tpl::emptyTxt.clone(), ret_4.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arrName.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_spec1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut str_9: ArcStr = arcstr::literal!("");
            let mut ret_8: i32 = 0;
            let mut ret_7: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut l_dimsLenStr: Tpl::Text;
            let mut ret_5: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_5 = ComponentReferenceBasics::crefStripLastSubs(a_cr.clone())?;
            l_arrName = contextCrefXml(Tpl::emptyTxt.clone(), ret_5.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            ret_7 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            ret_8 = (ret_7.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            str_9 = (Tpl::textString(l_arrayType.clone())?).clone();
            (txt, a_varDecls, a_preExp) = fun_296(txt.clone(), (str_9.clone()).clone(), a_ecr.clone(), l_arrName.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_298(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ecr_componentRef.clone(), in_a_ecr.clone(), in_a_preExp.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, false, _, a_ecr, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_0 = SimCodeFunctionUtil::crefSubIsScalar(a_cr.clone())?;
            (txt, a_preExp, a_varDecls) = fun_297(txt.clone(), ret_0.clone(), a_ecr.clone(), a_preExp.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr_componentRef, _, a_preExp, a_varDecls, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), a_ecr_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_299(mut in_txt: Tpl::Text, mut in_a_box: Tpl::Text, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_box.clone(), in_a_ecr_componentRef.clone(), in_a_ecr.clone(), in_a_preExp.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_ecr_componentRef, a_ecr, a_preExp, a_varDecls, a_ty, a_context, a_cr) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_0 = SimCodeFunctionUtil::crefIsScalar(a_cr.clone(), a_context.clone())?;
            (txt, a_preExp, a_varDecls) = fun_298(txt.clone(), ret_0.clone(), a_ecr_componentRef.clone(), a_ecr.clone(), a_preExp.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_box, _, _, a_preExp, a_varDecls, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_box.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpCrefRhs2Xml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_ecr @ Deref @ DAE::Exp::CREF { ty: i_ty, componentRef: i_ecr_componentRef @ i_cr }, a_context, a_preExp, a_varDecls) => {
            let mut l_box: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_box, a_preExp, a_varDecls) = daeExpCrefRhsArrayBoxXml(Tpl::emptyTxt.clone(), i_ecr.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_preExp, a_varDecls) = fun_299(txt.clone(), l_box.clone(), i_ecr_componentRef.clone(), i_ecr.clone(), a_preExp.clone(), a_varDecls.clone(), i_ty.clone(), a_context.clone(), i_cr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_ecr, _, a_preExp, a_varDecls) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpCrefRhs2: UNHANDLED EXPRESSION: ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(txt_1.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2273, 11), (Tpl::textString(txt_1.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_301(mut in_txt: Tpl::Text, mut in_a_dim: Arc<DAE::Dimension>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dim.clone())) {
        (txt, Deref @ DAE::Dimension::DIM_INTEGER { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_ENUM { size: i_size, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_size.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2292, 22), (literal!("Non-constant dimension in simulation context")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_302(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dim, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_301(txt.clone(), i_dim.clone())?;
            txt = lm_302(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_303(mut in_txt: Tpl::Text, mut in_a_subrest: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_dimrest: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subrest.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_dimrest.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_subrest, a_varDecls, a_preExp, a_context, a_dimrest) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            (txt, a_preExp, a_varDecls) = threadDimSubListXml(txt.clone(), a_dimrest.clone(), i_subrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_304(mut in_txt: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_subrest: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_sub_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dims.clone(), in_a_subrest.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_sub_exp.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: i_dimrest }, a_subrest, a_varDecls, a_preExp, a_context, a_sub_exp) => {
            let mut l_estr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_estr, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_sub_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("((")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_estr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = lm_302(txt.clone(), i_dimrest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt, a_varDecls, a_preExp) = fun_303(txt.clone(), a_subrest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_dimrest.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, a_varDecls, a_preExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2294, 18), (literal!("Less subscripts that dimensions in indexing cref? That's odd!")).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_305(mut in_txt: Tpl::Text, mut in_a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subs.clone(), in_a_dims.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2280, 22), (literal!("Empty dimensions in indexing cref?")).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: i_sub_exp }, tail: i_subrest }, a_dims, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_304(txt.clone(), a_dims.clone(), i_subrest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_sub_exp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2295, 14), (literal!("Non-index subscript in indexing cref? That's odd!")).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn threadDimSubListXml(mut txt: Tpl::Text, mut a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = fun_305(txt.clone(), a_subs.clone(), a_dims.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_307(mut in_txt: Tpl::Text, mut in_a_sub: Arc<DAE::Subscript>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sub.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }, a_varDecls, a_preExp, a_context) => {
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_str = Tpl::writeText(Tpl::emptyTxt.clone(), l_expPart.clone())?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM, a_varDecls, a_preExp, _) => {
            let mut l_str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(1), (int*)0, 'W'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: i_exp }, a_varDecls, a_preExp, a_context) => {
            let mut l_tmp: Tpl::Text;
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_integer")).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tmp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = size_of_dimension_integer_array(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expPart.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 1);")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_str = Tpl::writeText(Tpl::emptyTxt.clone(), l_tmp.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", integer_array_make_index_array(")).clone() }))?;
            l_str = Tpl::writeText(l_str.clone(), l_expPart.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), 'A'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn lm_308(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_sub, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_307(txt.clone(), i_sub.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_308(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpCrefRhsIndexSpecXml(mut txt: Tpl::Text, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_tmp: Tpl::Text;
    let mut l_idx__str: Tpl::Text;
    let mut ret_1: i32 = 0;
    let mut l_nridx__str: Tpl::Text;
    ret_1 = (a_subs.clone().len() as i32);
    l_nridx__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
    l_idx__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_idx__str, out_a_varDecls, out_a_preExp) = lm_308(l_idx__str.clone(), a_subs.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
    l_idx__str = Tpl::popIter(l_idx__str.clone())?;
    (l_tmp, out_a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("index_spec_t")).clone(), out_a_varDecls.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("create_index_spec(&")).clone() }))?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_tmp.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_nridx__str.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_idx__str.clone())?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
    out_a_preExp = Tpl::writeTok(out_a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_310(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dim, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dimensionXml(txt.clone(), i_dim.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_310(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_311(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_preExp: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_varDecls: Tpl::Text, mut in_a_aty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_ecr_componentRef.clone(), in_a_preExp.clone(), in_a_dims.clone(), in_a_varDecls.clone(), in_a_aty.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, a_preExp, _, a_varDecls, _) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr_componentRef, a_preExp, a_dims, a_varDecls, a_aty) => {
            let mut l_type: Tpl::Text;
            let mut l_dimsValuesStr: Tpl::Text;
            let mut ret_3: i32 = 0;
            let mut l_dimsLenStr: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_tmpArr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_1 = expTypeArrayXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            (l_tmpArr, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_1.clone())?).clone(), a_varDecls.clone())?;
            ret_3 = (a_dims.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_3.clone())).clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_dimsValuesStr = lm_310(l_dimsValuesStr.clone(), a_dims.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            l_type = expTypeShortXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            a_preExp = arrayCrefXmlStr(a_preExp.clone(), a_ecr_componentRef.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tmpArr.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpCrefRhsArrayBoxXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_ecr_componentRef, ty: Deref @ DAE::Type::T_ARRAY { dims: i_dims, ty: i_aty } }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = fun_311(txt.clone(), a_context.clone(), i_ecr_componentRef.clone(), a_preExp.clone(), i_dims.clone(), a_varDecls.clone(), i_aty.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_313(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Var>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: rest }, a_varDecls, a_preExp, a_context, a_cr) => {
            let mut ret_0: Arc<DAE::Exp>;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            ret_0 = SimCodeFunctionUtil::makeCrefRecordExp(a_cr.clone(), i_v.clone())?;
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), ret_0.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_313(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpRecordCrefRhsXml(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_cr.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { varLst: i_var__lst, complexClassType: _, .. }, a_cr, a_context, a_preExp, a_varDecls) => {
            let mut l_vars: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_vars = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_vars, a_varDecls, a_preExp) = lm_313(l_vars.clone(), i_var__lst.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), a_cr.clone())?;
            l_vars = Tpl::popIter(l_vars.clone())?;
            txt = Tpl::writeText(txt.clone(), l_vars.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_315(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_t: Arc<DAE::Type>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_t.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_exp.clone())) {
        (txt, i_context @ SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, _, a_varDecls, a_afterExp, a_exp) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (txt, a_afterExp, a_varDecls) = daeExpCrefLhs2Xml(txt.clone(), a_exp.clone(), i_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, i_context, a_cr, a_t, a_varDecls, a_afterExp, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (txt, a_afterExp, a_varDecls) = daeExpRecordCrefLhsXml(txt.clone(), a_t.clone(), a_cr.clone(), i_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

pub fn daeExpCrefLhsXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, i_exp @ Deref @ DAE::Exp::CREF { ty: i_t @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, componentRef: i_cr }, a_context, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_afterExp) = fun_315(txt.clone(), a_context.clone(), i_cr.clone(), i_t.clone(), a_varDecls.clone(), a_afterExp.clone(), i_exp.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: _, .. }, componentRef: i_cr }, _, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefFunctionNameXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }, componentRef: i_cr }, _, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefStrXml(txt.clone(), i_cr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_context, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_afterExp, a_varDecls) = daeExpCrefLhs2Xml(txt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn lm_317(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_afterExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Subscript::INDEX { exp: i_exp }, tail: rest }, a_varDecls, a_afterExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (txt, a_afterExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_afterExp) = lm_317(txt.clone(), rest.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_varDecls, a_afterExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (txt, a_varDecls, a_afterExp) = lm_317(txt.clone(), rest.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn fun_318(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_dimsValuesStr: Tpl::Text, mut in_a_arrName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_dimsValuesStr.clone(), in_a_arrName.clone())) {
        (txt, Deref @ "metatype_array", a_dimsValuesStr, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /* DAE.CREF */")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_dimsValuesStr, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Identifier>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:QualifiedNamePart name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <exp:ArraySubscripts>\n")).clone(), (literal!("    <exp:IndexExpression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    </exp:IndexExpression>\n")).clone(), (literal!("  </exp:ArraySubscripts>\n")).clone(), (literal!("</exp:QualifiedNamePart>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Identifier>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_319(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_afterExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_cr.clone(), in_a_ecr.clone(), in_a_afterExp.clone())) {
        (txt, false, a_varDecls, a_ty, a_context, a_cr, a_ecr, a_afterExp) => {
            let mut ret_4: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut l_spec1: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 SLICE(")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), a_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") afterExp  */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_arrName = contextArrayCrefXml(Tpl::emptyTxt.clone(), a_cr.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayType.clone())?).clone(), a_varDecls.clone())?;
            ret_4 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            (l_spec1, a_afterExp, a_varDecls) = daeExpCrefLhsIndexSpecXml(Tpl::emptyTxt.clone(), ret_4.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("indexed_assign_")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_arrayType.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_tmp.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_arrName.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_spec1.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, _, a_varDecls, a_ty, a_context, a_cr, a_ecr, a_afterExp) => {
            let mut str_11: ArcStr = arcstr::literal!("");
            let mut ret_10: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut l_dimsValuesStr: Tpl::Text;
            let mut ret_8: i32 = 0;
            let mut ret_7: Arc<metamodelica::List<Arc<DAE::Subscript>>> = metamodelica::nil();
            let mut l_dimsLenStr: Tpl::Text;
            let mut ret_5: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut l_arrayType: Tpl::Text;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 SCALAR(")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), a_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") afterExp  */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            ret_5 = ComponentReferenceBasics::crefStripLastSubs(a_cr.clone())?;
            l_arrName = contextCrefXml(Tpl::emptyTxt.clone(), ret_5.clone(), a_context.clone())?;
            l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
            ret_7 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            ret_8 = (ret_7.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            ret_10 = ComponentReferenceBasics::crefSubs(a_cr.clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_dimsValuesStr, a_varDecls, a_afterExp) = lm_317(l_dimsValuesStr.clone(), ret_10.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            str_11 = (Tpl::textString(l_arrayType.clone())?).clone();
            txt = fun_318(txt.clone(), (str_11.clone()).clone(), l_dimsValuesStr.clone(), l_arrName.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn fun_320(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_afterExp: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_context.clone(), in_a_ecr.clone(), in_a_afterExp.clone(), in_a_cr.clone())) {
        (txt, false, a_varDecls, a_ty, a_context, a_ecr, a_afterExp, a_cr) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            ret_0 = SimCodeFunctionUtil::crefSubIsScalar(a_cr.clone())?;
            (txt, a_varDecls, a_afterExp) = fun_319(txt.clone(), ret_0.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_cr.clone(), a_ecr.clone(), a_afterExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, _, a_varDecls, _, a_context, _, a_afterExp, a_cr) => {
            let mut txt = (*txt).clone();
            txt = contextCrefXml(txt.clone(), a_cr.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn fun_321(mut in_txt: Tpl::Text, mut in_a_box: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_box.clone(), in_a_varDecls.clone(), in_a_ty.clone(), in_a_ecr.clone(), in_a_afterExp.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_varDecls, a_ty, a_ecr, a_afterExp, a_context, a_cr) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            ret_0 = SimCodeFunctionUtil::crefIsScalar(a_cr.clone(), a_context.clone())?;
            (txt, a_varDecls, a_afterExp) = fun_320(txt.clone(), ret_0.clone(), a_varDecls.clone(), a_ty.clone(), a_context.clone(), a_ecr.clone(), a_afterExp.clone(), a_cr.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, i_box, a_varDecls, _, _, a_afterExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_box.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

pub fn daeExpCrefLhs2Xml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, i_ecr @ Deref @ DAE::Exp::CREF { ty: i_ty, componentRef: i_cr }, a_context, a_afterExp, a_varDecls) => {
            let mut l_box: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 begin afterExp (")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (l_box, a_afterExp, a_varDecls) = daeExpCrefLhsArrayBoxXml(Tpl::emptyTxt.clone(), i_ecr.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_afterExp) = fun_321(txt.clone(), l_box.clone(), a_varDecls.clone(), i_ty.clone(), i_ecr.clone(), a_afterExp.clone(), a_context.clone(), i_cr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, i_ecr, _, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* daeExpCrefLhs2 UNHANDLED(")).clone() }))?;
            a_afterExp = ExpressionDumpTpl::dumpExp(a_afterExp.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") afterExp */")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* SimCodeC.tpl template: daeExpCrefLhs2: UNHANDLED EXPRESSION:\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* ")).clone() }))?;
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_ecr.clone(), (literal!("\"")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn fun_323(mut in_txt: Tpl::Text, mut in_a_sub: Arc<DAE::Subscript>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sub.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }, a_varDecls, a_afterExp, a_context) => {
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (l_expPart, a_afterExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(0), make_index_array(1, (int) ")).clone() }))?;
            l_str = Tpl::writeText(l_str.clone(), l_expPart.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), 'S'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ DAE::Subscript::WHOLEDIM, a_varDecls, a_afterExp, _) => {
            let mut l_str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_str = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(1), (int*)0, 'W'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: i_exp }, a_varDecls, a_afterExp, a_context) => {
            let mut l_tmp: Tpl::Text;
            let mut l_str: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (l_expPart, a_afterExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_integer")).clone(), a_varDecls.clone())?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_tmp.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = size_of_dimension_integer_array(&")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_expPart.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 1);")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_str = Tpl::writeText(Tpl::emptyTxt.clone(), l_tmp.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", integer_array_make_index_array(")).clone() }))?;
            l_str = Tpl::writeText(l_str.clone(), l_expPart.clone())?;
            l_str = Tpl::writeTok(l_str.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), 'A'")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, _, a_varDecls, a_afterExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

fn lm_324(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_afterExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_sub, tail: rest }, a_varDecls, a_afterExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            (txt, a_varDecls, a_afterExp) = fun_323(txt.clone(), i_sub.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_afterExp) = lm_324(txt.clone(), rest.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

pub fn daeExpCrefLhsIndexSpecXml(mut txt: Tpl::Text, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_context: SimCodeFunction::Context, mut a_afterExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_tmp: Tpl::Text;
    let mut l_idx__str: Tpl::Text;
    let mut ret_1: i32 = 0;
    let mut l_nridx__str: Tpl::Text;
    ret_1 = (a_subs.clone().len() as i32);
    l_nridx__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
    l_idx__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_idx__str, out_a_varDecls, out_a_afterExp) = lm_324(l_idx__str.clone(), a_subs.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone())?;
    l_idx__str = Tpl::popIter(l_idx__str.clone())?;
    (l_tmp, out_a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("index_spec_t")).clone(), out_a_varDecls.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("create_index_spec(&")).clone() }))?;
    out_a_afterExp = Tpl::writeText(out_a_afterExp.clone(), l_tmp.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_afterExp = Tpl::writeText(out_a_afterExp.clone(), l_nridx__str.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_a_afterExp = Tpl::writeText(out_a_afterExp.clone(), l_idx__str.clone())?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
    out_a_afterExp = Tpl::writeTok(out_a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_326(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dim, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dimensionXml(txt.clone(), i_dim.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_326(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_327(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_ecr_componentRef: Arc<DAE::ComponentRef>, mut in_a_afterExp: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_varDecls: Tpl::Text, mut in_a_aty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_ecr_componentRef.clone(), in_a_afterExp.clone(), in_a_dims.clone(), in_a_varDecls.clone(), in_a_aty.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, _, a_afterExp, _, a_varDecls, _) => {
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_ecr_componentRef, a_afterExp, a_dims, a_varDecls, a_aty) => {
            let mut l_type: Tpl::Text;
            let mut l_dimsValuesStr: Tpl::Text;
            let mut ret_3: i32 = 0;
            let mut l_dimsLenStr: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_tmpArr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_1 = expTypeArrayXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            (l_tmpArr, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_1.clone())?).clone(), a_varDecls.clone())?;
            ret_3 = (a_dims.clone().len() as i32);
            l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_3.clone())).clone())?;
            l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_dimsValuesStr = lm_326(l_dimsValuesStr.clone(), a_dims.clone())?;
            l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
            l_type = expTypeShortXml(Tpl::emptyTxt.clone(), a_aty.clone())?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_type.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_array_create(&")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_tmpArr.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ((modelica_")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_type.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*)&(")).clone() }))?;
            a_afterExp = arrayCrefXmlStr(a_afterExp.clone(), a_ecr_componentRef.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")), ")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_dimsLenStr.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_dimsValuesStr.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tmpArr.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

pub fn daeExpCrefLhsArrayBoxXml(mut in_txt: Tpl::Text, mut in_a_ecr: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ecr.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_ecr_componentRef, ty: Deref @ DAE::Type::T_ARRAY { dims: i_dims, ty: i_aty } }, a_context, a_afterExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_afterExp, a_varDecls) = fun_327(txt.clone(), a_context.clone(), i_ecr_componentRef.clone(), a_afterExp.clone(), i_dims.clone(), a_varDecls.clone(), i_aty.clone())?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_afterExp, a_varDecls) => {
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn lm_329(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Var>>>, mut in_a_varDecls: Tpl::Text, mut in_a_afterExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_afterExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_afterExp.clone(), in_a_context.clone(), in_a_cr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_afterExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: rest }, a_varDecls, a_afterExp, a_context, a_cr) => {
            let mut ret_0: Arc<DAE::Exp>;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            ret_0 = SimCodeFunctionUtil::makeCrefRecordExp(a_cr.clone(), i_v.clone())?;
            (txt, a_afterExp, a_varDecls) = daeExpXml(txt.clone(), ret_0.clone(), a_context.clone(), a_afterExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_afterExp) = lm_329(txt.clone(), rest.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone(), a_cr.clone())?;
            (txt.clone(), a_varDecls.clone(), a_afterExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_afterExp))
}

pub fn daeExpRecordCrefLhsXml(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_cr: Arc<DAE::ComponentRef>, mut in_a_context: SimCodeFunction::Context, mut in_a_afterExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_afterExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_afterExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_cr.clone(), in_a_context.clone(), in_a_afterExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { varLst: i_var__lst, complexClassType: i_record__state, .. }, a_cr, a_context, a_afterExp, a_varDecls) => {
            let mut l_ret__var: Tpl::Text;
            let mut l_ret__type: Tpl::Text;
            let mut ret_2: Arc<Absyn::Path>;
            let mut l_record__type__name: Tpl::Text;
            let mut l_vars: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_afterExp = (*a_afterExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_vars = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_vars, a_varDecls, a_afterExp) = lm_329(l_vars.clone(), i_var__lst.clone(), a_varDecls.clone(), a_afterExp.clone(), a_context.clone(), a_cr.clone())?;
            l_vars = Tpl::popIter(l_vars.clone())?;
            ret_2 = ClassInfUtil::getStateName(i_record__state.clone());
            l_record__type__name = underscorePathXml(Tpl::emptyTxt.clone(), ret_2.clone())?;
            l_ret__type = Tpl::writeText(Tpl::emptyTxt.clone(), l_record__type__name.clone())?;
            l_ret__type = Tpl::writeTok(l_ret__type.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_rettype")).clone() }))?;
            (l_ret__var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ret__type.clone())?).clone(), a_varDecls.clone())?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_ret__var.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = _")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_record__type__name.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            a_afterExp = Tpl::writeText(a_afterExp.clone(), l_vars.clone())?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_afterExp = Tpl::writeTok(a_afterExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_ret__var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ret__type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_1")).clone() }))?;
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_afterExp, a_varDecls) => {
            (txt.clone(), a_afterExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_afterExp, out_a_varDecls))
}

fn fun_331(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_332(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_333(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_334(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_scalar")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_scalar")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_scalar")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_335(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_336(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_337(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_exp: Arc<DAE::Exp>, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_operator.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_exp.clone(), in_a_e2.clone(), in_a_e1.clone())) {
        (txt, DAE::Operator::ADD { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Add>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Add>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::SUB { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Sub>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Sub>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Mul>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Mul>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW { ty: _ }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Pow>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Pow>")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::UMINUS { ty: _ }, a_varDecls, a_preExp, a_context, a_exp, _, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpUnaryXml(txt.clone(), a_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::ADD_ARR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_331(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Add>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Add>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::SUB_ARR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_332(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Sub>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Sub>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_ARR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for MUL_ARR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV_ARR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for DIV_ARR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_333(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Mul>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Mul>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for ADD_ARRAY_SCALAR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for SUB_SCALAR_ARRAY")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            l_type = fun_334(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mul_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_product(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_typeShort: Tpl::Text;
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_typeShort = fun_335(Tpl::emptyTxt.clone(), i_ty.clone())?;
            l_type = Tpl::writeText(Tpl::emptyTxt.clone(), l_typeShort.clone())?;
            l_type = Tpl::writeTok(l_type.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_array")).clone() }))?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Mul>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Mul>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: i_ty }, a_varDecls, a_preExp, _, _, a_e2, a_e1) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_type = fun_336(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for DIV_SCALAR_ARRAY")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_ARRAY_SCALAR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_ARRAY_SCALAR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_SCALAR_ARRAY { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_SCALAR_ARRAY")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_ARR { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_ARR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, DAE::Operator::POW_ARR2 { ty: _ }, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR for POW_ARR2")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpBinary:ERR")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpBinaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_exp @ Deref @ DAE::Exp::BINARY { operator: i_operator, exp2: i_exp2, exp1: i_exp1 }, a_context, a_preExp, a_varDecls) => {
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_337(txt.clone(), i_operator.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_exp.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_339(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_e: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_operator.clone(), in_a_e.clone())) {
        (txt, DAE::Operator::UMINUS { ty: _ }, a_e) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:Neg>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </exp:Neg>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::UMINUS_ARR { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. } }, a_e) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:Neg>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </exp:Neg>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::UMINUS_ARR { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2675, 34), (literal!("unary minus for non-real arrays not implemented")).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 2676, 14), (literal!("daeExpUnary:ERR")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn daeExpUnaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::UNARY { operator: i_operator, exp: i_exp }, a_context, a_preExp, a_varDecls) => {
            let mut l_e: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_339(txt.clone(), i_operator.clone(), l_e.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_341(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_operator.clone(), in_a_e2.clone(), in_a_e1.clone()) {
        (mut txt, DAE::Operator::AND { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:And>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:And>")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::OR { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Or>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Or>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpLbinary:ERR")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn daeExpLbinaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::LBINARY { operator: i_operator, exp2: i_exp2, exp1: i_exp1 }, a_context, a_preExp, a_varDecls) => {
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_341(txt.clone(), i_operator.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_343(mut in_txt: Tpl::Text, mut in_a_operator: DAE::Operator, mut in_a_e: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_operator.clone(), in_a_e.clone()) {
        (mut txt, DAE::Operator::NOT { ty: _ }, mut a_e) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Not>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Not>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn daeExpLunaryXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::LUNARY { operator: i_operator, exp: i_exp }, a_context, a_preExp, a_varDecls) => {
            let mut l_e: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_e, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_343(txt.clone(), i_operator.clone(), l_e.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_345(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_operator.clone(), in_a_e2.clone(), in_a_e1.clone())) {
        (txt, DAE::Operator::LESS { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESS { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") < 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESS { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLt>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATER { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATER { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") > 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATER { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGt>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESSEQ { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESSEQ { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") <= 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::LESSEQ { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATEREQ { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATEREQ { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringCompare(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") >= 0)")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::GREATEREQ { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::EQUAL { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("((!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") || (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::EQUAL { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(stringEqual(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::EQUAL { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogEq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogEq>")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::NEQUAL { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("((!")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") || (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::NEQUAL { ty: Deref @ DAE::Type::T_STRING { varLst: _ } }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(!stringEqual(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Operator::NEQUAL { ty: _ }, a_e2, a_e1) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogNeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogNeq>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("daeExpRelation:ERR")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_346(mut in_txt: Tpl::Text, mut in_a_simRel: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simRel.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = fun_345(txt.clone(), a_rel_operator.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_simRel, _, _, a_varDecls, a_preExp, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_simRel.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpRelationXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, i_rel @ Deref @ DAE::Exp::RELATION { operator: i_rel_operator, exp2: i_rel_exp2, exp1: i_rel_exp1, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_simRel: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_simRel, a_preExp, a_varDecls) = daeExpRelationSimXml(Tpl::emptyTxt.clone(), i_rel.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_346(txt.clone(), l_simRel.clone(), i_rel_operator.clone(), i_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_rel_exp1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_348(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_349(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_350(mut in_txt: Tpl::Text, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_optionExpisASUB.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, None, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_348(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Some((i_exp, _, _)), a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_iterator: Tpl::Text;
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_iterator, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_349(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_351(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_352(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_res: Tpl::Text, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text, mut in_a_preExp: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_preExp) = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_res.clone(), in_a_e2.clone(), in_a_e1.clone(), in_a_preExp.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogLt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogLeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogLeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogGt>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGt>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_res, mut a_e2, mut a_e1, mut a_preExp) => {
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    <exp:LogGeq>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e1.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_e2.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:LogGeq>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            (txt.clone(), a_preExp.clone())
        },
        (mut txt, _, _, _, _, mut a_preExp) => {
            (txt.clone(), a_preExp.clone())
        },
    });
    Ok((out_txt, out_a_preExp))
}

fn fun_353(mut in_txt: Tpl::Text, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_optionExpisASUB.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, None, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_351(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Some((i_exp, _, _)), a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_iterator: Tpl::Text;
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            (l_iterator, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_preExp) = fun_352(txt.clone(), a_rel_operator.clone(), l_res.clone(), l_e2.clone(), l_e1.clone(), a_preExp.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_354(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_rel_exp1: Arc<DAE::Exp>, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_rel_exp1.clone(), in_a_rel_optionExpisASUB.clone())) {
        (txt, i_context @ SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: false }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_rel_exp1, a_rel_optionExpisASUB) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_350(txt.clone(), a_rel_optionExpisASUB.clone(), a_rel_operator.clone(), a_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_rel_exp1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context @ SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: true }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_rel_exp1, a_rel_optionExpisASUB) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_353(txt.clone(), a_rel_optionExpisASUB.clone(), a_rel_operator.clone(), a_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_rel_exp1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpRelationSimXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RELATION { operator: i_rel_operator, exp2: i_rel_exp2, exp1: i_rel_exp1, optionExpisASUB: i_rel_optionExpisASUB, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_354(txt.clone(), a_context.clone(), i_rel_operator.clone(), i_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_rel_exp1.clone(), i_rel_optionExpisASUB.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_356(mut in_txt: Tpl::Text, mut in_a_rel_operator: DAE::Operator, mut in_a_e2: Tpl::Text, mut in_a_e1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_rel_operator.clone(), in_a_e2.clone(), in_a_e1.clone()) {
        (mut txt, DAE::Operator::EQUAL { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ConstraintEqu>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ConstraintEqu>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ConstraintLeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ConstraintLeq>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }, mut a_e2, mut a_e1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<opt:ConstraintGeq>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_e1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_e2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</opt:ConstraintGeq>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  \"The XML schema does only support =, >= , <=  operators for constraints\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_357(mut in_txt: Tpl::Text, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_exp1: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rel_optionExpisASUB.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_rel_exp1.clone())) {
        (txt, None, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_context, a_rel_exp1) => {
            let mut l_res: Tpl::Text;
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_e2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_rel_exp2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_boolean")).clone(), a_varDecls.clone())?;
            txt = fun_356(txt.clone(), a_rel_operator.clone(), l_e2.clone(), l_e1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_358(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_rel_operator: DAE::Operator, mut in_a_rel_exp2: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_rel_exp1: Arc<DAE::Exp>, mut in_a_rel_optionExpisASUB: Option<(Arc<DAE::Exp>, i32, i32)>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_rel_operator.clone(), in_a_rel_exp2.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_rel_exp1.clone(), in_a_rel_optionExpisASUB.clone())) {
        (txt, i_context @ SimCodeFunction::Context::SIMULATION_CONTEXT { genDiscrete: true }, a_rel_operator, a_rel_exp2, a_varDecls, a_preExp, a_rel_exp1, a_rel_optionExpisASUB) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_varDecls, a_preExp) = fun_357(txt.clone(), a_rel_optionExpisASUB.clone(), a_rel_operator.clone(), a_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_context.clone(), a_rel_exp1.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpConstraintXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RELATION { operator: i_rel_operator, exp2: i_rel_exp2, exp1: i_rel_exp1, optionExpisASUB: i_rel_optionExpisASUB, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_358(txt.clone(), a_context.clone(), i_rel_operator.clone(), i_rel_exp2.clone(), a_varDecls.clone(), a_preExp.clone(), i_rel_exp1.clone(), i_rel_optionExpisASUB.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpIfXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::IFEXP { expElse: i_expElse, expThen: i_expThen, expCond: i_expCond }, a_context, a_preExp, a_varDecls) => {
            let mut l_eElse: Tpl::Text;
            let mut l_preExpElse: Tpl::Text;
            let mut l_eThen: Tpl::Text;
            let mut l_preExpThen: Tpl::Text;
            let mut l_resVar: Tpl::Text;
            let mut l_condExp: Tpl::Text;
            let mut l_preExpCond: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExpCond = Tpl::emptyTxt.clone();
            (l_condExp, l_preExpCond, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_expCond.clone(), a_context.clone(), l_preExpCond.clone(), a_varDecls.clone())?;
            l_resVar = Tpl::emptyTxt.clone();
            l_preExpThen = Tpl::emptyTxt.clone();
            (l_eThen, l_preExpThen, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_expThen.clone(), a_context.clone(), l_preExpThen.clone(), a_varDecls.clone())?;
            l_preExpElse = Tpl::emptyTxt.clone();
            (l_eElse, l_preExpElse, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_expElse.clone(), a_context.clone(), l_preExpElse.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:If>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_condExp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Statements>\n")).clone()], lastHasNewLine: true }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_eThen.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Statements>\n")).clone(), (literal!("  <fun:Else>\n")).clone()], lastHasNewLine: true }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_eElse.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Else>\n")).clone(), (literal!("</fun:If>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_resVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_361(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer_array")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real_array")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_362(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_var1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_var1.clone()) {
        (mut txt, false, mut a_var1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("puts(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_var1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("print(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_363(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dim, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_dim.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_363(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn lm_364(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_array, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_array.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_364(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn lm_365(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_365(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_366(mut in_txt: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_argStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_preExp.clone(), in_a_argStr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_argStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_argStr.clone())?;
            txt.clone()
        },
        (txt, i_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_preExp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_367(mut in_txt: Tpl::Text, mut in_a_attr_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attr_ty.clone())) {
        (txt, Deref @ DAE::Type::T_NORETCALL) => {
            txt.clone()
        },
        (txt, i_attr_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeModelicaXml(txt.clone(), i_attr_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_368(mut in_txt: Tpl::Text, mut in_a_attr_builtin: bool, mut in_a_attr_ty: Arc<DAE::Type>, mut in_a_funName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attr_builtin.clone(), in_a_attr_ty.clone(), in_a_funName.clone())) {
        (txt, false, _, a_funName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_funName.clone())?;
            txt.clone()
        },
        (txt, _, a_attr_ty, _) => {
            let mut txt = (*txt).clone();
            txt = fun_367(txt.clone(), a_attr_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_369(mut in_txt: Tpl::Text, mut in_a_attr_ty: Arc<DAE::Type>, mut in_a_varDecls: Tpl::Text, mut in_a_retType: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attr_ty.clone(), in_a_varDecls.clone(), in_a_retType.clone())) {
        (txt, Deref @ DAE::Type::T_NORETCALL, a_varDecls, _) => {
            (txt.clone(), a_varDecls.clone())
        },
        (txt, _, a_varDecls, a_retType) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls) = tempDeclXml(txt.clone(), (Tpl::textString(a_retType.clone())?).clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

fn fun_370(mut in_txt: Tpl::Text, mut in_a_attr_builtin: bool, mut in_a_builtinFunctionName: Tpl::Text, mut in_a_result: Tpl::Text, mut in_a_funName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attr_builtin.clone(), in_a_builtinFunctionName.clone(), in_a_result.clone(), in_a_funName.clone()) {
        (mut txt, false, _, mut a_result, mut a_funName) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<exp:FunctionCall>\n")).clone(), (literal!("  <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_funName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Name>\n")).clone(), (literal!("  <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_result.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Arguments>\n")).clone(), (literal!("</exp:FunctionCall>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, mut a_builtinFunctionName, mut a_result, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp:")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_builtinFunctionName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_result.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_builtinFunctionName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_371(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_builtinFunctionName: Tpl::Text, mut in_a_result: Tpl::Text, mut in_a_funName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_builtinFunctionName.clone(), in_a_result.clone(), in_a_funName.clone())) {
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_NORETCALL, .. }, .. }, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* NORETCALL */")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: i_attr_builtin, tuple_: false, .. }, .. }, a_builtinFunctionName, a_result, a_funName) => {
            let mut txt = (*txt).clone();
            txt = fun_370(txt.clone(), i_attr_builtin.clone(), a_builtinFunctionName.clone(), a_result.clone(), a_funName.clone())?;
            txt.clone()
        },
        (txt, _, _, a_result, a_funName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<exp:FunctionCall>\n")).clone(), (literal!("  <exp:Name>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_funName.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Name>\n")).clone(), (literal!("  <exp:Arguments>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_result.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </exp:Arguments>\n")).clone(), (literal!("</exp:FunctionCall>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn daeExpCallXml(mut in_txt: Tpl::Text, mut in_a_call: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_call.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::SCONST { string: i_string }, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_var3: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            ret_3 = (Util::escapeModelicaStringToXmlString((i_string.clone()).clone())?).clone();
            l_var3 = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Cons { head: i_e3 @ Deref @ DAE::Exp::SHARED_LITERAL { index: _, .. }, tail: Deref @ metamodelica::List::Nil } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_ARRAY_SCALAR" }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var: Tpl::Text;
            let mut l_type: Tpl::Text;
            let mut l_var3: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_type = fun_361(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_var, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), a_varDecls.clone())?;
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var3, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e3.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("division_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_type.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_scalar(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var2.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var3.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_var.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION_ARRAY_SCALAR" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_6: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_6 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support ")).clone() }))?;
            txt_6 = ExpressionDumpTpl::dumpExp(txt_6.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3088, 11), (Tpl::textString(txt_6.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Der>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Der>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_exp, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_7: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_7 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support der(")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpExp(txt_7.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_7 = Tpl::writeTok(txt_7.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3097, 11), (Tpl::textString(txt_7.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_arg, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpCallPreXml(txt.clone(), i_arg.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_exp, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_8: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_8 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support edge(")).clone() }))?;
            txt_8 = ExpressionDumpTpl::dumpExp(txt_8.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_8 = Tpl::writeTok(txt_8.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3105, 11), (Tpl::textString(txt_8.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::CREF { componentRef: i_arg_componentRef, .. }, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = crefXml(txt.clone(), i_arg_componentRef.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_exp, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_9: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_9 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support change(")).clone() }))?;
            txt_9 = ExpressionDumpTpl::dumpExp(txt_9.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_9 = Tpl::writeTok(txt_9.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3111, 11), (Tpl::textString(txt_9.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "print" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut ret_10: bool = false;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            ret_10 = Config::acceptMetaModelicaGrammar()?;
            txt = fun_362(txt.clone(), ret_10.clone(), l_var1.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Max>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Max>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Max>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Max>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } }, a_context, a_preExp, a_varDecls) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_arr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_arr, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("sum_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Min>\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Min>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Min>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Min>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Abs>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Abs>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Abs>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Abs>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: _, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" } }, a_context, a_preExp, a_varDecls) => {
            let mut l_argStr: Tpl::Text;
            let mut ret_15: Arc<DAE::Exp>;
            let mut ret_14: Arc<DAE::Exp>;
            let mut l_retPre: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_14 = SimCodeFunctionUtil::createAssertforSqrt(i_e1.clone())?;
            ret_15 = SimCodeFunctionUtil::createDAEString((literal!("Model error: Argument of sqrt should be >= 0")).clone());
            (l_retPre, a_varDecls) = assertCommonXml(Tpl::emptyTxt.clone(), ret_14.clone(), ret_15.clone(), a_context.clone(), a_varDecls.clone(), Absyn::dummyInfo.clone())?;
            (l_argStr, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_retPre.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Sqrt>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_argStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Sqrt>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Div>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Div>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" } }, a_context, a_preExp, a_varDecls) => {
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_mod_")).clone() }))?;
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_array, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "max" } }, a_context, a_preExp, a_varDecls) => {
            let mut txt_20: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_expVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_array.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt_20 = expTypeModelicaXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_20.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = max_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expVar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_array, tail: Deref @ metamodelica::List::Nil }, attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "min" } }, a_context, a_preExp, a_varDecls) => {
            let mut txt_21: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_expVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_array.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt_21 = expTypeModelicaXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_21.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = min_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expVar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_val, tail: i_dims }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" } }, a_context, a_preExp, a_varDecls) => {
            let mut ret_24: i32 = 0;
            let mut l_dimsExp: Tpl::Text;
            let mut l_valExp: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_valExp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_val.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_dimsExp = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_dimsExp, a_varDecls, a_preExp) = lm_363(l_dimsExp.clone(), i_dims.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_dimsExp = Tpl::popIter(l_dimsExp.clone())?;
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ty__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fill_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_ty__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_valExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            ret_24 = (i_dims.clone().len() as i32);
            a_preExp = Tpl::writeStr(a_preExp.clone(), (intString(ret_24.clone())).clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_dimsExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_call @ Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "vector" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt_25: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_25 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("vector() call does not have a C implementation ")).clone() }))?;
            txt_25 = ExpressionDumpTpl::dumpExp(txt_25.clone(), i_call.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3236, 11), (Tpl::textString(txt_25.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: i_ty, .. }, expLst: Deref @ metamodelica::List::Cons { head: i_dim, tail: i_arrays }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cat" } }, a_context, a_preExp, a_varDecls) => {
            let mut ret_28: i32 = 0;
            let mut l_arrays__exp: Tpl::Text;
            let mut l_dim__exp: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_dim__exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_dim.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arrays__exp = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_arrays__exp, a_varDecls, a_preExp) = lm_364(l_arrays__exp.clone(), i_arrays.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_arrays__exp = Tpl::popIter(l_arrays__exp.clone())?;
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ty__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cat_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_ty__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_dim__exp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            ret_28 = (i_arrays.clone().len() as i32);
            a_preExp = Tpl::writeStr(a_preExp.clone(), (intString(ret_28.clone())).clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arrays__exp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" where is cat2")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_A, tail: Deref @ metamodelica::List::Cons { head: i_n, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "promote" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_A.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_n.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_A.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("promote_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var2.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_A, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "transpose" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_A.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_A.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("transpose_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_v1, tail: Deref @ metamodelica::List::Cons { head: i_v2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "cross" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_v1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_v2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_v1.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cross_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var2.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_A, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "identity" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tvar: Tpl::Text;
            let mut l_arr__tp__str: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_A.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_arr__tp__str = expTypeFromExpArrayXml(Tpl::emptyTxt.clone(), i_A.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arr__tp__str.clone())?).clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("identity_alloc_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_arr__tp__str.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_var1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Cons { head: i_e2, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_typeStr: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e2.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_typeStr = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), i_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_rem_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_typeStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: _ }, tail: Deref @ metamodelica::List::Cons { head: i_e, tail: Deref @ metamodelica::List::Cons { head: i_d, tail: Deref @ metamodelica::List::Cons { head: i_delayMax, tail: Deref @ metamodelica::List::Nil } } } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_var3: Tpl::Text;
            let mut l_var2: Tpl::Text;
            let mut l_var1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_var1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var2, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_d.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_var3, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_delayMax.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Delay>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_var1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var3.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Delay>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_toBeCasted, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_castedVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_castedVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_toBeCasted.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_castedVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_toBeCasted, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_castedVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_castedVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_toBeCasted.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_castedVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Nil, path: Deref @ Absyn::Path::IDENT { name: Deref @ "clock" }, .. }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_clock()")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_e1, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "anyString" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_e1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: i_s1, tail: Deref @ metamodelica::List::Cons { head: Deref @ DAE::Exp::ICONST { integer: i_i }, tail: Deref @ metamodelica::List::Nil } }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mmc_get_field" }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_expPart: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (literal!("modelica_metatype")).clone(), a_varDecls.clone())?;
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_s1.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expPart.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            a_preExp = Tpl::writeStr(a_preExp.clone(), (intString(i_i.clone())).clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: _, .. }, expLst: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "mmc_unbox_record" } }, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  \"mmc_unbox_record\" is not necessary")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { expLst: i_expLst, attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { vars: i_tail_vars, .. }, .. }, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_res: Tpl::Text;
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_res = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* Tail recursive call ")).clone() }))?;
            l_res = ExpressionDumpTpl::dumpExp(l_res.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            l_res = Tpl::writeTok(l_res.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" */\n")).clone() }))?;
            (l_res, a_preExp, a_varDecls) = daeExpTailCallXml(l_res.clone(), i_expLst.clone(), i_tail_vars.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_res = Tpl::writeTok(l_res.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("goto _tailrecursive;\n")).clone(), (literal!("/* TODO: Make sure any eventual dead code below is never generated */")).clone()], lastHasNewLine: false }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp @ Deref @ DAE::Exp::CALL { path: i_path, expLst: i_expLst, attr: Deref @ DAE::CallAttributes { ty: i_attr_ty, builtin: i_attr_builtin, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut l_retVar: Tpl::Text;
            let mut l_retType: Tpl::Text;
            let mut l_funName: Tpl::Text;
            let mut l_builtinFunctionName: Tpl::Text;
            let mut l_result: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut l_argStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_argStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_argStr, a_varDecls, l_preExp) = lm_365(l_argStr.clone(), i_expLst.clone(), a_varDecls.clone(), l_preExp.clone(), a_context.clone())?;
            l_argStr = Tpl::popIter(l_argStr.clone())?;
            l_result = fun_366(Tpl::emptyTxt.clone(), l_preExp.clone(), l_argStr.clone())?;
            l_builtinFunctionName = builtinFunctionNameXml(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_funName = underscorePathXml(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_retType = fun_368(Tpl::emptyTxt.clone(), i_attr_builtin.clone(), i_attr_ty.clone(), l_funName.clone())?;
            (l_retVar, a_varDecls) = fun_369(Tpl::emptyTxt.clone(), i_attr_ty.clone(), a_varDecls.clone(), l_retType.clone())?;
            txt = fun_371(txt.clone(), i_exp.clone(), l_builtinFunctionName.clone(), l_result.clone(), l_funName.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn builtinFunctionNameXml(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "DIVISION" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Div")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "ADDITION" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Add")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "SUBTRACTION" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sub")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "POWER" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Pow")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sin" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sin")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "cos" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Cos")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "tan" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tan")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "asin" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Asin")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "acos" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Acos")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "atan" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Atan")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sinh")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Cosh")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tanh")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Exp")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "log" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Log")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Log10")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sqrt")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "atan2" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Atan2")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Abs")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sign")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "min" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Min")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "max" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Max")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NoEvent")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "array" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Array")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Sample")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Smooth")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "homotopy" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Homotopy")).clone() }))?;
            txt.clone()
        },
        (txt, i_path) => {
            let mut txt = (*txt).clone();
            txt = dotPathXml(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_374(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_vrest: Arc<metamodelica::List<ArcStr>>, mut in_a_erest: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_exp: Tpl::Text, mut in_a_v: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_vrest.clone(), in_a_erest.clone(), in_a_exp.clone(), in_a_v.clone())) {
        (txt, false, a_varDecls, a_preExp, a_context, a_vrest, a_erest, a_exp, a_v) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_v.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt, a_preExp, a_varDecls) = daeExpTailCallXml(txt.clone(), a_erest.clone(), a_vrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, a_context, a_vrest, a_erest, _, _) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpTailCallXml(txt.clone(), a_erest.clone(), a_vrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_375(mut in_txt: Tpl::Text, mut in_a_e: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_vrest: Arc<metamodelica::List<ArcStr>>, mut in_a_erest: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_exp: Tpl::Text, mut in_a_v: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_e.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_vrest.clone(), in_a_erest.clone(), in_a_exp.clone(), in_a_v.clone())) {
        (txt, Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }, componentRef: i_cr }, a_varDecls, a_preExp, a_context, a_vrest, a_erest, a_exp, a_v) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt_0 = crefStrXml(Tpl::emptyTxt.clone(), i_cr.clone())?;
            ret_1 = stringEq((a_v.clone()).clone(), (Tpl::textString(txt_0.clone())?).clone());
            (txt, a_varDecls, a_preExp) = fun_374(txt.clone(), ret_1.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), a_vrest.clone(), a_erest.clone(), a_exp.clone(), (a_v.clone()).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, a_context, a_vrest, a_erest, a_exp, a_v) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_v.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt, a_preExp, a_varDecls) = daeExpTailCallXml(txt.clone(), a_erest.clone(), a_vrest.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_376(mut in_txt: Tpl::Text, mut in_a_vs: Arc<metamodelica::List<ArcStr>>, mut in_a_erest: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_e: Arc<DAE::Exp>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_vs.clone(), in_a_erest.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_e.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: i_vrest }, a_erest, a_varDecls, a_preExp, a_context, a_e) => {
            let mut l_exp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_375(txt.clone(), a_e.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_vrest.clone(), a_erest.clone(), l_exp.clone(), (i_v.clone()).clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, _, a_varDecls, a_preExp, _, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpTailCallXml(mut in_txt: Tpl::Text, mut in_a_es: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_vs: Arc<metamodelica::List<ArcStr>>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_es.clone(), in_a_vs.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: i_erest }, a_vs, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_376(txt.clone(), a_vs.clone(), i_erest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone(), i_e.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpCallBuiltinPrefixXml(mut in_txt: Tpl::Text, mut in_a_builtin: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_builtin.clone()) {
        (mut txt, true) => {
            txt.clone()
        },
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_379(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_379(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpArrayXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ARRAY { array: i_array, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_params: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_params = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_params, a_varDecls, a_preExp) = lm_379(l_params.clone(), i_array.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            l_params = Tpl::popIter(l_params.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Array>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_params.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Array>")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_params.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_381(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut in_a_vars2: Tpl::Text, mut in_a_promote: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text, mut in_a_arrayTypeStr: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_vars2: Tpl::Text;
    let mut out_a_promote: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_vars2, out_a_promote, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_vars2.clone(), in_a_promote.clone(), in_a_context.clone(), in_a_varDecls.clone(), in_a_arrayTypeStr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_vars2, a_promote, _, a_varDecls, _) => {
            (txt.clone(), a_vars2.clone(), a_promote.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_row, tail: rest }, a_vars2, a_promote, a_context, a_varDecls, a_arrayTypeStr) => {
            let mut l_vars: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_vars2 = (*a_vars2).clone();
            let mut a_promote = (*a_promote).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(a_arrayTypeStr.clone())?).clone(), a_varDecls.clone())?;
            (l_vars, a_promote, a_varDecls) = daeExpMatrixRowXml(Tpl::emptyTxt.clone(), i_row.clone(), (Tpl::textString(a_arrayTypeStr.clone())?).clone(), a_context.clone(), a_promote.clone(), a_varDecls.clone())?;
            a_vars2 = Tpl::writeTok(a_vars2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_vars2 = Tpl::writeText(a_vars2.clone(), l_tmp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_vars2, a_promote, a_varDecls) = lm_381(txt.clone(), rest.clone(), a_vars2.clone(), a_promote.clone(), a_context.clone(), a_varDecls.clone(), a_arrayTypeStr.clone())?;
            (txt.clone(), a_vars2.clone(), a_promote.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_vars2, out_a_promote, out_a_varDecls))
}

pub fn daeExpMatrixXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Cons { head: Deref @ metamodelica::List::Nil, tail: Deref @ metamodelica::List::Nil }, .. }, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::List::Nil, .. }, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::MATRIX { matrix: i_m_matrix, ty: i_m_ty, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_tmp: Tpl::Text;
            let mut l_catAlloc: Tpl::Text;
            let mut l_promote: Tpl::Text;
            let mut l_vars2: Tpl::Text;
            let mut l_arrayTypeStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_arrayTypeStr = expTypeArrayXml(Tpl::emptyTxt.clone(), i_m_ty.clone())?;
            l_vars2 = Tpl::emptyTxt.clone();
            l_promote = Tpl::emptyTxt.clone();
            l_catAlloc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_catAlloc, l_vars2, l_promote, a_varDecls) = lm_381(l_catAlloc.clone(), i_m_matrix.clone(), l_vars2.clone(), l_promote.clone(), a_context.clone(), a_varDecls.clone(), l_arrayTypeStr.clone())?;
            l_catAlloc = Tpl::popIter(l_catAlloc.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_promote.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_catAlloc.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayTypeStr.clone())?).clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn lm_383(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_varLstStr: Tpl::Text, mut in_a_arrayTypeStr: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varLstStr: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varLstStr, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varLstStr.clone(), in_a_arrayTypeStr.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varLstStr, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varLstStr.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_varLstStr, a_arrayTypeStr, a_varDecls, a_preExp, a_context) => {
            let mut l_tmp: Tpl::Text;
            let mut l_expVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varLstStr = (*a_varLstStr).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (a_arrayTypeStr.clone()).clone(), a_varDecls.clone())?;
            a_varLstStr = Tpl::writeTok(a_varLstStr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_varLstStr = Tpl::writeText(a_varLstStr.clone(), l_tmp.clone())?;
            txt = Tpl::writeText(txt.clone(), l_expVar.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varLstStr, a_varDecls, a_preExp) = lm_383(txt.clone(), rest.clone(), a_varLstStr.clone(), (a_arrayTypeStr.clone()).clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varLstStr.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varLstStr, out_a_varDecls, out_a_preExp))
}

pub fn daeExpMatrixRowXml(mut txt: Tpl::Text, mut a_row: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_arrayTypeStr: ArcStr, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_preExp2: Tpl::Text;
    let mut l_varLstStr: Tpl::Text;
    l_varLstStr = Tpl::emptyTxt.clone();
    l_preExp2 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_preExp2, l_varLstStr, out_a_varDecls, out_a_preExp) = lm_383(l_preExp2.clone(), a_row.clone(), l_varLstStr.clone(), (a_arrayTypeStr.clone()).clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
    l_preExp2 = Tpl::popIter(l_preExp2.clone())?;
    l_preExp2 = Tpl::writeTok(l_preExp2.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_a_preExp = Tpl::writeText(out_a_preExp.clone(), l_preExp2.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_varLstStr.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_385(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_step.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Some(i_stepExp), a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_stepExp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

pub fn daeExpRangeXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::RANGE { step: i_step, stop: i_stop, start: i_start, ty: i_ty }, a_context, a_preExp, a_varDecls) => {
            let mut l_step__exp: Tpl::Text;
            let mut l_tmp: Tpl::Text;
            let mut l_stop__exp: Tpl::Text;
            let mut l_start__exp: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_ty__str = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_start__exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_start.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_stop__exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_stop.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_tmp, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_ty__str.clone())?).clone(), a_varDecls.clone())?;
            (l_step__exp, a_varDecls, a_preExp) = fun_385(Tpl::emptyTxt.clone(), i_step.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Range>\n")).clone() }))?;
            a_preExp = Tpl::pushBlock(a_preExp.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_start__exp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_step__exp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_stop__exp.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::popBlock(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Range>")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tmp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_387(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_preExp: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_expVar: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_preExp.clone(), in_a_exp.clone(), in_a_varDecls.clone(), in_a_expVar.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }, a_preExp, a_exp, a_varDecls, a_expVar) => {
            let mut l_from: Tpl::Text;
            let mut l_to: Tpl::Text;
            let mut l_tvar: Tpl::Text;
            let mut l_arrayTypeStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_arrayTypeStr = expTypeArrayXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            (l_tvar, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(l_arrayTypeStr.clone())?).clone(), a_varDecls.clone())?;
            l_to = expTypeShortXml(Tpl::emptyTxt.clone(), i_ty.clone())?;
            l_from = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), a_exp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cast_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_from.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_array_to_")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_to.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), a_expVar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_tvar.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_tvar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_preExp, _, a_varDecls, a_expVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_expVar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /* could not cast, using the variable as it is */")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpCastXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::CAST { ty: i_ty, exp: i_exp }, a_context, a_preExp, a_varDecls) => {
            let mut l_expVar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expVar, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_preExp, a_varDecls) = fun_387(txt.clone(), i_ty.clone(), a_preExp.clone(), i_exp.clone(), a_varDecls.clone(), l_expVar.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeSubscriptXML(mut in_txt: Tpl::Text, mut in_a_sub: Arc<DAE::Subscript>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sub.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3580, 14), (literal!("non INDEX(_) (i.e., slice) subscripts probably should not reach here. Check indexedAssign template.")).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_390(mut in_txt: Tpl::Text, mut in_a_inExp: Arc<DAE::Exp>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_inExp.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: i_idx, tail: Deref @ metamodelica::List::Nil }, exp: i_e }, a_varDecls, a_preExp, a_context) => {
            let mut l_idx1: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_e1, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_idx1, a_preExp, a_varDecls) = daeSubscriptXML(Tpl::emptyTxt.clone(), i_idx.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_idx1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /* DAE.ASUB */")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_391(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_res: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_res.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_res, a_context) => {
            let mut x_i1: i32 = 0;
            let mut l_v: Tpl::Text;
            let mut l_casePreExp: Tpl::Text;
            let mut l_caseVarDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_caseVarDecls = Tpl::emptyTxt.clone();
            l_casePreExp = Tpl::emptyTxt.clone();
            (l_v, l_casePreExp, l_caseVarDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), l_casePreExp.clone(), l_caseVarDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(": {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_caseVarDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_casePreExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("break;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_391(txt.clone(), rest.clone(), a_res.clone(), a_context.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_392(mut in_txt: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_ecr_ty: Arc<DAE::Type>, mut in_a_arrName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_context.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_subs.clone(), in_a_ecr_ty.clone(), in_a_arrName.clone())) {
        (txt, SimCodeFunction::Context::FUNCTION_CONTEXT { cref_prefix: _, .. }, a_varDecls, a_preExp, _, _, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_arrName.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_context, a_varDecls, a_preExp, a_subs, a_ecr_ty, a_arrName) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = arrayScalarRhsXml(txt.clone(), a_ecr_ty.clone(), a_subs.clone(), (Tpl::textString(a_arrName.clone())?).clone(), i_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Asub array scalar RHS")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_393(mut in_txt: Tpl::Text, mut in_a_inExp: Arc<DAE::Exp>, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_inExp.clone(), in_a_preExp.clone(), in_a_context.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ASUB { exp: i_exp @ Deref @ DAE::Exp::ASUB { exp: _, .. }, .. }, a_preExp, _, a_varDecls) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Nested array subscripting *should* have been handled by the routine creating the asub, but for some reason it was not: ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3600, 11), (Tpl::textString(txt_0.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: i_idx, tail: Deref @ metamodelica::List::Nil }, exp: i_exp @ Deref @ DAE::Exp::ARRAY { array: i_exp_array, scalar: true, .. } }, a_preExp, a_context, a_varDecls) => {
            let mut l_expl: Tpl::Text;
            let mut l_idx1: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut l_res: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_2 = expTypeFromExpModelicaXml(Tpl::emptyTxt.clone(), i_exp.clone())?;
            (l_res, a_varDecls) = tempDeclXml(Tpl::emptyTxt.clone(), (Tpl::textString(txt_2.clone())?).clone(), a_varDecls.clone())?;
            l_res = Tpl::writeTok(l_res.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" asub tmp test")).clone() }))?;
            (l_idx1, a_preExp, a_varDecls) = daeSubscriptXML(Tpl::emptyTxt.clone(), i_idx.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            l_expl = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_expl = lm_391(l_expl.clone(), i_exp_array.clone(), l_res.clone(), a_context.clone())?;
            l_expl = Tpl::popIter(l_expl.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("switch (")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_idx1.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") { /* ASUB */\n")).clone() }))?;
            a_preExp = Tpl::writeText(a_preExp.clone(), l_expl.clone())?;
            a_preExp = Tpl::softNewLine(a_preExp.clone())?;
            a_preExp = Tpl::writeTok(a_preExp.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  assert(NULL == \"index out of bounds\");\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, exp: i_exp @ Deref @ DAE::Exp::RANGE { ty: _, .. } }, a_preExp, _, a_varDecls) => {
            let mut txt_5: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_5 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ASUB_EASY_CASE ")).clone() }))?;
            txt_5 = ExpressionDumpTpl::dumpExp(txt_5.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3629, 11), (Tpl::textString(txt_5.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { sub: i_subs, exp: i_ecr @ Deref @ DAE::Exp::CREF { ty: i_ecr_ty, .. } }, a_preExp, a_context, a_varDecls) => {
            let mut ret_7: Arc<DAE::Exp>;
            let mut l_arrName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_7 = SimCodeFunctionUtil::buildCrefExpFromSubs(i_ecr.clone(), i_subs.clone())?;
            (l_arrName, a_preExp, a_varDecls) = daeExpCrefRhsXml(Tpl::emptyTxt.clone(), ret_7.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt, a_varDecls, a_preExp) = fun_392(txt.clone(), a_context.clone(), a_varDecls.clone(), a_preExp.clone(), i_subs.clone(), i_ecr_ty.clone(), l_arrName.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ASUB { sub: _, exp: i_e }, a_preExp, a_context, a_varDecls) => {
            let mut l_exp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_exp, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_e.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_preExp, _, a_varDecls) => {
            let mut txt_9: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_9 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OTHER_ASUB ")).clone() }))?;
            txt_9 = ExpressionDumpTpl::dumpExp(txt_9.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3644, 11), (Tpl::textString(txt_9.clone())?).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_394(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_inExp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_inExp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ "metatype", a_inExp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_preExp) = fun_390(txt.clone(), a_inExp.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, a_inExp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = fun_393(txt.clone(), a_inExp.clone(), a_preExp.clone(), a_context.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpAsubXml(mut txt: Tpl::Text, mut a_inExp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut str_1: ArcStr = arcstr::literal!("");
    let mut txt_0: Tpl::Text;
    txt_0 = expTypeFromExpShortXml(Tpl::emptyTxt.clone(), a_inExp.clone())?;
    str_1 = (Tpl::textString(txt_0.clone())?).clone();
    (out_txt, out_a_preExp, out_a_varDecls) = fun_394(txt.clone(), (str_1.clone()).clone(), a_inExp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpASubIndexXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }, _, a_preExp, a_varDecls) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = SimCodeFunctionUtil::incrementInt(i_integer.clone(), -1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }, _, a_preExp, a_varDecls) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            ret_1 = SimCodeFunctionUtil::incrementInt(i_index.clone(), -1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, i_exp, a_context, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_preExp, a_varDecls) = daeExpXml(txt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_397(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_cr_componentRef, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Pre>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = crefXml(txt.clone(), i_cr_componentRef.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Pre>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ASUB { sub: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil }, exp: Deref @ DAE::Exp::CREF { componentRef: _, .. } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"case ASUB(exp = cr as CREF(__), sub = {sub_exp}) is not yet implemented\"")).clone() }))?;
            txt.clone()
        },
        (txt, i_exp) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Code generation does not support pre(")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(txt_0.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3671, 11), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn daeExpCallPreXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_397(txt.clone(), a_exp.clone())?;
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpSizeXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::SIZE { sz: Some(i_dim), exp: i_exp @ Deref @ DAE::Exp::CREF { componentRef: _, .. } }, a_context, a_preExp, a_varDecls) => {
            let mut l_dimPart: Tpl::Text;
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_expPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            (l_dimPart, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_dim.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<exp:Size>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dimPart.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp:Size>")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("size(X) not implemented")).clone() }))?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpBoxXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::BOX { exp: i_exp_exp }, a_context, a_preExp, a_varDecls) => {
            let mut l_res: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_res, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn daeExpUnboxXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_context: SimCodeFunction::Context, mut in_a_preExp: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_preExp, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_context.clone(), in_a_preExp.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ DAE::Exp::UNBOX { exp: i_exp_exp, .. }, a_context, a_preExp, a_varDecls) => {
            let mut l_res: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preExp = (*a_preExp).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_res, a_preExp, a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), i_exp_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::writeText(txt.clone(), l_res.clone())?;
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_preExp, a_varDecls) => {
            (txt.clone(), a_preExp.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

fn fun_402(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { index: _, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_txt
}

pub fn daeExpSharedLiteralXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> (Tpl::Text, Tpl::Text, Tpl::Text) {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    out_txt = fun_402(txt.clone(), a_exp.clone());
    out_a_preExp = a_preExp.clone();
    out_a_varDecls = a_varDecls.clone();
    (out_txt, out_a_preExp, out_a_varDecls)
}

fn lm_404(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecls, a_preExp, _) => {
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_sub, tail: rest }, a_varDecls, a_preExp, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (txt, a_preExp, a_varDecls) = daeSubscriptXML(txt.clone(), i_sub.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_varDecls, a_preExp) = lm_404(txt.clone(), rest.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
            (txt.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_preExp))
}

fn fun_405(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_dimsValuesStr: Tpl::Text, mut in_a_arrName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_dimsValuesStr.clone(), in_a_arrName.clone())) {
        (txt, Deref @ "metatype_array", a_dimsValuesStr, a_arrName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("arrayGet(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") /*arrayScalarRhs*/")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_dimsValuesStr, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" wrong LHS\n")).clone(), (literal!("    <exp:ArraySubscripts>\n")).clone(), (literal!("      <exp:IndexExpression>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
            txt = Tpl::writeText(txt.clone(), a_dimsValuesStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("      </exp:IndexExpression>\n")).clone(), (literal!("    </exp:ArraySubscripts>\n")).clone(), (literal!("  </exp:QualifiedNamepart>\n")).clone(), (literal!("</exp:QualifiedName>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn arrayScalarRhsXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_subs: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut a_arrName: ArcStr, mut a_context: SimCodeFunction::Context, mut a_preExp: Tpl::Text, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut str_4: ArcStr = arcstr::literal!("");
    let mut l_dimsValuesStr: Tpl::Text;
    let mut ret_2: i32 = 0;
    let mut l_dimsLenStr: Tpl::Text;
    let mut l_arrayType: Tpl::Text;
    l_arrayType = expTypeArrayXml(Tpl::emptyTxt.clone(), a_ty.clone())?;
    ret_2 = (a_subs.clone().len() as i32);
    l_dimsLenStr = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2.clone())).clone())?;
    l_dimsValuesStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_dimsValuesStr, out_a_varDecls, out_a_preExp) = lm_404(l_dimsValuesStr.clone(), a_subs.clone(), a_varDecls.clone(), a_preExp.clone(), a_context.clone())?;
    l_dimsValuesStr = Tpl::popIter(l_dimsValuesStr.clone())?;
    str_4 = (Tpl::textString(l_arrayType.clone())?).clone();
    out_txt = fun_405(txt.clone(), (str_4.clone()).clone(), l_dimsValuesStr.clone(), (a_arrName.clone()).clone())?;
    Ok((out_txt, out_a_preExp, out_a_varDecls))
}

pub fn outDeclXml(mut txt: Tpl::Text, mut a_ty: ArcStr, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_newVar: Tpl::Text;
    l_newVar = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out")).clone() }))?;
    out_a_varDecls = Tpl::writeStr(a_varDecls.clone(), (a_ty.clone()).clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_a_varDecls = Tpl::writeText(out_a_varDecls.clone(), l_newVar.clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::writeText(txt.clone(), l_newVar.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_408(mut in_txt: Tpl::Text, mut in_a_ty: ArcStr, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ "modelica_metatype", a_varDecls) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmpMeta[")).clone() }))?;
            ret_0 = System::tmpTickIndex(1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "metamodelica_string", a_varDecls) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmpMeta[")).clone() }))?;
            ret_1 = System::tmpTickIndex(1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "metamodelica_string_const", a_varDecls) => {
            let mut ret_2: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmpMeta[")).clone() }))?;
            ret_2 = System::tmpTickIndex(1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            (txt.clone(), a_varDecls.clone())
        },
        (txt, i_ty, a_varDecls) => {
            let mut ret_4: i32 = 0;
            let mut l_newVarIx: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_newVarIx = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            ret_4 = System::tmpTick();
            l_newVarIx = Tpl::writeStr(l_newVarIx.clone(), (intString(ret_4.clone())).clone())?;
            a_varDecls = Tpl::writeStr(a_varDecls.clone(), (i_ty.clone()).clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_varDecls = Tpl::writeText(a_varDecls.clone(), l_newVarIx.clone())?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            a_varDecls = Tpl::writeTok(a_varDecls.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_newVarIx.clone())?;
            (txt.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls))
}

pub fn tempDeclXml(mut txt: Tpl::Text, mut a_ty: ArcStr, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_newVar: Tpl::Text;
    (l_newVar, out_a_varDecls) = fun_408(Tpl::emptyTxt.clone(), (a_ty.clone()).clone(), a_varDecls.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_newVar.clone())?;
    Ok((out_txt, out_a_varDecls))
}

pub fn tempDeclConstXml(mut txt: Tpl::Text, mut a_ty: ArcStr, mut a_val: ArcStr, mut a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut ret_1: i32 = 0;
    let mut l_newVar: Tpl::Text;
    l_newVar = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
    ret_1 = System::tmpTick();
    l_newVar = Tpl::writeStr(l_newVar.clone(), (intString(ret_1.clone())).clone())?;
    out_a_varDecls = Tpl::writeStr(a_varDecls.clone(), (a_ty.clone()).clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_a_varDecls = Tpl::writeText(out_a_varDecls.clone(), l_newVar.clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
    out_a_varDecls = Tpl::writeStr(out_a_varDecls.clone(), (a_val.clone()).clone())?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    out_a_varDecls = Tpl::writeTok(out_a_varDecls.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::writeText(txt.clone(), l_newVar.clone())?;
    Ok((out_txt, out_a_varDecls))
}

fn fun_411(mut in_txt: Tpl::Text, mut in_a_instDims: Arc<metamodelica::List<Arc<DAE::Dimension>>>, mut in_a_var_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_instDims.clone(), in_a_var_ty.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_var_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeArrayIfXml(txt.clone(), a_var_ty.clone())?;
            txt.clone()
        },
        (txt, _, a_var_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeArrayXml(txt.clone(), a_var_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn varTypeXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { ty: i_var_ty, instDims: i_instDims, .. }) => {
            let mut txt = (*txt).clone();
            txt = fun_411(txt.clone(), i_instDims.clone(), i_var_ty.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn varTypeBoxedXml(mut in_txt: Tpl::Text, mut in_a_var: Arc<SimCodeFunction::Variable::Variable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ SimCodeFunction::Variable::VARIABLE { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_fnptr")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn expTypeRWXml(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_INT")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_REAL")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_STRING")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_BOOL")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_INT")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeRWXml(txt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_ARRAY")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_RECORD")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_MMC")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TYPE_DESC_MMC")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_415(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MetaType")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn expTypeShortXml(mut in_txt: Tpl::Text, mut in_a_type: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Config::acceptMetaModelicaGrammar()?;
            txt = fun_415(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Complex")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_complexClassType, .. }) => {
            let mut ret_1: Arc<Absyn::Path>;
            let mut txt = (*txt).clone();
            ret_1 = ClassInfUtil::getStateName(i_complexClassType.clone());
            txt = underscorePathXml(txt.clone(), ret_1.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MetaType")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MetaType")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fnptr")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_UNKNOWN) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Complex")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ANYTYPE { anyClassType: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Complex")).clone() }))?;
            txt.clone()
        },
        (txt, i_type) => {
            let mut txt_2: Tpl::Text;
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expTypeShortXml:")).clone() }))?;
            ret_2 = (TypesDump::unparseType(i_type.clone())?).clone();
            txt_2 = Tpl::writeStr(txt_2.clone(), (ret_2.clone()).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3835, 14), (Tpl::textString(txt_2.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_417(mut in_txt: Tpl::Text, mut in_a_array: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_array.clone(), in_a_ty.clone())) {
        (txt, true, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeArrayXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, false, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeModelicaXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn expTypeXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_array: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_417(txt.clone(), a_array.clone(), a_ty.clone())?;
    Ok(out_txt)
}

pub fn expTypeModelicaXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFlagXml(txt.clone(), a_ty.clone(), 2)?;
    Ok(out_txt)
}

pub fn expTypeArrayXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFlagXml(txt.clone(), a_ty.clone(), 3)?;
    Ok(out_txt)
}

pub fn expTypeArrayIfXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFlagXml(txt.clone(), a_ty.clone(), 4)?;
    Ok(out_txt)
}

pub fn expTypeFromExpShortXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 1)?;
    Ok(out_txt)
}

pub fn expTypeFromExpModelicaXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 2)?;
    Ok(out_txt)
}

pub fn expTypeFromExpArrayXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 3)?;
    Ok(out_txt)
}

pub fn expTypeFromExpArrayIfXml(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = expTypeFromExpFlagXml(txt.clone(), a_exp.clone(), 4)?;
    Ok(out_txt)
}

fn fun_426(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_complexClassType, .. }) => {
            let mut ret_0: Arc<Absyn::Path>;
            let mut txt = (*txt).clone();
            ret_0 = ClassInfUtil::getStateName(i_complexClassType.clone());
            txt = underscorePathXml(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_427(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, i_ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt = (*txt).clone();
            txt = fun_426(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_428(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_ty.clone(), 2)?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_429(mut in_txt: Tpl::Text, mut in_a_flag: i32, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_flag.clone(), in_a_ty.clone())) {
        (txt, 1, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, 2, a_ty) => {
            let mut txt = (*txt).clone();
            txt = fun_427(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, 3, a_ty) => {
            let mut txt = (*txt).clone();
            txt = expTypeShortXml(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, 4, a_ty) => {
            let mut txt = (*txt).clone();
            txt = fun_428(txt.clone(), a_ty.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn expTypeFlagXml(mut txt: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_429(txt.clone(), a_flag.clone(), a_ty.clone())?;
    Ok(out_txt)
}

fn fun_431(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 8) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_432(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("real")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_real")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_433(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("string")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_string")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_434(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_435(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_flag.clone()) {
        (mut txt, false, mut a_flag) => {
            txt = fun_433(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_flag) => {
            txt = fun_434(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_436(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_437(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 8) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int")).clone() }))?;
            txt.clone()
        },
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integer")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_438(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_439(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_440(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_441(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_442(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn expTypeFromExpFlagXml(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_flag.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_431(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_432(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: _ }, a_flag) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Config::acceptMetaModelicaGrammar()?;
            txt = fun_435(txt.clone(), ret_0.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_436(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_437(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BINARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::UNARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LBINARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LUNARY { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RELATION { operator: i_e_operator, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFromOpFlagXml(txt.clone(), i_e_operator.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::IFEXP { expThen: i_expThen, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFromExpFlagXml(txt.clone(), i_expThen.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: i_attr_ty, .. }, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_attr_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATRIX { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RANGE { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CAST { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CODE { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, i_c @ Deref @ DAE::Exp::ASUB { exp: _, .. }, a_flag) => {
            let mut ret_1: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut txt = (*txt).clone();
            ret_1 = Expression::r#typeof(i_c.clone())?;
            txt = expTypeFlagXml(txt.clone(), ret_1.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ DAE::Exp::REDUCTION { reductionInfo: _, .. }, a_flag) => {
            let mut ret_2: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut txt = (*txt).clone();
            ret_2 = Expression::r#typeof(i_exp.clone())?;
            txt = expTypeFlagXml(txt.clone(), ret_2.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::BOX { exp: _ }, a_flag) => {
            let mut ret_3: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut txt = (*txt).clone();
            ret_3 = Expression::r#typeof(i_e.clone())?;
            txt = expTypeFlagXml(txt.clone(), ret_3.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::CONS { car: _, .. }, a_flag) => {
            let mut ret_4: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut txt = (*txt).clone();
            ret_4 = Expression::r#typeof(i_e.clone())?;
            txt = expTypeFlagXml(txt.clone(), ret_4.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::LIST { valList: _ }, a_flag) => {
            let mut ret_5: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut txt = (*txt).clone();
            ret_5 = Expression::r#typeof(i_e.clone())?;
            txt = expTypeFlagXml(txt.clone(), ret_5.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::SIZE { exp: _, .. }, a_flag) => {
            let mut ret_6: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut txt = (*txt).clone();
            ret_6 = Expression::r#typeof(i_e.clone())?;
            txt = expTypeFlagXml(txt.clone(), ret_6.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_TUPLE { listExp: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_438(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_439(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATCHEXPRESSION { matchType: _, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_440(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::METARECORDCALL { path: _, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_441(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BOX { exp: _ }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = fun_442(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::UNBOX { ty: i_c_ty, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFlagXml(txt.clone(), i_c_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { exp: i_c_exp, .. }, a_flag) => {
            let mut txt = (*txt).clone();
            txt = expTypeFromExpFlagXml(txt.clone(), i_c_exp.clone(), a_flag.clone())?;
            txt.clone()
        },
        (txt, i_exp, _) => {
            let mut txt_7: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_7 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expTypeFromExpFlag:")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpExp(txt_7.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = error(txt.clone(), Tpl::sourceInfo((literal!("CodegenXML.tpl")).clone(), 3951, 14), (Tpl::textString(txt_7.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_444(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_445(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_446(mut in_txt: Tpl::Text, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flag.clone()) {
        (mut txt, 1) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn expTypeFromOpFlagXml(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator, mut in_a_flag: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone(), in_a_flag.clone()) {
        (mut txt, DAE::Operator::ADD { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARRAY_SCALAR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_SCALAR_ARRAY { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR2 { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESS { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESSEQ { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATER { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::EQUAL { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NEQUAL { ty: ref i_o_ty }, mut a_flag) => {
            txt = expTypeFlagXml(txt.clone(), i_o_ty.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::AND { ty: _ }, mut a_flag) => {
            txt = fun_444(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::OR { ty: _ }, mut a_flag) => {
            txt = fun_445(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NOT { ty: _ }, mut a_flag) => {
            txt = fun_446(txt.clone(), a_flag.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expTypeFromOpFlag:ERROR")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dimensionXml(mut in_txt: Tpl::Text, mut in_a_d: Arc<DAE::Dimension>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_d.clone())) {
        (txt, Deref @ DAE::Dimension::DIM_INTEGER { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_ENUM { size: i_size, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_size.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_UNKNOWN) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INVALID_DIMENSION")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn assertCommonXml(mut txt: Tpl::Text, mut a_condition: Arc<DAE::Exp>, mut a_message: Arc<DAE::Exp>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text, mut a_info: SourceInfo) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut l_msgVar: Tpl::Text;
    let mut l_condVar: Tpl::Text;
    let mut l_preExpMsg: Tpl::Text;
    let mut l_preExpCond: Tpl::Text;
    l_preExpCond = Tpl::emptyTxt.clone();
    l_preExpMsg = Tpl::emptyTxt.clone();
    (l_condVar, l_preExpCond, out_a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_condition.clone(), a_context.clone(), l_preExpCond.clone(), a_varDecls.clone())?;
    (l_msgVar, l_preExpMsg, out_a_varDecls) = daeExpXml(Tpl::emptyTxt.clone(), a_message.clone(), a_context.clone(), l_preExpMsg.clone(), out_a_varDecls.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<fun:Assertion>\n")).clone(), (literal!("  <fun:Condition>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_condVar.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Condition>\n")).clone(), (literal!("  <fun:Message>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_msgVar.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </fun:Message>\n")).clone(), (literal!("</fun:Assertion>")).clone()], lastHasNewLine: false }))?;
    Ok((out_txt, out_a_varDecls))
}

pub fn error(mut txt: Tpl::Text, mut a_srcInfo: SourceInfo, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: ArcStr = arcstr::literal!("");
    Tpl::addSourceTemplateError((a_errMessage.clone()).clone(), a_srcInfo.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#error \"")).clone()], lastHasNewLine: false }))?;
    ret_0 = (Error::infoStr(a_srcInfo.clone())?).clone();
    out_txt = Tpl::writeStr(out_txt.clone(), (ret_0.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    Ok(out_txt)
}

