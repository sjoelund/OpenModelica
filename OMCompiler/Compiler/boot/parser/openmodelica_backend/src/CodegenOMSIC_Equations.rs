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
use crate::CodegenC;
use crate::CodegenCFunctions;
use crate::CodegenUtil;
use crate::CodegenUtilSimulation;
use crate::SimCode;
use crate::SimCodeFunction;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn equationFunctionPrototypes(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_modelNamePrefixStr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_ix: Tpl::Text;
    l_ix = CodegenUtilSimulation::equationIndex(Tpl::emptyTxt.clone(), a_eq.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_txt = CodegenUtil::symbolName(out_txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("eqFunction")).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_ix.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(Data_Struct_something* data, Data_Struct_something* threadData);")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("resFunction")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("algSystFunction")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("eqFunction")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct omsi_function_t* this_function, const omsi_values* model_vars_and_params")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("struct omsi_function_t* this_function, const omsi_values* model_vars_and_params")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_real* res = (omsi_real*) data;")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_varDecls: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_varDecls.clone()) {
        (mut txt, false, mut a_varDecls) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/* Variables */\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_varDecls.clone())?;
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

pub fn generateEquationFunction(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_modelNamePrefixStr: ArcStr, mut a_modelFunctionnamePrefixStr: ArcStr, mut a_context: SimCodeFunction::Context, mut a_functionPrototypes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut ret_9: bool = false;
    let mut l_dataCast: Tpl::Text;
    let mut l_funcArguments: Tpl::Text;
    let mut l_funcName: Tpl::Text;
    let mut l_equationCode: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    let mut ret_2: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
    let mut l_equationInfos: Tpl::Text;
    let mut l_ix: Tpl::Text;
    l_ix = CodegenUtilSimulation::equationIndex(Tpl::emptyTxt.clone(), a_eq.clone())?;
    ret_2 = List::fill(a_eq.clone(), 1);
    l_equationInfos = CodegenUtilSimulation::dumpEqs(Tpl::emptyTxt.clone(), ret_2.clone())?;
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    (l_equationCode, l_varDecls, l_auxFunction) = equationCStr(Tpl::emptyTxt.clone(), a_eq.clone(), l_varDecls.clone(), l_auxFunction.clone(), a_context.clone())?;
    l_funcName = fun_53(Tpl::emptyTxt.clone(), a_eq.clone())?;
    l_funcArguments = fun_54(Tpl::emptyTxt.clone(), a_eq.clone())?;
    l_dataCast = fun_55(Tpl::emptyTxt.clone(), a_eq.clone())?;
    out_a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_a_functionPrototypes = CodegenUtil::symbolName(out_a_functionPrototypes.clone(), (a_modelNamePrefixStr.clone()).clone(), (Tpl::textString(l_funcName.clone())?).clone())?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_a_functionPrototypes = Tpl::writeText(out_a_functionPrototypes.clone(), l_ix.clone())?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
    out_a_functionPrototypes = Tpl::writeText(out_a_functionPrototypes.clone(), l_funcArguments.clone())?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
    out_a_functionPrototypes = Tpl::writeTok(out_a_functionPrototypes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("/*\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_equationInfos.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("*/\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void ")).clone() }))?;
    out_txt = CodegenUtil::symbolName(out_txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (Tpl::textString(l_funcName.clone())?).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_ix.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_funcArguments.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("){\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_dataCast.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    ret_9 = stringEq((Tpl::textString(l_varDecls.clone())?).clone(), (literal!("")).clone());
    out_txt = fun_56(out_txt.clone(), ret_9.clone(), l_varDecls.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_auxFunction.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_equationCode.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    Ok((out_txt, out_a_functionPrototypes))
}

fn fun_58(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_preExp: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_preExp: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls, out_a_preExp) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_auxFunction.clone(), in_a_varDecls.clone(), in_a_preExp.clone(), in_a_context.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { exp: i_exp, cref: i_cref, .. }, a_auxFunction, a_varDecls, a_preExp, a_context) => {
            let mut l_expPart: Tpl::Text;
            let mut l_crefStr: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            l_crefStr = CodegenCFunctions::crefOMSI(Tpl::emptyTxt.clone(), i_cref.clone(), a_context.clone())?;
            (l_expPart, a_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_crefStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_exp, .. }, a_auxFunction, a_varDecls, a_preExp, a_context) => {
            let mut l_expPart: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_preExp = (*a_preExp).clone();
            (l_expPart, a_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_exp.clone(), a_context.clone(), a_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::writeText(txt.clone(), a_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*res = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expPart.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, i_eq @ Deref @ SimCode::SimEqSystem::SES_WHEN { index: _, .. }, a_auxFunction, a_varDecls, a_preExp, a_context) => {
            let mut l_whenEq: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (l_whenEq, a_varDecls, a_auxFunction) = equationWhen(Tpl::emptyTxt.clone(), i_eq.clone(), a_context.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::writeText(txt.clone(), l_whenEq.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        (txt, _, a_auxFunction, a_varDecls, a_preExp, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT IMPLEMENTED YET Error in function equationCStr in template CodegenOMSIC_Equations")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_preExp.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls, out_a_preExp))
}

pub fn equationCStr(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_varDecls: Tpl::Text, mut a_auxFunction: Tpl::Text, mut a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    l_preExp = Tpl::emptyTxt.clone();
    (out_txt, out_a_auxFunction, out_a_varDecls, l_preExp) = fun_58(txt.clone(), a_eq.clone(), a_auxFunction.clone(), a_varDecls.clone(), l_preExp.clone(), a_context.clone())?;
    Ok((out_txt, out_a_varDecls, out_a_auxFunction))
}

fn fun_60(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_modelNamePrefixStr: ArcStr, mut in_a_input: ArcStr, mut in_a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_modelNamePrefixStr.clone(), in_a_input.clone(), in_a_omsiName.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i_index, .. }, a_modelNamePrefixStr, a_input, _) => {
            let mut l_i: Tpl::Text;
            let mut txt = (*txt).clone();
            l_i = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_index.clone())).clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("eqFunction")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { index: i_index, .. }, a_modelNamePrefixStr, a_input, _) => {
            let mut l_i: Tpl::Text;
            let mut txt = (*txt).clone();
            l_i = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(i_index.clone())).clone())?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("eqFunction")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: i_index, .. }, a_modelNamePrefixStr, a_input, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (literal!("resFunction")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { algSysIndex: i_algSysIndex, .. }, a_modelNamePrefixStr, a_input, a_omsiName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("new_status = ")).clone() }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelNamePrefixStr.clone()).clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_algSystFunction_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_algSysIndex.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_input.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("status = (new_status==omsi_ok && status==omsi_ok) ? omsi_ok:new_status;")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* equationCall not implemented yet */")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationCall(mut txt: Tpl::Text, mut a_eq: Arc<SimCode::SimEqSystem>, mut a_modelNamePrefixStr: ArcStr, mut a_modelFunctionnamePrefixStr: ArcStr, mut a_input: ArcStr, mut a_omsiName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_60(txt.clone(), a_eq.clone(), (a_modelNamePrefixStr.clone()).clone(), (a_input.clone()).clone(), (a_omsiName.clone()).clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_62(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::OMSIFunction>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_col, tail: rest }) => {
            let mut l_columnsString: Tpl::Text;
            let mut txt = (*txt).clone();
            l_columnsString = generateMatrixColumnInitialization(Tpl::emptyTxt.clone(), i_col.clone())?;
            txt = lm_62(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_63(mut in_txt: Tpl::Text, mut in_a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_columnsString: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matrix.clone(), in_a_columnsString.clone())) {
        (txt, Some(Deref @ SimCode::DerivativeMatrix { columns: i_m_columns, .. }), a_columnsString) => {
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            l_0__ = lm_62(Tpl::emptyTxt.clone(), i_m_columns.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_columnsString.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
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

pub fn generateMatrixInitialization(mut txt: Tpl::Text, mut a_matrix: Option<Arc<SimCode::DerivativeMatrix>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_columnsString: Tpl::Text;
    l_columnsString = Tpl::emptyTxt.clone();
    out_txt = fun_63(txt.clone(), a_matrix.clone(), l_columnsString.clone())?;
    Ok(out_txt)
}

fn lm_65(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_omsiFunction_context: SimCodeFunction::Context, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_body: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_body: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls, out_a_body) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_omsiFunction_context.clone(), in_a_auxFunction.clone(), in_a_varDecls.clone(), in_a_body.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_auxFunction, a_varDecls, a_body) => {
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_omsiFunction_context, a_auxFunction, a_varDecls, a_body) => {
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_body = (*a_body).clone();
            (a_body, a_varDecls, a_auxFunction) = equationCStr(a_body.clone(), i_eq.clone(), a_varDecls.clone(), a_auxFunction.clone(), a_omsiFunction_context.clone())?;
            (txt, a_auxFunction, a_varDecls, a_body) = lm_65(txt.clone(), rest.clone(), a_omsiFunction_context.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls, out_a_body))
}

fn fun_66(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_body: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_body: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls, out_a_body) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_column.clone(), in_a_auxFunction.clone(), in_a_varDecls.clone(), in_a_body.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { context: i_omsiFunction_context, equations: i_equations, .. }, a_auxFunction, a_varDecls, a_body) => {
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_body = (*a_body).clone();
            (l_0__, a_auxFunction, a_varDecls, a_body) = lm_65(Tpl::emptyTxt.clone(), i_equations.clone(), i_omsiFunction_context.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())?;
            txt = Tpl::writeText(txt.clone(), a_body.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())
        },
        (txt, _, a_auxFunction, a_varDecls, a_body) => {
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_body.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls, out_a_body))
}

