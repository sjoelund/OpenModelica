// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenFMU;
use crate::CodegenOMSI_common;
use crate::CodegenUtilSimulation;
use openmodelica_ast::Absyn;
use openmodelica_backend::BackendDAE;
use openmodelica_backend::CodegenUtil;
use openmodelica_backend::SimCode;
use openmodelica_backend::SimCodeFunction;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::FMI;
use openmodelica_util::Flags;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;

fn fun_52(mut in_txt: Tpl::Text, mut in_a_varInfo_numZeroCrossings: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_varInfo_numZeroCrossings.clone()) {
        (mut txt, 0) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <omsi_event_helper.h>")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_includes: Tpl::Text, mut in_a_functionPrototypes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_includes: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_includes, out_a_functionPrototypes) = (match (in_txt.clone(), in_a_simCode.clone(), in_a_includes.clone(), in_a_functionPrototypes.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { timeEvents: ref i_timeEvents, modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, varInfo: SimCode::VarInfo { numZeroCrossings: ref i_varInfo_numZeroCrossings, .. }, functions: _, .. }, fullPathPrefix: ref i_fullPathPrefix, fileNamePrefix: ref i_fileNamePrefix, .. }, mut a_includes, mut a_functionPrototypes) => {
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut l_functionInitSampleCode: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut l_headerFileContent: Tpl::Text;
            let mut l_headerFileName: Tpl::Text;
            let mut l_modelNamePrefixStr: Tpl::Text;
            l_modelNamePrefixStr = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void initialize_start_function (omsi_template_callback_functions_t* callback);")).clone() }))?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            a_functionPrototypes = Tpl::writeText(a_functionPrototypes.clone(), l_modelNamePrefixStr.clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_instantiate_samples(omsi_sample* sample_events);")).clone() }))?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            a_includes = Tpl::writeStr(a_includes.clone(), (i_fileNamePrefix.clone()).clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns.h\"")).clone() }))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            a_includes = Tpl::writeStr(a_includes.clone(), (i_fileNamePrefix.clone()).clone())?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_eqns.h\"")).clone() }))?;
            a_includes = Tpl::writeTok(a_includes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_includes = fun_52(a_includes.clone(), i_varInfo_numZeroCrossings.clone())?;
            l_headerFileName = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            l_headerFileName = Tpl::writeTok(l_headerFileName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_omsic")).clone() }))?;
            (l_headerFileContent, a_includes, a_functionPrototypes) = CodegenOMSI_common::generateCodeHeader(Tpl::emptyTxt.clone(), (Tpl::textString(l_modelNamePrefixStr.clone())?).clone(), a_includes.clone(), (Tpl::textString(l_headerFileName.clone())?).clone(), a_functionPrototypes.clone())?;
            txt_3 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fullPathPrefix.clone()).clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt_3 = Tpl::writeText(txt_3.clone(), l_headerFileName.clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
            Tpl::textFile(l_headerFileContent.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            l_functionInitSampleCode = CodegenOMSI_common::functionInitSample(Tpl::emptyTxt.clone(), i_timeEvents.clone(), (Tpl::textString(l_modelNamePrefixStr.clone())?).clone())?;
            txt = CodegenOMSI_common::insertCopyrightOpenModelica(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#include \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_omsic.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("/* Simulation code for ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::dotPath(txt.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" generated by the OpenModelica Compiler ")).clone() }))?;
            ret_5 = (Settings::getVersionNr()).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_5.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(". */\n")).clone(), (literal!("/* Translated model ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelNamePrefixStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" to OMSIC */\n")).clone(), (literal!("\n")).clone(), (literal!("/* Set function pointers for initialization in global struct. */\n")).clone(), (literal!("void initialize_start_function (omsi_template_callback_functions_t* callback) {\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("callback->initialize_initialization_problem = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelNamePrefixStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init_eqns_instantiate_allEqns_OMSIFunc;\n")).clone(), (literal!("callback->initialize_simulation_problem = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelNamePrefixStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_sim_eqns_instantiate_allEqns_OMSIFunc;\n")).clone(), (literal!("\n")).clone(), (literal!("callback->initialize_samples = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_modelNamePrefixStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_instantiate_samples;\n")).clone(), (literal!("\n")).clone(), (literal!("callback->isSet = omsi_true;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_functionInitSampleCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_includes.clone(), a_functionPrototypes.clone())
        },
        (mut txt, _, mut a_includes, mut a_functionPrototypes) => {
            (txt.clone(), a_includes.clone(), a_functionPrototypes.clone())
        },
    });
    Ok((out_txt, out_a_includes, out_a_functionPrototypes))
}

pub fn generateOMSIC(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_includes: Tpl::Text;
    let mut l_functionPrototypes: Tpl::Text;
    l_functionPrototypes = Tpl::emptyTxt.clone();
    l_includes = Tpl::emptyTxt.clone();
    (out_txt, l_includes, l_functionPrototypes) = fun_53(txt.clone(), a_simCode.clone(), l_includes.clone(), l_functionPrototypes.clone())?;
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_fileNamePrefix: ArcStr, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_InitDerMatFiles: Tpl::Text;
    let mut out_a_InitAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_InitDerMatFiles.clone(), in_a_fileNamePrefix.clone(), in_a_InitAlgSystemFiles.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_system_algSysIndex, matrix: None, .. }, a_InitDerMatFiles, a_fileNamePrefix, a_InitAlgSystemFiles) => {
            let mut a_InitAlgSystemFiles = (*a_InitAlgSystemFiles).clone();
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_InitAlgSystemFiles = Tpl::writeStr(a_InitAlgSystemFiles.clone(), (a_fileNamePrefix.clone()).clone())?;
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_eqns_algSyst_")).clone() }))?;
            a_InitAlgSystemFiles = Tpl::writeStr(a_InitAlgSystemFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_system_algSysIndex, .. }, a_InitDerMatFiles, a_fileNamePrefix, a_InitAlgSystemFiles) => {
            let mut a_InitDerMatFiles = (*a_InitDerMatFiles).clone();
            let mut a_InitAlgSystemFiles = (*a_InitAlgSystemFiles).clone();
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_InitAlgSystemFiles = Tpl::writeStr(a_InitAlgSystemFiles.clone(), (a_fileNamePrefix.clone()).clone())?;
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_eqns_algSyst_")).clone() }))?;
            a_InitAlgSystemFiles = Tpl::writeStr(a_InitAlgSystemFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            a_InitDerMatFiles = Tpl::writeTok(a_InitDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_InitDerMatFiles = Tpl::writeStr(a_InitDerMatFiles.clone(), (a_fileNamePrefix.clone()).clone())?;
            a_InitDerMatFiles = Tpl::writeTok(a_InitDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_eqns_derMat_")).clone() }))?;
            a_InitDerMatFiles = Tpl::writeStr(a_InitDerMatFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_InitDerMatFiles = Tpl::writeTok(a_InitDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        (txt, _, a_InitDerMatFiles, _, a_InitAlgSystemFiles) => {
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles))
}

fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_fileNamePrefix: ArcStr, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_InitDerMatFiles: Tpl::Text;
    let mut out_a_InitAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_InitDerMatFiles.clone(), in_a_fileNamePrefix.clone(), in_a_InitAlgSystemFiles.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_InitDerMatFiles, _, a_InitAlgSystemFiles) => {
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_InitDerMatFiles, a_fileNamePrefix, a_InitAlgSystemFiles) => {
            let mut txt = (*txt).clone();
            let mut a_InitDerMatFiles = (*a_InitDerMatFiles).clone();
            let mut a_InitAlgSystemFiles = (*a_InitAlgSystemFiles).clone();
            (txt, a_InitDerMatFiles, a_InitAlgSystemFiles) = fun_55(txt.clone(), i_eq.clone(), a_InitDerMatFiles.clone(), (a_fileNamePrefix.clone()).clone(), a_InitAlgSystemFiles.clone())?;
            (txt, a_InitDerMatFiles, a_InitAlgSystemFiles) = lm_56(txt.clone(), rest.clone(), a_InitDerMatFiles.clone(), (a_fileNamePrefix.clone()).clone(), a_InitAlgSystemFiles.clone())?;
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles))
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_fileNamePrefix: ArcStr, mut in_a_SimAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_SimDerMatFiles: Tpl::Text;
    let mut out_a_SimAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_SimDerMatFiles.clone(), in_a_fileNamePrefix.clone(), in_a_SimAlgSystemFiles.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_system_algSysIndex, matrix: None, .. }, a_SimDerMatFiles, a_fileNamePrefix, a_SimAlgSystemFiles) => {
            let mut a_SimDerMatFiles = (*a_SimDerMatFiles).clone();
            let mut a_SimAlgSystemFiles = (*a_SimAlgSystemFiles).clone();
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_SimAlgSystemFiles = Tpl::writeStr(a_SimAlgSystemFiles.clone(), (a_fileNamePrefix.clone()).clone())?;
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns_algSyst_")).clone() }))?;
            a_SimAlgSystemFiles = Tpl::writeStr(a_SimAlgSystemFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeStr(a_SimDerMatFiles.clone(), (a_fileNamePrefix.clone()).clone())?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns_derMat_")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeStr(a_SimDerMatFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_system_algSysIndex, .. }, a_SimDerMatFiles, a_fileNamePrefix, a_SimAlgSystemFiles) => {
            let mut a_SimDerMatFiles = (*a_SimDerMatFiles).clone();
            let mut a_SimAlgSystemFiles = (*a_SimAlgSystemFiles).clone();
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_SimAlgSystemFiles = Tpl::writeStr(a_SimAlgSystemFiles.clone(), (a_fileNamePrefix.clone()).clone())?;
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns_algSyst_")).clone() }))?;
            a_SimAlgSystemFiles = Tpl::writeStr(a_SimAlgSystemFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeStr(a_SimDerMatFiles.clone(), (a_fileNamePrefix.clone()).clone())?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns_derMat_")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeStr(a_SimDerMatFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        (txt, _, a_SimDerMatFiles, _, a_SimAlgSystemFiles) => {
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles))
}

fn lm_58(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_fileNamePrefix: ArcStr, mut in_a_SimAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_SimDerMatFiles: Tpl::Text;
    let mut out_a_SimAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_SimDerMatFiles.clone(), in_a_fileNamePrefix.clone(), in_a_SimAlgSystemFiles.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_SimDerMatFiles, _, a_SimAlgSystemFiles) => {
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_SimDerMatFiles, a_fileNamePrefix, a_SimAlgSystemFiles) => {
            let mut txt = (*txt).clone();
            let mut a_SimDerMatFiles = (*a_SimDerMatFiles).clone();
            let mut a_SimAlgSystemFiles = (*a_SimAlgSystemFiles).clone();
            (txt, a_SimDerMatFiles, a_SimAlgSystemFiles) = fun_57(txt.clone(), i_eq.clone(), a_SimDerMatFiles.clone(), (a_fileNamePrefix.clone()).clone(), a_SimAlgSystemFiles.clone())?;
            (txt, a_SimDerMatFiles, a_SimAlgSystemFiles) = lm_58(txt.clone(), rest.clone(), a_SimDerMatFiles.clone(), (a_fileNamePrefix.clone()).clone(), a_SimAlgSystemFiles.clone())?;
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles))
}

