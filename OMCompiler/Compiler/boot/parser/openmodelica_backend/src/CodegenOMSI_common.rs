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
use crate::CodegenCFunctions;
use crate::CodegenFMU;
use crate::CodegenOMSIC_Equations;
use crate::CodegenUtil;
use crate::CodegenUtilSimulation;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn generateFMUModelDescriptionFile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_guid: ArcStr, mut a_FMUVersion: ArcStr, mut a_FMUType: ArcStr, mut a_sourceFiles: Arc<metamodelica::List<ArcStr>>, mut a_fileName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_content: Tpl::Text;
    l_content = CodegenFMU::fmuModelDescriptionFile(Tpl::emptyTxt.clone(), a_simCode.clone(), (a_guid.clone()).clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone(), a_sourceFiles.clone())?;
    Tpl::textFile(l_content.clone(), (a_fileName.clone()).clone())?;
    out_txt = txt.clone();
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_modelNameOMSIC: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_modelNameOMSIC.clone())) {
        (txt, SimCode::SimCode { omsiData: Some(SimCode::OMSIData { initialization: i_initialization @ Deref @ SimCode::OMSIFunction { equations: _, .. }, simulation: i_simulation @ Deref @ SimCode::OMSIFunction { equations: _, .. } }), fullPathPrefix: i_fullPathPrefix, fileNamePrefix: i_fileNamePrefix, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }, a_modelNameOMSIC) => {
            let mut txt_4: Tpl::Text;
            let mut l_content_1: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut l_content: Tpl::Text;
            let mut l_modelNamePrefix: Tpl::Text;
            l_modelNamePrefix = Tpl::writeText(Tpl::emptyTxt.clone(), a_modelNameOMSIC.clone())?;
            l_content = generateOmsiFunctionCode(Tpl::emptyTxt.clone(), i_simulation.clone(), (Tpl::textString(l_modelNamePrefix.clone())?).clone(), (literal!("")).clone(), (literal!("sim_eqns")).clone())?;
            txt_2 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fullPathPrefix.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt_2 = Tpl::writeStr(txt_2.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns.c")).clone() }))?;
            Tpl::textFile(l_content.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            l_content_1 = generateOmsiFunctionCode(Tpl::emptyTxt.clone(), i_initialization.clone(), (Tpl::textString(l_modelNamePrefix.clone())?).clone(), (literal!("")).clone(), (literal!("init_eqns")).clone())?;
            txt_4 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fullPathPrefix.clone()).clone())?;
            txt_4 = Tpl::writeTok(txt_4.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt_4 = Tpl::writeStr(txt_4.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_4 = Tpl::writeTok(txt_4.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_eqns.c")).clone() }))?;
            Tpl::textFile(l_content_1.clone(), (Tpl::textString(txt_4.clone())?).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateEquationsCode(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_FileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_modelNameOMSIC: Tpl::Text;
    ret_1 = (System::makeC89Identifier((a_FileNamePrefix.clone()).clone())).clone();
    l_modelNameOMSIC = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    out_txt = fun_54(txt.clone(), a_simCode.clone(), l_modelNameOMSIC.clone())?;
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_a_nAlgebraicSystems: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_nAlgebraicSystems.clone()) {
        (mut txt, 0) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("new_status = omsi_ok;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_omsiFunction: Arc<SimCode::OMSIFunction>, mut in_a_functionCall: Tpl::Text, mut in_a_omsiName: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_evaluationCode: Tpl::Text, mut in_a_initializationCode: Tpl::Text, mut in_a_headerFileName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_omsiFunction.clone(), in_a_functionCall.clone(), in_a_omsiName.clone(), in_a_FileNamePrefix.clone(), in_a_evaluationCode.clone(), in_a_initializationCode.clone(), in_a_headerFileName.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { nAlgebraicSystems: i_nAlgebraicSystems, .. }, a_functionCall, a_omsiName, a_FileNamePrefix, a_evaluationCode, a_initializationCode, a_headerFileName) => {
            let mut txt = (*txt).clone();
            txt = insertCopyrightOpenModelica(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  /* All Equations Code */\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_headerFileName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".h\"\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#if defined(__cplusplus)\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("/* Instantiation of omsi_function_t */\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_initializationCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* Evaluation functions for each equation */\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_evaluationCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("/* Equations evaluation */\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_allEqns(struct omsi_function_t* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(", const omsi_values* model_vars_and_params, void* data){\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("  /* Variables */\n")).clone(), (literal!("  omsi_status status, new_status;\n")).clone(), (literal!("\n")).clone(), (literal!("  status = omsi_ok;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_56(txt.clone(), i_nAlgebraicSystems.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_functionCall.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("return status;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateOmsiFunctionCode(mut txt: Tpl::Text, mut a_omsiFunction: Arc<SimCode::OMSIFunction>, mut a_FileNamePrefix: ArcStr, mut a_modelFunctionnamePrefixStr: ArcStr, mut a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut txt_12: Tpl::Text;
    let mut l_headerFileContent: Tpl::Text;
    let mut l_headerFileName: Tpl::Text;
    let mut l_0__: Tpl::Text;
    let mut l_initializationCode: Tpl::Text;
    let mut ret_7: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fullPathPrefix: Tpl::Text;
    let mut ret_5: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fileNamePrefix: Tpl::Text;
    let mut l_functionPrototypes: Tpl::Text;
    let mut l_functionCall: Tpl::Text;
    let mut l_evaluationCode: Tpl::Text;
    let mut l_includes: Tpl::Text;
    l_includes = Tpl::emptyTxt.clone();
    l_evaluationCode = Tpl::emptyTxt.clone();
    l_functionCall = Tpl::emptyTxt.clone();
    l_functionPrototypes = Tpl::emptyTxt.clone();
    ret_5 = SimCodeUtil::getSimCode()?;
    l_fileNamePrefix = CodegenUtilSimulation::fileNamePrefix(Tpl::emptyTxt.clone(), ret_5.clone())?;
    ret_7 = SimCodeUtil::getSimCode()?;
    l_fullPathPrefix = CodegenUtilSimulation::fullPathPrefix(Tpl::emptyTxt.clone(), ret_7.clone())?;
    (l_initializationCode, l_functionPrototypes, l_includes) = generateInitalizationOMSIFunction(Tpl::emptyTxt.clone(), a_omsiFunction.clone(), (literal!("allEqns")).clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), l_functionPrototypes.clone(), l_includes.clone(), false, (a_omsiName.clone()).clone())?;
    (l_0__, l_includes, l_evaluationCode, l_functionCall, _, l_functionPrototypes) = generateOmsiFunctionCode_inner(Tpl::emptyTxt.clone(), a_omsiFunction.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (a_omsiName.clone()).clone(), l_includes.clone(), l_evaluationCode.clone(), l_functionCall.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
    l_functionPrototypes = Tpl::writeTok(l_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_status ")).clone() }))?;
    l_functionPrototypes = Tpl::writeStr(l_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone())?;
    l_functionPrototypes = Tpl::writeTok(l_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    l_functionPrototypes = Tpl::writeStr(l_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
    l_functionPrototypes = Tpl::writeTok(l_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_allEqns(struct omsi_function_t* simulation, const omsi_values* model_vars_and_params, void* data);")).clone() }))?;
    l_functionPrototypes = Tpl::writeTok(l_functionPrototypes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    l_headerFileName = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefix.clone())?;
    l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    l_headerFileName = Tpl::writeStr(l_headerFileName.clone(), (a_omsiName.clone()).clone())?;
    (l_headerFileContent, l_includes, l_functionPrototypes) = generateCodeHeader(Tpl::emptyTxt.clone(), (a_FileNamePrefix.clone()).clone(), l_includes.clone(), (Tpl::textString(l_headerFileName.clone())?).clone(), l_functionPrototypes.clone())?;
    txt_12 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fullPathPrefix.clone())?;
    txt_12 = Tpl::writeTok(txt_12.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
    txt_12 = Tpl::writeText(txt_12.clone(), l_headerFileName.clone())?;
    txt_12 = Tpl::writeTok(txt_12.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
    Tpl::textFile(l_headerFileContent.clone(), (Tpl::textString(txt_12.clone())?).clone())?;
    out_txt = fun_57(txt.clone(), a_omsiFunction.clone(), l_functionCall.clone(), (a_omsiName.clone()).clone(), (a_FileNamePrefix.clone()).clone(), l_evaluationCode.clone(), l_initializationCode.clone(), l_headerFileName.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn lastIdentOfPath(mut in_txt: Tpl::Text, mut in_a_modelName: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelName.clone())) {
        (txt, Deref @ Absyn::Path::QUALIFIED { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            txt = lastIdentOfPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = lastIdentOfPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_60(mut in_txt: Tpl::Text, mut in_a_eqsystem: Arc<SimCode::SimEqSystem>, mut in_a_fullPathPrefix: Tpl::Text, mut in_a_fileNamePrefix: Tpl::Text, mut in_a_includes: Tpl::Text, mut in_a_residualCall: Tpl::Text, mut in_a_omsiName: ArcStr, mut in_a_funcCallArgName: ArcStr, mut in_a_functionCall: Tpl::Text, mut in_a_functionPrototypes: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_modelFunctionnamePrefixStr: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_evaluationCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_residualCall: Tpl::Text;
    let mut out_a_functionCall: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    (out_txt, out_a_includes, out_a_residualCall, out_a_functionCall, out_a_functionPrototypes, out_a_evaluationCode) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eqsystem.clone(), in_a_fullPathPrefix.clone(), in_a_fileNamePrefix.clone(), in_a_includes.clone(), in_a_residualCall.clone(), in_a_omsiName.clone(), in_a_funcCallArgName.clone(), in_a_functionCall.clone(), in_a_functionPrototypes.clone(), in_a_context.clone(), in_a_modelFunctionnamePrefixStr.clone(), in_a_FileNamePrefix.clone(), in_a_evaluationCode.clone())) {
        (txt, i_eqsystem @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: _, .. }, _, _, a_includes, a_residualCall, a_omsiName, a_funcCallArgName, a_functionCall, a_functionPrototypes, a_context, a_modelFunctionnamePrefixStr, a_FileNamePrefix, a_evaluationCode) => {
            let mut txt_0: Tpl::Text;
            let mut a_functionCall = (*a_functionCall).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (a_evaluationCode, a_functionPrototypes) = CodegenOMSIC_Equations::generateEquationFunction(a_evaluationCode.clone(), i_eqsystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), a_context.clone(), a_functionPrototypes.clone())?;
            a_evaluationCode = Tpl::writeTok(a_evaluationCode.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_0 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_funcCallArgName.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", model_vars_and_params")).clone() }))?;
            a_functionCall = CodegenOMSIC_Equations::equationCall(a_functionCall.clone(), i_eqsystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (Tpl::textString(txt_0.clone())?).clone(), (a_omsiName.clone()).clone())?;
            a_functionCall = Tpl::writeTok(a_functionCall.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, i_eqsystem @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }, _, _, a_includes, a_residualCall, a_omsiName, a_funcCallArgName, a_functionCall, a_functionPrototypes, a_context, a_modelFunctionnamePrefixStr, a_FileNamePrefix, a_evaluationCode) => {
            let mut txt_1: Tpl::Text;
            let mut a_residualCall = (*a_residualCall).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (a_evaluationCode, a_functionPrototypes) = CodegenOMSIC_Equations::generateEquationFunction(a_evaluationCode.clone(), i_eqsystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), a_context.clone(), a_functionPrototypes.clone())?;
            a_evaluationCode = Tpl::writeTok(a_evaluationCode.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_1 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_funcCallArgName.clone()).clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", model_vars_and_params, &res[i++]")).clone() }))?;
            a_residualCall = CodegenOMSIC_Equations::equationCall(a_residualCall.clone(), i_eqsystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (Tpl::textString(txt_1.clone())?).clone(), (a_omsiName.clone()).clone())?;
            a_residualCall = Tpl::writeTok(a_residualCall.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, i_algSystem @ Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_algSystem_algSysIndex @ i_algSysIndex, .. }, a_fullPathPrefix, a_fileNamePrefix, a_includes, a_residualCall, a_omsiName, a_funcCallArgName, a_functionCall, a_functionPrototypes, _, a_modelFunctionnamePrefixStr, a_FileNamePrefix, a_evaluationCode) => {
            let mut txt_4: Tpl::Text;
            let mut l_content: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut a_includes = (*a_includes).clone();
            let mut a_functionCall = (*a_functionCall).clone();
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            a_includes = Tpl::writeText(a_includes.clone(), a_fileNamePrefix.clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_includes = Tpl::writeStr(a_includes.clone(), (a_omsiName.clone()).clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSyst_")).clone() }))?;
            a_includes = Tpl::writeStr(a_includes.clone(), (intString(i_algSysIndex.clone())).clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h\"")).clone() }))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt_2 = Tpl::writeStr(txt_2.clone(), (a_funcCallArgName.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->algebraic_system_t[")).clone() }))?;
            txt_2 = Tpl::writeStr(txt_2.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("], model_vars_and_params, ")).clone() }))?;
            txt_2 = Tpl::writeStr(txt_2.clone(), (a_omsiName.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->function_vars")).clone() }))?;
            a_functionCall = CodegenOMSIC_Equations::equationCall(a_functionCall.clone(), i_algSystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (Tpl::textString(txt_2.clone())?).clone(), (a_omsiName.clone()).clone())?;
            a_functionCall = Tpl::writeTok(a_functionCall.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_content = generateOmsiAlgSystemCode(Tpl::emptyTxt.clone(), i_algSystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_omsiName.clone()).clone())?;
            txt_4 = Tpl::writeText(Tpl::emptyTxt.clone(), a_fullPathPrefix.clone())?;
            txt_4 = Tpl::writeTok(txt_4.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt_4 = Tpl::writeText(txt_4.clone(), a_fileNamePrefix.clone())?;
            txt_4 = Tpl::writeTok(txt_4.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt_4 = Tpl::writeStr(txt_4.clone(), (a_omsiName.clone()).clone())?;
            txt_4 = Tpl::writeTok(txt_4.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSyst_")).clone() }))?;
            txt_4 = Tpl::writeStr(txt_4.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt_4 = Tpl::writeTok(txt_4.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            Tpl::textFile(l_content.clone(), (Tpl::textString(txt_4.clone())?).clone())?;
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, i_whenEq @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, _, _, a_includes, a_residualCall, a_omsiName, a_funcCallArgName, a_functionCall, a_functionPrototypes, a_context, a_modelFunctionnamePrefixStr, a_FileNamePrefix, a_evaluationCode) => {
            let mut txt_5: Tpl::Text;
            let mut a_functionCall = (*a_functionCall).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (a_evaluationCode, a_functionPrototypes) = CodegenOMSIC_Equations::generateEquationFunction(a_evaluationCode.clone(), i_whenEq.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), a_context.clone(), a_functionPrototypes.clone())?;
            a_evaluationCode = Tpl::writeTok(a_evaluationCode.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_5 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_funcCallArgName.clone()).clone())?;
            txt_5 = Tpl::writeTok(txt_5.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", model_vars_and_params")).clone() }))?;
            a_functionCall = CodegenOMSIC_Equations::equationCall(a_functionCall.clone(), i_whenEq.clone(), (a_FileNamePrefix.clone()).clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (Tpl::textString(txt_5.clone())?).clone(), (a_omsiName.clone()).clone())?;
            a_functionCall = Tpl::writeTok(a_functionCall.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, _, _, _, a_includes, a_residualCall, _, _, a_functionCall, a_functionPrototypes, _, _, _, a_evaluationCode) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: Equation not implemented")).clone() }))?;
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_includes, out_a_residualCall, out_a_functionCall, out_a_functionPrototypes, out_a_evaluationCode))
}

fn lm_61(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_fullPathPrefix: Tpl::Text, mut in_a_fileNamePrefix: Tpl::Text, mut in_a_includes: Tpl::Text, mut in_a_residualCall: Tpl::Text, mut in_a_omsiName: ArcStr, mut in_a_funcCallArgName: ArcStr, mut in_a_functionCall: Tpl::Text, mut in_a_functionPrototypes: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_modelFunctionnamePrefixStr: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_evaluationCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_residualCall: Tpl::Text;
    let mut out_a_functionCall: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    (out_txt, out_a_includes, out_a_residualCall, out_a_functionCall, out_a_functionPrototypes, out_a_evaluationCode) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fullPathPrefix.clone(), in_a_fileNamePrefix.clone(), in_a_includes.clone(), in_a_residualCall.clone(), in_a_omsiName.clone(), in_a_funcCallArgName.clone(), in_a_functionCall.clone(), in_a_functionPrototypes.clone(), in_a_context.clone(), in_a_modelFunctionnamePrefixStr.clone(), in_a_FileNamePrefix.clone(), in_a_evaluationCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_includes, a_residualCall, _, _, a_functionCall, a_functionPrototypes, _, _, _, a_evaluationCode) => {
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eqsystem, tail: rest }, a_fullPathPrefix, a_fileNamePrefix, a_includes, a_residualCall, a_omsiName, a_funcCallArgName, a_functionCall, a_functionPrototypes, a_context, a_modelFunctionnamePrefixStr, a_FileNamePrefix, a_evaluationCode) => {
            let mut txt = (*txt).clone();
            let mut a_includes = (*a_includes).clone();
            let mut a_residualCall = (*a_residualCall).clone();
            let mut a_functionCall = (*a_functionCall).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (txt, a_includes, a_residualCall, a_functionCall, a_functionPrototypes, a_evaluationCode) = fun_60(txt.clone(), i_eqsystem.clone(), a_fullPathPrefix.clone(), a_fileNamePrefix.clone(), a_includes.clone(), a_residualCall.clone(), (a_omsiName.clone()).clone(), (a_funcCallArgName.clone()).clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_context.clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_evaluationCode.clone())?;
            (txt, a_includes, a_residualCall, a_functionCall, a_functionPrototypes, a_evaluationCode) = lm_61(txt.clone(), rest.clone(), a_fullPathPrefix.clone(), a_fileNamePrefix.clone(), a_includes.clone(), a_residualCall.clone(), (a_omsiName.clone()).clone(), (a_funcCallArgName.clone()).clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_context.clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_evaluationCode.clone())?;
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_includes, out_a_residualCall, out_a_functionCall, out_a_functionPrototypes, out_a_evaluationCode))
}

fn fun_62(mut in_txt: Tpl::Text, mut in_a_omsiFunction: Arc<SimCode::OMSIFunction>, mut in_a_fullPathPrefix: Tpl::Text, mut in_a_fileNamePrefix: Tpl::Text, mut in_a_includes: Tpl::Text, mut in_a_residualCall: Tpl::Text, mut in_a_omsiName: ArcStr, mut in_a_funcCallArgName: ArcStr, mut in_a_functionCall: Tpl::Text, mut in_a_functionPrototypes: Tpl::Text, mut in_a_modelFunctionnamePrefixStr: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_evaluationCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_residualCall: Tpl::Text;
    let mut out_a_functionCall: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    (out_txt, out_a_includes, out_a_residualCall, out_a_functionCall, out_a_functionPrototypes, out_a_evaluationCode) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_omsiFunction.clone(), in_a_fullPathPrefix.clone(), in_a_fileNamePrefix.clone(), in_a_includes.clone(), in_a_residualCall.clone(), in_a_omsiName.clone(), in_a_funcCallArgName.clone(), in_a_functionCall.clone(), in_a_functionPrototypes.clone(), in_a_modelFunctionnamePrefixStr.clone(), in_a_FileNamePrefix.clone(), in_a_evaluationCode.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { equations: i_equations, context: i_context @ SimCodeFunction::Context::OMSI_CONTEXT { hashTable: _ }, .. }, a_fullPathPrefix, a_fileNamePrefix, a_includes, a_residualCall, a_omsiName, a_funcCallArgName, a_functionCall, a_functionPrototypes, a_modelFunctionnamePrefixStr, a_FileNamePrefix, a_evaluationCode) => {
            let mut l_0__: Tpl::Text;
            let mut a_includes = (*a_includes).clone();
            let mut a_residualCall = (*a_residualCall).clone();
            let mut a_functionCall = (*a_functionCall).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (l_0__, a_includes, a_residualCall, a_functionCall, a_functionPrototypes, a_evaluationCode) = lm_61(Tpl::emptyTxt.clone(), i_equations.clone(), a_fullPathPrefix.clone(), a_fileNamePrefix.clone(), a_includes.clone(), a_residualCall.clone(), (a_omsiName.clone()).clone(), (a_funcCallArgName.clone()).clone(), a_functionCall.clone(), a_functionPrototypes.clone(), i_context.clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_evaluationCode.clone())?;
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, _, _, _, a_includes, a_residualCall, _, _, a_functionCall, a_functionPrototypes, _, _, a_evaluationCode) => {
            (txt.clone(), a_includes.clone(), a_residualCall.clone(), a_functionCall.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_includes, out_a_residualCall, out_a_functionCall, out_a_functionPrototypes, out_a_evaluationCode))
}

pub fn generateOmsiFunctionCode_inner(mut txt: Tpl::Text, mut a_omsiFunction: Arc<SimCode::OMSIFunction>, mut a_FileNamePrefix: ArcStr, mut a_modelFunctionnamePrefixStr: ArcStr, mut a_funcCallArgName: ArcStr, mut a_includes: Tpl::Text, mut a_evaluationCode: Tpl::Text, mut a_functionCall: Tpl::Text, mut a_residualCall: Tpl::Text, mut a_functionPrototypes: Tpl::Text, mut a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    let mut out_a_functionCall: Tpl::Text;
    let mut out_a_residualCall: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut ret_3: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fullPathPrefix: Tpl::Text;
    let mut ret_1: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fileNamePrefix: Tpl::Text;
    ret_1 = SimCodeUtil::getSimCode()?;
    l_fileNamePrefix = CodegenUtilSimulation::fileNamePrefix(Tpl::emptyTxt.clone(), ret_1.clone())?;
    ret_3 = SimCodeUtil::getSimCode()?;
    l_fullPathPrefix = CodegenUtilSimulation::fullPathPrefix(Tpl::emptyTxt.clone(), ret_3.clone())?;
    (out_txt, out_a_includes, out_a_residualCall, out_a_functionCall, out_a_functionPrototypes, out_a_evaluationCode) = fun_62(txt.clone(), a_omsiFunction.clone(), l_fullPathPrefix.clone(), l_fileNamePrefix.clone(), a_includes.clone(), a_residualCall.clone(), (a_omsiName.clone()).clone(), (a_funcCallArgName.clone()).clone(), a_functionCall.clone(), a_functionPrototypes.clone(), (a_modelFunctionnamePrefixStr.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_evaluationCode.clone())?;
    Ok((out_txt, out_a_includes, out_a_evaluationCode, out_a_functionCall, out_a_residualCall, out_a_functionPrototypes))
}

fn fun_64(mut in_txt: Tpl::Text, mut in_a_eqsystem: Arc<SimCode::SimEqSystem>, mut in_a_functionPrototypes: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_FunctionnamePrefix: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_evaluationCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    (out_txt, out_a_functionPrototypes, out_a_evaluationCode) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eqsystem.clone(), in_a_functionPrototypes.clone(), in_a_context.clone(), in_a_FunctionnamePrefix.clone(), in_a_FileNamePrefix.clone(), in_a_evaluationCode.clone())) {
        (txt, i_eqsystem @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: _, .. }, a_functionPrototypes, a_context, a_FunctionnamePrefix, a_FileNamePrefix, a_evaluationCode) => {
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (a_evaluationCode, a_functionPrototypes) = CodegenOMSIC_Equations::generateEquationFunction(a_evaluationCode.clone(), i_eqsystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_FunctionnamePrefix.clone()).clone(), a_context.clone(), a_functionPrototypes.clone())?;
            a_evaluationCode = Tpl::writeTok(a_evaluationCode.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, i_eqsystem @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }, a_functionPrototypes, a_context, a_FunctionnamePrefix, a_FileNamePrefix, a_evaluationCode) => {
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (a_evaluationCode, a_functionPrototypes) = CodegenOMSIC_Equations::generateEquationFunction(a_evaluationCode.clone(), i_eqsystem.clone(), (a_FileNamePrefix.clone()).clone(), (a_FunctionnamePrefix.clone()).clone(), a_context.clone(), a_functionPrototypes.clone())?;
            a_evaluationCode = Tpl::writeTok(a_evaluationCode.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { index: _, .. }, a_functionPrototypes, _, _, _, a_evaluationCode) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: Equation  SES_ALGEBRAIC_SYSTEM not implemented yet")).clone() }))?;
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, i_whenEq @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_functionPrototypes, a_context, a_FunctionnamePrefix, a_FileNamePrefix, a_evaluationCode) => {
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (a_evaluationCode, a_functionPrototypes) = CodegenOMSIC_Equations::generateEquationFunction(a_evaluationCode.clone(), i_whenEq.clone(), (a_FileNamePrefix.clone()).clone(), (a_FunctionnamePrefix.clone()).clone(), a_context.clone(), a_functionPrototypes.clone())?;
            a_evaluationCode = Tpl::writeTok(a_evaluationCode.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, _, a_functionPrototypes, _, _, _, a_evaluationCode) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: Equation not implemented")).clone() }))?;
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes, out_a_evaluationCode))
}