pub fn generateMatrixColumnInitialization(mut txt: Tpl::Text, mut a_column: Arc<SimCode::OMSIFunction>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_body: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    l_body = Tpl::emptyTxt.clone();
    (out_txt, l_auxFunction, l_varDecls, l_body) = fun_66(txt.clone(), a_column.clone(), l_auxFunction.clone(), l_varDecls.clone(), l_body.clone())?;
    Ok(out_txt)
}

fn lm_68(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::OMSIFunction>>>, mut in_a_omsiName: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_index: ArcStr, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_omsiName.clone(), in_a_functionPrototypes.clone(), in_a_index.clone(), in_a_modelName.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_functionPrototypes, _, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_col, tail: rest }, a_omsiName, a_functionPrototypes, a_index, a_modelName) => {
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            (txt, a_functionPrototypes) = generateDereivativeMatrixColumnFunction(txt.clone(), i_col.clone(), (a_modelName.clone()).clone(), (a_index.clone()).clone(), a_functionPrototypes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt, a_functionPrototypes) = generateDereivativeMatrixColumnCall(txt.clone(), i_col.clone(), (a_modelName.clone()).clone(), (a_index.clone()).clone(), a_functionPrototypes.clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_functionPrototypes) = lm_68(txt.clone(), rest.clone(), (a_omsiName.clone()).clone(), a_functionPrototypes.clone(), (a_index.clone()).clone(), (a_modelName.clone()).clone())?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

fn fun_69(mut in_txt: Tpl::Text, mut in_a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_omsiName: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_index: ArcStr, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matrix.clone(), in_a_omsiName.clone(), in_a_functionPrototypes.clone(), in_a_index.clone(), in_a_modelName.clone())) {
        (txt, Some(Deref @ SimCode::DerivativeMatrix { columns: i_m_columns, .. }), a_omsiName, a_functionPrototypes, a_index, a_modelName) => {
            let mut l_columnsString: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            l_columnsString = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_columnsString, a_functionPrototypes) = lm_68(l_columnsString.clone(), i_m_columns.clone(), (a_omsiName.clone()).clone(), a_functionPrototypes.clone(), (a_index.clone()).clone(), (a_modelName.clone()).clone())?;
            l_columnsString = Tpl::popIter(l_columnsString.clone())?;
            txt = Tpl::writeText(txt.clone(), l_columnsString.clone())?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, _, _, a_functionPrototypes, _, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

pub fn generateDerivativeMatrix(mut txt: Tpl::Text, mut a_matrix: Option<Arc<SimCode::DerivativeMatrix>>, mut a_modelName: ArcStr, mut a_index: ArcStr, mut a_functionPrototypes: Tpl::Text, mut a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut l_columnsString: Tpl::Text;
    l_columnsString = Tpl::emptyTxt.clone();
    (out_txt, out_a_functionPrototypes) = fun_69(txt.clone(), a_matrix.clone(), (a_omsiName.clone()).clone(), a_functionPrototypes.clone(), (a_index.clone()).clone(), (a_modelName.clone()).clone())?;
    Ok((out_txt, out_a_functionPrototypes))
}

fn lm_71(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_functionPrototypes: Tpl::Text, mut in_a_omsiFunction_context: SimCodeFunction::Context, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_functionPrototypes.clone(), in_a_omsiFunction_context.clone(), in_a_modelName.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_functionPrototypes, _, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_functionPrototypes, a_omsiFunction_context, a_modelName) => {
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            (txt, a_functionPrototypes) = generateEquationFunction(txt.clone(), i_eq.clone(), (a_modelName.clone()).clone(), (literal!("")).clone(), a_omsiFunction_context.clone(), a_functionPrototypes.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_functionPrototypes) = lm_71(txt.clone(), rest.clone(), a_functionPrototypes.clone(), a_omsiFunction_context.clone(), (a_modelName.clone()).clone())?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_functionPrototypes: Tpl::Text, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_column.clone(), in_a_functionPrototypes.clone(), in_a_modelName.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { context: i_omsiFunction_context, equations: i_equations, .. }, a_functionPrototypes, a_modelName) => {
            let mut l_bodyBuffer: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            l_bodyBuffer = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_bodyBuffer, a_functionPrototypes) = lm_71(l_bodyBuffer.clone(), i_equations.clone(), a_functionPrototypes.clone(), i_omsiFunction_context.clone(), (a_modelName.clone()).clone())?;
            l_bodyBuffer = Tpl::popIter(l_bodyBuffer.clone())?;
            txt = Tpl::writeText(txt.clone(), l_bodyBuffer.clone())?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, _, a_functionPrototypes, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

pub fn generateDereivativeMatrixColumnFunction(mut txt: Tpl::Text, mut a_column: Arc<SimCode::OMSIFunction>, mut a_modelName: ArcStr, mut a_index: ArcStr, mut a_functionPrototypes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    let mut l_bodyBuffer: Tpl::Text;
    l_bodyBuffer = Tpl::emptyTxt.clone();
    l_preExp = Tpl::emptyTxt.clone();
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    (out_txt, out_a_functionPrototypes) = fun_72(txt.clone(), a_column.clone(), a_functionPrototypes.clone(), (a_modelName.clone()).clone())?;
    Ok((out_txt, out_a_functionPrototypes))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_74(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_omsiName: ArcStr, mut in_a_modelName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_omsiName.clone(), in_a_modelName.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_omsiName, a_modelName) => {
            let mut txt = (*txt).clone();
            txt = equationCall(txt.clone(), i_eq.clone(), (a_modelName.clone()).clone(), (literal!("")).clone(), (literal!("this_function, model_vars_and_params")).clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_74(txt.clone(), rest.clone(), (a_omsiName.clone()).clone(), (a_modelName.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_75(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_index: ArcStr, mut in_a_functionPrototypes: Tpl::Text, mut in_a_omsiName: ArcStr, mut in_a_modelName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    (out_txt, out_a_functionPrototypes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_column.clone(), in_a_index.clone(), in_a_functionPrototypes.clone(), in_a_omsiName.clone(), in_a_modelName.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { equations: i_equations, .. }, a_index, a_functionPrototypes, a_omsiName, a_modelName) => {
            let mut l_bodyBuffer: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_functionPrototypes = (*a_functionPrototypes).clone();
            l_bodyBuffer = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_bodyBuffer = lm_74(l_bodyBuffer.clone(), i_equations.clone(), (a_omsiName.clone()).clone(), (a_modelName.clone()).clone())?;
            l_bodyBuffer = Tpl::popIter(l_bodyBuffer.clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omsi_status ")).clone() }))?;
            a_functionPrototypes = CodegenUtil::symbolName(a_functionPrototypes.clone(), (a_modelName.clone()).clone(), (a_omsiName.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derivativeMatFunc_")).clone() }))?;
            a_functionPrototypes = Tpl::writeStr(a_functionPrototypes.clone(), (a_index.clone()).clone())?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data);")).clone() }))?;
            a_functionPrototypes = Tpl::writeTok(a_functionPrototypes.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/*\n")).clone(), (literal!("Description something\n")).clone(), (literal!("*/\n")).clone(), (literal!("omsi_status ")).clone()], lastHasNewLine: false }))?;
            txt = CodegenUtil::symbolName(txt.clone(), (a_modelName.clone()).clone(), (a_omsiName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_derivativeMatFunc_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_index.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(struct omsi_function_t* this_function, const omsi_values* model_vars_and_params, void* data){\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_bodyBuffer.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("return omsi_ok;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_functionPrototypes.clone())
        },
        (txt, _, _, a_functionPrototypes, _, _) => {
            (txt.clone(), a_functionPrototypes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_functionPrototypes))
}

pub fn generateDereivativeMatrixColumnCall(mut txt: Tpl::Text, mut a_column: Arc<SimCode::OMSIFunction>, mut a_modelName: ArcStr, mut a_index: ArcStr, mut a_functionPrototypes: Tpl::Text, mut a_omsiName: ArcStr) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_functionPrototypes: Tpl::Text;
    let mut l_auxFunction: Tpl::Text;
    let mut l_varDecls: Tpl::Text;
    let mut l_preExp: Tpl::Text;
    let mut l_bodyBuffer: Tpl::Text;
    l_bodyBuffer = Tpl::emptyTxt.clone();
    l_preExp = Tpl::emptyTxt.clone();
    l_varDecls = Tpl::emptyTxt.clone();
    l_auxFunction = Tpl::emptyTxt.clone();
    (out_txt, out_a_functionPrototypes) = fun_75(txt.clone(), a_column.clone(), (a_index.clone()).clone(), a_functionPrototypes.clone(), (a_omsiName.clone()).clone(), (a_modelName.clone()).clone())?;
    Ok((out_txt, out_a_functionPrototypes))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_77(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut in_a_context: SimCodeFunction::Context) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cr, tail: rest }, a_context) => {
            let mut ret_0: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = CodegenCFunctions::crefOMSI(txt.clone(), i_cr.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" && !")).clone() }))?;
            ret_0 = ComponentReference::crefPrefixPre(i_cr.clone());
            txt = CodegenCFunctions::crefOMSI(txt.clone(), ret_0.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" /* edge */)")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_77(txt.clone(), rest.clone(), a_context.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_context: SimCodeFunction::Context, mut in_a_conditions: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_context.clone(), in_a_conditions.clone())) {
        (txt, false, a_context, a_conditions) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" || ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_77(txt.clone(), a_conditions.clone(), a_context.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_mArg: Arc<DAE::Type>, mut in_a_val: Tpl::Text, mut in_a_context: SimCodeFunction::Context, mut in_a_stateVar: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_val.clone(), in_a_context.clone(), in_a_stateVar.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: Implement for arrays!")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_val, a_context, a_stateVar) => {
            let mut txt = (*txt).clone();
            txt = CodegenCFunctions::crefOMSI(txt.clone(), a_stateVar.clone(), a_context.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_val.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_stmt: BackendDAE::WhenOperator, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone(), in_a_auxFunction.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, BackendDAE::WhenOperator::ASSIGN { right: i_right, left: Deref @ DAE::Exp::CREF { componentRef: i_left, .. }, .. }, a_auxFunction, a_varDecls, a_context) => {
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            l_lhs = CodegenCFunctions::crefOMSI(Tpl::emptyTxt.clone(), i_left.clone(), a_context.clone())?;
            (l_rhs, l_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_right.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        (txt, BackendDAE::WhenOperator::REINIT { source: _, value: i_value, stateVar: i_stateVar }, a_auxFunction, a_varDecls, a_context) => {
            let mut ret_4: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            let mut l_val: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_preExp: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            l_preExp = Tpl::emptyTxt.clone();
            (l_val, l_preExp, a_varDecls, a_auxFunction) = CodegenCFunctions::daeExp(Tpl::emptyTxt.clone(), i_value.clone(), a_context.clone(), l_preExp.clone(), a_varDecls.clone(), a_auxFunction.clone())?;
            ret_4 = ComponentReference::crefTypeConsiderSubs(i_stateVar.clone())?;
            l_lhs = fun_79(Tpl::emptyTxt.clone(), ret_4.clone(), l_val.clone(), a_context.clone(), i_stateVar.clone())?;
            txt = Tpl::writeText(txt.clone(), l_preExp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ToDo: Add some info that variable was reinitialized */")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        (txt, _, a_auxFunction, a_varDecls, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: when expression not supported yet")).clone() }))?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls))
}

fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<BackendDAE::WhenOperator>>, mut in_a_auxFunction: Tpl::Text, mut in_a_varDecls: Tpl::Text, mut in_a_context: SimCodeFunction::Context) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    (out_txt, out_a_auxFunction, out_a_varDecls) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_auxFunction.clone(), in_a_varDecls.clone(), in_a_context.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_auxFunction, a_varDecls, _) => {
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_auxFunction, a_varDecls, a_context) => {
            let mut txt = (*txt).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            (txt, a_auxFunction, a_varDecls) = fun_80(txt.clone(), i_stmt.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_context.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_auxFunction, a_varDecls) = lm_81(txt.clone(), rest.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_context.clone())?;
            (txt.clone(), a_auxFunction.clone(), a_varDecls.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_auxFunction, out_a_varDecls))
}

