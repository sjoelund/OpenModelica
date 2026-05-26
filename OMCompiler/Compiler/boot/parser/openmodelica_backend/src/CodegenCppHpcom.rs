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
use crate::CodegenUtil;
use crate::HpcOmScheduler;
use crate::HpcOmSimCode;
use crate::HpcOmTaskGraph;
use crate::SimCode;
use crate::SimCodeFunction;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Expression;
use openmodelica_frontend::HashTableCrIListArray;
use openmodelica_frontend::Types;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn fun_52(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <mpi.h>")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = mpiInit(txt.clone())?;
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
            txt = mpiFinalize(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fileNamePrefix: ArcStr, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_className: Tpl::Text, mut in_a_extraFuncsDeclInit: Tpl::Text, mut in_a_extraFuncsInit: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsDeclInit: Tpl::Text;
    let mut out_a_extraFuncsInit: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_extraFuncsDeclInit, out_a_extraFuncsInit) = (match (in_txt.clone(), in_mArg.clone(), in_a_fileNamePrefix.clone(), in_a_stateDerVectorName.clone(), in_a_className.clone(), in_a_extraFuncsDeclInit.clone(), in_a_extraFuncsInit.clone(), in_a_simCode.clone()) {
        (mut txt, true, mut a_fileNamePrefix, mut a_stateDerVectorName, mut a_className, mut a_extraFuncsDeclInit, mut a_extraFuncsInit, mut a_simCode) => {
            let mut txt_5: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeText(Tpl::emptyTxt.clone(), a_className.clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_1, a_extraFuncsInit, a_extraFuncsDeclInit, txt_0, a_stateDerVectorName) = CodegenCpp::simulationInitParameterCppFile(Tpl::emptyTxt.clone(), a_simCode.clone(), a_extraFuncsInit.clone(), a_extraFuncsDeclInit.clone(), txt_0.clone(), a_stateDerVectorName.clone(), false)?;
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_2 = Tpl::writeStr(txt_2.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("InitializeParameter.cpp")).clone() }))?;
            Tpl::textFile(txt_1.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt_3 = Tpl::writeText(Tpl::emptyTxt.clone(), a_className.clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_4, a_extraFuncsInit, a_extraFuncsDeclInit, txt_3, a_stateDerVectorName) = CodegenCpp::simulationInitAlgVarsCppFile(Tpl::emptyTxt.clone(), a_simCode.clone(), a_extraFuncsInit.clone(), a_extraFuncsDeclInit.clone(), txt_3.clone(), a_stateDerVectorName.clone(), false)?;
            txt_5 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_5 = Tpl::writeStr(txt_5.clone(), (a_fileNamePrefix.clone()).clone())?;
            txt_5 = Tpl::writeTok(txt_5.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("InitializeAlgVars.cpp")).clone() }))?;
            Tpl::textFile(txt_4.clone(), (Tpl::textString(txt_5.clone())?).clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDeclInit.clone(), a_extraFuncsInit.clone())
        },
        (mut txt, _, _, mut a_stateDerVectorName, _, mut a_extraFuncsDeclInit, mut a_extraFuncsInit, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDeclInit.clone(), a_extraFuncsInit.clone())
        },
    });
    Ok((out_txt, out_a_stateDerVectorName, out_a_extraFuncsDeclInit, out_a_extraFuncsInit))
}

fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::JacobianColumn>>>, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stateDerVectorName.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianColumn { columnEqns: i_eqs, .. }, tail: rest }, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, a_simCode) => {
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            (txt, a_extraFuncs, a_extraFuncsDecl, _, a_stateDerVectorName) = CodegenCpp::algloopfiles(txt.clone(), i_eqs.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloopJacobian().clone(), 0, a_stateDerVectorName.clone(), false)?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = lm_56(txt.clone(), rest.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, a_simCode) => {
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = lm_56(txt.clone(), rest.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs))
}

fn lm_57(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::JacobianMatrix>>>, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stateDerVectorName.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SimCode::JacobianMatrix { columns: i_mat, .. }, tail: rest }, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, a_simCode) => {
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = lm_56(txt.clone(), i_mat.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = lm_57(txt.clone(), rest.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, a_simCode) => {
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = lm_57(txt.clone(), rest.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs))
}

fn fun_58(mut in_txt: Tpl::Text, mut in_a_subPartition: SimCode::SubPartition, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_i: i32, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs) = (match (in_txt.clone(), in_a_subPartition.clone(), in_a_stateDerVectorName.clone(), in_a_i.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SubPartition { removedEquations: ref i_removedEquations, equations: ref i_equations, .. }, mut a_stateDerVectorName, mut a_i, mut a_extraFuncsDecl, mut a_extraFuncs, mut a_simCode) => {
            let mut ret_0: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            ret_0 = listAppend(i_equations.clone(), i_removedEquations.clone());
            (txt, a_extraFuncs, a_extraFuncsDecl, _, a_stateDerVectorName) = CodegenCpp::algloopfiles(txt.clone(), ret_0.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloop().clone(), a_i.clone(), a_stateDerVectorName.clone(), false)?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (mut txt, _, mut a_stateDerVectorName, _, mut a_extraFuncsDecl, mut a_extraFuncs, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
    });
    Ok((out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs))
}

fn lm_59(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCode::SubPartition>>, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    (out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stateDerVectorName.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, _) => {
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_subPartition, tail: rest }, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs, a_simCode) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = fun_58(txt.clone(), i_subPartition.clone(), a_stateDerVectorName.clone(), x_i.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_stateDerVectorName, a_extraFuncsDecl, a_extraFuncs) = lm_59(txt.clone(), rest.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone())?;
            (txt.clone(), a_stateDerVectorName.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_stateDerVectorName, out_a_extraFuncsDecl, out_a_extraFuncs))
}