fn lm_65(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_functionPrototypes: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_FunctionnamePrefix: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_evaluationCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    (out_txt, out_a_functionPrototypes, out_a_evaluationCode) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_functionPrototypes.clone(), in_a_context.clone(), in_a_FunctionnamePrefix.clone(), in_a_FileNamePrefix.clone(), in_a_evaluationCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_functionPrototypes, _, _, _, a_evaluationCode) => {
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eqsystem, tail: rest }, a_functionPrototypes, a_context, a_FunctionnamePrefix, a_FileNamePrefix, a_evaluationCode) => {
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (txt, a_functionPrototypes, a_evaluationCode) = fun_64(txt.clone(), i_eqsystem.clone(), a_functionPrototypes.clone(), a_context.clone(), (a_FunctionnamePrefix.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_evaluationCode.clone())?;
            (txt, a_functionPrototypes, a_evaluationCode) = lm_65(txt.clone(), rest.clone(), a_functionPrototypes.clone(), a_context.clone(), (a_FunctionnamePrefix.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_evaluationCode.clone())?;
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_functionPrototypes, out_a_evaluationCode))
}

fn fun_66(mut in_txt: Tpl::Text, mut in_a_omsiFunction: Arc<SimCode::OMSIFunction>, mut in_a_functionPrototypes: Tpl::Text, mut in_a_FunctionnamePrefix: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_evaluationCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    (out_txt, out_a_functionPrototypes, out_a_evaluationCode) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_omsiFunction.clone(), in_a_functionPrototypes.clone(), in_a_FunctionnamePrefix.clone(), in_a_FileNamePrefix.clone(), in_a_evaluationCode.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { equations: i_equations, context: i_context @ SimCodeFunction::Context::OMSI_CONTEXT { hashTable: _ }, .. }, a_functionPrototypes, a_FunctionnamePrefix, a_FileNamePrefix, a_evaluationCode) => {
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            (l_0__, a_functionPrototypes, a_evaluationCode) = lm_65(Tpl::emptyTxt.clone(), i_equations.clone(), a_functionPrototypes.clone(), i_context.clone(), (a_FunctionnamePrefix.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_evaluationCode.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("virtual omsi_status initialize_omsi_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FunctionnamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_functions (struct omsi_function_t* omsi_function);\n")).clone(), (literal!("virtual omsi_status omsi_")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FunctionnamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("All(struct omsi_function_t* simulation, const omsi_values* model_vars_and_params, void* data);\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_functionPrototypes.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        (txt, _, a_functionPrototypes, _, _, a_evaluationCode) => {
            (txt.clone(), a_functionPrototypes.clone(), a_evaluationCode.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes, out_a_evaluationCode))
}

pub fn generateOmsiMemberFunction(mut txt: Tpl::Text, mut a_omsiFunction: Arc<SimCode::OMSIFunction>, mut a_FileNamePrefix: ArcStr, mut a_FunctionnamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_functionPrototypes: Tpl::Text;
    let mut l_evaluationCode: Tpl::Text;
    l_evaluationCode = Tpl::emptyTxt.clone();
    l_functionPrototypes = Tpl::emptyTxt.clone();
    (out_txt, l_functionPrototypes, l_evaluationCode) = fun_66(txt.clone(), a_omsiFunction.clone(), l_functionPrototypes.clone(), (a_FunctionnamePrefix.clone()).clone(), (a_FileNamePrefix.clone()).clone(), l_evaluationCode.clone())?;
    Ok(out_txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_a_equationSystem: Arc<SimCode::SimEqSystem>, mut in_a_fullPathPrefix: Tpl::Text, mut in_a_fileNamePrefix: Tpl::Text, mut in_a_residualCall: Tpl::Text, mut in_a_functionCall: Tpl::Text, mut in_a_evaluationCode: Tpl::Text, mut in_a_omsiName: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_includes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_residualCall: Tpl::Text;
    let mut out_a_functionCall: Tpl::Text;
    let mut out_a_evaluationCode: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    (out_txt, out_a_residualCall, out_a_functionCall, out_a_evaluationCode, out_a_functionPrototypes, out_a_includes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equationSystem.clone(), in_a_fullPathPrefix.clone(), in_a_fileNamePrefix.clone(), in_a_residualCall.clone(), in_a_functionCall.clone(), in_a_evaluationCode.clone(), in_a_omsiName.clone(), in_a_functionPrototypes.clone(), in_a_FileNamePrefix.clone(), in_a_includes.clone())) {
        (txt, i_algSystem @ Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_algSystem_algSysIndex, residual: i_residual, matrix: None, .. }, a_fullPathPrefix, a_fileNamePrefix, a_residualCall, a_functionCall, a_evaluationCode, a_omsiName, a_functionPrototypes, a_FileNamePrefix, a_includes) => {
            let mut txt_7: Tpl::Text;
            let mut l_headerFileContent: Tpl::Text;
            let mut l_headerFileName: Tpl::Text;
            let mut l_matrixString: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut l_equationInfos: Tpl::Text;
            let mut l_0__: Tpl::Text;
            let mut l_initlaizationFunction: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_residualCall = (*a_residualCall).clone();
            let mut a_functionCall = (*a_functionCall).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_includes = (*a_includes).clone();
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <omsi_solve_alg_system.h>")).clone() }))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (l_initlaizationFunction, a_functionPrototypes, a_includes) = generateInitalizationAlgSystem(Tpl::emptyTxt.clone(), i_algSystem.clone(), (a_FileNamePrefix.clone()).clone(), a_functionPrototypes.clone(), a_includes.clone(), (a_omsiName.clone()).clone())?;
            (l_0__, a_includes, a_evaluationCode, a_functionCall, a_residualCall, a_functionPrototypes) = generateOmsiFunctionCode_inner(Tpl::emptyTxt.clone(), i_residual.clone(), (a_FileNamePrefix.clone()).clone(), (literal!("")).clone(), (literal!("this_function")).clone(), a_includes.clone(), a_evaluationCode.clone(), a_functionCall.clone(), a_residualCall.clone(), a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            ret_3 = List::fill(i_algSystem.clone(), 1);
            l_equationInfos = CodegenUtilSimulation::dumpEqs(Tpl::emptyTxt.clone(), ret_3.clone())?;
            l_matrixString = Tpl::emptyTxt.clone();
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_status ")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_resFunction_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" (struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data);\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSystFunction_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(omsi_algebraic_system_t* this_alg_system, const omsi_values* model_vars_and_params, void* data);")).clone() }))?;
            l_headerFileName = Tpl::writeText(Tpl::emptyTxt.clone(), a_fileNamePrefix.clone())?;
            l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            l_headerFileName = Tpl::writeStr(l_headerFileName.clone(), (a_omsiName.clone()).clone())?;
            l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSyst_")).clone() }))?;
            l_headerFileName = Tpl::writeStr(l_headerFileName.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            (l_headerFileContent, a_includes, a_functionPrototypes) = generateCodeHeader(Tpl::emptyTxt.clone(), (a_FileNamePrefix.clone()).clone(), a_includes.clone(), (Tpl::textString(l_headerFileName.clone())?).clone(), a_functionPrototypes.clone())?;
            txt_7 = Tpl::writeText(Tpl::emptyTxt.clone(), a_fullPathPrefix.clone())?;
            txt_7 = Tpl::writeTok(txt_7.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt_7 = Tpl::writeText(txt_7.clone(), l_headerFileName.clone())?;
            txt_7 = Tpl::writeTok(txt_7.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
            Tpl::textFile(l_headerFileContent.clone(), (Tpl::textString(txt_7.clone())?).clone())?;
            txt = insertCopyrightOpenModelica(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* Algebraic system code */\n")).clone(), (literal!("#include \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_headerFileName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("/* Instantiation and initialization */\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_initlaizationFunction.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_matrixString.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* Evaluation functions for ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_resFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" */\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_evaluationCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_resFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" (struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data) {\n")).clone(), (literal!("  omsi_real* res = (omsi_real*) data;\n")).clone(), (literal!("  omsi_unsigned_int i=0;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_functionCall.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_residualCall.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("return omsi_ok;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("/* Algebraic system evaluation */\n")).clone(), (literal!("/*\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_equationInfos.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("*/\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSystFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(omsi_algebraic_system_t* this_alg_system,\n")).clone(), (literal!("                          const omsi_values* model_vars_and_params,\n")).clone(), (literal!("                          void* data){\n")).clone(), (literal!("\n")).clone(), (literal!("  /* Variables */\n")).clone(), (literal!("  omsi_status status;\n")).clone(), (literal!("\n")).clone(), (literal!("  /* Log function call */\n")).clone(), (literal!("  filtered_base_logger(global_logCategories, log_all, omsi_ok,\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"fmi2Evaluate: Solve algebraic system ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".\");\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  /* call API function something */\n")).clone(), (literal!("  status = omsi_solve_algebraic_system(this_alg_system, model_vars_and_params);\n")).clone(), (literal!("\n")).clone(), (literal!("  return status;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_residualCall.clone(), a_functionCall.clone(), a_evaluationCode.clone(), a_functionPrototypes.clone(), a_includes.clone())
        },
        (txt, i_algSystem @ Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_algSystem_algSysIndex, residual: i_residual, matrix: i_matrix @ Some(Deref @ SimCode::DerivativeMatrix { columns: _, .. }), .. }, a_fullPathPrefix, a_fileNamePrefix, a_residualCall, a_functionCall, a_evaluationCode, a_omsiName, a_functionPrototypes, a_FileNamePrefix, a_includes) => {
            let mut txt_10: Tpl::Text;
            let mut l_0___1: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut l_headerFileContent: Tpl::Text;
            let mut l_headerFileName: Tpl::Text;
            let mut l_matrixString: Tpl::Text;
            let mut l_equationInfos: Tpl::Text;
            let mut l_0__: Tpl::Text;
            let mut l_initlaizationFunction: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_residualCall = (*a_residualCall).clone();
            let mut a_functionCall = (*a_functionCall).clone();
            let mut a_evaluationCode = (*a_evaluationCode).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_includes = (*a_includes).clone();
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <omsi_solve_alg_system.h>")).clone() }))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (l_initlaizationFunction, a_functionPrototypes, a_includes) = generateInitalizationAlgSystem(Tpl::emptyTxt.clone(), i_algSystem.clone(), (a_FileNamePrefix.clone()).clone(), a_functionPrototypes.clone(), a_includes.clone(), (a_omsiName.clone()).clone())?;
            (l_0__, a_includes, a_evaluationCode, a_functionCall, a_residualCall, a_functionPrototypes) = generateOmsiFunctionCode_inner(Tpl::emptyTxt.clone(), i_residual.clone(), (a_FileNamePrefix.clone()).clone(), (literal!("")).clone(), (literal!("this_function")).clone(), a_includes.clone(), a_evaluationCode.clone(), a_functionCall.clone(), a_residualCall.clone(), a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            ret_8 = List::fill(i_algSystem.clone(), 1);
            l_equationInfos = CodegenUtilSimulation::dumpEqs(Tpl::emptyTxt.clone(), ret_8.clone())?;
            l_matrixString = CodegenOMSIC_Equations::generateMatrixInitialization(Tpl::emptyTxt.clone(), i_matrix.clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            a_includes = Tpl::writeText(a_includes.clone(), a_fileNamePrefix.clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_includes = Tpl::writeStr(a_includes.clone(), (a_omsiName.clone()).clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derMat_")).clone() }))?;
            a_includes = Tpl::writeStr(a_includes.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h\"")).clone() }))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_0___1 = generateDerivativeFile(Tpl::emptyTxt.clone(), i_matrix.clone(), (a_FileNamePrefix.clone()).clone(), (intString(i_algSystem_algSysIndex.clone())).clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_status ")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_resFunction_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" (struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data);\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSystFunction_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(omsi_algebraic_system_t* this_alg_system, const omsi_values* model_vars_and_params, void* data);")).clone() }))?;
            l_headerFileName = Tpl::writeText(Tpl::emptyTxt.clone(), a_fileNamePrefix.clone())?;
            l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            l_headerFileName = Tpl::writeStr(l_headerFileName.clone(), (a_omsiName.clone()).clone())?;
            l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSyst_")).clone() }))?;
            l_headerFileName = Tpl::writeStr(l_headerFileName.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            (l_headerFileContent, a_includes, a_functionPrototypes) = generateCodeHeader(Tpl::emptyTxt.clone(), (a_FileNamePrefix.clone()).clone(), a_includes.clone(), (Tpl::textString(l_headerFileName.clone())?).clone(), a_functionPrototypes.clone())?;
            txt_10 = Tpl::writeText(Tpl::emptyTxt.clone(), a_fullPathPrefix.clone())?;
            txt_10 = Tpl::writeTok(txt_10.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt_10 = Tpl::writeText(txt_10.clone(), l_headerFileName.clone())?;
            txt_10 = Tpl::writeTok(txt_10.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
            Tpl::textFile(l_headerFileContent.clone(), (Tpl::textString(txt_10.clone())?).clone())?;
            txt = insertCopyrightOpenModelica(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* Algebraic system code */\n")).clone(), (literal!("#include \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_headerFileName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("/* Instantiation and initialization */\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_initlaizationFunction.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_matrixString.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* Evaluation functions for ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_resFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" */\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_evaluationCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_resFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" (struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data) {\n")).clone(), (literal!("  omsi_real* res = (omsi_real*) data;\n")).clone(), (literal!("  omsi_unsigned_int i=0;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_functionCall.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_residualCall.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("return omsi_ok;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("/* Algebraic system evaluation */\n")).clone(), (literal!("/*\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_equationInfos.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("*/\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSystFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(omsi_algebraic_system_t* this_alg_system,\n")).clone(), (literal!("                          const omsi_values* model_vars_and_params,\n")).clone(), (literal!("                          void* data){\n")).clone(), (literal!("\n")).clone(), (literal!("  /* Variables */\n")).clone(), (literal!("  omsi_status status;\n")).clone(), (literal!("\n")).clone(), (literal!("  /* Log function call */\n")).clone(), (literal!("  filtered_base_logger(global_logCategories, log_all, omsi_ok,\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"fmi2Evaluate: Solve algebraic system ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSystem_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".\");\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  /* call API function something */\n")).clone(), (literal!("  status = omsi_solve_algebraic_system(this_alg_system, model_vars_and_params);\n")).clone(), (literal!("\n")).clone(), (literal!("  return status;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_residualCall.clone(), a_functionCall.clone(), a_evaluationCode.clone(), a_functionPrototypes.clone(), a_includes.clone())
        },
        (txt, _, _, _, a_residualCall, a_functionCall, a_evaluationCode, _, a_functionPrototypes, _, a_includes) => {
            (txt.clone(), a_residualCall.clone(), a_functionCall.clone(), a_evaluationCode.clone(), a_functionPrototypes.clone(), a_includes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_residualCall, out_a_functionCall, out_a_evaluationCode, out_a_functionPrototypes, out_a_includes))
}

pub fn generateOmsiAlgSystemCode(mut txt: Tpl::Text, mut a_equationSystem: Arc<SimCode::SimEqSystem>, mut a_FileNamePrefix: ArcStr, mut a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_9: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fullPathPrefix: Tpl::Text;
    let mut ret_7: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fileNamePrefix: Tpl::Text;
    let mut l_functionPrototypes: Tpl::Text;
    let mut l_derivativeMatrix: Tpl::Text;
    let mut l_residualCall: Tpl::Text;
    let mut l_functionCall: Tpl::Text;
    let mut l_evaluationCode: Tpl::Text;
    let mut l_includes: Tpl::Text;
    l_includes = Tpl::emptyTxt.clone();
    l_evaluationCode = Tpl::emptyTxt.clone();
    l_functionCall = Tpl::emptyTxt.clone();
    l_residualCall = Tpl::emptyTxt.clone();
    l_derivativeMatrix = Tpl::emptyTxt.clone();
    l_functionPrototypes = Tpl::emptyTxt.clone();
    ret_7 = SimCodeUtil::getSimCode()?;
    l_fileNamePrefix = CodegenUtilSimulation::fileNamePrefix(Tpl::emptyTxt.clone(), ret_7.clone())?;
    ret_9 = SimCodeUtil::getSimCode()?;
    l_fullPathPrefix = CodegenUtilSimulation::fullPathPrefix(Tpl::emptyTxt.clone(), ret_9.clone())?;
    (out_txt, l_residualCall, l_functionCall, l_evaluationCode, l_functionPrototypes, l_includes) = fun_68(txt.clone(), a_equationSystem.clone(), l_fullPathPrefix.clone(), l_fileNamePrefix.clone(), l_residualCall.clone(), l_functionCall.clone(), l_evaluationCode.clone(), (a_omsiName.clone()).clone(), l_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone(), l_includes.clone())?;
    Ok(out_txt)
}

pub fn generateCodeHeader(mut txt: Tpl::Text, mut a_FileNamePrefix: ArcStr, mut a_includes: Tpl::Text, mut a_headerName: ArcStr, mut a_functionPrototypes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_macro__name: Tpl::Text;
    ret_1 = (System::makeC89Identifier((a_headerName.clone()).clone())).clone();
    l_macro__name = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    out_txt = insertCopyrightOpenModelica(txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#if !defined(")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_macro__name.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_H)\n")).clone(), (literal!("#define ")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_macro__name.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_H\n")).clone(), (literal!("\n")).clone(), (literal!("#include <omsi.h>\n")).clone(), (literal!("#include <omsic.h>\n")).clone(), (literal!("#include <omsi_callbacks.h>\n")).clone(), (literal!("#include <omsi_global.h>\n")).clone(), (literal!("\n")).clone(), (literal!("#include <stdlib.h>\n")).clone(), (literal!("#include <math.h>\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), a_includes.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("/* Function prototypes */\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), a_functionPrototypes.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_a_includes = a_includes.clone();
    out_a_functionPrototypes = a_functionPrototypes.clone();
    Ok((out_txt, out_a_includes, out_a_functionPrototypes))
}

fn lm_71(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::OMSIFunction>>>, mut in_a_omsiName: ArcStr, mut in_a_includes: Tpl::Text, mut in_a_functionPrototypes: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_index: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_includes, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_omsiName.clone(), in_a_includes.clone(), in_a_functionPrototypes.clone(), in_a_FileNamePrefix.clone(), in_a_index.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_includes, a_functionPrototypes, _, _) => {
            (txt.clone(), a_includes.clone(), a_functionPrototypes.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_column, tail: rest }, a_omsiName, a_includes, a_functionPrototypes, a_FileNamePrefix, a_index) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_includes = (*a_includes).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("derivativeMatFunc_")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (a_index.clone()).clone())?;
            (txt, a_functionPrototypes, a_includes) = generateInitalizationOMSIFunction(txt.clone(), i_column.clone(), (Tpl::textString(txt_0.clone())?).clone(), (a_FileNamePrefix.clone()).clone(), (literal!("")).clone(), a_functionPrototypes.clone(), a_includes.clone(), true, (a_omsiName.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_includes, a_functionPrototypes) = lm_71(txt.clone(), rest.clone(), (a_omsiName.clone()).clone(), a_includes.clone(), a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone(), (a_index.clone()).clone())?;
            (txt.clone(), a_includes.clone(), a_functionPrototypes.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_includes, out_a_functionPrototypes))
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_omsiName: ArcStr, mut in_a_includes: Tpl::Text, mut in_a_functionPrototypes: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_index: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_includes, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matrix.clone(), in_a_omsiName.clone(), in_a_includes.clone(), in_a_functionPrototypes.clone(), in_a_FileNamePrefix.clone(), in_a_index.clone())) {
        (txt, Some(Deref @ SimCode::DerivativeMatrix { columns: i_derMat_columns, .. }), a_omsiName, a_includes, a_functionPrototypes, a_FileNamePrefix, a_index) => {
            let mut l_initalizationCodeCol: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_includes = (*a_includes).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            l_initalizationCodeCol = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_initalizationCodeCol, a_includes, a_functionPrototypes) = lm_71(l_initalizationCodeCol.clone(), i_derMat_columns.clone(), (a_omsiName.clone()).clone(), a_includes.clone(), a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone(), (a_index.clone()).clone())?;
            l_initalizationCodeCol = Tpl::popIter(l_initalizationCodeCol.clone())?;
            txt = Tpl::writeText(txt.clone(), l_initalizationCodeCol.clone())?;
            (txt.clone(), a_includes.clone(), a_functionPrototypes.clone())
        },
        (txt, _, _, a_includes, a_functionPrototypes, _, _) => {
            (txt.clone(), a_includes.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_includes, out_a_functionPrototypes))
}

pub fn generateDerivativeFile(mut txt: Tpl::Text, mut a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut a_FileNamePrefix: ArcStr, mut a_index: ArcStr, mut a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut txt_14: Tpl::Text;
    let mut l_headerFileContent: Tpl::Text;
    let mut txt_12: Tpl::Text;
    let mut l_content: Tpl::Text;
    let mut ret_10: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fullPathPrefix: Tpl::Text;
    let mut ret_8: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_fileNamePrefix: Tpl::Text;
    let mut ret_6: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
    let mut l_headerFileName: Tpl::Text;
    let mut l_body: Tpl::Text;
    let mut l_initalizationCode: Tpl::Text;
    let mut l_functionPrototypes: Tpl::Text;
    let mut l_initalizationCodeCol: Tpl::Text;
    let mut l_includes: Tpl::Text;
    l_includes = Tpl::emptyTxt.clone();
    l_initalizationCodeCol = Tpl::emptyTxt.clone();
    l_functionPrototypes = Tpl::emptyTxt.clone();
    (l_initalizationCode, l_includes, l_functionPrototypes) = fun_72(Tpl::emptyTxt.clone(), a_matrix.clone(), (a_omsiName.clone()).clone(), l_includes.clone(), l_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone(), (a_index.clone()).clone())?;
    (l_body, l_functionPrototypes) = CodegenOMSIC_Equations::generateDerivativeMatrix(Tpl::emptyTxt.clone(), a_matrix.clone(), (a_FileNamePrefix.clone()).clone(), (a_index.clone()).clone(), l_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
    ret_6 = SimCodeUtil::getSimCode()?;
    l_headerFileName = CodegenUtilSimulation::fileNamePrefix(Tpl::emptyTxt.clone(), ret_6.clone())?;
    l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    l_headerFileName = Tpl::writeStr(l_headerFileName.clone(), (a_omsiName.clone()).clone())?;
    l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derMat_")).clone() }))?;
    l_headerFileName = Tpl::writeStr(l_headerFileName.clone(), (a_index.clone()).clone())?;
    ret_8 = SimCodeUtil::getSimCode()?;
    l_fileNamePrefix = CodegenUtilSimulation::fileNamePrefix(Tpl::emptyTxt.clone(), ret_8.clone())?;
    ret_10 = SimCodeUtil::getSimCode()?;
    l_fullPathPrefix = CodegenUtilSimulation::fullPathPrefix(Tpl::emptyTxt.clone(), ret_10.clone())?;
    l_content = insertCopyrightOpenModelica(Tpl::emptyTxt.clone())?;
    l_content = Tpl::softNewLine(l_content.clone())?;
    l_content = Tpl::writeTok(l_content.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* derivative matrix code for algebraic system ")).clone()], lastHasNewLine: false }))?;
    l_content = Tpl::writeStr(l_content.clone(), (a_index.clone()).clone())?;
    l_content = Tpl::writeTok(l_content.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("*/\n")).clone(), (literal!("#include \"")).clone()], lastHasNewLine: false }))?;
    l_content = Tpl::writeText(l_content.clone(), l_headerFileName.clone())?;
    l_content = Tpl::writeTok(l_content.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("/* Instantiation and initalization */\n")).clone()], lastHasNewLine: true }))?;
    l_content = Tpl::writeText(l_content.clone(), l_initalizationCode.clone())?;
    l_content = Tpl::softNewLine(l_content.clone())?;
    l_content = Tpl::writeTok(l_content.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* derivative matrix evaluation */\n")).clone()], lastHasNewLine: true }))?;
    l_content = Tpl::writeText(l_content.clone(), l_body.clone())?;
    l_content = Tpl::softNewLine(l_content.clone())?;
    l_content = Tpl::writeTok(l_content.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#if defined(__cplusplus)\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
    l_content = Tpl::writeTok(l_content.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    txt_12 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fullPathPrefix.clone())?;
    txt_12 = Tpl::writeTok(txt_12.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
    txt_12 = Tpl::writeText(txt_12.clone(), l_fileNamePrefix.clone())?;
    txt_12 = Tpl::writeTok(txt_12.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    txt_12 = Tpl::writeStr(txt_12.clone(), (a_omsiName.clone()).clone())?;
    txt_12 = Tpl::writeTok(txt_12.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derMat_")).clone() }))?;
    txt_12 = Tpl::writeStr(txt_12.clone(), (a_index.clone()).clone())?;
    txt_12 = Tpl::writeTok(txt_12.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
    Tpl::textFile(l_content.clone(), (Tpl::textString(txt_12.clone())?).clone())?;
    (l_headerFileContent, l_includes, l_functionPrototypes) = generateCodeHeader(Tpl::emptyTxt.clone(), (a_FileNamePrefix.clone()).clone(), l_includes.clone(), (Tpl::textString(l_headerFileName.clone())?).clone(), l_functionPrototypes.clone())?;
    txt_14 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fullPathPrefix.clone())?;
    txt_14 = Tpl::writeTok(txt_14.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
    txt_14 = Tpl::writeText(txt_14.clone(), l_headerFileName.clone())?;
    txt_14 = Tpl::writeTok(txt_14.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
    Tpl::textFile(l_headerFileContent.clone(), (Tpl::textString(txt_14.clone())?).clone())?;
    out_txt = txt.clone();
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_74(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cond, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_cond.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_74(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_75(mut in_txt: Tpl::Text, mut in_mArg: i32, mut in_a_zeroCrossingIndices: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_zeroCrossingIndices.clone()) {
        (mut txt, 0, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algSystem->zerocrossing_indices = NULL;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_zeroCrossingIndices) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algSystem->zerocrossing_indices[listLength(zeroCrossingConditions)] = {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_zeroCrossingIndices.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("};")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_76(mut in_txt: Tpl::Text, mut in_a_linearSystem: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_linearSystem.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_77(mut in_txt: Tpl::Text, mut in_a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_algSysIndex: i32, mut in_a_omsiName: ArcStr, mut in_a_FileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matrix.clone(), in_a_algSysIndex.clone(), in_a_omsiName.clone(), in_a_FileNamePrefix.clone())) {
        (txt, Some(_), a_algSysIndex, a_omsiName, a_FileNamePrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_derivativeMatFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_OMSIFunc(algSystem->jacobian) == omsi_error){\n")).clone(), (literal!("        return omsi_error;\n")).clone(), (literal!("      }")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_algSysIndex: i32, mut in_a_omsiName: ArcStr, mut in_a_FileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matrix.clone(), in_a_algSysIndex.clone(), in_a_omsiName.clone(), in_a_FileNamePrefix.clone())) {
        (txt, None, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NULL;")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_algSysIndex, a_omsiName, a_FileNamePrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derivativeMatFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateInitalizationAlgSystem(mut in_txt: Tpl::Text, mut in_a_equationSystem: Arc<SimCode::SimEqSystem>, mut in_a_FileNamePrefix: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_includes: Tpl::Text, mut in_a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    (out_txt, out_a_functionPrototypes, out_a_includes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equationSystem.clone(), in_a_FileNamePrefix.clone(), in_a_functionPrototypes.clone(), in_a_includes.clone(), in_a_omsiName.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { linearSystem: i_linearSystem, zeroCrossingConditions: i_zeroCrossingConditions, algSysIndex: i_algSysIndex, matrix: i_matrix, residual: i_residual @ Deref @ SimCode::OMSIFunction { equations: _, .. }, dim_n: i_dim__n, .. }, a_FileNamePrefix, a_functionPrototypes, a_includes, a_omsiName) => {
            let mut txt_3: Tpl::Text;
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut l_zeroCrossingIndices: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_includes = (*a_includes).clone();
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_status ")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_AlgSystem_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (intString(i_algSysIndex.clone())).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(omsi_algebraic_system_t* algSystem, omsi_values* function_vars, omsi_values* pre_vars);")).clone() }))?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_zeroCrossingIndices = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_zeroCrossingIndices = lm_74(l_zeroCrossingIndices.clone(), i_zeroCrossingConditions.clone())?;
            l_zeroCrossingIndices = Tpl::popIter(l_zeroCrossingIndices.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* Function instantiate omsi_algebraic_system_t struct */\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_AlgSystem_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("(omsi_algebraic_system_t* algSystem, omsi_values* function_vars, omsi_values* pre_vars) {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algSystem->n_iteration_vars = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_dim__n.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("\n")).clone(), (literal!("algSystem->n_conditions = ")).clone()], lastHasNewLine: false }))?;
            ret_1 = (i_zeroCrossingConditions.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            ret_2 = (i_zeroCrossingConditions.clone().len() as i32);
            txt = fun_75(txt.clone(), ret_2.clone(), l_zeroCrossingIndices.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("algSystem->isLinear = ")).clone()], lastHasNewLine: false }))?;
            txt = fun_76(txt.clone(), i_linearSystem.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("\n")).clone(), (literal!("/* Instantiate omsi_function_t jacobian */\n")).clone(), (literal!("algSystem->jacobian = omsu_instantiate_omsi_function (function_vars, pre_vars);\n")).clone(), (literal!("if (!algSystem->jacobian) {\n")).clone(), (literal!("  return omsi_error;\n")).clone(), (literal!("}\n")).clone()], lastHasNewLine: true }))?;
            txt = fun_77(txt.clone(), i_matrix.clone(), i_algSysIndex.clone(), (a_omsiName.clone()).clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* Instantiate omsi_function_t function */\n")).clone(), (literal!("algSystem->functions = omsu_instantiate_omsi_function (function_vars, pre_vars);\n")).clone(), (literal!("if (!algSystem->functions) {\n")).clone(), (literal!("  return omsi_error;\n")).clone(), (literal!("}\n")).clone(), (literal!("if (")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_resFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_OMSIFunc(algSystem->functions) == omsi_error){\n")).clone(), (literal!("  return omsi_error;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("/* ToDo: put into init functions */\n")).clone(), (literal!("algSystem->functions->evaluate = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_resFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("algSystem->jacobian->evaluate = ")).clone()], lastHasNewLine: false }))?;
            txt = fun_78(txt.clone(), i_matrix.clone(), i_algSysIndex.clone(), (a_omsiName.clone()).clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("algSystem->solver_data = NULL;\n")).clone(), (literal!("\n")).clone(), (literal!("return omsi_ok;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt_3 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("resFunction_")).clone() }))?;
            txt_3 = Tpl::writeStr(txt_3.clone(), (intString(i_algSysIndex.clone())).clone())?;
            (txt, a_functionPrototypes, a_includes) = generateInitalizationOMSIFunction(txt.clone(), i_residual.clone(), (Tpl::textString(txt_3.clone())?).clone(), (a_FileNamePrefix.clone()).clone(), (literal!("")).clone(), a_functionPrototypes.clone(), a_includes.clone(), false, (a_omsiName.clone()).clone())?;
            (txt.clone(), a_functionPrototypes.clone(), a_includes.clone())
        },
        (txt, _, _, a_functionPrototypes, a_includes, _) => {
            (txt.clone(), a_functionPrototypes.clone(), a_includes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes, out_a_includes))
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_variable: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variable.clone())) {
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMSI_TYPE_REAL")).clone() }))?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_INTEGER { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMSI_TYPE_INTEGER")).clone() }))?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_BOOL { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMSI_TYPE_BOOLEAN")).clone() }))?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_STRING { varLst: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMSI_TYPE_STRING")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMSI_TYPE_UNKNOWN")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_81(mut in_txt: Tpl::Text, mut in_a_variable: SimCodeVar::SimVar, mut in_a_stringIndex: Tpl::Text, mut in_a_stringName: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stringIndex: Tpl::Text;
    let mut out_a_stringName: Tpl::Text;
    (out_txt, out_a_stringIndex, out_a_stringName) = (match (in_txt.clone(), in_a_variable.clone(), in_a_stringIndex.clone(), in_a_stringName.clone()) {
        (mut txt, ref i_var @ SimCodeVar::SimVar { index: ref i_var_index, varKind: BackendDAE::VarKind::JAC_VAR, .. }, mut a_stringIndex, mut a_stringName) => {
            a_stringName = CodegenUtil::crefCCommentWithVariability(a_stringName.clone(), i_var.clone())?;
            a_stringIndex = Tpl::writeStr(a_stringIndex.clone(), (intString(i_var_index.clone())).clone())?;
            (txt.clone(), a_stringIndex.clone(), a_stringName.clone())
        },
        (mut txt, ref i_var @ SimCodeVar::SimVar { index: ref i_var_index, varKind: BackendDAE::VarKind::JAC_TMP_VAR, .. }, mut a_stringIndex, mut a_stringName) => {
            a_stringName = CodegenUtil::crefCCommentWithVariability(a_stringName.clone(), i_var.clone())?;
            a_stringIndex = Tpl::writeStr(a_stringIndex.clone(), (intString(i_var_index.clone())).clone())?;
            (txt.clone(), a_stringIndex.clone(), a_stringName.clone())
        },
        (mut txt, ref i_var @ SimCodeVar::SimVar { index: ref i_var_index, varKind: BackendDAE::VarKind::SEED_VAR, .. }, mut a_stringIndex, mut a_stringName) => {
            a_stringName = CodegenUtil::crefCCommentWithVariability(a_stringName.clone(), i_var.clone())?;
            a_stringIndex = Tpl::writeStr(a_stringIndex.clone(), (intString(i_var_index.clone())).clone())?;
            (txt.clone(), a_stringIndex.clone(), a_stringName.clone())
        },
        (mut txt, mut i_var @ SimCodeVar::SimVar { name: _, .. }, mut a_stringIndex, mut a_stringName) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut ret_0: SimCode::SimCode = <SimCode::SimCode as ::std::default::Default>::default();
            a_stringName = CodegenUtil::crefCCommentWithVariability(a_stringName.clone(), i_var.clone())?;
            ret_0 = SimCodeUtil::getSimCode()?;
            ret_1 = (SimCodeUtil::getValueReference(i_var.clone(), ret_0.clone(), false)?).clone();
            a_stringIndex = Tpl::writeStr(a_stringIndex.clone(), (ret_1.clone()).clone())?;
            (txt.clone(), a_stringIndex.clone(), a_stringName.clone())
        },
        (mut txt, _, mut a_stringIndex, mut a_stringName) => {
            (txt.clone(), a_stringIndex.clone(), a_stringName.clone())
        },
    });
    Ok((out_txt, out_a_stringIndex, out_a_stringName))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_82(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_omsiFuncName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_omsiFuncName.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_variable, tail: rest }, a_omsiFuncName) => {
            let mut x_i0: i32 = 0;
            let mut l_0__: Tpl::Text;
            let mut l_stringIndex: Tpl::Text;
            let mut l_stringName: Tpl::Text;
            let mut l_stringType: Tpl::Text;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            l_stringType = fun_80(Tpl::emptyTxt.clone(), i_variable.clone())?;
            l_stringName = Tpl::emptyTxt.clone();
            l_stringIndex = Tpl::emptyTxt.clone();
            (l_0__, l_stringIndex, l_stringName) = fun_81(Tpl::emptyTxt.clone(), i_variable.clone(), l_stringIndex.clone(), l_stringName.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiFuncName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].type  = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stringType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiFuncName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].index = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stringIndex.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";   ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stringName.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_82(txt.clone(), rest.clone(), (a_omsiFuncName.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_mArg: i32, mut in_a_stringBuffer: Tpl::Text, mut in_a_targetName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringBuffer.clone(), in_a_targetName.clone()) {
        (mut txt, 0, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_stringBuffer, mut a_targetName) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* maps to ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_targetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" */\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringBuffer.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn generateOmsiIndexTypeInitialization(mut txt: Tpl::Text, mut a_variables: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_StrucPrefix: ArcStr, mut a_targetName: ArcStr, mut a_omsiFuncName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: i32 = 0;
    let mut l_stringBuffer: Tpl::Text;
    l_stringBuffer = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_stringBuffer = lm_82(l_stringBuffer.clone(), a_variables.clone(), (a_omsiFuncName.clone()).clone())?;
    l_stringBuffer = Tpl::popIter(l_stringBuffer.clone())?;
    ret_1 = (a_variables.clone().len() as i32);
    out_txt = fun_83(txt.clone(), ret_1.clone(), l_stringBuffer.clone(), (a_targetName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_85(mut in_txt: Tpl::Text, mut in_a_nAlgebraicSystems: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_nAlgebraicSystems.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  #include <solver_api.h>")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_nAlgebraicSystems: i32, mut in_a_algSystemInit: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_nAlgebraicSystems.clone(), in_a_algSystemInit.clone()) {
        (mut txt, 0, _) => {
            txt.clone()
        },
        (mut txt, mut i_nAlgebraicSystems, mut a_algSystemInit) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* Instantiate algebraic system */\n")).clone(), (literal!("omsi_function->algebraic_system_t = omsu_instantiate_alg_system_array(")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_nAlgebraicSystems.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("if (!omsi_function->algebraic_system_t) {\n")).clone(), (literal!("  filtered_base_logger(global_logCategories, log_statuserror, omsi_error,\n")).clone(), (literal!("                \"fmi2Instantiate: Not enough memory.\");\n")).clone(), (literal!("  return omsi_error;\n")).clone(), (literal!("}\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_algSystemInit.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_a_hasLocalVars: bool, mut in_a_nAllVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_hasLocalVars.clone(), in_a_nAllVars.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_nAllVars) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* Allocate memory for local variables */\n")).clone(), (literal!("omsi_function->local_vars = instantiate_omsi_values(")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_nAllVars.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(", 0, 0, 0);\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_mArg: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_89(mut in_txt: Tpl::Text, mut in_a_omsiFunction: Arc<SimCode::OMSIFunction>, mut in_a_functionName: ArcStr, mut in_a_FileNamePrefix: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_includes: Tpl::Text, mut in_a_hasLocalVars: bool, mut in_a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    (out_txt, out_a_functionPrototypes, out_a_includes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_omsiFunction.clone(), in_a_functionName.clone(), in_a_FileNamePrefix.clone(), in_a_functionPrototypes.clone(), in_a_includes.clone(), in_a_hasLocalVars.clone(), in_a_omsiName.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { nAllVars: i_nAllVars, outputVars: i_outputVars, innerVars: i_innerVars, inputVars: i_inputVars, equations: i_equations, nAlgebraicSystems: i_nAlgebraicSystems, .. }, a_functionName, a_FileNamePrefix, a_functionPrototypes, a_includes, a_hasLocalVars, a_omsiName) => {
            let mut ret_7: i32 = 0;
            let mut ret_6: i32 = 0;
            let mut ret_5: i32 = 0;
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut ret_2: i32 = 0;
            let mut l_algSystemInit: Tpl::Text;
            let mut l_evaluationTarget: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            let mut a_includes = (*a_includes).clone();
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_status ")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_functionName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_OMSIFunc (struct omsi_function_t* omsi_function);\n")).clone() }))?;
            l_evaluationTarget = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_FileNamePrefix.clone()).clone())?;
            l_evaluationTarget = Tpl::writeTok(l_evaluationTarget.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            l_evaluationTarget = Tpl::writeStr(l_evaluationTarget.clone(), (a_omsiName.clone()).clone())?;
            l_evaluationTarget = Tpl::writeTok(l_evaluationTarget.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            l_evaluationTarget = Tpl::writeStr(l_evaluationTarget.clone(), (a_functionName.clone()).clone())?;
            l_algSystemInit = generateAlgebraicSystemInstantiation(Tpl::emptyTxt.clone(), (a_FileNamePrefix.clone()).clone(), i_nAlgebraicSystems.clone(), i_equations.clone(), (a_omsiName.clone()).clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#include <omsu_helper.h>\n")).clone() }))?;
            a_includes = fun_85(a_includes.clone(), i_nAlgebraicSystems.clone())?;
            a_includes = Tpl::softNewLine(a_includes.clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <omsi_input_sim_data.h>")).clone() }))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_functionName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_OMSIFunc (struct omsi_function_t* omsi_function) {\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("  filtered_base_logger(global_logCategories, log_all, omsi_ok,\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"fmi2Instantiate: Instantiate omsi_function ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_functionName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".\");\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_function->n_algebraic_system = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_nAlgebraicSystems.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("\n")).clone(), (literal!("omsi_function->n_input_vars = ")).clone()], lastHasNewLine: false }))?;
            ret_2 = (i_inputVars.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("omsi_function->n_inner_vars = ")).clone()], lastHasNewLine: false }))?;
            ret_3 = (i_innerVars.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_3.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("omsi_function->n_output_vars = ")).clone()], lastHasNewLine: false }))?;
            ret_4 = (i_outputVars.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_4.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = fun_86(txt.clone(), i_nAlgebraicSystems.clone(), l_algSystemInit.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("if (instantiate_input_inner_output_indices (omsi_function, ")).clone()], lastHasNewLine: false }))?;
            ret_5 = (i_inputVars.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_5.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            ret_6 = (i_outputVars.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_6.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(") == omsi_error) {\n")).clone(), (literal!("  return omsi_error;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = fun_87(txt.clone(), a_hasLocalVars.clone(), i_nAllVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* fill omsi_index_type indices */\n")).clone() }))?;
            txt = generateOmsiIndexTypeInitialization(txt.clone(), i_inputVars.clone(), (literal!("omsi_function->input_vars_indices")).clone(), (literal!("sim_data->model_vars_and_params")).clone(), (literal!("omsi_function->input_vars_indices")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_7 = (i_inputVars.clone().len() as i32);
            txt = fun_88(txt.clone(), ret_7.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = generateOmsiIndexTypeInitialization(txt.clone(), i_outputVars.clone(), (literal!("omsi_function->output_vars_indices")).clone(), (literal!("sim_data->model_vars_and_params")).clone(), (literal!("omsi_function->output_vars_indices")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("/* Set pointer for evaluation function */\n")).clone(), (literal!("omsi_function->evaluate = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_evaluationTarget.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("\n")).clone(), (literal!("return omsi_ok;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_functionPrototypes.clone(), a_includes.clone())
        },
        (txt, _, _, _, a_functionPrototypes, a_includes, _, _) => {
            (txt.clone(), a_functionPrototypes.clone(), a_includes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes, out_a_includes))
}

pub fn generateInitalizationOMSIFunction(mut txt: Tpl::Text, mut a_omsiFunction: Arc<SimCode::OMSIFunction>, mut a_functionName: ArcStr, mut a_FileNamePrefix: ArcStr, mut a_modelFunctionnamePrefixStr: ArcStr, mut a_functionPrototypes: Tpl::Text, mut a_includes: Tpl::Text, mut a_hasLocalVars: bool, mut a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    (out_txt, out_a_functionPrototypes, out_a_includes) = fun_89(txt.clone(), a_omsiFunction.clone(), (a_functionName.clone()).clone(), (a_FileNamePrefix.clone()).clone(), a_functionPrototypes.clone(), a_includes.clone(), a_hasLocalVars.clone(), (a_omsiName.clone()).clone())?;
    Ok((out_txt, out_a_functionPrototypes, out_a_includes))
}

fn fun_91(mut in_txt: Tpl::Text, mut in_a_equation: Arc<SimCode::SimEqSystem>, mut in_a_i0: i32, mut in_a_omsiName: ArcStr, mut in_a_FileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equation.clone(), in_a_i0.clone(), in_a_omsiName.clone(), in_a_FileNamePrefix.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_algSysIndex, .. }, a_i0, a_omsiName, a_FileNamePrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_AlgSystem_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(&(omsi_function->algebraic_system_t[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("]), omsi_function->function_vars, omsi_function->pre_vars);\n")).clone(), (literal!("if (!&omsi_function->algebraic_system_t[")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("]) {\n")).clone(), (literal!("  filtered_base_logger(global_logCategories, log_statuserror, omsi_error,\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"fmi2Instantiate: Function ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_AlgSystem_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" failed.\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  return omsi_error;\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_92(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_omsiName: ArcStr, mut in_a_FileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_omsiName.clone(), in_a_FileNamePrefix.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_equation, tail: rest }, a_omsiName, a_FileNamePrefix) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = fun_91(txt.clone(), i_equation.clone(), x_i0.clone(), (a_omsiName.clone()).clone(), (a_FileNamePrefix.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_92(txt.clone(), rest.clone(), (a_omsiName.clone()).clone(), (a_FileNamePrefix.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn generateAlgebraicSystemInstantiation(mut txt: Tpl::Text, mut a_FileNamePrefix: ArcStr, mut a_nAlgebraicSystems: i32, mut a_equations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_initialization: Tpl::Text;
    l_initialization = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_initialization = lm_92(l_initialization.clone(), a_equations.clone(), (a_omsiName.clone()).clone(), (a_FileNamePrefix.clone()).clone())?;
    l_initialization = Tpl::popIter(l_initialization.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_initialization.clone())?;
    Ok(out_txt)
}

fn fun_94(mut in_txt: Tpl::Text, mut in_a_timeEvent: BackendDAE::TimeEvent, mut in_a_i0: i32, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls) = (match (in_txt.clone(), in_a_timeEvent.clone(), in_a_i0.clone(), in_a_auxFunction.clone(), in_a_varDecls.clone()) {
        (mut txt, BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { index: mut i_index, intervalExp: ref i_intervalExp, startExp: ref i_startExp }, mut a_i0, mut a_auxFunction, mut a_varDecls) => {
            let mut l_e2: Tpl::Text;
            let mut l_e1: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            l_preExp = Tpl::emptyTxt.clone();
            (l_e1, l_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_startExp.clone(), SimCodeFunction::contextOther().clone(), l_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            (l_e2, l_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_intervalExp.clone(), SimCodeFunction::contextOther().clone(), l_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* sample ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" */\n")).clone(), (literal!("sample_events[")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].id = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("sample_events[")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].start_time = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_e1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("sample_events[")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].interval = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_e2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        (mut txt, _, _, mut a_auxFunction, mut a_varDecls) => {
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
    });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls))
}

fn lm_95(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<BackendDAE::TimeEvent>>, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_auxFunction.clone(), in_a_varDecls.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_auxFunction, a_varDecls) => {
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_timeEvent, tail: rest }, a_auxFunction, a_varDecls) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_auxFunction, a_varDecls) = fun_94(txt.clone(), i_timeEvent.clone(), x_i0.clone(), a_auxFunction.clone(), a_varDecls.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_auxFunction, a_varDecls) = lm_95(txt.clone(), rest.clone(), a_auxFunction.clone(), a_varDecls.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls))
}