fn fun_59(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_SimAlgSystemFiles: Tpl::Text, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_SimDerMatFiles: Tpl::Text;
    let mut out_a_SimAlgSystemFiles: Tpl::Text;
    let mut out_a_InitDerMatFiles: Tpl::Text;
    let mut out_a_InitAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_SimDerMatFiles.clone(), in_a_SimAlgSystemFiles.clone(), in_a_InitDerMatFiles.clone(), in_a_InitAlgSystemFiles.clone())) {
        (txt, SimCode::SimCode { omsiData: Some(SimCode::OMSIData { initialization: Deref @ SimCode::OMSIFunction { equations: i_initialization_equations, .. }, simulation: Deref @ SimCode::OMSIFunction { equations: i_simulation_equations, .. } }), fileNamePrefix: i_fileNamePrefix, .. }, a_SimDerMatFiles, a_SimAlgSystemFiles, a_InitDerMatFiles, a_InitAlgSystemFiles) => {
            let mut l_0___1: Tpl::Text;
            let mut l_0__: Tpl::Text;
            let mut a_SimDerMatFiles = (*a_SimDerMatFiles).clone();
            let mut a_SimAlgSystemFiles = (*a_SimAlgSystemFiles).clone();
            let mut a_InitDerMatFiles = (*a_InitDerMatFiles).clone();
            let mut a_InitAlgSystemFiles = (*a_InitAlgSystemFiles).clone();
            (l_0__, a_InitDerMatFiles, a_InitAlgSystemFiles) = lm_56(Tpl::emptyTxt.clone(), i_initialization_equations.clone(), a_InitDerMatFiles.clone(), (i_fileNamePrefix.clone()).clone(), a_InitAlgSystemFiles.clone())?;
            (l_0___1, a_SimDerMatFiles, a_SimAlgSystemFiles) = lm_58(Tpl::emptyTxt.clone(), i_simulation_equations.clone(), a_SimDerMatFiles.clone(), (i_fileNamePrefix.clone()).clone(), a_SimAlgSystemFiles.clone())?;
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        (txt, _, a_SimDerMatFiles, a_SimAlgSystemFiles, a_InitDerMatFiles, a_InitAlgSystemFiles) => {
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles))
}