pub fn translateModel(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { clockedPartitions: ref i_clockedPartitions, initialEquations: ref i_initialEquations, jacobianMatrices: ref i_jacobianMatrices, externalFunctionIncludes: ref i_externalFunctionIncludes, literals: ref i_literals, varToArrayIndexMapping: ref i_varToArrayIndexMapping, allEquations: ref i_allEquations, fileNamePrefix: ref i_fileNamePrefix, hpcomData: HpcOmSimCode::HpcOmData { schedules: ref i_hpcomData_schedules, hpcOmMemory: ref i_hpcomData_hpcOmMemory }, makefileParams: SimCodeFunction::MakefileParams { ccompiler: _, .. }, modelInfo: ref i_modelInfo @ SimCode::ModelInfo { functions: ref i_modelInfo_functions, name: ref i_modelInfo_name, .. }, .. }) => {
            let mut txt_100: Tpl::Text;
            let mut txt_99: Tpl::Text;
            let mut txt_98: Tpl::Text;
            let mut txt_97: Tpl::Text;
            let mut ret_96: Arc<metamodelica::List<SimCode::SubPartition>> = metamodelica::nil();
            let mut l_clk: Tpl::Text;
            let mut ret_94: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut l_alg: Tpl::Text;
            let mut l_jac: Tpl::Text;
            let mut txt_91: Tpl::Text;
            let mut txt_90: Tpl::Text;
            let mut txt_89: Tpl::Text;
            let mut txt_88: Tpl::Text;
            let mut txt_87: Tpl::Text;
            let mut txt_86: Tpl::Text;
            let mut txt_85: Tpl::Text;
            let mut txt_84: Tpl::Text;
            let mut txt_83: Tpl::Text;
            let mut txt_82: Tpl::Text;
            let mut txt_81: Tpl::Text;
            let mut txt_80: Tpl::Text;
            let mut txt_79: Tpl::Text;
            let mut txt_78: Tpl::Text;
            let mut txt_77: Tpl::Text;
            let mut txt_76: Tpl::Text;
            let mut txt_75: Tpl::Text;
            let mut txt_74: Tpl::Text;
            let mut txt_73: Tpl::Text;
            let mut txt_72: Tpl::Text;
            let mut txt_71: Tpl::Text;
            let mut ret_70: bool = false;
            let mut l_jacobianVarsInit: Tpl::Text;
            let mut txt_68: Tpl::Text;
            let mut txt_67: Tpl::Text;
            let mut txt_66: Tpl::Text;
            let mut ret_65: bool = false;
            let mut ret_64: bool = false;
            let mut ret_63: bool = false;
            let mut l_0__: Tpl::Text;
            let mut txt_61: Tpl::Text;
            let mut txt_60: Tpl::Text;
            let mut txt_59: Tpl::Text;
            let mut txt_58: Tpl::Text;
            let mut txt_57: Tpl::Text;
            let mut l_complexStartExpressions: Tpl::Text;
            let mut l_extraFuncsDeclInit: Tpl::Text;
            let mut l_extraFuncsInit: Tpl::Text;
            let mut txt_53: Tpl::Text;
            let mut txt_52: Tpl::Text;
            let mut txt_51: Tpl::Text;
            let mut txt_50: Tpl::Text;
            let mut l_extraFuncsDeclFun: Tpl::Text;
            let mut l_extraFuncsFun: Tpl::Text;
            let mut txt_47: Tpl::Text;
            let mut txt_46: Tpl::Text;
            let mut txt_45: Tpl::Text;
            let mut txt_44: Tpl::Text;
            let mut txt_43: Tpl::Text;
            let mut txt_42: Tpl::Text;
            let mut txt_41: Tpl::Text;
            let mut ret_40: bool = false;
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
            let mut txt_19: Tpl::Text;
            let mut ret_19: bool = false;
            let mut txt_17: Tpl::Text;
            let mut ret_17: bool = false;
            let mut txt_15: Tpl::Text;
            let mut ret_15: bool = false;
            let mut l_numPreVars: Tpl::Text;
            let mut l_numStringVars: Tpl::Text;
            let mut l_numBoolVars: Tpl::Text;
            let mut l_numIntVars: Tpl::Text;
            let mut l_numRealVars: Tpl::Text;
            let mut l_className: Tpl::Text;
            let mut ret_8: bool = false;
            let mut l_useMemoryOptimization: Tpl::Text;
            let mut l_stateDerVectorName: Tpl::Text;
            let mut l_dummyTypeElemCreation: Tpl::Text;
            let mut l_extraResidualsFuncsDecl: Tpl::Text;
            let mut l_extraFuncsDecl: Tpl::Text;
            let mut l_extraFuncs: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_target: Tpl::Text;
            ret_1 = (Config::simulationCodeTarget()?).clone();
            l_target = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            l_extraFuncs = Tpl::emptyTxt.clone();
            l_extraFuncsDecl = Tpl::emptyTxt.clone();
            l_extraResidualsFuncsDecl = Tpl::emptyTxt.clone();
            l_dummyTypeElemCreation = Tpl::emptyTxt.clone();
            l_stateDerVectorName = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__zDot")).clone() }))?;
            ret_8 = Flags::isSet(Flags::HPCOM_MEMORY_OPT.clone())?;
            l_useMemoryOptimization = Tpl::writeStr(Tpl::emptyTxt.clone(), (Tpl::booleanString(ret_8.clone())).clone())?;
            l_className = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            l_numRealVars = numRealvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numIntVars = numIntvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numBoolVars = numBoolvarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            l_numStringVars = CodegenCpp::numStringvars(Tpl::emptyTxt.clone(), i_modelInfo.clone())?;
            l_numPreVars = numPreVarsHpcom(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_hpcomData_hpcOmMemory.clone())?;
            ret_15 = Flags::isSet(Flags::USEMPI.clone())?;
            txt_15 = fun_52(Tpl::emptyTxt.clone(), ret_15.clone())?;
            ret_17 = Flags::isSet(Flags::USEMPI.clone())?;
            txt_17 = fun_53(Tpl::emptyTxt.clone(), ret_17.clone())?;
            ret_19 = Flags::isSet(Flags::USEMPI.clone())?;
            txt_19 = fun_54(Tpl::emptyTxt.clone(), ret_19.clone())?;
            (txt_21, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationMainFile(Tpl::emptyTxt.clone(), (Tpl::textString(l_target.clone())?).clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(txt_15.clone())?).clone(), (Tpl::textString(txt_17.clone())?).clone(), (Tpl::textString(txt_19.clone())?).clone(), (Tpl::textString(l_numRealVars.clone())?).clone(), (Tpl::textString(l_numIntVars.clone())?).clone(), (Tpl::textString(l_numBoolVars.clone())?).clone(), (Tpl::textString(l_numStringVars.clone())?).clone(), (Tpl::textString(l_numPreVars.clone())?).clone())?;
            txt_22 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_22 = Tpl::writeStr(txt_22.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_22 = Tpl::writeTok(txt_22.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Main.cpp")).clone() }))?;
            Tpl::textFile(txt_21.clone(), (Tpl::textString(txt_22.clone())?).clone())?;
            (txt_23, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = updateHpcom(Tpl::emptyTxt.clone(), i_allEquations.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextOther().clone(), l_stateDerVectorName.clone(), false)?;
            txt_24 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numRealVars.clone())?;
            txt_24 = Tpl::writeTok(txt_24.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_25 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numIntVars.clone())?;
            txt_25 = Tpl::writeTok(txt_25.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_26 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numBoolVars.clone())?;
            txt_26 = Tpl::writeTok(txt_26.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_27 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numStringVars.clone())?;
            txt_27 = Tpl::writeTok(txt_27.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_28 = additionalHpcomConstructorDefinitions(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone())?;
            txt_29 = CodegenUtil::dotPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            txt_30 = additionalHpcomConstructorBodyStatements(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone(), (Tpl::textString(l_className.clone())?).clone(), (Tpl::textString(txt_29.clone())?).clone())?;
            txt_31 = additionalHpcomDestructorBodyStatements(Tpl::emptyTxt.clone(), i_hpcomData_schedules.clone())?;
            (txt_32, txt_23, txt_24, txt_25, txt_26, txt_27, l_extraFuncs, l_extraFuncsDecl, l_className, txt_28, txt_30, txt_31, l_stateDerVectorName) = CodegenCpp::simulationCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), SimCodeFunction::contextOther().clone(), txt_23.clone(), txt_24.clone(), txt_25.clone(), txt_26.clone(), txt_27.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), l_className.clone(), txt_28.clone(), txt_30.clone(), txt_31.clone(), l_stateDerVectorName.clone(), false)?;
            txt_33 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_33 = Tpl::writeStr(txt_33.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_33 = Tpl::writeTok(txt_33.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".cpp")).clone() }))?;
            Tpl::textFile(txt_32.clone(), (Tpl::textString(txt_33.clone())?).clone())?;
            (txt_34, l_extraFuncs, l_extraFuncsDecl, l_className) = additionalHpcomIncludes(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), l_className.clone(), false)?;
            (txt_35, l_extraFuncs, l_extraFuncsDecl, _) = additionalHpcomProtectedMemberDeclaration(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), false)?;
            txt_36 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numRealVars.clone())?;
            txt_36 = Tpl::writeTok(txt_36.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_37 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numIntVars.clone())?;
            txt_37 = Tpl::writeTok(txt_37.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_38 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numBoolVars.clone())?;
            txt_38 = Tpl::writeTok(txt_38.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            txt_39 = Tpl::writeText(Tpl::emptyTxt.clone(), l_numStringVars.clone())?;
            txt_39 = Tpl::writeTok(txt_39.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-1")).clone() }))?;
            ret_40 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            (txt_41, txt_36, txt_37, txt_38, txt_39) = CodegenCpp::memberVariableDefine(Tpl::emptyTxt.clone(), i_modelInfo.clone(), i_varToArrayIndexMapping.clone(), txt_36.clone(), txt_37.clone(), txt_38.clone(), txt_39.clone(), ret_40.clone(), false)?;
            (txt_42, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), SimCodeFunction::contextOther().clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), (Tpl::textString(txt_34.clone())?).clone(), (literal!("")).clone(), (Tpl::textString(txt_35.clone())?).clone(), (Tpl::textString(txt_41.clone())?).clone(), false)?;
            txt_43 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_43 = Tpl::writeStr(txt_43.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_43 = Tpl::writeTok(txt_43.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".h")).clone() }))?;
            Tpl::textFile(txt_42.clone(), (Tpl::textString(txt_43.clone())?).clone())?;
            (txt_44, l_extraFuncs, l_extraFuncsDecl, _, l_dummyTypeElemCreation, l_stateDerVectorName) = CodegenCpp::simulationTypesHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_dummyTypeElemCreation.clone(), i_modelInfo_functions.clone(), i_literals.clone(), l_stateDerVectorName.clone(), false)?;
            txt_45 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_45 = Tpl::writeStr(txt_45.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_45 = Tpl::writeTok(txt_45.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Types.h")).clone() }))?;
            Tpl::textFile(txt_44.clone(), (Tpl::textString(txt_45.clone())?).clone())?;
            (txt_46, l_extraFuncs, l_extraFuncsDecl, _) = simulationMakefile(Tpl::emptyTxt.clone(), (Tpl::textString(l_target.clone())?).clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_47 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_47 = Tpl::writeTok(txt_47.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".makefile")).clone() }))?;
            Tpl::textFile(txt_46.clone(), (Tpl::textString(txt_47.clone())?).clone())?;
            l_extraFuncsFun = Tpl::emptyTxt.clone();
            l_extraFuncsDeclFun = Tpl::emptyTxt.clone();
            (txt_50, l_extraFuncsFun, l_extraFuncsDeclFun, _, l_stateDerVectorName) = CodegenCpp::simulationFunctionsHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsFun.clone(), l_extraFuncsDeclFun.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), i_modelInfo_functions.clone(), i_literals.clone(), l_stateDerVectorName.clone(), false)?;
            txt_51 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_51 = Tpl::writeStr(txt_51.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_51 = Tpl::writeTok(txt_51.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Functions.h")).clone() }))?;
            Tpl::textFile(txt_50.clone(), (Tpl::textString(txt_51.clone())?).clone())?;
            (txt_52, l_extraFuncsFun, l_extraFuncsDeclFun, _, l_stateDerVectorName) = CodegenCpp::simulationFunctionsFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsFun.clone(), l_extraFuncsDeclFun.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), i_modelInfo_functions.clone(), i_literals.clone(), i_externalFunctionIncludes.clone(), l_stateDerVectorName.clone(), false)?;
            txt_53 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_53 = Tpl::writeStr(txt_53.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_53 = Tpl::writeTok(txt_53.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Functions.cpp")).clone() }))?;
            Tpl::textFile(txt_52.clone(), (Tpl::textString(txt_53.clone())?).clone())?;
            l_extraFuncsInit = Tpl::emptyTxt.clone();
            l_extraFuncsDeclInit = Tpl::emptyTxt.clone();
            l_complexStartExpressions = Tpl::emptyTxt.clone();
            (txt_57, l_complexStartExpressions, l_stateDerVectorName) = CodegenCppInit::modelInitXMLFile(Tpl::emptyTxt.clone(), i_simCode.clone(), (Tpl::textString(l_numRealVars.clone())?).clone(), (Tpl::textString(l_numIntVars.clone())?).clone(), (Tpl::textString(l_numBoolVars.clone())?).clone(), (Tpl::textString(l_numStringVars.clone())?).clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), false, (literal!("")).clone(), l_complexStartExpressions.clone(), l_stateDerVectorName.clone())?;
            txt_58 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_58 = Tpl::writeTok(txt_58.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_init.xml")).clone() }))?;
            Tpl::textFile(txt_57.clone(), (Tpl::textString(txt_58.clone())?).clone())?;
            txt_59 = Tpl::writeText(Tpl::emptyTxt.clone(), l_className.clone())?;
            txt_59 = Tpl::writeTok(txt_59.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_60, l_extraFuncsInit, l_extraFuncsDeclInit, txt_59, l_dummyTypeElemCreation, l_stateDerVectorName, l_complexStartExpressions) = CodegenCpp::simulationInitCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsInit.clone(), l_extraFuncsDeclInit.clone(), txt_59.clone(), l_dummyTypeElemCreation.clone(), l_stateDerVectorName.clone(), false, l_complexStartExpressions.clone())?;
            txt_61 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_61 = Tpl::writeStr(txt_61.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_61 = Tpl::writeTok(txt_61.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize.cpp")).clone() }))?;
            Tpl::textFile(txt_60.clone(), (Tpl::textString(txt_61.clone())?).clone())?;
            ret_63 = Flags::isSet(Flags::HARDCODED_START_VALUES.clone())?;
            ret_64 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            ret_65 = boolOr(ret_63.clone(), ret_64.clone());
            (l_0__, l_stateDerVectorName, l_extraFuncsDeclInit, l_extraFuncsInit) = fun_55(Tpl::emptyTxt.clone(), ret_65.clone(), (i_fileNamePrefix.clone()).clone(), l_stateDerVectorName.clone(), l_className.clone(), l_extraFuncsDeclInit.clone(), l_extraFuncsInit.clone(), i_simCode.clone())?;
            txt_66 = Tpl::writeText(Tpl::emptyTxt.clone(), l_className.clone())?;
            txt_66 = Tpl::writeTok(txt_66.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize")).clone() }))?;
            (txt_67, l_extraFuncsInit, l_extraFuncsDeclInit, txt_66) = CodegenCpp::simulationInitHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncsInit.clone(), l_extraFuncsDeclInit.clone(), txt_66.clone())?;
            txt_68 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_68 = Tpl::writeStr(txt_68.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_68 = Tpl::writeTok(txt_68.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Initialize.h")).clone() }))?;
            Tpl::textFile(txt_67.clone(), (Tpl::textString(txt_68.clone())?).clone())?;
            l_jacobianVarsInit = Tpl::emptyTxt.clone();
            ret_70 = Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?;
            (txt_71, l_extraFuncs, l_extraFuncsDecl, _, l_jacobianVarsInit) = CodegenCpp::simulationJacobianHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_jacobianVarsInit.clone(), ret_70.clone())?;
            txt_72 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_72 = Tpl::writeStr(txt_72.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_72 = Tpl::writeTok(txt_72.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Jacobian.h")).clone() }))?;
            Tpl::textFile(txt_71.clone(), (Tpl::textString(txt_72.clone())?).clone())?;
            (txt_73, l_extraFuncs, l_extraFuncsDecl, _, l_jacobianVarsInit, l_stateDerVectorName) = CodegenCpp::simulationJacobianCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_jacobianVarsInit.clone(), l_stateDerVectorName.clone(), false)?;
            txt_74 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_74 = Tpl::writeStr(txt_74.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_74 = Tpl::writeTok(txt_74.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Jacobian.cpp")).clone() }))?;
            Tpl::textFile(txt_73.clone(), (Tpl::textString(txt_74.clone())?).clone())?;
            (txt_75, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::simulationStateSelectionCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_stateDerVectorName.clone(), false)?;
            txt_76 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_76 = Tpl::writeStr(txt_76.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_76 = Tpl::writeTok(txt_76.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelection.cpp")).clone() }))?;
            Tpl::textFile(txt_75.clone(), (Tpl::textString(txt_76.clone())?).clone())?;
            (txt_77, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationStateSelectionHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_78 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_78 = Tpl::writeStr(txt_78.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_78 = Tpl::writeTok(txt_78.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelection.h")).clone() }))?;
            Tpl::textFile(txt_77.clone(), (Tpl::textString(txt_78.clone())?).clone())?;
            (txt_79, l_extraResidualsFuncsDecl, l_className, l_stateDerVectorName) = CodegenCpp::updateResiduals(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraResidualsFuncsDecl.clone(), l_className.clone(), l_stateDerVectorName.clone(), false)?;
            (txt_80, txt_79, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::simulationMixedSystemCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), txt_79.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_stateDerVectorName.clone(), false)?;
            txt_81 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_81 = Tpl::writeStr(txt_81.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_81 = Tpl::writeTok(txt_81.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Mixed.cpp")).clone() }))?;
            Tpl::textFile(txt_80.clone(), (Tpl::textString(txt_81.clone())?).clone())?;
            (txt_82, l_extraResidualsFuncsDecl) = CodegenCpp::simulationMixedSystemHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraResidualsFuncsDecl.clone())?;
            txt_83 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_83 = Tpl::writeStr(txt_83.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_83 = Tpl::writeTok(txt_83.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Mixed.h")).clone() }))?;
            Tpl::textFile(txt_82.clone(), (Tpl::textString(txt_83.clone())?).clone())?;
            (txt_84, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationWriteOutputHeaderFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_85 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_85 = Tpl::writeStr(txt_85.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_85 = Tpl::writeTok(txt_85.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput.h")).clone() }))?;
            Tpl::textFile(txt_84.clone(), (Tpl::textString(txt_85.clone())?).clone())?;
            (txt_86, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::simulationWriteOutputCppFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), l_stateDerVectorName.clone(), false)?;
            txt_87 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_87 = Tpl::writeStr(txt_87.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_87 = Tpl::writeTok(txt_87.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WriteOutput.cpp")).clone() }))?;
            Tpl::textFile(txt_86.clone(), (Tpl::textString(txt_87.clone())?).clone())?;
            (txt_88, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationFactoryFile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_89 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_89 = Tpl::writeStr(txt_89.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_89 = Tpl::writeTok(txt_89.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FactoryExport.cpp")).clone() }))?;
            Tpl::textFile(txt_88.clone(), (Tpl::textString(txt_89.clone())?).clone())?;
            (txt_90, l_extraFuncs, l_extraFuncsDecl, _) = simulationMainRunScript(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_91 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_fileNamePrefix.clone()).clone())?;
            (txt_91, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::simulationMainRunScriptSuffix(txt_91.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            Tpl::textFile(txt_90.clone(), (Tpl::textString(txt_91.clone())?).clone())?;
            l_jac = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_jac, l_stateDerVectorName, l_extraFuncsDecl, l_extraFuncs) = lm_57(l_jac.clone(), i_jacobianMatrices.clone(), l_stateDerVectorName.clone(), l_extraFuncsDecl.clone(), l_extraFuncs.clone(), i_simCode.clone())?;
            l_jac = Tpl::popIter(l_jac.clone())?;
            ret_94 = listAppend(i_allEquations.clone(), i_initialEquations.clone());
            (l_alg, l_extraFuncs, l_extraFuncsDecl, _, l_stateDerVectorName) = CodegenCpp::algloopfiles(Tpl::emptyTxt.clone(), ret_94.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloop().clone(), 0, l_stateDerVectorName.clone(), false)?;
            ret_96 = SimCodeUtil::getSubPartitions(i_clockedPartitions.clone());
            l_clk = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_clk, l_stateDerVectorName, l_extraFuncsDecl, l_extraFuncs) = lm_59(l_clk.clone(), ret_96.clone(), l_stateDerVectorName.clone(), l_extraFuncsDecl.clone(), l_extraFuncs.clone(), i_simCode.clone())?;
            l_clk = Tpl::popIter(l_clk.clone())?;
            (txt_97, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::algloopMainfile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), SimCodeFunction::contextAlgloop().clone())?;
            txt_98 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_98 = Tpl::writeStr(txt_98.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_98 = Tpl::writeTok(txt_98.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("AlgLoopMain.cpp")).clone() }))?;
            Tpl::textFile(txt_97.clone(), (Tpl::textString(txt_98.clone())?).clone())?;
            (txt_99, l_extraFuncs, l_extraFuncsDecl, _) = CodegenCpp::calcHelperMainfile(Tpl::emptyTxt.clone(), i_simCode.clone(), l_extraFuncs.clone(), l_extraFuncsDecl.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })))?;
            txt_100 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OMCpp")).clone() }))?;
            txt_100 = Tpl::writeStr(txt_100.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt_100 = Tpl::writeTok(txt_100.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CalcHelperMain.cpp")).clone() }))?;
            Tpl::textFile(txt_99.clone(), (Tpl::textString(txt_100.clone())?).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_61(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (match (in_txt.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone()) {
        (mut txt, mut i_simCode @ SimCode::SimCode { modelInfo: _, .. }, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace) => {
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = additionalHpcomIncludesForParallelCode(txt.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (mut txt, _, mut a_extraFuncs, mut a_extraFuncsDecl, mut a_extraFuncsNamespace) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
    });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn additionalHpcomIncludes(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_61(txt.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_63(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "openmp") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <omp.h>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads") => {
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin") => {
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#include <tbb/tbb.h>\n")).clone(), (literal!("#include <tbb/flow_graph.h>\n")).clone(), (literal!("#include <tbb/tbb_stddef.h>\n")).clone(), (literal!("#include <boost/function.hpp>\n")).clone(), (literal!("#include <boost/bind.hpp>\n")).clone(), (literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("#include <tbb/task_arena.h>\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ "mpi") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include <mpi.h>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#include <boost/thread/mutex.hpp>\n")).clone(), (literal!("#include <boost/thread.hpp>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn additionalHpcomIncludesForParallelCode(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut str_2: ArcStr = arcstr::literal!("");
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    str_2 = (Tpl::textString(l_type.clone())?).clone();
    out_txt = fun_63(txt.clone(), (str_2.clone()).clone())?;
    out_a_extraFuncs = a_extraFuncs.clone();
    out_a_extraFuncsDecl = a_extraFuncsDecl.clone();
    out_a_extraFuncsNamespace = a_extraFuncsNamespace.clone();
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_65(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "openmp") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return (long unsigned int)omp_get_thread_num();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "mpi") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return -1; //not supported")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return -1; //not supported")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#if defined(USE_THREAD)\n")).clone(), (literal!("  #if !defined(USE_CPP_03)\n")).clone(), (literal!("    return std::hash<std::thread::id>()(std::this_thread::get_id());\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    boost::hash<std::string> string_hash;\n")).clone(), (literal!("    return (long unsigned int)string_hash(boost::lexical_cast<std::string>(boost::this_thread::get_id()));\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("#else\n")).clone(), (literal!("  return 0;\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_66(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MeasureTimeValues* measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_67(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MeasureTimeValues* measuredSchedulerEndValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_67(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            let mut ret_3: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_2: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("std::vector<MeasureTimeData*> *measureTimeArrayHpcom;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeSchedulerArrayHpcom_evaluateODE;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeSchedulerArrayHpcom_evaluateDAE;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeSchedulerArrayHpcom_evaluateZeroFuncs;\n")).clone(), (literal!("//MeasureTimeValues *measuredStartValuesODE, *measuredEndValuesODE;\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerStartValues, *measuredSchedulerEndValues;\n")).clone(), (literal!("\n")).clone(), (literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeThreadArrayOdeHpcom;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeThreadArrayDaeHpcom;\n")).clone(), (literal!("std::vector<MeasureTimeData*> *measureTimeThreadArrayZeroFuncHpcom;\n")).clone()], lastHasNewLine: true }))?;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_1 = List::intRange(ret_0.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_66(txt.clone(), ret_1.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_2 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_3 = List::intRange(ret_2.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_67(txt.clone(), ret_3.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_69(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncsDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl) = (match (in_txt.clone(), in_a_simCode.clone(), in_a_extraFuncsDecl.clone()) {
        (mut txt, SimCode::SimCode { hpcomData: HpcOmSimCode::HpcOmData { schedules: mut i_hpcomData_schedules, .. }, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }, mut a_extraFuncsDecl) => {
            let mut ret_5: bool = false;
            let mut ret_4: bool = false;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut str_2: ArcStr = arcstr::literal!("");
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_type: Tpl::Text;
            a_extraFuncsDecl = generateAdditionalFunctionHeaders(a_extraFuncsDecl.clone(), i_hpcomData_schedules.clone())?;
            a_extraFuncsDecl = generateAdditionalHpcomVarHeaders(a_extraFuncsDecl.clone(), i_hpcomData_schedules.clone())?;
            ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
            l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("static long unsigned int getThreadNumber()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            str_2 = (Tpl::textString(l_type.clone())?).clone();
            txt = fun_65(txt.clone(), (str_2.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_4 = stringEq((ret_3.clone()).clone(), (literal!("none")).clone());
            ret_5 = boolNot(ret_4.clone());
            txt = fun_68(txt.clone(), ret_5.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone())
        },
        (mut txt, _, mut a_extraFuncsDecl) => {
            (txt.clone(), a_extraFuncsDecl.clone())
        },
    });
    Ok((out_txt, out_a_extraFuncsDecl))
}

pub fn additionalHpcomProtectedMemberDeclaration(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl) = fun_69(txt.clone(), a_simCode.clone(), a_extraFuncsDecl.clone())?;
    out_a_extraFuncs = a_extraFuncs.clone();
    out_a_extraFuncsNamespace = a_extraFuncsNamespace.clone();
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_71(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "openmp") => {
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("//Required for Intel TBB\n")).clone(), (literal!("struct VoidFunctionBody {\n")).clone(), (literal!("  function<void(void)> void_function;\n")).clone(), (literal!("  VoidFunctionBody(function<void(void)> void_function) : void_function(void_function) { }\n")).clone(), (literal!("  FORCE_INLINE void operator()( tbb::flow::continue_msg ) const\n")).clone(), (literal!("  {\n")).clone(), (literal!("    void_function();\n")).clone(), (literal!("  }\n")).clone(), (literal!("};\n")).clone(), (literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("struct TbbArenaFunctor\n")).clone(), (literal!("{\n")).clone(), (literal!("  tbb::flow::graph * g;\n")).clone(), (literal!("  tbb::flow::broadcast_node<tbb::flow::continue_msg> * sn;\n")).clone(), (literal!("\n")).clone(), (literal!("  TbbArenaFunctor( )\n")).clone(), (literal!("  {\n")).clone(), (literal!("    g = NULL;\n")).clone(), (literal!("    sn = NULL;\n")).clone(), (literal!("  }\n")).clone(), (literal!("\n")).clone(), (literal!("  TbbArenaFunctor( tbb::flow::graph & in_g , tbb::flow::broadcast_node<tbb::flow::continue_msg> & in_sn )\n")).clone(), (literal!("  {\n")).clone(), (literal!("    g = &in_g;\n")).clone(), (literal!("    sn = &in_sn;\n")).clone(), (literal!("  }\n")).clone(), (literal!("\n")).clone(), (literal!("  void operator()()\n")).clone(), (literal!("  {\n")).clone(), (literal!("    sn->try_put( tbb::flow::continue_msg() );\n")).clone(), (literal!("    g->wait_for_all();\n")).clone(), (literal!("  }\n")).clone(), (literal!("\n")).clone(), (literal!("};\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_odeSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_odeSchedule.clone(), in_a_type.clone())) {
        (txt, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ }, a_type) => {
            let mut str_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_71(txt.clone(), (str_0.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateAdditionalStructHeaders(mut txt: Tpl::Text, mut a_odeSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    out_txt = fun_72(txt.clone(), a_odeSchedule.clone(), l_type.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_74(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadFunctionHeaderDecl(txt.clone(), x_i0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_74(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_75(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_odeSchedule_threadTasks.clone())) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_odeSchedule_threadTasks) => {
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_headers: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_2 = listRest(ret_1.clone())?;
            l_headers = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_headers = lm_74(l_headers.clone(), ret_2.clone())?;
            l_headers = Tpl::popIter(l_headers.clone())?;
            txt = Tpl::writeText(txt.clone(), l_headers.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_76(mut in_txt: Tpl::Text, mut in_a_task: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_task.clone())) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void taskFuncOde_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
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
fn lm_77(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_task, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_76(txt.clone(), i_task.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_77(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_a_task: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_task.clone())) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void taskFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
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
fn lm_79(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_task, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_78(txt.clone(), i_task.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_79(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_task: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_task.clone())) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void taskFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
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
fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_task, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_80(txt.clone(), i_task.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_81(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_odeSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_odeSchedule.clone(), in_a_zeroFuncSchedule_tasks.clone(), in_a_daeSchedule_tasks.clone(), in_a_odeSchedule_tasks.clone())) {
        (txt, Deref @ "openmp", _, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ "tbb", a_odeSchedule, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_odeSchedule_tasks) => {
            let mut l_voidfuncsZeroFunc: Tpl::Text;
            let mut l_voidfuncsDae: Tpl::Text;
            let mut l_voidfuncsOde: Tpl::Text;
            let mut txt = (*txt).clone();
            l_voidfuncsOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_voidfuncsOde = lm_77(l_voidfuncsOde.clone(), a_odeSchedule_tasks.clone())?;
            l_voidfuncsOde = Tpl::popIter(l_voidfuncsOde.clone())?;
            l_voidfuncsDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_voidfuncsDae = lm_79(l_voidfuncsDae.clone(), a_daeSchedule_tasks.clone())?;
            l_voidfuncsDae = Tpl::popIter(l_voidfuncsDae.clone())?;
            l_voidfuncsZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_voidfuncsZeroFunc = lm_81(l_voidfuncsZeroFunc.clone(), a_zeroFuncSchedule_tasks.clone())?;
            l_voidfuncsZeroFunc = Tpl::popIter(l_voidfuncsZeroFunc.clone())?;
            txt = generateAdditionalStructHeaders(txt.clone(), a_odeSchedule.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_voidfuncsOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_voidfuncsDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_voidfuncsZeroFunc.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_schedulesOpt.clone(), in_a_type.clone())) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_odeSchedule_threadTasks, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_75(txt.clone(), (str_0.clone()).clone(), i_odeSchedule_threadTasks.clone())?;
            txt.clone()
        },
        (txt, Some((i_odeSchedule @ Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), a_type) => {
            let mut str_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_1 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_82(txt.clone(), (str_1.clone()).clone(), i_odeSchedule.clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), i_odeSchedule_tasks.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateAdditionalFunctionHeaders(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("FORCE_INLINE void evaluateParallel(const UPDATETYPE command, int evaluateMode);\n")).clone() }))?;
    out_txt = fun_83(out_txt.clone(), a_schedulesOpt.clone(), l_type.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_85(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_85(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_86(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_86(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_type.clone())) {
        (txt, Deref @ "pthreads", a_type) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_1 = List::intRange(ret_0.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_85(txt.clone(), ret_1.clone(), a_type.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_2 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = createBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_2.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (literal!("measureTimeArrayLock")).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("bool _simulationFinished;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_type) => {
            let mut ret_5: i32 = 0;
            let mut ret_4: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_3: i32 = 0;
            let mut txt = (*txt).clone();
            ret_3 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_4 = List::intRange(ret_3.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_86(txt.clone(), ret_4.clone(), a_type.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_5 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = createBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_5.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (literal!("measureTimeArrayLock")).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("bool _simulationFinished;")).clone() }))?;
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
fn lm_88(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_88(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_89(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThreadHeaderDecl(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_89(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_90(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_90(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_91(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = createLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_91(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_92(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_zeroFuncLocks: Tpl::Text, mut in_a_daeLocks: Tpl::Text, mut in_a_odeLocks: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_zeroFuncLocks.clone(), in_a_daeLocks.clone(), in_a_odeLocks.clone(), in_a_type.clone(), in_a_odeSchedule_threadTasks.clone())) {
        (txt, Deref @ "openmp", a_zeroFuncLocks, a_daeLocks, a_odeLocks, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadDecl: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadDecl = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadDecl = lm_88(l_threadDecl.clone(), ret_1.clone(), a_type.clone())?;
            l_threadDecl = Tpl::popIter(l_threadDecl.clone())?;
            txt = Tpl::writeText(txt.clone(), a_odeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_daeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_zeroFuncLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadDecl.clone())?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//MF Todo BLABLUB")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_zeroFuncLocks, a_daeLocks, a_odeLocks, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_8: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_thLocks1: Tpl::Text;
            let mut ret_6: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_thLocks: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadDecl: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_3 = listRest(ret_2.clone())?;
            l_threadDecl = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadDecl = lm_89(l_threadDecl.clone(), ret_3.clone(), a_type.clone())?;
            l_threadDecl = Tpl::popIter(l_threadDecl.clone())?;
            ret_5 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_6 = listRest(ret_5.clone())?;
            l_thLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_thLocks = lm_90(l_thLocks.clone(), ret_6.clone(), a_type.clone())?;
            l_thLocks = Tpl::popIter(l_thLocks.clone())?;
            ret_8 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_9 = listRest(ret_8.clone())?;
            l_thLocks1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_thLocks1 = lm_91(l_thLocks1.clone(), ret_9.clone(), a_type.clone())?;
            l_thLocks1 = Tpl::popIter(l_thLocks1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("bool _terminateThreads;\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_odeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_daeLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_zeroFuncLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_thLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_thLocks1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadDecl.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_93(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "openmp") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("tbb::flow::graph _tbbGraphOde;\n")).clone(), (literal!("tbb::flow::broadcast_node<tbb::flow::continue_msg> _tbbStartNodeOde;\n")).clone(), (literal!("tbb::flow::graph _tbbGraphAll;\n")).clone(), (literal!("tbb::flow::broadcast_node<tbb::flow::continue_msg> _tbbStartNodeAll;\n")).clone(), (literal!("tbb::flow::graph _tbbGraphZeroFunc;\n")).clone(), (literal!("tbb::flow::broadcast_node<tbb::flow::continue_msg> _tbbStartNodeZeroFunc;\n")).clone(), (literal!("std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* > _tbbNodeListOde;\n")).clone(), (literal!("std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* > _tbbNodeListAll;\n")).clone(), (literal!("std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* > _tbbNodeListZeroFunc;\n")).clone(), (literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("tbb::task_arena _tbbArena;\n")).clone(), (literal!("TbbArenaFunctor _tbbArenaFunctorOde;\n")).clone(), (literal!("TbbArenaFunctor _tbbArenaFunctorAll;\n")).clone(), (literal!("TbbArenaFunctor _tbbArenaFunctorZeroFunc;\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_94(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_schedulesOpt.clone(), in_a_type.clone())) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_87(txt.clone(), (str_0.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_odeSchedule_threadTasks, outgoingDepTasks: i_odeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_daeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_zeroFuncSchedule_outgoingDepTasks, .. })), a_type) => {
            let mut str_7: ArcStr = arcstr::literal!("");
            let mut ret_6: i32 = 0;
            let mut l_zeroFuncLocks: Tpl::Text;
            let mut ret_4: i32 = 0;
            let mut l_daeLocks: Tpl::Text;
            let mut ret_2: i32 = 0;
            let mut l_odeLocks: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_odeLocks = createLockArrayByName(Tpl::emptyTxt.clone(), ret_2.clone(), (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_4 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_daeLocks = createLockArrayByName(Tpl::emptyTxt.clone(), ret_4.clone(), (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_6 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_zeroFuncLocks = createLockArrayByName(Tpl::emptyTxt.clone(), ret_6.clone(), (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            str_7 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_92(txt.clone(), (str_7.clone()).clone(), l_zeroFuncLocks.clone(), l_daeLocks.clone(), l_odeLocks.clone(), a_type.clone(), i_odeSchedule_threadTasks.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ }, _, _)), a_type) => {
            let mut str_8: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_8 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_93(txt.clone(), (str_8.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateAdditionalHpcomVarHeaders(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("UPDATETYPE _command;\n")).clone(), (literal!("int _evaluateMode;\n")).clone()], lastHasNewLine: true }))?;
    out_txt = fun_94(out_txt.clone(), a_schedulesOpt.clone(), l_type.clone())?;
    Ok(out_txt)
}

fn fun_96(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_threadIdx.clone())) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("thread* evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateThreadHeaderDecl(mut txt: Tpl::Text, mut a_threadIdx: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_96(txt.clone(), (a_iType.clone()).clone(), a_threadIdx.clone())?;
    Ok(out_txt)
}

pub fn generateThreadFunctionHeaderDecl(mut txt: Tpl::Text, mut a_threadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFunc")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_threadIdx.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    Ok(out_txt)
}

fn fun_99(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_type.clone())) {
        (txt, Deref @ "pthreads", a_type) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",_command(IContinuous::UNDEF_UPDATE)\n")).clone(), (literal!(",_simulationFinished(false)\n")).clone(), (literal!(",")).clone()], lastHasNewLine: false }))?;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = initializeBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_type) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",_command(IContinuous::UNDEF_UPDATE)\n")).clone(), (literal!(",_simulationFinished(false)\n")).clone(), (literal!(",")).clone()], lastHasNewLine: false }))?;
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = initializeBarrierByName(txt.clone(), (literal!("levelBarrier")).clone(), (literal!("")).clone(), ret_1.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_100(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_zeroFuncSchedule_tasks.clone(), in_a_daeSchedule_tasks.clone(), in_a_odeSchedule_tasks.clone())) {
        (txt, Deref @ "tbb", a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_odeSchedule_tasks) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",_tbbGraphOde()\n")).clone(), (literal!(",_tbbGraphAll()\n")).clone(), (literal!(",_tbbGraphZeroFunc()\n")).clone(), (literal!(",_tbbStartNodeOde(_tbbGraphOde)\n")).clone(), (literal!(",_tbbStartNodeAll(_tbbGraphAll)\n")).clone(), (literal!(",_tbbStartNodeZeroFunc(_tbbGraphZeroFunc)\n")).clone(), (literal!(",_tbbNodeListOde(")).clone()], lastHasNewLine: false }))?;
            ret_0 = (a_odeSchedule_tasks.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",NULL)\n")).clone(), (literal!(",_tbbNodeListAll(")).clone()], lastHasNewLine: false }))?;
            ret_1 = (a_daeSchedule_tasks.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",NULL)\n")).clone(), (literal!(",_tbbNodeListZeroFunc(")).clone()], lastHasNewLine: false }))?;
            ret_2 = (a_zeroFuncSchedule_tasks.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",NULL)")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_101(mut in_txt: Tpl::Text, mut in_a_scheduleOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_scheduleOpt.clone(), in_a_type.clone())) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_99(txt.clone(), (str_0.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), a_type) => {
            let mut str_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_1 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_100(txt.clone(), (str_1.clone()).clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), i_odeSchedule_tasks.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn additionalHpcomConstructorDefinitions(mut txt: Tpl::Text, mut a_scheduleOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    out_txt = fun_101(txt.clone(), a_scheduleOpt.clone(), l_type.clone())?;
    Ok(out_txt)
}

fn fun_103(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fullModelName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fullModelName.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_fullModelName) => {
            let mut ret_0: i32 = 0;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = generateThreadMeasureTimeDeclaration(txt.clone(), (a_fullModelName.clone()).clone(), ret_0.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_104(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_modelNamePrefixStr, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_104(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_105(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_levelIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateODE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateODE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_105(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_106(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_levelIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateDAE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateDAE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_106(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_107(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_levelIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateZeroFuncs)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateZeroFunc_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_107(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_108(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_zeroFuncSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_daeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_fullModelName: ArcStr, mut in_a_odeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_zeroFuncSchedule_tasksOfLevels.clone(), in_a_daeSchedule_tasksOfLevels.clone(), in_a_fullModelName.clone(), in_a_odeSchedule_tasksOfLevels.clone())) {
        (txt, false, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels) => {
            let mut ret_8: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_7: i32 = 0;
            let mut ret_6: i32 = 0;
            let mut ret_5: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut ret_2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateODE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_0 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ODE\",measureTimeSchedulerArrayHpcom_evaluateODE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_1 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            ret_2 = List::intRange(ret_1.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_105(txt.clone(), ret_2.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateDAE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_3 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_3.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_DAE\",measureTimeSchedulerArrayHpcom_evaluateDAE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_4 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            ret_5 = List::intRange(ret_4.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_106(txt.clone(), ret_5.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateZeroFuncs = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_6 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_6.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ZeroFuncs\",measureTimeSchedulerArrayHpcom_evaluateZeroFuncs);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_7 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            ret_8 = List::intRange(ret_7.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_107(txt.clone(), ret_8.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_109(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_modelNamePrefixStr, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_109(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_110(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_levelIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateODE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateODE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_110(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_111(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_levelIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateDAE)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateDAE_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_111(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_112(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_levelIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_evaluateZeroFuncs)[")).clone() }))?;
            ret_0 = intSub(i_levelIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateZeroFunc_level_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_levelIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_112(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_113(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_zeroFuncSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_daeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_fullModelName: ArcStr, mut in_a_odeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_zeroFuncSchedule_tasksOfLevels.clone(), in_a_daeSchedule_tasksOfLevels.clone(), in_a_fullModelName.clone(), in_a_odeSchedule_tasksOfLevels.clone())) {
        (txt, false, _, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels) => {
            let mut ret_8: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_7: i32 = 0;
            let mut ret_6: i32 = 0;
            let mut ret_5: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut ret_2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateODE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_0 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ODE\",measureTimeSchedulerArrayHpcom_evaluateODE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_1 = (a_odeSchedule_tasksOfLevels.clone().len() as i32);
            ret_2 = List::intRange(ret_1.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_110(txt.clone(), ret_2.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateDAE = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_3 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_3.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_DAE\",measureTimeSchedulerArrayHpcom_evaluateDAE);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_4 = (a_daeSchedule_tasksOfLevels.clone().len() as i32);
            ret_5 = List::intRange(ret_4.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_111(txt.clone(), ret_5.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("measureTimeSchedulerArrayHpcom_evaluateZeroFuncs = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
            ret_6 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_6.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_fullModelName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"functions_HPCOM_Sections_ZeroFuncs\",measureTimeSchedulerArrayHpcom_evaluateZeroFuncs);\n")).clone(), (literal!("measuredSchedulerStartValues = MeasureTime::getZeroValues();\n")).clone(), (literal!("measuredSchedulerEndValues = MeasureTime::getZeroValues();\n")).clone()], lastHasNewLine: true }))?;
            ret_7 = (a_zeroFuncSchedule_tasksOfLevels.clone().len() as i32);
            ret_8 = List::intRange(ret_7.clone());
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_112(txt.clone(), ret_8.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_114(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_zeroFuncSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_daeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_fullModelName: ArcStr, mut in_a_odeSchedule_tasksOfLevels: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_zeroFuncSchedule_tasksOfLevels.clone(), in_a_daeSchedule_tasksOfLevels.clone(), in_a_fullModelName.clone(), in_a_odeSchedule_tasksOfLevels.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone())) {
        (txt, Deref @ "pthreads", a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels, a_modelNamePrefixStr, a_type) => {
            let mut ret_6: bool = false;
            let mut ret_5: bool = false;
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_2 = intSub(ret_1.clone(), 1);
            ret_3 = List::intRange(ret_2.clone());
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadFuncs = lm_104(l_threadFuncs.clone(), ret_3.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            ret_4 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_5 = stringEq((ret_4.clone()).clone(), (literal!("none")).clone());
            ret_6 = boolNot(ret_5.clone());
            txt = fun_108(txt.clone(), ret_6.clone(), a_zeroFuncSchedule_tasksOfLevels.clone(), a_daeSchedule_tasksOfLevels.clone(), (a_fullModelName.clone()).clone(), a_odeSchedule_tasksOfLevels.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_zeroFuncSchedule_tasksOfLevels, a_daeSchedule_tasksOfLevels, a_fullModelName, a_odeSchedule_tasksOfLevels, a_modelNamePrefixStr, a_type) => {
            let mut ret_12: bool = false;
            let mut ret_11: bool = false;
            let mut ret_10: ArcStr = arcstr::literal!("");
            let mut ret_9: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_8: i32 = 0;
            let mut ret_7: i32 = 0;
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_7 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_8 = intSub(ret_7.clone(), 1);
            ret_9 = List::intRange(ret_8.clone());
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadFuncs = lm_109(l_threadFuncs.clone(), ret_9.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            ret_10 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_11 = stringEq((ret_10.clone()).clone(), (literal!("none")).clone());
            ret_12 = boolNot(ret_11.clone());
            txt = fun_113(txt.clone(), ret_12.clone(), a_zeroFuncSchedule_tasksOfLevels.clone(), a_daeSchedule_tasksOfLevels.clone(), (a_fullModelName.clone()).clone(), a_odeSchedule_tasksOfLevels.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_115(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_modelNamePrefixStr, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_115(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_116(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_modelNamePrefixStr, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = generateThread(txt.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("evaluateThreadFunc")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_116(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_117(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = initializeLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_117(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_118(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = initializeLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_118(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_119(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = assignLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_119(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_120(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = assignLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_120(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_121(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_assignLocksZeroFunc: Tpl::Text, mut in_a_assignLocksOde: Tpl::Text, mut in_a_assignLocksDae: Tpl::Text, mut in_a_initLocksZeroFunc: Tpl::Text, mut in_a_initLocksDae: Tpl::Text, mut in_a_initLocksOde: Tpl::Text, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_assignLocksZeroFunc.clone(), in_a_assignLocksOde.clone(), in_a_assignLocksDae.clone(), in_a_initLocksZeroFunc.clone(), in_a_initLocksDae.clone(), in_a_initLocksOde.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone(), in_a_odeSchedule_threadTasks.clone())) {
        (txt, Deref @ "openmp", _, _, _, a_initLocksZeroFunc, a_initLocksDae, a_initLocksOde, a_modelNamePrefixStr, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadFuncs = lm_115(l_threadFuncs.clone(), ret_1.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("omp_set_dynamic(0);\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksZeroFunc.clone())?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, _, _, _, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//MF: Initialize MPI related stuff - nothing todo?")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_assignLocksZeroFunc, a_assignLocksOde, a_assignLocksDae, a_initLocksZeroFunc, a_initLocksDae, a_initLocksOde, a_modelNamePrefixStr, a_type, a_odeSchedule_threadTasks) => {
            let mut ret_15: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_14: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadAssignLocks1: Tpl::Text;
            let mut ret_12: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_11: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadAssignLocks: Tpl::Text;
            let mut ret_9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_8: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadLocksInit1: Tpl::Text;
            let mut ret_6: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadLocksInit: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_3 = listRest(ret_2.clone())?;
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadFuncs = lm_116(l_threadFuncs.clone(), ret_3.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs.clone())?;
            ret_5 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_6 = listRest(ret_5.clone())?;
            l_threadLocksInit = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadLocksInit = lm_117(l_threadLocksInit.clone(), ret_6.clone(), a_type.clone())?;
            l_threadLocksInit = Tpl::popIter(l_threadLocksInit.clone())?;
            ret_8 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_9 = listRest(ret_8.clone())?;
            l_threadLocksInit1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadLocksInit1 = lm_118(l_threadLocksInit1.clone(), ret_9.clone(), a_type.clone())?;
            l_threadLocksInit1 = Tpl::popIter(l_threadLocksInit1.clone())?;
            ret_11 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_12 = listRest(ret_11.clone())?;
            l_threadAssignLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadAssignLocks = lm_119(l_threadAssignLocks.clone(), ret_12.clone(), a_type.clone())?;
            l_threadAssignLocks = Tpl::popIter(l_threadAssignLocks.clone())?;
            ret_14 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_15 = listRest(ret_14.clone())?;
            l_threadAssignLocks1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadAssignLocks1 = lm_120(l_threadAssignLocks1.clone(), ret_15.clone(), a_type.clone())?;
            l_threadAssignLocks1 = Tpl::popIter(l_threadAssignLocks1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_terminateThreads = false;\n")).clone(), (literal!("_command = IContinuous::UNDEF_UPDATE;\n")).clone(), (literal!("_evaluateMode = -1;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_initLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_initLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksInit.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksInit1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_assignLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_assignLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_assignLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocks1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_122(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_modelNamePrefixStr.clone(), in_a_zeroFuncSchedule_tasks.clone(), in_a_daeSchedule_tasks.clone(), in_a_odeSchedule_tasks.clone())) {
        (txt, Deref @ "tbb", a_modelNamePrefixStr, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_odeSchedule_tasks) => {
            let mut l_tbbVars: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tbbVars = generateTbbConstructorExtension(Tpl::emptyTxt.clone(), a_odeSchedule_tasks.clone(), a_daeSchedule_tasks.clone(), a_zeroFuncSchedule_tasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_tbbVars.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_123(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_fullModelName: ArcStr, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_schedulesOpt.clone(), in_a_fullModelName.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone())) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: i_odeSchedule_tasksOfLevels, useFixedAssignments: true }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: i_daeSchedule_tasksOfLevels, useFixedAssignments: true }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: i_zeroFuncSchedule_tasksOfLevels, useFixedAssignments: true })), a_fullModelName, a_modelNamePrefixStr, a_type) => {
            let mut str_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_114(txt.clone(), (str_0.clone()).clone(), i_zeroFuncSchedule_tasksOfLevels.clone(), i_daeSchedule_tasksOfLevels.clone(), (a_fullModelName.clone()).clone(), i_odeSchedule_tasksOfLevels.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_odeSchedule_threadTasks, outgoingDepTasks: i_odeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_daeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_zeroFuncSchedule_outgoingDepTasks, .. })), _, a_modelNamePrefixStr, a_type) => {
            let mut str_13: ArcStr = arcstr::literal!("");
            let mut ret_12: i32 = 0;
            let mut l_assignLocksZeroFunc: Tpl::Text;
            let mut ret_10: i32 = 0;
            let mut l_initLocksZeroFunc: Tpl::Text;
            let mut ret_8: i32 = 0;
            let mut l_assignLocksDae: Tpl::Text;
            let mut ret_6: i32 = 0;
            let mut l_initLocksDae: Tpl::Text;
            let mut ret_4: i32 = 0;
            let mut l_assignLocksOde: Tpl::Text;
            let mut ret_2: i32 = 0;
            let mut l_initLocksOde: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_initLocksOde = initializeArrayLocks(Tpl::emptyTxt.clone(), ret_2.clone(), (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_4 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_assignLocksOde = assignArrayLocks(Tpl::emptyTxt.clone(), ret_4.clone(), (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_6 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_initLocksDae = initializeArrayLocks(Tpl::emptyTxt.clone(), ret_6.clone(), (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_8 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_assignLocksDae = assignArrayLocks(Tpl::emptyTxt.clone(), ret_8.clone(), (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_10 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_initLocksZeroFunc = initializeArrayLocks(Tpl::emptyTxt.clone(), ret_10.clone(), (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_12 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_assignLocksZeroFunc = assignArrayLocks(Tpl::emptyTxt.clone(), ret_12.clone(), (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            str_13 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_121(txt.clone(), (str_13.clone()).clone(), l_assignLocksZeroFunc.clone(), l_assignLocksOde.clone(), l_assignLocksDae.clone(), l_initLocksZeroFunc.clone(), l_initLocksDae.clone(), l_initLocksOde.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone(), i_odeSchedule_threadTasks.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), _, a_modelNamePrefixStr, a_type) => {
            let mut str_14: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_14 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_122(txt.clone(), (str_14.clone()).clone(), (a_modelNamePrefixStr.clone()).clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), i_odeSchedule_tasks.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn additionalHpcomConstructorBodyStatements(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut a_modelNamePrefixStr: ArcStr, mut a_fullModelName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_schedulerSpecificReturn: Tpl::Text;
    let mut ret_5: bool = false;
    let mut ret_4: bool = false;
    let mut ret_3: ArcStr = arcstr::literal!("");
    let mut l_threadMeasureTimeBlocks: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
    ret_4 = stringEq((ret_3.clone()).clone(), (literal!("none")).clone());
    ret_5 = boolNot(ret_4.clone());
    l_threadMeasureTimeBlocks = fun_103(Tpl::emptyTxt.clone(), ret_5.clone(), (a_fullModelName.clone()).clone())?;
    l_schedulerSpecificReturn = fun_123(Tpl::emptyTxt.clone(), a_schedulesOpt.clone(), (a_fullModelName.clone()).clone(), (a_modelNamePrefixStr.clone()).clone(), l_type.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_schedulerSpecificReturn.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_threadMeasureTimeBlocks.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_125(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MeasureTime::getZeroValues();")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_125(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_126(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("measuredSchedulerEndValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MeasureTime::getZeroValues();")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_126(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_127(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayOdeHpcom)[")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateODE_thread_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_127(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_128(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayDaeHpcom)[")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateDAE_thread_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_128(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_129(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayZeroFuncHpcom)[")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = new MeasureTimeData(\"evaluateZeroFunc_thread_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\");")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_129(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn generateThreadMeasureTimeDeclaration(mut txt: Tpl::Text, mut a_fullModelName: ArcStr, mut a_numberOfThreads: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_4: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ret_3: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ret_2: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ret_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ret_0: Arc<metamodelica::List<i32>> = metamodelica::nil();
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("measureTimeThreadArrayOdeHpcom = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_numberOfThreads.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("measureTimeThreadArrayDaeHpcom = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_numberOfThreads.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("measureTimeThreadArrayZeroFuncHpcom = new std::vector<MeasureTimeData*>(size_t(")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_numberOfThreads.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("), NULL);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_fullModelName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"evaluateODE_threads\",measureTimeThreadArrayOdeHpcom);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_fullModelName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\"evaluateDAE_threads\",measureTimeThreadArrayDaeHpcom);\n")).clone(), (literal!("MeasureTime::addResultContentBlock(\"")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_fullModelName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\",\"evaluateZeroFunc_threads\",measureTimeThreadArrayZeroFuncHpcom);\n")).clone() }))?;
    ret_0 = List::intRange(a_numberOfThreads.clone());
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_125(out_txt.clone(), ret_0.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    ret_1 = List::intRange(a_numberOfThreads.clone());
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_126(out_txt.clone(), ret_1.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    ret_2 = List::intRange(a_numberOfThreads.clone());
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_127(out_txt.clone(), ret_2.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    ret_3 = List::intRange(a_numberOfThreads.clone());
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_128(out_txt.clone(), ret_3.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    ret_4 = List::intRange(a_numberOfThreads.clone());
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_129(out_txt.clone(), ret_4.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#endif //MEASURETIME_MODELFUNCTIONS")).clone() }))?;
    Ok(out_txt)
}

fn fun_131(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_numComms.clone(), in_a_lockName.clone())) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_init_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]);")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i] = new alignedLock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i] = new alignedSpinlock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initializeArrayLocks(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_131(txt.clone(), (a_iType.clone()).clone(), a_numComms.clone(), (a_lockName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_133(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_numComms.clone(), in_a_lockName.clone())) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_set_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]);")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]->lock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]->lock();")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn assignArrayLocks(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_133(txt.clone(), (a_iType.clone()).clone(), a_numComms.clone(), (a_lockName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_135(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_numComms.clone(), in_a_lockName.clone())) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_lock_t ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedLock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedSpinlock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("];")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn createLockArrayByName(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_135(txt.clone(), (a_iType.clone()).clone(), a_numComms.clone(), (a_lockName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_137(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_numComms: i32, mut in_a_lockName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_numComms.clone(), in_a_lockName.clone())) {
        (txt, Deref @ "openmp", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_destroy_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i]);")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i];")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_numComms, a_lockName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for(unsigned i=0;i<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numComms.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";++i)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_[i];")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Unsupported parallel instrumentation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn destroyArrayLocks(mut txt: Tpl::Text, mut a_numComms: i32, mut a_lockName: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_137(txt.clone(), (a_iType.clone()).clone(), a_numComms.clone(), (a_lockName.clone()).clone())?;
    Ok(out_txt)
}

fn fun_139(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "pthreads") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_simulationFinished = true;\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_simulationFinished = true;\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();")).clone()], lastHasNewLine: false }))?;
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
fn lm_140(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = function_HPCOM_destroyThread(txt.clone(), (intString(x_i0.clone())).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_140(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_141(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = destroyLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_141(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_142(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = destroyLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_142(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_143(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = function_HPCOM_joinThread(txt.clone(), (intString(x_i0.clone())).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_143(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_144(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = releaseLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_144(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_145(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_destroyLocksZeroFunc: Tpl::Text, mut in_a_destroyLocksDae: Tpl::Text, mut in_a_destroyLocksOde: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_type.clone(), in_a_odeSchedule_threadTasks.clone(), in_a_destroyLocksZeroFunc.clone(), in_a_destroyLocksDae.clone(), in_a_destroyLocksOde.clone())) {
        (txt, Deref @ "openmp", _, _, a_destroyLocksZeroFunc, a_destroyLocksDae, a_destroyLocksOde) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_destroyLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksZeroFunc.clone())?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//MF: Destruct MPI related stuff - nothing at the moment.")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_type, a_odeSchedule_threadTasks, a_destroyLocksZeroFunc, a_destroyLocksDae, a_destroyLocksOde) => {
            let mut ret_14: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_13: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadReleaseLocks: Tpl::Text;
            let mut ret_11: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_10: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_joinThreads: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_7: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadLocksDel1: Tpl::Text;
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_4: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadLocksDel: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_destroyThreads: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_2 = listRest(ret_1.clone())?;
            l_destroyThreads = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_destroyThreads = lm_140(l_destroyThreads.clone(), ret_2.clone(), a_type.clone())?;
            l_destroyThreads = Tpl::popIter(l_destroyThreads.clone())?;
            ret_4 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_5 = listRest(ret_4.clone())?;
            l_threadLocksDel = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadLocksDel = lm_141(l_threadLocksDel.clone(), ret_5.clone(), a_type.clone())?;
            l_threadLocksDel = Tpl::popIter(l_threadLocksDel.clone())?;
            ret_7 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_8 = listRest(ret_7.clone())?;
            l_threadLocksDel1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadLocksDel1 = lm_142(l_threadLocksDel1.clone(), ret_8.clone(), a_type.clone())?;
            l_threadLocksDel1 = Tpl::popIter(l_threadLocksDel1.clone())?;
            ret_10 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_11 = listRest(ret_10.clone())?;
            l_joinThreads = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_joinThreads = lm_143(l_joinThreads.clone(), ret_11.clone(), a_type.clone())?;
            l_joinThreads = Tpl::popIter(l_joinThreads.clone())?;
            ret_13 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_14 = listRest(ret_13.clone())?;
            l_threadReleaseLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadReleaseLocks = lm_144(l_threadReleaseLocks.clone(), ret_14.clone(), a_type.clone())?;
            l_threadReleaseLocks = Tpl::popIter(l_threadReleaseLocks.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_terminateThreads = true;\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_joinThreads.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_destroyLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksDel.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadLocksDel1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_destroyThreads.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_mArg: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Deref @ "tbb") => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("for(std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* >::iterator it = _tbbNodeListOde.begin(); it != _tbbNodeListOde.end(); it++)\n")).clone(), (literal!("  delete *it;\n")).clone(), (literal!("for(std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* >::iterator it = _tbbNodeListAll.begin(); it != _tbbNodeListAll.end(); it++)\n")).clone(), (literal!("  delete *it;\n")).clone(), (literal!("for(std::vector<tbb::flow::continue_node<tbb::flow::continue_msg>* >::iterator it = _tbbNodeListZeroFunc.begin(); it != _tbbNodeListZeroFunc.end(); it++)\n")).clone(), (literal!("  delete *it;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_147(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_schedulesOpt.clone(), in_a_type.clone())) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, _, _)), a_type) => {
            let mut str_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_0 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_139(txt.clone(), (str_0.clone()).clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_odeSchedule_threadTasks, outgoingDepTasks: i_odeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_daeSchedule_outgoingDepTasks, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { outgoingDepTasks: i_zeroFuncSchedule_outgoingDepTasks, .. })), a_type) => {
            let mut str_7: ArcStr = arcstr::literal!("");
            let mut ret_6: i32 = 0;
            let mut l_destroyLocksZeroFunc: Tpl::Text;
            let mut ret_4: i32 = 0;
            let mut l_destroyLocksDae: Tpl::Text;
            let mut ret_2: i32 = 0;
            let mut l_destroyLocksOde: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_odeSchedule_outgoingDepTasks.clone().len() as i32);
            l_destroyLocksOde = destroyArrayLocks(Tpl::emptyTxt.clone(), ret_2.clone(), (literal!("_lockOde")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_4 = (i_daeSchedule_outgoingDepTasks.clone().len() as i32);
            l_destroyLocksDae = destroyArrayLocks(Tpl::emptyTxt.clone(), ret_4.clone(), (literal!("_lockDae")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            ret_6 = (i_zeroFuncSchedule_outgoingDepTasks.clone().len() as i32);
            l_destroyLocksZeroFunc = destroyArrayLocks(Tpl::emptyTxt.clone(), ret_6.clone(), (literal!("_lockZeroFunc")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            str_7 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_145(txt.clone(), (str_7.clone()).clone(), a_type.clone(), i_odeSchedule_threadTasks.clone(), l_destroyLocksZeroFunc.clone(), l_destroyLocksDae.clone(), l_destroyLocksOde.clone())?;
            txt.clone()
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: _ }, _, _)), a_type) => {
            let mut str_8: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            str_8 = (Tpl::textString(a_type.clone())?).clone();
            txt = fun_146(txt.clone(), (str_8.clone()).clone())?;
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
fn lm_148(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_148(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_149(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete measuredSchedulerEndValues_")).clone() }))?;
            ret_0 = intSub(i_threadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_149(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn additionalHpcomDestructorBodyStatements(mut txt: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_6: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ret_5: i32 = 0;
    let mut ret_4: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut ret_3: i32 = 0;
    let mut l_schedulerSpecificCode: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    l_schedulerSpecificCode = fun_147(Tpl::emptyTxt.clone(), a_schedulesOpt.clone(), l_type.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone() }))?;
    ret_3 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    ret_4 = List::intRange(ret_3.clone());
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_148(out_txt.clone(), ret_4.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    ret_5 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    ret_6 = List::intRange(ret_5.clone());
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_149(out_txt.clone(), ret_6.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#endif //MEASURETIME_MODELFUNCTIONS\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_schedulerSpecificCode.clone())?;
    Ok(out_txt)
}

fn fun_151(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_stateDerVectorName: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_stateDerVectorName, out_a_extraFuncsNamespace, out_a_extraFuncsDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_useFlatArrayNotation.clone(), in_a_stateDerVectorName.clone(), in_a_context.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, i_simCode @ SimCode::SimCode { clockedPartitions: i_clockedPartitions, allEquations: i_allEquations, hpcomData: HpcOmSimCode::HpcOmData { schedules: i_hpcomData_schedules, .. }, modelInfo: SimCode::ModelInfo { name: i_modelInfo_name, .. }, .. }, a_extraFuncs, a_useFlatArrayNotation, a_stateDerVectorName, a_context, a_extraFuncsNamespace, a_extraFuncsDecl, a_allEquationsPlusWhen) => {
            let mut ret_6: bool = false;
            let mut ret_5: bool = false;
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: Arc<metamodelica::List<SimCode::SubPartition>> = metamodelica::nil();
            let mut txt_2: Tpl::Text;
            let mut l_parCode: Tpl::Text;
            let mut l_extraFuncsPar: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_stateDerVectorName = (*a_stateDerVectorName).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            l_extraFuncsPar = Tpl::emptyTxt.clone();
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), i_modelInfo_name.clone())?;
            (l_parCode, l_extraFuncsPar, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = generateParallelEvaluate(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), i_modelInfo_name.clone(), i_simCode.clone(), l_extraFuncsPar.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), i_hpcomData_schedules.clone(), a_context.clone(), a_stateDerVectorName.clone(), (Tpl::textString(txt_2.clone())?).clone(), a_useFlatArrayNotation.clone())?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = CodegenCpp::equationFunctions(txt.clone(), i_allEquations.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_stateDerVectorName.clone(), a_useFlatArrayNotation.clone(), false)?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = CodegenCpp::createEvaluateConditions(txt.clone(), i_allEquations.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), SimCodeFunction::contextOther().clone(), a_stateDerVectorName.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            ret_3 = SimCodeUtil::getSubPartitions(i_clockedPartitions.clone());
            ret_4 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_5 = stringEq((ret_4.clone()).clone(), (literal!("none")).clone());
            ret_6 = boolNot(ret_5.clone());
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_stateDerVectorName) = CodegenCpp::clockedFunctions(txt.clone(), ret_3.clone(), i_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), SimCodeFunction::contextSimulationDiscrete().clone(), a_stateDerVectorName.clone(), a_useFlatArrayNotation.clone(), ret_6.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_parCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_extraFuncsPar.clone())?;
            (txt.clone(), a_extraFuncs.clone(), a_stateDerVectorName.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone())
        },
        (txt, _, a_extraFuncs, _, a_stateDerVectorName, _, a_extraFuncsNamespace, a_extraFuncsDecl, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_stateDerVectorName.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncs, out_a_stateDerVectorName, out_a_extraFuncsNamespace, out_a_extraFuncsDecl))
}

pub fn updateHpcom(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_context: SimCodeFunction::Context, mut a_stateDerVectorName: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    (out_txt, out_a_extraFuncs, out_a_stateDerVectorName, out_a_extraFuncsNamespace, out_a_extraFuncsDecl) = fun_151(txt.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_useFlatArrayNotation.clone(), a_stateDerVectorName.clone(), a_context.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_allEquationsPlusWhen.clone())?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, out_a_stateDerVectorName))
}

fn fun_153(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("evaluateODE")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_154(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("measuredFunctionEndValues")).clone(), (literal!("(*measureTimeFunctionsArray)[0]")).clone(), (literal!("evaluateODE")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_155(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("evaluateAll")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_156(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("measuredFunctionEndValues")).clone(), (literal!("(*measureTimeFunctionsArray)[1]")).clone(), (literal!("evaluateAll")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_157(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("evaluateZeroFuncs")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_158(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredFunctionStartValues")).clone(), (literal!("measuredFunctionEndValues")).clone(), (literal!("(*measureTimeFunctionsArray)[4]")).clone(), (literal!("evaluateZeroFuncs")).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_159(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_159(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_160(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_160(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_161(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<HpcOmSimCode::TaskList>>, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_161(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_162(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_type: Tpl::Text, mut in_a_functionHead: Tpl::Text, mut in_a_zeroFuncEqs: Tpl::Text, mut in_a_daeEqs: Tpl::Text, mut in_a_odeEqs: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_type.clone(), in_a_functionHead.clone(), in_a_zeroFuncEqs.clone(), in_a_daeEqs.clone(), in_a_odeEqs.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone())) {
        (txt, Deref @ "openmp", _, a_functionHead, a_zeroFuncEqs, a_daeEqs, a_odeEqs, a_name, a_extraFuncsDecl) => {
            let mut ret_2: i32 = 0;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void evaluateODE_Parallel();\n")).clone(), (literal!("void evaluateAll_Parallel();\n")).clone(), (literal!("void evaluateZeroFuncs_Parallel();")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_0 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_daeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_2 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_zeroFuncEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  if(evaluateMode == 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateODE_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateAll_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateZeroFuncs_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsDecl.clone())
        },
        (txt, _, a_type, a_functionHead, _, _, _, _, a_extraFuncsDecl) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("{\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("throw std::runtime_error(\"Type ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" is unsupported for level scheduling.\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsDecl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl))
}

fn lm_163(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<Absyn::Path>, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsDecl, a_extraFuncs, _, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_163(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_164(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<Absyn::Path>, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsDecl, a_extraFuncs, _, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_164(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_165(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<Absyn::Path>, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsDecl, a_extraFuncs, _, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_165(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_166(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<Absyn::Path>, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsDecl, a_extraFuncs, _, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut x_i0: i32 = 0;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForThread(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_166(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_167(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_type) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadLock")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(i_tt.clone())).clone())?;
            txt = createLockByLockName(txt.clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_167(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_168(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<Absyn::Path>, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsDecl, a_extraFuncs, _, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen) => {
            let mut x_i0: i32 = 0;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = generateLevelFixedCodeForThread(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_168(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_169(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_type) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadLock")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(i_tt.clone())).clone())?;
            txt = createLockByLockName(txt.clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_169(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_170(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_functionHead: Tpl::Text, mut in_a_zeroFuncSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_daeSchedule: Arc<HpcOmSimCode::Schedule>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_name: Arc<Absyn::Path>, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_odeSchedule: Arc<HpcOmSimCode::Schedule>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_extraFuncsNamespace.clone(), in_a_functionHead.clone(), in_a_zeroFuncSchedule.clone(), in_a_daeSchedule.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_name.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone(), in_a_odeSchedule.clone())) {
        (txt, Deref @ "openmp", a_extraFuncsNamespace, a_functionHead, a_zeroFuncSchedule, a_daeSchedule, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule) => {
            let mut ret_11: i32 = 0;
            let mut ret_10: i32 = 0;
            let mut ret_9: i32 = 0;
            let mut ret_8: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
            let mut ret_7: i32 = 0;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut ret_5: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
            let mut ret_4: i32 = 0;
            let mut l_daeEqs: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>> = metamodelica::nil();
            let mut ret_1: i32 = 0;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_1 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_2 = HpcOmScheduler::convertFixedLevelScheduleToLevelThreadLists(a_odeSchedule.clone(), ret_1.clone());
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_163(l_odeEqs.clone(), ret_2.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            ret_4 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_5 = HpcOmScheduler::convertFixedLevelScheduleToLevelThreadLists(a_daeSchedule.clone(), ret_4.clone());
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_daeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_164(l_daeEqs.clone(), ret_5.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs.clone())?;
            ret_7 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_8 = HpcOmScheduler::convertFixedLevelScheduleToLevelThreadLists(a_zeroFuncSchedule.clone(), ret_7.clone());
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_zeroFuncEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_165(l_zeroFuncEqs.clone(), ret_8.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs.clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void evaluateODE_Parallel();\n")).clone(), (literal!("void evaluateAll_Parallel();\n")).clone(), (literal!("void evaluateZeroFuncs_Parallel();")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_9 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_9.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_10 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_10.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_11 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_11.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  if(evaluateMode == 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateODE_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateAll_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateZeroFuncs_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "pthreads", a_extraFuncsNamespace, a_functionHead, a_zeroFuncSchedule, a_daeSchedule, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule) => {
            let mut ret_18: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_17: i32 = 0;
            let mut l_threadLocks: Tpl::Text;
            let mut ret_15: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> = metamodelica::nil();
            let mut ret_14: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
            let mut ret_13: i32 = 0;
            let mut l_eqsFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_13 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_14 = HpcOmScheduler::convertFixedLevelScheduleToTaskLists(a_odeSchedule.clone(), a_daeSchedule.clone(), a_zeroFuncSchedule.clone(), ret_13.clone())?;
            ret_15 = Arc::new(ret_14.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_eqsFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_eqsFuncs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_166(l_eqsFuncs.clone(), ret_15.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_eqsFuncs = Tpl::popIter(l_eqsFuncs.clone())?;
            ret_17 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_18 = List::intRange(ret_17.clone());
            l_threadLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadLocks = lm_167(l_threadLocks.clone(), ret_18.clone(), a_type.clone())?;
            l_threadLocks = Tpl::popIter(l_threadLocks.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eqsFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  this->_evaluateMode = evaluateMode;\n")).clone(), (literal!("\n")).clone(), (literal!("  if(evaluateMode == 0) //evaluate ODE\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncODE_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateStateVarPrefetchCode(txt.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0) //evaluate All\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncAll_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else //evaluate ZeroFuncs\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncZeroFunc_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "pthreads_spin", a_extraFuncsNamespace, a_functionHead, a_zeroFuncSchedule, a_daeSchedule, a_useFlatArrayNotation, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_name, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule) => {
            let mut ret_23: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_22: i32 = 0;
            let mut ret_21: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>> = metamodelica::nil();
            let mut ret_20: metamodelica::Array<(Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>)>;
            let mut ret_19: i32 = 0;
            let mut l_threadLocks: Tpl::Text;
            let mut l_eqsFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_19 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_20 = HpcOmScheduler::convertFixedLevelScheduleToTaskLists(a_odeSchedule.clone(), a_daeSchedule.clone(), a_zeroFuncSchedule.clone(), ret_19.clone())?;
            ret_21 = Arc::new(ret_20.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_eqsFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_eqsFuncs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_168(l_eqsFuncs.clone(), ret_21.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_eqsFuncs = Tpl::popIter(l_eqsFuncs.clone())?;
            ret_22 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            ret_23 = List::intRange(ret_22.clone());
            l_threadLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadLocks = lm_169(l_threadLocks.clone(), ret_23.clone(), a_type.clone())?;
            l_threadLocks = Tpl::popIter(l_threadLocks.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eqsFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  this->_evaluateMode = evaluateMode;\n")).clone(), (literal!("\n")).clone(), (literal!("  if(evaluateMode == 0) //evaluate ODE\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncODE_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            (txt, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateStateVarPrefetchCode(txt.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0) //evaluate All\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncAll_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else //evaluate ZeroFuncs\n")).clone(), (literal!("  {\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    evaluateThreadFuncZeroFunc_0();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, a_extraFuncsNamespace, a_functionHead, _, _, _, a_extraFuncsDecl, a_extraFuncs, _, _, a_varDecls, a_type, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("{\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("throw std::runtime_error(\"Type ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_type.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" is unsupported for levelfix scheduling.\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_171(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone(), in_a_threadTasksOde.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type, a_threadTasksOde) => {
            let mut x_i0: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0.clone(), 1);
            ret_1 = a_threadTasksOde.clone().borrow()[(ret_0.clone()-1) as usize].clone();
            txt = function_HPCOM_assignThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockOde")).clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_171(txt.clone(), rest.clone(), a_type.clone(), a_threadTasksOde.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_172(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone(), in_a_threadTasksOde.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type, a_threadTasksOde) => {
            let mut x_i0: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0.clone(), 1);
            ret_1 = a_threadTasksOde.clone().borrow()[(ret_0.clone()-1) as usize].clone();
            txt = function_HPCOM_releaseThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockOde")).clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_172(txt.clone(), rest.clone(), a_type.clone(), a_threadTasksOde.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_173(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone(), in_a_threadTasksDae.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type, a_threadTasksDae) => {
            let mut x_i0: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0.clone(), 1);
            ret_1 = a_threadTasksDae.clone().borrow()[(ret_0.clone()-1) as usize].clone();
            txt = function_HPCOM_assignThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockDae")).clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_173(txt.clone(), rest.clone(), a_type.clone(), a_threadTasksDae.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_174(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone(), in_a_threadTasksDae.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type, a_threadTasksDae) => {
            let mut x_i0: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0.clone(), 1);
            ret_1 = a_threadTasksDae.clone().borrow()[(ret_0.clone()-1) as usize].clone();
            txt = function_HPCOM_releaseThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockDae")).clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_174(txt.clone(), rest.clone(), a_type.clone(), a_threadTasksDae.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_175(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone(), in_a_threadTasksZeroFunc.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type, a_threadTasksZeroFunc) => {
            let mut x_i0: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0.clone(), 1);
            ret_1 = a_threadTasksZeroFunc.clone().borrow()[(ret_0.clone()-1) as usize].clone();
            txt = function_HPCOM_assignThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockZeroFunc")).clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_175(txt.clone(), rest.clone(), a_type.clone(), a_threadTasksZeroFunc.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_176(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone(), in_a_threadTasksZeroFunc.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type, a_threadTasksZeroFunc) => {
            let mut x_i0: i32 = 0;
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intAdd(x_i0.clone(), 1);
            ret_1 = a_threadTasksZeroFunc.clone().borrow()[(ret_0.clone()-1) as usize].clone();
            txt = function_HPCOM_releaseThreadLocks(txt.clone(), ret_1.clone(), (literal!("_lockZeroFunc")).clone(), x_i0.clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_176(txt.clone(), rest.clone(), a_type.clone(), a_threadTasksZeroFunc.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_177(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_threadTasksOde.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_threadTasksOde, a_allEquationsPlusWhen) => {
            let mut x_i0: i32 = 0;
            let mut txt_2: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = (a_threadTasksOde.clone().borrow().len() as i32);
            ret_1 = intSub(ret_0.clone(), 1);
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0.clone(), ret_1.clone(), (Tpl::textString(a_type.clone())?).clone(), (literal!("_lockOde")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2.clone(), (literal!("evaluateODE")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_177(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksOde.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_178(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_threadTasksDae.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_threadTasksDae, a_allEquationsPlusWhen) => {
            let mut x_i0: i32 = 0;
            let mut txt_2: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = (a_threadTasksDae.clone().borrow().len() as i32);
            ret_1 = intSub(ret_0.clone(), 1);
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0.clone(), ret_1.clone(), (Tpl::textString(a_type.clone())?).clone(), (literal!("_lockDae")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2.clone(), (literal!("evaluateAll")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_178(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksDae.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_179(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_threadTasksZeroFunc.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_threadTasksZeroFunc, a_allEquationsPlusWhen) => {
            let mut x_i0: i32 = 0;
            let mut txt_2: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = (a_threadTasksZeroFunc.clone().borrow().len() as i32);
            ret_1 = intSub(ret_0.clone(), 1);
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0.clone(), ret_1.clone(), (Tpl::textString(a_type.clone())?).clone(), (literal!("_lockZeroFunc")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2.clone(), (literal!("evaluateZeroFunc")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_179(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksZeroFunc.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_180(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_useFlatArrayNotation: bool, mut in_a_mainThreadCode: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_type: Tpl::Text, mut in_a_zeroFuncSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_daeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_mainThreadCode: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_mainThreadCode, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_mainThreadCode.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_modelNamePrefixStr.clone(), in_a_type.clone(), in_a_zeroFuncSchedule_threadTasks.clone(), in_a_daeSchedule_threadTasks.clone(), in_a_odeSchedule_threadTasks.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_mainThreadCode, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _, _, _, _) => {
            (txt.clone(), a_mainThreadCode.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadIdx, tail: rest }, a_useFlatArrayNotation, a_mainThreadCode, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_modelNamePrefixStr, a_type, a_zeroFuncSchedule_threadTasks, a_daeSchedule_threadTasks, a_odeSchedule_threadTasks, a_allEquationsPlusWhen) => {
            let mut txt_4: Tpl::Text;
            let mut ret_3: i32 = 0;
            let mut ret_2: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_1: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut ret_0: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            let mut a_mainThreadCode = (*a_mainThreadCode).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_0 = a_odeSchedule_threadTasks.clone().borrow()[(i_threadIdx.clone()-1) as usize].clone();
            ret_1 = a_daeSchedule_threadTasks.clone().borrow()[(i_threadIdx.clone()-1) as usize].clone();
            ret_2 = a_zeroFuncSchedule_threadTasks.clone().borrow()[(i_threadIdx.clone()-1) as usize].clone();
            ret_3 = intSub(i_threadIdx.clone(), 1);
            txt_4 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_4, a_mainThreadCode) = generateThreadFunc(txt.clone(), a_allEquationsPlusWhen.clone(), ret_0.clone(), ret_1.clone(), ret_2.clone(), (Tpl::textString(a_type.clone())?).clone(), ret_3.clone(), (a_modelNamePrefixStr.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_4.clone(), a_mainThreadCode.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_mainThreadCode, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_180(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_mainThreadCode.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone(), a_zeroFuncSchedule_threadTasks.clone(), a_daeSchedule_threadTasks.clone(), a_odeSchedule_threadTasks.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_mainThreadCode.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_mainThreadCode, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_181(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = assignLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock1")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_181(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_182(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_type: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_type.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_type) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = releaseLockByLockName(txt.clone(), (intString(x_i0.clone())).clone(), (literal!("th_lock")).clone(), (Tpl::textString(a_type.clone())?).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_182(txt.clone(), rest.clone(), a_type.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_183(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_zeroFuncSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_daeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_odeSchedule_threadTasks: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_functionHead: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_threadTasksZeroFunc: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_threadTasksDae: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut in_a_type: Tpl::Text, mut in_a_threadTasksOde: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_modelNamePrefixStr.clone(), in_a_zeroFuncSchedule_threadTasks.clone(), in_a_daeSchedule_threadTasks.clone(), in_a_odeSchedule_threadTasks.clone(), in_a_functionHead.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_allEquationsPlusWhen.clone(), in_a_threadTasksZeroFunc.clone(), in_a_threadTasksDae.clone(), in_a_type.clone(), in_a_threadTasksOde.clone())) {
        (txt, Deref @ "openmp", _, _, _, _, a_functionHead, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_threadTasksZeroFunc, a_threadTasksDae, a_type, a_threadTasksOde) => {
            let mut ret_20: i32 = 0;
            let mut ret_19: i32 = 0;
            let mut ret_18: i32 = 0;
            let mut ret_17: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut ret_15: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_daeEqs: Tpl::Text;
            let mut ret_13: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_odeEqs: Tpl::Text;
            let mut ret_11: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadReleaseLocksZeroFunc: Tpl::Text;
            let mut ret_9: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadAssignLocksZeroFunc: Tpl::Text;
            let mut ret_7: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadReleaseLocksDae: Tpl::Text;
            let mut ret_5: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadAssignLocksDae: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadReleaseLocksOde: Tpl::Text;
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadAssignLocksOde: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            ret_1 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadAssignLocksOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadAssignLocksOde = lm_171(l_threadAssignLocksOde.clone(), ret_1.clone(), a_type.clone(), a_threadTasksOde.clone())?;
            l_threadAssignLocksOde = Tpl::popIter(l_threadAssignLocksOde.clone())?;
            ret_3 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadReleaseLocksOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadReleaseLocksOde = lm_172(l_threadReleaseLocksOde.clone(), ret_3.clone(), a_type.clone(), a_threadTasksOde.clone())?;
            l_threadReleaseLocksOde = Tpl::popIter(l_threadReleaseLocksOde.clone())?;
            ret_5 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadAssignLocksDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadAssignLocksDae = lm_173(l_threadAssignLocksDae.clone(), ret_5.clone(), a_type.clone(), a_threadTasksDae.clone())?;
            l_threadAssignLocksDae = Tpl::popIter(l_threadAssignLocksDae.clone())?;
            ret_7 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadReleaseLocksDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadReleaseLocksDae = lm_174(l_threadReleaseLocksDae.clone(), ret_7.clone(), a_type.clone(), a_threadTasksDae.clone())?;
            l_threadReleaseLocksDae = Tpl::popIter(l_threadReleaseLocksDae.clone())?;
            ret_9 = Arc::new(a_threadTasksZeroFunc.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadAssignLocksZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadAssignLocksZeroFunc = lm_175(l_threadAssignLocksZeroFunc.clone(), ret_9.clone(), a_type.clone(), a_threadTasksZeroFunc.clone())?;
            l_threadAssignLocksZeroFunc = Tpl::popIter(l_threadAssignLocksZeroFunc.clone())?;
            ret_11 = Arc::new(a_threadTasksZeroFunc.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_threadReleaseLocksZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadReleaseLocksZeroFunc = lm_176(l_threadReleaseLocksZeroFunc.clone(), ret_11.clone(), a_type.clone(), a_threadTasksZeroFunc.clone())?;
            l_threadReleaseLocksZeroFunc = Tpl::popIter(l_threadReleaseLocksZeroFunc.clone())?;
            ret_13 = Arc::new(a_threadTasksOde.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_177(l_odeEqs.clone(), ret_13.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksOde.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            ret_15 = Arc::new(a_threadTasksDae.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_daeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_178(l_daeEqs.clone(), ret_15.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksDae.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs.clone())?;
            ret_17 = Arc::new(a_threadTasksZeroFunc.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_zeroFuncEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_179(l_zeroFuncEqs.clone(), ret_17.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_threadTasksZeroFunc.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs.clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void evaluateODE_Parallel();\n")).clone(), (literal!("void evaluateAll_Parallel();\n")).clone(), (literal!("void evaluateZeroFuncs_Parallel();")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_18 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_18.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocksOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_19 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_19.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocksDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs_Parallel()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp parallel num_threads(")).clone() }))?;
            ret_20 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_20.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone(), (literal!("  int threadNum = getThreadNumber();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("#pragma omp barrier\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocksZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone(), (literal!("  if(evaluateMode == 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateODE_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0)\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateAll_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("  else\n")).clone(), (literal!("  {\n")).clone(), (literal!("    evaluateZeroFuncs_Parallel();\n")).clone(), (literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "mpi", _, _, _, _, a_functionHead, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  // MFlehmig: Todo\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, a_modelNamePrefixStr, a_zeroFuncSchedule_threadTasks, a_daeSchedule_threadTasks, a_odeSchedule_threadTasks, a_functionHead, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, _, _, a_type, _) => {
            let mut ret_30: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_29: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadReleaseLocks: Tpl::Text;
            let mut ret_27: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut ret_26: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
            let mut l_threadAssignLocks1: Tpl::Text;
            let mut ret_24: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_23: i32 = 0;
            let mut l_threadFuncs: Tpl::Text;
            let mut l_mainThreadCode: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_mainThreadCode = Tpl::emptyTxt.clone();
            ret_23 = (a_odeSchedule_threadTasks.clone().borrow().len() as i32);
            ret_24 = List::intRange(ret_23.clone());
            l_threadFuncs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_threadFuncs, l_mainThreadCode, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_180(l_threadFuncs.clone(), ret_24.clone(), a_useFlatArrayNotation.clone(), l_mainThreadCode.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_modelNamePrefixStr.clone()).clone(), a_type.clone(), a_zeroFuncSchedule_threadTasks.clone(), a_daeSchedule_threadTasks.clone(), a_odeSchedule_threadTasks.clone(), a_allEquationsPlusWhen.clone())?;
            l_threadFuncs = Tpl::popIter(l_threadFuncs.clone())?;
            ret_26 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_27 = listRest(ret_26.clone())?;
            l_threadAssignLocks1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadAssignLocks1 = lm_181(l_threadAssignLocks1.clone(), ret_27.clone(), a_type.clone())?;
            l_threadAssignLocks1 = Tpl::popIter(l_threadAssignLocks1.clone())?;
            ret_29 = Arc::new(a_odeSchedule_threadTasks.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_30 = listRest(ret_29.clone())?;
            l_threadReleaseLocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_threadReleaseLocks = lm_182(l_threadReleaseLocks.clone(), ret_30.clone(), a_type.clone())?;
            l_threadReleaseLocks = Tpl::popIter(l_threadReleaseLocks.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_threadReleaseLocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mainThreadCode.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_threadAssignLocks1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_184(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_name: Arc<Absyn::Path>, mut in_a_functionHead: Tpl::Text, mut in_a_zeroFuncSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_daeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_odeSchedule_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_name.clone(), in_a_functionHead.clone(), in_a_zeroFuncSchedule_tasks.clone(), in_a_daeSchedule_tasks.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_type.clone(), in_a_allEquationsPlusWhen.clone(), in_a_odeSchedule_tasks.clone())) {
        (txt, Deref @ "openmp", _, a_functionHead, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule_tasks) => {
            let mut l_zeroFuncTaskEqs: Tpl::Text;
            let mut l_daeTaskEqs: Tpl::Text;
            let mut l_odeTaskEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_odeTaskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep(Tpl::emptyTxt.clone(), a_odeSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            (l_daeTaskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep(Tpl::emptyTxt.clone(), a_daeSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            (l_zeroFuncTaskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep(Tpl::emptyTxt.clone(), a_zeroFuncSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeTaskEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeTaskEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncTaskEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ "tbb", a_name, a_functionHead, a_zeroFuncSchedule_tasks, a_daeSchedule_tasks, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_type, a_allEquationsPlusWhen, a_odeSchedule_tasks) => {
            let mut l_taskFuncs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_taskFuncs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc(Tpl::emptyTxt.clone(), a_odeSchedule_tasks.clone(), a_daeSchedule_tasks.clone(), a_zeroFuncSchedule_tasks.clone(), a_allEquationsPlusWhen.clone(), (Tpl::textString(a_type.clone())?).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("//void functions for functionhandling in tbb_nodes\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_taskFuncs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  this->_evaluateMode = _evaluateMode;\n")).clone(), (literal!("  this->_command = command;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_varDecls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone(), (literal!("  #if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("    _tbbArena.execute(_tbbArenaFunctorOde);\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    _tbbStartNodeOde.try_put(tbb::flow::continue_msg());\n")).clone(), (literal!("    _tbbGraphOde.wait_for_all();\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone(), (literal!("  #if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("    _tbbArena.execute(_tbbArenaFunctorAll);\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    _tbbStartNodeAll.try_put(tbb::flow::continue_msg());\n")).clone(), (literal!("    _tbbGraphAll.wait_for_all();\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone(), (literal!("  #if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("    _tbbArena.execute(_tbbArenaFunctorZeroFunc);\n")).clone(), (literal!("  #else\n")).clone(), (literal!("    _tbbStartNodeZeroFunc.try_put(tbb::flow::continue_msg());\n")).clone(), (literal!("    _tbbGraphZeroFunc.wait_for_all();\n")).clone(), (literal!("  #endif\n")).clone(), (literal!("}\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, _, _, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_185(mut in_txt: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_type: Tpl::Text, mut in_a_useFlatArrayNotation: bool, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_functionHead: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_schedulesOpt.clone(), in_a_modelNamePrefixStr.clone(), in_a_extraFuncsNamespace.clone(), in_a_type.clone(), in_a_useFlatArrayNotation.clone(), in_a_name.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_allEquationsPlusWhen.clone(), in_a_functionHead.clone())) {
        (txt, Some((Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_taskListOde, .. } }, Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_taskListDae, .. } }, Deref @ HpcOmSimCode::Schedule::EMPTYSCHEDULE { tasks: HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_taskListZeroFunc, .. } })), _, a_extraFuncsNamespace, _, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            txt = Tpl::writeText(txt.clone(), a_functionHead.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  if(evaluateMode == 0) //evaluate ODE\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt_0 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_0) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_taskListOde.clone(), 1, 1, (literal!("")).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_0.clone(), (literal!("evaluateODE_Th1")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else if(evaluateMode < 0) //evaluate All\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt_1 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_1) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_taskListDae.clone(), 1, 1, (literal!("")).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_1.clone(), (literal!("evaluateAll_Th1")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("  else //evaluate ZeroFuncs\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt_2 = CodegenCpp::lastIdentOfPath(Tpl::emptyTxt.clone(), a_name.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, txt_2) = parallelThreadCodeWithSplit(txt.clone(), a_allEquationsPlusWhen.clone(), i_taskListZeroFunc.clone(), 1, 1, (literal!("")).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), txt_2.clone(), (literal!("evaluateZeroFunc_Th1")).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: i_tasksOfLevelsOde, useFixedAssignments: false }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: i_tasksOfLevelsDae, useFixedAssignments: false }, Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { tasksOfLevels: i_tasksOfLevelsZeroFunc, useFixedAssignments: false })), _, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_6: ArcStr = arcstr::literal!("");
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut l_daeEqs: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_159(l_odeEqs.clone(), i_tasksOfLevelsOde.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_daeEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_160(l_daeEqs.clone(), i_tasksOfLevelsDae.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs.clone())?;
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_zeroFuncEqs, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_161(l_zeroFuncEqs.clone(), i_tasksOfLevelsZeroFunc.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs.clone())?;
            str_6 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsDecl) = fun_162(txt.clone(), (str_6.clone()).clone(), a_type.clone(), a_functionHead.clone(), l_zeroFuncEqs.clone(), l_daeEqs.clone(), l_odeEqs.clone(), a_name.clone(), a_extraFuncsDecl.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((i_odeSchedule @ Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, i_daeSchedule @ Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. }, i_zeroFuncSchedule @ Deref @ HpcOmSimCode::Schedule::LEVELSCHEDULE { useFixedAssignments: true, .. })), _, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_7: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            str_7 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_170(txt.clone(), (str_7.clone()).clone(), a_extraFuncsNamespace.clone(), a_functionHead.clone(), i_zeroFuncSchedule.clone(), i_daeSchedule.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_name.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone(), i_odeSchedule.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_odeSchedule_threadTasks @ i_threadTasksOde, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_daeSchedule_threadTasks @ i_threadTasksDae, .. }, Deref @ HpcOmSimCode::Schedule::THREADSCHEDULE { threadTasks: i_zeroFuncSchedule_threadTasks @ i_threadTasksZeroFunc, .. })), a_modelNamePrefixStr, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_8: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            str_8 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_183(txt.clone(), (str_8.clone()).clone(), (a_modelNamePrefixStr.clone()).clone(), i_zeroFuncSchedule_threadTasks.clone(), i_daeSchedule_threadTasks.clone(), i_odeSchedule_threadTasks.clone(), a_functionHead.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone(), i_threadTasksZeroFunc.clone(), i_threadTasksDae.clone(), a_type.clone(), i_threadTasksOde.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Some((Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_odeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_daeSchedule_tasks }, Deref @ HpcOmSimCode::Schedule::TASKDEPSCHEDULE { tasks: i_zeroFuncSchedule_tasks })), _, a_extraFuncsNamespace, a_type, a_useFlatArrayNotation, a_name, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen, a_functionHead) => {
            let mut str_9: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            str_9 = (Tpl::textString(a_type.clone())?).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_184(txt.clone(), (str_9.clone()).clone(), a_name.clone(), a_functionHead.clone(), i_zeroFuncSchedule_tasks.clone(), i_daeSchedule_tasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_type.clone(), a_allEquationsPlusWhen.clone(), i_odeSchedule_tasks.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, a_extraFuncsNamespace, _, _, _, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_186(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_name: Arc<Absyn::Path>, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_allEquationsPlusWhen.clone(), in_a_name.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_schedulesOpt.clone(), in_a_modelNamePrefixStr.clone(), in_a_useFlatArrayNotation.clone())) {
        (txt, i_simCode @ SimCode::SimCode { clockedPartitions: i_clockedPartitions, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }, a_allEquationsPlusWhen, a_name, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_schedulesOpt, a_modelNamePrefixStr, a_useFlatArrayNotation) => {
            let mut txt_28: Tpl::Text;
            let mut l_functionHead: Tpl::Text;
            let mut ret_26: ArcStr = arcstr::literal!("");
            let mut l_type: Tpl::Text;
            let mut ret_24: bool = false;
            let mut ret_23: bool = false;
            let mut ret_22: ArcStr = arcstr::literal!("");
            let mut l_measureTimeEvaluateZeroFuncEnd: Tpl::Text;
            let mut ret_20: bool = false;
            let mut ret_19: bool = false;
            let mut ret_18: ArcStr = arcstr::literal!("");
            let mut l_measureTimeEvaluateZeroFuncStart: Tpl::Text;
            let mut ret_16: bool = false;
            let mut ret_15: bool = false;
            let mut ret_14: ArcStr = arcstr::literal!("");
            let mut l_measureTimeEvaluateAllEnd: Tpl::Text;
            let mut ret_12: bool = false;
            let mut ret_11: bool = false;
            let mut ret_10: ArcStr = arcstr::literal!("");
            let mut l_measureTimeEvaluateAllStart: Tpl::Text;
            let mut ret_8: bool = false;
            let mut ret_7: bool = false;
            let mut ret_6: ArcStr = arcstr::literal!("");
            let mut l_measureTimeEvaluateOdeEnd: Tpl::Text;
            let mut ret_4: bool = false;
            let mut ret_3: bool = false;
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut l_measureTimeEvaluateOdeStart: Tpl::Text;
            let mut l_varDecls: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_varDecls = Tpl::emptyTxt.clone();
            ret_2 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_3 = stringEq((ret_2.clone()).clone(), (literal!("none")).clone());
            ret_4 = boolNot(ret_3.clone());
            l_measureTimeEvaluateOdeStart = fun_153(Tpl::emptyTxt.clone(), ret_4.clone())?;
            ret_6 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_7 = stringEq((ret_6.clone()).clone(), (literal!("none")).clone());
            ret_8 = boolNot(ret_7.clone());
            l_measureTimeEvaluateOdeEnd = fun_154(Tpl::emptyTxt.clone(), ret_8.clone())?;
            ret_10 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_11 = stringEq((ret_10.clone()).clone(), (literal!("none")).clone());
            ret_12 = boolNot(ret_11.clone());
            l_measureTimeEvaluateAllStart = fun_155(Tpl::emptyTxt.clone(), ret_12.clone())?;
            ret_14 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_15 = stringEq((ret_14.clone()).clone(), (literal!("none")).clone());
            ret_16 = boolNot(ret_15.clone());
            l_measureTimeEvaluateAllEnd = fun_156(Tpl::emptyTxt.clone(), ret_16.clone())?;
            ret_18 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_19 = stringEq((ret_18.clone()).clone(), (literal!("none")).clone());
            ret_20 = boolNot(ret_19.clone());
            l_measureTimeEvaluateZeroFuncStart = fun_157(Tpl::emptyTxt.clone(), ret_20.clone())?;
            ret_22 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_23 = stringEq((ret_22.clone()).clone(), (literal!("none")).clone());
            ret_24 = boolNot(ret_23.clone());
            l_measureTimeEvaluateZeroFuncEnd = fun_158(Tpl::emptyTxt.clone(), ret_24.clone())?;
            ret_26 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
            l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_26.clone()).clone())?;
            l_functionHead = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//using type: ")).clone() }))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_type.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead.clone(), a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateZeroFuncs(const UPDATETYPE command)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::pushBlock(l_functionHead.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_measureTimeEvaluateZeroFuncStart.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("evaluateParallel(command, 1);\n")).clone() }))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_measureTimeEvaluateZeroFuncEnd.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::popBlock(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("bool ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead.clone(), a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateAll(const UPDATETYPE command)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::pushBlock(l_functionHead.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_measureTimeEvaluateAllStart.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_28 = CodegenCppCommon::timeEventLength(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            l_functionHead = CodegenCpp::createTimeConditionTreatments(l_functionHead.clone(), (Tpl::textString(txt_28.clone())?).clone(), i_clockedPartitions.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_varDecls.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("evaluateParallel(command, -1);\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_measureTimeEvaluateAllEnd.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("return _state_var_reinitialized;\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::popBlock(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead.clone(), a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("::evaluateODE(const UPDATETYPE command)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            l_functionHead = Tpl::pushBlock(l_functionHead.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_measureTimeEvaluateOdeStart.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("evaluateParallel(command, 0);\n")).clone() }))?;
            l_functionHead = Tpl::writeText(l_functionHead.clone(), l_measureTimeEvaluateOdeEnd.clone())?;
            l_functionHead = Tpl::softNewLine(l_functionHead.clone())?;
            l_functionHead = Tpl::popBlock(l_functionHead.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("//evaluateMode = 0 : evaluateODE\n")).clone(), (literal!("//evaluateMode < 0 : evaluateAll\n")).clone(), (literal!("//evaluateMode > 0 : evaluateZeroFunc\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            l_functionHead = CodegenCpp::lastIdentOfPath(l_functionHead.clone(), a_name.clone())?;
            l_functionHead = Tpl::writeTok(l_functionHead.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateParallel(const UPDATETYPE command, int evaluateMode)")).clone() }))?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, l_varDecls) = fun_185(txt.clone(), a_schedulesOpt.clone(), (a_modelNamePrefixStr.clone()).clone(), a_extraFuncsNamespace.clone(), l_type.clone(), a_useFlatArrayNotation.clone(), a_name.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), i_simCode.clone(), l_varDecls.clone(), a_allEquationsPlusWhen.clone(), l_functionHead.clone())?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _, _, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn generateParallelEvaluate(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_name: Arc<Absyn::Path>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_schedulesOpt: Option<(Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>, Arc<HpcOmSimCode::Schedule>)>, mut a_context: SimCodeFunction::Context, mut a_stateDerVectorName: Tpl::Text, mut a_modelNamePrefixStr: ArcStr, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_stateDerVectorName: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_186(txt.clone(), a_simCode.clone(), a_allEquationsPlusWhen.clone(), a_name.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_schedulesOpt.clone(), (a_modelNamePrefixStr.clone()).clone(), a_useFlatArrayNotation.clone())?;
    out_a_stateDerVectorName = a_stateDerVectorName.clone();
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, out_a_stateDerVectorName))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_188(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_index, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PREFETCH(&__z[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("], 0, 3);")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_188(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_189(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { vars: SimCodeVar::SimVars { stateVars: ref i_vars_stateVars, .. }, .. }, .. }) => {
            let mut ret_2: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            ret_0 = (i_vars_stateVars.clone().len() as i32);
            ret_1 = intSub(ret_0.clone(), 1);
            ret_2 = List::intRange3(0, 8, ret_1.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_188(txt.clone(), ret_2.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn generateStateVarPrefetchCode(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    out_txt = fun_189(txt.clone(), a_simCode.clone())?;
    out_a_extraFuncs = a_extraFuncs.clone();
    out_a_extraFuncsDecl = a_extraFuncsDecl.clone();
    out_a_extraFuncsNamespace = a_extraFuncsNamespace.clone();
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_191(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_task, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelCodeForTask(txt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_191(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_192(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_task, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelCodeForTask(txt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_192(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_193(mut in_txt: Tpl::Text, mut in_a_tasksOfLevel: HpcOmSimCode::TaskList, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_tasksOfLevel.clone(), in_a_allEquationsPlusWhen.clone(), in_a_iType.clone(), in_a_varDecls.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_useFlatArrayNotation.clone())) {
        (txt, HpcOmSimCode::TaskList::PARALLELTASKLIST { tasks: i_tasks }, a_allEquationsPlusWhen, a_iType, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_191(l_odeEqs.clone(), i_tasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#pragma omp sections\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, HpcOmSimCode::TaskList::SERIALTASKLIST { tasks: i_tasks, .. }, a_allEquationsPlusWhen, a_iType, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_192(l_odeEqs.clone(), i_tasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#pragma omp master\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("#pragma omp barrier")).clone()], lastHasNewLine: false }))?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn generateLevelCodeForLevel(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevel: HpcOmSimCode::TaskList, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_193(txt.clone(), a_tasksOfLevel.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn generateLevelCodeForTask(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iTask: Arc<HpcOmSimCode::Task>, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#pragma omp section\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = taskCode(out_txt.clone(), a_allEquationsPlusWhen.clone(), a_iTask.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_196(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_t.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_196(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_197(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_threadTasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_allEquationsPlusWhen) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(threadNum == ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_196(txt.clone(), i_threadTasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_197(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

pub fn generateLevelFixedCodeForLevel(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevel: metamodelica::Array<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut l_eqs: Tpl::Text;
    ret_1 = Arc::new(a_tasksOfLevel.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    l_eqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_eqs, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_197(l_eqs.clone(), ret_1.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_eqs = Tpl::popIter(l_eqs.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_eqs.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp barrier")).clone() }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_199(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut x_levelIdx: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx.clone(), (literal!("evaluateODE")).clone(), (a_iType.clone()).clone(), x_levelIdx.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_199(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_200(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut x_levelIdx: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx.clone(), (literal!("evaluateDAE")).clone(), (a_iType.clone()).clone(), x_levelIdx.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_200(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_201(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut x_levelIdx: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx.clone(), (literal!("evaluateZeroFuncs")).clone(), (a_iType.clone()).clone(), x_levelIdx.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_201(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_202(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("MeasureTimeValues *valuesStart = MeasureTime::getZeroValues();\n")).clone(), (literal!("MeasureTimeValues *valuesEnd = MeasureTime::getZeroValues();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_203(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("valuesStart")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_204(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayOdeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_205(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayDaeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateDaeThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_206(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayZeroFuncHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateZeroFuncThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_207(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("delete valuesStart;\n")).clone(), (literal!("delete valuesEnd;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_208(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone(), in_a_name.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_iThreadIdx, a_name) => {
            let mut ret_17: bool = false;
            let mut ret_16: bool = false;
            let mut ret_15: ArcStr = arcstr::literal!("");
            let mut ret_14: bool = false;
            let mut ret_13: bool = false;
            let mut ret_12: ArcStr = arcstr::literal!("");
            let mut ret_11: bool = false;
            let mut ret_10: bool = false;
            let mut ret_9: ArcStr = arcstr::literal!("");
            let mut ret_8: bool = false;
            let mut ret_7: bool = false;
            let mut ret_6: ArcStr = arcstr::literal!("");
            let mut ret_5: bool = false;
            let mut ret_4: bool = false;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_0 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_1 = stringEq((ret_0.clone()).clone(), (literal!("none")).clone());
            ret_2 = boolNot(ret_1.clone());
            txt = fun_202(txt.clone(), ret_2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("while(!_simulationFinished)\n")).clone(), (literal!("{\n")).clone(), (literal!("    //_evaluateBarrier.wait();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    if(_simulationFinished)\n")).clone(), (literal!("    {\n")).clone(), (literal!("        //_evaluateBarrier.wait();\n")).clone(), (literal!("        _levelBarrier.wait();\n")).clone(), (literal!("        break;\n")).clone(), (literal!("    }\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_4 = stringEq((ret_3.clone()).clone(), (literal!("none")).clone());
            ret_5 = boolNot(ret_4.clone());
            txt = fun_203(txt.clone(), ret_5.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_6 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_7 = stringEq((ret_6.clone()).clone(), (literal!("none")).clone());
            ret_8 = boolNot(ret_7.clone());
            txt = fun_204(txt.clone(), ret_8.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_9 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_10 = stringEq((ret_9.clone()).clone(), (literal!("none")).clone());
            ret_11 = boolNot(ret_10.clone());
            txt = fun_205(txt.clone(), ret_11.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_12 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_13 = stringEq((ret_12.clone()).clone(), (literal!("none")).clone());
            ret_14 = boolNot(ret_13.clone());
            txt = fun_206(txt.clone(), ret_14.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            ret_15 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_16 = stringEq((ret_15.clone()).clone(), (literal!("none")).clone());
            ret_17 = boolNot(ret_16.clone());
            txt = fun_207(txt.clone(), ret_17.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_209(mut in_txt: Tpl::Text, mut in_a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut in_a_name: Arc<Absyn::Path>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_tasksOfLevels.clone(), in_a_name.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, (i_odeTasksOfLevel, i_daeTasksOfLevel, i_zeroFuncTasksOfLevel), a_name, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut ret_3: bool = false;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut l_daeEqs: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_199(l_odeEqs.clone(), i_odeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_daeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_200(l_daeEqs.clone(), i_daeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs.clone())?;
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_zeroFuncEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_201(l_zeroFuncEqs.clone(), i_zeroFuncTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs.clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFuncODE_")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncAll_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncZeroFunc_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFunc")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            ret_3 = intGt(a_iThreadIdx.clone(), 0);
            txt = fun_208(txt.clone(), ret_3.clone(), a_iThreadIdx.clone(), a_name.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_210(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut x_levelIdx: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx.clone(), (literal!("evaluateODE")).clone(), (a_iType.clone()).clone(), x_levelIdx.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_210(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_211(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut x_levelIdx: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx.clone(), (literal!("evaluateDAE")).clone(), (a_iType.clone()).clone(), x_levelIdx.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_211(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_212(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tasks, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut x_levelIdx: i32 = 0;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_levelIdx = Tpl::getIteri_i0(txt.clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = generateLevelFixedCodeForThreadLevel(txt.clone(), a_allEquationsPlusWhen.clone(), i_tasks.clone(), a_iThreadIdx.clone(), (literal!("evaluateZeroFuncs")).clone(), (a_iType.clone()).clone(), x_levelIdx.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_212(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_213(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("MeasureTimeValues *valuesStart = MeasureTime::getZeroValues();\n")).clone(), (literal!("MeasureTimeValues *valuesEnd = MeasureTime::getZeroValues();")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_214(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("valuesStart")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_215(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayOdeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateODEThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_216(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayDaeHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateDaeThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_217(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iThreadIdx) => {
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeThreadArrayZeroFuncHpcom)[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateZeroFuncThread")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("valuesStart")).clone(), (literal!("valuesEnd")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_218(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("delete valuesStart;\n")).clone(), (literal!("delete valuesEnd;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_219(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iThreadIdx: i32, mut in_a_name: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_iThreadIdx.clone(), in_a_name.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_iThreadIdx, a_name) => {
            let mut ret_17: bool = false;
            let mut ret_16: bool = false;
            let mut ret_15: ArcStr = arcstr::literal!("");
            let mut ret_14: bool = false;
            let mut ret_13: bool = false;
            let mut ret_12: ArcStr = arcstr::literal!("");
            let mut ret_11: bool = false;
            let mut ret_10: bool = false;
            let mut ret_9: ArcStr = arcstr::literal!("");
            let mut ret_8: bool = false;
            let mut ret_7: bool = false;
            let mut ret_6: ArcStr = arcstr::literal!("");
            let mut ret_5: bool = false;
            let mut ret_4: bool = false;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_0 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_1 = stringEq((ret_0.clone()).clone(), (literal!("none")).clone());
            ret_2 = boolNot(ret_1.clone());
            txt = fun_213(txt.clone(), ret_2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("while(!_simulationFinished)\n")).clone(), (literal!("{\n")).clone(), (literal!("    //_evaluateBarrier.wait();\n")).clone(), (literal!("    _levelBarrier.wait();\n")).clone(), (literal!("    if(_simulationFinished)\n")).clone(), (literal!("    {\n")).clone(), (literal!("        //_evaluateBarrier.wait();\n")).clone(), (literal!("        _levelBarrier.wait();\n")).clone(), (literal!("        break;\n")).clone(), (literal!("    }\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            ret_3 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_4 = stringEq((ret_3.clone()).clone(), (literal!("none")).clone());
            ret_5 = boolNot(ret_4.clone());
            txt = fun_214(txt.clone(), ret_5.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_6 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_7 = stringEq((ret_6.clone()).clone(), (literal!("none")).clone());
            ret_8 = boolNot(ret_7.clone());
            txt = fun_215(txt.clone(), ret_8.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_9 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_10 = stringEq((ret_9.clone()).clone(), (literal!("none")).clone());
            ret_11 = boolNot(ret_10.clone());
            txt = fun_216(txt.clone(), ret_11.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            ret_12 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_13 = stringEq((ret_12.clone()).clone(), (literal!("none")).clone());
            ret_14 = boolNot(ret_13.clone());
            txt = fun_217(txt.clone(), ret_14.clone(), a_iThreadIdx.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("//_evaluateBarrier.wait();\n")).clone(), (literal!("_levelBarrier.wait();\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            ret_15 = (Flags::getConfigString(Flags::PROFILING_LEVEL.clone())?).clone();
            ret_16 = stringEq((ret_15.clone()).clone(), (literal!("none")).clone());
            ret_17 = boolNot(ret_16.clone());
            txt = fun_218(txt.clone(), ret_17.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_220(mut in_txt: Tpl::Text, mut in_a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut in_a_name: Arc<Absyn::Path>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_tasksOfLevels.clone(), in_a_name.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_iThreadIdx.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, (i_odeTasksOfLevel, i_daeTasksOfLevel, i_zeroFuncTasksOfLevel), a_name, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_iThreadIdx, a_allEquationsPlusWhen) => {
            let mut ret_3: bool = false;
            let mut l_zeroFuncEqs: Tpl::Text;
            let mut l_daeEqs: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_210(l_odeEqs.clone(), i_odeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            l_daeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_daeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_211(l_daeEqs.clone(), i_daeTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_daeEqs = Tpl::popIter(l_daeEqs.clone())?;
            l_zeroFuncEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_zeroFuncEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_212(l_zeroFuncEqs.clone(), i_zeroFuncTasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            l_zeroFuncEqs = Tpl::popIter(l_zeroFuncEqs.clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFuncODE_")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncAll_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncZeroFunc_")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFunc")).clone()], lastHasNewLine: false }))?;
            a_extraFuncsDecl = Tpl::writeStr(a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_extraFuncsDecl = Tpl::writeTok(a_extraFuncsDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_daeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_zeroFuncEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            ret_3 = intGt(a_iThreadIdx.clone(), 0);
            txt = fun_219(txt.clone(), ret_3.clone(), a_iThreadIdx.clone(), a_name.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, _, _, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_221(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut in_a_iThreadIdx: i32, mut in_a_varDecls: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone(), in_a_tasksOfLevels.clone(), in_a_iThreadIdx.clone(), in_a_varDecls.clone(), in_a_name.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_useFlatArrayNotation.clone())) {
        (txt, i_iType @ Deref @ "pthreads", a_allEquationsPlusWhen, a_tasksOfLevels, a_iThreadIdx, a_varDecls, a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_209(txt.clone(), a_tasksOfLevels.clone(), a_name.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (i_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, i_iType @ Deref @ "pthreads_spin", a_allEquationsPlusWhen, a_tasksOfLevels, a_iThreadIdx, a_varDecls, a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = fun_220(txt.clone(), a_tasksOfLevels.clone(), a_name.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (i_iType.clone()).clone(), a_iThreadIdx.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, _, a_varDecls, _, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn generateLevelFixedCodeForThread(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevels: (Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>), mut a_iThreadIdx: i32, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_name: Arc<Absyn::Path>, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_221(txt.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone(), a_tasksOfLevels.clone(), a_iThreadIdx.clone(), a_varDecls.clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_223(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_t.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_223(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_224(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iLevelIdx: i32, mut in_a_functionName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iLevelIdx.clone(), in_a_functionName.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iLevelIdx, mut a_functionName) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: i32 = 0;
            txt_0 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_functionName.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_level_")).clone() }))?;
            ret_0 = intAdd(a_iLevelIdx.clone(), 1);
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(ret_0.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeStartCode(txt.clone(), (literal!("measuredSchedulerStartValues")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_225(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    out_txt
}

fn fun_226(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_iLevelIdx: i32, mut in_a_functionName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_iLevelIdx.clone(), in_a_functionName.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_iLevelIdx, mut a_functionName) => {
            let mut txt_1: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut txt_0: Tpl::Text;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(*measureTimeSchedulerArrayHpcom_")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (a_functionName.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")[")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iLevelIdx.clone())).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt_1 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_functionName.clone()).clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_level_")).clone() }))?;
            ret_1 = intAdd(a_iLevelIdx.clone(), 1);
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(ret_1.clone())).clone())?;
            txt = CodegenCpp::generateMeasureTimeEndCode(txt.clone(), (literal!("measuredSchedulerStartValues")).clone(), (literal!("measuredSchedulerEndValues")).clone(), (Tpl::textString(txt_0.clone())?).clone(), (Tpl::textString(txt_1.clone())?).clone(), (literal!("MEASURETIME_MODELFUNCTIONS")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn generateLevelFixedCodeForThreadLevel(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_tasksOfLevel: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iThreadIdx: i32, mut a_functionName: ArcStr, mut a_iType: ArcStr, mut a_iLevelIdx: i32, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_3: bool = false;
    let mut ret_2: bool = false;
    let mut ret_1: bool = false;
    let mut l_tasks: Tpl::Text;
    l_tasks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_tasks, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_223(l_tasks.clone(), a_tasksOfLevel.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_tasks = Tpl::popIter(l_tasks.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//Start of Level ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_iLevelIdx.clone())).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    ret_1 = intEq(a_iThreadIdx.clone(), 0);
    out_txt = fun_224(out_txt.clone(), ret_1.clone(), a_iLevelIdx.clone(), (a_functionName.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    ret_2 = stringEq((Tpl::textString(l_tasks.clone())?).clone(), (literal!("")).clone());
    out_txt = fun_225(out_txt.clone(), ret_2.clone());
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_tasks.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_levelBarrier.wait();\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    ret_3 = intEq(a_iThreadIdx.clone(), 0);
    out_txt = fun_226(out_txt.clone(), ret_3.clone(), a_iLevelIdx.clone(), (a_functionName.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//End of Level ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_iLevelIdx.clone())).clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_228(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_228(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

pub fn function_HPCOM_TaskDep(mut txt: Tpl::Text, mut a_tasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iType: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_odeEqs: Tpl::Text;
    l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_odeEqs, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_228(l_odeEqs.clone(), a_tasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("int t[0];\n")).clone(), (literal!("#pragma omp parallel\n")).clone(), (literal!("{\n")).clone(), (literal!("  #pragma omp master\n")).clone(), (literal!("  {\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_odeEqs.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  }\n")).clone(), (literal!("}")).clone()], lastHasNewLine: false }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_230(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_p, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("t[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_p.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_230(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_231(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_p, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_p.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_231(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_232(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_parentDependencies: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_parentDependencies.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_parentDependencies) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("depend(in:")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_parentDependencies.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn function_HPCOM_TaskDep0(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_taskIn.clone(), in_a_allEquationsPlusWhen.clone(), in_a_iType.clone(), in_a_varDecls.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_useFlatArrayNotation.clone())) {
        (txt, (i_task @ Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, i_parents), a_allEquationsPlusWhen, a_iType, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut ret_5: bool = false;
            let mut ret_4: i32 = 0;
            let mut l_depIn: Tpl::Text;
            let mut l_taskDependencies: Tpl::Text;
            let mut l_parentDependencies: Tpl::Text;
            let mut l_taskEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            (l_taskEqs, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            l_parentDependencies = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_parentDependencies = lm_230(l_parentDependencies.clone(), i_parents.clone())?;
            l_parentDependencies = Tpl::popIter(l_parentDependencies.clone())?;
            l_taskDependencies = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_taskDependencies = lm_231(l_taskDependencies.clone(), i_parents.clone())?;
            l_taskDependencies = Tpl::popIter(l_taskDependencies.clone())?;
            ret_4 = (i_parents.clone().len() as i32);
            ret_5 = intGt(ret_4.clone(), 0);
            l_depIn = fun_232(Tpl::emptyTxt.clone(), ret_5.clone(), l_parentDependencies.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("//TG_NODE: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" TG_PARENTS: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_taskDependencies.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#pragma omp task ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_depIn.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("depend(out:t[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("])\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_taskEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_234(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_modelNamePrefixStr) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionNodes(txt.clone(), i_t.clone(), x_i.clone(), (literal!("Ode")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_234(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_235(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_modelNamePrefixStr) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionEdges(txt.clone(), i_t.clone(), x_i.clone(), (literal!("Ode")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_235(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_236(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_modelNamePrefixStr) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionNodes(txt.clone(), i_t.clone(), x_i.clone(), (literal!("All")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_236(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_237(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_modelNamePrefixStr) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionEdges(txt.clone(), i_t.clone(), x_i.clone(), (literal!("All")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_237(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_238(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_modelNamePrefixStr) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionNodes(txt.clone(), i_t.clone(), x_i.clone(), (literal!("ZeroFunc")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_238(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_239(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_modelNamePrefixStr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_modelNamePrefixStr) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = generateTbbConstructorExtensionEdges(txt.clone(), i_t.clone(), x_i.clone(), (literal!("ZeroFunc")).clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_239(txt.clone(), rest.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn generateTbbConstructorExtension(mut txt: Tpl::Text, mut a_odeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_daeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_zeroFuncTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_6: i32 = 0;
    let mut l_zeroFuncEdges: Tpl::Text;
    let mut l_zeroFuncNodes: Tpl::Text;
    let mut l_daeEdges: Tpl::Text;
    let mut l_daeNodes: Tpl::Text;
    let mut l_odeEdges: Tpl::Text;
    let mut l_odeNodes: Tpl::Text;
    l_odeNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_odeNodes = lm_234(l_odeNodes.clone(), a_odeTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_odeNodes = Tpl::popIter(l_odeNodes.clone())?;
    l_odeEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_odeEdges = lm_235(l_odeEdges.clone(), a_odeTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_odeEdges = Tpl::popIter(l_odeEdges.clone())?;
    l_daeNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_daeNodes = lm_236(l_daeNodes.clone(), a_daeTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_daeNodes = Tpl::popIter(l_daeNodes.clone())?;
    l_daeEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_daeEdges = lm_237(l_daeEdges.clone(), a_daeTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_daeEdges = Tpl::popIter(l_daeEdges.clone())?;
    l_zeroFuncNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_zeroFuncNodes = lm_238(l_zeroFuncNodes.clone(), a_zeroFuncTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_zeroFuncNodes = Tpl::popIter(l_zeroFuncNodes.clone())?;
    l_zeroFuncEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_zeroFuncEdges = lm_239(l_zeroFuncEdges.clone(), a_zeroFuncTasks.clone(), (a_modelNamePrefixStr.clone()).clone())?;
    l_zeroFuncEdges = Tpl::popIter(l_zeroFuncEdges.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("tbb::flow::continue_node<tbb::flow::continue_msg> *tbb_task;\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_odeNodes.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_odeEdges.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_daeNodes.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_daeEdges.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_zeroFuncNodes.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_zeroFuncEdges.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#if TBB_INTERFACE_VERSION >= 8000\n")).clone(), (literal!("_tbbArena = tbb::task_arena(")).clone()], lastHasNewLine: false }))?;
    ret_6 = Flags::getConfigInt(Flags::NUM_PROC.clone())?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(ret_6.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("_tbbArenaFunctorOde = TbbArenaFunctor(_tbbGraphOde,_tbbStartNodeOde);\n")).clone(), (literal!("_tbbArenaFunctorAll = TbbArenaFunctor(_tbbGraphAll,_tbbStartNodeAll);\n")).clone(), (literal!("_tbbArenaFunctorZeroFunc = TbbArenaFunctor(_tbbGraphZeroFunc,_tbbStartNodeZeroFunc);\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

pub fn generateTbbConstructorExtensionNodes(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_taskIndex: i32, mut in_a_funcSuffix: ArcStr, mut in_a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_taskIn.clone(), in_a_taskIndex.clone(), in_a_funcSuffix.clone(), in_a_modelNamePrefixStr.clone())) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _), a_taskIndex, a_funcSuffix, a_modelNamePrefixStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tbb_task = new tbb::flow::continue_node<tbb::flow::continue_msg>(_tbbGraph")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",VoidFunctionBody(bind<void>(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::taskFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",this)));\n")).clone(), (literal!("_tbbNodeList")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_taskIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") = tbb_task;")).clone() }))?;
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
fn lm_242(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_taskIndex: i32, mut in_a_funcSuffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_taskIndex.clone(), in_a_funcSuffix.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_p, tail: rest }, a_taskIndex, a_funcSuffix) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tbb::flow::make_edge(*(_tbbNodeList")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            ret_0 = intSub(i_p.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")),*(_tbbNodeList")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_taskIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")));")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_242(txt.clone(), rest.clone(), a_taskIndex.clone(), (a_funcSuffix.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_243(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_taskIndex: i32, mut in_a_funcSuffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_taskIndex.clone(), in_a_funcSuffix.clone()) {
        (mut txt, false, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_taskIndex, mut a_funcSuffix) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tbb::flow::make_edge(_tbbStartNode")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",*(_tbbNodeList")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".at(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_taskIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")));")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_244(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_taskIndex: i32, mut in_a_funcSuffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_taskIn.clone(), in_a_taskIndex.clone(), in_a_funcSuffix.clone())) {
        (txt, (Deref @ HpcOmSimCode::Task::CALCTASK { weighting: _, .. }, i_parents), a_taskIndex, a_funcSuffix) => {
            let mut ret_3: bool = false;
            let mut ret_2: i32 = 0;
            let mut l_startNodeEdge: Tpl::Text;
            let mut l_parentEdges: Tpl::Text;
            let mut txt = (*txt).clone();
            l_parentEdges = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_parentEdges = lm_242(l_parentEdges.clone(), i_parents.clone(), a_taskIndex.clone(), (a_funcSuffix.clone()).clone())?;
            l_parentEdges = Tpl::popIter(l_parentEdges.clone())?;
            ret_2 = (i_parents.clone().len() as i32);
            ret_3 = intEq(0, ret_2.clone());
            l_startNodeEdge = fun_243(Tpl::emptyTxt.clone(), ret_3.clone(), a_taskIndex.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_parentEdges.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_startNodeEdge.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateTbbConstructorExtensionEdges(mut txt: Tpl::Text, mut a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut a_taskIndex: i32, mut a_funcSuffix: ArcStr, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_244(txt.clone(), a_taskIn.clone(), a_taskIndex.clone(), (a_funcSuffix.clone()).clone())?;
    Ok(out_txt)
}

fn lm_246(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_name.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_name, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (literal!("Ode")).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_246(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_247(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_name.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_name, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (literal!("All")).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_247(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_248(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_name: Arc<Absyn::Path>, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_name.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_name, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = function_HPCOM_TaskDep_voidfunc0(txt.clone(), i_t.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (literal!("ZeroFunc")).clone(), a_name.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_248(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

pub fn function_HPCOM_TaskDep_voidfunc(mut txt: Tpl::Text, mut a_odeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_daeTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_zeroFuncTasks: Arc<metamodelica::List<(Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>)>>, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iType: ArcStr, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_funcTasksZeroFunc: Tpl::Text;
    let mut l_funcTasksDae: Tpl::Text;
    let mut l_funcTasksOde: Tpl::Text;
    l_funcTasksOde = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_funcTasksOde, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_246(l_funcTasksOde.clone(), a_odeTasks.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_funcTasksOde = Tpl::popIter(l_funcTasksOde.clone())?;
    l_funcTasksDae = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_funcTasksDae, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_247(l_funcTasksDae.clone(), a_daeTasks.clone(), a_useFlatArrayNotation.clone(), out_a_extraFuncsNamespace.clone(), out_a_extraFuncsDecl.clone(), out_a_extraFuncs.clone(), a_simCode.clone(), out_a_varDecls.clone(), a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_funcTasksDae = Tpl::popIter(l_funcTasksDae.clone())?;
    l_funcTasksZeroFunc = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_funcTasksZeroFunc, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_248(l_funcTasksZeroFunc.clone(), a_zeroFuncTasks.clone(), a_useFlatArrayNotation.clone(), out_a_extraFuncsNamespace.clone(), out_a_extraFuncsDecl.clone(), out_a_extraFuncs.clone(), a_simCode.clone(), out_a_varDecls.clone(), a_name.clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_funcTasksZeroFunc = Tpl::popIter(l_funcTasksZeroFunc.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_funcTasksOde.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_funcTasksDae.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_funcTasksZeroFunc.clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_250(mut in_txt: Tpl::Text, mut in_a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_funcSuffix: ArcStr, mut in_a_name: Arc<Absyn::Path>, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_taskIn.clone(), in_a_allEquationsPlusWhen.clone(), in_a_iType.clone(), in_a_funcSuffix.clone(), in_a_name.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_useFlatArrayNotation.clone())) {
        (txt, (i_task @ Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, .. }, _), a_allEquationsPlusWhen, a_iType, a_funcSuffix, a_name, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_taskEqs: Tpl::Text;
            let mut l_tempvarDecl: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_tempvarDecl = Tpl::emptyTxt.clone();
            (l_taskEqs, l_tempvarDecl, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), i_task.clone(), (a_iType.clone()).clone(), (literal!("")).clone(), l_tempvarDecl.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = CodegenCpp::lastIdentOfPath(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::taskFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcSuffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_tempvarDecl.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_taskEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, _, _, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn function_HPCOM_TaskDep_voidfunc0(mut txt: Tpl::Text, mut a_taskIn: (Arc<HpcOmSimCode::Task>, Arc<metamodelica::List<i32>>), mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iType: ArcStr, mut a_funcSuffix: ArcStr, mut a_name: Arc<Absyn::Path>, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_250(txt.clone(), a_taskIn.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (a_funcSuffix.clone()).clone(), a_name.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_252(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_relLock: Tpl::Text, mut in_a_assLock: Tpl::Text, mut in_a_taskEqsZeroFunc: Tpl::Text, mut in_a_taskEqsDae: Tpl::Text, mut in_a_taskEqsOde: Tpl::Text, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_iThreadIdx: i32, mut in_a_varDeclsLoc: Tpl::Text, mut in_a_mainThreadCode: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_mainThreadCode: Tpl::Text;
    (out_txt, out_a_mainThreadCode) = (match (in_txt.clone(), in_mArg.clone(), in_a_relLock.clone(), in_a_assLock.clone(), in_a_taskEqsZeroFunc.clone(), in_a_taskEqsDae.clone(), in_a_taskEqsOde.clone(), in_a_modelNamePrefixStr.clone(), in_a_iThreadIdx.clone(), in_a_varDeclsLoc.clone(), in_a_mainThreadCode.clone()) {
        (mut txt, false, _, _, mut a_taskEqsZeroFunc, mut a_taskEqsDae, mut a_taskEqsOde, mut a_modelNamePrefixStr, mut a_iThreadIdx, mut a_varDeclsLoc, mut a_mainThreadCode) => {
            a_mainThreadCode = Tpl::writeText(a_mainThreadCode.clone(), a_varDeclsLoc.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerStartValues = measuredSchedulerStartValues_0;\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerEndValues = measuredSchedulerEndValues_0;\n")).clone(), (literal!("#endif //MEASURETIME_MODELFUNCTIONS\n")).clone(), (literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            a_mainThreadCode = Tpl::pushBlock(a_mainThreadCode.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            a_mainThreadCode = Tpl::writeStr(a_mainThreadCode.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_mainThreadCode = Tpl::popBlock(a_mainThreadCode.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            a_mainThreadCode = Tpl::pushBlock(a_mainThreadCode.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            a_mainThreadCode = Tpl::writeStr(a_mainThreadCode.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_mainThreadCode = Tpl::popBlock(a_mainThreadCode.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            a_mainThreadCode = Tpl::pushBlock(a_mainThreadCode.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            a_mainThreadCode = Tpl::writeStr(a_mainThreadCode.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            a_mainThreadCode = Tpl::popBlock(a_mainThreadCode.clone())?;
            a_mainThreadCode = Tpl::writeTok(a_mainThreadCode.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_mainThreadCode.clone())
        },
        (mut txt, _, mut a_relLock, mut a_assLock, mut a_taskEqsZeroFunc, mut a_taskEqsDae, mut a_taskEqsOde, mut a_modelNamePrefixStr, mut a_iThreadIdx, mut a_varDeclsLoc, mut a_mainThreadCode) => {
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsOde.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsDae.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_taskEqsZeroFunc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("void ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::evaluateThreadFunc")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone(), (literal!("  #ifdef MEASURETIME_MODELFUNCTIONS\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MeasureTimeValues *measuredSchedulerStartValues = measuredSchedulerStartValues_")).clone() }))?;
            ret_0 = intSub(a_iThreadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("MeasureTimeValues *measuredSchedulerEndValues = measuredSchedulerEndValues_")).clone()], lastHasNewLine: false }))?;
            ret_1 = intSub(a_iThreadIdx.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("#endif //MEASURETIME_MODELFUNCTIONS\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), a_varDeclsLoc.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("while(1)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_assLock.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("if(_terminateThreads)\n")).clone(), (literal!("   return;\n")).clone(), (literal!("\n")).clone(), (literal!("if(_evaluateMode == 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncODE_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else if(_evaluateMode < 0)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncAll_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("else\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThreadFuncZeroFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("();\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_relLock.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_mainThreadCode.clone())
        },
    });
    Ok((out_txt, out_a_mainThreadCode))
}

pub fn generateThreadFunc(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_threadTasksOde: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_threadTasksDae: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_threadTasksZeroFunc: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iType: ArcStr, mut a_iThreadIdx: i32, mut a_modelNamePrefixStr: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_mainThreadCode: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_mainThreadCode: Tpl::Text;
    let mut ret_6: bool = false;
    let mut l_relLock: Tpl::Text;
    let mut l_assLock: Tpl::Text;
    let mut l_taskEqsZeroFunc: Tpl::Text;
    let mut l_taskEqsDae: Tpl::Text;
    let mut l_taskEqsOde: Tpl::Text;
    let mut l_varDeclsLoc: Tpl::Text;
    l_varDeclsLoc = Tpl::emptyTxt.clone();
    (l_taskEqsOde, l_varDeclsLoc, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = parallelThreadCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), a_threadTasksOde.clone(), a_iThreadIdx.clone(), (a_iType.clone()).clone(), (literal!("_lockOde")).clone(), l_varDeclsLoc.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), (literal!("evaluateODE")).clone(), a_useFlatArrayNotation.clone())?;
    (l_taskEqsDae, l_varDeclsLoc, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = parallelThreadCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), a_threadTasksDae.clone(), a_iThreadIdx.clone(), (a_iType.clone()).clone(), (literal!("_lockDae")).clone(), l_varDeclsLoc.clone(), a_simCode.clone(), out_a_extraFuncs.clone(), out_a_extraFuncsDecl.clone(), out_a_extraFuncsNamespace.clone(), (literal!("evaluateAll")).clone(), a_useFlatArrayNotation.clone())?;
    (l_taskEqsZeroFunc, l_varDeclsLoc, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = parallelThreadCode(Tpl::emptyTxt.clone(), a_allEquationsPlusWhen.clone(), a_threadTasksZeroFunc.clone(), a_iThreadIdx.clone(), (a_iType.clone()).clone(), (literal!("_lockZeroFunc")).clone(), l_varDeclsLoc.clone(), a_simCode.clone(), out_a_extraFuncs.clone(), out_a_extraFuncsDecl.clone(), out_a_extraFuncsNamespace.clone(), (literal!("evaluateZeroFunc")).clone(), a_useFlatArrayNotation.clone())?;
    l_assLock = assignLockByLockName(Tpl::emptyTxt.clone(), (intString(a_iThreadIdx.clone())).clone(), (literal!("th_lock")).clone(), (a_iType.clone()).clone())?;
    l_relLock = releaseLockByLockName(Tpl::emptyTxt.clone(), (intString(a_iThreadIdx.clone())).clone(), (literal!("th_lock1")).clone(), (a_iType.clone()).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void evaluateThreadFuncODE_")).clone() }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncAll_")).clone()], lastHasNewLine: false }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("();\n")).clone(), (literal!("void evaluateThreadFuncZeroFunc_")).clone()], lastHasNewLine: false }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl.clone(), (intString(a_iThreadIdx.clone())).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    ret_6 = intGt(a_iThreadIdx.clone(), 0);
    (out_txt, out_a_mainThreadCode) = fun_252(txt.clone(), ret_6.clone(), l_relLock.clone(), l_assLock.clone(), l_taskEqsZeroFunc.clone(), l_taskEqsDae.clone(), l_taskEqsOde.clone(), (a_modelNamePrefixStr.clone()).clone(), a_iThreadIdx.clone(), l_varDeclsLoc.clone(), a_mainThreadCode.clone())?;
    out_a_varDecls = a_varDecls.clone();
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace, out_a_mainThreadCode))
}

fn fun_254(mut in_txt: Tpl::Text, mut in_a_tt: Arc<HpcOmSimCode::Task>, mut in_a_iType: ArcStr, mut in_a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_tt.clone(), in_a_iType.clone(), in_a_iLockPrefix.clone())) {
        (txt, i_task @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: true, .. }, a_iType, a_iLockPrefix) => {
            let mut txt = (*txt).clone();
            txt = assignLockByDepTask(txt.clone(), i_task.clone(), (a_iLockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
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
fn lm_255(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut in_a_iType: ArcStr, mut in_a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_iType.clone(), in_a_iLockPrefix.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_iType, a_iLockPrefix) => {
            let mut txt = (*txt).clone();
            txt = fun_254(txt.clone(), i_tt.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_255(txt.clone(), rest.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_256(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn function_HPCOM_assignThreadLocks(mut txt: Tpl::Text, mut a_iThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iLockPrefix: ArcStr, mut a_iThreadNum: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: bool = false;
    let mut l_lockAssign: Tpl::Text;
    l_lockAssign = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_lockAssign = lm_255(l_lockAssign.clone(), a_iThreadTasks.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
    l_lockAssign = Tpl::popIter(l_lockAssign.clone())?;
    ret_1 = intNe(a_iThreadNum.clone(), 0);
    out_txt = fun_256(txt.clone(), ret_1.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(threadNum == ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_iThreadNum.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_lockAssign.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok(out_txt)
}

fn fun_258(mut in_txt: Tpl::Text, mut in_a_tt: Arc<HpcOmSimCode::Task>, mut in_a_iType: ArcStr, mut in_a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_tt.clone(), in_a_iType.clone(), in_a_iLockPrefix.clone())) {
        (txt, i_tt @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: false, .. }, a_iType, a_iLockPrefix) => {
            let mut txt = (*txt).clone();
            txt = releaseLockByDepTask(txt.clone(), i_tt.clone(), (a_iLockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
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
fn lm_259(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut in_a_iType: ArcStr, mut in_a_iLockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_iType.clone(), in_a_iLockPrefix.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_iType, a_iLockPrefix) => {
            let mut txt = (*txt).clone();
            txt = fun_258(txt.clone(), i_tt.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_259(txt.clone(), rest.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_260(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn function_HPCOM_releaseThreadLocks(mut txt: Tpl::Text, mut a_iThreadTasks: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iLockPrefix: ArcStr, mut a_iThreadNum: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: bool = false;
    let mut l_lockAssign: Tpl::Text;
    l_lockAssign = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_lockAssign = lm_259(l_lockAssign.clone(), a_iThreadTasks.clone(), (a_iType.clone()).clone(), (a_iLockPrefix.clone()).clone())?;
    l_lockAssign = Tpl::popIter(l_lockAssign.clone())?;
    ret_1 = intNe(a_iThreadNum.clone(), 0);
    out_txt = fun_260(txt.clone(), ret_1.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(threadNum == ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_iThreadNum.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_lockAssign.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok(out_txt)
}

fn lm_262(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_iThreadNum: i32, mut in_a_extraFunctionName: ArcStr, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_lockPrefix: ArcStr, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_iThreadNum.clone(), in_a_extraFunctionName.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_lockPrefix.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_useFlatArrayNotation, a_iThreadNum, a_extraFunctionName, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_lockPrefix, a_iType, a_allEquationsPlusWhen) => {
            let mut x_i0: i32 = 0;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt_0 = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_extraFunctionName.clone()).clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_Th")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(a_iThreadNum.clone())).clone())?;
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = parallelThreadCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), x_i0.clone(), (a_iType.clone()).clone(), (a_lockPrefix.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), (Tpl::textString(txt_0.clone())?).clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_262(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_iThreadNum.clone(), (a_extraFunctionName.clone()).clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn fun_263(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("switch(threadNum) ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_264(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_265(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_iMaxThreadNumber: i32, mut in_a_functionCalls: Tpl::Text, mut in_a_iThreadNum: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_iMaxThreadNumber.clone(), in_a_functionCalls.clone(), in_a_iThreadNum.clone())) {
        (txt, Deref @ "openmp", a_iMaxThreadNumber, a_functionCalls, a_iThreadNum) => {
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intEq(a_iThreadNum.clone(), 0);
            txt = fun_263(txt.clone(), ret_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadNum.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(":\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_functionCalls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("break;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            ret_1 = intEq(a_iThreadNum.clone(), a_iMaxThreadNumber.clone());
            txt = fun_264(txt.clone(), ret_1.clone())?;
            txt.clone()
        },
        (txt, Deref @ "mpi", _, a_functionCalls, a_iThreadNum) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (world_rank == ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_iThreadNum.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_functionCalls.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, a_functionCalls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_functionCalls.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn parallelThreadCodeWithSplit(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_threadTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iThreadNum: i32, mut a_iMaxThreadNumber: i32, mut a_iType: ArcStr, mut a_lockPrefix: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFunctionName: ArcStr, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>>> = metamodelica::nil();
    let mut l_functionCalls: Tpl::Text;
    ret_1 = List::partition(a_threadTaskList.clone(), 100)?;
    l_functionCalls = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_functionCalls, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_262(l_functionCalls.clone(), ret_1.clone(), a_useFlatArrayNotation.clone(), a_iThreadNum.clone(), (a_extraFunctionName.clone()).clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_functionCalls = Tpl::popIter(l_functionCalls.clone())?;
    out_txt = fun_265(txt.clone(), (a_iType.clone()).clone(), a_iMaxThreadNumber.clone(), l_functionCalls.clone(), a_iThreadNum.clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_267(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_lockPrefix: ArcStr, mut in_a_iType: ArcStr, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_lockPrefix.clone(), in_a_iType.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _, _, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_tt, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_lockPrefix, a_iType, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = taskCode(txt.clone(), a_allEquationsPlusWhen.clone(), i_tt.clone(), (a_iType.clone()).clone(), (a_lockPrefix.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_267(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

pub fn parallelThreadCode(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_threadTaskList: Arc<metamodelica::List<Arc<HpcOmSimCode::Task>>>, mut a_iPartitionIndex: i32, mut a_iType: ArcStr, mut a_lockPrefix: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_extraFunctionName: ArcStr, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_threadTasks: Tpl::Text;
    l_threadTasks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_threadTasks, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = lm_267(l_threadTasks.clone(), a_threadTaskList.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone(), a_allEquationsPlusWhen.clone())?;
    l_threadTasks = Tpl::popIter(l_threadTasks.clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_a_extraFuncs = Tpl::writeText(out_a_extraFuncs.clone(), out_a_extraFuncsNamespace.clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
    out_a_extraFuncs = Tpl::writeStr(out_a_extraFuncs.clone(), (a_extraFunctionName.clone()).clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_a_extraFuncs = Tpl::writeStr(out_a_extraFuncs.clone(), (intString(a_iPartitionIndex.clone())).clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("()\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
    out_a_extraFuncs = Tpl::pushBlock(out_a_extraFuncs.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_a_extraFuncs = Tpl::writeText(out_a_extraFuncs.clone(), l_threadTasks.clone())?;
    out_a_extraFuncs = Tpl::softNewLine(out_a_extraFuncs.clone())?;
    out_a_extraFuncs = Tpl::popBlock(out_a_extraFuncs.clone())?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_a_extraFuncs = Tpl::writeTok(out_a_extraFuncs.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl.clone(), (a_extraFunctionName.clone()).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_a_extraFuncsDecl = Tpl::writeStr(out_a_extraFuncsDecl.clone(), (intString(a_iPartitionIndex.clone())).clone())?;
    out_a_extraFuncsDecl = Tpl::writeTok(out_a_extraFuncsDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    out_txt = Tpl::writeStr(txt.clone(), (a_extraFunctionName.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_iPartitionIndex.clone())).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("();")).clone() }))?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn lm_269(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = equationNamesHPCOM_(txt.clone(), i_eq.clone(), a_allEquationsPlusWhen.clone(), SimCodeFunction::contextSimulationNonDiscrete().clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_269(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

fn lm_270(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_useFlatArrayNotation: bool, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncs: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_varDecls: Tpl::Text, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_useFlatArrayNotation.clone(), in_a_extraFuncsNamespace.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncs.clone(), in_a_simCode.clone(), in_a_varDecls.clone(), in_a_allEquationsPlusWhen.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, _, a_varDecls, _) => {
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_useFlatArrayNotation, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_simCode, a_varDecls, a_allEquationsPlusWhen) => {
            let mut txt = (*txt).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_varDecls, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace) = equationNamesHPCOM_(txt.clone(), i_eq.clone(), a_allEquationsPlusWhen.clone(), SimCodeFunction::contextSimulationNonDiscrete().clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_270(txt.clone(), rest.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone())?;
            (txt.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_extraFuncsNamespace, out_a_extraFuncsDecl, out_a_extraFuncs, out_a_varDecls))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_271(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_task, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_task.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_271(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_272(mut in_txt: Tpl::Text, mut in_a_iTask: Arc<HpcOmSimCode::Task>, mut in_a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_iType: ArcStr, mut in_a_lockPrefix: ArcStr, mut in_a_varDecls: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_extraFuncs: Tpl::Text, mut in_a_extraFuncsDecl: Tpl::Text, mut in_a_extraFuncsNamespace: Tpl::Text, mut in_a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iTask.clone(), in_a_allEquationsPlusWhen.clone(), in_a_iType.clone(), in_a_lockPrefix.clone(), in_a_varDecls.clone(), in_a_simCode.clone(), in_a_extraFuncs.clone(), in_a_extraFuncsDecl.clone(), in_a_extraFuncsNamespace.clone(), in_a_useFlatArrayNotation.clone())) {
        (txt, Deref @ HpcOmSimCode::Task::CALCTASK { index: i_task_index, eqIdc: i_task_eqIdc, .. }, a_allEquationsPlusWhen, _, _, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_varDeclsLocal: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_269(l_odeEqs.clone(), i_task_eqIdc.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            l_varDeclsLocal = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// Task ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// End Task ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_index.clone())).clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, Deref @ HpcOmSimCode::Task::CALCTASK_LEVEL { nodeIdc: i_task_nodeIdc, eqIdc: i_task_eqIdc, .. }, a_allEquationsPlusWhen, _, _, a_varDecls, a_simCode, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, a_useFlatArrayNotation) => {
            let mut l_taskStr: Tpl::Text;
            let mut l_varDeclsLocal: Tpl::Text;
            let mut l_odeEqs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_extraFuncs = (*a_extraFuncs).clone();
            let mut a_extraFuncsDecl = (*a_extraFuncsDecl).clone();
            let mut a_extraFuncsNamespace = (*a_extraFuncsNamespace).clone();
            l_odeEqs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_odeEqs, a_extraFuncsNamespace, a_extraFuncsDecl, a_extraFuncs, a_varDecls) = lm_270(l_odeEqs.clone(), i_task_eqIdc.clone(), a_useFlatArrayNotation.clone(), a_extraFuncsNamespace.clone(), a_extraFuncsDecl.clone(), a_extraFuncs.clone(), a_simCode.clone(), a_varDecls.clone(), a_allEquationsPlusWhen.clone())?;
            l_odeEqs = Tpl::popIter(l_odeEqs.clone())?;
            l_taskStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_taskStr = lm_271(l_taskStr.clone(), i_task_nodeIdc.clone())?;
            l_taskStr = Tpl::popIter(l_taskStr.clone())?;
            l_varDeclsLocal = Tpl::emptyTxt.clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// Tasks ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_taskStr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_odeEqs.clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, i_task @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: false, .. }, _, a_iType, a_lockPrefix, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            let mut l_assLck: Tpl::Text;
            let mut txt = (*txt).clone();
            l_assLck = assignLockByDepTask(Tpl::emptyTxt.clone(), i_task.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_assLck.clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, i_task @ Deref @ HpcOmSimCode::Task::DEPTASK { outgoing: true, .. }, _, a_iType, a_lockPrefix, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            let mut l_relLck: Tpl::Text;
            let mut txt = (*txt).clone();
            l_relLck = releaseLockByDepTask(Tpl::emptyTxt.clone(), i_task.clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_relLck.clone())?;
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        (txt, _, _, _, _, a_varDecls, _, a_extraFuncs, a_extraFuncsDecl, a_extraFuncsNamespace, _) => {
            (txt.clone(), a_varDecls.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn taskCode(mut txt: Tpl::Text, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_iTask: Arc<HpcOmSimCode::Task>, mut a_iType: ArcStr, mut a_lockPrefix: ArcStr, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = fun_272(txt.clone(), a_iTask.clone(), a_allEquationsPlusWhen.clone(), (a_iType.clone()).clone(), (a_lockPrefix.clone()).clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn equationNamesHPCOM_(mut txt: Tpl::Text, mut a_idx: i32, mut a_allEquationsPlusWhen: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_1: Arc<SimCode::SimEqSystem>;
    let mut l_eq: Tpl::Text;
    ret_1 = HpcOmTaskGraph::getSimCodeEqByIndex(a_allEquationsPlusWhen.clone(), a_idx.clone())?;
    (l_eq, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = equationHPCOM_(Tpl::emptyTxt.clone(), ret_1.clone(), a_idx.clone(), a_context.clone(), a_varDecls.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), a_useFlatArrayNotation.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_eq.clone())?;
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

pub fn equationHPCOM_(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_idx: i32, mut a_context: SimCodeFunction::Context, mut a_varDecls: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text, mut a_useFlatArrayNotation: bool) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    (out_txt, _) = CodegenCpp::equation_function_call(txt.clone(), a_eq.clone(), a_context.clone(), a_simCode.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluate")).clone() })))?;
    out_a_varDecls = a_varDecls.clone();
    out_a_extraFuncs = a_extraFuncs.clone();
    out_a_extraFuncsDecl = a_extraFuncsDecl.clone();
    out_a_extraFuncsNamespace = a_extraFuncsNamespace.clone();
    Ok((out_txt, out_a_varDecls, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_276(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_threadIdx.clone())) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_threadIdx.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->join();")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn function_HPCOM_joinThread(mut txt: Tpl::Text, mut a_threadIdx: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_276(txt.clone(), (a_iType.clone()).clone(), (a_threadIdx.clone()).clone())?;
    Ok(out_txt)
}

fn fun_278(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_threadIdx.clone())) {
        (txt, Deref @ "openmp", _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_threadIdx.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn function_HPCOM_destroyThread(mut txt: Tpl::Text, mut a_threadIdx: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_278(txt.clone(), (a_iType.clone()).clone(), (a_threadIdx.clone()).clone())?;
    Ok(out_txt)
}

fn fun_280(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_threadIdx: i32, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_funcName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_threadIdx.clone(), in_a_modelNamePrefixStr.clone(), in_a_funcName.clone())) {
        (txt, Deref @ "openmp", _, _, _) => {
            txt.clone()
        },
        (txt, _, a_threadIdx, a_modelNamePrefixStr, a_funcName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("evaluateThread")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = new thread(bind(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_modelNamePrefixStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_funcName.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_threadIdx.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", this));")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn generateThread(mut txt: Tpl::Text, mut a_threadIdx: i32, mut a_iType: ArcStr, mut a_modelNamePrefixStr: ArcStr, mut a_funcName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_280(txt.clone(), (a_iType.clone()).clone(), a_threadIdx.clone(), (a_modelNamePrefixStr.clone()).clone(), (a_funcName.clone()).clone())?;
    Ok(out_txt)
}

pub fn getLockNameByDepTask(mut in_txt: Tpl::Text, mut in_a_depTask: Arc<HpcOmSimCode::Task>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_depTask.clone())) {
        (txt, Deref @ HpcOmSimCode::Task::DEPTASK { id: i_task_id, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_task_id.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("invalidLockTask")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initializeLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask.clone())?;
    out_txt = initializeLockByLockName(txt.clone(), (Tpl::textString(l_lockName.clone())?).clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
    Ok(out_txt)
}

fn fun_284(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_lockName.clone(), in_a_lockPrefix.clone())) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_init_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = new alignedLock();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = new alignedSpinlock();")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initializeLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_284(txt.clone(), (a_iType.clone()).clone(), (a_lockName.clone()).clone(), (a_lockPrefix.clone()).clone())?;
    Ok(out_txt)
}

fn fun_286(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr, mut in_a_numberOfThreads: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_lockName.clone(), in_a_lockPrefix.clone(), in_a_numberOfThreads.clone())) {
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix, a_numberOfThreads) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numberOfThreads.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix, a_numberOfThreads) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_numberOfThreads.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn initializeBarrierByName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_numberOfThreads: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_286(txt.clone(), (a_iType.clone()).clone(), (a_lockName.clone()).clone(), (a_lockPrefix.clone()).clone(), a_numberOfThreads.clone())?;
    Ok(out_txt)
}

pub fn createLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask.clone())?;
    out_txt = createLockByLockName(txt.clone(), (Tpl::textString(l_lockName.clone())?).clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
    Ok(out_txt)
}

fn fun_289(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_lockName.clone(), in_a_lockPrefix.clone())) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_lock_t ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedLock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alignedSpinlock* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn createLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_289(txt.clone(), (a_iType.clone()).clone(), (a_lockName.clone()).clone(), (a_lockPrefix.clone()).clone())?;
    Ok(out_txt)
}

fn fun_291(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_lockName.clone(), in_a_lockPrefix.clone())) {
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("busywaiting_barrier ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("busywaiting_barrier ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn createBarrierByName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_numOfThreads: i32, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_291(txt.clone(), (a_iType.clone()).clone(), (a_lockName.clone()).clone(), (a_lockPrefix.clone()).clone())?;
    Ok(out_txt)
}

pub fn destroyLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask.clone())?;
    out_txt = destroyLockByLockName(txt.clone(), (Tpl::textString(l_lockName.clone())?).clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
    Ok(out_txt)
}

fn fun_294(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_lockName.clone(), in_a_lockPrefix.clone())) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_destroy_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("delete ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn destroyLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_294(txt.clone(), (a_iType.clone()).clone(), (a_lockName.clone()).clone(), (a_lockPrefix.clone()).clone())?;
    Ok(out_txt)
}

pub fn assignLockByDepTask(mut in_txt: Tpl::Text, mut in_a_depTask: Arc<HpcOmSimCode::Task>, mut in_a_lockPrefix: ArcStr, mut in_a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_depTask.clone(), in_a_lockPrefix.clone(), in_a_iType.clone())) {
        (txt, i_depTask @ Deref @ HpcOmSimCode::Task::DEPTASK { sourceTask: _, .. }, a_lockPrefix, a_iType) => {
            let mut l_lockName: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), i_depTask.clone())?;
            txt = assignLockByLockName(txt.clone(), (Tpl::textString(l_lockName.clone())?).clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn printCommunicationInfoVariables(mut txt: Tpl::Text, mut a_commInfo: HpcOmSimCode::CommunicationInfo) -> Tpl::Text {
    let mut out_txt: Tpl::Text;
    out_txt = txt.clone();
    out_txt
}

fn fun_298(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_lockName.clone(), in_a_lockPrefix.clone())) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_set_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->lock();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->lock();")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn assignLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_298(txt.clone(), (a_iType.clone()).clone(), (a_lockName.clone()).clone(), (a_lockPrefix.clone()).clone())?;
    Ok(out_txt)
}

pub fn releaseLockByDepTask(mut txt: Tpl::Text, mut a_depTask: Arc<HpcOmSimCode::Task>, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_lockName: Tpl::Text;
    l_lockName = getLockNameByDepTask(Tpl::emptyTxt.clone(), a_depTask.clone())?;
    out_txt = releaseLockByLockName(txt.clone(), (Tpl::textString(l_lockName.clone())?).clone(), (a_lockPrefix.clone()).clone(), (a_iType.clone()).clone())?;
    Ok(out_txt)
}

fn fun_301(mut in_txt: Tpl::Text, mut in_a_iType: ArcStr, mut in_a_lockName: ArcStr, mut in_a_lockPrefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iType.clone(), in_a_lockName.clone(), in_a_lockPrefix.clone())) {
        (txt, Deref @ "openmp", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omp_unset_lock(&")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->unlock();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ "pthreads_spin", a_lockName, a_lockPrefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_lockPrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_lockName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("->unlock();")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn releaseLockByLockName(mut txt: Tpl::Text, mut a_lockName: ArcStr, mut a_lockPrefix: ArcStr, mut a_iType: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_301(txt.clone(), (a_iType.clone()).clone(), (a_lockName.clone()).clone(), (a_lockPrefix.clone()).clone())?;
    Ok(out_txt)
}

pub fn mpiFinalize(mut txt: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("} // End sequential\n")).clone(), (literal!("MPI_Finalize();")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

pub fn mpiInit(mut txt: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("char** argvNotConst = const_cast<char**>(argv);\n")).clone(), (literal!("MPI_Init(&argc, &argvNotConst);\n")).clone(), (literal!("int world_rank, world_size;\n")).clone(), (literal!("MPI_Comm_size(MPI_COMM_WORLD, &world_size);\n")).clone(), (literal!("MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);\n")).clone(), (literal!("std::cout << \"Hello world! This is MPI process \" << world_rank\n")).clone(), (literal!("          << \" of \" << world_size << \" processes.\"  << endl;\n")).clone(), (literal!("\n")).clone(), (literal!("// Run simulation in sequential\n")).clone(), (literal!("if (0 == world_rank) {\n")).clone(), (literal!("  std::cout << \"Remark: Simulation is not (yet) MPI parallel!\\n\";")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

pub fn mpiRunCommandInRunScript(mut in_txt: Tpl::Text, mut in_a_type: ArcStr, mut in_a_getNumOfProcs: Tpl::Text, mut in_a_execCommandLinux: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_getNumOfProcs: Tpl::Text;
    let mut out_a_execCommandLinux: Tpl::Text;
    (out_txt, out_a_getNumOfProcs, out_a_execCommandLinux) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type.clone(), in_a_getNumOfProcs.clone(), in_a_execCommandLinux.clone())) {
        (txt, Deref @ "mpi", a_getNumOfProcs, a_execCommandLinux) => {
            let mut a_getNumOfProcs = (*a_getNumOfProcs).clone();
            let mut a_execCommandLinux = (*a_execCommandLinux).clone();
            a_execCommandLinux = Tpl::writeTok(a_execCommandLinux.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mpirun -np ${NPROCESSORS}")).clone() }))?;
            a_getNumOfProcs = Tpl::writeTok(a_getNumOfProcs.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("if [ $# -gt 0 ]; then\n")).clone(), (literal!("  NPROCESSORS=$1\n")).clone(), (literal!(" shift \n")).clone(), (literal!("else\n")).clone(), (literal!("  NPROCESSORS=1\n")).clone(), (literal!("fi\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            (txt.clone(), a_getNumOfProcs.clone(), a_execCommandLinux.clone())
        },
        (txt, _, a_getNumOfProcs, a_execCommandLinux) => {
            let mut a_execCommandLinux = (*a_execCommandLinux).clone();
            a_execCommandLinux = Tpl::writeTok(a_execCommandLinux.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("exec")).clone() }))?;
            (txt.clone(), a_getNumOfProcs.clone(), a_execCommandLinux.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_getNumOfProcs, out_a_execCommandLinux))
}

fn fun_306(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mpi")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn simulationMainRunScript(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut l_preRunCommandWindows: Tpl::Text;
    let mut l_0__: Tpl::Text;
    let mut l_execCommandLinux: Tpl::Text;
    let mut l_preRunCommandLinux: Tpl::Text;
    let mut ret_1: bool = false;
    let mut l_type: Tpl::Text;
    ret_1 = Flags::isSet(Flags::USEMPI.clone())?;
    l_type = fun_306(Tpl::emptyTxt.clone(), ret_1.clone())?;
    l_preRunCommandLinux = Tpl::emptyTxt.clone();
    l_execCommandLinux = Tpl::emptyTxt.clone();
    (l_0__, l_preRunCommandLinux, l_execCommandLinux) = mpiRunCommandInRunScript(Tpl::emptyTxt.clone(), (Tpl::textString(l_type.clone())?).clone(), l_preRunCommandLinux.clone(), l_execCommandLinux.clone())?;
    l_preRunCommandWindows = Tpl::emptyTxt.clone();
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = CodegenCpp::simulationMainRunScript(txt.clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), (Tpl::textString(l_preRunCommandLinux.clone())?).clone(), (Tpl::textString(l_preRunCommandWindows.clone())?).clone(), (Tpl::textString(l_execCommandLinux.clone())?).clone())?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_308(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -fopenmp")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_309(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -I\"$(INTEL_TBB_INCLUDE)\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_310(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/openmp")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_311(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-L$(INTEL_TBB_LIBS) $(INTEL_TBB_LIBRARIES) ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_312(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -fopenmp")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getAdditionalMakefileFlags(mut txt: Tpl::Text, mut a_additionalLinkerFlags__GCC: Tpl::Text, mut a_additionalLinkerFlags__MSVC: Tpl::Text, mut a_additionalCFlags__GCC: Tpl::Text, mut a_additionalCFlags__MSVC: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_additionalLinkerFlags__GCC: Tpl::Text;
    let mut out_a_additionalLinkerFlags__MSVC: Tpl::Text;
    let mut out_a_additionalCFlags__GCC: Tpl::Text;
    let mut out_a_additionalCFlags__MSVC: Tpl::Text;
    let mut ret_6: bool = false;
    let mut ret_5: bool = false;
    let mut ret_4: bool = false;
    let mut ret_3: bool = false;
    let mut ret_2: bool = false;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut l_type: Tpl::Text;
    ret_1 = (Flags::getConfigString(Flags::HPCOM_CODE.clone())?).clone();
    l_type = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
    ret_2 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("openmp")).clone());
    out_a_additionalCFlags__GCC = fun_308(a_additionalCFlags__GCC.clone(), ret_2.clone())?;
    ret_3 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("tbb")).clone());
    out_a_additionalCFlags__GCC = fun_309(out_a_additionalCFlags__GCC.clone(), ret_3.clone())?;
    ret_4 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("openmp")).clone());
    out_a_additionalCFlags__MSVC = fun_310(a_additionalCFlags__MSVC.clone(), ret_4.clone())?;
    ret_5 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("tbb")).clone());
    out_a_additionalLinkerFlags__GCC = fun_311(a_additionalLinkerFlags__GCC.clone(), ret_5.clone())?;
    ret_6 = stringEq((Tpl::textString(l_type.clone())?).clone(), (literal!("openmp")).clone());
    out_a_additionalLinkerFlags__GCC = fun_312(out_a_additionalLinkerFlags__GCC.clone(), ret_6.clone())?;
    out_txt = txt.clone();
    out_a_additionalLinkerFlags__MSVC = a_additionalLinkerFlags__MSVC.clone();
    Ok((out_txt, out_a_additionalLinkerFlags__GCC, out_a_additionalLinkerFlags__MSVC, out_a_additionalCFlags__GCC, out_a_additionalCFlags__MSVC))
}

pub fn simulationMakefile(mut txt: Tpl::Text, mut a_target: ArcStr, mut a_simCode: SimCode::SimCode, mut a_extraFuncs: Tpl::Text, mut a_extraFuncsDecl: Tpl::Text, mut a_extraFuncsNamespace: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_extraFuncs: Tpl::Text;
    let mut out_a_extraFuncsDecl: Tpl::Text;
    let mut out_a_extraFuncsNamespace: Tpl::Text;
    let mut ret_4: bool = false;
    let mut l_additionalLinkerFlags__MSVC: Tpl::Text;
    let mut l_additionalLinkerFlags__GCC: Tpl::Text;
    let mut l_additionalCFlags__MSVC: Tpl::Text;
    let mut l_additionalCFlags__GCC: Tpl::Text;
    l_additionalCFlags__GCC = Tpl::emptyTxt.clone();
    l_additionalCFlags__MSVC = Tpl::emptyTxt.clone();
    l_additionalLinkerFlags__GCC = Tpl::emptyTxt.clone();
    l_additionalLinkerFlags__MSVC = Tpl::emptyTxt.clone();
    (out_txt, l_additionalLinkerFlags__GCC, l_additionalLinkerFlags__MSVC, l_additionalCFlags__GCC, l_additionalCFlags__MSVC) = getAdditionalMakefileFlags(txt.clone(), l_additionalLinkerFlags__GCC.clone(), l_additionalLinkerFlags__MSVC.clone(), l_additionalCFlags__GCC.clone(), l_additionalCFlags__MSVC.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    ret_4 = Flags::isSet(Flags::USEMPI.clone())?;
    (out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace) = CodegenCpp::simulationMakefile(out_txt.clone(), (a_target.clone()).clone(), a_simCode.clone(), a_extraFuncs.clone(), a_extraFuncsDecl.clone(), a_extraFuncsNamespace.clone(), (Tpl::textString(l_additionalLinkerFlags__GCC.clone())?).clone(), (Tpl::textString(l_additionalLinkerFlags__MSVC.clone())?).clone(), (Tpl::textString(l_additionalCFlags__GCC.clone())?).clone(), (Tpl::textString(l_additionalCFlags__MSVC.clone())?).clone(), ret_4.clone())?;
    Ok((out_txt, out_a_extraFuncs, out_a_extraFuncsDecl, out_a_extraFuncsNamespace))
}

fn fun_315(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_hpcOmMemoryOpt.clone(), in_a_modelInfo.clone()) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { boolArraySize: mut i_boolArraySize, intArraySize: mut i_intArraySize, floatArraySize: mut i_floatArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_floatArraySize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_intArraySize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_boolArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::getPreVarsCount(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn numPreVarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_315(txt.clone(), a_hpcOmMemoryOpt.clone(), a_modelInfo.clone())?;
    Ok(out_txt)
}

fn fun_317(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_hpcOmMemoryOpt.clone(), in_a_modelInfo.clone()) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { floatArraySize: mut i_floatArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_floatArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numRealvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn numRealvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_317(txt.clone(), a_hpcOmMemoryOpt.clone(), a_modelInfo.clone())?;
    Ok(out_txt)
}

fn fun_319(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_hpcOmMemoryOpt.clone(), in_a_modelInfo.clone()) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { intArraySize: mut i_intArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_intArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numIntvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn numIntvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_319(txt.clone(), a_hpcOmMemoryOpt.clone(), a_modelInfo.clone())?;
    Ok(out_txt)
}

fn fun_321(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_hpcOmMemoryOpt.clone(), in_a_modelInfo.clone()) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { boolArraySize: mut i_boolArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_boolArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numBoolvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn numBoolvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_321(txt.clone(), a_hpcOmMemoryOpt.clone(), a_modelInfo.clone())?;
    Ok(out_txt)
}

fn fun_323(mut in_txt: Tpl::Text, mut in_a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_hpcOmMemoryOpt.clone(), in_a_modelInfo.clone()) {
        (mut txt, Some(HpcOmSimCode::MemoryMap::MEMORYMAP_ARRAY { stringArraySize: mut i_stringArraySize, .. }), _) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_stringArraySize.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_modelInfo) => {
            txt = CodegenCpp::numStringvars(txt.clone(), a_modelInfo.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn numStringvarsHpcom(mut txt: Tpl::Text, mut a_modelInfo: SimCode::ModelInfo, mut a_hpcOmMemoryOpt: Option<HpcOmSimCode::MemoryMap>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_323(txt.clone(), a_hpcOmMemoryOpt.clone(), a_modelInfo.clone())?;
    Ok(out_txt)
}