pub fn functionInitSample(mut txt: Tpl::Text, mut a_timeEvents: Arc<metamodelica::List<BackendDAE::TimeEvent>>, mut a_fileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_body: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    l_body = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_body, l_auxFunction, l_varDecls) = lm_95(l_body.clone(), a_timeEvents.clone(), l_auxFunction.clone(), l_varDecls.clone())?;
    l_body = Tpl::popIter(l_body.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_auxFunction.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" /* Initializes sample time events */\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_fileNamePrefix.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_instantiate_samples(omsi_sample* sample_events)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_varDecls.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_body.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok(out_txt)
}

pub fn insertCopyrightOpenModelica(mut txt: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/*\n")).clone(), (literal!(" * This file is part of OpenModelica.\n")).clone(), (literal!(" *\n")).clone(), (literal!(" * Copyright (c) 1998-2014, Open Source Modelica Consortium (OSMC),\n")).clone(), (literal!(" * c/o Linköpings universitet, Department of Computer and Information Science,\n")).clone(), (literal!(" * SE-58183 Linköping, Sweden.\n")).clone(), (literal!(" *\n")).clone(), (literal!(" * All rights reserved.\n")).clone(), (literal!(" *\n")).clone(), (literal!(" * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 LICENSE OR\n")).clone(), (literal!(" * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2.\n")).clone(), (literal!(" * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES\n")).clone(), (literal!(" * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3,\n")).clone(), (literal!(" * ACCORDING TO RECIPIENTS CHOICE.\n")).clone(), (literal!(" *\n")).clone(), (literal!(" * The OpenModelica software and the Open Source Modelica\n")).clone(), (literal!(" * Consortium (OSMC) Public License (OSMC-PL) are obtained\n")).clone(), (literal!(" * from OSMC, either from the above address,\n")).clone(), (literal!(" * from the URLs: http://www.ida.liu.se/projects/OpenModelica or\n")).clone(), (literal!(" * http://www.openmodelica.org, and in the OpenModelica distribution.\n")).clone(), (literal!(" * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.\n")).clone(), (literal!(" *\n")).clone(), (literal!(" * This program is distributed WITHOUT ANY WARRANTY; without\n")).clone(), (literal!(" * even the implied warranty of  MERCHANTABILITY or FITNESS\n")).clone(), (literal!(" * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH\n")).clone(), (literal!(" * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.\n")).clone(), (literal!(" *\n")).clone(), (literal!(" * See the full OSMC Public License conditions for more details.\n")).clone(), (literal!(" *\n")).clone(), (literal!(" */")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