pub fn equationWhen(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>, mut in_a_context: SimCodeFunction::Context, mut in_a_varDecls: Tpl::Text, mut in_a_auxFunction: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecls: Tpl::Text;
    let mut out_a_auxFunction: Tpl::Text;
    (out_txt, out_a_varDecls, out_a_auxFunction) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_context.clone(), in_a_varDecls.clone(), in_a_auxFunction.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { elseWhen: None, conditions: i_conditions, whenStmtLst: i_whenStmtLst, .. }, a_context, a_varDecls, a_auxFunction) => {
            let mut l_assign: Tpl::Text;
            let mut ret_1: bool = false;
            let mut l_helpIf: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecls = (*a_varDecls).clone();
            let mut a_auxFunction = (*a_auxFunction).clone();
            ret_1 = i_conditions.clone().is_empty();
            l_helpIf = fun_78(Tpl::emptyTxt.clone(), ret_1.clone(), a_context.clone(), i_conditions.clone())?;
            l_assign = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_assign, a_auxFunction, a_varDecls) = lm_81(l_assign.clone(), i_whenStmtLst.clone(), a_auxFunction.clone(), a_varDecls.clone(), a_context.clone())?;
            l_assign = Tpl::popIter(l_assign.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_helpIf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_assign.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_auxFunction.clone())
        },
        (txt, _, _, a_varDecls, a_auxFunction) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TODO: Implement elsewhen")).clone() }))?;
            (txt.clone(), a_varDecls.clone(), a_auxFunction.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecls, out_a_auxFunction))
}

