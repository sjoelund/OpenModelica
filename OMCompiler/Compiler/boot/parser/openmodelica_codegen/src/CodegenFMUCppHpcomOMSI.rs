// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenCppHpcomOMSI;
use crate::CodegenCppInit;
use crate::CodegenCppOMSI;
use crate::CodegenFMUCppOMSI;
use openmodelica_ast::Absyn;
use openmodelica_backend::CodegenUtil;
use openmodelica_backend::HpcOmSimCode;
use openmodelica_backend::SimCode;
use openmodelica_backend::SimCodeFunction;
use openmodelica_frontend::Expression;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn translateModel(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUVersion: ArcStr, mut in_a_FMUType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_FMUVersion.clone(), in_a_FMUType.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { varToArrayIndexMapping: ref i_varToArrayIndexMapping, allEquations: ref i_allEquations, fileNamePrefix: ref i_fileNamePrefix, hpcomData: HpcOmSimCode::HpcOmData { schedules: ref i_hpcomData_schedules, hpcOmMemory: ref i_hpcomData_hpcOmMemory }, makefileParams: SimCodeFunction::MakefileParams { ccompiler: _, .. }, modelInfo: ref i_modelInfo @ SimCode::ModelInfo { name: ref i_modelInfo_name, .. }, .. }, mut a_FMUVersion, mut a_FMUType) => {
            let mut txt_46: Tpl::Text;
            let mut txt_45: Tpl::Text;
            let mut txt_44: Tpl::Text;
            let mut ret_43: bool = false;
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
            let mut txt_20: Tpl::Text;
            let mut txt_19: Tpl::Text;
            let mut txt_18: Tpl::Text;
            let mut txt_17: Tpl::Text;
            let mut txt_16: Tpl::Text;
            let mut l_cpp: Tpl::Text;
            let mut txt_14: Tpl::Text;
            let mut l_numPreVars: Tpl::Text;
            let mut l_numStringVars: Tpl::Text;
            let mut l_numBoolVars: Tpl::Text;
            let mut l_numIntVars: Tpl::Text;
            let mut l_numRealVars: Tpl::Text;
            let mut l_className: Tpl::Text;
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
            l_className = CodegenCppOMSI::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_numRealVars = CodegenCppHpcomOMSI::numRealvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numIntVars = CodegenCppHpcomOMSI::numIntvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numBoolVars = CodegenCppHpcomOMSI::numBoolvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numStringVars = CodegenCppHpcomOMSI::numStringvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numPreVars = CodegenCppHpcomOMSI::numPreVarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            (txt_14, l_complexStartExpressions, l_stateDerVectorName) = CodegenCppInit::modelInitXMLFile(Tpl::emptyTxt.clone(), i_simCode.clone(), (Tpl::textString(l_numRealVars.clone())?).clone(), (Tpl::textString(l_numIntVars.clone())?).clone(), (Tpl::textString(l_numBoolVars.clone())?).clone(), (Tpl::textString(l_numStringVars.clone())?).clone(), (a_FMUVersion.clone()).clone(), (a_FMUType.clone()).clone(), (Tpl::textString(l_guid.clone())?).clone(), true, (literal!("hpcom cpp-runtime")).clone(), l_complexStartExpressions.clone(), l_stateDerVectorName.clone())?;
            Tpl::textFile(txt_14.clone(), (literal!("modelDescription.xml")).clone())?;
            l_cpp = CodegenCppOMSI::translateModel(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            (txt_16, l_extraFuncs, l_extraFuncsDecl, _) = CodegenFMUCppOMSI::fmuWriteOutputHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_17 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_17 = Tpl::writeStr(txt_17.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_17 = Tpl::writeTok(txt_17.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput.h")).clone() }))?;
            Tpl::textFile(txt_16.clone(), (Tpl::textString(txt_17.clone())?).clone())?;
            (txt_18, l_extraFuncs, l_extraFuncsDecl, _) = CodegenFMUCppOMSI::fmuModelHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(l_guid.clone())?).clone(), (a_FMUVersion.clone()).clone())?;
            txt_19 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_19 = Tpl::writeStr(txt_19.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_19 = Tpl::writeTok(txt_19.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU.h")).clone() }))?;
            Tpl::textFile(txt_18.clone(), (Tpl::textString(txt_19.clone())?).clone())?;
            (txt_20, l_extraFuncs, l_extraFuncsDecl, _) = CodegenFMUCppOMSI::fmuModelCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(l_guid.clone())?).clone(), (a_FMUVersion.clone()).clone())?;
            txt_21 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_21 = Tpl::writeStr(txt_21.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_21 = Tpl::writeTok(txt_21.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FMU.cpp")).clone() }))?;
            Tpl::textFile(txt_20.clone(), (Tpl::textString(txt_21.clone())?).clone())?;
            (txt_22, l_extraFuncs, l_extraFuncsDecl, _) = fmuMakefile(Tpl::emptyTxt.clone(), (Tpl::textString(l_target.clone())?).clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (a_FMUVersion.clone()).clone())?;
            txt_23 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_23 = Tpl::writeTok(txt_23.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_FMU.makefile")).clone() }))?;
            Tpl::textFile(txt_22.clone(), (Tpl::textString(txt_23.clone())?).clone())?;
            txt_24 = CodegenFMUCppOMSI::fmuCalcHelperMainfile(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt_25 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_25 = Tpl::writeStr(txt_25.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_25 = Tpl::writeTok(txt_25.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CalcHelperMain.cpp")).clone() }))?;
            Tpl::textFile(txt_24.clone(), (Tpl::textString(txt_25.clone())?).clone())?;
            (txt_26, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCppHpcomOMSI::updateHpcom(Tpl::emptyTxt.clone(), i_allEquations.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextOther().clone(), l_stateDerVectorName.clone(), false)?;
            txt_27 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numRealVars.clone())?;
            txt_27 = Tpl::writeTok(txt_27.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_28 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numIntVars.clone())?;
            txt_28 = Tpl::writeTok(txt_28.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_29 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numBoolVars.clone())?;
            txt_29 = Tpl::writeTok(txt_29.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_30 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numStringVars.clone())?;
            txt_30 = Tpl::writeTok(txt_30.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_31 = CodegenCppHpcomOMSI::additionalHpcomConstructorDefinitions(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone())?;
            txt_32 = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            txt_33 = CodegenCppHpcomOMSI::additionalHpcomConstructorBodyStatements(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone(), (Tpl::textString(l_className.clone())?).clone(), (Tpl::textString(txt_32.clone())?).clone())?;
            txt_34 = CodegenCppHpcomOMSI::additionalHpcomDestructorBodyStatements(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone())?;
            (txt_35, txt_26, txt_27, txt_28, txt_29, txt_30, l_extraFuncs, l_extraFuncsDecl, l_className, txt_31, txt_33, txt_34, l_stateDerVectorName) = CodegenCppOMSI::simulationCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), SimCodeFunction::contextOther().clone(), txt_26.clone(), txt_27.clone(), txt_28.clone(), txt_29.clone(), txt_30.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), l_className.clone(), txt_31.clone(), txt_33.clone(), txt_34.clone(), l_stateDerVectorName.clone(), false, (Tpl::textString(l_numRealVars.clone())?).clone(), (Tpl::textString(l_numIntVars.clone())?).clone(), (Tpl::textString(l_numBoolVars.clone())?).clone(), (Tpl::textString(l_numStringVars.clone())?).clone(), (Tpl::textString(l_numPreVars.clone())?).clone())?;
            txt_36 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_36 = Tpl::writeStr(txt_36.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_36 = Tpl::writeTok(txt_36.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".cpp")).clone() }))?;
            Tpl::textFile(txt_35.clone(), (Tpl::textString(txt_36.clone())?).clone())?;
            (txt_37, l_extraFuncs, l_extraFuncsDecl, l_className) = CodegenCppHpcomOMSI::additionalHpcomIncludes(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), l_className.clone(), false)?;
            (txt_38, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCppHpcomOMSI::additionalHpcomProtectedMemberDeclaration(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), false)?;
            txt_39 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numRealVars.clone())?;
            txt_39 = Tpl::writeTok(txt_39.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_40 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numIntVars.clone())?;
            txt_40 = Tpl::writeTok(txt_40.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_41 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numBoolVars.clone())?;
            txt_41 = Tpl::writeTok(txt_41.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_42 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numStringVars.clone())?;
            txt_42 = Tpl::writeTok(txt_42.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            ret_43 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            (txt_44, txt_39, txt_40, txt_41, txt_42) = CodegenCppOMSI::memberVariableDefine(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_varToArrayIndexMapping.clone(), txt_39.clone(), txt_40.clone(), txt_41.clone(), txt_42.clone(), ret_43.clone(), false)?;
            (txt_45, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCppOMSI::simulationHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), SimCodeFunction::contextOther().clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(txt_37.clone())?).clone(), (literal!("")).clone(), (Tpl::textString(txt_38.clone())?).clone(), (Tpl::textString(txt_44.clone())?).clone(), false)?;
            txt_46 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_46 = Tpl::writeStr(txt_46.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_46 = Tpl::writeTok(txt_46.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
            Tpl::textFile(txt_45.clone(), (Tpl::textString(txt_46.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -lboost_thread")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmuMakefile(mut txt: Tpl::Text, mut a_target: ArcStr, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_FMUVersion: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_8: bool = false;
    let mut ret_7: bool = false;
    let mut ret_6: bool = false;
    let mut l_additionalLinkerFlags__MSVC: Tpl::Text;
    let mut l_additionalLinkerFlags__GCC: Tpl::Text;
    let mut l_additionalCFlags__MSVC: Tpl::Text;
    let mut l_additionalCFlags__GCC: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    l_additionalCFlags__GCC = Tpl::emptyTxt.clone();
    l_additionalCFlags__MSVC = Tpl::emptyTxt.clone();
    l_additionalLinkerFlags__GCC = Tpl::emptyTxt.clone();
    l_additionalLinkerFlags__MSVC = Tpl::emptyTxt.clone();
    ret_6 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("pthreads")).clone());
    ret_7 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("pthreads_spin")).clone());
    ret_8 = boolOr(ret_6.clone(), ret_7.clone());
    l_additionalLinkerFlags__GCC = fun_54(l_additionalLinkerFlags__GCC.clone(), ret_8.clone())?;
    (out_txt, l_additionalCFlags__GCC, l_additionalCFlags__MSVC, l_additionalLinkerFlags__GCC, l_additionalLinkerFlags__MSVC) = CodegenCppHpcomOMSI::getAdditionalMakefileFlags(txt.clone(), l_additionalCFlags__GCC.clone(), l_additionalCFlags__MSVC.clone(), l_additionalLinkerFlags__GCC.clone(), l_additionalLinkerFlags__MSVC.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = CodegenFMUCppOMSI::fmuMakefile(out_txt.clone(), (a_target.clone()).clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), (a_FMUVersion.clone()).clone(), (Tpl::textString(l_additionalLinkerFlags__GCC.clone())?).clone(), (Tpl::textString(l_additionalLinkerFlags__MSVC.clone())?).clone(), (Tpl::textString(l_additionalCFlags__GCC.clone())?).clone(), (Tpl::textString(l_additionalCFlags__MSVC.clone())?).clone())?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

