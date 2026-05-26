// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::AbsynDumpTpl;
use crate::ClassInfUtil;
use crate::DAEDumpTypes;
use crate::ExpressionDumpTpl;
use crate::SCodeDump;
use crate::SCodeDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_14(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dae, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpComp(txt.clone(), i_dae.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_14(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_15(mut in_txt: Tpl::Text, mut in_a_funLists: DAEDumpTypes::functionList) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_funLists.clone()) {
        (mut txt, DAEDumpTypes::functionList { funcs: ref i_funcs }) => {
            txt = dumpFunctions(txt.clone(), i_funcs.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_16(mut in_txt: Tpl::Text, mut in_a_fun__str: Tpl::Text, mut in_a_comp__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fun__str.clone(), in_a_comp__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_comp__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_comp__str.clone())?;
            txt.clone()
        },
        (txt, i_fun__str, a_comp__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_fun__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_comp__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpDAE(mut txt: Tpl::Text, mut a_fixedDaeList: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>>, mut a_funLists: DAEDumpTypes::functionList) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_fun__str: Tpl::Text;
    let mut l_comp__str: Tpl::Text;
    l_comp__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_comp__str = lm_14(l_comp__str.clone(), a_fixedDaeList.clone())?;
    l_comp__str = Tpl::popIter(l_comp__str.clone())?;
    l_fun__str = fun_15(Tpl::emptyTxt.clone(), a_funLists.clone())?;
    out_txt = fun_16(txt.clone(), l_fun__str.clone(), l_comp__str.clone())?;
    Ok(out_txt)
}

fn fun_18(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, mut a_name) => {
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            ret_0 = (System::stringReplace((a_name.clone()).clone(), (literal!(".")).clone(), (literal!("__")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_19(mut in_txt: Tpl::Text, mut in_a_ann__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpComp(mut in_txt: Tpl::Text, mut in_a_fixedDae: Arc<DAEDumpTypes::compWithSplitElements>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fixedDae.clone())) {
        (txt, Deref @ DAEDumpTypes::compWithSplitElements { spltElems: i_spltElems, name: i_name, comment: i_comment }) => {
            let mut ret_3: bool = false;
            let mut l_name__rep: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_ann__str = dumpClassAnnotation(Tpl::emptyTxt.clone(), i_comment.clone())?;
            ret_3 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            l_name__rep = fun_18(Tpl::emptyTxt.clone(), ret_3.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("class ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__rep.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = dumpCompStream(txt.clone(), i_spltElems.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = fun_19(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__rep.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_21(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAEDumpTypes::compWithSplitElements>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_flatSM, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStateMachineSection(txt.clone(), i_flatSM.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_21(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpCompStream(mut in_txt: Tpl::Text, mut in_a_elems: Arc<DAEDumpTypes::splitElements>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elems.clone())) {
        (txt, Deref @ DAEDumpTypes::splitElements { sm: i_sm, a: i_a, e: i_e, ia: i_ia, ie: i_ie, v: i_v, .. }) => {
            let mut l_sm__str: Tpl::Text;
            let mut l_al__str: Tpl::Text;
            let mut l_eq__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_var__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_var__str = dumpVars(Tpl::emptyTxt.clone(), i_v.clone())?;
            l_ieq__str = dumpInitialEquationSection(Tpl::emptyTxt.clone(), i_ie.clone())?;
            l_ial__str = dumpInitialAlgorithmSection(Tpl::emptyTxt.clone(), i_ia.clone())?;
            l_eq__str = dumpEquationSection(Tpl::emptyTxt.clone(), i_e.clone())?;
            l_al__str = dumpAlgorithmSection(Tpl::emptyTxt.clone(), i_a.clone())?;
            l_sm__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_sm__str = lm_21(l_sm__str.clone(), i_sm.clone())?;
            l_sm__str = Tpl::popIter(l_sm__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_sm__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ieq__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ial__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_al__str.clone())?;
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
fn lm_23(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<DAE::Function>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_func, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFunction(txt.clone(), i_func.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_23(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctions(mut txt: Tpl::Text, mut a_funcs: Arc<metamodelica::List<DAE::Function>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_23(out_txt.clone(), a_funcs.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_25(mut in_txt: Tpl::Text, mut in_a_isImpure: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isImpure.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("impure ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_26(mut in_txt: Tpl::Text, mut in_a_ann__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_27(mut in_txt: Tpl::Text, mut in_a_isImpure: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isImpure.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("impure ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_28(mut in_txt: Tpl::Text, mut in_a_ann__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_29(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_type__: Arc<DAE::Type>, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_type__.clone(), in_a_path.clone())) {
        (txt, false, a_type__, a_path) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), a_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" \"Automatically generated record constructor for ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), a_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpRecordInputVarStr(txt.clone(), a_type__.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output ")).clone() }))?;
            txt = dumpPathLastIndent(txt.clone(), a_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" res;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), a_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_type__, _) => {
            let mut txt = (*txt).clone();
            txt = dumpRecordType(txt.clone(), a_type__.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFunction(mut in_txt: Tpl::Text, mut in_a_function: DAE::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_function.clone())) {
        (txt, DAE::Function::FUNCTION { isImpure: i_isImpure, comment: i_comment, path: i_path, functions: i_functions @ Deref @ metamodelica::List::Cons { head: DAE::FunctionDefinition::FUNCTION_PARTIAL_DERIVATIVE { derivedFunction: _, .. }, tail: Deref @ metamodelica::List::Nil }, .. }) => {
            let mut l_impure__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_fn__name: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fn__name = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_ann__str = dumpClassAnnotation(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_impure__str = fun_25(Tpl::emptyTxt.clone(), i_isImpure.clone())?;
            txt = Tpl::writeText(txt.clone(), l_impure__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fn__name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpFunctionDefinitions(txt.clone(), i_functions.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = fun_26(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Function::FUNCTION { functions: i_functions, path: i_path, isImpure: i_isImpure, comment: i_comment, .. }) => {
            let mut l_impure__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_ann__str = dumpClassAnnotation(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_impure__str = fun_27(Tpl::emptyTxt.clone(), i_isImpure.clone())?;
            txt = Tpl::writeText(txt.clone(), l_impure__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), i_path.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = dumpFunctionDefinitions(txt.clone(), i_functions.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = fun_28(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::Function::RECORD_CONSTRUCTOR { type_: i_type__, path: i_path, .. }) => {
            let mut ret_4: bool = false;
            let mut txt = (*txt).clone();
            ret_4 = Flags::isSet(Flags::PRINT_RECORD_TYPES.clone())?;
            txt = fun_29(txt.clone(), ret_4.clone(), i_type__.clone(), i_path.clone())?;
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
fn lm_31(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<DAE::FunctionDefinition>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_func, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFunctionDefinition(txt.clone(), i_func.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_31(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionDefinitions(mut txt: Tpl::Text, mut a_functions: Arc<metamodelica::List<DAE::FunctionDefinition>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_31(out_txt.clone(), a_functions.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_33(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_var.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_33(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionDefinition(mut in_txt: Tpl::Text, mut in_a_functions: DAE::FunctionDefinition) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_functions.clone()) {
        (mut txt, DAE::FunctionDefinition::FUNCTION_DEF { body: ref i_body }) => {
            txt = dumpFunctionBody(txt.clone(), i_body.clone())?;
            txt.clone()
        },
        (mut txt, DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: mut i_externalDecl, body: ref i_body }) => {
            txt = dumpFunctionBody(txt.clone(), i_body.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = dumpExternalDecl(txt.clone(), i_externalDecl.clone())?;
            txt.clone()
        },
        (mut txt, DAE::FunctionDefinition::FUNCTION_DER_MAPPER { derivedFunction: _, .. }) => {
            txt.clone()
        },
        (mut txt, DAE::FunctionDefinition::FUNCTION_INVERSE { inputParam: _, .. }) => {
            txt.clone()
        },
        (mut txt, DAE::FunctionDefinition::FUNCTION_PARTIAL_DERIVATIVE { derivedFunction: ref i_derivedFunction, derivedVars: ref i_derivedVars }) => {
            let mut l_vars: Tpl::Text;
            l_vars = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_vars = lm_33(l_vars.clone(), i_derivedVars.clone())?;
            l_vars = Tpl::popIter(l_vars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der(")).clone() }))?;
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), i_derivedFunction.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_vars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_35(mut in_txt: Tpl::Text, mut in_a_func__name__str: Tpl::Text, mut in_a_func__args__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_func__name__str.clone(), in_a_func__args__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, i_func__name__str, a_func__args__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), i_func__name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_func__args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_36(mut in_txt: Tpl::Text, mut in_a_ext__output__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ext__output__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_ext__output__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), i_ext__output__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" =")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_37(mut in_txt: Tpl::Text, mut in_a_ann: Option<Arc<SCode::Annotation>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann.clone())) {
        (txt, Some(i_annotation)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = dumpAnnotation(txt.clone(), i_annotation.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpExternalDecl(mut in_txt: Tpl::Text, mut in_a_externalDecl: DAE::ExternalDecl) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_externalDecl.clone()) {
        (mut txt, DAE::ExternalDecl { ann: mut i_ann, language: mut i_language, returnArg: mut i_returnArg, args: ref i_args, name: mut i_name }) => {
            let mut l_ann__str: Tpl::Text;
            let mut l_lang__str: Tpl::Text;
            let mut l_output__str: Tpl::Text;
            let mut l_ext__output__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_func__args__str: Tpl::Text;
            let mut l_func__name__str: Tpl::Text;
            l_func__name__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_name.clone()).clone())?;
            l_func__args__str = dumpExtArgs(Tpl::emptyTxt.clone(), i_args.clone())?;
            l_func__str = fun_35(Tpl::emptyTxt.clone(), l_func__name__str.clone(), l_func__args__str.clone())?;
            l_ext__output__str = dumpExtArg(Tpl::emptyTxt.clone(), i_returnArg.clone())?;
            l_output__str = fun_36(Tpl::emptyTxt.clone(), l_ext__output__str.clone())?;
            l_lang__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_language.clone()).clone())?;
            l_ann__str = fun_37(Tpl::emptyTxt.clone(), i_ann.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("external \"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lang__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_output__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_39(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<DAE::ExtArg>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExtArg(txt.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_39(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpExtArgs(mut txt: Tpl::Text, mut a_args: Arc<metamodelica::List<DAE::ExtArg>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_39(out_txt.clone(), a_args.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpExtArg(mut in_txt: Tpl::Text, mut in_a_arg: DAE::ExtArg) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_arg.clone()) {
        (mut txt, DAE::ExtArg::EXTARG { componentRef: ref i_componentRef, .. }) => {
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (mut txt, DAE::ExtArg::EXTARGEXP { exp: ref i_exp, .. }) => {
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (mut txt, DAE::ExtArg::EXTARGSIZE { exp: ref i_exp, componentRef: ref i_componentRef, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("size(")).clone() }))?;
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
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
pub fn dumpRecordInputVarStr(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { varLst: i_varLst, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpRecordVars(txt.clone(), i_varLst.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_FUNCTION { funcResultType: i_funcResultType, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpRecordInputVarStr(txt.clone(), i_funcResultType.clone())?;
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
fn lm_43(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpRecordVar(txt.clone(), i_v.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_43(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpRecordVars(mut txt: Tpl::Text, mut a_varLst: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_43(out_txt.clone(), a_varLst.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpRecordVar(mut in_txt: Tpl::Text, mut in_a_v: Arc<DAE::Var>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_v.clone())) {
        (txt, Deref @ DAE::Var { name: i_name, ty: i_ty, binding: i_binding, attributes: i_attributes, .. }) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_attr: Tpl::Text;
            let mut l_binding__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_attr__str = dumpRecordConstructorInputAttr(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            l_binding__str = dumpRecordConstructorBinding(Tpl::emptyTxt.clone(), i_binding.clone())?;
            l_attr = Tpl::emptyTxt.clone();
            (l_ty__str, l_attr) = dumpType(Tpl::emptyTxt.clone(), i_ty.clone(), l_attr.clone())?;
            txt = Tpl::writeText(txt.clone(), l_attr__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_attr.clone())?;
            txt = Tpl::writeText(txt.clone(), l_binding__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpRecordConstructorInputAttr(mut in_txt: Tpl::Text, mut in_a_attr: Arc<DAE::Attributes>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attr.clone())) {
        (txt, Deref @ DAE::Attributes { direction: Absyn::Direction::INPUT, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input ")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Attributes { visibility: SCode::Visibility::PROTECTED, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("protected ")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Attributes { variability: SCode::Variability::CONST, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpRecordConstructorBinding(mut in_txt: Tpl::Text, mut in_a_binding: Arc<DAE::Binding>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_binding.clone())) {
        (txt, Deref @ DAE::Binding::UNBOUND) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Binding::EQBOUND { exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpRecordVarBinding(mut in_txt: Tpl::Text, mut in_a_binding: Arc<DAE::Binding>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_binding.clone())) {
        (txt, Deref @ DAE::Binding::UNBOUND) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Binding::EQBOUND { exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Binding::VALBOUND { valBound: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("value bound***** check what to display")).clone() }))?;
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
fn lm_49(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lst, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFunctionElement(txt.clone(), i_lst.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_49(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_50(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lst, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFunctionAnnotation(txt.clone(), i_lst.clone())?;
            txt = lm_50(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionBody(mut txt: Tpl::Text, mut a_dAElist: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_49(out_txt.clone(), a_dAElist.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = lm_50(out_txt.clone(), a_dAElist.clone())?;
    Ok(out_txt)
}

pub fn dumpFunctionElement(mut in_txt: Tpl::Text, mut in_a_lst: Arc<DAE::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lst.clone())) {
        (txt, i_lst @ Deref @ DAE::Element::VAR { componentRef: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpVar(txt.clone(), i_lst.clone(), true)?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIALALGORITHM { algorithm_: i_algorithm__, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpFunctionAlgorithm(txt.clone(), i_algorithm__.clone(), (literal!("initial algorithm")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::ALGORITHM { algorithm_: i_algorithm__, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpFunctionAlgorithm(txt.clone(), i_algorithm__.clone(), (literal!("algorithm")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::COMMENT { cmt: _ }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Element not found")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_x: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_x.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_x) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), i_x.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionAnnotation(mut in_txt: Tpl::Text, mut in_a_lst: Arc<DAE::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lst.clone())) {
        (txt, Deref @ DAE::Element::COMMENT { cmt: i_cmt }) => {
            let mut l_x: Tpl::Text;
            let mut txt = (*txt).clone();
            l_x = dumpCommentAnnotationNoOpt(Tpl::emptyTxt.clone(), i_cmt.clone())?;
            txt = fun_53(txt.clone(), l_x.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionAlgorithm(mut in_txt: Tpl::Text, mut in_a_algorithm__: Arc<DAE::Algorithm>, mut in_a_label: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_algorithm__.clone(), in_a_label.clone())) {
        (txt, Deref @ DAE::Algorithm { statementLst: i_statementLst }, a_label) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_label.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpStatements(txt.clone(), i_statementLst.clone())?;
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

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpVar(txt.clone(), i_var.clone(), false)?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpVars(mut txt: Tpl::Text, mut a_v: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_56(out_txt.clone(), a_v.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_58(mut in_txt: Tpl::Text, mut in_a_variableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variableAttributesOption.clone())) {
        (txt, Some(i_VariableAttributes)) => {
            let mut txt = (*txt).clone();
            txt = dumpFinalPrefix(txt.clone(), i_VariableAttributes.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_59(mut in_txt: Tpl::Text, mut in_a_printTypeDimension: bool, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_printTypeDimension.clone(), in_a_dims.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_dims) => {
            let mut txt = (*txt).clone();
            txt = dumpTypeDimensions(txt.clone(), a_dims.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_60(mut in_txt: Tpl::Text, mut in_a_binding: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_binding.clone())) {
        (txt, Some(i_exp)) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_61(mut in_txt: Tpl::Text, mut in_a_variableAttributesOption: Option<Arc<DAE::VariableAttributes>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variableAttributesOption.clone())) {
        (txt, Some(i_VariableAttributes)) => {
            let mut txt = (*txt).clone();
            txt = dumpVariableAttributes(txt.clone(), i_VariableAttributes.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_62(mut in_txt: Tpl::Text, mut in_a_bindingExp: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_bindingExp.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_bindingExp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_bindingExp.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpVar(mut in_txt: Tpl::Text, mut in_a_lst: Arc<DAE::Element>, mut in_a_printTypeDimension: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lst.clone(), in_a_printTypeDimension.clone())) {
        (txt, Deref @ DAE::Element::VAR { comment: i_comment, binding: i_binding, componentRef: i_componentRef, dims: i_dims, ty: i_ty, direction: i_direction, kind: i_kind, parallelism: i_parallelism, protection: i_protection, variableAttributesOption: i_variableAttributesOption, .. }, a_printTypeDimension) => {
            let mut l_binding__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_varAttr: Tpl::Text;
            let mut l_bindingExp: Tpl::Text;
            let mut l_varName: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            let mut l_varType: Tpl::Text;
            let mut l_attr: Tpl::Text;
            let mut l_varDirection: Tpl::Text;
            let mut l_varKind: Tpl::Text;
            let mut l_varParallelism: Tpl::Text;
            let mut l_varVisibility: Tpl::Text;
            let mut l_final: Tpl::Text;
            let mut txt = (*txt).clone();
            l_final = fun_58(Tpl::emptyTxt.clone(), i_variableAttributesOption.clone())?;
            l_varVisibility = dumpVarVisibility(Tpl::emptyTxt.clone(), i_protection.clone())?;
            l_varParallelism = dumpVarParallelism(Tpl::emptyTxt.clone(), i_parallelism.clone())?;
            l_varKind = dumpVarKind(Tpl::emptyTxt.clone(), i_kind.clone())?;
            l_varDirection = dumpVarDirection(Tpl::emptyTxt.clone(), i_direction.clone())?;
            l_attr = Tpl::emptyTxt.clone();
            (l_varType, l_attr) = dumpType(Tpl::emptyTxt.clone(), i_ty.clone(), l_attr.clone())?;
            l_dim__str = fun_59(Tpl::emptyTxt.clone(), a_printTypeDimension.clone(), i_dims.clone())?;
            l_varName = dumpCref(Tpl::emptyTxt.clone(), i_componentRef.clone())?;
            l_bindingExp = fun_60(Tpl::emptyTxt.clone(), i_binding.clone())?;
            l_varAttr = fun_61(Tpl::emptyTxt.clone(), i_variableAttributesOption.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_ann__str = dumpCompAnnotation(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_binding__str = fun_62(Tpl::emptyTxt.clone(), l_bindingExp.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), l_varVisibility.clone())?;
            txt = Tpl::writeText(txt.clone(), l_final.clone())?;
            txt = Tpl::writeText(txt.clone(), l_varParallelism.clone())?;
            txt = Tpl::writeText(txt.clone(), l_varKind.clone())?;
            txt = Tpl::writeText(txt.clone(), l_varDirection.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_varType.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_varName.clone())?;
            txt = Tpl::writeText(txt.clone(), l_attr.clone())?;
            txt = Tpl::writeText(txt.clone(), l_varAttr.clone())?;
            txt = Tpl::writeText(txt.clone(), l_binding__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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

pub fn dumpFinalPrefix(mut in_txt: Tpl::Text, mut in_a_varAttr: Arc<DAE::VariableAttributes>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varAttr.clone())) {
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { finalPrefix: Some(true), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" final")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_INT { finalPrefix: Some(true), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" final")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { finalPrefix: Some(true), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" final")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { finalPrefix: Some(true), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" final")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { finalPrefix: Some(true), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" final")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpVarVisibility(mut in_txt: Tpl::Text, mut in_a_protection: DAE::VarVisibility) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_protection.clone()) {
        (mut txt, DAE::VarVisibility::PROTECTED) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" protected")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpVarParallelism(mut in_txt: Tpl::Text, mut in_a_parallelism: DAE::VarParallelism) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_parallelism.clone()) {
        (mut txt, DAE::VarParallelism::PARGLOBAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" parglobal")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::VarParallelism::PARLOCAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" parlocal")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpVarKind(mut in_txt: Tpl::Text, mut in_a_kind: DAE::VarKind) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_kind.clone()) {
        (mut txt, DAE::VarKind::CONST) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" constant")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::VarKind::PARAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::VarKind::DISCRETE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" discrete")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpVarDirection(mut in_txt: Tpl::Text, mut in_a_direction: DAE::VarDirection) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_direction.clone()) {
        (mut txt, DAE::VarDirection::INPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" input")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::VarDirection::OUTPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" output")).clone() }))?;
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
fn lm_69(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_69(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_attributes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_attributes: Tpl::Text;
    (out_txt, out_a_attributes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_attributes.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: i_varLst }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            a_attributes = dumpVarAttributes(a_attributes.clone(), i_varLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: i_varLst }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            a_attributes = dumpVarAttributes(a_attributes.clone(), i_varLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: i_varLst }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            a_attributes = dumpVarAttributes(a_attributes.clone(), i_varLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: i_varLst }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            a_attributes = dumpVarAttributes(a_attributes.clone(), i_varLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_CLOCK { varLst: i_varLst }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            a_attributes = dumpVarAttributes(a_attributes.clone(), i_varLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { names: i_names, .. }, a_attributes) => {
            let mut l_lit__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lit__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_lit__str = lm_69(l_lit__str.clone(), i_names.clone())?;
            l_lit__str = Tpl::popIter(l_lit__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("enumeration(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lit__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_ARRAY { dims: i_dims, ty: i_ty }, a_attributes) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            txt_1 = dumpDimensions(Tpl::emptyTxt.clone(), i_dims.clone())?;
            (txt, a_attributes) = dumpArrayType(txt.clone(), i_ty.clone(), (Tpl::textString(txt_1.clone())?).clone(), a_attributes.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: i_rname }, .. }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), i_rname.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_complexClassType, .. }, a_attributes) => {
            let mut ret_2: Arc<Absyn::Path>;
            let mut txt = (*txt).clone();
            ret_2 = ClassInfUtil::getStateName(i_complexClassType.clone());
            txt = AbsynDumpTpl::dumpPath(txt.clone(), ret_2.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: i_complexType, .. }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            (txt, a_attributes) = dumpType(txt.clone(), i_complexType.clone(), a_attributes.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, i_ty @ Deref @ DAE::Type::T_FUNCTION { funcArg: _, .. }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = dumpFunctionType(txt.clone(), i_ty.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_TUPLE { types: i_types, .. }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = dumpTupleType(txt.clone(), i_types.clone(), (literal!("(")).clone(), (literal!(")")).clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: i_types }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = dumpTupleType(txt.clone(), i_types.clone(), (literal!("tuple<")).clone(), (literal!(">")).clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: i_ty }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list<")).clone() }))?;
            (txt, a_attributes) = dumpType(txt.clone(), i_ty.clone(), a_attributes.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: i_ty }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("array<")).clone() }))?;
            (txt, a_attributes) = dumpType(txt.clone(), i_ty.clone(), a_attributes.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: i_name }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("polymorphic<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { path: i_path, .. }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), i_path.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METARECORD { path: i_path, .. }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPathNoQual(txt.clone(), i_path.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: i_ty }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#")).clone() }))?;
            (txt, a_attributes) = dumpType(txt.clone(), i_ty.clone(), a_attributes.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: Deref @ DAE::Type::T_UNKNOWN }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Option<Any>")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: i_ty }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Option<")).clone() }))?;
            (txt, a_attributes) = dumpType(txt.clone(), i_ty.clone(), a_attributes.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: i_ty }, a_attributes) => {
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            (txt, a_attributes) = dumpType(txt.clone(), i_ty.clone(), a_attributes.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_NORETCALL, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_NORETCALL#")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_UNKNOWN, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_UNKNOWN#")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, Deref @ DAE::Type::T_ANYTYPE { anyClassType: _ }, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_ANYTYPE#")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, _, a_attributes) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DAEDumpTpl.dumpType: Not yet implemented")).clone() }))?;
            (txt.clone(), a_attributes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_attributes))
}

fn fun_71(mut in_txt: Tpl::Text, mut in_a_dims__accum: ArcStr, mut in_a_dims__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dims__accum.clone(), in_a_dims__str.clone())) {
        (txt, Deref @ "", a_dims__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_dims__str.clone())?;
            txt.clone()
        },
        (txt, i_dims__accum, a_dims__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_dims__accum.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_dims__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_72(mut in_txt: Tpl::Text, mut in_a_dims__accum: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dims__accum.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_dims__accum) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_dims__accum.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpArrayType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_dims__accum: ArcStr, mut in_a_attributes: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_attributes: Tpl::Text;
    (out_txt, out_a_attributes) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_dims__accum.clone(), in_a_attributes.clone())) {
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, dims: i_dims }, a_dims__accum, a_attributes) => {
            let mut l_dims__accum__str: Tpl::Text;
            let mut l_dims__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            l_dims__str = dumpDimensions(Tpl::emptyTxt.clone(), i_dims.clone())?;
            l_dims__accum__str = fun_71(Tpl::emptyTxt.clone(), (a_dims__accum.clone()).clone(), l_dims__str.clone())?;
            (txt, a_attributes) = dumpArrayType(txt.clone(), i_ty.clone(), (Tpl::textString(l_dims__accum__str.clone())?).clone(), a_attributes.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        (txt, i_ty, a_dims__accum, a_attributes) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_dims__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_attributes = (*a_attributes).clone();
            (l_ty__str, a_attributes) = dumpType(Tpl::emptyTxt.clone(), i_ty.clone(), a_attributes.clone())?;
            l_dims__str = fun_72(Tpl::emptyTxt.clone(), (a_dims__accum.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dims__str.clone())?;
            (txt.clone(), a_attributes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_attributes))
}

fn lm_74(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_attr: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_attr: Tpl::Text;
    (out_txt, out_a_attr) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_attr.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_attr) => {
            (txt.clone(), a_attr.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }, a_attr) => {
            let mut txt = (*txt).clone();
            let mut a_attr = (*a_attr).clone();
            (txt, a_attr) = dumpType(txt.clone(), i_ty.clone(), a_attr.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_attr) = lm_74(txt.clone(), rest.clone(), a_attr.clone())?;
            (txt.clone(), a_attr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_attr))
}

pub fn dumpTupleType(mut txt: Tpl::Text, mut a_tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut a_ty__begin: ArcStr, mut a_ty__end: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_attr: Tpl::Text;
    l_attr = Tpl::emptyTxt.clone();
    out_txt = Tpl::writeStr(txt.clone(), (a_ty__begin.clone()).clone())?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (out_txt, l_attr) = lm_74(out_txt.clone(), a_tys.clone(), l_attr.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_ty__end.clone()).clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_76(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFuncArg(txt.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_76(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_FUNCTION { funcResultType: i_funcResultType, path: i_path, funcArg: i_funcArg, .. }) => {
            let mut l_res__str: Tpl::Text;
            let mut l_attr: Tpl::Text;
            let mut l_src__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_args__str = lm_76(l_args__str.clone(), i_funcArg.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            l_src__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_attr = Tpl::emptyTxt.clone();
            (l_res__str, l_attr) = dumpType(Tpl::emptyTxt.clone(), i_funcResultType.clone(), l_attr.clone())?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<function>(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") => ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_res__str.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_a_defaultBinding: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_defaultBinding.clone())) {
        (txt, Some(i_bexp)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":= ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_bexp.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFuncArg(mut in_txt: Tpl::Text, mut in_a_arg: Arc<DAE::FuncArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arg.clone())) {
        (txt, Deref @ DAE::FuncArg { name: i_name, defaultBinding: i_defaultBinding, par: i_par, r#const: i_const, ty: i_ty }) => {
            let mut l_binding__str: Tpl::Text;
            let mut l_p__str: Tpl::Text;
            let mut l_c__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr: Tpl::Text;
            let mut txt = (*txt).clone();
            l_attr = Tpl::emptyTxt.clone();
            (l_ty__str, l_attr) = dumpType(Tpl::emptyTxt.clone(), i_ty.clone(), l_attr.clone())?;
            l_c__str = dumpConst(Tpl::emptyTxt.clone(), i_const.clone())?;
            l_p__str = dumpParallelism(Tpl::emptyTxt.clone(), i_par.clone())?;
            l_binding__str = fun_78(Tpl::emptyTxt.clone(), i_defaultBinding.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_c__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_p__str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_binding__str.clone())?;
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
pub fn dumpRecordType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_COMPLEX { varLst: i_varLst, complexClassType: i_complexClassType, .. }) => {
            let mut l_vars: Tpl::Text;
            let mut ret_1: Arc<Absyn::Path>;
            let mut l_name: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = ClassInfUtil::getStateName(i_complexClassType.clone());
            l_name = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), ret_1.clone())?;
            l_vars = dumpRecordVars(Tpl::emptyTxt.clone(), i_varLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("record ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_vars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_FUNCTION { funcResultType: i_funcResultType, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpRecordType(txt.clone(), i_funcResultType.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpConst(mut in_txt: Tpl::Text, mut in_a_c: DAE::Const) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_c.clone()) {
        (mut txt, DAE::Const::C_PARAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter ")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Const::C_CONST) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpParallelism(mut in_txt: Tpl::Text, mut in_a_p: DAE::VarParallelism) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_p.clone()) {
        (mut txt, DAE::VarParallelism::PARGLOBAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parglobal ")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::VarParallelism::PARLOCAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parlocal ")).clone() }))?;
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
fn lm_83(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpVarAttribute(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_83(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpVarAttributes(mut in_txt: Tpl::Text, mut in_a_literalVarLst: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_literalVarLst.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_literalVarLst) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_83(txt.clone(), i_literalVarLst.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpVarAttribute(mut in_txt: Tpl::Text, mut in_a_var: Arc<DAE::Var>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_var.clone())) {
        (txt, Deref @ DAE::Var { name: i_name, binding: Deref @ DAE::Binding::EQBOUND { exp: i_e, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_e.clone())?;
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
fn lm_86(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dim, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpDimension(txt.clone(), i_dim.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_86(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpDimensions(mut in_txt: Tpl::Text, mut in_a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dims.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_dims) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_86(txt.clone(), i_dims.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpDimension(mut in_txt: Tpl::Text, mut in_a_dim: Arc<DAE::Dimension>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dim.clone())) {
        (txt, Deref @ DAE::Dimension::DIM_INTEGER { integer: i_integer }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_integer.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: i_enumTypeName, .. }) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_enumTypeName.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_EXP { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Dimension::DIM_UNKNOWN) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_89(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_90(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_91(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_92(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_93(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_94(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_95(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_96(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_97(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_98(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_99(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_100(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_101(mut in_txt: Tpl::Text, mut in_a_attrs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attrs__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_attrs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_attrs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_102(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_103(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_104(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_105(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_106(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_107(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_108(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_109(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_110(mut in_txt: Tpl::Text, mut in_a_attrs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attrs__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_attrs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_attrs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_111(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_112(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_113(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_114(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_115(mut in_txt: Tpl::Text, mut in_a_attrs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attrs__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_attrs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_attrs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_116(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_117(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_118(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_119(mut in_txt: Tpl::Text, mut in_a_attrs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attrs__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_attrs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_attrs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_120(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_121(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_122(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_123(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_124(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn smf_125(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_it) => {
            txt = Tpl::writeText(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_126(mut in_txt: Tpl::Text, mut in_a_attrs__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_attrs__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_attrs__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_attrs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpVariableAttributes(mut in_txt: Tpl::Text, mut in_a_variableAttributesOption: Arc<DAE::VariableAttributes>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_variableAttributesOption.clone())) {
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { startOrigin: i_startOrigin, distributionOption: i_distributionOption, uncertainOption: i_uncertainOption, stateSelectOption: i_stateSelectOption, nominal: i_nominal, fixed: i_fixed, start: i_start, max: i_max, min: i_min, displayUnit: i_displayUnit, unit: i_unit, quantity: i_quantity, .. }) => {
            let mut l_attrs__str: Tpl::Text;
            let mut l_so__str: Tpl::Text;
            let mut l_dist__str: Tpl::Text;
            let mut l_uncert__str: Tpl::Text;
            let mut l_statesel__str: Tpl::Text;
            let mut l_nominal__str: Tpl::Text;
            let mut l_fixed__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_max__str: Tpl::Text;
            let mut l_min__str: Tpl::Text;
            let mut l_displayunit__str: Tpl::Text;
            let mut l_unit__str: Tpl::Text;
            let mut l_quantity__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_quantity__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_quantity.clone(), (literal!("quantity")).clone())?;
            l_unit__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_unit.clone(), (literal!("unit")).clone())?;
            l_displayunit__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_displayUnit.clone(), (literal!("displayUnit")).clone())?;
            l_min__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_min.clone(), (literal!("min")).clone())?;
            l_max__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_max.clone(), (literal!("max")).clone())?;
            l_start__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_start.clone(), (literal!("start")).clone())?;
            l_fixed__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_fixed.clone(), (literal!("fixed")).clone())?;
            l_nominal__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_nominal.clone(), (literal!("nominal")).clone())?;
            l_statesel__str = dumpStateSelectAttrOpt(Tpl::emptyTxt.clone(), i_stateSelectOption.clone())?;
            l_uncert__str = dumpUncertaintyAttrOpt(Tpl::emptyTxt.clone(), i_uncertainOption.clone())?;
            l_dist__str = dumpDistributionAttrOpt(Tpl::emptyTxt.clone(), i_distributionOption.clone())?;
            l_so__str = dumpStartOriginAttrOpt(Tpl::emptyTxt.clone(), i_startOrigin.clone())?;
            l_attrs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attrs__str = smf_89(l_attrs__str.clone(), l_quantity__str.clone())?;
            l_attrs__str = smf_90(l_attrs__str.clone(), l_unit__str.clone())?;
            l_attrs__str = smf_91(l_attrs__str.clone(), l_displayunit__str.clone())?;
            l_attrs__str = smf_92(l_attrs__str.clone(), l_min__str.clone())?;
            l_attrs__str = smf_93(l_attrs__str.clone(), l_max__str.clone())?;
            l_attrs__str = smf_94(l_attrs__str.clone(), l_start__str.clone())?;
            l_attrs__str = smf_95(l_attrs__str.clone(), l_fixed__str.clone())?;
            l_attrs__str = smf_96(l_attrs__str.clone(), l_nominal__str.clone())?;
            l_attrs__str = smf_97(l_attrs__str.clone(), l_statesel__str.clone())?;
            l_attrs__str = smf_98(l_attrs__str.clone(), l_uncert__str.clone())?;
            l_attrs__str = smf_99(l_attrs__str.clone(), l_dist__str.clone())?;
            l_attrs__str = smf_100(l_attrs__str.clone(), l_so__str.clone())?;
            l_attrs__str = Tpl::popIter(l_attrs__str.clone())?;
            txt = fun_101(txt.clone(), l_attrs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_INT { startOrigin: i_startOrigin, distributionOption: i_distributionOption, uncertainOption: i_uncertainOption, fixed: i_fixed, start: i_start, max: i_max, min: i_min, quantity: i_quantity, .. }) => {
            let mut l_attrs__str: Tpl::Text;
            let mut l_so__str: Tpl::Text;
            let mut l_dist__str: Tpl::Text;
            let mut l_uncert__str: Tpl::Text;
            let mut l_fixed__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_max__str: Tpl::Text;
            let mut l_min__str: Tpl::Text;
            let mut l_quantity__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_quantity__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_quantity.clone(), (literal!("quantity")).clone())?;
            l_min__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_min.clone(), (literal!("min")).clone())?;
            l_max__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_max.clone(), (literal!("max")).clone())?;
            l_start__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_start.clone(), (literal!("start")).clone())?;
            l_fixed__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_fixed.clone(), (literal!("fixed")).clone())?;
            l_uncert__str = dumpUncertaintyAttrOpt(Tpl::emptyTxt.clone(), i_uncertainOption.clone())?;
            l_dist__str = dumpDistributionAttrOpt(Tpl::emptyTxt.clone(), i_distributionOption.clone())?;
            l_so__str = dumpStartOriginAttrOpt(Tpl::emptyTxt.clone(), i_startOrigin.clone())?;
            l_attrs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attrs__str = smf_102(l_attrs__str.clone(), l_quantity__str.clone())?;
            l_attrs__str = smf_103(l_attrs__str.clone(), l_min__str.clone())?;
            l_attrs__str = smf_104(l_attrs__str.clone(), l_max__str.clone())?;
            l_attrs__str = smf_105(l_attrs__str.clone(), l_start__str.clone())?;
            l_attrs__str = smf_106(l_attrs__str.clone(), l_fixed__str.clone())?;
            l_attrs__str = smf_107(l_attrs__str.clone(), l_uncert__str.clone())?;
            l_attrs__str = smf_108(l_attrs__str.clone(), l_dist__str.clone())?;
            l_attrs__str = smf_109(l_attrs__str.clone(), l_so__str.clone())?;
            l_attrs__str = Tpl::popIter(l_attrs__str.clone())?;
            txt = fun_110(txt.clone(), l_attrs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { startOrigin: i_startOrigin, fixed: i_fixed, start: i_start, quantity: i_quantity, .. }) => {
            let mut l_attrs__str: Tpl::Text;
            let mut l_so__str: Tpl::Text;
            let mut l_fixed__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_quantity__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_quantity__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_quantity.clone(), (literal!("quantity")).clone())?;
            l_start__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_start.clone(), (literal!("start")).clone())?;
            l_fixed__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_fixed.clone(), (literal!("fixed")).clone())?;
            l_so__str = dumpStartOriginAttrOpt(Tpl::emptyTxt.clone(), i_startOrigin.clone())?;
            l_attrs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attrs__str = smf_111(l_attrs__str.clone(), l_quantity__str.clone())?;
            l_attrs__str = smf_112(l_attrs__str.clone(), l_start__str.clone())?;
            l_attrs__str = smf_113(l_attrs__str.clone(), l_fixed__str.clone())?;
            l_attrs__str = smf_114(l_attrs__str.clone(), l_so__str.clone())?;
            l_attrs__str = Tpl::popIter(l_attrs__str.clone())?;
            txt = fun_115(txt.clone(), l_attrs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { startOrigin: i_startOrigin, start: i_start, quantity: i_quantity, .. }) => {
            let mut l_attrs__str: Tpl::Text;
            let mut l_so__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_quantity__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_quantity__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_quantity.clone(), (literal!("quantity")).clone())?;
            l_start__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_start.clone(), (literal!("start")).clone())?;
            l_so__str = dumpStartOriginAttrOpt(Tpl::emptyTxt.clone(), i_startOrigin.clone())?;
            l_attrs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attrs__str = smf_116(l_attrs__str.clone(), l_quantity__str.clone())?;
            l_attrs__str = smf_117(l_attrs__str.clone(), l_start__str.clone())?;
            l_attrs__str = smf_118(l_attrs__str.clone(), l_so__str.clone())?;
            l_attrs__str = Tpl::popIter(l_attrs__str.clone())?;
            txt = fun_119(txt.clone(), l_attrs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { startOrigin: i_startOrigin, fixed: i_fixed, start: i_start, max: i_max, min: i_min, quantity: i_quantity, .. }) => {
            let mut l_attrs__str: Tpl::Text;
            let mut l_so__str: Tpl::Text;
            let mut l_fixed__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_max__str: Tpl::Text;
            let mut l_min__str: Tpl::Text;
            let mut l_quantity__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_quantity__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_quantity.clone(), (literal!("quantity")).clone())?;
            l_min__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_min.clone(), (literal!("min")).clone())?;
            l_max__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_max.clone(), (literal!("max")).clone())?;
            l_start__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_start.clone(), (literal!("start")).clone())?;
            l_fixed__str = dumpExpAttrOpt(Tpl::emptyTxt.clone(), i_fixed.clone(), (literal!("fixed")).clone())?;
            l_so__str = dumpStartOriginAttrOpt(Tpl::emptyTxt.clone(), i_startOrigin.clone())?;
            l_attrs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_attrs__str = smf_120(l_attrs__str.clone(), l_quantity__str.clone())?;
            l_attrs__str = smf_121(l_attrs__str.clone(), l_min__str.clone())?;
            l_attrs__str = smf_122(l_attrs__str.clone(), l_max__str.clone())?;
            l_attrs__str = smf_123(l_attrs__str.clone(), l_start__str.clone())?;
            l_attrs__str = smf_124(l_attrs__str.clone(), l_fixed__str.clone())?;
            l_attrs__str = smf_125(l_attrs__str.clone(), l_so__str.clone())?;
            l_attrs__str = Tpl::popIter(l_attrs__str.clone())?;
            txt = fun_126(txt.clone(), l_attrs__str.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpExpAttrOpt(mut in_txt: Tpl::Text, mut in_a_exp: Option<Arc<DAE::Exp>>, mut in_a_attr: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_attr.clone())) {
        (txt, Some(i_e), a_attr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_attr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_e.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpStateSelectAttrOpt(mut in_txt: Tpl::Text, mut in_a_stateSelect: Option<DAE::StateSelect>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_stateSelect.clone()) {
        (mut txt, Some(mut i_ss)) => {
            txt = dumpStateSelectAttr(txt.clone(), i_ss.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpStateSelectAttr(mut txt: Tpl::Text, mut a_stateSelect: DAE::StateSelect) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("stateSelect = ")).clone() }))?;
    out_txt = dumpStateSelect(out_txt.clone(), a_stateSelect.clone())?;
    Ok(out_txt)
}

pub fn dumpStateSelect(mut in_txt: Tpl::Text, mut in_a_stateSelect: DAE::StateSelect) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_stateSelect.clone()) {
        (mut txt, DAE::StateSelect::NEVER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelect.never")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::StateSelect::AVOID) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelect.avoid")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::StateSelect::DEFAULT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelect.default")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::StateSelect::PREFER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelect.prefer")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::StateSelect::ALWAYS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StateSelect.always")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpUncertaintyAttrOpt(mut in_txt: Tpl::Text, mut in_a_uncertainty: Option<DAE::Uncertainty>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_uncertainty.clone()) {
        (mut txt, Some(mut i_u)) => {
            txt = dumpUncertaintyAttr(txt.clone(), i_u.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpUncertaintyAttr(mut txt: Tpl::Text, mut a_uncertainty: DAE::Uncertainty) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("uncertainty = ")).clone() }))?;
    out_txt = dumpUncertainty(out_txt.clone(), a_uncertainty.clone())?;
    Ok(out_txt)
}

pub fn dumpUncertainty(mut in_txt: Tpl::Text, mut in_a_uncertainty: DAE::Uncertainty) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_uncertainty.clone()) {
        (mut txt, DAE::Uncertainty::GIVEN) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Uncertainty.given")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Uncertainty::SOUGHT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Uncertainty.sought")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Uncertainty::REFINE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Uncertainty.refine")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Uncertainty::PROPAGATE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Uncertainty.propagate")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpDistributionAttrOpt(mut in_txt: Tpl::Text, mut in_a_distribution: Option<Arc<DAE::Distribution>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_distribution.clone())) {
        (txt, Some(i_d)) => {
            let mut txt = (*txt).clone();
            txt = dumpDistributionAttr(txt.clone(), i_d.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpDistributionAttr(mut txt: Tpl::Text, mut a_distribution: Arc<DAE::Distribution>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("distribution = ")).clone() }))?;
    out_txt = dumpDistribution(out_txt.clone(), a_distribution.clone())?;
    Ok(out_txt)
}

pub fn dumpDistribution(mut in_txt: Tpl::Text, mut in_a_distribution: Arc<DAE::Distribution>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_distribution.clone())) {
        (txt, Deref @ DAE::Distribution { paramNames: i_paramNames, params: i_params, name: i_name }) => {
            let mut l_paramnames__str: Tpl::Text;
            let mut l_params__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = dumpExp(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_params__str = dumpExp(Tpl::emptyTxt.clone(), i_params.clone())?;
            l_paramnames__str = dumpExp(Tpl::emptyTxt.clone(), i_paramNames.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Distribution(name = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", params = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_params__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", paramNames = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_paramnames__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_138(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_startOrigin: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_startOrigin.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_startOrigin) => {
            let mut txt = (*txt).clone();
            txt = dumpExpAttrOpt(txt.clone(), a_startOrigin.clone(), (literal!("startOrigin")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpStartOriginAttrOpt(mut txt: Tpl::Text, mut a_startOrigin: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: bool = false;
    ret_0 = Config::showStartOrigin()?;
    out_txt = fun_138(txt.clone(), ret_0.clone(), a_startOrigin.clone())?;
    Ok(out_txt)
}

fn fun_140(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_componentRef: Arc<DAE::ComponentRef>, mut in_a_subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>, mut in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_componentRef.clone(), in_a_subscriptLst.clone(), in_a_ident.clone())) {
        (txt, false, a_componentRef, a_subscriptLst, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = dumpSubscripts(txt.clone(), a_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpCref(txt.clone(), a_componentRef.clone())?;
            txt.clone()
        },
        (txt, _, a_componentRef, a_subscriptLst, a_ident) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = dumpSubscripts(txt.clone(), a_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            txt = dumpCref(txt.clone(), a_componentRef.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCref(mut in_txt: Tpl::Text, mut in_a_c: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_c.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: i_componentRef, subscriptLst: i_subscriptLst, ident: i_ident, .. }) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_140(txt.clone(), ret_0.clone(), i_componentRef.clone(), i_subscriptLst.clone(), (i_ident.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: i_subscriptLst, ident: i_ident @ Deref @ "$DER", .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("der(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = dumpSubscripts(txt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: i_subscriptLst, ident: i_ident, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = dumpSubscripts(txt.clone(), i_subscriptLst.clone())?;
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
fn lm_142(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpDimension(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_142(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpTypeDimensions(mut in_txt: Tpl::Text, mut in_a_dimensionLst: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_dimensionLst.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_dimensionLst) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_sub__str = lm_142(l_sub__str.clone(), i_dimensionLst.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_144(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpSubscript(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_144(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_145(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpSubscript(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_145(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_subscriptLst.clone())) {
        (txt, false, a_subscriptLst) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_sub__str = lm_144(l_sub__str.clone(), a_subscriptLst.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_subscriptLst) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_sub__str = lm_145(l_sub__str.clone(), a_subscriptLst.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSubscripts(mut in_txt: Tpl::Text, mut in_a_subscriptLst: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscriptLst.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscriptLst) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_146(txt.clone(), ret_0.clone(), i_subscriptLst.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSubscript(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<DAE::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone())) {
        (txt, Deref @ DAE::Subscript::WHOLEDIM) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::SLICE { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLE_NONEXP { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1:")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
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
fn lm_149(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ineq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_ineq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_149(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpInitialEquationSection(mut in_txt: Tpl::Text, mut in_a_ie: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ie.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_ie) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("initial equation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_149(txt.clone(), i_ie.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_151(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_151(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpEquationSection(mut in_txt: Tpl::Text, mut in_a_e: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_e.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_e) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("equation\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_151(txt.clone(), i_e.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEquationElement(mut in_txt: Tpl::Text, mut in_a_lst: Arc<DAE::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lst.clone())) {
        (txt, Deref @ DAE::Element::EQUATION { source: i_source, scalar: i_scalar, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_exp.clone(), i_scalar.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::EQUEQUATION { source: i_source, cr2: i_cr2, cr1: i_cr1 }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquEquation(txt.clone(), i_cr1.clone(), i_cr2.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::ARRAY_EQUATION { source: i_source, array: i_array, exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_exp.clone(), i_array.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::COMPLEX_EQUATION { source: i_source, rhs: i_rhs, lhs: i_lhs }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_lhs.clone(), i_rhs.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::DEFINE { source: i_source, exp: i_exp, componentRef: i_componentRef }) => {
            let mut txt = (*txt).clone();
            txt = dumpDefine(txt.clone(), i_componentRef.clone(), i_exp.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, i_lst @ Deref @ DAE::Element::WHEN_EQUATION { condition: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpWhenEquation(txt.clone(), i_lst.clone())?;
            txt.clone()
        },
        (txt, i_lst @ Deref @ DAE::Element::FOR_EQUATION { type_: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpForEquation(txt.clone(), i_lst.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::IF_EQUATION { source: i_source, equations3: i_equations3, equations2: i_equations2, condition1: i_condition1 }) => {
            let mut txt = (*txt).clone();
            txt = dumpIfEquation(txt.clone(), i_condition1.clone(), i_equations2.clone(), i_equations3.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::ASSERT { source: i_source, level: i_level, message: i_message, condition: i_condition }) => {
            let mut txt = (*txt).clone();
            txt = dumpAssert(txt.clone(), i_condition.clone(), i_message.clone(), i_level.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIAL_ASSERT { source: i_source, level: i_level, message: i_message, condition: i_condition }) => {
            let mut txt = (*txt).clone();
            txt = dumpAssert(txt.clone(), i_condition.clone(), i_message.clone(), i_level.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::TERMINATE { source: i_source, message: i_message }) => {
            let mut txt = (*txt).clone();
            txt = dumpTerminate(txt.clone(), i_message.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIAL_TERMINATE { source: i_source, message: i_message }) => {
            let mut txt = (*txt).clone();
            txt = dumpTerminate(txt.clone(), i_message.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::REINIT { source: i_source, exp: i_exp, componentRef: i_componentRef }) => {
            let mut txt = (*txt).clone();
            txt = dumpReinit(txt.clone(), i_componentRef.clone(), i_exp.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::NORETCALL { source: i_source, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpNoRetCall(txt.clone(), i_exp.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIAL_NORETCALL { source: i_source, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpNoRetCall(txt.clone(), i_exp.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIALDEFINE { source: i_source, exp: i_exp, componentRef: i_componentRef }) => {
            let mut txt = (*txt).clone();
            txt = dumpDefine(txt.clone(), i_componentRef.clone(), i_exp.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { source: i_source, array: i_array, exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_exp.clone(), i_array.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { source: i_source, rhs: i_rhs, lhs: i_lhs }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_lhs.clone(), i_rhs.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, i_lst @ Deref @ DAE::Element::INITIAL_FOR_EQUATION { type_: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpForEquation(txt.clone(), i_lst.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIAL_IF_EQUATION { source: i_source, equations3: i_equations3, equations2: i_equations2, condition1: i_condition1 }) => {
            let mut txt = (*txt).clone();
            txt = dumpIfEquation(txt.clone(), i_condition1.clone(), i_equations2.clone(), i_equations3.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIALEQUATION { source: i_source, exp2: i_exp2, exp1: i_exp1 }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_exp1.clone(), i_exp2.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKNOWN EQUATION TYPE")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_154(mut in_txt: Tpl::Text, mut in_a_lhs: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lhs.clone())) {
        (txt, i_lhs @ Deref @ DAE::Exp::IFEXP { expCond: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_lhs) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_lhs.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEquation(mut txt: Tpl::Text, mut a_lhs: Arc<DAE::Exp>, mut a_rhs: Arc<DAE::Exp>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_rhs__str: Tpl::Text;
    let mut l_lhs__str: Tpl::Text;
    l_lhs__str = fun_154(Tpl::emptyTxt.clone(), a_lhs.clone())?;
    l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), a_rhs.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_rhs__str.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub fn dumpEquEquation(mut txt: Tpl::Text, mut a_lhs: Arc<DAE::ComponentRef>, mut a_rhs: Arc<DAE::ComponentRef>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_rhs__str: Tpl::Text;
    let mut l_lhs__str: Tpl::Text;
    l_lhs__str = dumpCref(Tpl::emptyTxt.clone(), a_lhs.clone())?;
    l_rhs__str = dumpCref(Tpl::emptyTxt.clone(), a_rhs.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_rhs__str.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub fn dumpDefine(mut txt: Tpl::Text, mut a_lhs: Arc<DAE::ComponentRef>, mut a_rhs: Arc<DAE::Exp>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_rhs__str: Tpl::Text;
    let mut l_lhs__str: Tpl::Text;
    l_lhs__str = dumpCref(Tpl::emptyTxt.clone(), a_lhs.clone())?;
    l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), a_rhs.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_rhs__str.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

fn fun_158(mut in_txt: Tpl::Text, mut in_a_lvl: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lvl.clone())) {
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: 1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", AssertionLevel.warning")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAssert(mut txt: Tpl::Text, mut a_cond: Arc<DAE::Exp>, mut a_msg: Arc<DAE::Exp>, mut a_lvl: Arc<DAE::Exp>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_lvl__str: Tpl::Text;
    let mut l_msg__str: Tpl::Text;
    let mut l_cond__str: Tpl::Text;
    l_cond__str = dumpExp(Tpl::emptyTxt.clone(), a_cond.clone())?;
    l_msg__str = dumpExp(Tpl::emptyTxt.clone(), a_msg.clone())?;
    l_lvl__str = fun_158(Tpl::emptyTxt.clone(), a_lvl.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_cond__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_msg__str.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_lvl__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub fn dumpTerminate(mut txt: Tpl::Text, mut a_msg: Arc<DAE::Exp>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_msg__str: Tpl::Text;
    l_msg__str = dumpExp(Tpl::emptyTxt.clone(), a_msg.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("terminate(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_msg__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub fn dumpReinit(mut txt: Tpl::Text, mut a_cref: Arc<DAE::ComponentRef>, mut a_exp: Arc<DAE::Exp>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_exp__str: Tpl::Text;
    let mut l_cref__str: Tpl::Text;
    l_cref__str = dumpCref(Tpl::emptyTxt.clone(), a_cref.clone())?;
    l_exp__str = dumpExp(Tpl::emptyTxt.clone(), a_exp.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_cref__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_exp__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

fn fun_162(mut in_txt: Tpl::Text, mut in_a_call__exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_call__exp.clone())) {
        (txt, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { .. }, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpNoRetCall(mut txt: Tpl::Text, mut a_call__exp: Arc<DAE::Exp>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_tail__str: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_call__str: Tpl::Text;
    l_call__str = dumpExp(Tpl::emptyTxt.clone(), a_call__exp.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    l_tail__str = fun_162(Tpl::emptyTxt.clone(), a_call__exp.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_tail__str.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_call__str.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_164(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_164(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_165(mut in_txt: Tpl::Text, mut in_a_elsewhen__: Option<Arc<DAE::Element>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elsewhen__.clone())) {
        (txt, Some(i_el)) => {
            let mut txt = (*txt).clone();
            txt = dumpWhenEquation(txt.clone(), i_el.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_166(mut in_txt: Tpl::Text, mut in_a_elsewhen__str: Tpl::Text, mut in_a_src__str: Tpl::Text, mut in_a_body__str: Tpl::Text, mut in_a_when__cond__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elsewhen__str.clone(), in_a_src__str.clone(), in_a_body__str.clone(), in_a_when__cond__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_src__str, a_body__str, a_when__cond__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_when__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, i_elsewhen__str, _, a_body__str, a_when__cond__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_when__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_elsewhen__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpWhenEquation(mut in_txt: Tpl::Text, mut in_a_lst: Arc<DAE::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lst.clone())) {
        (txt, Deref @ DAE::Element::WHEN_EQUATION { source: i_source, elsewhen_: i_elsewhen__, equations: i_equations, condition: i_condition }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_when__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_when__cond__str = dumpExp(Tpl::emptyTxt.clone(), i_condition.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_164(l_body__str.clone(), i_equations.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_elsewhen__str = fun_165(Tpl::emptyTxt.clone(), i_elsewhen__.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = fun_166(txt.clone(), l_elsewhen__str.clone(), l_src__str.clone(), l_body__str.clone(), l_when__cond__str.clone())?;
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
fn lm_168(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_168(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_169(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_169(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpForEquation(mut in_txt: Tpl::Text, mut in_a_lst: Arc<DAE::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lst.clone())) {
        (txt, Deref @ DAE::Element::FOR_EQUATION { iter: i_iter, source: i_source, equations: i_equations, range: i_range, .. }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_range__str = dumpExp(Tpl::emptyTxt.clone(), i_range.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_168(l_body__str.clone(), i_equations.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_iter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Element::INITIAL_FOR_EQUATION { iter: i_iter, source: i_source, equations: i_equations, range: i_range, .. }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_range__str = dumpExp(Tpl::emptyTxt.clone(), i_range.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_169(l_body__str.clone(), i_equations.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_iter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_171(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_171(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_172(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_172(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_173(mut in_txt: Tpl::Text, mut in_a_else__branch: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__branch.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_else__branch) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_172(txt.clone(), i_else__branch.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_174(mut in_txt: Tpl::Text, mut in_a_branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut in_a_src: Arc<DAE::ElementSource>, mut in_a_else__branch: Arc<metamodelica::List<Arc<DAE::Element>>>, mut in_a_elseif__conds: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_if__cond: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_branches.clone(), in_a_src.clone(), in_a_else__branch.clone(), in_a_elseif__conds.clone(), in_a_if__cond.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_if__branch, tail: i_elseif__branches }, a_src, a_else__branch, a_elseif__conds, a_if__cond) => {
            let mut l_src__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__branch__str: Tpl::Text;
            let mut l_if__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_if__cond__str = dumpExp(Tpl::emptyTxt.clone(), a_if__cond.clone())?;
            l_if__branch__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_if__branch__str = lm_171(l_if__branch__str.clone(), i_if__branch.clone())?;
            l_if__branch__str = Tpl::popIter(l_if__branch__str.clone())?;
            l_elseif__str = dumpElseIfEquation(Tpl::emptyTxt.clone(), a_elseif__conds.clone(), i_elseif__branches.clone())?;
            l_else__str = fun_173(Tpl::emptyTxt.clone(), a_else__branch.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_if__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_if__branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elseif__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end if")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
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

pub fn dumpIfEquation(mut in_txt: Tpl::Text, mut in_a_conds: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut in_a_else__branch: Arc<metamodelica::List<Arc<DAE::Element>>>, mut in_a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_conds.clone(), in_a_branches.clone(), in_a_else__branch.clone(), in_a_src.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_if__cond, tail: i_elseif__conds }, a_branches, a_else__branch, a_src) => {
            let mut txt = (*txt).clone();
            txt = fun_174(txt.clone(), a_branches.clone(), a_src.clone(), a_else__branch.clone(), i_elseif__conds.clone(), i_if__cond.clone())?;
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
fn lm_176(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationElement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_176(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_177(mut in_txt: Tpl::Text, mut in_a_equations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>, mut in_a_rest__conds: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_cond: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equations.clone(), in_a_rest__conds.clone(), in_a_cond.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_branch, tail: i_rest__branches }, a_rest__conds, a_cond) => {
            let mut l_rest__str: Tpl::Text;
            let mut l_branch__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), a_cond.clone())?;
            l_branch__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_branch__str = lm_176(l_branch__str.clone(), i_branch.clone())?;
            l_branch__str = Tpl::popIter(l_branch__str.clone())?;
            l_rest__str = dumpElseIfEquation(Tpl::emptyTxt.clone(), a_rest__conds.clone(), i_rest__branches.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elseif ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rest__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElseIfEquation(mut in_txt: Tpl::Text, mut in_a_condition1: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_equations: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Element>>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_condition1.clone(), in_a_equations.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_cond, tail: i_rest__conds }, a_equations) => {
            let mut txt = (*txt).clone();
            txt = fun_177(txt.clone(), a_equations.clone(), i_rest__conds.clone(), i_cond.clone())?;
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
fn lm_179(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_alg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpInitialAlgorithm(txt.clone(), i_alg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_179(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpInitialAlgorithmSection(mut txt: Tpl::Text, mut a_ia: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_179(out_txt.clone(), a_ia.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpInitialAlgorithm(mut in_txt: Tpl::Text, mut in_a_alg: Arc<DAE::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone())) {
        (txt, Deref @ DAE::Element::INITIALALGORITHM { algorithm_: i_algorithm__, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithm(txt.clone(), i_algorithm__.clone(), (literal!("initial algorithm")).clone())?;
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
fn lm_182(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_alg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmElement(txt.clone(), i_alg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_182(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgorithmSection(mut txt: Tpl::Text, mut a_a: Arc<metamodelica::List<Arc<DAE::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_182(out_txt.clone(), a_a.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpAlgorithmElement(mut in_txt: Tpl::Text, mut in_a_alg: Arc<DAE::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone())) {
        (txt, Deref @ DAE::Element::ALGORITHM { algorithm_: i_algorithm__, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithm(txt.clone(), i_algorithm__.clone(), (literal!("algorithm")).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgorithm(mut in_txt: Tpl::Text, mut in_a_algorithm__: Arc<DAE::Algorithm>, mut in_a_header: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_algorithm__.clone(), in_a_header.clone())) {
        (txt, Deref @ DAE::Algorithm { statementLst: i_statementLst }, a_header) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_header.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpStatements(txt.clone(), i_statementLst.clone())?;
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

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_186(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_186(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpStatements(mut txt: Tpl::Text, mut a_stmts: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_186(out_txt.clone(), a_stmts.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSIGN { source: i_source, exp: i_exp, exp1: i_exp1, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpAssignment(txt.clone(), i_exp1.clone(), i_exp.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpTupleAssignStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpArrayAssignStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_IF { exp: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpIfStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_FOR { iterIsArray: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpForStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_PARFOR { iterIsArray: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpParForStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_WHILE { exp: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpWhileStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_WHEN { exp: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpWhenStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_ASSERT { source: i_source, level: i_level, msg: i_msg, cond: i_cond }) => {
            let mut txt = (*txt).clone();
            txt = dumpAssert(txt.clone(), i_cond.clone(), i_msg.clone(), i_level.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_TERMINATE { source: i_source, msg: i_msg }) => {
            let mut txt = (*txt).clone();
            txt = dumpTerminate(txt.clone(), i_msg.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, i_stmt @ Deref @ DAE::Statement::STMT_REINIT { var: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpReinitStatement(txt.clone(), i_stmt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_NORETCALL { source: i_source, exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpNoRetCall(txt.clone(), i_exp.clone(), i_source.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_RETURN { source: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_BREAK { source: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("break;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_CONTINUE { source: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continue;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Statement::STMT_FAILURE { body: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fail();")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("DAEDumpTypes.dumpStatement: Unknown statement.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_189(mut in_txt: Tpl::Text, mut in_a_lhs: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lhs.clone())) {
        (txt, i_lhs @ Deref @ DAE::Exp::IFEXP { expCond: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_lhs) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_lhs.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAssignment(mut txt: Tpl::Text, mut a_lhs: Arc<DAE::Exp>, mut a_rhs: Arc<DAE::Exp>, mut a_src: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_src__str: Tpl::Text;
    let mut l_rhs__str: Tpl::Text;
    let mut l_lhs__str: Tpl::Text;
    l_lhs__str = fun_189(Tpl::emptyTxt.clone(), a_lhs.clone())?;
    l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), a_rhs.clone())?;
    l_src__str = dumpSource(Tpl::emptyTxt.clone(), a_src.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" := ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_rhs__str.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_src__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_191(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_191(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpTupleAssignStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { source: i_source, exp: i_exp, expExpLst: i_expExpLst, .. }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_lhs__str = lm_191(l_lhs__str.clone(), i_expExpLst.clone())?;
            l_lhs__str = Tpl::popIter(l_lhs__str.clone())?;
            l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") := ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpArrayAssignStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_ASSIGN_ARR { source: i_source, exp: i_exp, lhs: i_lhs, .. }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExp(Tpl::emptyTxt.clone(), i_lhs.clone())?;
            l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" := ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_194(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_194(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpIfStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_IF { source: i_source, else_: i_else__, statementLst: i_statementLst, exp: i_exp }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_else__if__str: Tpl::Text;
            let mut l_true__branch__str: Tpl::Text;
            let mut l_if__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_if__cond__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_true__branch__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_true__branch__str = lm_194(l_true__branch__str.clone(), i_statementLst.clone())?;
            l_true__branch__str = Tpl::popIter(l_true__branch__str.clone())?;
            l_else__if__str = dumpElseIfStatements(Tpl::emptyTxt.clone(), i_else__.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_if__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_true__branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__if__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end if")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_196(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_196(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_197(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_197(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpElseIfStatements(mut in_txt: Tpl::Text, mut in_a_else__: Arc<DAE::Else>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__.clone())) {
        (txt, Deref @ DAE::Else::ELSEIF { else_: i_else__, statementLst: i_statementLst, exp: i_exp }) => {
            let mut l_else__str: Tpl::Text;
            let mut l_elseif__body__str: Tpl::Text;
            let mut l_elseif__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_elseif__cond__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_elseif__body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elseif__body__str = lm_196(l_elseif__body__str.clone(), i_statementLst.clone())?;
            l_elseif__body__str = Tpl::popIter(l_elseif__body__str.clone())?;
            l_else__str = dumpElseIfStatements(Tpl::emptyTxt.clone(), i_else__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elseif ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elseif__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_elseif__body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Else::ELSE { statementLst: i_statementLst }) => {
            let mut l_else__body__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_else__body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_else__body__str = lm_197(l_else__body__str.clone(), i_statementLst.clone())?;
            l_else__body__str = Tpl::popIter(l_else__body__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_else__body__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
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
fn lm_199(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_199(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpForStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_FOR { iter: i_iter, source: i_source, statementLst: i_statementLst, range: i_range, .. }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_alg__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_range__str = dumpExp(Tpl::emptyTxt.clone(), i_range.clone())?;
            l_alg__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_alg__str = lm_199(l_alg__str.clone(), i_statementLst.clone())?;
            l_alg__str = Tpl::popIter(l_alg__str.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_iter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_alg__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_201(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_201(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpParForStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_PARFOR { iter: i_iter, source: i_source, statementLst: i_statementLst, range: i_range, .. }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_alg__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_range__str = dumpExp(Tpl::emptyTxt.clone(), i_range.clone())?;
            l_alg__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_alg__str = lm_201(l_alg__str.clone(), i_statementLst.clone())?;
            l_alg__str = Tpl::popIter(l_alg__str.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parfor ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_iter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_alg__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_203(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_203(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpWhileStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_WHILE { source: i_source, statementLst: i_statementLst, exp: i_exp }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_while__cond: Tpl::Text;
            let mut txt = (*txt).clone();
            l_while__cond = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_203(l_body__str.clone(), i_statementLst.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("while ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_while__cond.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end while")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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
fn lm_205(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_205(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_206(mut in_txt: Tpl::Text, mut in_a_elseWhen: Option<Arc<DAE::Statement>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elseWhen.clone())) {
        (txt, Some(i_ew)) => {
            let mut txt = (*txt).clone();
            txt = dumpWhenStatement(txt.clone(), i_ew.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_207(mut in_txt: Tpl::Text, mut in_a_elsewhen__str: Tpl::Text, mut in_a_src__str: Tpl::Text, mut in_a_body__str: Tpl::Text, mut in_a_when__cond__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elsewhen__str.clone(), in_a_src__str.clone(), in_a_body__str.clone(), in_a_when__cond__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_src__str, a_body__str, a_when__cond__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_when__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, i_elsewhen__str, _, a_body__str, a_when__cond__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_when__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_elsewhen__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpWhenStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_WHEN { source: i_source, elseWhen: i_elseWhen, statementLst: i_statementLst, exp: i_exp, .. }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_when__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_when__cond__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_205(l_body__str.clone(), i_statementLst.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_elsewhen__str = fun_206(Tpl::emptyTxt.clone(), i_elseWhen.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = fun_207(txt.clone(), l_elsewhen__str.clone(), l_src__str.clone(), l_body__str.clone(), l_when__cond__str.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpReinitStatement(mut in_txt: Tpl::Text, mut in_a_stmt: Arc<DAE::Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_stmt.clone())) {
        (txt, Deref @ DAE::Statement::STMT_REINIT { source: i_source, value: i_value, var: i_var }) => {
            let mut l_src__str: Tpl::Text;
            let mut l_new__exp__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_var.clone())?;
            l_new__exp__str = dumpExp(Tpl::emptyTxt.clone(), i_value.clone())?;
            l_src__str = dumpSource(Tpl::emptyTxt.clone(), i_source.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_new__exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_src__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_210(mut in_txt: Tpl::Text, mut in_a_comment: Option<Arc<SCode::Comment>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Some(i_co)) => {
            let mut txt = (*txt).clone();
            txt = dumpStateMachineComment(txt.clone(), i_co.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpStateMachineSection(mut in_txt: Tpl::Text, mut in_a_fixedDae: Arc<DAEDumpTypes::compWithSplitElements>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fixedDae.clone())) {
        (txt, Deref @ DAEDumpTypes::compWithSplitElements { spltElems: i_spltElems, name: i_name, comment: i_comment }) => {
            let mut l_kind: Tpl::Text;
            let mut txt = (*txt).clone();
            l_kind = fun_210(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_kind.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpCompStream(txt.clone(), i_spltElems.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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

fn fun_212(mut in_txt: Tpl::Text, mut in_a_comment: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_comment.clone()) {
        (mut txt, Some(mut i_co)) => {
            txt = Tpl::writeStr(txt.clone(), (i_co.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpStateMachineComment(mut in_txt: Tpl::Text, mut in_a_cmt: Arc<SCode::Comment>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cmt.clone())) {
        (txt, Deref @ SCode::Comment { comment: i_comment, .. }) => {
            let mut l_kind__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_kind__str = fun_212(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_kind__str.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpExp(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = ExpressionDumpTpl::dumpExp(txt.clone(), a_exp.clone(), (literal!("\"")).clone())?;
    Ok(out_txt)
}

fn fun_215(mut in_txt: Tpl::Text, mut in_a_cmt__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cmt__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_cmt__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassAnnotation(mut txt: Tpl::Text, mut a_comment: Option<Arc<SCode::Comment>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_cmt__str: Tpl::Text;
    l_cmt__str = dumpCommentAnnotation(Tpl::emptyTxt.clone(), a_comment.clone())?;
    out_txt = fun_215(txt.clone(), l_cmt__str.clone())?;
    Ok(out_txt)
}

fn fun_217(mut in_txt: Tpl::Text, mut in_a_cmt__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cmt__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_cmt__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_cmt__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCompAnnotation(mut txt: Tpl::Text, mut a_comment: Option<Arc<SCode::Comment>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_cmt__str: Tpl::Text;
    l_cmt__str = dumpCommentAnnotation(Tpl::emptyTxt.clone(), a_comment.clone())?;
    out_txt = fun_217(txt.clone(), l_cmt__str.clone())?;
    Ok(out_txt)
}

pub fn dumpCommentAnnotation(mut in_txt: Tpl::Text, mut in_a_comment: Option<Arc<SCode::Comment>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Some(i_cmt)) => {
            let mut txt = (*txt).clone();
            txt = dumpCommentAnnotationNoOpt(txt.clone(), i_cmt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCommentAnnotationNoOpt(mut in_txt: Tpl::Text, mut in_a_comment: Arc<SCode::Comment>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ SCode::Comment { annotation_: Some(i_ann), .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpAnnotation(txt.clone(), i_ann.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCommentOpt(mut in_txt: Tpl::Text, mut in_a_comment: Option<Arc<SCode::Comment>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Some(i_cmt)) => {
            let mut txt = (*txt).clone();
            txt = dumpComment(txt.clone(), i_cmt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpComment(mut in_txt: Tpl::Text, mut in_a_comment: Arc<SCode::Comment>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ SCode::Comment { comment: i_comment, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpCommentStr(txt.clone(), i_comment.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCommentStr(mut in_txt: Tpl::Text, mut in_a_comment: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_comment.clone()) {
        (mut txt, Some(mut i_cmt)) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            ret_0 = (System::escapedString((i_cmt.clone()).clone(), false)).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpAnnotationOpt(mut in_txt: Tpl::Text, mut in_a_annotation: Option<Arc<SCode::Annotation>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annotation.clone())) {
        (txt, Some(i_ann)) => {
            let mut txt = (*txt).clone();
            txt = dumpAnnotation(txt.clone(), i_ann.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_225(mut in_txt: Tpl::Text, mut in_a_ann__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann__str.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_ann__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("annotation")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_ann__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_226(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ann__mod: Arc<SCode::Mod>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ann__mod.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ann__mod) => {
            let mut ret_1: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
            let mut l_ann__str: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = DAEDumpTypes::filterStructuralMods(a_ann__mod.clone());
            l_ann__str = SCodeDumpTpl::dumpModifier(Tpl::emptyTxt.clone(), ret_1.clone(), SCodeDump::defaultOptions.clone())?;
            txt = fun_225(txt.clone(), l_ann__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_227(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ann__mod: Arc<SCode::Mod>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ann__mod.clone())) {
        (txt, false, a_ann__mod) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Config::showStructuralAnnotations()?;
            txt = fun_226(txt.clone(), ret_0.clone(), a_ann__mod.clone())?;
            txt.clone()
        },
        (txt, _, a_ann__mod) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("annotation")).clone() }))?;
            txt = SCodeDumpTpl::dumpModifier(txt.clone(), a_ann__mod.clone(), SCodeDump::defaultOptions.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAnnotation(mut in_txt: Tpl::Text, mut in_a_annotation: Arc<SCode::Annotation>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annotation.clone())) {
        (txt, Deref @ SCode::Annotation { modification: i_ann__mod }) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Config::showAnnotations()?;
            txt = fun_227(txt.clone(), ret_0.clone(), i_ann__mod.clone())?;
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
pub fn dumpPathLastIndent(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = dumpPathLastIndent(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::QUALIFIED { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpPathLastIndent(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("dumpPathLastIndent: Unknown path.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_230(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Comment>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_c, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpComment(txt.clone(), i_c.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_230(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpSource(mut in_txt: Tpl::Text, mut in_a_source: Arc<DAE::ElementSource>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_source.clone())) {
        (txt, Deref @ DAE::ElementSource { comment: i_comment, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_230(txt.clone(), i_comment.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn errorMsg(mut txt: Tpl::Text, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    Tpl::addTemplateError((a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeStr(txt.clone(), (a_errMessage.clone()).clone())?;
    Ok(out_txt)
}