fn fun_60(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
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

fn fun_61(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("lib")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("lib")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "linux64") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("lib/x86_64-linux-gnu")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("lib")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_62(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win64") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(MSYSTEM_PREFIX)/bin")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_63(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dll")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dll")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("so")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_64(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"-Wl,-rpath,\\$$ORIGIN/.\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_65(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-fPIC ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_66(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("openblas")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("openblas")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("lapack")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_67(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("blas")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone())) {
        (txt, Deref @ "win32") => {
            txt.clone()
        },
        (txt, Deref @ "win64") => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-l$(BLAS_LIB)")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_69(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr, mut in_a_libEnding: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_makefileParams_platform.clone(), in_a_libEnding.clone())) {
        (txt, Deref @ "win32", a_libEnding) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(LAPACK_LIBDIR)/lib$(LAPACK_LIB).")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_libEnding.clone())?;
            txt.clone()
        },
        (txt, Deref @ "win64", a_libEnding) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(LAPACK_LIBDIR)/lib$(LAPACK_LIB).")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_libEnding.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_70(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_fileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fileNamePrefix.clone())) {
        (txt, Deref @ "omsicpp", a_fileNamePrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("chmod +x ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".sh")).clone() }))?;
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

fn fun_71(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fileNamePrefix.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_fileNamePrefix) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            ret_0 = (Config::simCodeTarget()?).clone();
            txt = fun_70(txt.clone(), (ret_0.clone()).clone(), (a_fileNamePrefix.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_SimAlgSystemFiles: Tpl::Text, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_SimDerMatFiles.clone(), in_a_SimAlgSystemFiles.clone(), in_a_InitDerMatFiles.clone(), in_a_InitAlgSystemFiles.clone()) {
        (mut txt, SimCode::SimCode { fmuTargetName: mut i_fmuTargetName, fileNamePrefix: mut i_fileNamePrefix, simulationSettingsOpt: _, makefileParams: SimCodeFunction::MakefileParams { dllext: mut i_makefileParams_dllext, exeext: mut i_makefileParams_exeext, cxxcompiler: mut i_makefileParams_cxxcompiler, ccompiler: mut i_makefileParams_ccompiler, omhome: mut i_makefileParams_omhome, platform: mut i_makefileParams_platform, .. }, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }, mut a_SimDerMatFiles, mut a_SimAlgSystemFiles, mut a_InitDerMatFiles, mut a_InitAlgSystemFiles) => {
            let mut ret_10: bool = false;
            let mut ret_9: bool = false;
            let mut ret_8: bool = false;
            let mut ret_7: bool = false;
            let mut l_fPIC: Tpl::Text;
            let mut l_rpath: Tpl::Text;
            let mut l_libEnding: Tpl::Text;
            let mut l_lapackDirWin: Tpl::Text;
            let mut l_OMLibs: Tpl::Text;
            let mut l_mkdir: Tpl::Text;
            let mut l_includedir: Tpl::Text;
            l_includedir = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            l_includedir = Tpl::writeTok(l_includedir.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources/include/")).clone() }))?;
            l_mkdir = fun_60(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_OMLibs = fun_61(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_lapackDirWin = fun_62(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_libEnding = fun_63(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_rpath = fun_64(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_fPIC = fun_65(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# Makefile generated by OpenModelica\n")).clone(), (literal!("OMHOME='")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("'\n")).clone(), (literal!("OMLIB='")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_OMLibs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("'\n")).clone(), (literal!("\n")).clone(), (literal!("CC=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_ccompiler.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CFLAGS= ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fPIC.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("-Wall -Wextra -pedantic -g\n")).clone(), (literal!("CXX=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_cxxcompiler.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("LD=$(CC) -shared\n")).clone(), (literal!("\n")).clone(), (literal!("RUNTIMEPATH=.\n")).clone(), (literal!("\n")).clone(), (literal!("# Files\n")).clone(), (literal!("MAINFILE=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_omsic.c\n")).clone(), (literal!("MAINOBJ=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_omsic.o\n")).clone(), (literal!("INIT_FILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init_eqns.c $(INIT_ALGLOOP_FILES) $(INIT_DERMAT_FILES)\n")).clone(), (literal!("INIT_ALGLOOP_FILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_InitAlgSystemFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INIT_DERMAT_FILES=")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_InitDerMatFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SIM_FILES=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_sim_eqns.c $(SIM_ALGLOOP_FILES) $(SIM_DERMAT_FILES)\n")).clone(), (literal!("SIM_ALGLOOP_FILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_SimAlgSystemFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SIM_DERMAT_FILES=")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_SimDerMatFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("CFILES=$(MAINFILE) $(INIT_FILES) $(SIM_FILES)\n")).clone(), (literal!("OFILES=$(CFILES:.c=.o)\n")).clone(), (literal!("HFILES=$(CFILES:.c=.h)\n")).clone(), (literal!("\n")).clone(), (literal!("RESOURCE_FILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_info.json ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init.xml\n")).clone(), (literal!("\n")).clone(), (literal!("GENERATEDFILES=$(MAINFILE) ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.makefile # ...\n")).clone(), (literal!("\n")).clone(), (literal!("# Includes\n")).clone(), (literal!("INCLUDE_DIR_OMSI=$(OMHOME)/include/omc/omsi\n")).clone(), (literal!("INCLUDE_DIR_OMSI_BASE=$(OMHOME)/include/omc/omsi/base\n")).clone(), (literal!("INCLUDE_DIR_OMSI_SOLVER=$(OMHOME)/include/omc/omsi/solver\n")).clone(), (literal!("INCLUDE_DIR_OMSI_FMI2=$(OMHOME)/include/omc/omsi/fmi2\n")).clone(), (literal!("INCLUDE_DIR_OMSIC=$(OMHOME)/include/omc/omsic\n")).clone(), (literal!("INCLUDE_DIR_OMSU=$(OMHOME)/include/omc/omsic/omsu\n")).clone(), (literal!("\n")).clone(), (literal!("# Libraries\n")).clone(), (literal!("EXPAT_LIBDIR=$(OMLIB)/omc\n")).clone(), (literal!("EXPAT_LIB=expat\n")).clone(), (literal!("\n")).clone(), (literal!("LAPACK_LIBDIR=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_lapackDirWin.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LAPACK_LIB=")).clone() }))?;
            txt = fun_66(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("BLAS_LIB=")).clone() }))?;
            txt = fun_67(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("KINSOL_LIBDIR=$(OMLIB)/omc\n")).clone(), (literal!("KINSOL_LIB=sundials_kinsol\n")).clone(), (literal!("SUNDIALS_NVECSERIAL=sundials_nvecserial\n")).clone(), (literal!("\n")).clone(), (literal!("OMSU_STATIC_LIB=-Wl,--whole-archive -lOMSISolver_static -lOMSIBase_static -lOMSIC_static -Wl,--no-whole-archive\n")).clone(), (literal!("OMSU_STATIC_LIBDIR=-L$(OMLIB)/omc/omsi\n")).clone(), (literal!("LIBS = $(OMSU_STATIC_LIB) -Wl,-Bdynamic -l$(EXPAT_LIB) -l$(LAPACK_LIB) ")).clone()], lastHasNewLine: false }))?;
            txt = fun_68(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" -l$(KINSOL_LIB) -l$(SUNDIALS_NVECSERIAL)\n")).clone(), (literal!("LIBSDIR= $(OMSU_STATIC_LIBDIR) -L$(EXPAT_LIBDIR) -L$(LAPACK_LIBDIR) -L$(KINSOL_LIBDIR)\n")).clone(), (literal!("\n")).clone(), (literal!("THIRD_PARTY_DYNAMIC_LIBS =")).clone()], lastHasNewLine: false }))?;
            txt = fun_69(txt.clone(), (i_makefileParams_platform.clone()).clone(), l_libEnding.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("       \\\n")).clone(), (literal!(" $(KINSOL_LIBDIR)/lib$(KINSOL_LIB).*                                \\\n")).clone(), (literal!(" $(KINSOL_LIBDIR)/lib$(SUNDIALS_NVECSERIAL).*                       \\\n")).clone(), (literal!("\n")).clone(), (literal!(".PHONY: copyFiles makeStructure compile fmiImport OMSimulation clean\n")).clone(), (literal!("\n")).clone(), (literal!("all: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmu: compile\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp; \\\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("zip")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_exeext.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -r ../../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmu *;\\\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("cd ..;\\\n")).clone() }))?;
            ret_7 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win32")).clone());
            ret_8 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win64")).clone());
            ret_9 = boolOr(ret_7.clone(), ret_8.clone());
            ret_10 = boolNot(ret_9.clone());
            txt = fun_71(txt.clone(), ret_10.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("copyFiles: makeStructure\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("# Basic OMSI and OMSIC files\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a $(OMHOME)/include/omc/omsi/* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_includedir.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a $(OMHOME)/include/omc/omsic/* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_includedir.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a $(OMLIB)/omc/omsi/libOMSIBase_static.* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/libs\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a $(OMLIB)/omc/omsi/libOMSIC_static.* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/libs\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a $(OMLIB)/omc/omsi/libOMSISolver_static.* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/libs\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("# Third party libraries\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -f $(EXPAT_LIBDIR)/lib$(EXPAT_LIB).* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/libs\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -fP $(THIRD_PARTY_DYNAMIC_LIBS) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/binaries/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a modelDescription.xml ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a $(CFILES) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a $(HFILES) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/include\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_info.json ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/resources/\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init.xml ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmutmp/resources/\n")).clone(), (literal!("\n")).clone(), (literal!("makeStructure:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -p ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources/include ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/libs\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -p ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/resources\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -p ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/binaries/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("compile: $(OFILES) copyFiles\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(LD) -o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_dllext.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" $(OFILES) $(LIBSDIR) $(LIBS) ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rpath.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_dllext.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/binaries/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/\n")).clone(), (literal!("\n")).clone(), (literal!("%.o : %.c copyFiles\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("$(CC) $(CFLAGS) -I$(INCLUDE_DIR_OMSI)  -I$(INCLUDE_DIR_OMSI_BASE) -I$(INCLUDE_DIR_OMSI_SOLVER) -I$(INCLUDE_DIR_OMSI_FMI2) -I$(INCLUDE_DIR_OMSIC) -I$(INCLUDE_DIR_OMSU) -c $<\n")).clone(), (literal!("\n")).clone(), (literal!("fmiImport:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd ..; omc ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiImport.mos\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd ..; mv ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_me_FMU ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("OMSimulation:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd ..; @echo \"#!/bin/bash\\nOMSimulator ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".lua\" > ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; chmod +x ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("clean:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_dllext.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -f $(OFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -Rf helloWorld.fmutmp\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.libs ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.log\n")).clone(), (literal!("\n")).clone(), (literal!("purge: clean\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -f $(CFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -f $(HFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("rm -f modelDescription.xml $(RESOURCE_FILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.makefile\n")).clone(), (literal!("\n")).clone(), (literal!("distclean:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f -R ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmutmp\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn createMakefile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_target: ArcStr, mut a_makeflieName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_0__: Tpl::Text;
    let mut l_SimDerMatFiles: Tpl::Text;
    let mut l_SimAlgSystemFiles: Tpl::Text;
    let mut l_InitDerMatFiles: Tpl::Text;
    let mut l_InitAlgSystemFiles: Tpl::Text;
    let mut l_FMUVersion: Tpl::Text;
    l_FMUVersion = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("2.0")).clone() }))?;
    l_InitAlgSystemFiles = Tpl::emptyTxt.clone();
    l_InitDerMatFiles = Tpl::emptyTxt.clone();
    l_SimAlgSystemFiles = Tpl::emptyTxt.clone();
    l_SimDerMatFiles = Tpl::emptyTxt.clone();
    (l_0__, l_SimDerMatFiles, l_SimAlgSystemFiles, l_InitDerMatFiles, l_InitAlgSystemFiles) = fun_59(Tpl::emptyTxt.clone(), a_simCode.clone(), l_SimDerMatFiles.clone(), l_SimAlgSystemFiles.clone(), l_InitDerMatFiles.clone(), l_InitAlgSystemFiles.clone())?;
    out_txt = fun_72(txt.clone(), a_simCode.clone(), l_SimDerMatFiles.clone(), l_SimAlgSystemFiles.clone(), l_InitDerMatFiles.clone(), l_InitAlgSystemFiles.clone())?;
    Ok(out_txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_InitDerMatFiles: Tpl::Text;
    let mut out_a_InitAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_InitDerMatFiles.clone(), in_a_FileNamePrefix.clone(), in_a_InitAlgSystemFiles.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_system_algSysIndex, .. }, a_InitDerMatFiles, a_FileNamePrefix, a_InitAlgSystemFiles) => {
            let mut a_InitDerMatFiles = (*a_InitDerMatFiles).clone();
            let mut a_InitAlgSystemFiles = (*a_InitAlgSystemFiles).clone();
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_InitAlgSystemFiles = Tpl::writeStr(a_InitAlgSystemFiles.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_eqns_algSyst_")).clone() }))?;
            a_InitAlgSystemFiles = Tpl::writeStr(a_InitAlgSystemFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_InitAlgSystemFiles = Tpl::writeTok(a_InitAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            a_InitDerMatFiles = Tpl::writeTok(a_InitDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_InitDerMatFiles = Tpl::writeStr(a_InitDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_InitDerMatFiles = Tpl::writeTok(a_InitDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_eqns_derMat_")).clone() }))?;
            a_InitDerMatFiles = Tpl::writeStr(a_InitDerMatFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_InitDerMatFiles = Tpl::writeTok(a_InitDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        (txt, _, a_InitDerMatFiles, _, a_InitAlgSystemFiles) => {
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles))
}

fn lm_75(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_InitDerMatFiles: Tpl::Text;
    let mut out_a_InitAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_InitDerMatFiles.clone(), in_a_FileNamePrefix.clone(), in_a_InitAlgSystemFiles.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_InitDerMatFiles, _, a_InitAlgSystemFiles) => {
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_InitDerMatFiles, a_FileNamePrefix, a_InitAlgSystemFiles) => {
            let mut txt = (*txt).clone();
            let mut a_InitDerMatFiles = (*a_InitDerMatFiles).clone();
            let mut a_InitAlgSystemFiles = (*a_InitAlgSystemFiles).clone();
            (txt, a_InitDerMatFiles, a_InitAlgSystemFiles) = fun_74(txt.clone(), i_eq.clone(), a_InitDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone(), a_InitAlgSystemFiles.clone())?;
            (txt, a_InitDerMatFiles, a_InitAlgSystemFiles) = lm_75(txt.clone(), rest.clone(), a_InitDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone(), a_InitAlgSystemFiles.clone())?;
            (txt.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles))
}

fn fun_76(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_SimAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_SimDerMatFiles: Tpl::Text;
    let mut out_a_SimAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_SimDerMatFiles.clone(), in_a_FileNamePrefix.clone(), in_a_SimAlgSystemFiles.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_system_algSysIndex, .. }, a_SimDerMatFiles, a_FileNamePrefix, a_SimAlgSystemFiles) => {
            let mut a_SimDerMatFiles = (*a_SimDerMatFiles).clone();
            let mut a_SimAlgSystemFiles = (*a_SimAlgSystemFiles).clone();
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_SimAlgSystemFiles = Tpl::writeStr(a_SimAlgSystemFiles.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns_algSyst_")).clone() }))?;
            a_SimAlgSystemFiles = Tpl::writeStr(a_SimAlgSystemFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_SimAlgSystemFiles = Tpl::writeTok(a_SimAlgSystemFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeStr(a_SimDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone())?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_sim_eqns_derMat_")).clone() }))?;
            a_SimDerMatFiles = Tpl::writeStr(a_SimDerMatFiles.clone(), (intString(i_system_algSysIndex.clone())).clone())?;
            a_SimDerMatFiles = Tpl::writeTok(a_SimDerMatFiles.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        (txt, _, a_SimDerMatFiles, _, a_SimAlgSystemFiles) => {
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles))
}

fn lm_77(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_SimAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_SimDerMatFiles: Tpl::Text;
    let mut out_a_SimAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_SimDerMatFiles.clone(), in_a_FileNamePrefix.clone(), in_a_SimAlgSystemFiles.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_SimDerMatFiles, _, a_SimAlgSystemFiles) => {
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_SimDerMatFiles, a_FileNamePrefix, a_SimAlgSystemFiles) => {
            let mut txt = (*txt).clone();
            let mut a_SimDerMatFiles = (*a_SimDerMatFiles).clone();
            let mut a_SimAlgSystemFiles = (*a_SimAlgSystemFiles).clone();
            (txt, a_SimDerMatFiles, a_SimAlgSystemFiles) = fun_76(txt.clone(), i_eq.clone(), a_SimDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone(), a_SimAlgSystemFiles.clone())?;
            (txt, a_SimDerMatFiles, a_SimAlgSystemFiles) = lm_77(txt.clone(), rest.clone(), a_SimDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone(), a_SimAlgSystemFiles.clone())?;
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles))
}

fn fun_78(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_SimAlgSystemFiles: Tpl::Text, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_FileNamePrefix: ArcStr, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_SimDerMatFiles: Tpl::Text;
    let mut out_a_SimAlgSystemFiles: Tpl::Text;
    let mut out_a_InitDerMatFiles: Tpl::Text;
    let mut out_a_InitAlgSystemFiles: Tpl::Text;
    (out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_SimDerMatFiles.clone(), in_a_SimAlgSystemFiles.clone(), in_a_InitDerMatFiles.clone(), in_a_FileNamePrefix.clone(), in_a_InitAlgSystemFiles.clone())) {
        (txt, SimCode::SimCode { omsiData: Some(SimCode::OMSIData { initialization: Deref @ SimCode::OMSIFunction { equations: i_initialization_equations, .. }, simulation: Deref @ SimCode::OMSIFunction { equations: i_simulation_equations, .. } }), .. }, a_SimDerMatFiles, a_SimAlgSystemFiles, a_InitDerMatFiles, a_FileNamePrefix, a_InitAlgSystemFiles) => {
            let mut l_0___1: Tpl::Text;
            let mut l_0__: Tpl::Text;
            let mut a_SimDerMatFiles = (*a_SimDerMatFiles).clone();
            let mut a_SimAlgSystemFiles = (*a_SimAlgSystemFiles).clone();
            let mut a_InitDerMatFiles = (*a_InitDerMatFiles).clone();
            let mut a_InitAlgSystemFiles = (*a_InitAlgSystemFiles).clone();
            (l_0__, a_InitDerMatFiles, a_InitAlgSystemFiles) = lm_75(Tpl::emptyTxt.clone(), i_initialization_equations.clone(), a_InitDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone(), a_InitAlgSystemFiles.clone())?;
            (l_0___1, a_SimDerMatFiles, a_SimAlgSystemFiles) = lm_77(Tpl::emptyTxt.clone(), i_simulation_equations.clone(), a_SimDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone(), a_SimAlgSystemFiles.clone())?;
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        (txt, _, a_SimDerMatFiles, a_SimAlgSystemFiles, a_InitDerMatFiles, _, a_InitAlgSystemFiles) => {
            (txt.clone(), a_SimDerMatFiles.clone(), a_SimAlgSystemFiles.clone(), a_InitDerMatFiles.clone(), a_InitAlgSystemFiles.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_SimDerMatFiles, out_a_SimAlgSystemFiles, out_a_InitDerMatFiles, out_a_InitAlgSystemFiles))
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_SimDerMatFiles: Tpl::Text, mut in_a_SimAlgSystemFiles: Tpl::Text, mut in_a_InitDerMatFiles: Tpl::Text, mut in_a_InitAlgSystemFiles: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_SimDerMatFiles.clone(), in_a_SimAlgSystemFiles.clone(), in_a_InitDerMatFiles.clone(), in_a_InitAlgSystemFiles.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, simulationSettingsOpt: _, makefileParams: SimCodeFunction::MakefileParams { omhome: mut i_makefileParams_omhome, .. }, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }, mut a_SimDerMatFiles, mut a_SimAlgSystemFiles, mut a_InitDerMatFiles, mut a_InitAlgSystemFiles) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MAINFILE=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_omsic.c\n")).clone(), (literal!("MAINOBJ=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_omsic.o\n")).clone(), (literal!("INIT_FILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init_eqns.c $(INIT_ALGLOOP_FILES) $(INIT_DERMAT_FILES)\n")).clone(), (literal!("INIT_ALGLOOP_FILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_InitAlgSystemFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INIT_DERMAT_FILES=")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_InitDerMatFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SIM_FILES=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_sim_eqns.c $(SIM_ALGLOOP_FILES) $(SIM_DERMAT_FILES)\n")).clone(), (literal!("SIM_ALGLOOP_FILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_SimAlgSystemFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SIM_DERMAT_FILES=")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_SimDerMatFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("CFILES= $(INIT_FILES) $(SIM_FILES)\n")).clone(), (literal!("OFILES=$(CFILES:.c=.o)\n")).clone(), (literal!("GENERATEDFILES=$(MAINFILE) ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.makefile # ...\n")).clone(), (literal!("\n")).clone(), (literal!("# FIXME: before you push into master...\n")).clone(), (literal!("RUNTIMEDIR=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/include/omc/omsic/\n")).clone(), (literal!("OMC_MINIMAL_RUNTIME=1\n")).clone(), (literal!("OMC_FMI_RUNTIME=1\n")).clone(), (literal!("# include $(RUNTIMEDIR)/Makefile.objs\n")).clone(), (literal!("ifneq ($(NEED_RUNTIME),)\n")).clone(), (literal!("RUNTIMEFILES=$(FMI_ME_OBJS:%=$(RUNTIMEDIR)/%.o)\n")).clone(), (literal!("endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_modelInfo_directory: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelInfo_directory.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_modelInfo_directory) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/LIBPATH:\"")).clone() }))?;
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
fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lib, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lib.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_81(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dirExtra.clone(), in_a_libsStr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_libsStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_libsStr.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dirExtra.clone(), in_a_libsStr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_libsStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_libsStr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_s_method: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_s_method.clone())) {
        (txt, Deref @ "inline-euler") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-D_OMC_INLINE_EULER")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "inline-rungekutta") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-D_OMC_INLINE_RK")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_85(mut in_txt: Tpl::Text, mut in_a_sopt: Option<SimCode::SimulationSettings>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_sopt.clone()) {
        (mut txt, Some(SimCode::SimulationSettings { method: mut i_s_method, .. })) => {
            txt = fun_84(txt.clone(), (i_s_method.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
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

fn fun_87(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_makefileParams_omhome: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_makefileParams_omhome.clone()) {
        (mut txt, false, mut a_makefileParams_omhome) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/I\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/include/omc/c/fmi1\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_makefileParams_omhome) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/I\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/include/omc/c/fmi2\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/DFMU_EXPERIMENTAL")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_89(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_common: Tpl::Text, mut in_a_FMUVersion: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_common.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { fmuTargetName: ref i_fmuTargetName, fileNamePrefix: ref i_fileNamePrefix, simulationSettingsOpt: ref i_sopt, makefileParams: SimCodeFunction::MakefileParams { omhome: ref i_makefileParams_omhome, platform: ref i_makefileParams_platform, libs: ref i_makefileParams_libs, .. }, modelInfo: SimCode::ModelInfo { directory: ref i_modelInfo_directory, .. }, .. }, mut a_common, mut a_FMUVersion) => {
            let mut ret_14: ArcStr = arcstr::literal!("");
            let mut ret_13: ArcStr = arcstr::literal!("");
            let mut ret_12: ArcStr = arcstr::literal!("");
            let mut ret_11: ArcStr = arcstr::literal!("");
            let mut ret_10: bool = false;
            let mut ret_9: bool = false;
            let mut l_mkdir: Tpl::Text;
            let mut txt_7: Tpl::Text;
            let mut l_compilecmds: Tpl::Text;
            let mut l_extraCflags: Tpl::Text;
            let mut l_fmudirname: Tpl::Text;
            let mut l_libsPos2: Tpl::Text;
            let mut l_libsPos1: Tpl::Text;
            let mut l_libsStr: Tpl::Text;
            let mut l_dirExtra: Tpl::Text;
            l_dirExtra = fun_80(Tpl::emptyTxt.clone(), (i_modelInfo_directory.clone()).clone())?;
            l_libsStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_libsStr = lm_81(l_libsStr.clone(), i_makefileParams_libs.clone())?;
            l_libsStr = Tpl::popIter(l_libsStr.clone())?;
            l_libsPos1 = fun_82(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            l_libsPos2 = fun_83(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            l_fmudirname = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            l_fmudirname = Tpl::writeTok(l_fmudirname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp")).clone() }))?;
            l_extraCflags = fun_85(Tpl::emptyTxt.clone(), i_sopt.clone())?;
            txt_7 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            l_compilecmds = CodegenFMU::getPlatformString2(Tpl::emptyTxt.clone(), (Tpl::textString(txt_7.clone())?).clone(), (i_makefileParams_platform.clone()).clone(), (i_fileNamePrefix.clone()).clone(), (i_fmuTargetName.clone()).clone(), (Tpl::textString(l_dirExtra.clone())?).clone(), (Tpl::textString(l_libsPos1.clone())?).clone(), (Tpl::textString(l_libsPos2.clone())?).clone(), (i_makefileParams_omhome.clone()).clone(), (Tpl::textString(a_FMUVersion.clone())?).clone())?;
            l_mkdir = fun_86(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# Makefile generated by OpenModelica\n")).clone(), (literal!("\n")).clone(), (literal!("# Simulations use -O3 by default\n")).clone(), (literal!("SIM_OR_DYNLOAD_OPT_LEVEL=\n")).clone(), (literal!("MODELICAUSERCFLAGS=\n")).clone(), (literal!("CXX=cl\n")).clone(), (literal!("EXEEXT=.exe\n")).clone(), (literal!("DLLEXT=.dll\n")).clone(), (literal!("FMUEXT=.fmu\n")).clone(), (literal!("PLATWIN32 = win32\n")).clone(), (literal!("\n")).clone(), (literal!("# /Od - Optimization disabled\n")).clone(), (literal!("# /EHa enable C++ EH (w/ SEH exceptions)\n")).clone(), (literal!("# /fp:except - consider floating-point exceptions when generating code\n")).clone(), (literal!("# /arch:SSE2 - enable use of instructions available with SSE2 enabled CPUs\n")).clone(), (literal!("# /I - Include Directories\n")).clone(), (literal!("# /DNOMINMAX - Define NOMINMAX (does what it says)\n")).clone(), (literal!("# /TP - Use C++ Compiler\n")).clone(), (literal!("CFLAGS=/MP /Od /ZI /EHa /fp:except /I\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/include/omc/c\" /I\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/include/omc/msvc/\" ")).clone() }))?;
            ret_9 = FMI::isFMIVersion20((Tpl::textString(a_FMUVersion.clone())?).clone());
            txt = fun_87(txt.clone(), ret_9.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /I. /DNOMINMAX /TP /DNO_INTERACTIVE_DEPENDENCY  ")).clone() }))?;
            ret_10 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_88(txt.clone(), ret_10.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("# /ZI enable Edit and Continue debug info\n")).clone(), (literal!("CDFLAGS=/ZI\n")).clone(), (literal!("\n")).clone(), (literal!("# /MD - link with MSVCRT.LIB\n")).clone(), (literal!("# /link - [linker options and libraries]\n")).clone(), (literal!("# /LIBPATH: - Directories where libs can be found\n")).clone(), (literal!("LDFLAGS=/MD /link /dll /debug /pdb:\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".pdb\" /LIBPATH:\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/lib/omc/msvc/\" /LIBPATH:\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/lib/omc/msvc/release/\" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_dirExtra.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_libsPos1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_libsPos2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" f2c.lib initialization.lib libexpat.lib math-support.lib meta.lib results.lib simulation.lib solver.lib sundials_kinsol.lib sundials_nvecserial.lib util.lib lapack_win32_MT.lib lis.lib  omcgc.lib user32.lib pthreadVC2.lib wsock32.lib cminpack.lib umfpack.lib amd.lib\n")).clone(), (literal!("\n")).clone(), (literal!("# /MDd link with MSVCRTD.LIB debug lib\n")).clone(), (literal!("# lib names should not be appended with a d just switch to lib/omc/msvc/debug\n")).clone(), (literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_common.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(FMUEXT): ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("$(DLLEXT) modelDescription.xml\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if not exist ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\binaries\\$(PLATWIN32) ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("if not exist ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mkdir.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\sources\n")).clone(), (literal!("\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".dll ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".lib ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".pdb ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".c\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_model.h ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_model.h\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.c ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.c\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_info.c ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_info.c\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_fmu.c ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_init_fmu.c\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_functions.c ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_functions.c\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_functions.h ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_functions.h\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_records.c ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\sources\\")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_records.c\n")).clone(), (literal!("copy modelDescription.xml ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\modelDescription.xml\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_11 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_11.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\SUNDIALS_KINSOL.DLL ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_12 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_12.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\SUNDIALS_NVECSERIAL.DLL ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_13 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_13.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\LAPACK_WIN32_MT.DLL ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_14 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_14.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\pthreadVC2.dll ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("cd ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"zip.exe\" -r ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu *\n")).clone(), (literal!("cd ..\n")).clone(), (literal!("rm -rf ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("$(DLLEXT): $(MAINOBJ) $(CFILES)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(CXX) /Fe")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) $(MAINFILE) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.c $(CFILES) $(CFLAGS) $(LDFLAGS)")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_90(mut in_txt: Tpl::Text, mut in_a_modelInfo_directory: ArcStr) -> Result<Tpl::Text> {
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
fn lm_91(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lib, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lib.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_91(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_92(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dirExtra.clone(), in_a_libsStr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_libsStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_libsStr.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_93(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dirExtra.clone(), in_a_libsStr.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_libsStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_libsStr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_94(mut in_txt: Tpl::Text, mut in_a_s_method: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_s_method.clone())) {
        (txt, Deref @ "inline-euler") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-D_OMC_INLINE_EULER")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "inline-rungekutta") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-D_OMC_INLINE_RK")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_95(mut in_txt: Tpl::Text, mut in_a_sopt: Option<SimCode::SimulationSettings>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_sopt.clone()) {
        (mut txt, Some(SimCode::SimulationSettings { method: mut i_s_method, .. })) => {
            txt = fun_94(txt.clone(), (i_s_method.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_96(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("2")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_97(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_97(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_98(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-DFMU_EXPERIMENTAL")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_99(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_common: Tpl::Text, mut in_a_FMUVersion: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_common.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { fmuTargetName: ref i_fmuTargetName, fileNamePrefix: ref i_fileNamePrefix, simulationSettingsOpt: ref i_sopt, makefileParams: SimCodeFunction::MakefileParams { includes: ref i_makefileParams_includes, omhome: ref i_makefileParams_omhome, platform: ref i_makefileParams_platform, libs: ref i_makefileParams_libs, .. }, delayedExps: SimCode::DelayedExpression { maxDelayedIndex: ref i_maxDelayedIndex, .. }, modelInfo: SimCode::ModelInfo { directory: ref i_modelInfo_directory, varInfo: SimCode::VarInfo { numStringAlgVars: ref i_varInfo_numStringAlgVars, numMixedSystems: ref i_varInfo_numMixedSystems, numNonLinearSystems: ref i_varInfo_numNonLinearSystems, numLinearSystems: ref i_varInfo_numLinearSystems, .. }, .. }, .. }, mut a_common, mut a_FMUVersion) => {
            let mut ret_9: bool = false;
            let mut ret_8: bool = false;
            let mut l_platformstr: Tpl::Text;
            let mut txt_6: Tpl::Text;
            let mut l_compilecmds: Tpl::Text;
            let mut l_extraCflags: Tpl::Text;
            let mut l_libsPos2: Tpl::Text;
            let mut l_libsPos1: Tpl::Text;
            let mut l_libsStr: Tpl::Text;
            let mut l_dirExtra: Tpl::Text;
            l_dirExtra = fun_90(Tpl::emptyTxt.clone(), (i_modelInfo_directory.clone()).clone())?;
            l_libsStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_libsStr = lm_91(l_libsStr.clone(), i_makefileParams_libs.clone())?;
            l_libsStr = Tpl::popIter(l_libsStr.clone())?;
            l_libsPos1 = fun_92(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            l_libsPos2 = fun_93(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            l_extraCflags = fun_95(Tpl::emptyTxt.clone(), i_sopt.clone())?;
            txt_6 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            l_compilecmds = CodegenFMU::getPlatformString2(Tpl::emptyTxt.clone(), (Tpl::textString(txt_6.clone())?).clone(), (i_makefileParams_platform.clone()).clone(), (i_fileNamePrefix.clone()).clone(), (i_fmuTargetName.clone()).clone(), (Tpl::textString(l_dirExtra.clone())?).clone(), (Tpl::textString(l_libsPos1.clone())?).clone(), (Tpl::textString(l_libsPos2.clone())?).clone(), (i_makefileParams_omhome.clone()).clone(), (Tpl::textString(a_FMUVersion.clone())?).clone())?;
            l_platformstr = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# Makefile generated by OpenModelica\n")).clone(), (literal!("CC=@CC@\n")).clone(), (literal!("AR=@AR@\n")).clone(), (literal!("CFLAGS=@CFLAGS@\n")).clone(), (literal!("LD=$(CC) -shared\n")).clone(), (literal!("LDFLAGS=@LDFLAGS@ @LIBS@\n")).clone(), (literal!("DLLEXT=@DLLEXT@\n")).clone(), (literal!("NEED_RUNTIME=@NEED_RUNTIME@\n")).clone(), (literal!("NEED_DGESV=@NEED_DGESV@\n")).clone(), (literal!("FMIPLATFORM=@FMIPLATFORM@\n")).clone(), (literal!("# Note: Simulation of the fmu with dymola does not work with -finline-small-functions (enabled by most optimization levels)\n")).clone(), (literal!("CPPFLAGS=@CPPFLAGS@\n")).clone(), (literal!("OMC_NUM_LINEAR_SYSTEMS=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numLinearSystems.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMC_NUM_NONLINEAR_SYSTEMS=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numNonLinearSystems.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMC_NUM_MIXED_SYSTEMS=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numMixedSystems.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMC_NDELAY_EXPRESSIONS=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_maxDelayedIndex.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMC_NVAR_STRING=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numStringAlgVars.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("override CPPFLAGS += -DFMI2_OVERRIDE_FUNCTION_PREFIX\n")).clone(), (literal!("override CPPFLAGS += -Iinclude/ -Iinclude/fmi")).clone()], lastHasNewLine: false }))?;
            ret_8 = FMI::isFMIVersion20((Tpl::textString(a_FMUVersion.clone())?).clone());
            txt = fun_96(txt.clone(), ret_8.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -I. ")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_97(txt.clone(), i_makefileParams_includes.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            ret_9 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_98(txt.clone(), ret_9.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  -DOMC_MODEL_PREFIX=")).clone() }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -DOMC_NUM_MIXED_SYSTEMS=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numMixedSystems.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -DOMC_NUM_LINEAR_SYSTEMS=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numLinearSystems.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -DOMC_NUM_NONLINEAR_SYSTEMS=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numNonLinearSystems.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -DOMC_NDELAY_EXPRESSIONS=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_maxDelayedIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -DOMC_NVAR_STRING=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numStringAlgVars.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_common.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("PHONY: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_FMU\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_compilecmds.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_100(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_target: ArcStr, mut in_a_common: Tpl::Text, mut in_a_FMUVersion: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_target.clone(), in_a_common.clone(), in_a_FMUVersion.clone(), in_a_simCode.clone())) {
        (txt, Deref @ "msvc", _, a_common, a_FMUVersion, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_89(txt.clone(), a_simCode.clone(), a_common.clone(), a_FMUVersion.clone())?;
            txt.clone()
        },
        (txt, Deref @ "gcc", _, a_common, a_FMUVersion, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_99(txt.clone(), a_simCode.clone(), a_common.clone(), a_FMUVersion.clone())?;
            txt.clone()
        },
        (txt, _, a_target, _, _, _) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("target ")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (a_target.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" is not handled!")).clone() }))?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenOMSIC.tpl")).clone(), 469, 25), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn createMakefileIn(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_target: ArcStr, mut a_FileNamePrefix: ArcStr, mut a_makeflieName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut str_8: ArcStr = arcstr::literal!("");
    let mut txt_7: Tpl::Text;
    let mut l_common: Tpl::Text;
    let mut l_0__: Tpl::Text;
    let mut l_SimDerMatFiles: Tpl::Text;
    let mut l_SimAlgSystemFiles: Tpl::Text;
    let mut l_InitDerMatFiles: Tpl::Text;
    let mut l_InitAlgSystemFiles: Tpl::Text;
    let mut l_FMUVersion: Tpl::Text;
    l_FMUVersion = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("2.0")).clone() }))?;
    l_InitAlgSystemFiles = Tpl::emptyTxt.clone();
    l_InitDerMatFiles = Tpl::emptyTxt.clone();
    l_SimAlgSystemFiles = Tpl::emptyTxt.clone();
    l_SimDerMatFiles = Tpl::emptyTxt.clone();
    (l_0__, l_SimDerMatFiles, l_SimAlgSystemFiles, l_InitDerMatFiles, l_InitAlgSystemFiles) = fun_78(Tpl::emptyTxt.clone(), a_simCode.clone(), l_SimDerMatFiles.clone(), l_SimAlgSystemFiles.clone(), l_InitDerMatFiles.clone(), (a_FileNamePrefix.clone()).clone(), l_InitAlgSystemFiles.clone())?;
    l_common = fun_79(Tpl::emptyTxt.clone(), a_simCode.clone(), l_SimDerMatFiles.clone(), l_SimAlgSystemFiles.clone(), l_InitDerMatFiles.clone(), l_InitAlgSystemFiles.clone())?;
    txt_7 = CodegenUtil::getGeneralTarget(Tpl::emptyTxt.clone(), (a_target.clone()).clone())?;
    str_8 = (Tpl::textString(txt_7.clone())?).clone();
    out_txt = fun_100(txt.clone(), (str_8.clone()).clone(), (a_target.clone()).clone(), l_common.clone(), l_FMUVersion.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

