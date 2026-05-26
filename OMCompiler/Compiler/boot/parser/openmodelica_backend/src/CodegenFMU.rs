// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenC;
use crate::CodegenCFunctions;
use crate::CodegenFMU1;
use crate::CodegenFMU2;
use crate::CodegenFMUCommon;
use crate::CodegenUtil;
use crate::CodegenUtilSimulation;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Expression;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_susan::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::FMI;
use openmodelica_util::Flags;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn fun_55(mut in_txt: Tpl::Text, mut in_a_sc_fmiSimulationFlags: Option<SimCode::FmiSimulationFlags>, mut in_a_fileNamePrefix: ArcStr, mut in_a_fileNamePrefixHash: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_sc_fmiSimulationFlags.clone(), in_a_fileNamePrefix.clone(), in_a_fileNamePrefixHash.clone()) {
        (mut txt, Some(mut i_fmiSimFlags @ SimCode::FmiSimulationFlags::FMI_SIMULATION_FLAGS { nameValueTuples: _ }), mut a_fileNamePrefix, mut a_fileNamePrefixHash) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = fmuSimulationFlagsFile(Tpl::emptyTxt.clone(), i_fmiSimFlags.clone())?;
            txt_1 = Tpl::writeText(Tpl::emptyTxt.clone(), a_fileNamePrefixHash.clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/resources/")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_flags.json")).clone() }))?;
            Tpl::textFile(txt_0.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn translateModel(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUVersion: ArcStr, mut in_a_FMUType: ArcStr, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_FMUVersion.clone(), in_a_FMUType.clone(), in_a_sourceFiles.clone())) {
        (txt, i_sc @ SimCode::SimCode { fmiSimulationFlags: i_sc_fmiSimulationFlags, externalFunctionIncludes: i_sc_externalFunctionIncludes, generic_loop_calls: i_generic__loop__calls @ i_sc_generic__loop__calls, recordDecls: i_recordDecls, literals: i_literals, fileNamePrefix: i_fileNamePrefix, modelInfo: SimCode::ModelInfo { functions: i_modelInfo_functions, .. }, .. }, a_FMUVersion, a_FMUType, a_sourceFiles) => {
            let mut txt_34: Tpl::Text;
            let mut txt_33: Tpl::Text;
            let mut txt_32: Tpl::Text;
            let mut txt_31: Tpl::Text;
            let mut txt_30: Tpl::Text;
            let mut txt_29: Tpl::Text;
            let mut l_0___1: Tpl::Text;
            let mut txt_27: Tpl::Text;
            let mut txt_26: Tpl::Text;
            let mut txt_25: Tpl::Text;
            let mut txt_24: Tpl::Text;
            let mut txt_23: Tpl::Text;
            let mut txt_22: Tpl::Text;
            let mut txt_21: Tpl::Text;
            let mut txt_20: Tpl::Text;
            let mut l_0__: Tpl::Text;
            let mut txt_18: Tpl::Text;
            let mut txt_17: Tpl::Text;
            let mut txt_16: Tpl::Text;
            let mut txt_15: Tpl::Text;
            let mut txt_14: Tpl::Text;
            let mut txt_13: Tpl::Text;
            let mut txt_12: Tpl::Text;
            let mut txt_11: Tpl::Text;
            let mut txt_10: Tpl::Text;
            let mut txt_9: Tpl::Text;
            let mut txt_8: Tpl::Text;
            let mut txt_7: Tpl::Text;
            let mut l_fileNamePrefixTmpDir: Tpl::Text;
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut l_fileNamePrefixHash: Tpl::Text;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_target: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_guid: Tpl::Text;
            ret_1 = (System::getUUIDStr()).clone();
            l_guid = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            ret_3 = (Config::simulationCodeTarget()?).clone();
            l_target = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            ret_5 = (Util::hashFileNamePrefix((i_fileNamePrefix.clone()).clone())).clone();
            l_fileNamePrefixHash = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_5.clone()).clone())?;
            l_fileNamePrefixTmpDir = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixHash.clone())?;
            l_fileNamePrefixTmpDir = Tpl::writeTok(l_fileNamePrefixTmpDir.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources/")).clone() }))?;
            l_fileNamePrefixTmpDir = Tpl::writeStr(l_fileNamePrefixTmpDir.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_7 = CodegenC::simulationLiteralsFile(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone(), i_literals.clone())?;
            txt_8 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_8 = Tpl::writeTok(txt_8.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_literals.h")).clone() }))?;
            Tpl::textFile(txt_7.clone(), (Tpl::textString(txt_8.clone())?).clone())?;
            txt_9 = CodegenC::simulationFunctionsHeaderFile(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone(), i_modelInfo_functions.clone(), i_recordDecls.clone(), i_sc_generic__loop__calls.clone())?;
            txt_10 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_10 = Tpl::writeTok(txt_10.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_functions.h")).clone() }))?;
            Tpl::textFile(txt_9.clone(), (Tpl::textString(txt_10.clone())?).clone())?;
            txt_11 = CodegenC::simulationFunctionsFile(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone(), i_modelInfo_functions.clone(), i_generic__loop__calls.clone())?;
            txt_12 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_12 = Tpl::writeTok(txt_12.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_functions.c")).clone() }))?;
            Tpl::textFile(txt_11.clone(), (Tpl::textString(txt_12.clone())?).clone())?;
            txt_13 = CodegenCFunctions::externalFunctionIncludes(Tpl::emptyTxt.clone(), i_sc_externalFunctionIncludes.clone())?;
            txt_14 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_14 = Tpl::writeTok(txt_14.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_includes.h")).clone() }))?;
            Tpl::textFile(txt_13.clone(), (Tpl::textString(txt_14.clone())?).clone())?;
            txt_15 = CodegenCFunctions::recordsFile(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone(), i_recordDecls.clone(), true)?;
            txt_16 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_16 = Tpl::writeTok(txt_16.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_records.c")).clone() }))?;
            Tpl::textFile(txt_15.clone(), (Tpl::textString(txt_16.clone())?).clone())?;
            txt_17 = CodegenC::simulationHeaderFile(Tpl::emptyTxt.clone(), i_sc.clone())?;
            txt_18 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_18 = Tpl::writeTok(txt_18.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_model.h")).clone() }))?;
            Tpl::textFile(txt_17.clone(), (Tpl::textString(txt_18.clone())?).clone())?;
            l_0__ = generateSimulationFiles(Tpl::emptyTxt.clone(), i_sc.clone(), (Tpl::textString(l_guid.clone())?).clone(), (Tpl::textString(l_fileNamePrefixTmpDir.clone())?).clone(), (a_FMUVersion.clone()).clone())?;
            txt_20 = simulationInitFunction(Tpl::emptyTxt.clone(), i_sc.clone(), (Tpl::textString(l_guid.clone())?).clone())?;
            txt_21 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_21 = Tpl::writeTok(txt_21.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init_fmu.c")).clone() }))?;
            Tpl::textFile(txt_20.clone(), (Tpl::textString(txt_21.clone())?).clone())?;
            txt_22 = fmumodel_identifierHeaderFile(Tpl::emptyTxt.clone(), i_sc.clone(), (Tpl::textString(l_guid.clone())?).clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone())?;
            txt_23 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_23 = Tpl::writeTok(txt_23.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.h")).clone() }))?;
            Tpl::textFile(txt_22.clone(), (Tpl::textString(txt_23.clone())?).clone())?;
            txt_24 = fmumodel_identifierFile(Tpl::emptyTxt.clone(), i_sc.clone(), (Tpl::textString(l_guid.clone())?).clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone())?;
            txt_25 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixTmpDir.clone())?;
            txt_25 = Tpl::writeTok(txt_25.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.c")).clone() }))?;
            Tpl::textFile(txt_24.clone(), (Tpl::textString(txt_25.clone())?).clone())?;
            txt_26 = fmuModelDescriptionFile(Tpl::emptyTxt.clone(), i_sc.clone(), (Tpl::textString(l_guid.clone())?).clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone(), a_sourceFiles.clone())?;
            txt_27 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixHash.clone())?;
            txt_27 = Tpl::writeTok(txt_27.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/modelDescription.xml")).clone() }))?;
            Tpl::textFile(txt_26.clone(), (Tpl::textString(txt_27.clone())?).clone())?;
            l_0___1 = fun_55(Tpl::emptyTxt.clone(), i_sc_fmiSimulationFlags.clone(), (i_fileNamePrefix.clone()).clone(), l_fileNamePrefixHash.clone())?;
            txt_29 = fmudeffile(Tpl::emptyTxt.clone(), i_sc.clone(), (a_FMUVersion.clone()).clone())?;
            txt_30 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixHash.clone())?;
            txt_30 = Tpl::writeTok(txt_30.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources/")).clone() }))?;
            txt_30 = Tpl::writeStr(txt_30.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_30 = Tpl::writeTok(txt_30.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".def")).clone() }))?;
            Tpl::textFile(txt_29.clone(), (Tpl::textString(txt_30.clone())?).clone())?;
            txt_31 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("# Dummy file so OMDEV Compile.bat works")).clone() }))?;
            txt_31 = Tpl::writeTok(txt_31.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_31 = Tpl::writeTok(txt_31.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("include Makefile")).clone() }))?;
            txt_31 = Tpl::writeTok(txt_31.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_32 = Tpl::writeText(Tpl::emptyTxt.clone(), l_fileNamePrefixHash.clone())?;
            txt_32 = Tpl::writeTok(txt_32.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources/")).clone() }))?;
            txt_32 = Tpl::writeStr(txt_32.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_32 = Tpl::writeTok(txt_32.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".makefile")).clone() }))?;
            Tpl::textFile(txt_31.clone(), (Tpl::textString(txt_32.clone())?).clone())?;
            txt_33 = fmuSourceMakefile(Tpl::emptyTxt.clone(), i_sc.clone(), (a_FMUVersion.clone()).clone(), (Tpl::textString(l_fileNamePrefixHash.clone())?).clone())?;
            txt_34 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_34 = Tpl::writeTok(txt_34.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.makefile")).clone() }))?;
            Tpl::textFile(txt_33.clone(), (Tpl::textString(txt_34.clone())?).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateSimulationFiles(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr, mut in_a_modelNamePrefix: ArcStr, mut in_a_fmuVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone(), in_a_modelNamePrefix.clone(), in_a_fmuVersion.clone()) {
        (mut txt, mut i_simCode @ SimCode::SimCode { modelInfo: _, .. }, mut a_guid, mut a_modelNamePrefix, mut a_fmuVersion) => {
            let mut txt_43: Tpl::Text;
            let mut txt_42: Tpl::Text;
            let mut txt_41: Tpl::Text;
            let mut txt_40: Tpl::Text;
            let mut txt_39: Tpl::Text;
            let mut txt_38: Tpl::Text;
            let mut txt_37: Tpl::Text;
            let mut txt_36: Tpl::Text;
            let mut txt_35: Tpl::Text;
            let mut txt_34: Tpl::Text;
            let mut txt_33: Tpl::Text;
            let mut txt_32: Tpl::Text;
            let mut txt_31: Tpl::Text;
            let mut txt_30: Tpl::Text;
            let mut txt_29: Tpl::Text;
            let mut txt_28: Tpl::Text;
            let mut txt_27: Tpl::Text;
            let mut txt_26: Tpl::Text;
            let mut txt_25: Tpl::Text;
            let mut txt_24: Tpl::Text;
            let mut txt_23: Tpl::Text;
            let mut txt_22: Tpl::Text;
            let mut txt_21: Tpl::Text;
            let mut l_mixheader: Tpl::Text;
            let mut txt_19: Tpl::Text;
            let mut txt_18: Tpl::Text;
            let mut txt_17: Tpl::Text;
            let mut txt_16: Tpl::Text;
            let mut txt_15: Tpl::Text;
            let mut txt_14: Tpl::Text;
            let mut txt_13: Tpl::Text;
            let mut txt_12: Tpl::Text;
            let mut txt_11: Tpl::Text;
            let mut txt_10: Tpl::Text;
            let mut txt_9: Tpl::Text;
            let mut txt_8: Tpl::Text;
            let mut txt_7: Tpl::Text;
            let mut txt_6: Tpl::Text;
            let mut txt_5: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_0 = CodegenC::simulationFile_exo(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_1 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_01exo.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_0.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_2 = CodegenC::simulationFile_nls(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_3 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_02nls.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_2.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_4 = CodegenC::simulationFile_lsy(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_5 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_5 = Tpl::writeTok(txt_5.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_03lsy.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_4.clone(), (Tpl::textString(txt_5.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_6 = CodegenC::simulationFile_set(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_7 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_7 = Tpl::writeTok(txt_7.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_04set.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_6.clone(), (Tpl::textString(txt_7.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_8 = CodegenC::simulationFile_evt(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_9 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_9 = Tpl::writeTok(txt_9.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_05evt.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_8.clone(), (Tpl::textString(txt_9.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_10 = CodegenC::simulationFile_inz(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_11 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_11 = Tpl::writeTok(txt_11.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_06inz.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_10.clone(), (Tpl::textString(txt_11.clone())?).clone())?;
            txt_12 = CodegenC::simulationFile_dly(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_13 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_13 = Tpl::writeTok(txt_13.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_07dly.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_12.clone(), (Tpl::textString(txt_13.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_14 = CodegenC::simulationFile_bnd(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_15 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_15 = Tpl::writeTok(txt_15.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_08bnd.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_14.clone(), (Tpl::textString(txt_15.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_16 = CodegenC::simulationFile_alg(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_17 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_17 = Tpl::writeTok(txt_17.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_09alg.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_16.clone(), (Tpl::textString(txt_17.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_18 = CodegenC::simulationFile_asr(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_19 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_19 = Tpl::writeTok(txt_19.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_10asr.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_18.clone(), (Tpl::textString(txt_19.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            l_mixheader = Tpl::emptyTxt.clone();
            (txt_21, l_mixheader) = CodegenC::simulationFile_mix(Tpl::emptyTxt.clone(), i_simCode.clone(), l_mixheader.clone())?;
            txt_22 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_22 = Tpl::writeTok(txt_22.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_11mix.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_21.clone(), (Tpl::textString(txt_22.clone())?).clone())?;
            txt_23 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_23 = Tpl::writeTok(txt_23.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_11mix.h")).clone() }))?;
            Tpl::textFile(l_mixheader.clone(), (Tpl::textString(txt_23.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_24 = CodegenC::simulationFile_jac(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_25 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_25 = Tpl::writeTok(txt_25.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_12jac.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_24.clone(), (Tpl::textString(txt_25.clone())?).clone())?;
            txt_26 = CodegenC::simulationFile_jac_header(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_27 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_27 = Tpl::writeTok(txt_27.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_12jac.h")).clone() }))?;
            Tpl::textFile(txt_26.clone(), (Tpl::textString(txt_27.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_28 = CodegenC::simulationFile_opt(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_29 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_29 = Tpl::writeTok(txt_29.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_13opt.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_28.clone(), (Tpl::textString(txt_29.clone())?).clone())?;
            txt_30 = CodegenC::simulationFile_opt_header(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_31 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_31 = Tpl::writeTok(txt_31.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_13opt.h")).clone() }))?;
            Tpl::textFile(txt_30.clone(), (Tpl::textString(txt_31.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_32 = CodegenC::simulationFile_lnz(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_33 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_33 = Tpl::writeTok(txt_33.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_14lnz.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_32.clone(), (Tpl::textString(txt_33.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_34 = CodegenC::simulationFile_syn(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_35 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_35 = Tpl::writeTok(txt_35.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_15syn.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_34.clone(), (Tpl::textString(txt_35.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_36 = CodegenC::simulationFile_dae(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_37 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_37 = Tpl::writeTok(txt_37.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_16dae.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_36.clone(), (Tpl::textString(txt_37.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_38 = CodegenC::simulationFile_inl(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_39 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_39 = Tpl::writeTok(txt_39.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_17inl.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_38.clone(), (Tpl::textString(txt_39.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_40 = CodegenC::simulationFile_spd(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_41 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_41 = Tpl::writeTok(txt_41.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_18spd.c")).clone() }))?;
            Tpl::textFileConvertLines(txt_40.clone(), (Tpl::textString(txt_41.clone())?).clone())?;
            System::tmpTickResetIndex(0, 0);
            System::tmpTickResetIndex(0, 1);
            txt_42 = CodegenC::simulationFile(Tpl::emptyTxt.clone(), i_simCode.clone(), (a_guid.clone()).clone(), (a_fmuVersion.clone()).clone())?;
            txt_43 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt_43 = Tpl::writeTok(txt_43.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".c")).clone() }))?;
            Tpl::textFileConvertLines(txt_42.clone(), (Tpl::textString(txt_43.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_58(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_FMUType: ArcStr, mut in_a_guid: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_sourceFiles.clone(), in_a_FMUType.clone(), in_a_guid.clone(), in_a_simCode.clone())) {
        (txt, false, _, a_FMUType, a_guid, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = CodegenFMU1::fmiModelDescription(txt.clone(), a_simCode.clone(), (a_guid.clone()).clone(), (a_FMUType.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_sourceFiles, a_FMUType, a_guid, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = CodegenFMU2::fmiModelDescription(txt.clone(), a_simCode.clone(), (a_guid.clone()).clone(), (a_FMUType.clone()).clone(), a_sourceFiles.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn fmuModelDescriptionFile(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr, mut in_a_FMUVersion: ArcStr, mut in_a_FMUType: ArcStr, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone(), in_a_FMUVersion.clone(), in_a_FMUType.clone(), in_a_sourceFiles.clone())) {
        (txt, i_simCode @ SimCode::SimCode { modelInfo: _, .. }, a_guid, a_FMUVersion, a_FMUType, a_sourceFiles) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")).clone() }))?;
            ret_0 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_58(txt.clone(), ret_0.clone(), a_sourceFiles.clone(), (a_FMUType.clone()).clone(), (a_guid.clone()).clone(), i_simCode.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_60(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_name, i_value), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" : \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_value.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_60(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn fmuSimulationFlagsFile(mut in_txt: Tpl::Text, mut in_a_fmiSimulationFlags: SimCode::FmiSimulationFlags) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiSimulationFlags.clone()) {
        (mut txt, SimCode::FmiSimulationFlags::FMI_SIMULATION_FLAGS { nameValueTuples: ref i_flags_nameValueTuples }) => {
            let mut l_fileContent: Tpl::Text;
            l_fileContent = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(",\n")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_fileContent = lm_60(l_fileContent.clone(), i_flags_nameValueTuples.clone())?;
            l_fileContent = Tpl::popIter(l_fileContent.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("{\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_fileContent.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn VendorAnnotations(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<VendorAnnotations>\n")).clone(), (literal!("</VendorAnnotations>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_63(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_simCode.clone()) {
        (mut txt, false, mut a_simCode) => {
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define fmu1_model_interface_setupDataStruc ")).clone() }))?;
            txt_0 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), a_simCode.clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("setupDataStruc")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"fmi-export/fmu1_model_interface.c.inc\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_simCode) => {
            let mut txt_1: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define fmu2_model_interface_setupDataStruc ")).clone() }))?;
            txt_1 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), a_simCode.clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("setupDataStruc")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#include \"fmi-export/fmu2_model_interface.h\"\n")).clone(), (literal!("#include \"fmi-export/fmu_read_flags.h\"")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_64(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_simCode.clone()) {
        (mut txt, false, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#include \"fmi2Functions.h\"\n")).clone(), (literal!("#include \"fmi-export/fmu1_model_interface.h\"")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define FMI2_FUNCTION_PREFIX ")).clone() }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), a_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_\n")).clone(), (literal!("#include \"fmi2Functions.h\"\n")).clone(), (literal!("#include \"fmi-export/fmu2_model_interface.h\"\n")).clone(), (literal!("#include \"fmi-export/fmu_read_flags.h\"")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_65(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void eventUpdate(ModelInstance* comp, fmiEventInfo* eventInfo);\n")).clone(), (literal!("fmiReal getReal(ModelInstance* comp, const fmiValueReference vr);\n")).clone(), (literal!("fmiStatus setReal(ModelInstance* comp, const fmiValueReference vr, const fmiReal value);\n")).clone(), (literal!("fmiInteger getInteger(ModelInstance* comp, const fmiValueReference vr);\n")).clone(), (literal!("fmiStatus setInteger(ModelInstance* comp, const fmiValueReference vr, const fmiInteger value);\n")).clone(), (literal!("fmiBoolean getBoolean(ModelInstance* comp, const fmiValueReference vr);\n")).clone(), (literal!("fmiStatus setBoolean(ModelInstance* comp, const fmiValueReference vr, const fmiBoolean value);\n")).clone(), (literal!("fmiString getString(ModelInstance* comp, const fmiValueReference vr);\n")).clone(), (literal!("fmiStatus setString(ModelInstance* comp, const fmiValueReference vr, fmiString value);\n")).clone(), (literal!("fmiStatus setExternalFunction(ModelInstance* c, const fmiValueReference vr, const void* value);")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void eventUpdate(ModelInstance* comp, fmi2EventInfo* eventInfo);\n")).clone(), (literal!("fmi2Real getReal(ModelInstance* comp, const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2Status setReal(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Real value);\n")).clone(), (literal!("fmi2Integer getInteger(ModelInstance* comp, const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2Status setInteger(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Integer value);\n")).clone(), (literal!("fmi2Boolean getBoolean(ModelInstance* comp, const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2Status setBoolean(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Boolean value);\n")).clone(), (literal!("fmi2String getString(ModelInstance* comp, const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2Status setString(ModelInstance* comp, const fmi2ValueReference vr, fmi2String value);\n")).clone(), (literal!("fmi2Status setExternalFunction(ModelInstance* c, const fmi2ValueReference vr, const void* value);\n")).clone(), (literal!("fmi2ValueReference mapInputReference2InputNumber(const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2ValueReference mapOutputReference2OutputNumber(const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2ValueReference mapOutputReference2RealOutputDerivatives(const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2ValueReference mapInitialUnknownsdependentIndex(const fmi2ValueReference vr);\n")).clone(), (literal!("fmi2ValueReference mapInitialUnknownsIndependentIndex(const fmi2ValueReference vr);")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_66(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_guid: ArcStr, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_guid.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: ref i_modelInfo, .. }, mut a_guid, mut a_FMUVersion) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#ifndef ")).clone() }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU_H\n")).clone(), (literal!("#define ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU_H\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"simulation_data.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("// define class name and unique id\n")).clone(), (literal!("#define MODEL_IDENTIFIER ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define MODEL_GUID \"{")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_guid.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\"\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = ModelDefineData(txt.clone(), i_simCode.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#ifdef __cplusplus\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("extern void ")).clone()], lastHasNewLine: false }))?;
            txt_0 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("setupDataStruc")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("(DATA *data, threadData_t *threadData);\n")).clone() }))?;
            ret_1 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_63(txt.clone(), ret_1.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            ret_2 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_64(txt.clone(), ret_2.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("void setStartValues(ModelInstance *comp);\n")).clone(), (literal!("void setDefaultStartValues(ModelInstance *comp);\n")).clone()], lastHasNewLine: true }))?;
            ret_3 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_65(txt.clone(), ret_3.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("#ifdef __cplusplus\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone(), (literal!("#endif /* ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU_H */")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmumodel_identifierHeaderFile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_guid: ArcStr, mut a_FMUVersion: ArcStr, mut a_FMUType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_66(txt.clone(), a_simCode.clone(), (a_guid.clone()).clone(), (a_FMUVersion.clone()).clone())?;
    Ok(out_txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_FMUType: ArcStr, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_FMUType.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, false, _, mut a_modelInfo, mut a_simCode) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = eventUpdateFunction(txt.clone(), a_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getRealFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setRealFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getIntegerFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setIntegerFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getBooleanFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setBooleanFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getStringFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setStringFunction(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setExternalFunction(txt.clone(), a_modelInfo.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_FMUType, mut a_modelInfo, mut a_simCode) => {
            txt = eventUpdateFunction2(txt.clone(), a_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getRealFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setRealFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getIntegerFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setIntegerFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getBooleanFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setBooleanFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = getStringFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setStringFunction2(txt.clone(), a_simCode.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setExternalFunction2(txt.clone(), a_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = mapInputAndOutputs(txt.clone(), a_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = mapRealOutputDerivatives(txt.clone(), a_simCode.clone(), (a_FMUType.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = mapInitialUnknownsdependentCrefs(txt.clone(), a_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = mapInitialUnknownsIndependentCrefs(txt.clone(), a_simCode.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_69(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUVersion: ArcStr, mut in_a_FMUType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_FMUVersion.clone(), in_a_FMUType.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: ref i_modelInfo, fileNamePrefix: ref i_fileNamePrefix, .. }, mut a_FMUVersion, mut a_FMUType) => {
            let mut ret_0: bool = false;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("// include fmu header files, typedefs and macros\n")).clone(), (literal!("#include <stdio.h>\n")).clone(), (literal!("#include <string.h>\n")).clone(), (literal!("#include <assert.h>\n")).clone(), (literal!("#include \"openmodelica.h\"\n")).clone(), (literal!("#include \"openmodelica_func.h\"\n")).clone(), (literal!("#include \"util/omc_error.h\"\n")).clone(), (literal!("#include \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_functions.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"simulation/solver/events.h\"\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = setDefaultStartValues(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = setStartValues(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("// implementation of the Model Exchange functions\n")).clone()], lastHasNewLine: true }))?;
            ret_0 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_68(txt.clone(), ret_0.clone(), (a_FMUType.clone()).clone(), i_modelInfo.clone(), i_simCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmumodel_identifierFile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_guid: ArcStr, mut a_FMUVersion: ArcStr, mut a_FMUType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_69(txt.clone(), a_simCode.clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone())?;
    Ok(out_txt)
}

fn fun_71(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_listStates: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_varInfo_numStateVars: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_listStates.clone(), in_a_varInfo_numStateVars.clone())) {
        (txt, false, _, a_varInfo_numStateVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(a_varInfo_numStateVars.clone())).clone())?;
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

fn fun_72(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_simCode.clone(), in_a_name.clone())) {
        (txt, false, a_simCode, a_name) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_73(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { name: i_name, .. }, tail: rest }, a_simCode) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_72(txt.clone(), ret_1.clone(), a_simCode.clone(), i_name.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_73(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = lm_73(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_simCode.clone(), in_a_name.clone())) {
        (txt, false, a_simCode, a_name) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_75(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { name: i_name, .. }, tail: rest }, a_simCode) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("der($dummy)")).clone());
            txt = fun_74(txt.clone(), ret_1.clone(), a_simCode.clone(), i_name.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_75(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = lm_75(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_76(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = defineExternalFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_76(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_77(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone())) {
        (txt, SimCode::ModelInfo { functions: i_functions, vars: SimCodeVar::SimVars { derivativeVars: i_vars_derivativeVars, stateVars: i_vars_stateVars @ i_listStates, .. }, varInfo: SimCode::VarInfo { numZeroCrossings: i_varInfo_numZeroCrossings, numRealInputVars: i_varInfo_numRealInputVars, numBoolAliasVars: i_varInfo_numBoolAliasVars, numBoolParams: i_varInfo_numBoolParams, numBoolAlgVars: i_varInfo_numBoolAlgVars, numStringAliasVars: i_varInfo_numStringAliasVars, numStringParamVars: i_varInfo_numStringParamVars, numStringAlgVars: i_varInfo_numStringAlgVars, numIntAliasVars: i_varInfo_numIntAliasVars, numIntParams: i_varInfo_numIntParams, numIntAlgVars: i_varInfo_numIntAlgVars, numAlgAliasVars: i_varInfo_numAlgAliasVars, numParams: i_varInfo_numParams, numAlgVars: i_varInfo_numAlgVars, numDiscreteReal: i_varInfo_numDiscreteReal, numStateVars: i_varInfo_numStateVars, .. }, .. }, a_simCode) => {
            let mut ret_17: i32 = 0;
            let mut ret_16: bool = false;
            let mut l_numberOfRealInputs: Tpl::Text;
            let mut ret_14: i32 = 0;
            let mut ret_13: i32 = 0;
            let mut l_numberOfBooleans: Tpl::Text;
            let mut ret_11: i32 = 0;
            let mut ret_10: i32 = 0;
            let mut l_numberOfStrings: Tpl::Text;
            let mut ret_8: i32 = 0;
            let mut ret_7: i32 = 0;
            let mut l_numberOfIntegers: Tpl::Text;
            let mut ret_5: i32 = 0;
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut l_numberOfReals: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = intMul(i_varInfo_numStateVars.clone(), 2);
            ret_2 = intAdd(i_varInfo_numParams.clone(), i_varInfo_numAlgAliasVars.clone());
            ret_3 = intAdd(i_varInfo_numAlgVars.clone(), ret_2.clone());
            ret_4 = intAdd(i_varInfo_numDiscreteReal.clone(), ret_3.clone());
            ret_5 = intAdd(ret_1.clone(), ret_4.clone());
            l_numberOfReals = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_5.clone())).clone())?;
            ret_7 = intAdd(i_varInfo_numIntParams.clone(), i_varInfo_numIntAliasVars.clone());
            ret_8 = intAdd(i_varInfo_numIntAlgVars.clone(), ret_7.clone());
            l_numberOfIntegers = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            ret_10 = intAdd(i_varInfo_numStringParamVars.clone(), i_varInfo_numStringAliasVars.clone());
            ret_11 = intAdd(i_varInfo_numStringAlgVars.clone(), ret_10.clone());
            l_numberOfStrings = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_11.clone())).clone())?;
            ret_13 = intAdd(i_varInfo_numBoolParams.clone(), i_varInfo_numBoolAliasVars.clone());
            ret_14 = intAdd(i_varInfo_numBoolAlgVars.clone(), ret_13.clone());
            l_numberOfBooleans = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_14.clone())).clone())?;
            l_numberOfRealInputs = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_varInfo_numRealInputVars.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// define model size\n")).clone(), (literal!("#define NUMBER_OF_STATES ")).clone()], lastHasNewLine: false }))?;
            ret_16 = intEq(i_varInfo_numStateVars.clone(), 1);
            txt = fun_71(txt.clone(), ret_16.clone(), i_listStates.clone(), i_varInfo_numStateVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_EVENT_INDICATORS ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numZeroCrossings.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_REALS ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfReals.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_REAL_INPUTS ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfRealInputs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_INTEGERS ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfIntegers.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_STRINGS ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfStrings.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_BOOLEANS ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_numberOfBooleans.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define NUMBER_OF_EXTERNALFUNCTIONS ")).clone() }))?;
            ret_17 = SimCodeUtil::countDynamicExternalFunctions(i_functions.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_17.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("// define initial state vector as vector of value references\n")).clone(), (literal!("#define STATES { ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_73(txt.clone(), i_vars_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" }\n")).clone(), (literal!("#define STATESDERIVATIVES { ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_75(txt.clone(), i_vars_derivativeVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" }\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            System::tmpTickReset(0);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_76(txt.clone(), i_functions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ModelDefineData(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_77(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

pub fn dervativeNameCStyle(mut in_txt: Tpl::Text, mut in_a_cr: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cr.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, ident: Deref @ "$DER", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der_")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn defineExternalFunction(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: i_language, extName: i_extName, dynamicLoad: true, .. }) => {
            let mut ret_1: i32 = 0;
            let mut l_fname: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fname = CodegenCFunctions::extFunctionName(Tpl::emptyTxt.clone(), (i_extName.clone()).clone(), (i_language.clone()).clone())?;
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
fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_81(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_82(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_82(txt.clone(), rest.clone())?;
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
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
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
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
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
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("integerVars")).clone())?;
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
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("booleanVars")).clone())?;
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
            txt = initValsDefault(txt.clone(), i_var.clone(), (literal!("stringVars")).clone())?;
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
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("realParameter")).clone())?;
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
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("integerParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_89(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_90(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_90(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
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
            txt = initParamsDefault(txt.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_91(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn setDefaultStartValues(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringParamVars: ref i_vars_stringParamVars, boolParamVars: ref i_vars_boolParamVars, intParamVars: ref i_vars_intParamVars, paramVars: ref i_vars_paramVars, stringAlgVars: ref i_vars_stringAlgVars, boolAlgVars: ref i_vars_boolAlgVars, intAlgVars: ref i_vars_intAlgVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, varInfo: SimCode::VarInfo { numAlgVars: _, numStateVars: _, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// Set values for all variables that define a start value\n")).clone(), (literal!("OMC_DISABLE_OPT\n")).clone(), (literal!("void setDefaultStartValues(ModelInstance *comp) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_81(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_82(txt.clone(), i_vars_derivativeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_83(txt.clone(), i_vars_algVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_84(txt.clone(), i_vars_discreteAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_85(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_86(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_87(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_88(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_89(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_90(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_91(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
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
fn lm_93(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_93(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_94(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_94(txt.clone(), rest.clone())?;
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
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
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
            txt = initVals(txt.clone(), i_var.clone(), (literal!("realVars")).clone())?;
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
            txt = initVals(txt.clone(), i_var.clone(), (literal!("integerVars")).clone())?;
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
            txt = initVals(txt.clone(), i_var.clone(), (literal!("booleanVars")).clone())?;
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
            txt = initVals(txt.clone(), i_var.clone(), (literal!("stringVars")).clone())?;
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
            txt = initParams(txt.clone(), i_var.clone(), (literal!("realParameter")).clone())?;
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
            txt = initParams(txt.clone(), i_var.clone(), (literal!("integerParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_101(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_102(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParams(txt.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_102(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_103(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = initParams(txt.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_103(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn setStartValues(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringParamVars: ref i_vars_stringParamVars, boolParamVars: ref i_vars_boolParamVars, intParamVars: ref i_vars_intParamVars, paramVars: ref i_vars_paramVars, stringAlgVars: ref i_vars_stringAlgVars, boolAlgVars: ref i_vars_boolAlgVars, intAlgVars: ref i_vars_intAlgVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, varInfo: SimCode::VarInfo { numAlgVars: _, numStateVars: _, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// Set values for all variables that define a start value\n")).clone(), (literal!("OMC_DISABLE_OPT\n")).clone(), (literal!("void setStartValues(ModelInstance *comp) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_93(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_94(txt.clone(), i_vars_derivativeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_95(txt.clone(), i_vars_algVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_96(txt.clone(), i_vars_discreteAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_97(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_98(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_99(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_100(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_101(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_102(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_103(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_105(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_sub: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_sub: Tpl::Text;
    (out_txt, out_a_sub) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_sub.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_sub) => {
            (txt.clone(), a_sub.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { cref: i_cref, .. }, tail: rest }, a_sub) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_sub = (*a_sub).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (sim_verbose) { printf(\"Setting variable start value: %s(start=%f)\\n\", \"")).clone() }))?;
            txt_0 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_cref.clone())?;
            ret_1 = (Util::escapeModelicaStringToCString((Tpl::textString(txt_0.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            (txt, a_sub) = CodegenCFunctions::cref(txt.clone(), i_cref.clone(), a_sub.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("); }")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_sub) = lm_105(txt.clone(), rest.clone(), a_sub.clone())?;
            (txt.clone(), a_sub.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_sub) => {
            let mut txt = (*txt).clone();
            let mut a_sub = (*a_sub).clone();
            (txt, a_sub) = lm_105(txt.clone(), rest.clone(), a_sub.clone())?;
            (txt.clone(), a_sub.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_sub))
}

pub fn initializeFunction(mut txt: Tpl::Text, mut a_allEquations: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_eqPart: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    let mut l_sub: Tpl::Text;
    l_sub = Tpl::emptyTxt.clone();
    l_varDecls = Tpl::emptyTxt.clone();
    l_eqPart = Tpl::emptyTxt.clone();
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// Used to set the first time event, if any.\n")).clone(), (literal!("void initialize(ModelInstance* comp, fmiEventInfo* eventInfo) {\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_varDecls.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_eqPart.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (out_txt, l_sub) = lm_105(out_txt.clone(), a_allEquations.clone(), l_sub.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok(out_txt)
}

fn fun_107(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_var_index: i32, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_var_index.clone(), in_a_arrayName.clone())) {
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_var_index, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("put_real_element(comp->fmuData->localData[0]->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_var_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("], 0, &comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_var_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start);")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_var_index, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_var_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start = comp->fmuData->localData[0]->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_var_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_108(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_var_index: i32, mut in_a_arrayName: ArcStr, mut in_a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_var_index.clone(), in_a_arrayName.clone(), in_a_type__.clone())) {
        (txt, false, a_var_index, a_arrayName, a_type__) => {
            let mut txt = (*txt).clone();
            txt = fun_107(txt.clone(), a_type__.clone(), a_var_index.clone(), (a_arrayName.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_109(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_var_index: i32, mut in_a_arrayName: ArcStr, mut in_a_type__: Arc<DAE::Type>, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_var_index.clone(), in_a_arrayName.clone(), in_a_type__.clone(), in_a_name.clone())) {
        (txt, false, a_var_index, a_arrayName, a_type__, a_name) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), a_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("der($dummy)")).clone());
            txt = fun_108(txt.clone(), ret_1.clone(), a_var_index.clone(), (a_arrayName.clone()).clone(), a_type__.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initVals(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone()) {
        (mut txt, SimCodeVar::SimVar { index: mut i_var_index, name: ref i_name, type_: ref i_type__, .. }, mut a_arrayName) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_109(txt.clone(), ret_1.clone(), i_var_index.clone(), (a_arrayName.clone()).clone(), i_type__.clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn initParams(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone())) {
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, index: i_index, .. }, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("put_real_element(comp->fmuData->simulationInfo->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("], 0, &comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start);")).clone() }))?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { index: i_index, .. }, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start = comp->fmuData->simulationInfo->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initValsDefault(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone())) {
        (txt, i_var @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, index: i_index, .. }, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("put_real_element(")).clone() }))?;
            txt = initValDefault(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 0, &comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start);")).clone() }))?;
            txt.clone()
        },
        (txt, i_var @ SimCodeVar::SimVar { index: i_index, .. }, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start = ")).clone() }))?;
            txt = initValDefault(txt.clone(), i_var.clone())?;
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
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_arrayName.clone())) {
        (txt, i_var @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, index: i_index, .. }, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("put_real_element(")).clone() }))?;
            txt = initValDefault(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 0, &comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start);")).clone() }))?;
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { initialValue: Some(i_v @ Deref @ DAE::Exp::SCONST { string: _ }), type_: Deref @ DAE::Type::T_STRING { varLst: _ }, index: i_index, .. }, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start = mmc_mk_scon_persist(")).clone() }))?;
            txt = initVal(txt.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("); /* TODO: these are not freed currently, see #6161 */")).clone() }))?;
            txt.clone()
        },
        (txt, i_var @ SimCodeVar::SimVar { index: i_index, .. }, a_arrayName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->modelData->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Data[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute.start = ")).clone() }))?;
            txt = initValDefault(txt.clone(), i_var.clone())?;
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

fn fun_114(mut in_txt: Tpl::Text, mut in_a_var_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var_type__.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_scon(\"\")")).clone() }))?;
            txt.clone()
        },
        (txt, i_var_type__) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Unknown type for initValDefault: ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_var_type__.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenFMU.tpl")).clone(), 618, 22), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_115(mut in_txt: Tpl::Text, mut in_a_var_initialValue: Option<Arc<DAE::Exp>>, mut in_a_var_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var_initialValue.clone(), in_a_var_type__.clone())) {
        (txt, Some(i_v @ Deref @ DAE::Exp::ICONST { integer: _ }), _) => {
            let mut txt = (*txt).clone();
            txt = initVal(txt.clone(), i_v.clone())?;
            txt.clone()
        },
        (txt, Some(i_v @ Deref @ DAE::Exp::RCONST { real: _ }), _) => {
            let mut txt = (*txt).clone();
            txt = initVal(txt.clone(), i_v.clone())?;
            txt.clone()
        },
        (txt, Some(i_v @ Deref @ DAE::Exp::SCONST { string: _ }), _) => {
            let mut txt = (*txt).clone();
            txt = initVal(txt.clone(), i_v.clone())?;
            txt.clone()
        },
        (txt, Some(i_v @ Deref @ DAE::Exp::BCONST { bool: _ }), _) => {
            let mut txt = (*txt).clone();
            txt = initVal(txt.clone(), i_v.clone())?;
            txt.clone()
        },
        (txt, Some(i_v @ Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. }), _) => {
            let mut txt = (*txt).clone();
            txt = initVal(txt.clone(), i_v.clone())?;
            txt.clone()
        },
        (txt, _, a_var_type__) => {
            let mut txt = (*txt).clone();
            txt = fun_114(txt.clone(), a_var_type__.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initValDefault(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, SimCodeVar::SimVar { type_: ref i_var_type__, initialValue: mut i_var_initialValue, .. }) => {
            txt = fun_115(txt.clone(), i_var_initialValue.clone(), i_var_type__.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_117(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
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
            txt = fun_117(txt.clone(), i_bool.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, i_initialValue) => {
            let mut txt_1: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("initial value of unknown type: ")).clone() }))?;
            ret_1 = (ExpressionBasics::printExpStr(i_initialValue.clone())?).clone();
            txt_1 = Tpl::writeStr(txt_1.clone(), (ret_1.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenFMU.tpl")).clone(), 629, 14), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn eventUpdateFunction(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// Used to set the next time event, if any.\n")).clone(), (literal!("void eventUpdate(ModelInstance* comp, fmiEventInfo* eventInfo) {\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
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
fn lm_120(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_120(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_121(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_121(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_122(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_122(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_123(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_123(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_124(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParameters(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_124(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_125(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Real")).clone(), (literal!("-")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_125(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_126(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numAlgVars: _, numStateVars: _, .. }, vars: SimCodeVar::SimVars { aliasVars: ref i_vars_aliasVars, paramVars: ref i_vars_paramVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiReal getReal(ModelInstance* comp, const fmiValueReference vr) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_120(txt.clone(), i_vars_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_121(txt.clone(), i_vars_derivativeVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_122(txt.clone(), i_vars_algVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_123(txt.clone(), i_vars_discreteAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_124(txt.clone(), i_vars_paramVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_125(txt.clone(), i_vars_aliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return 0;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getRealFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_126(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_128(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_128(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_129(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_129(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_130(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_130(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_131(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_131(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_132(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParametersSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("realParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_132(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_133(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Real")).clone(), (literal!("-")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_133(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_134(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numAlgVars: _, numStateVars: _, .. }, vars: SimCodeVar::SimVars { aliasVars: ref i_vars_aliasVars, paramVars: ref i_vars_paramVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiStatus setReal(ModelInstance* comp, const fmiValueReference vr, const fmiReal value) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_128(txt.clone(), i_vars_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_129(txt.clone(), i_vars_derivativeVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_130(txt.clone(), i_vars_algVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_131(txt.clone(), i_vars_discreteAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_132(txt.clone(), i_vars_paramVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_133(txt.clone(), i_vars_aliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmiError;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmiOK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setRealFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_134(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_136(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("integerVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_136(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_137(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParameters(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("integerParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_137(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_138(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Integer")).clone(), (literal!("-")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_138(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_139(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { intAliasVars: ref i_vars_intAliasVars, intParamVars: ref i_vars_intParamVars, intAlgVars: ref i_vars_intAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiInteger getInteger(ModelInstance* comp, const fmiValueReference vr) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_136(txt.clone(), i_vars_intAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_137(txt.clone(), i_vars_intParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_138(txt.clone(), i_vars_intAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return 0;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getIntegerFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_139(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_141(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("integerVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_141(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_142(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParametersSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("integerParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_142(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_143(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Integer")).clone(), (literal!("-")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_143(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_144(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { intAliasVars: ref i_vars_intAliasVars, intParamVars: ref i_vars_intParamVars, intAlgVars: ref i_vars_intAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiStatus setInteger(ModelInstance* comp, const fmiValueReference vr, const fmiInteger value) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_141(txt.clone(), i_vars_intAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_142(txt.clone(), i_vars_intParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_143(txt.clone(), i_vars_intAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmiError;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmiOK;\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setIntegerFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_144(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_146(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_146(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_147(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParameters(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_147(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_148(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Boolean")).clone(), (literal!("!")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_148(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_149(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { boolAliasVars: ref i_vars_boolAliasVars, boolParamVars: ref i_vars_boolParamVars, boolAlgVars: ref i_vars_boolAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiBoolean getBoolean(ModelInstance* comp, const fmiValueReference vr) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_146(txt.clone(), i_vars_boolAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_147(txt.clone(), i_vars_boolParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_148(txt.clone(), i_vars_boolAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmiFalse;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getBooleanFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_149(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_151(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_151(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_152(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParametersSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_152(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_153(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Boolean")).clone(), (literal!("!")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_153(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_154(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { boolAliasVars: ref i_vars_boolAliasVars, boolParamVars: ref i_vars_boolParamVars, boolAlgVars: ref i_vars_boolAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiStatus setBoolean(ModelInstance* comp, const fmiValueReference vr, const fmiBoolean value) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_151(txt.clone(), i_vars_boolAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_152(txt.clone(), i_vars_boolParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_153(txt.clone(), i_vars_boolAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmiError;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmiOK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setBooleanFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_154(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_156(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_156(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_157(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParameters(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_157(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_158(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("String")).clone(), (literal!("")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_158(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_159(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringAliasVars: ref i_vars_stringAliasVars, stringParamVars: ref i_vars_stringParamVars, stringAlgVars: ref i_vars_stringAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiString getString(ModelInstance* comp, const fmiValueReference vr) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_156(txt.clone(), i_vars_stringAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_157(txt.clone(), i_vars_stringParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_158(txt.clone(), i_vars_stringAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return \"\";\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getStringFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_159(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_161(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_161(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_162(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParametersSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_162(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_163(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("String")).clone(), (literal!("")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_163(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_164(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringAliasVars: ref i_vars_stringAliasVars, stringParamVars: ref i_vars_stringParamVars, stringAlgVars: ref i_vars_stringAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiStatus setString(ModelInstance* comp, const fmiValueReference vr, fmiString value) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_161(txt.clone(), i_vars_stringAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_162(txt.clone(), i_vars_stringParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_163(txt.clone(), i_vars_stringAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmiError;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmiOK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setStringFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_164(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

pub fn setExternalFunction(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { functions: ref i_functions, vars: SimCodeVar::SimVars { stateVars: _, .. }, .. }) => {
            let mut l_externalFuncs: Tpl::Text;
            l_externalFuncs = setExternalFunctionsSwitch(Tpl::emptyTxt.clone(), i_functions.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmiStatus setExternalFunction(ModelInstance* c, const fmiValueReference vr, const void* value){\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_externalFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmiError;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmiOK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn eventUpdateFunction2(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// Used to set the next time event, if any.\n")).clone(), (literal!("void eventUpdate(ModelInstance* comp, fmi2EventInfo* eventInfo) {\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
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
fn lm_168(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { aliasvar: i_aliasvar, .. }, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = aliasSetVR(txt.clone(), a_simCode.clone(), i_aliasvar.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_168(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = lm_168(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_169(mut in_txt: Tpl::Text, mut in_a_numAlgAliasVars: i32, mut in_a_simCode: SimCode::SimCode, mut in_a_vars_aliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_numAlgAliasVars.clone(), in_a_simCode.clone(), in_a_vars_aliasVars.clone())) {
        (txt, 0, _, _) => {
            txt.clone()
        },
        (txt, i_numAlgAliasVars, a_simCode, a_vars_aliasVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static const int realAliasIndexes[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_numAlgAliasVars.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("] = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 20, alignOfset: 0, alignSeparator: Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(",\n")).clone() }), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_168(txt.clone(), a_vars_aliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_170(mut in_txt: Tpl::Text, mut in_a_numAlgAliasVars: i32, mut in_a_ixFirstAlias: Tpl::Text, mut in_a_ixEnd: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_numAlgAliasVars.clone(), in_a_ixFirstAlias.clone(), in_a_ixEnd.clone()) {
        (mut txt, 0, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_ixFirstAlias, mut a_ixEnd) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixEnd.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int ix = realAliasIndexes[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("];\n")).clone(), (literal!("return ix>=0 ? getReal(comp, ix) : -getReal(comp, -(ix+1));\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_171(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numDiscreteReal: mut i_numDiscreteReal, numAlgVars: mut i_numAlgVars, numStateVars: mut i_numStateVars, numParams: mut i_numParams, numAlgAliasVars: mut i_numAlgAliasVars, .. }, vars: SimCodeVar::SimVars { aliasVars: ref i_vars_aliasVars, .. }, .. }, mut a_simCode) => {
            let mut ret_14: i32 = 0;
            let mut ret_13: i32 = 0;
            let mut ret_12: i32 = 0;
            let mut ret_11: i32 = 0;
            let mut ret_10: i32 = 0;
            let mut l_ixEnd: Tpl::Text;
            let mut ret_8: i32 = 0;
            let mut ret_7: i32 = 0;
            let mut ret_6: i32 = 0;
            let mut ret_5: i32 = 0;
            let mut l_ixFirstAlias: Tpl::Text;
            let mut ret_3: i32 = 0;
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut l_ixFirstParam: Tpl::Text;
            ret_1 = intMul(2, i_numStateVars.clone());
            ret_2 = intAdd(i_numAlgVars.clone(), i_numDiscreteReal.clone());
            ret_3 = intAdd(ret_1.clone(), ret_2.clone());
            l_ixFirstParam = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_3.clone())).clone())?;
            ret_5 = intMul(2, i_numStateVars.clone());
            ret_6 = intAdd(i_numAlgVars.clone(), i_numDiscreteReal.clone());
            ret_7 = intAdd(ret_5.clone(), ret_6.clone());
            ret_8 = intAdd(i_numParams.clone(), ret_7.clone());
            l_ixFirstAlias = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            ret_10 = intMul(2, i_numStateVars.clone());
            ret_11 = intAdd(i_numAlgVars.clone(), i_numDiscreteReal.clone());
            ret_12 = intAdd(ret_10.clone(), ret_11.clone());
            ret_13 = intAdd(i_numParams.clone(), ret_12.clone());
            ret_14 = intAdd(i_numAlgAliasVars.clone(), ret_13.clone());
            l_ixEnd = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_14.clone())).clone())?;
            txt = fun_169(txt.clone(), i_numAlgAliasVars.clone(), a_simCode.clone(), i_vars_aliasVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("fmi2Real getReal(ModelInstance* comp, const fmi2ValueReference vr) {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(") {\n")).clone(), (literal!("  return comp->fmuData->localData[0]->realVars[vr];\n")).clone(), (literal!("}\n")).clone(), (literal!("if (vr < ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return comp->fmuData->simulationInfo->realParameter[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("];\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = fun_170(txt.clone(), i_numAlgAliasVars.clone(), l_ixFirstAlias.clone(), l_ixEnd.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("return NAN;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getRealFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_171(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

fn fun_173(mut in_txt: Tpl::Text, mut in_a_numAlgAliasVars: i32, mut in_a_ixFirstAlias: Tpl::Text, mut in_a_ixEnd: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_numAlgAliasVars.clone(), in_a_ixFirstAlias.clone(), in_a_ixEnd.clone()) {
        (mut txt, 0, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_ixFirstAlias, mut a_ixEnd) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixEnd.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int ix = realAliasIndexes[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("];\n")).clone(), (literal!("return ix >= 0 ? setReal(comp, ix, value) : setReal(comp, -(ix+1), -value);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_174(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numDiscreteReal: mut i_numDiscreteReal, numAlgVars: mut i_numAlgVars, numStateVars: mut i_numStateVars, numParams: mut i_numParams, numAlgAliasVars: mut i_numAlgAliasVars, .. }, vars: SimCodeVar::SimVars { stateVars: _, .. }, .. }) => {
            let mut ret_14: i32 = 0;
            let mut ret_13: i32 = 0;
            let mut ret_12: i32 = 0;
            let mut ret_11: i32 = 0;
            let mut ret_10: i32 = 0;
            let mut l_ixEnd: Tpl::Text;
            let mut ret_8: i32 = 0;
            let mut ret_7: i32 = 0;
            let mut ret_6: i32 = 0;
            let mut ret_5: i32 = 0;
            let mut l_ixFirstAlias: Tpl::Text;
            let mut ret_3: i32 = 0;
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut l_ixFirstParam: Tpl::Text;
            ret_1 = intMul(2, i_numStateVars.clone());
            ret_2 = intAdd(i_numAlgVars.clone(), i_numDiscreteReal.clone());
            ret_3 = intAdd(ret_1.clone(), ret_2.clone());
            l_ixFirstParam = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_3.clone())).clone())?;
            ret_5 = intMul(2, i_numStateVars.clone());
            ret_6 = intAdd(i_numAlgVars.clone(), i_numDiscreteReal.clone());
            ret_7 = intAdd(ret_5.clone(), ret_6.clone());
            ret_8 = intAdd(i_numParams.clone(), ret_7.clone());
            l_ixFirstAlias = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            ret_10 = intMul(2, i_numStateVars.clone());
            ret_11 = intAdd(i_numAlgVars.clone(), i_numDiscreteReal.clone());
            ret_12 = intAdd(ret_10.clone(), ret_11.clone());
            ret_13 = intAdd(i_numParams.clone(), ret_12.clone());
            ret_14 = intAdd(i_numAlgAliasVars.clone(), ret_13.clone());
            l_ixEnd = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_14.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmi2Status setReal(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Real value) {\n")).clone(), (literal!("  // set start value attribute for all variable that has start value, till initialization mode\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" && (comp->state == model_state_instantiated || comp->state == model_state_initialization_mode)) {\n")).clone(), (literal!("  put_real_element(value, 0, &comp->fmuData->modelData->realVarsData[vr].attribute.start);\n")).clone(), (literal!("}\n")).clone(), (literal!("if (vr < ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(") {\n")).clone(), (literal!("  comp->fmuData->localData[0]->realVars[vr] = value;\n")).clone(), (literal!("  return fmi2OK;\n")).clone(), (literal!("}\n")).clone(), (literal!("if (vr < ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->simulationInfo->realParameter[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("] = value;\n")).clone(), (literal!("return fmi2OK;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = fun_173(txt.clone(), i_numAlgAliasVars.clone(), l_ixFirstAlias.clone(), l_ixEnd.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("return fmi2Error;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setRealFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_174(txt.clone(), a_modelInfo.clone())?;
    Ok(out_txt)
}

fn fun_176(mut in_txt: Tpl::Text, mut in_a_v: SimCodeVar::AliasVariable, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_v.clone(), in_a_simCode.clone()) {
        (mut txt, SimCodeVar::AliasVariable::NOALIAS, _) => {
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenFMU.tpl")).clone(), 914, 31), (literal!("aliasSetVR expected an alias")).clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }, mut a_simCode) => {
            let mut ret_0: i32 = 0;
            ret_0 = SimCodeUtil::lookupVR(i_varName.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }, mut a_simCode) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            ret_1 = SimCodeUtil::lookupVR(i_varName.clone(), a_simCode.clone())?;
            ret_2 = intSub(-1, ret_1.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn aliasSetVR(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_v: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_176(txt.clone(), a_v.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_178(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { aliasvar: i_aliasvar, .. }, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = aliasSetVR(txt.clone(), a_simCode.clone(), i_aliasvar.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_178(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = lm_178(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_179(mut in_txt: Tpl::Text, mut in_a_numAliasVars: i32, mut in_a_simCode: SimCode::SimCode, mut in_a_vars_intAliasVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_numAliasVars.clone(), in_a_simCode.clone(), in_a_vars_intAliasVars.clone())) {
        (txt, 0, _, _) => {
            txt.clone()
        },
        (txt, i_numAliasVars, a_simCode, a_vars_intAliasVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static const int intAliasIndexes[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_numAliasVars.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("] = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 20, alignOfset: 0, alignSeparator: Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(",\n")).clone() }), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_178(txt.clone(), a_vars_intAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_180(mut in_txt: Tpl::Text, mut in_a_numAliasVars: i32, mut in_a_ixFirstAlias: Tpl::Text, mut in_a_ixEnd: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_numAliasVars.clone(), in_a_ixFirstAlias.clone(), in_a_ixEnd.clone()) {
        (mut txt, 0, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_ixFirstAlias, mut a_ixEnd) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixEnd.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int ix = intAliasIndexes[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("];\n")).clone(), (literal!("return ix>=0 ? getInteger(comp, ix) : -getInteger(comp, -(ix+1));\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_181(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numIntAlgVars: mut i_numAlgVars, numIntParams: mut i_numParams, numIntAliasVars: mut i_numAliasVars, .. }, vars: SimCodeVar::SimVars { intAliasVars: ref i_vars_intAliasVars, .. }, .. }, mut a_simCode) => {
            let mut ret_5: i32 = 0;
            let mut ret_4: i32 = 0;
            let mut l_ixEnd: Tpl::Text;
            let mut ret_2: i32 = 0;
            let mut l_ixFirstAlias: Tpl::Text;
            let mut l_ixFirstParam: Tpl::Text;
            l_ixFirstParam = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_numAlgVars.clone())).clone())?;
            ret_2 = intAdd(i_numParams.clone(), i_numAlgVars.clone());
            l_ixFirstAlias = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2.clone())).clone())?;
            ret_4 = intAdd(i_numParams.clone(), i_numAlgVars.clone());
            ret_5 = intAdd(i_numAliasVars.clone(), ret_4.clone());
            l_ixEnd = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_5.clone())).clone())?;
            txt = fun_179(txt.clone(), i_numAliasVars.clone(), a_simCode.clone(), i_vars_intAliasVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("fmi2Integer getInteger(ModelInstance* comp, const fmi2ValueReference vr) {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(") {\n")).clone(), (literal!("  return comp->fmuData->localData[0]->integerVars[vr];\n")).clone(), (literal!("}\n")).clone(), (literal!("if (vr < ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return comp->fmuData->simulationInfo->integerParameter[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("];\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = fun_180(txt.clone(), i_numAliasVars.clone(), l_ixFirstAlias.clone(), l_ixEnd.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("return 0;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getIntegerFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_181(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

fn fun_183(mut in_txt: Tpl::Text, mut in_a_numAliasVars: i32, mut in_a_ixFirstAlias: Tpl::Text, mut in_a_ixEnd: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_numAliasVars.clone(), in_a_ixFirstAlias.clone(), in_a_ixEnd.clone()) {
        (mut txt, 0, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_ixFirstAlias, mut a_ixEnd) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixEnd.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int ix = intAliasIndexes[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("];\n")).clone(), (literal!("return ix >= 0 ? setInteger(comp, ix, value) : setInteger(comp, -(ix+1), -value);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_184(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { varInfo: SimCode::VarInfo { numIntAlgVars: mut i_numAlgVars, numIntParams: mut i_numParams, numIntAliasVars: mut i_numAliasVars, .. }, vars: SimCodeVar::SimVars { stateVars: _, .. }, .. }) => {
            let mut ret_5: i32 = 0;
            let mut ret_4: i32 = 0;
            let mut l_ixEnd: Tpl::Text;
            let mut ret_2: i32 = 0;
            let mut l_ixFirstAlias: Tpl::Text;
            let mut l_ixFirstParam: Tpl::Text;
            l_ixFirstParam = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_numAlgVars.clone())).clone())?;
            ret_2 = intAdd(i_numParams.clone(), i_numAlgVars.clone());
            l_ixFirstAlias = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2.clone())).clone())?;
            ret_4 = intAdd(i_numParams.clone(), i_numAlgVars.clone());
            ret_5 = intAdd(i_numAliasVars.clone(), ret_4.clone());
            l_ixEnd = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_5.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmi2Status setInteger(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Integer value) {\n")).clone(), (literal!("  // set start value attribute for all variable that has start value, till initialization mode\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (vr < ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" && (comp->state == model_state_instantiated || comp->state == model_state_initialization_mode)) {\n")).clone(), (literal!("  comp->fmuData->modelData->integerVarsData[vr].attribute.start = value;\n")).clone(), (literal!("}\n")).clone(), (literal!("if (vr < ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(") {\n")).clone(), (literal!("  comp->fmuData->localData[0]->integerVars[vr] = value;\n")).clone(), (literal!("  return fmi2OK;\n")).clone(), (literal!("}\n")).clone(), (literal!("if (vr < ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstAlias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("comp->fmuData->simulationInfo->integerParameter[vr-")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ixFirstParam.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("] = value;\n")).clone(), (literal!("return fmi2OK;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = fun_183(txt.clone(), i_numAliasVars.clone(), l_ixFirstAlias.clone(), l_ixEnd.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("return fmi2Error;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setIntegerFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_184(txt.clone(), a_modelInfo.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_186(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_186(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_187(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParameters(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_187(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_188(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Boolean")).clone(), (literal!("!")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_188(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_189(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { boolAliasVars: ref i_vars_boolAliasVars, boolParamVars: ref i_vars_boolParamVars, boolAlgVars: ref i_vars_boolAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmi2Boolean getBoolean(ModelInstance* comp, const fmi2ValueReference vr) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_186(txt.clone(), i_vars_boolAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_187(txt.clone(), i_vars_boolParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_188(txt.clone(), i_vars_boolAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmi2False;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getBooleanFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_189(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_191(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_191(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_192(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParametersSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("booleanParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_192(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_193(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("Boolean")).clone(), (literal!("!")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_193(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_194(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { boolAliasVars: ref i_vars_boolAliasVars, boolParamVars: ref i_vars_boolParamVars, boolAlgVars: ref i_vars_boolAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmi2Status setBoolean(ModelInstance* comp, const fmi2ValueReference vr, const fmi2Boolean value) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_191(txt.clone(), i_vars_boolAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_192(txt.clone(), i_vars_boolParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_193(txt.clone(), i_vars_boolAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmi2Error;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmi2OK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setBooleanFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_194(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_196(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_196(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_197(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParameters(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_197(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_198(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVars(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("String")).clone(), (literal!("")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_198(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_199(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringAliasVars: ref i_vars_stringAliasVars, stringParamVars: ref i_vars_stringParamVars, stringAlgVars: ref i_vars_stringAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmi2String getString(ModelInstance* comp, const fmi2ValueReference vr) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_196(txt.clone(), i_vars_stringAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_197(txt.clone(), i_vars_stringParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_198(txt.clone(), i_vars_stringAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return \"\";\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getStringFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_199(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_201(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringVars")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_201(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_202(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchParametersSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("stringParameter")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_202(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_203(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = SwitchAliasVarsSet(txt.clone(), a_simCode.clone(), i_var.clone(), (literal!("String")).clone(), (literal!("")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_203(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_204(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringAliasVars: ref i_vars_stringAliasVars, stringParamVars: ref i_vars_stringParamVars, stringAlgVars: ref i_vars_stringAlgVars, .. }, .. }, mut a_simCode) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmi2Status setString(ModelInstance* comp, const fmi2ValueReference vr, fmi2String value) {\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_201(txt.clone(), i_vars_stringAlgVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_202(txt.clone(), i_vars_stringParamVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_203(txt.clone(), i_vars_stringAliasVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmi2Error;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmi2OK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn setStringFunction2(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_204(txt.clone(), a_modelInfo.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

pub fn setExternalFunction2(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { functions: ref i_functions, vars: SimCodeVar::SimVars { stateVars: _, .. }, .. }) => {
            let mut l_externalFuncs: Tpl::Text;
            l_externalFuncs = setExternalFunctionsSwitch(Tpl::emptyTxt.clone(), i_functions.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fmi2Status setExternalFunction(ModelInstance* c, const fmi2ValueReference vr, const void* value){\n")).clone(), (literal!("  switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_externalFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return fmi2Error;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  return fmi2OK;\n")).clone(), (literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
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
fn lm_207(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = setExternalFunctionSwitch(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_207(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn setExternalFunctionsSwitch(mut txt: Tpl::Text, mut a_functions: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_207(out_txt.clone(), a_functions.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn setExternalFunctionSwitch(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: i_language, extName: i_extName, dynamicLoad: true, .. }) => {
            let mut l_fname: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fname = CodegenCFunctions::extFunctionName(Tpl::emptyTxt.clone(), (i_extName.clone()).clone(), (i_language.clone()).clone())?;
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

fn fun_210(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
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

fn fun_211(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_arrayName: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_arrayName.clone(), in_a_simCode.clone(), in_a_name.clone())) {
        (txt, false, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return comp->fmuData->localData[0]->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_1 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return MMC_STRINGDATA(comp->fmuData->localData[0]->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]); break;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_212(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_arrayName.clone())) {
        (txt, false, a_index, a_simCode, a_name, a_arrayName) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = stringEq((a_arrayName.clone()).clone(), (literal!("stringVars")).clone());
            txt = fun_211(txt.clone(), ret_0.clone(), a_index.clone(), (a_arrayName.clone()).clone(), a_simCode.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_213(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_simCode.clone(), in_a_arrayName.clone(), in_a_name.clone())) {
        (txt, false, a_index, a_simCode, a_arrayName, a_name) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), a_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("der($dummy)")).clone());
            txt = fun_212(txt.clone(), ret_1.clone(), a_index.clone(), a_simCode.clone(), a_name.clone(), (a_arrayName.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_214(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_arrayName.clone()) {
        (mut txt, SimCodeVar::SimVar { index: mut i_index, name: ref i_name, comment: mut i_comment, .. }, mut a_simCode, mut a_arrayName) => {
            let mut ret_2: bool = false;
            let mut txt_1: Tpl::Text;
            let mut l_description: Tpl::Text;
            l_description = fun_210(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            txt_1 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_2 = stringEq((Tpl::textString(txt_1.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_213(txt.clone(), ret_2.clone(), i_index.clone(), a_simCode.clone(), (a_arrayName.clone()).clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn SwitchVars(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar, mut a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_214(txt.clone(), a_simVar.clone(), a_simCode.clone(), (a_arrayName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_216(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
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

fn fun_217(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_arrayName: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_arrayName.clone(), in_a_simCode.clone(), in_a_name.clone())) {
        (txt, false, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return comp->fmuData->simulationInfo->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_1 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return MMC_STRINGDATA(comp->fmuData->simulationInfo->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]); break;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_218(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_arrayName.clone()) {
        (mut txt, SimCodeVar::SimVar { index: mut i_index, name: ref i_name, comment: mut i_comment, .. }, mut a_simCode, mut a_arrayName) => {
            let mut ret_1: bool = false;
            let mut l_description: Tpl::Text;
            l_description = fun_216(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            ret_1 = stringEq((a_arrayName.clone()).clone(), (literal!("stringParameter")).clone());
            txt = fun_217(txt.clone(), ret_1.clone(), i_index.clone(), (a_arrayName.clone()).clone(), a_simCode.clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn SwitchParameters(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar, mut a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_218(txt.clone(), a_simVar.clone(), a_simCode.clone(), (a_arrayName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_220(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
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

fn fun_221(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode, mut in_a_varName: Arc<DAE::ComponentRef>, mut in_a_arrayName: ArcStr, mut in_a_crefName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_simCode.clone(), in_a_varName.clone(), in_a_arrayName.clone(), in_a_crefName.clone())) {
        (txt, false, a_simCode, a_varName, a_arrayName, a_crefName) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_crefName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return get")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(comp, ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_varName.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("); break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, a_crefName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_crefName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return comp->fmuData->localData[0]->timeValue; break;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_222(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode, mut in_a_varName: Arc<DAE::ComponentRef>, mut in_a_arrayName: ArcStr, mut in_a_negate: ArcStr, mut in_a_crefName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_simCode.clone(), in_a_varName.clone(), in_a_arrayName.clone(), in_a_negate.clone(), in_a_crefName.clone())) {
        (txt, false, a_simCode, a_varName, a_arrayName, a_negate, a_crefName) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_crefName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return (")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_negate.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" get")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(comp, ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_varName.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")); break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _, a_crefName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_crefName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return comp->fmuData->localData[0]->timeValue; break;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_223(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable, mut in_a_negate: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr, mut in_a_crefName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone(), in_a_negate.clone(), in_a_simCode.clone(), in_a_arrayName.clone(), in_a_crefName.clone()) {
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }, _, mut a_simCode, mut a_arrayName, mut a_crefName) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("time")).clone());
            txt = fun_221(txt.clone(), ret_1.clone(), a_simCode.clone(), i_varName.clone(), (a_arrayName.clone()).clone(), a_crefName.clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }, mut a_negate, mut a_simCode, mut a_arrayName, mut a_crefName) => {
            let mut ret_3: bool = false;
            let mut txt_2: Tpl::Text;
            txt_2 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_3 = stringEq((Tpl::textString(txt_2.clone())?).clone(), (literal!("time")).clone());
            txt = fun_222(txt.clone(), ret_3.clone(), a_simCode.clone(), i_varName.clone(), (a_arrayName.clone()).clone(), (a_negate.clone()).clone(), a_crefName.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_224(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr, mut in_a_negate: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_arrayName.clone(), in_a_negate.clone()) {
        (mut txt, SimCodeVar::SimVar { aliasvar: mut i_aliasvar, name: ref i_name, comment: mut i_comment, .. }, mut a_simCode, mut a_arrayName, mut a_negate) => {
            let mut ret_2: i32 = 0;
            let mut l_crefName: Tpl::Text;
            let mut l_description: Tpl::Text;
            l_description = fun_220(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            ret_2 = SimCodeUtil::lookupVR(i_name.clone(), a_simCode.clone())?;
            l_crefName = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2.clone())).clone())?;
            txt = fun_223(txt.clone(), i_aliasvar.clone(), (a_negate.clone()).clone(), a_simCode.clone(), (a_arrayName.clone()).clone(), l_crefName.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn SwitchAliasVars(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar, mut a_arrayName: ArcStr, mut a_negate: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_224(txt.clone(), a_simVar.clone(), a_simCode.clone(), (a_arrayName.clone()).clone(), (a_negate.clone()).clone())?;
    Ok(out_txt)
}

fn fun_226(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
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

fn fun_227(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_arrayName: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_arrayName.clone(), in_a_simCode.clone(), in_a_name.clone())) {
        (txt, false, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : comp->fmuData->localData[0]->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = value; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_1 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : comp->fmuData->localData[0]->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = mmc_mk_scon(value); break;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_228(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_arrayName.clone())) {
        (txt, false, a_index, a_simCode, a_name, a_arrayName) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = stringEq((a_arrayName.clone()).clone(), (literal!("stringVars")).clone());
            txt = fun_227(txt.clone(), ret_0.clone(), a_index.clone(), (a_arrayName.clone()).clone(), a_simCode.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_229(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_simCode.clone(), in_a_arrayName.clone(), in_a_name.clone())) {
        (txt, false, a_index, a_simCode, a_arrayName, a_name) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), a_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("der($dummy)")).clone());
            txt = fun_228(txt.clone(), ret_1.clone(), a_index.clone(), a_simCode.clone(), a_name.clone(), (a_arrayName.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_230(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_arrayName.clone()) {
        (mut txt, SimCodeVar::SimVar { index: mut i_index, name: ref i_name, comment: mut i_comment, .. }, mut a_simCode, mut a_arrayName) => {
            let mut ret_2: bool = false;
            let mut txt_1: Tpl::Text;
            let mut l_description: Tpl::Text;
            l_description = fun_226(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            txt_1 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_2 = stringEq((Tpl::textString(txt_1.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_229(txt.clone(), ret_2.clone(), i_index.clone(), a_simCode.clone(), (a_arrayName.clone()).clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn SwitchVarsSet(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar, mut a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_230(txt.clone(), a_simVar.clone(), a_simCode.clone(), (a_arrayName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_232(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
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

fn fun_233(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_arrayName: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_arrayName.clone(), in_a_simCode.clone(), in_a_name.clone())) {
        (txt, false, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : comp->fmuData->simulationInfo->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = value; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_index, a_arrayName, a_simCode, a_name) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_1 = SimCodeUtil::lookupVR(a_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : comp->fmuData->simulationInfo->")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = mmc_mk_scon(value); break;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_234(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_arrayName.clone()) {
        (mut txt, SimCodeVar::SimVar { index: mut i_index, name: ref i_name, comment: mut i_comment, .. }, mut a_simCode, mut a_arrayName) => {
            let mut ret_1: bool = false;
            let mut l_description: Tpl::Text;
            l_description = fun_232(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            ret_1 = stringEq((a_arrayName.clone()).clone(), (literal!("stringParameter")).clone());
            txt = fun_233(txt.clone(), ret_1.clone(), i_index.clone(), (a_arrayName.clone()).clone(), a_simCode.clone(), i_name.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn SwitchParametersSet(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar, mut a_arrayName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_234(txt.clone(), a_simVar.clone(), a_simCode.clone(), (a_arrayName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_236(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
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

fn fun_237(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode, mut in_a_varName: Arc<DAE::ComponentRef>, mut in_a_arrayName: ArcStr, mut in_a_crefName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_simCode.clone(), in_a_varName.clone(), in_a_arrayName.clone(), in_a_crefName.clone())) {
        (txt, false, a_simCode, a_varName, a_arrayName, a_crefName) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_crefName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return set")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(comp, ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_varName.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", value); break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_238(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_negate: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_varName: Arc<DAE::ComponentRef>, mut in_a_arrayName: ArcStr, mut in_a_crefName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_negate.clone(), in_a_simCode.clone(), in_a_varName.clone(), in_a_arrayName.clone(), in_a_crefName.clone())) {
        (txt, false, a_negate, a_simCode, a_varName, a_arrayName, a_crefName) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_crefName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : return set")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_arrayName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(comp, ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(a_varName.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", (")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_negate.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" value)); break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_239(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable, mut in_a_negate: ArcStr, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr, mut in_a_crefName: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone(), in_a_negate.clone(), in_a_simCode.clone(), in_a_arrayName.clone(), in_a_crefName.clone()) {
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }, _, mut a_simCode, mut a_arrayName, mut a_crefName) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("time")).clone());
            txt = fun_237(txt.clone(), ret_1.clone(), a_simCode.clone(), i_varName.clone(), (a_arrayName.clone()).clone(), a_crefName.clone())?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }, mut a_negate, mut a_simCode, mut a_arrayName, mut a_crefName) => {
            let mut ret_3: bool = false;
            let mut txt_2: Tpl::Text;
            txt_2 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_3 = stringEq((Tpl::textString(txt_2.clone())?).clone(), (literal!("time")).clone());
            txt = fun_238(txt.clone(), ret_3.clone(), (a_negate.clone()).clone(), a_simCode.clone(), i_varName.clone(), (a_arrayName.clone()).clone(), a_crefName.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_240(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_arrayName: ArcStr, mut in_a_negate: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_arrayName.clone(), in_a_negate.clone()) {
        (mut txt, SimCodeVar::SimVar { aliasvar: mut i_aliasvar, name: ref i_name, comment: mut i_comment, .. }, mut a_simCode, mut a_arrayName, mut a_negate) => {
            let mut ret_2: i32 = 0;
            let mut l_crefName: Tpl::Text;
            let mut l_description: Tpl::Text;
            l_description = fun_236(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            ret_2 = SimCodeUtil::lookupVR(i_name.clone(), a_simCode.clone())?;
            l_crefName = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2.clone())).clone())?;
            txt = fun_239(txt.clone(), i_aliasvar.clone(), (a_negate.clone()).clone(), a_simCode.clone(), (a_arrayName.clone()).clone(), l_crefName.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn SwitchAliasVarsSet(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_simVar: SimCodeVar::SimVar, mut a_arrayName: ArcStr, mut a_negate: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_240(txt.clone(), a_simVar.clone(), a_simCode.clone(), (a_arrayName.clone()).clone(), (a_negate.clone()).clone())?;
    Ok(out_txt)
}

fn fun_242(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_index0: i32, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_index0.clone(), in_a_simCode.clone())) {
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name: i_name, .. }, a_index0, a_simCode) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(i_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": return ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_243(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut x_index0: i32 = 0;
            let mut txt = (*txt).clone();
            x_index0 = Tpl::getIteri_i0(txt.clone())?;
            txt = fun_242(txt.clone(), i_var.clone(), x_index0.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_243(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_244(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_index0: i32, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_index0.clone(), in_a_simCode.clone())) {
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name: i_name, .. }, a_index0, a_simCode) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(i_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": return ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_245(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_simCode) => {
            let mut x_index0: i32 = 0;
            let mut txt = (*txt).clone();
            x_index0 = Tpl::getIteri_i0(txt.clone())?;
            txt = fun_244(txt.clone(), i_var.clone(), x_index0.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_245(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn mapInputAndOutputs(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { vars: SimCodeVar::SimVars { outputVars: ref i_outputVars, inputVars: ref i_inputVars, .. }, .. }, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* function maps input references to a input index used in partialDerivatives */\n")).clone(), (literal!("fmi2ValueReference mapInputReference2InputNumber(const fmi2ValueReference vr) {\n")).clone(), (literal!("    switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_243(txt.clone(), i_inputVars.clone(), i_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return -1;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    }\n")).clone(), (literal!("}\n")).clone(), (literal!("/* function maps output references to a input index used in partialDerivatives */\n")).clone(), (literal!("fmi2ValueReference mapOutputReference2OutputNumber(const fmi2ValueReference vr) {\n")).clone(), (literal!("    switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_245(txt.clone(), i_outputVars.clone(), i_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return -1;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_247(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar, mut in_a_FMUType: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone(), in_a_FMUType.clone(), in_a_simCode.clone())) {
        (txt, SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_REAL { varLst: _ }, name: i_name, .. }, a_FMUType, a_simCode) => {
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(i_name.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": return ")).clone() }))?;
            ret_1 = SimCodeUtil::lookupVRForRealOutputDerivative(i_name.clone(), a_simCode.clone(), (a_FMUType.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; break;")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_248(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUType: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUType.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUType, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_247(txt.clone(), i_var.clone(), (a_FMUType.clone()).clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_248(txt.clone(), rest.clone(), (a_FMUType.clone()).clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn mapRealOutputDerivatives(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_FMUType.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: SimCode::ModelInfo { vars: SimCodeVar::SimVars { outputVars: ref i_outputVars, .. }, .. }, .. }, mut a_FMUType) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* function maps output references to an internal output Real derivatives */\n")).clone(), (literal!("fmi2ValueReference mapOutputReference2RealOutputDerivatives(const fmi2ValueReference vr) {\n")).clone(), (literal!("    switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_248(txt.clone(), i_outputVars.clone(), (a_FMUType.clone()).clone(), i_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return -1;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_250(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, Arc<DAE::ComponentRef>)>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_index, i_cref), tail: rest }, a_simCode) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(i_cref.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": return ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; break;")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_250(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn mapInitialUnknownsdependentCrefs(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelStructure: Some(SimCode::FmiModelStructure { fmiInitialUnknowns: SimCode::FmiInitialUnknowns { sortedUnknownCrefs: ref i_sortedUnknownCrefs, .. }, .. }), .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* function maps initialUnknowns UnknownVars ValueReferences to an internal partial derivatives index */\n")).clone(), (literal!("fmi2ValueReference mapInitialUnknownsdependentIndex(const fmi2ValueReference vr) {\n")).clone(), (literal!("    switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_250(txt.clone(), i_sortedUnknownCrefs.clone(), i_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return -1;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
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
fn lm_252(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, Arc<DAE::ComponentRef>)>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_index, i_cref), tail: rest }, a_simCode) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            ret_0 = SimCodeUtil::lookupVR(i_cref.clone(), a_simCode.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": return ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; break;")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_252(txt.clone(), rest.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn mapInitialUnknownsIndependentCrefs(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelStructure: Some(SimCode::FmiModelStructure { fmiInitialUnknowns: SimCode::FmiInitialUnknowns { sortedknownCrefs: ref i_sortedknownCrefs, .. }, .. }), .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* function maps initialUnknowns knownVars ValueReferences to an internal partial derivatives index */\n")).clone(), (literal!("fmi2ValueReference mapInitialUnknownsIndependentIndex(const fmi2ValueReference vr) {\n")).clone(), (literal!("    switch (vr) {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_252(txt.clone(), i_sortedknownCrefs.clone(), i_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("default:\n")).clone(), (literal!("  return -1;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_254(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("head -n20 Makefile > ../resources/$(FMIPLATFORM).summary")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_255(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t$(MAKE) distclean")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_256(mut in_txt: Tpl::Text, mut in_a_platform: ArcStr, mut in_a_libsPos2: ArcStr, mut in_a_libsPos1: ArcStr, mut in_a_dirExtra: ArcStr, mut in_a_modelNamePrefix: ArcStr, mut in_a_fmuTargetName: ArcStr, mut in_a_fileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_platform.clone(), in_a_libsPos2.clone(), in_a_libsPos1.clone(), in_a_dirExtra.clone(), in_a_modelNamePrefix.clone(), in_a_fmuTargetName.clone(), in_a_fileNamePrefix.clone())) {
        (txt, i_platform @ Deref @ "win32", a_libsPos2, a_libsPos1, a_dirExtra, a_modelNamePrefix, a_fmuTargetName, a_fileNamePrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_FMU: nozip\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd .. && rm -f ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmu && zip -r ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu *\n")).clone(), (literal!("nozip: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_functions.h ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_literals.h $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(CXX) -shared -I. -o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) $(RUNTIMEFILES) $(FMISUNDIALSFILES) $(OFILES) $(CPPFLAGS) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_dirExtra.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_libsPos1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_libsPos2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" $(CFLAGS) $(LDFLAGS) -llis -Wl,--kill-at\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mkdir.exe -p ../binaries/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_platform.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dlltool -d ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".def --dllname ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) --output-lib ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".lib --kill-at\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".lib ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.libs ../binaries/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f *.o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("$(DLLEXT) $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd .. && rm -f ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmu && zip -r ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu *\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (txt, i_platform @ Deref @ "win64", a_libsPos2, a_libsPos1, a_dirExtra, a_modelNamePrefix, a_fmuTargetName, a_fileNamePrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_FMU: nozip\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd .. && rm -f ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmu && zip -r ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu *\n")).clone(), (literal!("nozip: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_functions.h ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_literals.h $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(CXX) -shared -I. -o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) $(RUNTIMEFILES) $(FMISUNDIALSFILES) $(OFILES) $(CPPFLAGS) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_dirExtra.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_libsPos1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_libsPos2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" $(CFLAGS) $(LDFLAGS) -llis -Wl,--kill-at\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mkdir.exe -p ../binaries/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_platform.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dlltool -d ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".def --dllname ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) --output-lib ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".lib --kill-at\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".lib ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.libs ../binaries/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f *.o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("$(DLLEXT) $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd .. && rm -f ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmu && zip -r ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu *\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (txt, _, a_libsPos2, a_libsPos1, a_dirExtra, a_modelNamePrefix, a_fmuTargetName, a_fileNamePrefix) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_FMU: nozip\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd .. && rm -f ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmu && zip -r ../")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmuTargetName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".fmu *\n")).clone(), (literal!("nozip: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_functions.h ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_literals.h $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("mkdir -p ../binaries/$(FMIPLATFORM)\n")).clone(), (literal!("ifeq (@LIBTYPE_DYNAMIC@,1)\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(LD) -o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_dirExtra.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_libsPos1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_libsPos2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" @BDYNAMIC@ $(LDFLAGS)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.libs ../binaries/$(FMIPLATFORM)/\n")).clone(), (literal!("endif\n")).clone()], lastHasNewLine: true }))?;
            ret_0 = Flags::getConfigEnum(Flags::FMI_FILTER.clone())?;
            ret_1 = intLt(ret_0.clone(), 4);
            txt = fun_254(txt.clone(), ret_1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("ifeq (@LIBTYPE_STATIC@,1)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".a\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(AR) -rsu ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".a $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".a ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_FMU.libs ../binaries/$(FMIPLATFORM)/\n")).clone(), (literal!("endif\n")).clone()], lastHasNewLine: true }))?;
            ret_2 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            txt = fun_255(txt.clone(), ret_2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("distclean: clean\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("rm -f Makefile config.status config.log\n")).clone(), (literal!("clean:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".def ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".o ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".a ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) $(MAINOBJ) $(OFILES) $(RUNTIMEFILES) $(FMISUNDIALSFILES)")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getPlatformString2(mut txt: Tpl::Text, mut a_modelNamePrefix: ArcStr, mut a_platform: ArcStr, mut a_fileNamePrefix: ArcStr, mut a_fmuTargetName: ArcStr, mut a_dirExtra: ArcStr, mut a_libsPos1: ArcStr, mut a_libsPos2: ArcStr, mut a_omhome: ArcStr, mut a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_fmudirname: Tpl::Text;
    ret_1 = (Util::hashFileNamePrefix((a_fileNamePrefix.clone()).clone())).clone();
    l_fmudirname = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    l_fmudirname = Tpl::writeTok(l_fmudirname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp")).clone() }))?;
    out_txt = fun_256(txt.clone(), (a_platform.clone()).clone(), (a_libsPos2.clone()).clone(), (a_libsPos1.clone()).clone(), (a_dirExtra.clone()).clone(), (a_modelNamePrefix.clone()).clone(), (a_fmuTargetName.clone()).clone(), (a_fileNamePrefix.clone()).clone())?;
    Ok(out_txt)
}

fn fun_258(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define FMU_EXPERIMENTAL 1")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn settingsfile(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { fmiSimulationFlags: _, delayedExps: SimCode::DelayedExpression { maxDelayedIndex: ref i_maxDelayedIndex, .. }, modelInfo: SimCode::ModelInfo { varInfo: SimCode::VarInfo { numStringAlgVars: ref i_varInfo_numStringAlgVars, numMixedSystems: ref i_varInfo_numMixedSystems, numNonLinearSystems: ref i_varInfo_numNonLinearSystems, numLinearSystems: ref i_varInfo_numLinearSystems, .. }, .. }, .. }) => {
            let mut ret_0: bool = false;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#if !defined(OMC_SIM_SETTINGS_CMDLINE)\n")).clone(), (literal!("#define OMC_SIM_SETTINGS_CMDLINE\n")).clone(), (literal!("#define OMC_NUM_LINEAR_SYSTEMS ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numLinearSystems.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define OMC_NUM_NONLINEAR_SYSTEMS ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numNonLinearSystems.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define OMC_NUM_MIXED_SYSTEMS ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numMixedSystems.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define OMC_NDELAY_EXPRESSIONS ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_maxDelayedIndex.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define OMC_NVAR_STRING ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_varInfo_numStringAlgVars.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_0 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_258(txt.clone(), ret_0.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define OMC_MODEL_PREFIX \"")).clone() }))?;
            txt = CodegenUtilSimulation::modelNamePrefix(txt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("#define OMC_MINIMAL_RUNTIME 1\n")).clone(), (literal!("#define OMC_FMI_RUNTIME 1\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
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
fn lm_260(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_260(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_261(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_261(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_262(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_262(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_263(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_263(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_264(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_264(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_265(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_sundialsObjectFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_runtimeObjectFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_cminpackObjectFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_dgesvObjectFiles: Arc<metamodelica::List<ArcStr>>, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_sundialsObjectFiles.clone(), in_a_runtimeObjectFiles.clone(), in_a_cminpackObjectFiles.clone(), in_a_dgesvObjectFiles.clone(), in_a_sourceFiles.clone())) {
        (txt, SimCode::SimCode { simulationSettingsOpt: _, makefileParams: SimCodeFunction::MakefileParams { ccompiler: _, .. }, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }, a_sundialsObjectFiles, a_runtimeObjectFiles, a_cminpackObjectFiles, a_dgesvObjectFiles, a_sourceFiles) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CFILES = ")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" \\\n")).clone(), (literal!("         ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_260(txt.clone(), a_sourceFiles.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("OFILES=$(CFILES:.c=.o)\n")).clone(), (literal!("\n")).clone(), (literal!("RUNTIMEDIR=.\n")).clone(), (literal!("ifneq ($(NEED_DGESV),)\n")).clone(), (literal!("DGESV_OBJS = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_261(txt.clone(), a_dgesvObjectFiles.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("endif\n")).clone(), (literal!("ifneq ($(NEED_CMINPACK),)\n")).clone(), (literal!("CMINPACK_OBJS=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_262(txt.clone(), a_cminpackObjectFiles.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("endif\n")).clone(), (literal!("ifneq ($(NEED_RUNTIME),)\n")).clone(), (literal!("RUNTIMEFILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_263(txt.clone(), a_runtimeObjectFiles.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" $(DGESV_OBJS) $(CMINPACK_OBJS)\n")).clone(), (literal!("endif\n")).clone(), (literal!("ifneq ($(NEED_SUNDIALS),)\n")).clone(), (literal!("FMISUNDIALSFILES=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_264(txt.clone(), a_sundialsObjectFiles.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("LDFLAGS+=-Wl,-Bstatic -lsundials_cvode -lsundials_nvecserial -Wl,-Bdynamic\n")).clone(), (literal!("endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_266(mut in_txt: Tpl::Text, mut in_a_modelInfo_directory: ArcStr) -> Result<Tpl::Text> {
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
fn lm_267(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lib, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lib.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_267(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_268(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_269(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_270(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
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

fn fun_271(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_makefileParams_omhome: ArcStr) -> Result<Tpl::Text> {
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

fn fun_272(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
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

fn fun_273(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_common: Tpl::Text, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_common.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { fmuTargetName: ref i_fmuTargetName, fileNamePrefix: ref i_fileNamePrefix, simulationSettingsOpt: _, makefileParams: SimCodeFunction::MakefileParams { omhome: ref i_makefileParams_omhome, platform: ref i_makefileParams_platform, libs: ref i_makefileParams_libs, .. }, modelInfo: SimCode::ModelInfo { directory: ref i_modelInfo_directory, .. }, .. }, mut a_common, mut a_FMUVersion) => {
            let mut ret_15: ArcStr = arcstr::literal!("");
            let mut ret_14: ArcStr = arcstr::literal!("");
            let mut ret_13: ArcStr = arcstr::literal!("");
            let mut ret_12: ArcStr = arcstr::literal!("");
            let mut ret_11: ArcStr = arcstr::literal!("");
            let mut ret_10: bool = false;
            let mut ret_9: bool = false;
            let mut l_mkdir: Tpl::Text;
            let mut txt_7: Tpl::Text;
            let mut l_compilecmds: Tpl::Text;
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut l_fmudirname: Tpl::Text;
            let mut l_libsPos2: Tpl::Text;
            let mut l_libsPos1: Tpl::Text;
            let mut l_libsStr: Tpl::Text;
            let mut l_dirExtra: Tpl::Text;
            l_dirExtra = fun_266(Tpl::emptyTxt.clone(), (i_modelInfo_directory.clone()).clone())?;
            l_libsStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_libsStr = lm_267(l_libsStr.clone(), i_makefileParams_libs.clone())?;
            l_libsStr = Tpl::popIter(l_libsStr.clone())?;
            l_libsPos1 = fun_268(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            l_libsPos2 = fun_269(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            ret_5 = (Util::hashFileNamePrefix((i_fileNamePrefix.clone()).clone())).clone();
            l_fmudirname = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_5.clone()).clone())?;
            l_fmudirname = Tpl::writeTok(l_fmudirname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp")).clone() }))?;
            txt_7 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            l_compilecmds = getPlatformString2(Tpl::emptyTxt.clone(), (Tpl::textString(txt_7.clone())?).clone(), (i_makefileParams_platform.clone()).clone(), (i_fileNamePrefix.clone()).clone(), (i_fmuTargetName.clone()).clone(), (Tpl::textString(l_dirExtra.clone())?).clone(), (Tpl::textString(l_libsPos1.clone())?).clone(), (Tpl::textString(l_libsPos2.clone())?).clone(), (i_makefileParams_omhome.clone()).clone(), (a_FMUVersion.clone()).clone())?;
            l_mkdir = fun_270(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# Makefile generated by OpenModelica\n")).clone(), (literal!("\n")).clone(), (literal!("# Simulations use -O3 by default\n")).clone(), (literal!("SIM_OR_DYNLOAD_OPT_LEVEL=\n")).clone(), (literal!("MODELICAUSERCFLAGS=\n")).clone(), (literal!("CXX=cl\n")).clone(), (literal!("EXEEXT=.exe\n")).clone(), (literal!("DLLEXT=.dll\n")).clone(), (literal!("FMUEXT=.fmu\n")).clone(), (literal!("PLATWIN32 = win32\n")).clone(), (literal!("\n")).clone(), (literal!("# /Od - Optimization disabled\n")).clone(), (literal!("# /EHa enable C++ EH (w/ SEH exceptions)\n")).clone(), (literal!("# /fp:except - consider floating-point exceptions when generating code\n")).clone(), (literal!("# /arch:SSE2 - enable use of instructions available with SSE2 enabled CPUs\n")).clone(), (literal!("# /I - Include Directories\n")).clone(), (literal!("# /DNOMINMAX - Define NOMINMAX (does what it says)\n")).clone(), (literal!("# /TP - Use C++ Compiler\n")).clone(), (literal!("CFLAGS=/MP /Od /ZI /EHa /fp:except /I\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/include/omc/c\" /I\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/include/omc/msvc/\" ")).clone() }))?;
            ret_9 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_271(txt.clone(), ret_9.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /I. /DNOMINMAX /TP /DNO_INTERACTIVE_DEPENDENCY  ")).clone() }))?;
            ret_10 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_272(txt.clone(), ret_10.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("# /ZI enable Edit and Continue debug info\n")).clone(), (literal!("CDFLAGS=/ZI\n")).clone(), (literal!("\n")).clone(), (literal!("# /MD - link with MSVCRT.LIB\n")).clone(), (literal!("# /link - [linker options and libraries]\n")).clone(), (literal!("# /LIBPATH: - Directories where libs can be found\n")).clone(), (literal!("LDFLAGS=/MD /link /dll /debug /pdb:\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".pdb\" /LIBPATH:\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/lib/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (arcstr::literal!(Autoconf::triple)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/omc/msvc/\" /LIBPATH:\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/lib/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (arcstr::literal!(Autoconf::triple)).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/omc/msvc/release/\" ")).clone() }))?;
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
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\SUNDIALS_CVODE.DLL ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_12 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_12.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\SUNDIALS_KINSOL.DLL ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_13 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_13.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\SUNDIALS_NVECSERIAL.DLL ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_14 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_14.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\bin\\LAPACK_WIN32_MT.DLL ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fmudirname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\binaries\\$(PLATWIN32)\n")).clone(), (literal!("copy ")).clone()], lastHasNewLine: false }))?;
            ret_15 = (System::stringReplace((i_makefileParams_omhome.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_15.clone()).clone())?;
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
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$(DLLEXT) ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.c ")).clone() }))?;
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

fn fun_274(mut in_txt: Tpl::Text, mut in_a_modelInfo_directory: ArcStr) -> Result<Tpl::Text> {
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
fn lm_275(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lib, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lib.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_275(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_276(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_277(mut in_txt: Tpl::Text, mut in_a_dirExtra: Tpl::Text, mut in_a_libsStr: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_278(mut in_txt: Tpl::Text, mut in_a_fmiSimulationFlags: Option<SimCode::FmiSimulationFlags>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiSimulationFlags.clone()) {
        (mut txt, Some(_)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-Isundials/ -I/util")).clone() }))?;
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
fn lm_279(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_279(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_280(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_common: Tpl::Text, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_common.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { fmuTargetName: ref i_fmuTargetName, fileNamePrefix: ref i_fileNamePrefix, fmiSimulationFlags: ref i_fmiSimulationFlags, simulationSettingsOpt: _, makefileParams: SimCodeFunction::MakefileParams { includes: ref i_makefileParams_includes, omhome: ref i_makefileParams_omhome, platform: ref i_makefileParams_platform, libs: ref i_makefileParams_libs, .. }, delayedExps: SimCode::DelayedExpression { maxDelayedIndex: _, .. }, modelInfo: SimCode::ModelInfo { directory: ref i_modelInfo_directory, varInfo: SimCode::VarInfo { numZeroCrossings: _, .. }, .. }, .. }, mut a_common, mut a_FMUVersion) => {
            let mut l_thirdPartyInclude: Tpl::Text;
            let mut l_platformstr: Tpl::Text;
            let mut txt_5: Tpl::Text;
            let mut l_compilecmds: Tpl::Text;
            let mut l_libsPos2: Tpl::Text;
            let mut l_libsPos1: Tpl::Text;
            let mut l_libsStr: Tpl::Text;
            let mut l_dirExtra: Tpl::Text;
            l_dirExtra = fun_274(Tpl::emptyTxt.clone(), (i_modelInfo_directory.clone()).clone())?;
            l_libsStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_libsStr = lm_275(l_libsStr.clone(), i_makefileParams_libs.clone())?;
            l_libsStr = Tpl::popIter(l_libsStr.clone())?;
            l_libsPos1 = fun_276(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            l_libsPos2 = fun_277(Tpl::emptyTxt.clone(), l_dirExtra.clone(), l_libsStr.clone())?;
            txt_5 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            l_compilecmds = getPlatformString2(Tpl::emptyTxt.clone(), (Tpl::textString(txt_5.clone())?).clone(), (i_makefileParams_platform.clone()).clone(), (i_fileNamePrefix.clone()).clone(), (i_fmuTargetName.clone()).clone(), (Tpl::textString(l_dirExtra.clone())?).clone(), (Tpl::textString(l_libsPos1.clone())?).clone(), (Tpl::textString(l_libsPos2.clone())?).clone(), (i_makefileParams_omhome.clone()).clone(), (a_FMUVersion.clone()).clone())?;
            l_platformstr = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            l_thirdPartyInclude = fun_278(Tpl::emptyTxt.clone(), i_fmiSimulationFlags.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# Makefile generated by OpenModelica\n")).clone(), (literal!("CC=@CC@\n")).clone(), (literal!("AR=@AR@\n")).clone(), (literal!("CFLAGS=@CFLAGS@\n")).clone(), (literal!("LD=$(CC) -shared\n")).clone(), (literal!("# define OMC_LDFLAGS_LINK_TYPE env variable to override this\n")).clone(), (literal!("OMC_LDFLAGS_LINK_TYPE=static\n")).clone(), (literal!("LDFLAGS=@LDFLAGS@ @LIBS@\n")).clone(), (literal!("DLLEXT=@DLLEXT@\n")).clone(), (literal!("NEED_RUNTIME=@NEED_RUNTIME@\n")).clone(), (literal!("NEED_DGESV=@NEED_DGESV@\n")).clone(), (literal!("NEED_CMINPACK=@NEED_CMINPACK@\n")).clone(), (literal!("NEED_SUNDIALS=@NEED_SUNDIALS@\n")).clone(), (literal!("FMIPLATFORM=@FMIPLATFORM@\n")).clone(), (literal!("# Note: Simulation of the fmu with dymola does not work with -finline-small-functions (enabled by most optimization levels)\n")).clone(), (literal!("CPPFLAGS=@CPPFLAGS@\n")).clone(), (literal!("override CPPFLAGS += -DFMI2_OVERRIDE_FUNCTION_PREFIX\n")).clone(), (literal!("\n")).clone(), (literal!("override CPPFLAGS += ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_279(txt.clone(), i_makefileParams_includes.clone())?;
            txt = Tpl::popIter(txt.clone())?;
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

fn fun_281(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_target: ArcStr, mut in_a_common: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_target.clone(), in_a_common.clone(), in_a_FMUVersion.clone(), in_a_simCode.clone())) {
        (txt, Deref @ "msvc", _, a_common, a_FMUVersion, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_273(txt.clone(), a_simCode.clone(), a_common.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ "gcc", _, a_common, a_FMUVersion, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = fun_280(txt.clone(), a_simCode.clone(), a_common.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_target, _, _, _) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("target ")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (a_target.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" is not handled!")).clone() }))?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenFMU.tpl")).clone(), 1534, 11), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn fmuMakefile(mut txt: Tpl::Text, mut a_target: ArcStr, mut a_simCode: SimCode::SimCode, mut a_FMUVersion: ArcStr, mut a_sourceFiles: Arc<metamodelica::List<ArcStr>>, mut a_runtimeObjectFiles: Arc<metamodelica::List<ArcStr>>, mut a_dgesvObjectFiles: Arc<metamodelica::List<ArcStr>>, mut a_cminpackObjectFiles: Arc<metamodelica::List<ArcStr>>, mut a_sundialsObjectFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut str_2: ArcStr = arcstr::literal!("");
    let mut txt_1: Tpl::Text;
    let mut l_common: Tpl::Text;
    l_common = fun_265(Tpl::emptyTxt.clone(), a_simCode.clone(), a_sundialsObjectFiles.clone(), a_runtimeObjectFiles.clone(), a_cminpackObjectFiles.clone(), a_dgesvObjectFiles.clone(), a_sourceFiles.clone())?;
    txt_1 = CodegenUtil::getGeneralTarget(Tpl::emptyTxt.clone(), (a_target.clone()).clone())?;
    str_2 = (Tpl::textString(txt_1.clone())?).clone();
    out_txt = fun_281(txt.clone(), (str_2.clone()).clone(), (a_target.clone()).clone(), l_common.clone(), (a_FMUVersion.clone()).clone(), a_simCode.clone())?;
    Ok(out_txt)
}

fn fun_283(mut in_txt: Tpl::Text, mut in_a_makefileParams_platform: ArcStr) -> Result<Tpl::Text> {
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

fn fun_284(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_modelInfo_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_modelInfo_name.clone())) {
        (txt, Deref @ "omsicpp", a_modelInfo_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("chmod +x ")).clone() }))?;
            txt = CodegenUtil::dotPath(txt.clone(), a_modelInfo_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".sh")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_285(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_modelInfo_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_modelInfo_name.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_modelInfo_name) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (Config::simCodeTarget()?).clone();
            txt = fun_284(txt.clone(), (ret_0.clone()).clone(), a_modelInfo_name.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_286(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_fileNamePrefixHash: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_fileNamePrefixHash.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, simulationSettingsOpt: _, makefileParams: SimCodeFunction::MakefileParams { omhome: mut i_makefileParams_omhome, platform: mut i_makefileParams_platform, .. }, modelInfo: SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }, mut a_fileNamePrefixHash) => {
            let mut ret_5: bool = false;
            let mut ret_4: bool = false;
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut l_mkdir: Tpl::Text;
            let mut l_includedir: Tpl::Text;
            l_includedir = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_fileNamePrefixHash.clone()).clone())?;
            l_includedir = Tpl::writeTok(l_includedir.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources/")).clone() }))?;
            l_mkdir = fun_283(Tpl::emptyTxt.clone(), (i_makefileParams_platform.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("# FIXME: before you push into master...\n")).clone(), (literal!("RUNTIMEDIR=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/include/omc/c/\n")).clone(), (literal!("#COPY_RUNTIMEFILES=$(FMI_ME_OBJS:%= && (OMCFILE=% && cp $(RUNTIMEDIR)/$$OMCFILE.c $$OMCFILE.c))\n")).clone(), (literal!("\n")).clone(), (literal!("fmu:\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("rm -f ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefixHash.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fmutmp/sources/")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_init.xml\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/share/omc/runtime/c/fmi/buildproject/\"* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefixHash.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources\n")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\t")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cp -a ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.libs ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefixHash.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".fmutmp/sources/\n")).clone() }))?;
            ret_2 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win32")).clone());
            ret_3 = stringEq((i_makefileParams_platform.clone()).clone(), (literal!("win64")).clone());
            ret_4 = boolOr(ret_2.clone(), ret_3.clone());
            ret_5 = boolNot(ret_4.clone());
            txt = fun_285(txt.clone(), ret_5.clone(), i_modelInfo_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmuSourceMakefile(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_FMUVersion: ArcStr, mut a_fileNamePrefixHash: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_286(txt.clone(), a_simCode.clone(), (a_fileNamePrefixHash.clone()).clone())?;
    Ok(out_txt)
}

fn fun_288(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fileNamePrefix.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_fileNamePrefix) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";***************************************************\n")).clone(), (literal!("; Experimetnal function for FMI for ModelExchange\n")).clone(), (literal!(";****************************************************\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_fmiGetSpecificDerivatives @45")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_289(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fileNamePrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fileNamePrefix.clone()) {
        (mut txt, false, mut a_fileNamePrefix) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("EXPORTS\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiCompletedIntegratorStep @1\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiEventUpdate @2\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiFreeModelInstance @3\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetBoolean @4\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetContinuousStates @5\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetDerivatives @6\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetEventIndicators @7\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetInteger @8\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetModelTypesPlatform @9\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetNominalContinuousStates @10\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetReal @11\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetStateValueReferences @12\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetString @13\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetVersion @14\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiInitialize @15\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiInstantiateModel @16\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetBoolean @17\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetContinuousStates @18\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetDebugLogging @19\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetExternalFunction @20\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetInteger @21\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetReal @22\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetString @23\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetTime @24\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_fmiTerminate @25")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fileNamePrefix) => {
            let mut ret_0: bool = false;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("EXPORTS\n")).clone(), (literal!("  ;***************************************************\n")).clone(), (literal!("  ;Common Functions\n")).clone(), (literal!("  ;****************************************************\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetTypesPlatform @1\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetVersion @2\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetDebugLogging @3\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiInstantiate @4\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiFreeInstance @5\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetupExperiment @6\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiEnterInitializationMode @7\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiExitInitializationMode @8\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiTerminate @9\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiReset @10\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetReal @11\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetInteger @12\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetBoolean @13\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetString @14\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetReal @15\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetInteger @16\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetBoolean @17\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetString @18\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetFMUstate @19\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetFMUstate @20\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiFreeFMUstate @21\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSerializedFMUstateSize @22\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSerializeFMUstate @23\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiDeSerializeFMUstate @24\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_fmiGetDirectionalDerivative @25\n")).clone(), (literal!(";***************************************************\n")).clone(), (literal!(";Functions for FMI for Model Exchange\n")).clone(), (literal!(";****************************************************\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiEnterEventMode @26\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiNewDiscreteStates @27\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiEnterContinuousTimeMode @28\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiCompletedIntegratorStep @29\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetTime @30\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetContinuousStates @31\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetDerivatives @32\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetEventIndicators @33\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetContinuousStates @34\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_fmiGetNominalsOfContinuousStates @35\n")).clone(), (literal!(";***************************************************\n")).clone(), (literal!(";Functions for FMI for Co-Simulation\n")).clone(), (literal!(";****************************************************\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiSetRealInputDerivatives @36\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetRealOutputDerivatives @37\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiDoStep @38\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiCancelStep @39\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetStatus @40\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetRealStatus @41\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetIntegerStatus @42\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetBooleanStatus @43\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_fmiGetStringStatus @44\n")).clone() }))?;
            ret_0 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_288(txt.clone(), ret_0.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmudeffile(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_FMUVersion.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, simulationSettingsOpt: _, makefileParams: SimCodeFunction::MakefileParams { ccompiler: _, .. }, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }, mut a_FMUVersion) => {
            let mut ret_0: bool = false;
            ret_0 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_289(txt.clone(), ret_0.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_291(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiDescription: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiDescription.clone()) {
        (mut txt, false, mut a_fmiInfo_fmiDescription) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn importFMUModelDescription(mut in_txt: Tpl::Text, mut in_a_fmi: FMI::FmiImport) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmi.clone()) {
        (mut txt, FMI::FmiImport { generateOutputConnectors: mut i_generateOutputConnectors, generateInputConnectors: mut i_generateInputConnectors, fmiModelVariablesList: ref i_fmiModelVariablesList, fmiTypeDefinitionsList: ref i_fmiTypeDefinitionsList, fmiExperimentAnnotation: FMI::ExperimentAnnotation { fmiExperimentStartTime: _, .. }, fmiInfo: FMI::Info { fmiDescription: mut i_fmiInfo_fmiDescription, fmiModelIdentifier: mut i_fmiInfo_fmiModelIdentifier, .. }, .. }) => {
            let mut ret_0: bool = false;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("model ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_Input_Output_FMU")).clone() }))?;
            ret_0 = stringEq((i_fmiInfo_fmiDescription.clone()).clone(), (literal!("")).clone());
            txt = fun_291(txt.clone(), ret_0.clone(), (i_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = dumpFMUModelDescriptionVariablesList(txt.clone(), (literal!("1.0")).clone(), i_fmiModelVariablesList.clone(), i_fmiTypeDefinitionsList.clone(), i_generateInputConnectors.clone(), i_generateOutputConnectors.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_Input_Output_FMU;")).clone() }))?;
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
fn lm_293(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::ModelVariables>>, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone(), in_a_fmiTypeDefinitionsList.clone(), in_a_FMUVersion.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiModelVariable, tail: rest }, a_generateOutputConnectors, a_generateInputConnectors, a_fmiTypeDefinitionsList, a_FMUVersion) => {
            let mut txt = (*txt).clone();
            txt = dumpFMUModelDescriptionVariable(txt.clone(), (a_FMUVersion.clone()).clone(), i_fmiModelVariable.clone(), a_fmiTypeDefinitionsList.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_293(txt.clone(), rest.clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), a_fmiTypeDefinitionsList.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFMUModelDescriptionVariablesList(mut txt: Tpl::Text, mut a_FMUVersion: ArcStr, mut a_fmiModelVariablesList: Arc<metamodelica::List<FMI::ModelVariables>>, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut a_generateInputConnectors: bool, mut a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_293(out_txt.clone(), a_fmiModelVariablesList.clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), a_fmiTypeDefinitionsList.clone(), (a_FMUVersion.clone()).clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_295(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_296(mut in_txt: Tpl::Text, mut in_a_isInputOrOutput: Tpl::Text, mut in_a_y2Placement: i32, mut in_a_y1Placement: i32, mut in_a_x2Placement: i32, mut in_a_x1Placement: i32, mut in_a_description: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isInputOrOutput.clone(), in_a_y2Placement.clone(), in_a_y1Placement.clone(), in_a_x2Placement.clone(), in_a_x1Placement.clone(), in_a_description.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_name.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, _, _, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_y2Placement, a_y1Placement, a_x2Placement, a_x1Placement, a_description, a_generateOutputConnectors, a_generateInputConnectors, a_baseType, a_causality, a_name) => {
            let mut txt = (*txt).clone();
            txt = dumpFMUModelDescriptionInputOutputVariable(txt.clone(), (a_name.clone()).clone(), (a_causality.clone()).clone(), (a_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (a_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), a_x1Placement.clone(), a_x2Placement.clone(), a_y1Placement.clone(), a_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (a_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_297(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_298(mut in_txt: Tpl::Text, mut in_a_isInputOrOutput: Tpl::Text, mut in_a_y2Placement: i32, mut in_a_y1Placement: i32, mut in_a_x2Placement: i32, mut in_a_x1Placement: i32, mut in_a_description: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isInputOrOutput.clone(), in_a_y2Placement.clone(), in_a_y1Placement.clone(), in_a_x2Placement.clone(), in_a_x1Placement.clone(), in_a_description.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_name.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, _, _, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_y2Placement, a_y1Placement, a_x2Placement, a_x1Placement, a_description, a_generateOutputConnectors, a_generateInputConnectors, a_baseType, a_causality, a_name) => {
            let mut txt = (*txt).clone();
            txt = dumpFMUModelDescriptionInputOutputVariable(txt.clone(), (a_name.clone()).clone(), (a_causality.clone()).clone(), (a_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (a_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), a_x1Placement.clone(), a_x2Placement.clone(), a_y1Placement.clone(), a_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (a_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_299(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_300(mut in_txt: Tpl::Text, mut in_a_isInputOrOutput: Tpl::Text, mut in_a_y2Placement: i32, mut in_a_y1Placement: i32, mut in_a_x2Placement: i32, mut in_a_x1Placement: i32, mut in_a_description: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isInputOrOutput.clone(), in_a_y2Placement.clone(), in_a_y1Placement.clone(), in_a_x2Placement.clone(), in_a_x1Placement.clone(), in_a_description.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_name.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, _, _, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_y2Placement, a_y1Placement, a_x2Placement, a_x1Placement, a_description, a_generateOutputConnectors, a_generateInputConnectors, a_baseType, a_causality, a_name) => {
            let mut txt = (*txt).clone();
            txt = dumpFMUModelDescriptionInputOutputVariable(txt.clone(), (a_name.clone()).clone(), (a_causality.clone()).clone(), (a_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (a_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), a_x1Placement.clone(), a_x2Placement.clone(), a_y1Placement.clone(), a_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (a_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_301(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_302(mut in_txt: Tpl::Text, mut in_a_isInputOrOutput: Tpl::Text, mut in_a_y2Placement: i32, mut in_a_y1Placement: i32, mut in_a_x2Placement: i32, mut in_a_x1Placement: i32, mut in_a_description: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isInputOrOutput.clone(), in_a_y2Placement.clone(), in_a_y1Placement.clone(), in_a_x2Placement.clone(), in_a_x1Placement.clone(), in_a_description.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_name.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, _, _, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_y2Placement, a_y1Placement, a_x2Placement, a_x1Placement, a_description, a_generateOutputConnectors, a_generateInputConnectors, a_baseType, a_causality, a_name) => {
            let mut txt = (*txt).clone();
            txt = dumpFMUModelDescriptionInputOutputVariable(txt.clone(), (a_name.clone()).clone(), (a_causality.clone()).clone(), (a_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (a_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), a_x1Placement.clone(), a_x2Placement.clone(), a_y1Placement.clone(), a_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (a_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_303(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_304(mut in_txt: Tpl::Text, mut in_a_isInputOrOutput: Tpl::Text, mut in_a_y2Placement: i32, mut in_a_y1Placement: i32, mut in_a_x2Placement: i32, mut in_a_x1Placement: i32, mut in_a_description: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_isInputOrOutput.clone(), in_a_y2Placement.clone(), in_a_y1Placement.clone(), in_a_x2Placement.clone(), in_a_x1Placement.clone(), in_a_description.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_name.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _, _, _, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_y2Placement, a_y1Placement, a_x2Placement, a_x1Placement, a_description, a_generateOutputConnectors, a_generateInputConnectors, a_baseType, a_causality, a_name) => {
            let mut txt = (*txt).clone();
            txt = dumpFMUModelDescriptionInputOutputVariable(txt.clone(), (a_name.clone()).clone(), (a_causality.clone()).clone(), (a_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (a_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), a_x1Placement.clone(), a_x2Placement.clone(), a_y1Placement.clone(), a_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (a_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_305(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone()) {
        (mut txt, FMI::ModelVariables::REALVARIABLE { y2Placement: mut i_y2Placement, y1Placement: mut i_y1Placement, x2Placement: mut i_x2Placement, x1Placement: mut i_x1Placement, description: mut i_description, baseType: mut i_baseType, name: mut i_name, causality: mut i_causality, .. }, mut a_generateOutputConnectors, mut a_generateInputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut l_isInputOrOutput: Tpl::Text;
            ret_1 = stringEq((i_causality.clone()).clone(), (literal!("input")).clone());
            ret_2 = stringEq((i_causality.clone()).clone(), (literal!("output")).clone());
            ret_3 = boolOr(ret_1.clone(), ret_2.clone());
            l_isInputOrOutput = fun_295(Tpl::emptyTxt.clone(), ret_3.clone())?;
            txt = fun_296(txt.clone(), l_isInputOrOutput.clone(), i_y2Placement.clone(), i_y1Placement.clone(), i_x2Placement.clone(), i_x1Placement.clone(), (i_description.clone()).clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), (i_baseType.clone()).clone(), (i_causality.clone()).clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, FMI::ModelVariables::INTEGERVARIABLE { y2Placement: mut i_y2Placement, y1Placement: mut i_y1Placement, x2Placement: mut i_x2Placement, x1Placement: mut i_x1Placement, description: mut i_description, baseType: mut i_baseType, name: mut i_name, causality: mut i_causality, .. }, mut a_generateOutputConnectors, mut a_generateInputConnectors) => {
            let mut ret_6: bool = false;
            let mut ret_5: bool = false;
            let mut ret_4: bool = false;
            let mut l_isInputOrOutput: Tpl::Text;
            ret_4 = stringEq((i_causality.clone()).clone(), (literal!("input")).clone());
            ret_5 = stringEq((i_causality.clone()).clone(), (literal!("output")).clone());
            ret_6 = boolOr(ret_4.clone(), ret_5.clone());
            l_isInputOrOutput = fun_297(Tpl::emptyTxt.clone(), ret_6.clone())?;
            txt = fun_298(txt.clone(), l_isInputOrOutput.clone(), i_y2Placement.clone(), i_y1Placement.clone(), i_x2Placement.clone(), i_x1Placement.clone(), (i_description.clone()).clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), (i_baseType.clone()).clone(), (i_causality.clone()).clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, FMI::ModelVariables::BOOLEANVARIABLE { y2Placement: mut i_y2Placement, y1Placement: mut i_y1Placement, x2Placement: mut i_x2Placement, x1Placement: mut i_x1Placement, description: mut i_description, baseType: mut i_baseType, name: mut i_name, causality: mut i_causality, .. }, mut a_generateOutputConnectors, mut a_generateInputConnectors) => {
            let mut ret_9: bool = false;
            let mut ret_8: bool = false;
            let mut ret_7: bool = false;
            let mut l_isInputOrOutput: Tpl::Text;
            ret_7 = stringEq((i_causality.clone()).clone(), (literal!("input")).clone());
            ret_8 = stringEq((i_causality.clone()).clone(), (literal!("output")).clone());
            ret_9 = boolOr(ret_7.clone(), ret_8.clone());
            l_isInputOrOutput = fun_299(Tpl::emptyTxt.clone(), ret_9.clone())?;
            txt = fun_300(txt.clone(), l_isInputOrOutput.clone(), i_y2Placement.clone(), i_y1Placement.clone(), i_x2Placement.clone(), i_x1Placement.clone(), (i_description.clone()).clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), (i_baseType.clone()).clone(), (i_causality.clone()).clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, FMI::ModelVariables::STRINGVARIABLE { y2Placement: mut i_y2Placement, y1Placement: mut i_y1Placement, x2Placement: mut i_x2Placement, x1Placement: mut i_x1Placement, description: mut i_description, baseType: mut i_baseType, name: mut i_name, causality: mut i_causality, .. }, mut a_generateOutputConnectors, mut a_generateInputConnectors) => {
            let mut ret_12: bool = false;
            let mut ret_11: bool = false;
            let mut ret_10: bool = false;
            let mut l_isInputOrOutput: Tpl::Text;
            ret_10 = stringEq((i_causality.clone()).clone(), (literal!("input")).clone());
            ret_11 = stringEq((i_causality.clone()).clone(), (literal!("output")).clone());
            ret_12 = boolOr(ret_10.clone(), ret_11.clone());
            l_isInputOrOutput = fun_301(Tpl::emptyTxt.clone(), ret_12.clone())?;
            txt = fun_302(txt.clone(), l_isInputOrOutput.clone(), i_y2Placement.clone(), i_y1Placement.clone(), i_x2Placement.clone(), i_x1Placement.clone(), (i_description.clone()).clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), (i_baseType.clone()).clone(), (i_causality.clone()).clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, FMI::ModelVariables::ENUMERATIONVARIABLE { y2Placement: mut i_y2Placement, y1Placement: mut i_y1Placement, x2Placement: mut i_x2Placement, x1Placement: mut i_x1Placement, description: mut i_description, baseType: mut i_baseType, name: mut i_name, causality: mut i_causality, .. }, mut a_generateOutputConnectors, mut a_generateInputConnectors) => {
            let mut ret_15: bool = false;
            let mut ret_14: bool = false;
            let mut ret_13: bool = false;
            let mut l_isInputOrOutput: Tpl::Text;
            ret_13 = stringEq((i_causality.clone()).clone(), (literal!("input")).clone());
            ret_14 = stringEq((i_causality.clone()).clone(), (literal!("output")).clone());
            ret_15 = boolOr(ret_13.clone(), ret_14.clone());
            l_isInputOrOutput = fun_303(Tpl::emptyTxt.clone(), ret_15.clone())?;
            txt = fun_304(txt.clone(), l_isInputOrOutput.clone(), i_y2Placement.clone(), i_y1Placement.clone(), i_x2Placement.clone(), i_x1Placement.clone(), (i_description.clone()).clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), (i_baseType.clone()).clone(), (i_causality.clone()).clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_306(mut in_txt: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_generateInputConnectors: bool, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_FMUVersion.clone(), in_a_fmiModelVariable.clone(), in_a_generateInputConnectors.clone(), in_a_generateOutputConnectors.clone())) {
        (txt, Deref @ "1.0", a_fmiModelVariable, a_generateInputConnectors, a_generateOutputConnectors) => {
            let mut txt = (*txt).clone();
            txt = fun_305(txt.clone(), a_fmiModelVariable.clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFMUModelDescriptionVariable(mut txt: Tpl::Text, mut a_FMUVersion: ArcStr, mut a_fmiModelVariable: FMI::ModelVariables, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut a_generateInputConnectors: bool, mut a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_306(txt.clone(), (a_FMUVersion.clone()).clone(), a_fmiModelVariable.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
    Ok(out_txt)
}

fn fun_308(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.BooleanOutput ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_309(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_name, mut a_baseType, mut a_causality, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("output")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Boolean")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateOutputConnectors.clone(), ret_2.clone());
            txt = fun_308(txt.clone(), ret_3.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.IntegerOutput ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_310(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_name, mut a_baseType, mut a_causality, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("output")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Integer")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateOutputConnectors.clone(), ret_2.clone());
            txt = fun_309(txt.clone(), ret_3.clone(), (a_name.clone()).clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateOutputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.RealOutput ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_311(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_name, mut a_baseType, mut a_causality, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("output")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Real")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateOutputConnectors.clone(), ret_2.clone());
            txt = fun_310(txt.clone(), ret_3.clone(), (a_name.clone()).clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateOutputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.BooleanInput ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_312(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateInputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_generateOutputConnectors.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateInputConnectors.clone()) {
        (mut txt, false, mut a_name, mut a_generateOutputConnectors, mut a_baseType, mut a_causality, mut a_generateInputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("input")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Boolean")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateInputConnectors.clone(), ret_2.clone());
            txt = fun_311(txt.clone(), ret_3.clone(), (a_name.clone()).clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateOutputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.IntegerInput ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_313(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_causality: ArcStr, mut in_a_baseType: ArcStr, mut in_a_generateInputConnectors: bool, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_causality.clone(), in_a_baseType.clone(), in_a_generateInputConnectors.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_name, mut a_causality, mut a_baseType, mut a_generateInputConnectors, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("input")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Integer")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateInputConnectors.clone(), ret_2.clone());
            txt = fun_312(txt.clone(), ret_3.clone(), (a_name.clone()).clone(), a_generateOutputConnectors.clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateInputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.RealInput ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMUModelDescriptionInputOutputVariable(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_causality: ArcStr, mut a_baseType: ArcStr, mut a_generateInputConnectors: bool, mut a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_3: bool = false;
    let mut ret_2: bool = false;
    let mut ret_1: bool = false;
    let mut ret_0: bool = false;
    ret_0 = stringEq((a_causality.clone()).clone(), (literal!("input")).clone());
    ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Real")).clone());
    ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
    ret_3 = boolAnd(a_generateInputConnectors.clone(), ret_2.clone());
    out_txt = fun_313(txt.clone(), ret_3.clone(), (a_name.clone()).clone(), (a_causality.clone()).clone(), (a_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
    Ok(out_txt)
}

fn fun_315(mut in_txt: Tpl::Text, mut in_a_fmiInfo: FMI::Info, mut in_a_name: ArcStr, mut in_a_fmi: FMI::FmiImport) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiInfo.clone(), in_a_name.clone(), in_a_fmi.clone())) {
        (txt, FMI::Info { fmiType: 0, fmiVersion: Deref @ "1.0", .. }, a_name, a_fmi) => {
            let mut txt = (*txt).clone();
            txt = importFMU1ModelExchange(txt.clone(), a_fmi.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (txt, FMI::Info { fmiType: 1, fmiVersion: Deref @ "1.0", .. }, a_name, a_fmi) => {
            let mut txt = (*txt).clone();
            txt = importFMU1CoSimulationStandAlone(txt.clone(), a_fmi.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (txt, FMI::Info { fmiType: 1, fmiVersion: Deref @ "2.0", .. }, a_name, a_fmi) => {
            let mut txt = (*txt).clone();
            txt = importFMU2ModelExchange(txt.clone(), a_fmi.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn importFMUModelica(mut in_txt: Tpl::Text, mut in_a_fmi: FMI::FmiImport, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmi.clone(), in_a_name.clone()) {
        (mut txt, ref i_fmi @ FMI::FmiImport { fmiInfo: ref i_fmiInfo, .. }, mut a_name) => {
            txt = fun_315(txt.clone(), i_fmiInfo.clone(), (a_name.clone()).clone(), i_fmi.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_317(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo: FMI::Info, mut in_a_fmiInfo_fmiModelIdentifier: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo.clone(), in_a_fmiInfo_fmiModelIdentifier.clone(), in_a_name.clone()) {
        (mut txt, false, _, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiInfo, mut a_fmiInfo_fmiModelIdentifier, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_0 = (FMI::getFMIType(a_fmiInfo.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_318(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiDescription: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiDescription.clone()) {
        (mut txt, false, mut a_fmiInfo_fmiDescription) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_319(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_realInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_320(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_integerInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_321(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_booleanInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_322(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_stringInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_323(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi_x = fmi1Functions.fmi1GetContinuousStates(fmi1me, numberOfContinuousStates, flowParamsStart+flowInitialized);")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_324(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realParametersNames: Tpl::Text, mut in_a_realParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realParametersNames.clone(), in_a_realParametersVRs.clone()) {
        (mut txt, false, mut a_realParametersNames, mut a_realParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi1Functions.fmi1SetRealParameter(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_325(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerParametersNames: Tpl::Text, mut in_a_integerParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerParametersNames.clone(), in_a_integerParametersVRs.clone()) {
        (mut txt, false, mut a_integerParametersNames, mut a_integerParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi1Functions.fmi1SetIntegerParameter(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_326(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanParametersNames: Tpl::Text, mut in_a_booleanParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanParametersNames.clone(), in_a_booleanParametersVRs.clone()) {
        (mut txt, false, mut a_booleanParametersNames, mut a_booleanParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi1Functions.fmi1SetBooleanParameter(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_327(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringParametersNames: Tpl::Text, mut in_a_stringParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringParametersNames.clone(), in_a_stringParametersVRs.clone()) {
        (mut txt, false, mut a_stringParametersNames, mut a_stringParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi1Functions.fmi1SetStringParameter(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_328(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realDependentParametersVRs: Tpl::Text, mut in_a_realDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realDependentParametersVRs.clone(), in_a_realDependentParametersNames.clone()) {
        (mut txt, false, mut a_realDependentParametersVRs, mut a_realDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetReal(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_329(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerDependentParametersVRs: Tpl::Text, mut in_a_integerDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerDependentParametersVRs.clone(), in_a_integerDependentParametersNames.clone()) {
        (mut txt, false, mut a_integerDependentParametersVRs, mut a_integerDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetInteger(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_330(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanDependentParametersVRs: Tpl::Text, mut in_a_booleanDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanDependentParametersVRs.clone(), in_a_booleanDependentParametersNames.clone()) {
        (mut txt, false, mut a_booleanDependentParametersVRs, mut a_booleanDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetBoolean(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_331(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringDependentParametersVRs: Tpl::Text, mut in_a_stringDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringDependentParametersVRs.clone(), in_a_stringDependentParametersNames.clone()) {
        (mut txt, false, mut a_stringDependentParametersVRs, mut a_stringDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetString(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_332(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realInputVariablesNames: Tpl::Text, mut in_a_realInputVariablesVRs: Tpl::Text, mut in_a_realInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realInputVariablesNames.clone(), in_a_realInputVariablesVRs.clone(), in_a_realInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_realInputVariablesNames, mut a_realInputVariablesVRs, mut a_realInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetReal(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_333(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerInputVariablesNames: Tpl::Text, mut in_a_integerInputVariablesVRs: Tpl::Text, mut in_a_integerInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerInputVariablesNames.clone(), in_a_integerInputVariablesVRs.clone(), in_a_integerInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_integerInputVariablesNames, mut a_integerInputVariablesVRs, mut a_integerInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetInteger(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_334(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanInputVariablesNames: Tpl::Text, mut in_a_booleanInputVariablesVRs: Tpl::Text, mut in_a_booleanInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanInputVariablesNames.clone(), in_a_booleanInputVariablesVRs.clone(), in_a_booleanInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_booleanInputVariablesNames, mut a_booleanInputVariablesVRs, mut a_booleanInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetBoolean(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_335(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringStartVariablesNames: Tpl::Text, mut in_a_stringInputVariablesVRs: Tpl::Text, mut in_a_stringInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringStartVariablesNames.clone(), in_a_stringInputVariablesVRs.clone(), in_a_stringInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_stringStartVariablesNames, mut a_stringInputVariablesVRs, mut a_stringInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetString(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringStartVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_336(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realOutputVariablesVRs: Tpl::Text, mut in_a_realOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realOutputVariablesVRs.clone(), in_a_realOutputVariablesNames.clone()) {
        (mut txt, false, mut a_realOutputVariablesVRs, mut a_realOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetReal(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_337(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerOutputVariablesVRs: Tpl::Text, mut in_a_integerOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerOutputVariablesVRs.clone(), in_a_integerOutputVariablesNames.clone()) {
        (mut txt, false, mut a_integerOutputVariablesVRs, mut a_integerOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetInteger(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_338(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanOutputVariablesVRs: Tpl::Text, mut in_a_booleanOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanOutputVariablesVRs.clone(), in_a_booleanOutputVariablesNames.clone()) {
        (mut txt, false, mut a_booleanOutputVariablesVRs, mut a_booleanOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetBoolean(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_339(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringOutputVariablesVRs: Tpl::Text, mut in_a_stringOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringOutputVariablesVRs.clone(), in_a_stringOutputVariablesNames.clone()) {
        (mut txt, false, mut a_stringOutputVariablesVRs, mut a_stringOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetString(fmi1me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_340(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eventIndicator, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("change(fmi_z_positive[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_eventIndicator.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("])")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_340(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_341(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiNumberOfEventIndicators: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiNumberOfEventIndicators.clone())) {
        (txt, false, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  when {triggerDSSEvent > flowStatesInputs, nextEventTime < time, terminal()} then")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_fmiInfo_fmiNumberOfEventIndicators) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when {")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" or ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_340(txt.clone(), a_fmiInfo_fmiNumberOfEventIndicators.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", triggerDSSEvent > flowStatesInputs, nextEventTime < time, terminal()} then")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_342(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_continuousStates, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(fmi_x[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_continuousStates.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("], fmi_x_new[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_continuousStates.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]);")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_342(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_343(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiNumberOfContinuousStates: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiNumberOfContinuousStates.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_fmiInfo_fmiNumberOfContinuousStates) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    if newStatesAvailable then\n")).clone(), (literal!("      fmi_x_new := fmi1Functions.fmi1GetContinuousStates(fmi1me, numberOfContinuousStates, flowStatesInputs);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_342(txt.clone(), a_fmiInfo_fmiNumberOfContinuousStates.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("    end if;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_344(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo: FMI::Info, mut in_a_fmiInfo_fmiModelIdentifier: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo.clone(), in_a_fmiInfo_fmiModelIdentifier.clone(), in_a_name.clone()) {
        (mut txt, false, _, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiInfo, mut a_fmiInfo_fmiModelIdentifier, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_0 = (FMI::getFMIType(a_fmiInfo.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn importFMU1ModelExchange(mut in_txt: Tpl::Text, mut in_a_fmi: FMI::FmiImport, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmi.clone(), in_a_name.clone()) {
        (mut txt, FMI::FmiImport { generateOutputConnectors: mut i_generateOutputConnectors, generateInputConnectors: mut i_generateInputConnectors, fmiDebugOutput: mut i_fmiDebugOutput, fmiLogLevel: mut i_fmiLogLevel, fmuWorkingDirectory: mut i_fmuWorkingDirectory, fmiTypeDefinitionsList: ref i_fmiTypeDefinitionsList, fmiModelVariablesList: ref i_fmiModelVariablesList, fmiExperimentAnnotation: FMI::ExperimentAnnotation { fmiExperimentTolerance: mut i_fmiExperimentAnnotation_fmiExperimentTolerance, fmiExperimentStopTime: mut i_fmiExperimentAnnotation_fmiExperimentStopTime, fmiExperimentStartTime: mut i_fmiExperimentAnnotation_fmiExperimentStartTime }, fmiInfo: ref i_fmiInfo @ FMI::Info { fmiNumberOfEventIndicators: ref i_fmiInfo_fmiNumberOfEventIndicators, fmiNumberOfContinuousStates: ref i_fmiInfo_fmiNumberOfContinuousStates, fmiDescription: ref i_fmiInfo_fmiDescription, fmiModelIdentifier: ref i_fmiInfo_fmiModelIdentifier, .. }, .. }, mut a_name) => {
            let mut ret_74: bool = false;
            let mut ret_73: bool = false;
            let mut ret_72: i32 = 0;
            let mut ret_71: bool = false;
            let mut ret_70: i32 = 0;
            let mut ret_69: bool = false;
            let mut ret_68: bool = false;
            let mut ret_67: bool = false;
            let mut ret_66: bool = false;
            let mut ret_65: bool = false;
            let mut ret_64: bool = false;
            let mut ret_63: bool = false;
            let mut ret_62: bool = false;
            let mut ret_61: bool = false;
            let mut ret_60: bool = false;
            let mut ret_59: bool = false;
            let mut ret_58: bool = false;
            let mut ret_57: bool = false;
            let mut ret_56: bool = false;
            let mut ret_55: bool = false;
            let mut ret_54: bool = false;
            let mut ret_53: bool = false;
            let mut ret_52: bool = false;
            let mut ret_51: bool = false;
            let mut ret_50: bool = false;
            let mut ret_49: bool = false;
            let mut ret_48: bool = false;
            let mut ret_47: bool = false;
            let mut ret_46: bool = false;
            let mut ret_45: bool = false;
            let mut ret_44: i32 = 0;
            let mut ret_43: bool = false;
            let mut ret_42: bool = false;
            let mut ret_41: bool = false;
            let mut ret_40: bool = false;
            let mut ret_39: i32 = 0;
            let mut ret_38: i32 = 0;
            let mut ret_37: bool = false;
            let mut ret_36: bool = false;
            let mut l_stringOutputVariablesNames: Tpl::Text;
            let mut l_stringOutputVariablesVRs: Tpl::Text;
            let mut l_booleanOutputVariablesNames: Tpl::Text;
            let mut l_booleanOutputVariablesVRs: Tpl::Text;
            let mut l_integerOutputVariablesNames: Tpl::Text;
            let mut l_integerOutputVariablesVRs: Tpl::Text;
            let mut l_realOutputVariablesNames: Tpl::Text;
            let mut l_realOutputVariablesVRs: Tpl::Text;
            let mut l_stringInputVariablesReturnNames: Tpl::Text;
            let mut l_stringStartVariablesNames: Tpl::Text;
            let mut l_stringInputVariablesVRs: Tpl::Text;
            let mut l_booleanInputVariablesReturnNames: Tpl::Text;
            let mut l_booleanInputVariablesNames: Tpl::Text;
            let mut l_booleanInputVariablesVRs: Tpl::Text;
            let mut l_integerInputVariablesReturnNames: Tpl::Text;
            let mut l_integerInputVariablesNames: Tpl::Text;
            let mut l_integerInputVariablesVRs: Tpl::Text;
            let mut l_realInputVariablesReturnNames: Tpl::Text;
            let mut l_realInputVariablesNames: Tpl::Text;
            let mut l_realInputVariablesVRs: Tpl::Text;
            let mut l_stringDependentParametersNames: Tpl::Text;
            let mut l_stringDependentParametersVRs: Tpl::Text;
            let mut l_booleanDependentParametersNames: Tpl::Text;
            let mut l_booleanDependentParametersVRs: Tpl::Text;
            let mut l_integerDependentParametersNames: Tpl::Text;
            let mut l_integerDependentParametersVRs: Tpl::Text;
            let mut l_realDependentParametersNames: Tpl::Text;
            let mut l_realDependentParametersVRs: Tpl::Text;
            let mut l_stringParametersNames: Tpl::Text;
            let mut l_stringParametersVRs: Tpl::Text;
            let mut l_booleanParametersNames: Tpl::Text;
            let mut l_booleanParametersVRs: Tpl::Text;
            let mut l_integerParametersNames: Tpl::Text;
            let mut l_integerParametersVRs: Tpl::Text;
            let mut l_realParametersNames: Tpl::Text;
            let mut l_realParametersVRs: Tpl::Text;
            l_realParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_realParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_integerParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_integerParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_booleanParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_booleanParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_stringParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_stringParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_realDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_realDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_integerDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_integerDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_booleanDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_booleanDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_stringDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_stringDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_realInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_realInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_realInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_integerInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_integerInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_integerInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_booleanInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_booleanInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_booleanInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_stringInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_stringStartVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_stringInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_realOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_realOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_integerOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_integerOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_booleanOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_booleanOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_stringOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_stringOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("model ")).clone() }))?;
            ret_36 = stringEq((a_name.clone()).clone(), (literal!("")).clone());
            txt = fun_317(txt.clone(), ret_36.clone(), i_fmiInfo.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone(), (a_name.clone()).clone())?;
            ret_37 = stringEq((i_fmiInfo_fmiDescription.clone()).clone(), (literal!("")).clone());
            txt = fun_318(txt.clone(), ret_37.clone(), (i_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant String fmuWorkingDir = \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuWorkingDirectory.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("parameter Integer logLevel = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_fmiLogLevel.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" \"log level used during the loading of FMU\" annotation (Dialog(tab=\"FMI\", group=\"Enable logging\"));\n")).clone(), (literal!("parameter Boolean debugLogging = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_fmiDebugOutput.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" \"enables the FMU simulation logging\" annotation (Dialog(tab=\"FMI\", group=\"Enable logging\"));\n")).clone() }))?;
            txt = dumpFMIModelVariablesList(txt.clone(), (literal!("1.0")).clone(), i_fmiModelVariablesList.clone(), i_fmiTypeDefinitionsList.clone(), i_generateInputConnectors.clone(), i_generateOutputConnectors.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("protected\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMI1ModelExchange fmi1me = FMI1ModelExchange(logLevel, fmuWorkingDir, \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\", debugLogging);\n")).clone(), (literal!("constant Integer numberOfContinuousStates = ")).clone()], lastHasNewLine: false }))?;
            ret_38 = (i_fmiInfo_fmiNumberOfContinuousStates.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_38.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("Real fmi_x[numberOfContinuousStates] \"States\";\n")).clone(), (literal!("Real fmi_x_new[numberOfContinuousStates](each fixed = true) \"New States\";\n")).clone(), (literal!("constant Integer numberOfEventIndicators = ")).clone()], lastHasNewLine: false }))?;
            ret_39 = (i_fmiInfo_fmiNumberOfEventIndicators.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_39.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("Real fmi_z[numberOfEventIndicators] \"Events Indicators\";\n")).clone(), (literal!("Boolean fmi_z_positive[numberOfEventIndicators](each fixed = true);\n")).clone(), (literal!("parameter Real flowStartTime(fixed=false);\n")).clone(), (literal!("Real flowTime;\n")).clone(), (literal!("parameter Real flowInitialized(fixed=false);\n")).clone(), (literal!("parameter Real flowParamsStart(fixed=false);\n")).clone(), (literal!("parameter Real flowInitInputs(fixed=false);\n")).clone(), (literal!("Real flowStatesInputs;\n")).clone()], lastHasNewLine: true }))?;
            ret_40 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_319(txt.clone(), ret_40.clone(), l_realInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_41 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_320(txt.clone(), ret_41.clone(), l_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_42 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_321(txt.clone(), ret_42.clone(), l_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_43 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_322(txt.clone(), ret_43.clone(), l_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Boolean callEventUpdate;\n")).clone(), (literal!("constant Boolean intermediateResults = false;\n")).clone(), (literal!("Boolean newStatesAvailable(fixed = true);\n")).clone(), (literal!("Real triggerDSSEvent;\n")).clone(), (literal!("Real nextEventTime;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("initial equation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_44 = (i_fmiInfo_fmiNumberOfContinuousStates.clone().len() as i32);
            ret_45 = intGt(ret_44.clone(), 0);
            txt = fun_323(txt.clone(), ret_45.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("initial algorithm\n")).clone(), (literal!("  flowParamsStart := 1;\n")).clone(), (literal!("  flowStartTime := fmi1Functions.fmi1SetTime(fmi1me, time, 1);\n")).clone(), (literal!("  flowInitialized := fmi1Functions.fmi1Initialize(fmi1me, flowParamsStart+flowInitInputs+flowStartTime);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_46 = stringEq((Tpl::textString(l_realParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_324(txt.clone(), ret_46.clone(), l_realParametersNames.clone(), l_realParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_47 = stringEq((Tpl::textString(l_integerParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_325(txt.clone(), ret_47.clone(), l_integerParametersNames.clone(), l_integerParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_48 = stringEq((Tpl::textString(l_booleanParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_326(txt.clone(), ret_48.clone(), l_booleanParametersNames.clone(), l_booleanParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_49 = stringEq((Tpl::textString(l_stringParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_327(txt.clone(), ret_49.clone(), l_stringParametersNames.clone(), l_stringParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("flowInitInputs := 1;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("initial equation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_50 = stringEq((Tpl::textString(l_realDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_328(txt.clone(), ret_50.clone(), l_realDependentParametersVRs.clone(), l_realDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_51 = stringEq((Tpl::textString(l_integerDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_329(txt.clone(), ret_51.clone(), l_integerDependentParametersVRs.clone(), l_integerDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_52 = stringEq((Tpl::textString(l_booleanDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_330(txt.clone(), ret_52.clone(), l_booleanDependentParametersVRs.clone(), l_booleanDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_53 = stringEq((Tpl::textString(l_stringDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_331(txt.clone(), ret_53.clone(), l_stringDependentParametersVRs.clone(), l_stringDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("equation\n")).clone(), (literal!("  flowTime = fmi1Functions.fmi1SetTime(fmi1me, time, flowInitialized);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_54 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_332(txt.clone(), ret_54.clone(), l_realInputVariablesNames.clone(), l_realInputVariablesVRs.clone(), l_realInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_55 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_333(txt.clone(), ret_55.clone(), l_integerInputVariablesNames.clone(), l_integerInputVariablesVRs.clone(), l_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_56 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_334(txt.clone(), ret_56.clone(), l_booleanInputVariablesNames.clone(), l_booleanInputVariablesVRs.clone(), l_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_57 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_335(txt.clone(), ret_57.clone(), l_stringStartVariablesNames.clone(), l_stringInputVariablesVRs.clone(), l_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("flowStatesInputs = fmi1Functions.fmi1SetContinuousStates(fmi1me, fmi_x, flowParamsStart + flowTime);\n")).clone(), (literal!("der(fmi_x) = fmi1Functions.fmi1GetDerivatives(fmi1me, numberOfContinuousStates, flowStatesInputs);\n")).clone(), (literal!("fmi_z  = fmi1Functions.fmi1GetEventIndicators(fmi1me, numberOfEventIndicators, flowStatesInputs);\n")).clone(), (literal!("for i in 1:size(fmi_z,1) loop\n")).clone(), (literal!("  fmi_z_positive[i] = if not terminal() then fmi_z[i] > 0 else pre(fmi_z_positive[i]);\n")).clone(), (literal!("end for;\n")).clone(), (literal!("callEventUpdate = fmi1Functions.fmi1CompletedIntegratorStep(fmi1me, flowStatesInputs);\n")).clone(), (literal!("triggerDSSEvent = noEvent(if callEventUpdate then flowStatesInputs+1.0 else flowStatesInputs-1.0);\n")).clone(), (literal!("nextEventTime = fmi1Functions.fmi1nextEventTime(fmi1me, flowStatesInputs);\n")).clone()], lastHasNewLine: true }))?;
            ret_58 = stringEq((Tpl::textString(l_realOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_59 = stringEq((Tpl::textString(l_realOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_60 = boolAnd(ret_58.clone(), ret_59.clone());
            txt = fun_336(txt.clone(), ret_60.clone(), l_realOutputVariablesVRs.clone(), l_realOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_61 = stringEq((Tpl::textString(l_integerOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_62 = stringEq((Tpl::textString(l_integerOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_63 = boolAnd(ret_61.clone(), ret_62.clone());
            txt = fun_337(txt.clone(), ret_63.clone(), l_integerOutputVariablesVRs.clone(), l_integerOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_64 = stringEq((Tpl::textString(l_booleanOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_65 = stringEq((Tpl::textString(l_booleanOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_66 = boolAnd(ret_64.clone(), ret_65.clone());
            txt = fun_338(txt.clone(), ret_66.clone(), l_booleanOutputVariablesVRs.clone(), l_booleanOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_67 = stringEq((Tpl::textString(l_stringOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_68 = stringEq((Tpl::textString(l_stringOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_69 = boolAnd(ret_67.clone(), ret_68.clone());
            txt = fun_339(txt.clone(), ret_69.clone(), l_stringOutputVariablesVRs.clone(), l_stringOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = dumpOutputGetEnumerationVariables(txt.clone(), i_fmiModelVariablesList.clone(), i_fmiTypeDefinitionsList.clone(), (literal!("fmi1Functions.fmi1GetInteger")).clone(), (literal!("fmi1me")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("algorithm\n")).clone() }))?;
            ret_70 = (i_fmiInfo_fmiNumberOfEventIndicators.clone().len() as i32);
            ret_71 = intGt(ret_70.clone(), 0);
            txt = fun_341(txt.clone(), ret_71.clone(), i_fmiInfo_fmiNumberOfEventIndicators.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    newStatesAvailable := fmi1Functions.fmi1EventUpdate(fmi1me, intermediateResults);\n")).clone() }))?;
            ret_72 = (i_fmiInfo_fmiNumberOfContinuousStates.clone().len() as i32);
            ret_73 = intGt(ret_72.clone(), 0);
            txt = fun_343(txt.clone(), ret_73.clone(), i_fmiInfo_fmiNumberOfContinuousStates.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  end when;\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("annotation(experiment(StartTime=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStartTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", StopTime=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", Tolerance=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentTolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("));\n")).clone(), (literal!("annotation (Icon(graphics={\n")).clone(), (literal!("    Rectangle(\n")).clone(), (literal!("      extent={{-100,100},{100,-100}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      fillColor={240,240,240},\n")).clone(), (literal!("      fillPattern=FillPattern.Solid,\n")).clone(), (literal!("      lineThickness=0.5),\n")).clone(), (literal!("    Text(\n")).clone(), (literal!("      extent={{-100,40},{100,0}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      textString=\"%name\"),\n")).clone(), (literal!("    Text(\n")).clone(), (literal!("      extent={{-100,-50},{100,-90}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      textString=\"V1.0\")}));\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("protected\n")).clone(), (literal!("  class FMI1ModelExchange\n")).clone(), (literal!("    extends ExternalObject;\n")).clone(), (literal!("      function constructor\n")).clone(), (literal!("        input Integer logLevel;\n")).clone(), (literal!("        input String workingDirectory;\n")).clone(), (literal!("        input String instanceName;\n")).clone(), (literal!("        input Boolean debugLogging;\n")).clone(), (literal!("        output FMI1ModelExchange fmi1me;\n")).clone(), (literal!("        external \"C\" fmi1me = FMI1ModelExchangeConstructor_OMC(logLevel, workingDirectory, instanceName, debugLogging) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("      end constructor;\n")).clone(), (literal!("\n")).clone(), (literal!("      function destructor\n")).clone(), (literal!("        input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("        external \"C\" FMI1ModelExchangeDestructor_OMC(fmi1me) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("      end destructor;\n")).clone(), (literal!("  end FMI1ModelExchange;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitionsMappingFunctions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = dumpFMITypeDefinitionsArrayMappingFunctions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("package fmi1Functions\n")).clone(), (literal!("  function fmi1Initialize\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real preInitialized;\n")).clone(), (literal!("    output Real postInitialized=preInitialized;\n")).clone(), (literal!("    external \"C\" fmi1Initialize_OMC(fmi1me) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1Initialize;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetTime\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real inTime;\n")).clone(), (literal!("    input Real inFlow;\n")).clone(), (literal!("    output Real outFlow = inFlow;\n")).clone(), (literal!("    external \"C\" fmi1SetTime_OMC(fmi1me, inTime) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetTime;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetContinuousStates\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Integer numberOfContinuousStates;\n")).clone(), (literal!("    input Real inFlowParams;\n")).clone(), (literal!("    output Real fmi_x[numberOfContinuousStates];\n")).clone(), (literal!("    external \"C\" fmi1GetContinuousStates_OMC(fmi1me, numberOfContinuousStates, inFlowParams, fmi_x) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetContinuousStates;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetContinuousStates\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real fmi_x[:];\n")).clone(), (literal!("    input Real inFlowParams;\n")).clone(), (literal!("    output Real outFlowStates;\n")).clone(), (literal!("    external \"C\" outFlowStates = fmi1SetContinuousStates_OMC(fmi1me, size(fmi_x, 1), inFlowParams, fmi_x) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetContinuousStates;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetDerivatives\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Integer numberOfContinuousStates;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Real fmi_x[numberOfContinuousStates];\n")).clone(), (literal!("    external \"C\" fmi1GetDerivatives_OMC(fmi1me, numberOfContinuousStates, inFlowStates, fmi_x) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetDerivatives;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetEventIndicators\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Integer numberOfEventIndicators;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Real fmi_z[numberOfEventIndicators];\n")).clone(), (literal!("    external \"C\" fmi1GetEventIndicators_OMC(fmi1me, numberOfEventIndicators, inFlowStates, fmi_z) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetEventIndicators;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetReal\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real realValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Real realValues[size(realValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetReal_OMC(fmi1me, size(realValuesReferences, 1), realValuesReferences, inFlowStatesInput, realValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetReal;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetReal\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real realValueReferences[:];\n")).clone(), (literal!("    input Real realValues[size(realValueReferences, 1)];\n")).clone(), (literal!("    output Real outValues[size(realValueReferences, 1)] = realValues;\n")).clone(), (literal!("    external \"C\" fmi1SetReal_OMC(fmi1me, size(realValueReferences, 1), realValueReferences, realValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetReal;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetRealParameter\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real realValueReferences[:];\n")).clone(), (literal!("    input Real realValues[size(realValueReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi1SetReal_OMC(fmi1me, size(realValueReferences, 1), realValueReferences, realValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetRealParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetInteger\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real integerValueReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Integer integerValues[size(integerValueReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetInteger_OMC(fmi1me, size(integerValueReferences, 1), integerValueReferences, inFlowStatesInput, integerValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetInteger;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetInteger\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real integerValuesReferences[:];\n")).clone(), (literal!("    input Integer integerValues[size(integerValuesReferences, 1)];\n")).clone(), (literal!("    output Integer outValues[size(integerValuesReferences, 1)] = integerValues;\n")).clone(), (literal!("    external \"C\" fmi1SetInteger_OMC(fmi1me, size(integerValuesReferences, 1), integerValuesReferences, integerValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetInteger;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetIntegerParameter\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real integerValuesReferences[:];\n")).clone(), (literal!("    input Integer integerValues[size(integerValuesReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi1SetInteger_OMC(fmi1me, size(integerValuesReferences, 1), integerValuesReferences, integerValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetIntegerParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetBoolean\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real booleanValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Boolean booleanValues[size(booleanValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetBoolean_OMC(fmi1me, size(booleanValuesReferences, 1), booleanValuesReferences, inFlowStatesInput, booleanValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetBoolean;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetBoolean\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real booleanValueReferences[:];\n")).clone(), (literal!("    input Boolean booleanValues[size(booleanValueReferences, 1)];\n")).clone(), (literal!("    output Boolean outValues[size(booleanValueReferences, 1)] = booleanValues;\n")).clone(), (literal!("    external \"C\" fmi1SetBoolean_OMC(fmi1me, size(booleanValueReferences, 1), booleanValueReferences, booleanValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetBoolean;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetBooleanParameter\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real booleanValueReferences[:];\n")).clone(), (literal!("    input Boolean booleanValues[size(booleanValueReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi1SetBoolean_OMC(fmi1me, size(booleanValueReferences, 1), booleanValueReferences, booleanValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetBooleanParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetString\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real stringValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output String stringValues[size(stringValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetString_OMC(fmi1me, size(stringValuesReferences, 1), stringValuesReferences, inFlowStatesInput, stringValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetString;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetString\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real stringValueReferences[:];\n")).clone(), (literal!("    input String stringValues[size(stringValueReferences, 1)];\n")).clone(), (literal!("    output String outValues[size(stringValueReferences, 1)] = stringValues;\n")).clone(), (literal!("    external \"C\" fmi1SetString_OMC(fmi1me, size(stringValueReferences, 1), stringValueReferences, stringValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetString;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetStringParameter\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real stringValueReferences[:];\n")).clone(), (literal!("    input String stringValues[size(stringValueReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi1SetString_OMC(fmi1me, size(stringValueReferences, 1), stringValueReferences, stringValues, 1) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetStringParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1EventUpdate\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Boolean intermediateResults;\n")).clone(), (literal!("    output Boolean outNewStatesAvailable;\n")).clone(), (literal!("    external \"C\" outNewStatesAvailable = fmi1EventUpdate_OMC(fmi1me, intermediateResults) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1EventUpdate;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1nextEventTime\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Real outNewnextTime;\n")).clone(), (literal!("    external \"C\" outNewnextTime = fmi1nextEventTime_OMC(fmi1me, inFlowStates) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1nextEventTime;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1CompletedIntegratorStep\n")).clone(), (literal!("    input FMI1ModelExchange fmi1me;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Boolean outCallEventUpdate;\n")).clone(), (literal!("    external \"C\" outCallEventUpdate = fmi1CompletedIntegratorStep_OMC(fmi1me, inFlowStates) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1CompletedIntegratorStep;\n")).clone(), (literal!("end fmi1Functions;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            ret_74 = stringEq((a_name.clone()).clone(), (literal!("")).clone());
            txt = fun_344(txt.clone(), ret_74.clone(), i_fmiInfo.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_346(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo: FMI::Info, mut in_a_fmiInfo_fmiModelIdentifier: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo.clone(), in_a_fmiInfo_fmiModelIdentifier.clone(), in_a_name.clone()) {
        (mut txt, false, _, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiInfo, mut a_fmiInfo_fmiModelIdentifier, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_0 = (FMI::getFMIType(a_fmiInfo.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_347(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiDescription: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiDescription.clone()) {
        (mut txt, false, mut a_fmiInfo_fmiDescription) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_348(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nRealInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nRealInputVariables.clone()) {
        (mut txt, false, mut a_nRealInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real realInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nRealInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_349(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_realInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_350(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nIntegerInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nIntegerInputVariables.clone()) {
        (mut txt, false, mut a_nIntegerInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer integerInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nIntegerInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_351(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_integerInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_352(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nBooleanInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nBooleanInputVariables.clone()) {
        (mut txt, false, mut a_nBooleanInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean booleanInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nBooleanInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_353(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_booleanInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_354(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nStringInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nStringInputVariables.clone()) {
        (mut txt, false, mut a_nStringInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String stringInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nStringInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_355(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_stringInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_356(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nRealEventInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nRealEventInputVariables.clone()) {
        (mut txt, false, mut a_nRealEventInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real realEventInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nRealEventInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("](each fixed=true);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_357(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nIntegerEventInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nIntegerEventInputVariables.clone()) {
        (mut txt, false, mut a_nIntegerEventInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer integerEventInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nIntegerEventInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("](each fixed=true);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_358(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nBooleanEventInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nBooleanEventInputVariables.clone()) {
        (mut txt, false, mut a_nBooleanEventInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean booleanEventInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nBooleanEventInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("](each fixed=true);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_359(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_nStringEventInputVariables: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_nStringEventInputVariables.clone()) {
        (mut txt, false, mut a_nStringEventInputVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String stringEventInputVariables[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nStringEventInputVariables.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("](each fixed=true);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_360(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi_x = fmi2Functions.fmi2GetContinuousStates(fmi2me, numberOfContinuousStates, flowParamsStart+flowInitialized);")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_361(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realParametersNames: Tpl::Text, mut in_a_realParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realParametersNames.clone(), in_a_realParametersVRs.clone()) {
        (mut txt, false, mut a_realParametersNames, mut a_realParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi2Functions.fmi2SetRealParameter(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_362(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerParametersNames: Tpl::Text, mut in_a_integerParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerParametersNames.clone(), in_a_integerParametersVRs.clone()) {
        (mut txt, false, mut a_integerParametersNames, mut a_integerParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi2Functions.fmi2SetIntegerParameter(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_363(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanParametersNames: Tpl::Text, mut in_a_booleanParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanParametersNames.clone(), in_a_booleanParametersVRs.clone()) {
        (mut txt, false, mut a_booleanParametersNames, mut a_booleanParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi2Functions.fmi2SetBooleanParameter(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_364(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringParametersNames: Tpl::Text, mut in_a_stringParametersVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringParametersNames.clone(), in_a_stringParametersVRs.clone()) {
        (mut txt, false, mut a_stringParametersNames, mut a_stringParametersVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flowParamsStart := fmi2Functions.fmi2SetStringParameter(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_365(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realDependentParametersVRs: Tpl::Text, mut in_a_realDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realDependentParametersVRs.clone(), in_a_realDependentParametersNames.clone()) {
        (mut txt, false, mut a_realDependentParametersVRs, mut a_realDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetReal(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_366(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerDependentParametersVRs: Tpl::Text, mut in_a_integerDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerDependentParametersVRs.clone(), in_a_integerDependentParametersNames.clone()) {
        (mut txt, false, mut a_integerDependentParametersVRs, mut a_integerDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetInteger(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_367(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanDependentParametersVRs: Tpl::Text, mut in_a_booleanDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanDependentParametersVRs.clone(), in_a_booleanDependentParametersNames.clone()) {
        (mut txt, false, mut a_booleanDependentParametersVRs, mut a_booleanDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetBoolean(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_368(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringDependentParametersVRs: Tpl::Text, mut in_a_stringDependentParametersNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringDependentParametersVRs.clone(), in_a_stringDependentParametersNames.clone()) {
        (mut txt, false, mut a_stringDependentParametersVRs, mut a_stringDependentParametersNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringDependentParametersNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetString(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringDependentParametersVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_369(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realInputVariablesNames: Tpl::Text, mut in_a_realInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realInputVariablesNames.clone(), in_a_realInputVariablesVRs.clone()) {
        (mut txt, false, mut a_realInputVariablesNames, mut a_realInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("realInputVariables := fmi2Functions.fmi2SetReal(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_370(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerInputVariablesNames: Tpl::Text, mut in_a_integerInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerInputVariablesNames.clone(), in_a_integerInputVariablesVRs.clone()) {
        (mut txt, false, mut a_integerInputVariablesNames, mut a_integerInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integerInputVariables := fmi2Functions.fmi2SetInteger(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_371(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanInputVariablesNames: Tpl::Text, mut in_a_booleanInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanInputVariablesNames.clone(), in_a_booleanInputVariablesVRs.clone()) {
        (mut txt, false, mut a_booleanInputVariablesNames, mut a_booleanInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("booleanInputVariables := fmi2Functions.fmi2SetBoolean(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_372(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringStartVariablesNames: Tpl::Text, mut in_a_stringInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringStartVariablesNames.clone(), in_a_stringInputVariablesVRs.clone()) {
        (mut txt, false, mut a_stringStartVariablesNames, mut a_stringInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("stringInputVariables := fmi2Functions.fmi2SetString(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringStartVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_373(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_realInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = realInputVariables;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_374(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_integerInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = integerInputVariables;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_375(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_booleanInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = booleanInputVariables;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_376(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_stringInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = stringInputVariables;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_377(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realOutputVariablesVRs: Tpl::Text, mut in_a_realOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realOutputVariablesVRs.clone(), in_a_realOutputVariablesNames.clone()) {
        (mut txt, false, mut a_realOutputVariablesVRs, mut a_realOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetReal(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_378(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerOutputVariablesVRs: Tpl::Text, mut in_a_integerOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerOutputVariablesVRs.clone(), in_a_integerOutputVariablesNames.clone()) {
        (mut txt, false, mut a_integerOutputVariablesVRs, mut a_integerOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetInteger(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_379(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanOutputVariablesVRs: Tpl::Text, mut in_a_booleanOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanOutputVariablesVRs.clone(), in_a_booleanOutputVariablesNames.clone()) {
        (mut txt, false, mut a_booleanOutputVariablesVRs, mut a_booleanOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetBoolean(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_380(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringOutputVariablesVRs: Tpl::Text, mut in_a_stringOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringOutputVariablesVRs.clone(), in_a_stringOutputVariablesNames.clone()) {
        (mut txt, false, mut a_stringOutputVariablesVRs, mut a_stringOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi2Functions.fmi2GetString(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_381(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eventIndicator, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("change(fmi_z_positive[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_eventIndicator.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("])")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_381(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_382(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiNumberOfEventIndicators: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiNumberOfEventIndicators.clone())) {
        (txt, false, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  when {triggerDSSEvent > flowStatesInputs, pre(nextEventTime) < time, terminal()} then")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_fmiInfo_fmiNumberOfEventIndicators) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when {")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" or ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_381(txt.clone(), a_fmiInfo_fmiNumberOfEventIndicators.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", triggerDSSEvent > flowStatesInputs, pre(nextEventTime) < time, terminal()} then")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_383(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realEventInputVariablesNames: Tpl::Text, mut in_a_realEventInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realEventInputVariablesNames.clone(), in_a_realEventInputVariablesVRs.clone()) {
        (mut txt, false, mut a_realEventInputVariablesNames, mut a_realEventInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("realEventInputVariables := fmi2Functions.fmi2SetReal(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realEventInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realEventInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_384(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerEventInputVariablesNames: Tpl::Text, mut in_a_integerEventInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerEventInputVariablesNames.clone(), in_a_integerEventInputVariablesVRs.clone()) {
        (mut txt, false, mut a_integerEventInputVariablesNames, mut a_integerEventInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("integerEventInputVariables := fmi2Functions.fmi2SetInteger(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerEventInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerEventInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_385(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanEventInputVariablesNames: Tpl::Text, mut in_a_booleanEventInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanEventInputVariablesNames.clone(), in_a_booleanEventInputVariablesVRs.clone()) {
        (mut txt, false, mut a_booleanEventInputVariablesNames, mut a_booleanEventInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("booleanEventInputVariables := fmi2Functions.fmi2SetBoolean(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanEventInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanEventInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_386(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringEventStartVariablesNames: Tpl::Text, mut in_a_stringEventInputVariablesVRs: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringEventStartVariablesNames.clone(), in_a_stringEventInputVariablesVRs.clone()) {
        (mut txt, false, mut a_stringEventStartVariablesNames, mut a_stringEventInputVariablesVRs) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("stringEventInputVariables := fmi2Functions.fmi2SetString(fmi2me, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringEventInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringEventStartVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_387(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_continuousStates, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(fmi_x[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_continuousStates.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("], fmi_x_new[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_continuousStates.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]);")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_387(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_388(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiNumberOfContinuousStates: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiNumberOfContinuousStates.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_fmiInfo_fmiNumberOfContinuousStates) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("    if newStatesAvailable then\n")).clone(), (literal!("      fmi_x_new := fmi2Functions.fmi2GetContinuousStates(fmi2me, numberOfContinuousStates, flowStatesInputs);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_387(txt.clone(), a_fmiInfo_fmiNumberOfContinuousStates.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("    end if;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_389(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo: FMI::Info, mut in_a_fmiInfo_fmiModelIdentifier: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo.clone(), in_a_fmiInfo_fmiModelIdentifier.clone(), in_a_name.clone()) {
        (mut txt, false, _, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiInfo, mut a_fmiInfo_fmiModelIdentifier, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_0 = (FMI::getFMIType(a_fmiInfo.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn importFMU2ModelExchange(mut in_txt: Tpl::Text, mut in_a_fmi: FMI::FmiImport, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmi.clone(), in_a_name.clone()) {
        (mut txt, FMI::FmiImport { generateOutputConnectors: mut i_generateOutputConnectors, generateInputConnectors: mut i_generateInputConnectors, fmiDebugOutput: mut i_fmiDebugOutput, fmiLogLevel: mut i_fmiLogLevel, fmuWorkingDirectory: mut i_fmuWorkingDirectory, fmiTypeDefinitionsList: ref i_fmiTypeDefinitionsList, fmiModelVariablesList: ref i_fmiModelVariablesList, fmiExperimentAnnotation: FMI::ExperimentAnnotation { fmiExperimentTolerance: mut i_fmiExperimentAnnotation_fmiExperimentTolerance, fmiExperimentStopTime: mut i_fmiExperimentAnnotation_fmiExperimentStopTime, fmiExperimentStartTime: mut i_fmiExperimentAnnotation_fmiExperimentStartTime }, fmiInfo: ref i_fmiInfo @ FMI::Info { fmiNumberOfEventIndicators: ref i_fmiInfo_fmiNumberOfEventIndicators, fmiNumberOfContinuousStates: ref i_fmiInfo_fmiNumberOfContinuousStates, fmiDescription: ref i_fmiInfo_fmiDescription, fmiModelIdentifier: ref i_fmiInfo_fmiModelIdentifier, .. }, .. }, mut a_name) => {
            let mut ret_126: bool = false;
            let mut ret_125: bool = false;
            let mut ret_124: i32 = 0;
            let mut ret_123: bool = false;
            let mut ret_122: bool = false;
            let mut ret_121: bool = false;
            let mut ret_120: bool = false;
            let mut ret_119: bool = false;
            let mut ret_118: i32 = 0;
            let mut ret_117: bool = false;
            let mut ret_116: bool = false;
            let mut ret_115: bool = false;
            let mut ret_114: bool = false;
            let mut ret_113: bool = false;
            let mut ret_112: bool = false;
            let mut ret_111: bool = false;
            let mut ret_110: bool = false;
            let mut ret_109: bool = false;
            let mut ret_108: bool = false;
            let mut ret_107: bool = false;
            let mut ret_106: bool = false;
            let mut ret_105: bool = false;
            let mut ret_104: bool = false;
            let mut ret_103: bool = false;
            let mut ret_102: bool = false;
            let mut ret_101: bool = false;
            let mut ret_100: bool = false;
            let mut ret_99: bool = false;
            let mut ret_98: bool = false;
            let mut ret_97: bool = false;
            let mut ret_96: bool = false;
            let mut ret_95: bool = false;
            let mut ret_94: bool = false;
            let mut ret_93: bool = false;
            let mut ret_92: bool = false;
            let mut ret_91: bool = false;
            let mut ret_90: bool = false;
            let mut ret_89: bool = false;
            let mut ret_88: i32 = 0;
            let mut ret_87: bool = false;
            let mut ret_86: bool = false;
            let mut ret_85: bool = false;
            let mut ret_84: bool = false;
            let mut ret_83: bool = false;
            let mut ret_82: bool = false;
            let mut ret_81: bool = false;
            let mut ret_80: bool = false;
            let mut ret_79: bool = false;
            let mut ret_78: bool = false;
            let mut ret_77: bool = false;
            let mut ret_76: bool = false;
            let mut ret_75: i32 = 0;
            let mut ret_74: i32 = 0;
            let mut ret_73: bool = false;
            let mut ret_72: bool = false;
            let mut l_stringOutputVariablesNames: Tpl::Text;
            let mut l_stringOutputVariablesVRs: Tpl::Text;
            let mut l_booleanOutputVariablesNames: Tpl::Text;
            let mut l_booleanOutputVariablesVRs: Tpl::Text;
            let mut l_integerOutputVariablesNames: Tpl::Text;
            let mut l_integerOutputVariablesVRs: Tpl::Text;
            let mut l_realOutputVariablesNames: Tpl::Text;
            let mut l_realOutputVariablesVRs: Tpl::Text;
            let mut l_stringEventInputVariablesReturnNames: Tpl::Text;
            let mut l_stringEventStartVariablesNames: Tpl::Text;
            let mut l_stringEventInputVariablesVRs: Tpl::Text;
            let mut ret_60: i32 = 0;
            let mut ret_59: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nStringEventInputVariables: Tpl::Text;
            let mut l_booleanEventInputVariablesReturnNames: Tpl::Text;
            let mut l_booleanEventInputVariablesNames: Tpl::Text;
            let mut l_booleanEventInputVariablesVRs: Tpl::Text;
            let mut ret_54: i32 = 0;
            let mut ret_53: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nBooleanEventInputVariables: Tpl::Text;
            let mut l_integerEventInputVariablesReturnNames: Tpl::Text;
            let mut l_integerEventInputVariablesNames: Tpl::Text;
            let mut l_integerEventInputVariablesVRs: Tpl::Text;
            let mut ret_48: i32 = 0;
            let mut ret_47: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nIntegerEventInputVariables: Tpl::Text;
            let mut l_realEventInputVariablesReturnNames: Tpl::Text;
            let mut l_realEventInputVariablesNames: Tpl::Text;
            let mut l_realEventInputVariablesVRs: Tpl::Text;
            let mut ret_42: i32 = 0;
            let mut ret_41: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nRealEventInputVariables: Tpl::Text;
            let mut l_stringInputVariablesReturnNames: Tpl::Text;
            let mut l_stringStartVariablesNames: Tpl::Text;
            let mut l_stringInputVariablesVRs: Tpl::Text;
            let mut ret_36: i32 = 0;
            let mut ret_35: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nStringInputVariables: Tpl::Text;
            let mut l_booleanInputVariablesReturnNames: Tpl::Text;
            let mut l_booleanInputVariablesNames: Tpl::Text;
            let mut l_booleanInputVariablesVRs: Tpl::Text;
            let mut ret_30: i32 = 0;
            let mut ret_29: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nBooleanInputVariables: Tpl::Text;
            let mut l_integerInputVariablesReturnNames: Tpl::Text;
            let mut l_integerInputVariablesNames: Tpl::Text;
            let mut l_integerInputVariablesVRs: Tpl::Text;
            let mut ret_24: i32 = 0;
            let mut ret_23: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nIntegerInputVariables: Tpl::Text;
            let mut l_realInputVariablesReturnNames: Tpl::Text;
            let mut l_realInputVariablesNames: Tpl::Text;
            let mut l_realInputVariablesVRs: Tpl::Text;
            let mut ret_18: i32 = 0;
            let mut ret_17: Arc<metamodelica::List<FMI::ModelVariables>> = metamodelica::nil();
            let mut l_nRealInputVariables: Tpl::Text;
            let mut l_stringDependentParametersNames: Tpl::Text;
            let mut l_stringDependentParametersVRs: Tpl::Text;
            let mut l_booleanDependentParametersNames: Tpl::Text;
            let mut l_booleanDependentParametersVRs: Tpl::Text;
            let mut l_integerDependentParametersNames: Tpl::Text;
            let mut l_integerDependentParametersVRs: Tpl::Text;
            let mut l_realDependentParametersNames: Tpl::Text;
            let mut l_realDependentParametersVRs: Tpl::Text;
            let mut l_stringParametersNames: Tpl::Text;
            let mut l_stringParametersVRs: Tpl::Text;
            let mut l_booleanParametersNames: Tpl::Text;
            let mut l_booleanParametersVRs: Tpl::Text;
            let mut l_integerParametersNames: Tpl::Text;
            let mut l_integerParametersVRs: Tpl::Text;
            let mut l_realParametersNames: Tpl::Text;
            let mut l_realParametersVRs: Tpl::Text;
            l_realParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_realParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_integerParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_integerParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_booleanParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_booleanParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_stringParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_stringParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_realDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("2.0")).clone())?;
            l_realDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("2.0")).clone())?;
            l_integerDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("2.0")).clone())?;
            l_integerDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("2.0")).clone())?;
            l_booleanDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("2.0")).clone())?;
            l_booleanDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("2.0")).clone())?;
            l_stringDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("2.0")).clone())?;
            l_stringDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("2.0")).clone())?;
            ret_17 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone());
            ret_18 = (ret_17.clone().len() as i32);
            l_nRealInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_18.clone())).clone())?;
            l_realInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_realInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_realInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            ret_23 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone());
            ret_24 = (ret_23.clone().len() as i32);
            l_nIntegerInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_24.clone())).clone())?;
            l_integerInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_integerInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_integerInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            ret_29 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone());
            ret_30 = (ret_29.clone().len() as i32);
            l_nBooleanInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_30.clone())).clone())?;
            l_booleanInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_booleanInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_booleanInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            ret_35 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone());
            ret_36 = (ret_35.clone().len() as i32);
            l_nStringInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_36.clone())).clone())?;
            l_stringInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_stringStartVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_stringInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            ret_41 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone());
            ret_42 = (ret_41.clone().len() as i32);
            l_nRealEventInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_42.clone())).clone())?;
            l_realEventInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_realEventInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_realEventInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            ret_47 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone());
            ret_48 = (ret_47.clone().len() as i32);
            l_nIntegerEventInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_48.clone())).clone())?;
            l_integerEventInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_integerEventInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_integerEventInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            ret_53 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone());
            ret_54 = (ret_53.clone().len() as i32);
            l_nBooleanEventInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_54.clone())).clone())?;
            l_booleanEventInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_booleanEventInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_booleanEventInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            ret_59 = FMI::filterModelVariables(i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone());
            ret_60 = (ret_59.clone().len() as i32);
            l_nStringEventInputVariables = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_60.clone())).clone())?;
            l_stringEventInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_stringEventStartVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_stringEventInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 3, (literal!("2.0")).clone())?;
            l_realOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("output")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_realOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("output")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_integerOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("output")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_integerOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("output")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_booleanOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("output")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_booleanOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("output")).clone(), false, 2, (literal!("2.0")).clone())?;
            l_stringOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("output")).clone(), false, 1, (literal!("2.0")).clone())?;
            l_stringOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("output")).clone(), false, 2, (literal!("2.0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("model ")).clone() }))?;
            ret_72 = stringEq((a_name.clone()).clone(), (literal!("")).clone());
            txt = fun_346(txt.clone(), ret_72.clone(), i_fmiInfo.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone(), (a_name.clone()).clone())?;
            ret_73 = stringEq((i_fmiInfo_fmiDescription.clone()).clone(), (literal!("")).clone());
            txt = fun_347(txt.clone(), ret_73.clone(), (i_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant String fmuWorkingDir = \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuWorkingDirectory.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("parameter Integer logLevel = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_fmiLogLevel.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" \"log level used during the loading of FMU\" annotation (Dialog(tab=\"FMI\", group=\"Enable logging\"));\n")).clone(), (literal!("parameter Boolean debugLogging = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_fmiDebugOutput.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" \"enables the FMU simulation logging\" annotation (Dialog(tab=\"FMI\", group=\"Enable logging\"));\n")).clone() }))?;
            txt = dumpFMIModelVariablesList(txt.clone(), (literal!("2.0")).clone(), i_fmiModelVariablesList.clone(), i_fmiTypeDefinitionsList.clone(), i_generateInputConnectors.clone(), i_generateOutputConnectors.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("protected\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMI2ModelExchange fmi2me = FMI2ModelExchange(logLevel, fmuWorkingDir, \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\", debugLogging);\n")).clone(), (literal!("constant Integer numberOfContinuousStates = ")).clone()], lastHasNewLine: false }))?;
            ret_74 = (i_fmiInfo_fmiNumberOfContinuousStates.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_74.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("Real fmi_x[numberOfContinuousStates] \"States\";\n")).clone(), (literal!("Real fmi_x_new[numberOfContinuousStates](each fixed=true) \"New States\";\n")).clone(), (literal!("constant Integer numberOfEventIndicators = ")).clone()], lastHasNewLine: false }))?;
            ret_75 = (i_fmiInfo_fmiNumberOfEventIndicators.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_75.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("Real fmi_z[numberOfEventIndicators] \"Events Indicators\";\n")).clone(), (literal!("Boolean fmi_z_positive[numberOfEventIndicators](each fixed=true);\n")).clone(), (literal!("parameter Real flowStartTime(fixed=false);\n")).clone(), (literal!("Real flowTime;\n")).clone(), (literal!("parameter Real flowEnterInitialization(fixed=false);\n")).clone(), (literal!("parameter Real flowInitialized(fixed=false);\n")).clone(), (literal!("parameter Real flowParamsStart(fixed=false);\n")).clone(), (literal!("parameter Real flowInitInputs(fixed=false);\n")).clone(), (literal!("Real flowStatesInputs;\n")).clone()], lastHasNewLine: true }))?;
            ret_76 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_348(txt.clone(), ret_76.clone(), l_nRealInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_77 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_349(txt.clone(), ret_77.clone(), l_realInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_78 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_350(txt.clone(), ret_78.clone(), l_nIntegerInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_79 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_351(txt.clone(), ret_79.clone(), l_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_80 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_352(txt.clone(), ret_80.clone(), l_nBooleanInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_81 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_353(txt.clone(), ret_81.clone(), l_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_82 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_354(txt.clone(), ret_82.clone(), l_nStringInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_83 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_355(txt.clone(), ret_83.clone(), l_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_84 = stringEq((Tpl::textString(l_realEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_356(txt.clone(), ret_84.clone(), l_nRealEventInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_85 = stringEq((Tpl::textString(l_integerEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_357(txt.clone(), ret_85.clone(), l_nIntegerEventInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_86 = stringEq((Tpl::textString(l_booleanEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_358(txt.clone(), ret_86.clone(), l_nBooleanEventInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_87 = stringEq((Tpl::textString(l_stringEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_359(txt.clone(), ret_87.clone(), l_nStringEventInputVariables.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Boolean callEventUpdate;\n")).clone(), (literal!("Boolean newStatesAvailable(fixed = true);\n")).clone(), (literal!("Real triggerDSSEvent;\n")).clone(), (literal!("Real nextEventTime(fixed = true);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("initial equation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_88 = (i_fmiInfo_fmiNumberOfContinuousStates.clone().len() as i32);
            ret_89 = intGt(ret_88.clone(), 0);
            txt = fun_360(txt.clone(), ret_89.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("initial algorithm\n")).clone(), (literal!("  flowParamsStart := 1;\n")).clone(), (literal!("  flowInitInputs := 1;\n")).clone(), (literal!("  flowStartTime := fmi2Functions.fmi2SetupExperiment(fmi2me, false, 0.0, time, false, 0.0, flowParamsStart+flowInitInputs);\n")).clone(), (literal!("  flowEnterInitialization := fmi2Functions.fmi2EnterInitialization(fmi2me, flowParamsStart+flowInitInputs+flowStartTime);\n")).clone(), (literal!("  flowInitialized := fmi2Functions.fmi2ExitInitialization(fmi2me, flowParamsStart+flowInitInputs+flowStartTime+flowEnterInitialization);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_90 = stringEq((Tpl::textString(l_realParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_361(txt.clone(), ret_90.clone(), l_realParametersNames.clone(), l_realParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_91 = stringEq((Tpl::textString(l_integerParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_362(txt.clone(), ret_91.clone(), l_integerParametersNames.clone(), l_integerParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_92 = stringEq((Tpl::textString(l_booleanParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_363(txt.clone(), ret_92.clone(), l_booleanParametersNames.clone(), l_booleanParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_93 = stringEq((Tpl::textString(l_stringParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_364(txt.clone(), ret_93.clone(), l_stringParametersNames.clone(), l_stringParametersVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("initial equation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_94 = stringEq((Tpl::textString(l_realDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_365(txt.clone(), ret_94.clone(), l_realDependentParametersVRs.clone(), l_realDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_95 = stringEq((Tpl::textString(l_integerDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_366(txt.clone(), ret_95.clone(), l_integerDependentParametersVRs.clone(), l_integerDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_96 = stringEq((Tpl::textString(l_booleanDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_367(txt.clone(), ret_96.clone(), l_booleanDependentParametersVRs.clone(), l_booleanDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_97 = stringEq((Tpl::textString(l_stringDependentParametersVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_368(txt.clone(), ret_97.clone(), l_stringDependentParametersVRs.clone(), l_stringDependentParametersNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("algorithm\n")).clone(), (literal!("  flowTime := if not initial() then fmi2Functions.fmi2SetTime(fmi2me, time, flowInitialized) else time;\n")).clone(), (literal!("  /* algorithm section ensures that inputs to fmi (if any) are set directly after the new time is set */\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_98 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_369(txt.clone(), ret_98.clone(), l_realInputVariablesNames.clone(), l_realInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_99 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_370(txt.clone(), ret_99.clone(), l_integerInputVariablesNames.clone(), l_integerInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_100 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_371(txt.clone(), ret_100.clone(), l_booleanInputVariablesNames.clone(), l_booleanInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_101 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_372(txt.clone(), ret_101.clone(), l_stringStartVariablesNames.clone(), l_stringInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("equation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_102 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_373(txt.clone(), ret_102.clone(), l_realInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_103 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_374(txt.clone(), ret_103.clone(), l_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_104 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_375(txt.clone(), ret_104.clone(), l_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_105 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_376(txt.clone(), ret_105.clone(), l_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("flowStatesInputs = fmi2Functions.fmi2SetContinuousStates(fmi2me, fmi_x, flowParamsStart + flowTime);\n")).clone(), (literal!("der(fmi_x) = fmi2Functions.fmi2GetDerivatives(fmi2me, numberOfContinuousStates, flowStatesInputs);\n")).clone(), (literal!("fmi_z  = fmi2Functions.fmi2GetEventIndicators(fmi2me, numberOfEventIndicators, flowStatesInputs);\n")).clone(), (literal!("for i in 1:size(fmi_z,1) loop\n")).clone(), (literal!("  fmi_z_positive[i] = if not terminal() then fmi_z[i] > 0 else pre(fmi_z_positive[i]);\n")).clone(), (literal!("end for;\n")).clone(), (literal!("\n")).clone(), (literal!("triggerDSSEvent = noEvent(if callEventUpdate then flowStatesInputs+1.0 else flowStatesInputs-1.0);\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            ret_106 = stringEq((Tpl::textString(l_realOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_107 = stringEq((Tpl::textString(l_realOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_108 = boolAnd(ret_106.clone(), ret_107.clone());
            txt = fun_377(txt.clone(), ret_108.clone(), l_realOutputVariablesVRs.clone(), l_realOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_109 = stringEq((Tpl::textString(l_integerOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_110 = stringEq((Tpl::textString(l_integerOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_111 = boolAnd(ret_109.clone(), ret_110.clone());
            txt = fun_378(txt.clone(), ret_111.clone(), l_integerOutputVariablesVRs.clone(), l_integerOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_112 = stringEq((Tpl::textString(l_booleanOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_113 = stringEq((Tpl::textString(l_booleanOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_114 = boolAnd(ret_112.clone(), ret_113.clone());
            txt = fun_379(txt.clone(), ret_114.clone(), l_booleanOutputVariablesVRs.clone(), l_booleanOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_115 = stringEq((Tpl::textString(l_stringOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_116 = stringEq((Tpl::textString(l_stringOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_117 = boolAnd(ret_115.clone(), ret_116.clone());
            txt = fun_380(txt.clone(), ret_117.clone(), l_stringOutputVariablesVRs.clone(), l_stringOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = dumpOutputGetEnumerationVariables(txt.clone(), i_fmiModelVariablesList.clone(), i_fmiTypeDefinitionsList.clone(), (literal!("fmi2Functions.fmi2GetInteger")).clone(), (literal!("fmi2me")).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("callEventUpdate = fmi2Functions.fmi2CompletedIntegratorStep(fmi2me, flowStatesInputs+flowTime);\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("algorithm\n")).clone() }))?;
            ret_118 = (i_fmiInfo_fmiNumberOfEventIndicators.clone().len() as i32);
            ret_119 = intGt(ret_118.clone(), 0);
            txt = fun_382(txt.clone(), ret_119.clone(), i_fmiInfo_fmiNumberOfEventIndicators.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("    fmi2Functions.fmi2StartEventUpdate(fmi2me);\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            ret_120 = stringEq((Tpl::textString(l_realEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_383(txt.clone(), ret_120.clone(), l_realEventInputVariablesNames.clone(), l_realEventInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_121 = stringEq((Tpl::textString(l_integerEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_384(txt.clone(), ret_121.clone(), l_integerEventInputVariablesNames.clone(), l_integerEventInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_122 = stringEq((Tpl::textString(l_booleanEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_385(txt.clone(), ret_122.clone(), l_booleanEventInputVariablesNames.clone(), l_booleanEventInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_123 = stringEq((Tpl::textString(l_stringEventInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_386(txt.clone(), ret_123.clone(), l_stringEventStartVariablesNames.clone(), l_stringEventInputVariablesVRs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("newStatesAvailable := fmi2Functions.fmi2EndEventUpdate(fmi2me);\n")).clone(), (literal!("nextEventTime := fmi2Functions.fmi2nextEventTime(fmi2me, flowStatesInputs);\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            ret_124 = (i_fmiInfo_fmiNumberOfContinuousStates.clone().len() as i32);
            ret_125 = intGt(ret_124.clone(), 0);
            txt = fun_388(txt.clone(), ret_125.clone(), i_fmiInfo_fmiNumberOfContinuousStates.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  end when;\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("annotation(experiment(StartTime=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStartTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", StopTime=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", Tolerance=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentTolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("));\n")).clone(), (literal!("annotation (Icon(graphics={\n")).clone(), (literal!("    Rectangle(\n")).clone(), (literal!("      extent={{-100,100},{100,-100}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      fillColor={240,240,240},\n")).clone(), (literal!("      fillPattern=FillPattern.Solid,\n")).clone(), (literal!("      lineThickness=0.5),\n")).clone(), (literal!("    Text(\n")).clone(), (literal!("      extent={{-100,40},{100,0}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      textString=\"%name\"),\n")).clone(), (literal!("    Text(\n")).clone(), (literal!("      extent={{-100,-50},{100,-90}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      textString=\"V2.0\")}));\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("protected\n")).clone(), (literal!("  class FMI2ModelExchange\n")).clone(), (literal!("    extends ExternalObject;\n")).clone(), (literal!("      function constructor\n")).clone(), (literal!("        input Integer logLevel;\n")).clone(), (literal!("        input String workingDirectory;\n")).clone(), (literal!("        input String instanceName;\n")).clone(), (literal!("        input Boolean debugLogging;\n")).clone(), (literal!("        output FMI2ModelExchange fmi2me;\n")).clone(), (literal!("        external \"C\" fmi2me = FMI2ModelExchangeConstructor_OMC(logLevel, workingDirectory, instanceName, debugLogging) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("      end constructor;\n")).clone(), (literal!("\n")).clone(), (literal!("      function destructor\n")).clone(), (literal!("        input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("        external \"C\" FMI2ModelExchangeDestructor_OMC(fmi2me) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("      end destructor;\n")).clone(), (literal!("  end FMI2ModelExchange;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitionsMappingFunctions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = dumpFMITypeDefinitionsArrayMappingFunctions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("package fmi2Functions\n")).clone(), (literal!("  function fmi2SetupExperiment\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Boolean inToleranceDefined;\n")).clone(), (literal!("    input Real inTolerance;\n")).clone(), (literal!("    input Real inStartTime;\n")).clone(), (literal!("    input Boolean inStopTimeDefined;\n")).clone(), (literal!("    input Real inStopTime;\n")).clone(), (literal!("    input Real inFlow;\n")).clone(), (literal!("    output Real outFlow = inFlow;\n")).clone(), (literal!("    external \"C\" fmi2SetupExperiment_OMC(fmi2me, inToleranceDefined, inTolerance, inStartTime, inStopTimeDefined, inStopTime) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetupExperiment;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetTime\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real inTime;\n")).clone(), (literal!("    input Real inFlow;\n")).clone(), (literal!("    output Real outFlow = inFlow;\n")).clone(), (literal!("    external \"C\" fmi2SetTime_OMC(fmi2me, inTime) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetTime;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2EnterInitialization\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real inFlowVariable;\n")).clone(), (literal!("    output Real outFlowVariable = inFlowVariable;\n")).clone(), (literal!("    external \"C\" fmi2EnterInitializationModel_OMC(fmi2me) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2EnterInitialization;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2ExitInitialization\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real inFlowVariable;\n")).clone(), (literal!("    output Real outFlowVariable = inFlowVariable;\n")).clone(), (literal!("    external \"C\" fmi2ExitInitializationModel_OMC(fmi2me) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2ExitInitialization;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2GetContinuousStates\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Integer numberOfContinuousStates;\n")).clone(), (literal!("    input Real inFlowParams;\n")).clone(), (literal!("    output Real fmi_x[numberOfContinuousStates];\n")).clone(), (literal!("    external \"C\" fmi2GetContinuousStates_OMC(fmi2me, numberOfContinuousStates, inFlowParams, fmi_x) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2GetContinuousStates;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetContinuousStates\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real fmi_x[:];\n")).clone(), (literal!("    input Real inFlowParams;\n")).clone(), (literal!("    output Real outFlowStates;\n")).clone(), (literal!("    external \"C\" outFlowStates = fmi2SetContinuousStates_OMC(fmi2me, size(fmi_x, 1), inFlowParams, fmi_x) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetContinuousStates;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2GetDerivatives\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Integer numberOfContinuousStates;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Real fmi_x[numberOfContinuousStates];\n")).clone(), (literal!("    external \"C\" fmi2GetDerivatives_OMC(fmi2me, numberOfContinuousStates, inFlowStates, fmi_x) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2GetDerivatives;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2GetEventIndicators\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Integer numberOfEventIndicators;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Real fmi_z[numberOfEventIndicators];\n")).clone(), (literal!("    external \"C\" fmi2GetEventIndicators_OMC(fmi2me, numberOfEventIndicators, inFlowStates, fmi_z) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2GetEventIndicators;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2GetReal\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real realValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Real realValues[size(realValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi2GetReal_OMC(fmi2me, size(realValuesReferences, 1), realValuesReferences, inFlowStatesInput, realValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2GetReal;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetReal\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real realValueReferences[:];\n")).clone(), (literal!("    input Real realValues[size(realValueReferences, 1)];\n")).clone(), (literal!("    output Real outValues[size(realValueReferences, 1)] = realValues;\n")).clone(), (literal!("    external \"C\" fmi2SetReal_OMC(fmi2me, size(realValueReferences, 1), realValueReferences, realValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetReal;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetRealParameter\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real realValueReferences[:];\n")).clone(), (literal!("    input Real realValues[size(realValueReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi2SetReal_OMC(fmi2me, size(realValueReferences, 1), realValueReferences, realValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetRealParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2GetInteger\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real integerValueReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Integer integerValues[size(integerValueReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi2GetInteger_OMC(fmi2me, size(integerValueReferences, 1), integerValueReferences, inFlowStatesInput, integerValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2GetInteger;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetInteger\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real integerValuesReferences[:];\n")).clone(), (literal!("    input Integer integerValues[size(integerValuesReferences, 1)];\n")).clone(), (literal!("    output Integer outValues[size(integerValuesReferences, 1)] = integerValues;\n")).clone(), (literal!("    external \"C\" fmi2SetInteger_OMC(fmi2me, size(integerValuesReferences, 1), integerValuesReferences, integerValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetInteger;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetIntegerParameter\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real integerValuesReferences[:];\n")).clone(), (literal!("    input Integer integerValues[size(integerValuesReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi2SetInteger_OMC(fmi2me, size(integerValuesReferences, 1), integerValuesReferences, integerValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetIntegerParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2GetBoolean\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real booleanValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Boolean booleanValues[size(booleanValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi2GetBoolean_OMC(fmi2me, size(booleanValuesReferences, 1), booleanValuesReferences, inFlowStatesInput, booleanValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2GetBoolean;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetBoolean\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real booleanValueReferences[:];\n")).clone(), (literal!("    input Boolean booleanValues[size(booleanValueReferences, 1)];\n")).clone(), (literal!("    output Boolean outValues[size(booleanValueReferences, 1)] = booleanValues;\n")).clone(), (literal!("    external \"C\" fmi2SetBoolean_OMC(fmi2me, size(booleanValueReferences, 1), booleanValueReferences, booleanValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetBoolean;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetBooleanParameter\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real booleanValueReferences[:];\n")).clone(), (literal!("    input Boolean booleanValues[size(booleanValueReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi2SetBoolean_OMC(fmi2me, size(booleanValueReferences, 1), booleanValueReferences, booleanValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetBooleanParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2GetString\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real stringValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output String stringValues[size(stringValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi2GetString_OMC(fmi2me, size(stringValuesReferences, 1), stringValuesReferences, inFlowStatesInput, stringValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2GetString;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetString\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real stringValueReferences[:];\n")).clone(), (literal!("    input String stringValues[size(stringValueReferences, 1)];\n")).clone(), (literal!("    output String outValues[size(stringValueReferences, 1)] = stringValues;\n")).clone(), (literal!("    external \"C\" fmi2SetString_OMC(fmi2me, size(stringValueReferences, 1), stringValueReferences, stringValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetString;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2SetStringParameter\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real stringValueReferences[:];\n")).clone(), (literal!("    input String stringValues[size(stringValueReferences, 1)];\n")).clone(), (literal!("    output Real out_Value = 1;\n")).clone(), (literal!("    external \"C\" fmi2SetString_OMC(fmi2me, size(stringValueReferences, 1), stringValueReferences, stringValues) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2SetStringParameter;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2StartEventUpdate\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    external \"C\" fmi2StartEventUpdate_OMC(fmi2me) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2StartEventUpdate;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2EndEventUpdate\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    output Boolean outNewStatesAvailable;\n")).clone(), (literal!("    external \"C\" outNewStatesAvailable = fmi2EndEventUpdate_OMC(fmi2me) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2EndEventUpdate;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2nextEventTime\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Real outNewnextTime;\n")).clone(), (literal!("    external \"C\" outNewnextTime = fmi2nextEventTime_OMC(fmi2me, inFlowStates) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2nextEventTime;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi2CompletedIntegratorStep\n")).clone(), (literal!("    input FMI2ModelExchange fmi2me;\n")).clone(), (literal!("    input Real inFlowStates;\n")).clone(), (literal!("    output Boolean outCallEventUpdate;\n")).clone(), (literal!("    external \"C\" outCallEventUpdate = fmi2CompletedIntegratorStep_OMC(fmi2me, inFlowStates) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi2CompletedIntegratorStep;\n")).clone(), (literal!("end fmi2Functions;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            ret_126 = stringEq((a_name.clone()).clone(), (literal!("")).clone());
            txt = fun_389(txt.clone(), ret_126.clone(), i_fmiInfo.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_391(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo: FMI::Info, mut in_a_fmiInfo_fmiModelIdentifier: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo.clone(), in_a_fmiInfo_fmiModelIdentifier.clone(), in_a_name.clone()) {
        (mut txt, false, _, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiInfo, mut a_fmiInfo_fmiModelIdentifier, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_0 = (FMI::getFMIType(a_fmiInfo.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_392(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo_fmiDescription: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo_fmiDescription.clone()) {
        (mut txt, false, mut a_fmiInfo_fmiDescription) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_393(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_realInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_394(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_integerInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_395(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_booleanInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_396(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_stringInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_397(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realOutputVariablesVRs: Tpl::Text, mut in_a_realOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realOutputVariablesVRs.clone(), in_a_realOutputVariablesNames.clone()) {
        (mut txt, false, mut a_realOutputVariablesVRs, mut a_realOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetReal(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_398(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerOutputVariablesVRs: Tpl::Text, mut in_a_integerOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerOutputVariablesVRs.clone(), in_a_integerOutputVariablesNames.clone()) {
        (mut txt, false, mut a_integerOutputVariablesVRs, mut a_integerOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetInteger(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_399(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanOutputVariablesVRs: Tpl::Text, mut in_a_booleanOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanOutputVariablesVRs.clone(), in_a_booleanOutputVariablesNames.clone()) {
        (mut txt, false, mut a_booleanOutputVariablesVRs, mut a_booleanOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetBoolean(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_400(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringOutputVariablesVRs: Tpl::Text, mut in_a_stringOutputVariablesNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringOutputVariablesVRs.clone(), in_a_stringOutputVariablesNames.clone()) {
        (mut txt, false, mut a_stringOutputVariablesVRs, mut a_stringOutputVariablesNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringOutputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1GetString(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringOutputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowInitialized);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_401(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_realInputVariablesNames: Tpl::Text, mut in_a_realInputVariablesVRs: Tpl::Text, mut in_a_realInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_realInputVariablesNames.clone(), in_a_realInputVariablesVRs.clone(), in_a_realInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_realInputVariablesNames, mut a_realInputVariablesVRs, mut a_realInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetReal(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_realInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_402(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_integerInputVariablesNames: Tpl::Text, mut in_a_integerInputVariablesVRs: Tpl::Text, mut in_a_integerInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_integerInputVariablesNames.clone(), in_a_integerInputVariablesVRs.clone(), in_a_integerInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_integerInputVariablesNames, mut a_integerInputVariablesVRs, mut a_integerInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetInteger(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_integerInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_403(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_booleanInputVariablesNames: Tpl::Text, mut in_a_booleanInputVariablesVRs: Tpl::Text, mut in_a_booleanInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_booleanInputVariablesNames.clone(), in_a_booleanInputVariablesVRs.clone(), in_a_booleanInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_booleanInputVariablesNames, mut a_booleanInputVariablesVRs, mut a_booleanInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetBoolean(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_booleanInputVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_404(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stringStartVariablesNames: Tpl::Text, mut in_a_stringInputVariablesVRs: Tpl::Text, mut in_a_stringInputVariablesReturnNames: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stringStartVariablesNames.clone(), in_a_stringInputVariablesVRs.clone(), in_a_stringInputVariablesReturnNames.clone()) {
        (mut txt, false, mut a_stringStartVariablesNames, mut a_stringInputVariablesVRs, mut a_stringInputVariablesReturnNames) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = fmi1Functions.fmi1SetString(fmi1cs, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringInputVariablesVRs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_stringStartVariablesNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("});")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_405(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiInfo: FMI::Info, mut in_a_fmiInfo_fmiModelIdentifier: ArcStr, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiInfo.clone(), in_a_fmiInfo_fmiModelIdentifier.clone(), in_a_name.clone()) {
        (mut txt, false, _, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiInfo, mut a_fmiInfo_fmiModelIdentifier, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (a_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_0 = (FMI::getFMIType(a_fmiInfo.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn importFMU1CoSimulationStandAlone(mut in_txt: Tpl::Text, mut in_a_fmi: FMI::FmiImport, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmi.clone(), in_a_name.clone()) {
        (mut txt, FMI::FmiImport { generateOutputConnectors: mut i_generateOutputConnectors, generateInputConnectors: mut i_generateInputConnectors, fmiDebugOutput: mut i_fmiDebugOutput, fmiLogLevel: mut i_fmiLogLevel, fmuWorkingDirectory: mut i_fmuWorkingDirectory, fmiTypeDefinitionsList: ref i_fmiTypeDefinitionsList, fmiModelVariablesList: ref i_fmiModelVariablesList, fmiExperimentAnnotation: FMI::ExperimentAnnotation { fmiExperimentTolerance: mut i_fmiExperimentAnnotation_fmiExperimentTolerance, fmiExperimentStopTime: mut i_fmiExperimentAnnotation_fmiExperimentStopTime, fmiExperimentStartTime: mut i_fmiExperimentAnnotation_fmiExperimentStartTime }, fmiInfo: ref i_fmiInfo @ FMI::Info { fmiDescription: ref i_fmiInfo_fmiDescription, fmiModelIdentifier: ref i_fmiInfo_fmiModelIdentifier, .. }, .. }, mut a_name) => {
            let mut ret_58: bool = false;
            let mut ret_57: bool = false;
            let mut ret_56: bool = false;
            let mut ret_55: bool = false;
            let mut ret_54: bool = false;
            let mut ret_53: bool = false;
            let mut ret_52: bool = false;
            let mut ret_51: bool = false;
            let mut ret_50: bool = false;
            let mut ret_49: bool = false;
            let mut ret_48: bool = false;
            let mut ret_47: bool = false;
            let mut ret_46: bool = false;
            let mut ret_45: bool = false;
            let mut ret_44: bool = false;
            let mut ret_43: bool = false;
            let mut ret_42: bool = false;
            let mut ret_41: bool = false;
            let mut ret_40: bool = false;
            let mut ret_39: bool = false;
            let mut ret_38: bool = false;
            let mut ret_37: bool = false;
            let mut ret_36: bool = false;
            let mut l_stringOutputVariablesNames: Tpl::Text;
            let mut l_stringOutputVariablesVRs: Tpl::Text;
            let mut l_booleanOutputVariablesNames: Tpl::Text;
            let mut l_booleanOutputVariablesVRs: Tpl::Text;
            let mut l_integerOutputVariablesNames: Tpl::Text;
            let mut l_integerOutputVariablesVRs: Tpl::Text;
            let mut l_realOutputVariablesNames: Tpl::Text;
            let mut l_realOutputVariablesVRs: Tpl::Text;
            let mut l_stringInputVariablesReturnNames: Tpl::Text;
            let mut l_stringStartVariablesNames: Tpl::Text;
            let mut l_stringInputVariablesVRs: Tpl::Text;
            let mut l_booleanInputVariablesReturnNames: Tpl::Text;
            let mut l_booleanInputVariablesNames: Tpl::Text;
            let mut l_booleanInputVariablesVRs: Tpl::Text;
            let mut l_integerInputVariablesReturnNames: Tpl::Text;
            let mut l_integerInputVariablesNames: Tpl::Text;
            let mut l_integerInputVariablesVRs: Tpl::Text;
            let mut l_realInputVariablesReturnNames: Tpl::Text;
            let mut l_realInputVariablesNames: Tpl::Text;
            let mut l_realInputVariablesVRs: Tpl::Text;
            let mut l_stringDependentParametersNames: Tpl::Text;
            let mut l_stringDependentParametersVRs: Tpl::Text;
            let mut l_booleanDependentParametersNames: Tpl::Text;
            let mut l_booleanDependentParametersVRs: Tpl::Text;
            let mut l_integerDependentParametersNames: Tpl::Text;
            let mut l_integerDependentParametersVRs: Tpl::Text;
            let mut l_realDependentParametersNames: Tpl::Text;
            let mut l_realDependentParametersVRs: Tpl::Text;
            let mut l_stringParametersNames: Tpl::Text;
            let mut l_stringParametersVRs: Tpl::Text;
            let mut l_booleanParametersNames: Tpl::Text;
            let mut l_booleanParametersVRs: Tpl::Text;
            let mut l_integerParametersNames: Tpl::Text;
            let mut l_integerParametersVRs: Tpl::Text;
            let mut l_realParametersNames: Tpl::Text;
            let mut l_realParametersVRs: Tpl::Text;
            l_realParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_realParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_integerParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_integerParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_booleanParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_booleanParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_stringParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_stringParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_realDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_realDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_integerDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_integerDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_booleanDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_booleanDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_stringDependentParametersVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), true, 1, (literal!("1.0")).clone())?;
            l_stringDependentParametersNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("parameter")).clone(), true, 2, (literal!("1.0")).clone())?;
            l_realInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_realInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_realInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_integerInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_integerInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_integerInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_booleanInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_booleanInputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_booleanInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_stringInputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_stringStartVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_stringInputVariablesReturnNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("input")).clone(), false, 3, (literal!("1.0")).clone())?;
            l_realOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_realOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("real")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_integerOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_integerOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("integer")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_booleanOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_booleanOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("boolean")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            l_stringOutputVariablesVRs = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("output")).clone(), false, 1, (literal!("1.0")).clone())?;
            l_stringOutputVariablesNames = dumpVariables(Tpl::emptyTxt.clone(), i_fmiModelVariablesList.clone(), (literal!("string")).clone(), (literal!("output")).clone(), false, 2, (literal!("1.0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("model ")).clone() }))?;
            ret_36 = stringEq((a_name.clone()).clone(), (literal!("")).clone());
            txt = fun_391(txt.clone(), ret_36.clone(), i_fmiInfo.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone(), (a_name.clone()).clone())?;
            ret_37 = stringEq((i_fmiInfo_fmiDescription.clone()).clone(), (literal!("")).clone());
            txt = fun_392(txt.clone(), ret_37.clone(), (i_fmiInfo_fmiDescription.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant String fmuLocation = \"file://")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuWorkingDirectory.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/resources\";\n")).clone(), (literal!("constant String fmuWorkingDir = \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmuWorkingDirectory.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("parameter Integer logLevel = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_fmiLogLevel.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" \"log level used during the loading of FMU\" annotation (Dialog(tab=\"FMI\", group=\"Enable logging\"));\n")).clone(), (literal!("parameter Boolean debugLogging = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_fmiDebugOutput.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" \"enables the FMU simulation logging\" annotation (Dialog(tab=\"FMI\", group=\"Enable logging\"));\n")).clone(), (literal!("constant String mimeType = \"\";\n")).clone(), (literal!("constant Real timeout = 0.0;\n")).clone(), (literal!("constant Boolean visible = false;\n")).clone(), (literal!("constant Boolean interactive = false;\n")).clone(), (literal!("parameter Real startTime = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStartTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" \"start time used to initialize the slave\" annotation (Dialog(tab=\"FMI\", group=\"Step time\"));\n")).clone(), (literal!("parameter Real stopTime = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" \"stop time used to initialize the slave\" annotation (Dialog(tab=\"FMI\", group=\"Step time\"));\n")).clone(), (literal!("parameter Real numberOfSteps = 500 annotation (Dialog(tab=\"FMI\", group=\"Step time\"));\n")).clone(), (literal!("parameter Real communicationStepSize = (stopTime-startTime)/numberOfSteps \"step size used by fmiDoStep\" annotation (Dialog(tab=\"FMI\", group=\"Step time\"));\n")).clone(), (literal!("constant Boolean stopTimeDefined = true;\n")).clone()], lastHasNewLine: true }))?;
            txt = dumpFMIModelVariablesList(txt.clone(), (literal!("1.0")).clone(), i_fmiModelVariablesList.clone(), i_fmiTypeDefinitionsList.clone(), i_generateInputConnectors.clone(), i_generateOutputConnectors.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("protected\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMI1CoSimulation fmi1cs = FMI1CoSimulation(logLevel, fmuWorkingDir, \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\", debugLogging, fmuLocation, mimeType, timeout, visible, interactive, startTime, stopTimeDefined, stopTime);\n")).clone(), (literal!("parameter Real flowInitialized(fixed=false);\n")).clone(), (literal!("Real flowStep;\n")).clone()], lastHasNewLine: true }))?;
            ret_38 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_393(txt.clone(), ret_38.clone(), l_realInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_39 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_394(txt.clone(), ret_39.clone(), l_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_40 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_395(txt.clone(), ret_40.clone(), l_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_41 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_396(txt.clone(), ret_41.clone(), l_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("initial equation\n")).clone(), (literal!("  flowInitialized = fmi1Functions.fmi1InitializeSlave(fmi1cs, 1);\n")).clone(), (literal!("equation\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_42 = stringEq((Tpl::textString(l_realOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_43 = stringEq((Tpl::textString(l_realOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_44 = boolAnd(ret_42.clone(), ret_43.clone());
            txt = fun_397(txt.clone(), ret_44.clone(), l_realOutputVariablesVRs.clone(), l_realOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_45 = stringEq((Tpl::textString(l_integerOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_46 = stringEq((Tpl::textString(l_integerOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_47 = boolAnd(ret_45.clone(), ret_46.clone());
            txt = fun_398(txt.clone(), ret_47.clone(), l_integerOutputVariablesVRs.clone(), l_integerOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_48 = stringEq((Tpl::textString(l_booleanOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_49 = stringEq((Tpl::textString(l_booleanOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_50 = boolAnd(ret_48.clone(), ret_49.clone());
            txt = fun_399(txt.clone(), ret_50.clone(), l_booleanOutputVariablesVRs.clone(), l_booleanOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_51 = stringEq((Tpl::textString(l_stringOutputVariablesNames.clone())?).clone(), (literal!("")).clone());
            ret_52 = stringEq((Tpl::textString(l_stringOutputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            ret_53 = boolAnd(ret_51.clone(), ret_52.clone());
            txt = fun_400(txt.clone(), ret_53.clone(), l_stringOutputVariablesVRs.clone(), l_stringOutputVariablesNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_54 = stringEq((Tpl::textString(l_realInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_401(txt.clone(), ret_54.clone(), l_realInputVariablesNames.clone(), l_realInputVariablesVRs.clone(), l_realInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_55 = stringEq((Tpl::textString(l_integerInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_402(txt.clone(), ret_55.clone(), l_integerInputVariablesNames.clone(), l_integerInputVariablesVRs.clone(), l_integerInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_56 = stringEq((Tpl::textString(l_booleanInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_403(txt.clone(), ret_56.clone(), l_booleanInputVariablesNames.clone(), l_booleanInputVariablesVRs.clone(), l_booleanInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_57 = stringEq((Tpl::textString(l_stringInputVariablesVRs.clone())?).clone(), (literal!("")).clone());
            txt = fun_404(txt.clone(), ret_57.clone(), l_stringStartVariablesNames.clone(), l_stringInputVariablesVRs.clone(), l_stringInputVariablesReturnNames.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("flowStep = fmi1Functions.fmi1DoStep(fmi1cs, time, communicationStepSize, true, flowInitialized);\n")).clone(), (literal!("annotation(experiment(StartTime=")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStartTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", StopTime=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentStopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", Tolerance=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_fmiExperimentAnnotation_fmiExperimentTolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("));\n")).clone(), (literal!("annotation (Icon(graphics={\n")).clone(), (literal!("    Rectangle(\n")).clone(), (literal!("      extent={{-100,100},{100,-100}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      fillColor={240,240,240},\n")).clone(), (literal!("      fillPattern=FillPattern.Solid,\n")).clone(), (literal!("      lineThickness=0.5),\n")).clone(), (literal!("    Text(\n")).clone(), (literal!("      extent={{-100,40},{100,0}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      textString=\"%name\"),\n")).clone(), (literal!("    Text(\n")).clone(), (literal!("      extent={{-100,-50},{100,-90}},\n")).clone(), (literal!("      lineColor={0,0,0},\n")).clone(), (literal!("      textString=\"V1.0\")}));\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("protected\n")).clone(), (literal!("  class FMI1CoSimulation\n")).clone(), (literal!("    extends ExternalObject;\n")).clone(), (literal!("      function constructor\n")).clone(), (literal!("        input Integer fmiLogLevel;\n")).clone(), (literal!("        input String workingDirectory;\n")).clone(), (literal!("        input String instanceName;\n")).clone(), (literal!("        input Boolean debugLogging;\n")).clone(), (literal!("        input String fmuLocation;\n")).clone(), (literal!("        input String mimeType;\n")).clone(), (literal!("        input Real timeOut;\n")).clone(), (literal!("        input Boolean visible;\n")).clone(), (literal!("        input Boolean interactive;\n")).clone(), (literal!("        input Real tStart;\n")).clone(), (literal!("        input Boolean stopTimeDefined;\n")).clone(), (literal!("        input Real tStop;\n")).clone(), (literal!("        output FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("        external \"C\" fmi1cs = FMI1CoSimulationConstructor_OMC(fmiLogLevel, workingDirectory, instanceName, debugLogging, fmuLocation, mimeType, timeOut, visible, interactive, tStart, stopTimeDefined, tStop) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("      end constructor;\n")).clone(), (literal!("\n")).clone(), (literal!("      function destructor\n")).clone(), (literal!("        input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("        external \"C\" FMI1CoSimulationDestructor_OMC(fmi1cs) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("      end destructor;\n")).clone(), (literal!("  end FMI1CoSimulation;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitionsMappingFunctions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = dumpFMITypeDefinitionsArrayMappingFunctions(txt.clone(), i_fmiTypeDefinitionsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("package fmi1Functions\n")).clone(), (literal!("  function fmi1InitializeSlave\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real preInitialized;\n")).clone(), (literal!("    output Real postInitialized=preInitialized;\n")).clone(), (literal!("    external \"C\" fmi1InitializeSlave_OMC(fmi1cs) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1InitializeSlave;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1DoStep\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real currentCommunicationPoint;\n")).clone(), (literal!("    input Real communicationStepSize;\n")).clone(), (literal!("    input Boolean newStep;\n")).clone(), (literal!("    input Real preInitialized;\n")).clone(), (literal!("    output Real postInitialized=preInitialized;\n")).clone(), (literal!("    external \"C\" fmi1DoStep_OMC(fmi1cs, currentCommunicationPoint, communicationStepSize, newStep) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1DoStep;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetReal\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real realValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Real realValues[size(realValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetReal_OMC(fmi1cs, size(realValuesReferences, 1), realValuesReferences, inFlowStatesInput, realValues, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetReal;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetReal\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real realValuesReferences[:];\n")).clone(), (literal!("    input Real realValues[size(realValuesReferences, 1)];\n")).clone(), (literal!("    output Real out_Values[size(realValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1SetReal_OMC(fmi1cs, size(realValuesReferences, 1), realValuesReferences, realValues, out_Values, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetReal;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetInteger\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real integerValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Integer integerValues[size(integerValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetInteger_OMC(fmi1cs, size(integerValuesReferences, 1), integerValuesReferences, inFlowStatesInput, integerValues, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetInteger;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetInteger\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real integerValuesReferences[:];\n")).clone(), (literal!("    input Integer integerValues[size(integerValuesReferences, 1)];\n")).clone(), (literal!("    output Real out_Values[size(integerValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1SetInteger_OMC(fmi1cs, size(integerValuesReferences, 1), integerValuesReferences, integerValues, out_Values, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetInteger;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetBoolean\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real booleanValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output Boolean booleanValues[size(booleanValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetBoolean_OMC(fmi1cs, size(booleanValuesReferences, 1), booleanValuesReferences, inFlowStatesInput, booleanValues, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetBoolean;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetBoolean\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real booleanValuesReferences[:];\n")).clone(), (literal!("    input Boolean booleanValues[size(booleanValuesReferences, 1)];\n")).clone(), (literal!("    output Boolean out_Values[size(booleanValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1SetBoolean_OMC(fmi1cs, size(booleanValuesReferences, 1), booleanValuesReferences, booleanValues, out_Values, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetBoolean;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1GetString\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real stringValuesReferences[:];\n")).clone(), (literal!("    input Real inFlowStatesInput;\n")).clone(), (literal!("    output String stringValues[size(stringValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1GetString_OMC(fmi1cs, size(stringValuesReferences, 1), stringValuesReferences, inFlowStatesInput, stringValues, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1GetString;\n")).clone(), (literal!("\n")).clone(), (literal!("  function fmi1SetString\n")).clone(), (literal!("    input FMI1CoSimulation fmi1cs;\n")).clone(), (literal!("    input Real stringValuesReferences[:];\n")).clone(), (literal!("    input String stringValues[size(stringValuesReferences, 1)];\n")).clone(), (literal!("    output String out_Values[size(stringValuesReferences, 1)];\n")).clone(), (literal!("    external \"C\" fmi1SetString_OMC(fmi1cs, size(stringValuesReferences, 1), stringValuesReferences, stringValues, out_Values, 2) annotation(Library = {\"OpenModelicaFMIRuntimeC\", \"fmilib\"});\n")).clone(), (literal!("  end fmi1SetString;\n")).clone(), (literal!("end fmi1Functions;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            ret_58 = stringEq((a_name.clone()).clone(), (literal!("")).clone());
            txt = fun_405(txt.clone(), ret_58.clone(), i_fmiInfo.clone(), (i_fmiInfo_fmiModelIdentifier.clone()).clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_407(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::TypeDefinitions>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiTypeDefinition, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFMITypeDefinition(txt.clone(), i_fmiTypeDefinition.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_407(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitions(mut txt: Tpl::Text, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_407(out_txt.clone(), a_fmiTypeDefinitionsList.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpFMITypeDefinition(mut in_txt: Tpl::Text, mut in_a_fmiTypeDefinition: FMI::TypeDefinitions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiTypeDefinition.clone()) {
        (mut txt, FMI::TypeDefinitions { items: ref i_items, name: mut i_name, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("type ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" = enumeration(\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpFMITypeDefinitionsItems(txt.clone(), i_items.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
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
fn lm_410(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::EnumerationItem>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_item, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFMITypeDefinitionsItem(txt.clone(), i_item.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_410(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitionsItems(mut txt: Tpl::Text, mut a_items: Arc<metamodelica::List<FMI::EnumerationItem>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(",\n")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_410(out_txt.clone(), a_items.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitionsItem(mut in_txt: Tpl::Text, mut in_a_item: FMI::EnumerationItem) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_item.clone()) {
        (mut txt, FMI::EnumerationItem { name: mut i_name, .. }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
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
fn lm_413(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::TypeDefinitions>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiTypeDefinition, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFMITypeDefinitionMappingFunction(txt.clone(), i_fmiTypeDefinition.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_413(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitionsMappingFunctions(mut txt: Tpl::Text, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_413(out_txt.clone(), a_fmiTypeDefinitionsList.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_415(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::EnumerationItem>>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_name.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_item, tail: rest }, a_name) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = dumpFMITypeDefinitionMappingFunctionItems(txt.clone(), i_item.clone(), (a_name.clone()).clone(), x_i0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_415(txt.clone(), rest.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_416(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end if;")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitionMappingFunction(mut in_txt: Tpl::Text, mut in_a_fmiTypeDefinition: FMI::TypeDefinitions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiTypeDefinition.clone()) {
        (mut txt, FMI::TypeDefinitions { items: ref i_items, name: mut i_name, .. }) => {
            let mut ret_1: bool = false;
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function map_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_from_integer\n")).clone(), (literal!("  input Integer i;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" outType;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("algorithm\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_415(txt.clone(), i_items.clone(), (i_name.clone()).clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_0 = (i_items.clone().len() as i32);
            ret_1 = intGt(ret_0.clone(), 1);
            txt = fun_416(txt.clone(), ret_1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end map_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_from_integer;")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_418(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_typeName: ArcStr, mut in_a_i: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_typeName.clone(), in_a_i.clone()) {
        (mut txt, false, mut a_name, mut a_typeName, mut a_i) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elseif i == ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" then outType := ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_typeName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_name, mut a_typeName, mut a_i) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if i == ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_i.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" then outType := ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_typeName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitionMappingFunctionItems(mut in_txt: Tpl::Text, mut in_a_item: FMI::EnumerationItem, mut in_a_typeName: ArcStr, mut in_a_i: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_item.clone(), in_a_typeName.clone(), in_a_i.clone()) {
        (mut txt, FMI::EnumerationItem { name: mut i_name, .. }, mut a_typeName, mut a_i) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_i.clone(), 1);
            txt = fun_418(txt.clone(), ret_0.clone(), (i_name.clone()).clone(), (a_typeName.clone()).clone(), a_i.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_420(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::TypeDefinitions>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiTypeDefinition, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFMITypeDefinitionsArrayMappingFunction(txt.clone(), i_fmiTypeDefinition.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_420(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitionsArrayMappingFunctions(mut txt: Tpl::Text, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_420(out_txt.clone(), a_fmiTypeDefinitionsList.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpFMITypeDefinitionsArrayMappingFunction(mut in_txt: Tpl::Text, mut in_a_fmiTypeDefinition: FMI::TypeDefinitions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiTypeDefinition.clone()) {
        (mut txt, FMI::TypeDefinitions { name: mut i_name, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function map_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_from_integers\n")).clone(), (literal!("  input Integer fromInt[size(fromInt, 1)];\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" toEnum[size(fromInt, 1)];\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("protected\n")).clone(), (literal!("  Integer n = size(fromInt, 1);\n")).clone(), (literal!("algorithm\n")).clone(), (literal!("  for i in 1:n loop\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("toEnum[i] := map_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_from_integer(fromInt[i]);\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  end for;\n")).clone(), (literal!("end map_")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_from_integers;")).clone() }))?;
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
fn lm_423(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::ModelVariables>>, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone(), in_a_fmiTypeDefinitionsList.clone(), in_a_FMUVersion.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiModelVariable, tail: rest }, a_generateOutputConnectors, a_generateInputConnectors, a_fmiTypeDefinitionsList, a_FMUVersion) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariable(txt.clone(), (a_FMUVersion.clone()).clone(), i_fmiModelVariable.clone(), a_fmiTypeDefinitionsList.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_423(txt.clone(), rest.clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), a_fmiTypeDefinitionsList.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFMIModelVariablesList(mut txt: Tpl::Text, mut a_FMUVersion: ArcStr, mut a_fmiModelVariablesList: Arc<metamodelica::List<FMI::ModelVariables>>, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut a_generateInputConnectors: bool, mut a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_423(out_txt.clone(), a_fmiModelVariablesList.clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone(), a_fmiTypeDefinitionsList.clone(), (a_FMUVersion.clone()).clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_425(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut in_a_FMUVersion: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_fmiTypeDefinitionsList.clone(), in_a_FMUVersion.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIRealModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_variability.clone()).clone(), i_hasStartValue.clone(), i_startValue.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::INTEGERVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_1, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIIntegerModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_variability.clone()).clone(), i_hasStartValue.clone(), i_startValue_1.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_2, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIBooleanModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_variability.clone()).clone(), i_hasStartValue.clone(), i_startValue_2.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::STRINGVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_3, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIStringModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_variability.clone()).clone(), i_hasStartValue.clone(), (i_startValue_3.clone()).clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::ENUMERATIONVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_1, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, a_fmiTypeDefinitionsList, _, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIEnumerationModelVariableStartValue(txt.clone(), a_fmiTypeDefinitionsList.clone(), (i_baseType.clone()).clone(), i_hasStartValue.clone(), i_startValue_1.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_426(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut in_a_FMUVersion: ArcStr, mut in_a_generateOutputConnectors: bool, mut in_a_generateInputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_fmiTypeDefinitionsList.clone(), in_a_FMUVersion.clone(), in_a_generateOutputConnectors.clone(), in_a_generateInputConnectors.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIRealModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_causality.clone()).clone(), i_hasStartValue.clone(), i_startValue.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::INTEGERVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_1, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIIntegerModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_causality.clone()).clone(), i_hasStartValue.clone(), i_startValue_1.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_2, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIBooleanModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_causality.clone()).clone(), i_hasStartValue.clone(), i_startValue_2.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::STRINGVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_3, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, _, a_FMUVersion, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIStringModelVariableStartValue(txt.clone(), (a_FMUVersion.clone()).clone(), (i_causality.clone()).clone(), i_hasStartValue.clone(), (i_startValue_3.clone()).clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::ENUMERATIONVARIABLE { y2Placement: i_y2Placement, y1Placement: i_y1Placement, x2Placement: i_x2Placement, x1Placement: i_x1Placement, description: i_description, isFixed: i_isFixed, startValue: i_startValue_1, hasStartValue: i_hasStartValue, name: i_name, baseType: i_baseType, causality: i_causality, variability: i_variability, .. }, a_fmiTypeDefinitionsList, _, a_generateOutputConnectors, a_generateInputConnectors) => {
            let mut txt = (*txt).clone();
            txt = dumpFMIModelVariableVariability(txt.clone(), (i_variability.clone()).clone())?;
            txt = dumpFMIModelVariableCausalityAndBaseType(txt.clone(), (i_causality.clone()).clone(), (i_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpFMIEnumerationModelVariableStartValue(txt.clone(), a_fmiTypeDefinitionsList.clone(), (i_baseType.clone()).clone(), i_hasStartValue.clone(), i_startValue_1.clone(), i_isFixed.clone())?;
            txt = dumpFMIModelVariableDescription(txt.clone(), (i_description.clone()).clone())?;
            txt = dumpFMIModelVariablePlacementAnnotation(txt.clone(), i_x1Placement.clone(), i_x2Placement.clone(), i_y1Placement.clone(), i_y2Placement.clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone(), (i_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFMIModelVariable(mut in_txt: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut in_a_generateInputConnectors: bool, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_FMUVersion.clone(), in_a_fmiModelVariable.clone(), in_a_fmiTypeDefinitionsList.clone(), in_a_generateInputConnectors.clone(), in_a_generateOutputConnectors.clone())) {
        (txt, i_FMUVersion @ Deref @ "1.0", a_fmiModelVariable, a_fmiTypeDefinitionsList, a_generateInputConnectors, a_generateOutputConnectors) => {
            let mut txt = (*txt).clone();
            txt = fun_425(txt.clone(), a_fmiModelVariable.clone(), a_fmiTypeDefinitionsList.clone(), (i_FMUVersion.clone()).clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone())?;
            txt.clone()
        },
        (txt, i_FMUVersion @ Deref @ "2.0", a_fmiModelVariable, a_fmiTypeDefinitionsList, a_generateInputConnectors, a_generateOutputConnectors) => {
            let mut txt = (*txt).clone();
            txt = fun_426(txt.clone(), a_fmiModelVariable.clone(), a_fmiTypeDefinitionsList.clone(), (i_FMUVersion.clone()).clone(), a_generateOutputConnectors.clone(), a_generateInputConnectors.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_428(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_variability: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_variability.clone()) {
        (mut txt, false, mut a_variability) => {
            txt = Tpl::writeStr(txt.clone(), (a_variability.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMIModelVariableVariability(mut txt: Tpl::Text, mut a_variability: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: bool = false;
    ret_0 = stringEq((a_variability.clone()).clone(), (literal!("")).clone());
    out_txt = fun_428(txt.clone(), ret_0.clone(), (a_variability.clone()).clone())?;
    Ok(out_txt)
}

fn fun_430(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_baseType.clone(), in_a_causality.clone()) {
        (mut txt, false, mut a_baseType, mut a_causality) => {
            txt = Tpl::writeStr(txt.clone(), (a_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_baseType.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_baseType, _) => {
            txt = Tpl::writeStr(txt.clone(), (a_baseType.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_431(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_baseType.clone(), in_a_causality.clone()) {
        (mut txt, false, mut a_baseType, mut a_causality) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("")).clone());
            txt = fun_430(txt.clone(), ret_0.clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.BooleanOutput")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_432(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_baseType, mut a_causality, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("output")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Boolean")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateOutputConnectors.clone(), ret_2.clone());
            txt = fun_431(txt.clone(), ret_3.clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.IntegerOutput")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_433(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_baseType, mut a_causality, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("output")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Integer")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateOutputConnectors.clone(), ret_2.clone());
            txt = fun_432(txt.clone(), ret_3.clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateOutputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.RealOutput")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_434(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_baseType, mut a_causality, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("output")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Real")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateOutputConnectors.clone(), ret_2.clone());
            txt = fun_433(txt.clone(), ret_3.clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateOutputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.BooleanInput")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_435(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_generateOutputConnectors: bool, mut in_a_baseType: ArcStr, mut in_a_causality: ArcStr, mut in_a_generateInputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_generateOutputConnectors.clone(), in_a_baseType.clone(), in_a_causality.clone(), in_a_generateInputConnectors.clone()) {
        (mut txt, false, mut a_generateOutputConnectors, mut a_baseType, mut a_causality, mut a_generateInputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("input")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Boolean")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateInputConnectors.clone(), ret_2.clone());
            txt = fun_434(txt.clone(), ret_3.clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateOutputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.IntegerInput")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_436(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_causality: ArcStr, mut in_a_baseType: ArcStr, mut in_a_generateInputConnectors: bool, mut in_a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_causality.clone(), in_a_baseType.clone(), in_a_generateInputConnectors.clone(), in_a_generateOutputConnectors.clone()) {
        (mut txt, false, mut a_causality, mut a_baseType, mut a_generateInputConnectors, mut a_generateOutputConnectors) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("input")).clone());
            ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Integer")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            ret_3 = boolAnd(a_generateInputConnectors.clone(), ret_2.clone());
            txt = fun_435(txt.clone(), ret_3.clone(), a_generateOutputConnectors.clone(), (a_baseType.clone()).clone(), (a_causality.clone()).clone(), a_generateInputConnectors.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Modelica.Blocks.Interfaces.RealInput")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMIModelVariableCausalityAndBaseType(mut txt: Tpl::Text, mut a_causality: ArcStr, mut a_baseType: ArcStr, mut a_generateInputConnectors: bool, mut a_generateOutputConnectors: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_3: bool = false;
    let mut ret_2: bool = false;
    let mut ret_1: bool = false;
    let mut ret_0: bool = false;
    ret_0 = stringEq((a_causality.clone()).clone(), (literal!("input")).clone());
    ret_1 = stringEq((a_baseType.clone()).clone(), (literal!("Real")).clone());
    ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
    ret_3 = boolAnd(a_generateInputConnectors.clone(), ret_2.clone());
    out_txt = fun_436(txt.clone(), ret_3.clone(), (a_causality.clone()).clone(), (a_baseType.clone()).clone(), a_generateInputConnectors.clone(), a_generateOutputConnectors.clone())?;
    Ok(out_txt)
}

fn fun_438(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_causality: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_causality.clone()) {
        (mut txt, false, mut a_causality) => {
            txt = Tpl::writeStr(txt.clone(), (a_causality.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMIModelVariableCausality(mut txt: Tpl::Text, mut a_causality: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: bool = false;
    ret_0 = stringEq((a_causality.clone()).clone(), (literal!("")).clone());
    out_txt = fun_438(txt.clone(), ret_0.clone(), (a_causality.clone()).clone())?;
    Ok(out_txt)
}

fn fun_440(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_441(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_440(txt.clone(), ret_2.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_442(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, _, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_441(txt.clone(), ret_1.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_443(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_442(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startValue.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_444(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_445(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_443(txt.clone(), ret_0.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(a_hasStartValue.clone(), ret_1.clone());
            txt = fun_444(txt.clone(), ret_2.clone(), a_startValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_446(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_447(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_446(txt.clone(), ret_1.clone(), a_startValue.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_448(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_447(txt.clone(), ret_2.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_449(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_448(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_450(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_449(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startValue.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_451(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_450(txt.clone(), ret_0.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFMIRealModelVariableStartValue(mut in_txt: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_variabilityCausality: ArcStr, mut in_a_hasStartValue: bool, mut in_a_startValue: metamodelica::Real, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_FMUVersion.clone(), in_a_variabilityCausality.clone(), in_a_hasStartValue.clone(), in_a_startValue.clone(), in_a_isFixed.clone())) {
        (txt, Deref @ "1.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_445(txt.clone(), (a_variabilityCausality.clone()).clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "2.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_451(txt.clone(), (a_variabilityCausality.clone()).clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_453(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_454(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_453(txt.clone(), ret_2.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_455(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, _, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_454(txt.clone(), ret_1.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_456(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_455(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_startValue.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_457(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_458(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_456(txt.clone(), ret_0.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(a_hasStartValue.clone(), ret_1.clone());
            txt = fun_457(txt.clone(), ret_2.clone(), a_startValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_459(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_460(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_459(txt.clone(), ret_1.clone(), a_startValue.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_461(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_460(txt.clone(), ret_2.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_462(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_461(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_463(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_462(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_startValue.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_464(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: i32, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_463(txt.clone(), ret_0.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFMIIntegerModelVariableStartValue(mut in_txt: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_variabilityCausality: ArcStr, mut in_a_hasStartValue: bool, mut in_a_startValue: i32, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_FMUVersion.clone(), in_a_variabilityCausality.clone(), in_a_hasStartValue.clone(), in_a_startValue.clone(), in_a_isFixed.clone())) {
        (txt, Deref @ "1.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_458(txt.clone(), (a_variabilityCausality.clone()).clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "2.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_464(txt.clone(), (a_variabilityCausality.clone()).clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_466(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_467(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_466(txt.clone(), ret_2.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_468(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, _, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_467(txt.clone(), ret_1.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_469(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_468(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_startValue.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_470(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_471(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_469(txt.clone(), ret_0.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(a_hasStartValue.clone(), ret_1.clone());
            txt = fun_470(txt.clone(), ret_2.clone(), a_startValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_472(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_473(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_472(txt.clone(), ret_1.clone(), a_startValue.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_474(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_473(txt.clone(), ret_2.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_475(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_474(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_476(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_475(txt.clone(), ret_1.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_startValue.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_477(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_476(txt.clone(), ret_0.clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFMIBooleanModelVariableStartValue(mut in_txt: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_variabilityCausality: ArcStr, mut in_a_hasStartValue: bool, mut in_a_startValue: bool, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_FMUVersion.clone(), in_a_variabilityCausality.clone(), in_a_hasStartValue.clone(), in_a_startValue.clone(), in_a_isFixed.clone())) {
        (txt, Deref @ "1.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_471(txt.clone(), (a_variabilityCausality.clone()).clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "2.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_477(txt.clone(), (a_variabilityCausality.clone()).clone(), a_startValue.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_479(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_480(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_479(txt.clone(), ret_2.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_481(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, _, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_480(txt.clone(), ret_1.clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_startValue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_482(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_481(txt.clone(), ret_1.clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_startValue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_483(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_startValue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_484(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_482(txt.clone(), ret_0.clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(a_hasStartValue.clone(), ret_1.clone());
            txt = fun_483(txt.clone(), ret_2.clone(), (a_startValue.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_485(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_startValue) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_startValue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_486(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_485(txt.clone(), ret_1.clone(), (a_startValue.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_487(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolNot(a_isFixed.clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_486(txt.clone(), ret_2.clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, _, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(fixed=true)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_488(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_hasStartValue.clone());
            ret_1 = boolAnd(ret_0.clone(), a_isFixed.clone());
            txt = fun_487(txt.clone(), ret_1.clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(start=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_startValue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\",fixed=false)")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_489(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone()) {
        (mut txt, false, mut a_startValue, mut a_isFixed, mut a_hasStartValue) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = boolNot(a_isFixed.clone());
            ret_1 = boolAnd(a_hasStartValue.clone(), ret_0.clone());
            txt = fun_488(txt.clone(), ret_1.clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_startValue, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_startValue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_490(mut in_txt: Tpl::Text, mut in_a_variabilityCausality: ArcStr, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool, mut in_a_hasStartValue: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variabilityCausality.clone(), in_a_startValue.clone(), in_a_isFixed.clone(), in_a_hasStartValue.clone())) {
        (txt, Deref @ "parameter", a_startValue, a_isFixed, a_hasStartValue) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = boolAnd(a_hasStartValue.clone(), a_isFixed.clone());
            txt = fun_489(txt.clone(), ret_0.clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFMIStringModelVariableStartValue(mut in_txt: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_variabilityCausality: ArcStr, mut in_a_hasStartValue: bool, mut in_a_startValue: ArcStr, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_FMUVersion.clone(), in_a_variabilityCausality.clone(), in_a_hasStartValue.clone(), in_a_startValue.clone(), in_a_isFixed.clone())) {
        (txt, Deref @ "1.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_484(txt.clone(), (a_variabilityCausality.clone()).clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, Deref @ "2.0", a_variabilityCausality, a_hasStartValue, a_startValue, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = fun_490(txt.clone(), (a_variabilityCausality.clone()).clone(), (a_startValue.clone()).clone(), a_isFixed.clone(), a_hasStartValue.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_492(mut in_txt: Tpl::Text, mut in_a_hasStartValue: bool, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut in_a_baseType: ArcStr, mut in_a_startValue: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_hasStartValue.clone(), in_a_fmiTypeDefinitionsList.clone(), in_a_baseType.clone(), in_a_startValue.clone())) {
        (txt, false, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_fmiTypeDefinitionsList, a_baseType, a_startValue) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = map_")).clone() }))?;
            ret_0 = (FMI::getEnumerationTypeFromTypes(a_fmiTypeDefinitionsList.clone(), (a_baseType.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_from_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_startValue.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFMIEnumerationModelVariableStartValue(mut txt: Tpl::Text, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut a_baseType: ArcStr, mut a_hasStartValue: bool, mut a_startValue: i32, mut a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_492(txt.clone(), a_hasStartValue.clone(), a_fmiTypeDefinitionsList.clone(), (a_baseType.clone()).clone(), a_startValue.clone())?;
    Ok(out_txt)
}

fn fun_494(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_description: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_description.clone()) {
        (mut txt, false, mut a_description) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_description.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMIModelVariableDescription(mut txt: Tpl::Text, mut a_description: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: bool = false;
    ret_0 = stringEq((a_description.clone()).clone(), (literal!("")).clone());
    out_txt = fun_494(txt.clone(), ret_0.clone(), (a_description.clone()).clone())?;
    Ok(out_txt)
}

fn fun_496(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_y2Placement: i32, mut in_a_x2Placement: i32, mut in_a_y1Placement: i32, mut in_a_x1Placement: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_y2Placement.clone(), in_a_x2Placement.clone(), in_a_y1Placement.clone(), in_a_x1Placement.clone()) {
        (mut txt, false, _, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_y2Placement, mut a_x2Placement, mut a_y1Placement, mut a_x1Placement) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" annotation(Placement(transformation(extent={{")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_x1Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_y1Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},{")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_x2Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_y2Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}})))")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_497(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_x1Placement: i32, mut in_a_x2Placement: i32, mut in_a_y1Placement: i32, mut in_a_y2Placement: i32, mut in_a_generateOutputConnectors: bool, mut in_a_causality: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_x1Placement.clone(), in_a_x2Placement.clone(), in_a_y1Placement.clone(), in_a_y2Placement.clone(), in_a_generateOutputConnectors.clone(), in_a_causality.clone()) {
        (mut txt, false, mut a_x1Placement, mut a_x2Placement, mut a_y1Placement, mut a_y2Placement, mut a_generateOutputConnectors, mut a_causality) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_causality.clone()).clone(), (literal!("output")).clone());
            ret_1 = boolAnd(a_generateOutputConnectors.clone(), ret_0.clone());
            txt = fun_496(txt.clone(), ret_1.clone(), a_y2Placement.clone(), a_x2Placement.clone(), a_y1Placement.clone(), a_x1Placement.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_x1Placement, mut a_x2Placement, mut a_y1Placement, mut a_y2Placement, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" annotation(Placement(transformation(extent={{")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_x1Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_y1Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("},{")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_x2Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_y2Placement.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}})))")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFMIModelVariablePlacementAnnotation(mut txt: Tpl::Text, mut a_x1Placement: i32, mut a_x2Placement: i32, mut a_y1Placement: i32, mut a_y2Placement: i32, mut a_generateInputConnectors: bool, mut a_generateOutputConnectors: bool, mut a_causality: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: bool = false;
    let mut ret_0: bool = false;
    ret_0 = stringEq((a_causality.clone()).clone(), (literal!("input")).clone());
    ret_1 = boolAnd(a_generateInputConnectors.clone(), ret_0.clone());
    out_txt = fun_497(txt.clone(), ret_1.clone(), a_x1Placement.clone(), a_x2Placement.clone(), a_y1Placement.clone(), a_y2Placement.clone(), a_generateOutputConnectors.clone(), (a_causality.clone()).clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_499(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::ModelVariables>>, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_dependent: bool, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_dependent.clone(), in_a_variabilityCausality.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiModelVariable, tail: rest }, a_fmiVersion, a_what, a_dependent, a_variabilityCausality, a_type) => {
            let mut txt = (*txt).clone();
            txt = dumpVariable(txt.clone(), i_fmiModelVariable.clone(), (a_type.clone()).clone(), (a_variabilityCausality.clone()).clone(), a_dependent.clone(), a_what.clone(), (a_fmiVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_499(txt.clone(), rest.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpVariables(mut txt: Tpl::Text, mut a_fmiModelVariablesList: Arc<metamodelica::List<FMI::ModelVariables>>, mut a_type: ArcStr, mut a_variabilityCausality: ArcStr, mut a_dependent: bool, mut a_what: i32, mut a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_499(out_txt.clone(), a_fmiModelVariablesList.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_501(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_502(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_501(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_503(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_504(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_503(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_505(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::STRINGVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "", variability: Deref @ "", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_502(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::STRINGVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "output", variability: Deref @ "", .. }, a_what) => {
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = intEq(a_what.clone(), 1);
            txt = fun_504(txt.clone(), ret_1.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_506(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_505(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_507(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_508(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_507(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_509(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_510(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_509(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_511(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "", variability: Deref @ "", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_508(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "output", variability: Deref @ "", .. }, a_what) => {
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = intEq(a_what.clone(), 1);
            txt = fun_510(txt.clone(), ret_1.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_512(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("string")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("output")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_506(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _, _) => {
            txt = fun_511(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_513(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_514(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_513(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_515(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_516(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_515(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_517(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::INTEGERVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "", variability: Deref @ "", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_514(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::INTEGERVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "output", variability: Deref @ "", .. }, a_what) => {
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = intEq(a_what.clone(), 1);
            txt = fun_516(txt.clone(), ret_1.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_518(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("boolean")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("output")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_512(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _, _) => {
            txt = fun_517(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_519(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_520(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_519(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_521(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_522(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_521(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_523(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "", variability: Deref @ "", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_520(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::REALVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "output", variability: Deref @ "", .. }, a_what) => {
            let mut ret_1: bool = false;
            let mut txt = (*txt).clone();
            ret_1 = intEq(a_what.clone(), 1);
            txt = fun_522(txt.clone(), ret_1.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_524(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("integer")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("output")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_518(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _, _) => {
            txt = fun_523(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_525(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi_input_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_526(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 3);
            txt = fun_525(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_527(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_526(txt.clone(), ret_0.clone(), (a_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_528(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::STRINGVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "input", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_527(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_529(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("real")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("output")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_524(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _, _) => {
            txt = fun_528(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_530(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi_input_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_531(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 3);
            txt = fun_530(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_532(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_531(txt.clone(), ret_0.clone(), (a_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_533(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "input", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_532(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_534(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("string")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("input")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_529(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _, _) => {
            txt = fun_533(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_535(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi_input_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_536(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 3);
            txt = fun_535(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_537(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_536(txt.clone(), ret_0.clone(), (a_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_538(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::INTEGERVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "input", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_537(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_539(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("boolean")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("input")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_534(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _, _) => {
            txt = fun_538(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_540(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fmi_input_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_541(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 3);
            txt = fun_540(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name, _) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_542(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_541(txt.clone(), ret_0.clone(), (a_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_543(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { valueReference: i_valueReference, name: i_name, causality: Deref @ "input", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_542(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_544(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("integer")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("input")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_539(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _, _) => {
            txt = fun_543(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_545(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_546(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_545(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_547(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::STRINGVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_546(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_548(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_547(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_549(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_550(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_549(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_551(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::STRINGVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_550(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_552(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_548(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_551(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_553(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, _, mut a_what, mut a_fmiModelVariable, mut a_variabilityCausality, mut a_type) => {
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("real")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("input")).clone());
            ret_2 = boolAnd(ret_0.clone(), ret_1.clone());
            txt = fun_544(txt.clone(), ret_2.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, _, _) => {
            let mut ret_3: bool = false;
            ret_3 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_552(txt.clone(), ret_3.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_554(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_555(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_554(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_556(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_555(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_557(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_556(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_558(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_559(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_558(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_560(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_559(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_561(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_557(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_560(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_562(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_dependent: bool, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_dependent.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, mut a_dependent, mut a_variabilityCausality, mut a_type) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("string")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
            ret_2 = boolAnd(ret_1.clone(), a_dependent.clone());
            ret_3 = boolAnd(ret_0.clone(), ret_2.clone());
            txt = fun_553(txt.clone(), ret_3.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, _, _, _) => {
            let mut ret_4: bool = false;
            ret_4 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_561(txt.clone(), ret_4.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_563(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_564(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_563(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_565(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::INTEGERVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_564(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_566(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_565(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_567(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_568(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_567(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_569(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::INTEGERVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_568(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_570(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_566(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_569(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_571(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_dependent: bool, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_dependent.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, mut a_dependent, mut a_variabilityCausality, mut a_type) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("boolean")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
            ret_2 = boolAnd(ret_1.clone(), a_dependent.clone());
            ret_3 = boolAnd(ret_0.clone(), ret_2.clone());
            txt = fun_562(txt.clone(), ret_3.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_fmiModelVariable.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, _, _, _) => {
            let mut ret_4: bool = false;
            ret_4 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_570(txt.clone(), ret_4.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_572(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_573(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_572(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_574(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_573(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_575(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_574(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_576(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_577(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_576(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_578(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { valueReference: i_valueReference, name: i_name, isFixed: false, hasStartValue: false, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_577(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_579(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_575(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_578(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_580(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_dependent: bool, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_dependent.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, mut a_dependent, mut a_variabilityCausality, mut a_type) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("integer")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
            ret_2 = boolAnd(ret_1.clone(), a_dependent.clone());
            ret_3 = boolAnd(ret_0.clone(), ret_2.clone());
            txt = fun_571(txt.clone(), ret_3.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_fmiModelVariable.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, _, _, _) => {
            let mut ret_4: bool = false;
            ret_4 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_579(txt.clone(), ret_4.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_581(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_582(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_581(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_583(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::STRINGVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_582(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_584(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_583(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_585(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_586(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_585(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_587(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::STRINGVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_586(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_588(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_584(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_587(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_589(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_dependent: bool, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_dependent.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, mut a_dependent, mut a_variabilityCausality, mut a_type) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("real")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
            ret_2 = boolAnd(ret_1.clone(), a_dependent.clone());
            ret_3 = boolAnd(ret_0.clone(), ret_2.clone());
            txt = fun_580(txt.clone(), ret_3.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_fmiModelVariable.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, _, _, _) => {
            let mut ret_4: bool = false;
            ret_4 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_588(txt.clone(), ret_4.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_590(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_591(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_590(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_592(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_591(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_593(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_592(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_594(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_595(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_594(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_596(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::BOOLEANVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_595(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_597(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_593(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_596(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_598(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_dependent: bool, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_dependent.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, mut a_dependent, mut a_variabilityCausality, mut a_type) => {
            let mut ret_4: bool = false;
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("string")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
            ret_2 = boolNot(a_dependent.clone());
            ret_3 = boolAnd(ret_1.clone(), ret_2.clone());
            ret_4 = boolAnd(ret_0.clone(), ret_3.clone());
            txt = fun_589(txt.clone(), ret_4.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_fmiModelVariable.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, _, _, _) => {
            let mut ret_5: bool = false;
            ret_5 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_597(txt.clone(), ret_5.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_599(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_600(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_599(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_601(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::INTEGERVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_600(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_602(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_601(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_603(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_604(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_603(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_605(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::INTEGERVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_604(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_606(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_602(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_605(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_607(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiVersion: ArcStr, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_dependent: bool, mut in_a_variabilityCausality: ArcStr, mut in_a_type: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiVersion.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_dependent.clone(), in_a_variabilityCausality.clone(), in_a_type.clone()) {
        (mut txt, false, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, mut a_dependent, mut a_variabilityCausality, mut a_type) => {
            let mut ret_4: bool = false;
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("boolean")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
            ret_2 = boolNot(a_dependent.clone());
            ret_3 = boolAnd(ret_1.clone(), ret_2.clone());
            ret_4 = boolAnd(ret_0.clone(), ret_3.clone());
            txt = fun_598(txt.clone(), ret_4.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_fmiModelVariable.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiVersion, mut a_what, mut a_fmiModelVariable, _, _, _) => {
            let mut ret_5: bool = false;
            ret_5 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_606(txt.clone(), ret_5.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_608(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_609(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_608(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_610(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, causality: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_609(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_611(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable) => {
            txt = fun_610(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_612(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_613(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_valueReference: metamodelica::Real, mut in_a_name: ArcStr, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_valueReference.clone(), in_a_name.clone(), in_a_what.clone()) {
        (mut txt, false, _, mut a_name, mut a_what) => {
            let mut ret_0: bool = false;
            ret_0 = intEq(a_what.clone(), 2);
            txt = fun_612(txt.clone(), ret_0.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_valueReference, _, _) => {
            txt = Tpl::writeStr(txt.clone(), (realString(a_valueReference.clone())).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_614(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_what: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_what.clone())) {
        (txt, FMI::ModelVariables::REALVARIABLE { valueReference: i_valueReference, name: i_name, hasStartValue: true, variability: Deref @ "parameter", .. }, a_what) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_what.clone(), 1);
            txt = fun_613(txt.clone(), ret_0.clone(), i_valueReference.clone(), (i_name.clone()).clone(), a_what.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_615(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_what: i32, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_what.clone(), in_a_fmiModelVariable.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_what, mut a_fmiModelVariable, mut a_fmiVersion) => {
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_fmiVersion.clone()).clone(), (literal!("2.0")).clone());
            txt = fun_611(txt.clone(), ret_0.clone(), a_what.clone(), a_fmiModelVariable.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_what, mut a_fmiModelVariable, _) => {
            txt = fun_614(txt.clone(), a_fmiModelVariable.clone(), a_what.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_616(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_type: ArcStr, mut in_a_variabilityCausality: ArcStr, mut in_a_dependent: bool, mut in_a_what: i32, mut in_a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmiModelVariable.clone(), in_a_type.clone(), in_a_variabilityCausality.clone(), in_a_dependent.clone(), in_a_what.clone(), in_a_fmiVersion.clone()) {
        (mut txt, false, mut a_fmiModelVariable, mut a_type, mut a_variabilityCausality, mut a_dependent, mut a_what, mut a_fmiVersion) => {
            let mut ret_4: bool = false;
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            ret_0 = stringEq((a_type.clone()).clone(), (literal!("integer")).clone());
            ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
            ret_2 = boolNot(a_dependent.clone());
            ret_3 = boolAnd(ret_1.clone(), ret_2.clone());
            ret_4 = boolAnd(ret_0.clone(), ret_3.clone());
            txt = fun_607(txt.clone(), ret_4.clone(), (a_fmiVersion.clone()).clone(), a_what.clone(), a_fmiModelVariable.clone(), a_dependent.clone(), (a_variabilityCausality.clone()).clone(), (a_type.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_fmiModelVariable, _, _, _, mut a_what, mut a_fmiVersion) => {
            let mut ret_5: bool = false;
            ret_5 = stringEq((a_fmiVersion.clone()).clone(), (literal!("1.0")).clone());
            txt = fun_615(txt.clone(), ret_5.clone(), a_what.clone(), a_fmiModelVariable.clone(), (a_fmiVersion.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpVariable(mut txt: Tpl::Text, mut a_fmiModelVariable: FMI::ModelVariables, mut a_type: ArcStr, mut a_variabilityCausality: ArcStr, mut a_dependent: bool, mut a_what: i32, mut a_fmiVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_4: bool = false;
    let mut ret_3: bool = false;
    let mut ret_2: bool = false;
    let mut ret_1: bool = false;
    let mut ret_0: bool = false;
    ret_0 = stringEq((a_type.clone()).clone(), (literal!("real")).clone());
    ret_1 = stringEq((a_variabilityCausality.clone()).clone(), (literal!("parameter")).clone());
    ret_2 = boolNot(a_dependent.clone());
    ret_3 = boolAnd(ret_1.clone(), ret_2.clone());
    ret_4 = boolAnd(ret_0.clone(), ret_3.clone());
    out_txt = fun_616(txt.clone(), ret_4.clone(), a_fmiModelVariable.clone(), (a_type.clone()).clone(), (a_variabilityCausality.clone()).clone(), a_dependent.clone(), a_what.clone(), (a_fmiVersion.clone()).clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_618(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<FMI::ModelVariables>>, mut in_a_fmiType: ArcStr, mut in_a_fmiGetFunction: ArcStr, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fmiType.clone(), in_a_fmiGetFunction.clone(), in_a_fmiTypeDefinitionsList.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiModelVariable, tail: rest }, a_fmiType, a_fmiGetFunction, a_fmiTypeDefinitionsList) => {
            let mut txt = (*txt).clone();
            txt = dumpOutputGetEnumerationVariable(txt.clone(), i_fmiModelVariable.clone(), a_fmiTypeDefinitionsList.clone(), (a_fmiGetFunction.clone()).clone(), (a_fmiType.clone()).clone())?;
            txt = lm_618(txt.clone(), rest.clone(), (a_fmiType.clone()).clone(), (a_fmiGetFunction.clone()).clone(), a_fmiTypeDefinitionsList.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpOutputGetEnumerationVariables(mut txt: Tpl::Text, mut a_fmiModelVariablesList: Arc<metamodelica::List<FMI::ModelVariables>>, mut a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut a_fmiGetFunction: ArcStr, mut a_fmiType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = lm_618(txt.clone(), a_fmiModelVariablesList.clone(), (a_fmiType.clone()).clone(), (a_fmiGetFunction.clone()).clone(), a_fmiTypeDefinitionsList.clone())?;
    Ok(out_txt)
}

pub fn dumpOutputGetEnumerationVariable(mut in_txt: Tpl::Text, mut in_a_fmiModelVariable: FMI::ModelVariables, mut in_a_fmiTypeDefinitionsList: Arc<metamodelica::List<FMI::TypeDefinitions>>, mut in_a_fmiGetFunction: ArcStr, mut in_a_fmiType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiModelVariable.clone(), in_a_fmiTypeDefinitionsList.clone(), in_a_fmiGetFunction.clone(), in_a_fmiType.clone())) {
        (txt, FMI::ModelVariables::ENUMERATIONVARIABLE { valueReference: i_valueReference, baseType: i_baseType, name: i_name, causality: Deref @ "", variability: Deref @ "", .. }, a_fmiTypeDefinitionsList, a_fmiGetFunction, a_fmiType) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = map_")).clone() }))?;
            ret_0 = (FMI::getEnumerationTypeFromTypes(a_fmiTypeDefinitionsList.clone(), (i_baseType.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_from_integers(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiGetFunction.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiType.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", {")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_valueReference.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs));")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, FMI::ModelVariables::ENUMERATIONVARIABLE { valueReference: i_valueReference, baseType: i_baseType, name: i_name, causality: Deref @ "output", variability: Deref @ "", .. }, a_fmiTypeDefinitionsList, a_fmiGetFunction, a_fmiType) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} = map_")).clone() }))?;
            ret_1 = (FMI::getEnumerationTypeFromTypes(a_fmiTypeDefinitionsList.clone(), (i_baseType.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_from_integers(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiGetFunction.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fmiType.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", {")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_valueReference.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, flowStatesInputs));")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
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
fn lm_621(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("realVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_621(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_622(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("realVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_622(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_623(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("realVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_623(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_624(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("realVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_624(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_625(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("realVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_625(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_626(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("realVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_626(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_627(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("realParameterData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_627(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_628(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("integerVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_628(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_629(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("integerParameterData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_629(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_630(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("booleanVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_630(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_631(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("booleanParameterData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_631(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_632(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("stringVarsData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_632(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_633(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariableFMU(txt.clone(), i_var.clone(), (literal!("stringParameterData")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_633(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_634(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { makefileParams: SimCodeFunction::MakefileParams { omhome: ref i_makefileParams_omhome, .. }, simulationSettingsOpt: Some(SimCode::SimulationSettings { variableFilter: ref i_s_variableFilter, outputFormat: ref i_s_outputFormat, method: ref i_s_method, tolerance: ref i_s_tolerance, stepSize: ref i_s_stepSize, stopTime: ref i_s_stopTime, startTime: ref i_s_startTime, .. }), modelInfo: SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringParamVars: ref i_vars_stringParamVars, stringAlgVars: ref i_vars_stringAlgVars, boolParamVars: ref i_vars_boolParamVars, boolAlgVars: ref i_vars_boolAlgVars, intParamVars: ref i_vars_intParamVars, intAlgVars: ref i_vars_intAlgVars, paramVars: ref i_vars_paramVars, realOptimizeFinalConstraintsVars: ref i_vars_realOptimizeFinalConstraintsVars, realOptimizeConstraintsVars: ref i_vars_realOptimizeConstraintsVars, discreteAlgVars: ref i_vars_discreteAlgVars, algVars: ref i_vars_algVars, derivativeVars: ref i_vars_derivativeVars, stateVars: ref i_vars_stateVars, .. }, varInfo: SimCode::VarInfo { numZeroCrossings: _, .. }, functions: _, .. }, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#include \"simulation_data.h\"\n")).clone(), (literal!("#include \"util/real_array.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("OMC_DISABLE_OPT\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt_0 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("read_simulation_info")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(SIMULATION_INFO* simulationInfo)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("simulationInfo->startTime = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_s_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("simulationInfo->stopTime = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_s_stopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("simulationInfo->stepSize = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_s_stepSize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("simulationInfo->tolerance = ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_s_tolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("simulationInfo->solverMethod = \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_s_method.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("simulationInfo->outputFormat = \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_s_outputFormat.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("simulationInfo->variableFilter = \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_s_variableFilter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("simulationInfo->OPENMODELICAHOME = \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_makefileParams_omhome.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\";\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt_1 = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("read_input_fmu")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(MODEL_DATA* modelData)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            System::tmpTickReset(1000);
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_621(txt.clone(), i_vars_stateVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_622(txt.clone(), i_vars_derivativeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_623(txt.clone(), i_vars_algVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_624(txt.clone(), i_vars_discreteAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_625(txt.clone(), i_vars_realOptimizeConstraintsVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_626(txt.clone(), i_vars_realOptimizeFinalConstraintsVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_627(txt.clone(), i_vars_paramVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_628(txt.clone(), i_vars_intAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_629(txt.clone(), i_vars_intParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_630(txt.clone(), i_vars_boolAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_631(txt.clone(), i_vars_boolParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_632(txt.clone(), i_vars_stringAlgVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_633(txt.clone(), i_vars_stringParamVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickResetIndex(0, 2);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn simulationInitFunction(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_guid: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_634(txt.clone(), a_simCode.clone())?;
    Ok(out_txt)
}

fn fun_636(mut in_txt: Tpl::Text, mut in_a_isReadOnly: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isReadOnly.clone()) {
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

fn fun_637(mut in_txt: Tpl::Text, mut in_a_info: SourceInfo, mut in_a_str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_info.clone(), in_a_str.clone()) {
        (mut txt, SourceInfo { isReadOnly: mut i_isReadOnly, columnNumberEnd: mut i_columnNumberEnd, lineNumberEnd: mut i_lineNumberEnd, columnNumberStart: mut i_columnNumberStart, lineNumberStart: mut i_lineNumberStart, fileName: mut i_fileName, .. }, mut a_str) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (a_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".filename = \"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((i_fileName.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".lineStart = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".colStart = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".lineEnd = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".colEnd = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".readonly = ")).clone() }))?;
            txt = fun_636(txt.clone(), i_isReadOnly.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getInfoArgsFMU(mut txt: Tpl::Text, mut a_str: ArcStr, mut a_info: SourceInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_637(txt.clone(), a_info.clone(), (a_str.clone()).clone())?;
    Ok(out_txt)
}

fn fun_639(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_comment) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (Util::escapeModelicaStringToCString((i_comment.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableFMU(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_classType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simVar.clone(), in_a_classType.clone())) {
        (txt, SimCodeVar::SimVar { type_: i_type__, isFixed: i_isFixed, nominalValue: i_nominalValue, initialValue: i_initialValue, maxValue: i_maxValue, minValue: i_minValue, displayUnit: i_displayUnit, unit: i_unit, name: i_name, comment: i_comment, source: Deref @ DAE::ElementSource { info: i_info, .. }, .. }, a_classType) => {
            let mut txt_9: Tpl::Text;
            let mut ret_8: ArcStr = arcstr::literal!("");
            let mut txt_7: Tpl::Text;
            let mut l_attrstr: Tpl::Text;
            let mut l_infostr: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut ret_3: i32 = 0;
            let mut l_ci: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut l_valueReference: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = System::tmpTick();
            l_valueReference = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            ret_3 = System::tmpTickIndex(2);
            l_ci = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_3.clone())).clone())?;
            l_description = fun_639(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            l_infostr = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelData->")).clone() }))?;
            l_infostr = Tpl::writeStr(l_infostr.clone(), (a_classType.clone()).clone())?;
            l_infostr = Tpl::writeTok(l_infostr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            l_infostr = Tpl::writeText(l_infostr.clone(), l_ci.clone())?;
            l_infostr = Tpl::writeTok(l_infostr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].info")).clone() }))?;
            l_attrstr = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelData->")).clone() }))?;
            l_attrstr = Tpl::writeStr(l_attrstr.clone(), (a_classType.clone()).clone())?;
            l_attrstr = Tpl::writeTok(l_attrstr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            l_attrstr = Tpl::writeText(l_attrstr.clone(), l_ci.clone())?;
            l_attrstr = Tpl::writeTok(l_attrstr.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("].attribute")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_infostr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".id = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_valueReference.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_infostr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".name = \"")).clone() }))?;
            txt_7 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_8 = (Util::escapeModelicaStringToCString((Tpl::textString(txt_7.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_8.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\";\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_infostr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".comment = \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_description.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\";\n")).clone() }))?;
            txt_9 = Tpl::writeText(Tpl::emptyTxt.clone(), l_infostr.clone())?;
            txt_9 = Tpl::writeTok(txt_9.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".info")).clone() }))?;
            txt = getInfoArgsFMU(txt.clone(), (Tpl::textString(txt_9.clone())?).clone(), i_info.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = ScalarVariableTypeFMU(txt.clone(), (Tpl::textString(l_attrstr.clone())?).clone(), (i_unit.clone()).clone(), (i_displayUnit.clone()).clone(), i_minValue.clone(), i_maxValue.clone(), i_initialValue.clone(), i_nominalValue.clone(), i_isFixed.clone(), i_type__.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_641(mut in_txt: Tpl::Text, mut in_a_bool: bool) -> Result<Tpl::Text> {
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

fn fun_642(mut in_txt: Tpl::Text, mut in_a_e: Arc<DAE::Exp>, mut in_a_default: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_e.clone(), in_a_default.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: i_integer }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: i_real }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_real.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, _) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_scon(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((i_string.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }, _) => {
            let mut txt = (*txt).clone();
            txt = fun_641(txt.clone(), i_bool.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, _, a_default) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_default.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn optInitValFMU(mut in_txt: Tpl::Text, mut in_a_exp: Option<Arc<DAE::Exp>>, mut in_a_default: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_default.clone())) {
        (txt, Some(i_e), a_default) => {
            let mut txt = (*txt).clone();
            txt = fun_642(txt.clone(), i_e.clone(), (a_default.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_default) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_default.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_644(mut in_txt: Tpl::Text, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isFixed.clone()) {
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

fn fun_645(mut in_txt: Tpl::Text, mut in_a_nominalValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_nominalValue.clone())) {
        (txt, None) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_646(mut in_txt: Tpl::Text, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isFixed.clone()) {
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

fn fun_647(mut in_txt: Tpl::Text, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isFixed.clone()) {
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

fn fun_648(mut in_txt: Tpl::Text, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isFixed.clone()) {
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

fn fun_649(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_attrstr: ArcStr, mut in_a_unit: ArcStr, mut in_a_displayUnit: ArcStr, mut in_a_minValue: Option<Arc<DAE::Exp>>, mut in_a_maxValue: Option<Arc<DAE::Exp>>, mut in_a_startValue: Option<Arc<DAE::Exp>>, mut in_a_nominalValue: Option<Arc<DAE::Exp>>, mut in_a_isFixed: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_attrstr.clone(), in_a_unit.clone(), in_a_displayUnit.clone(), in_a_minValue.clone(), in_a_maxValue.clone(), in_a_startValue.clone(), in_a_nominalValue.clone(), in_a_isFixed.clone())) {
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_attrstr, a_unit, a_displayUnit, a_minValue, a_maxValue, a_startValue, a_nominalValue, a_isFixed) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".unit = \"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((a_unit.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".displayUnit = \"")).clone() }))?;
            ret_1 = (Util::escapeModelicaStringToCString((a_displayUnit.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("put_real_element(")).clone()], lastHasNewLine: false }))?;
            txt = optInitValFMU(txt.clone(), a_minValue.clone(), (literal!("-DBL_MAX")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 0, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".min);\n")).clone(), (literal!("put_real_element(")).clone()], lastHasNewLine: false }))?;
            txt = optInitValFMU(txt.clone(), a_maxValue.clone(), (literal!("DBL_MAX")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 0, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".max);\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fixed = ")).clone() }))?;
            txt = fun_644(txt.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".useNominal = ")).clone() }))?;
            txt = fun_645(txt.clone(), a_nominalValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("put_real_element(")).clone()], lastHasNewLine: false }))?;
            txt = optInitValFMU(txt.clone(), a_nominalValue.clone(), (literal!("1.0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 0, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".nominal);\n")).clone(), (literal!("put_real_element(")).clone()], lastHasNewLine: false }))?;
            txt = optInitValFMU(txt.clone(), a_startValue.clone(), (literal!("0.0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", 0, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".start);")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_attrstr, _, _, a_minValue, a_maxValue, a_startValue, _, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".min = ")).clone() }))?;
            txt = optInitValFMU(txt.clone(), a_minValue.clone(), (literal!("-LONG_MAX")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".max = ")).clone() }))?;
            txt = optInitValFMU(txt.clone(), a_maxValue.clone(), (literal!("LONG_MAX")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fixed = ")).clone() }))?;
            txt = fun_646(txt.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".start = ")).clone() }))?;
            txt = optInitValFMU(txt.clone(), a_startValue.clone(), (literal!("0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_attrstr, _, _, _, _, a_startValue, _, a_isFixed) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fixed = ")).clone() }))?;
            txt = fun_647(txt.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".start = ")).clone() }))?;
            txt = optInitValFMU(txt.clone(), a_startValue.clone(), (literal!("0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_attrstr, _, _, _, _, a_startValue, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".start = ")).clone() }))?;
            txt = optInitValFMU(txt.clone(), a_startValue.clone(), (literal!("mmc_mk_scon(\"\")")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { names: i_names, .. }, a_attrstr, _, _, a_minValue, a_maxValue, a_startValue, _, a_isFixed) => {
            let mut ret_2: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".min = ")).clone() }))?;
            txt = optInitValFMU(txt.clone(), a_minValue.clone(), (literal!("1")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".max = ")).clone() }))?;
            ret_2 = (i_names.clone().len() as i32);
            txt = optInitValFMU(txt.clone(), a_maxValue.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".fixed = ")).clone() }))?;
            txt = fun_648(txt.clone(), a_isFixed.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_attrstr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".start = ")).clone() }))?;
            txt = optInitValFMU(txt.clone(), a_startValue.clone(), (literal!("0")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, i_type__, _, _, _, _, _, _, _, _) => {
            let mut txt_3: Tpl::Text;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_3 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ScalarVariableTypeFMU: ")).clone() }))?;
            ret_3 = (TypesDump::unparseType(i_type__.clone())?).clone();
            txt_3 = Tpl::writeStr(txt_3.clone(), (ret_3.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenFMU.tpl")).clone(), 3415, 16), (Tpl::textString(txt_3.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableTypeFMU(mut txt: Tpl::Text, mut a_attrstr: ArcStr, mut a_unit: ArcStr, mut a_displayUnit: ArcStr, mut a_minValue: Option<Arc<DAE::Exp>>, mut a_maxValue: Option<Arc<DAE::Exp>>, mut a_startValue: Option<Arc<DAE::Exp>>, mut a_nominalValue: Option<Arc<DAE::Exp>>, mut a_isFixed: bool, mut a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_649(txt.clone(), a_type__.clone(), (a_attrstr.clone()).clone(), (a_unit.clone()).clone(), (a_displayUnit.clone()).clone(), a_minValue.clone(), a_maxValue.clone(), a_startValue.clone(), a_nominalValue.clone(), a_isFixed.clone())?;
    Ok(out_txt)
}

