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
use crate::CodegenUtil;
use crate::SimCode;
use crate::SimCodeVar;
use openmodelica_frontend::DAEDump;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn modelNamePrefix(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, .. }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            ret_0 = (System::makeC89Identifier((i_fileNamePrefix.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fileNamePrefix(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fileNamePrefix: mut i_fileNamePrefix, .. }) => {
            txt = Tpl::writeStr(txt.clone(), (i_fileNamePrefix.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fullPathPrefix(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { fullPathPrefix: mut i_fullPathPrefix, .. }) => {
            txt = Tpl::writeStr(txt.clone(), (i_fullPathPrefix.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn equationIndex(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_IFEQUATION { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { index: i_ls_index, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_ls_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { index: i_nls_index, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_nls_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_MIXED { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_WHEN { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_EQUATION { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: i_aliasOf, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_aliasOf.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { index: i_index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenUtilSimulation.tpl")).clone(), 138, 14), (literal!("equationIndex failed")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationIndexAlternativeTearing(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(Deref @ SimCode::LinearSystem { index: i_at_index, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_at_index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(Deref @ SimCode::NonlinearSystem { index: i_at_index, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_at_index.clone())).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn equationIndexGeneral(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, i_eq @ Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(_), .. }) => {
            let mut txt = (*txt).clone();
            txt = equationIndexAlternativeTearing(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        (txt, i_eq @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(_), .. }) => {
            let mut txt = (*txt).clone();
            txt = equationIndexAlternativeTearing(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        (txt, i_eq) => {
            let mut txt = (*txt).clone();
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_51(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_eqs.clone())) {
        (txt, Deref @ "none", a_eqs) => {
            let mut txt = (*txt).clone();
            txt = dumpEqsWork(txt.clone(), a_eqs.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEqs(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: ArcStr = arcstr::literal!("");
    ret_0 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
    out_txt = fun_51(txt.clone(), (ret_0.clone()).clone(), a_eqs.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_53(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (DAEDump::ppStmtStr(i_stmt.clone(), 2)?).clone();
            txt = CodegenUtil::escapeCComments(txt.clone(), (ret_0.clone()).clone())?;
            txt = lm_53(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_54(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (DAEDump::ppStmtStr(i_stmt.clone(), 2)?).clone();
            txt = CodegenUtil::escapeCComments(txt.clone(), (ret_0.clone()).clone())?;
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
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { name: i_cr, .. }, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<var>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cr.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</var>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_55(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_55(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_2: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
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
fn lm_58(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_i1, i_i2, i_eq), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell row=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" col=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_57(txt.clone(), i_eq.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt = lm_58(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_59(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cr, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cr.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
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
fn lm_60(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = equationIndex(txt.clone(), i_eq.clone())?;
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
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { name: i_cr, .. }, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<var>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cr.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</var>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_61(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_61(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_62(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<discrete index=\"")).clone() }))?;
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
            txt = lm_62(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_63(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = equationIndex(txt.clone(), i_eq.clone())?;
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
fn lm_64(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cond, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_cond.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
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
fn lm_65(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<DAE::Exp>, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: (_, i_eqs), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEqs(txt.clone(), i_eqs.clone())?;
            txt = lm_65(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_66(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: RESIDUAL\n")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: FOR_RESIDUAL\n")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_2: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: GENERIC_RESIDUAL\n")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { exp: i_e_exp, cref: i_e_cref, .. }) => {
            let mut txt_4: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: SIMPLE_ASSIGN\n")).clone() }))?;
            txt_3 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_e_cref.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_4 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_4.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN_CONSTRAINTS { cons: i_e_cons, exp: i_e_exp, cref: i_e_cref, .. }) => {
            let mut txt_7: Tpl::Text;
            let mut txt_6: Tpl::Text;
            let mut txt_5: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: SIMPLE_ASSIGN_CONSTRAINTS\n")).clone() }))?;
            txt_5 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_e_cref.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_5.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_6 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_6.clone())?).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constraints: ")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpConstraints(Tpl::emptyTxt.clone(), i_e_cons.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_7.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN { exp: i_e_exp, lhs: Deref @ DAE::Exp::CREF { componentRef: i_lhs_componentRef, .. }, .. }) => {
            let mut txt_9: Tpl::Text;
            let mut txt_8: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: ARRAY_CALL_ASSIGN\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt_8 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_lhs_componentRef.clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_8.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_9 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_9.clone())?).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_RESIZABLE_ASSIGN { call_index: i_e_call__index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("type: SES_RESIZABLE_ASSIGN call index: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_e_call__index.clone())).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_GENERIC_ASSIGN { call_index: i_e_call__index, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("type: SES_GENERIC_ASSIGN call index: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_e_call__index.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("empty algorithm")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ALGORITHM { statements: i_e_statements @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: ALGORITHM\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = lm_53(txt.clone(), i_e_statements.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_INVERSE_ALGORITHM { statements: i_e_statements @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: INVERSE ALGORITHM\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = lm_54(txt.clone(), i_e_statements.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_LINEAR { lSystem: Deref @ SimCode::LinearSystem { simJac: i_ls_simJac, beqs: i_ls_beqs, vars: i_ls_vars, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: LINEAR\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_55(txt.clone(), i_ls_vars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<row>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_56(txt.clone(), i_ls_beqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</row>\n")).clone(), (literal!("<matrix>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_58(txt.clone(), i_ls_simJac.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</matrix>")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { nlSystem: Deref @ SimCode::NonlinearSystem { eqs: i_nls_eqs, crefs: i_nls_crefs, indexNonLinearSystem: i_nls_indexNonLinearSystem, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("indexNonlinear: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_nls_indexNonLinearSystem.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: NONLINEAR\n")).clone(), (literal!("\n")).clone(), (literal!("vars: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_59(txt.clone(), i_nls_crefs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("eqns: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_60(txt.clone(), i_nls_eqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_MIXED { discVars: i_e_discVars, discEqs: i_e_discEqs, cont: i_e_cont, .. }) => {
            let mut ret_10: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: MIXED\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            ret_10 = List::fill(i_e_cont.clone(), 1);
            txt = dumpEqs(txt.clone(), ret_10.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = dumpEqs(txt.clone(), i_e_discEqs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone(), (literal!("<mixed>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<continuous index=\"")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e_cont.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\" />\n")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_61(txt.clone(), i_e_discVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_62(txt.clone(), i_e_discEqs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</mixed>")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ALGEBRAIC_SYSTEM { linearSystem: i_e_linearSystem, matrix: i_matrix, residual: Deref @ SimCode::OMSIFunction { equations: i_residual_equations, .. }, .. }) => {
            let mut ret_12: i32 = 0;
            let mut l_detailedDescription: Tpl::Text;
            let mut txt = (*txt).clone();
            l_detailedDescription = dumpAlgSystemOps(Tpl::emptyTxt.clone(), i_matrix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: ALGEBRAIC_SYSTEM\n")).clone(), (literal!("is linear: ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_e_linearSystem.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("depending functions indices: ")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_63(txt.clone(), i_residual_equations.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dimension: ")).clone() }))?;
            ret_12 = (i_residual_equations.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_12.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_detailedDescription.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_WHEN { conditions: i_conditions, whenStmtLst: i_whenStmtLst, .. }) => {
            let mut l_body: Tpl::Text;
            let mut txt = (*txt).clone();
            l_body = dumpWhenOps(Tpl::emptyTxt.clone(), i_whenStmtLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: WHEN\n")).clone(), (literal!("\n")).clone(), (literal!("when {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_64(txt.clone(), i_conditions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("} then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when;")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_IFEQUATION { elsebranch: i_elsebranch, ifbranches: i_ifbranches, .. }) => {
            let mut l_elsebr: Tpl::Text;
            let mut l_branches: Tpl::Text;
            let mut txt = (*txt).clone();
            l_branches = lm_65(Tpl::emptyTxt.clone(), i_ifbranches.clone())?;
            l_elsebr = dumpEqs(Tpl::emptyTxt.clone(), i_elsebranch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: IFEQUATION\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_branches.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elsebr.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_FOR_LOOP { exp: i_e_exp, cref: i_e_cref, endIt: i_e_endIt, startIt: i_e_startIt, iter: i_e_iter, .. }) => {
            let mut txt_21: Tpl::Text;
            let mut txt_20: Tpl::Text;
            let mut txt_19: Tpl::Text;
            let mut txt_18: Tpl::Text;
            let mut txt_17: Tpl::Text;
            let mut l_forstatement: Tpl::Text;
            let mut txt = (*txt).clone();
            l_forstatement = Tpl::emptyTxt.clone();
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt_17 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_iter.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_17.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt_18 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_startIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_18.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : ")).clone() }))?;
            txt_19 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_endIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_19.clone())?).clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("loop")).clone() }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt_20 = ExpressionDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_e_cref.clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_20.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_21 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_21.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; ")).clone() }))?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: FOR_LOOP\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_forstatement.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_FOR_EQUATION { body: i_e_body, endIt: i_e_endIt, startIt: i_e_startIt, iter: i_e_iter, .. }) => {
            let mut txt_24: Tpl::Text;
            let mut txt_23: Tpl::Text;
            let mut txt_22: Tpl::Text;
            let mut l_forstatement: Tpl::Text;
            let mut txt = (*txt).clone();
            l_forstatement = Tpl::emptyTxt.clone();
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt_22 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_iter.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_22.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt_23 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_startIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_23.clone())?).clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" : ")).clone() }))?;
            txt_24 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_endIt.clone(), (literal!("\"")).clone())?;
            l_forstatement = CodegenUtil::escapeCComments(l_forstatement.clone(), (Tpl::textString(txt_24.clone())?).clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("loop")).clone() }))?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::pushBlock(l_forstatement.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_forstatement = dumpEqs(l_forstatement.clone(), i_e_body.clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            l_forstatement = Tpl::popBlock(l_forstatement.clone())?;
            l_forstatement = Tpl::writeTok(l_forstatement.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: FOR_EQUATION\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_forstatement.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ALIAS { aliasOf: i_e_aliasOf, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: ALIAS\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("alias of ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_e_aliasOf.clone())).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_ENTWINED_ASSIGN { single_calls: i_e_single__calls, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndex(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("type: ENTWINED_ASSIGN\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = dumpEqs(txt.clone(), i_e_single__calls.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unknown equation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_67(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_66(txt.clone(), i_eq.clone())?;
            txt = lm_67(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpEqsWork(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = lm_67(txt.clone(), a_eqs.clone())?;
    Ok(out_txt)
}

fn lm_69(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::OMSIFunction>>>, mut in_a_varsBuffer: Tpl::Text, mut in_a_columnBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varsBuffer: Tpl::Text;
    let mut out_a_columnBuffer: Tpl::Text;
    (out_txt, out_a_varsBuffer, out_a_columnBuffer) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varsBuffer.clone(), in_a_columnBuffer.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varsBuffer, a_columnBuffer) => {
            (txt.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_column, tail: rest }, a_varsBuffer, a_columnBuffer) => {
            let mut txt = (*txt).clone();
            let mut a_varsBuffer = (*a_varsBuffer).clone();
            let mut a_columnBuffer = (*a_columnBuffer).clone();
            (txt, a_columnBuffer, a_varsBuffer) = dumpAlgSystemColumn(txt.clone(), i_column.clone(), a_columnBuffer.clone(), a_varsBuffer.clone())?;
            (txt, a_varsBuffer, a_columnBuffer) = lm_69(txt.clone(), rest.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())?;
            (txt.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varsBuffer, out_a_columnBuffer))
}

fn fun_70(mut in_txt: Tpl::Text, mut in_a_derivativeMatrix: Option<Arc<SimCode::DerivativeMatrix>>, mut in_a_varsBuffer: Tpl::Text, mut in_a_columnBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varsBuffer: Tpl::Text;
    let mut out_a_columnBuffer: Tpl::Text;
    (out_txt, out_a_varsBuffer, out_a_columnBuffer) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_derivativeMatrix.clone(), in_a_varsBuffer.clone(), in_a_columnBuffer.clone())) {
        (txt, Some(Deref @ SimCode::DerivativeMatrix { columns: i_matrix_columns, .. }), a_varsBuffer, a_columnBuffer) => {
            let mut l_0__: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varsBuffer = (*a_varsBuffer).clone();
            let mut a_columnBuffer = (*a_columnBuffer).clone();
            (l_0__, a_varsBuffer, a_columnBuffer) = lm_69(Tpl::emptyTxt.clone(), i_matrix_columns.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("iteration vars: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_varsBuffer.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_columnBuffer.clone())?;
            (txt.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())
        },
        (txt, _, a_varsBuffer, a_columnBuffer) => {
            (txt.clone(), a_varsBuffer.clone(), a_columnBuffer.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varsBuffer, out_a_columnBuffer))
}

pub fn dumpAlgSystemOps(mut txt: Tpl::Text, mut a_derivativeMatrix: Option<Arc<SimCode::DerivativeMatrix>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_columnBuffer: Tpl::Text;
    let mut l_varsBuffer: Tpl::Text;
    l_varsBuffer = Tpl::emptyTxt.clone();
    l_columnBuffer = Tpl::emptyTxt.clone();
    (out_txt, l_varsBuffer, l_columnBuffer) = fun_70(txt.clone(), a_derivativeMatrix.clone(), l_varsBuffer.clone(), l_columnBuffer.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_72(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { name: i_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_name.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_72(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_72(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_73(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>, mut in_a_columnBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_columnBuffer: Tpl::Text;
    (out_txt, out_a_columnBuffer) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_columnBuffer.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_columnBuffer) => {
            (txt.clone(), a_columnBuffer.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SimCode::SimEqSystem::SES_SIMPLE_ASSIGN { exp: i_equation_exp, cref: i_equation_cref, .. }, tail: rest }, a_columnBuffer) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_columnBuffer = (*a_columnBuffer).clone();
            a_columnBuffer = ExpressionDumpTpl::dumpCref(a_columnBuffer.clone(), i_equation_cref.clone())?;
            a_columnBuffer = Tpl::writeTok(a_columnBuffer.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_equation_exp.clone(), (literal!("\"")).clone())?;
            a_columnBuffer = CodegenUtil::escapeCComments(a_columnBuffer.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            a_columnBuffer = Tpl::writeTok(a_columnBuffer.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt, a_columnBuffer) = lm_73(txt.clone(), rest.clone(), a_columnBuffer.clone())?;
            (txt.clone(), a_columnBuffer.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_columnBuffer) => {
            let mut txt = (*txt).clone();
            let mut a_columnBuffer = (*a_columnBuffer).clone();
            (txt, a_columnBuffer) = lm_73(txt.clone(), rest.clone(), a_columnBuffer.clone())?;
            (txt.clone(), a_columnBuffer.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_columnBuffer))
}

pub fn dumpAlgSystemColumn(mut in_txt: Tpl::Text, mut in_a_column: Arc<SimCode::OMSIFunction>, mut in_a_columnBuffer: Tpl::Text, mut in_a_varsBuffer: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_columnBuffer: Tpl::Text;
    let mut out_a_varsBuffer: Tpl::Text;
    (out_txt, out_a_columnBuffer, out_a_varsBuffer) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_column.clone(), in_a_columnBuffer.clone(), in_a_varsBuffer.clone())) {
        (txt, Deref @ SimCode::OMSIFunction { equations: i_equations, inputVars: i_inputVars, .. }, a_columnBuffer, a_varsBuffer) => {
            let mut l_0__: Tpl::Text;
            let mut a_columnBuffer = (*a_columnBuffer).clone();
            let mut a_varsBuffer = (*a_varsBuffer).clone();
            a_varsBuffer = Tpl::pushIter(a_varsBuffer.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            a_varsBuffer = lm_72(a_varsBuffer.clone(), i_inputVars.clone())?;
            a_varsBuffer = Tpl::popIter(a_varsBuffer.clone())?;
            (l_0__, a_columnBuffer) = lm_73(Tpl::emptyTxt.clone(), i_equations.clone(), a_columnBuffer.clone())?;
            (txt.clone(), a_columnBuffer.clone(), a_varsBuffer.clone())
        },
        (txt, _, a_columnBuffer, a_varsBuffer) => {
            (txt.clone(), a_columnBuffer.clone(), a_varsBuffer.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_columnBuffer, out_a_varsBuffer))
}

pub fn dumpWhenOps(mut in_txt: Tpl::Text, mut in_a_whenOps: Arc<metamodelica::List<BackendDAE::WhenOperator>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_whenOps.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: i_e_right, left: Deref @ DAE::Exp::CREF { componentRef: i_left_componentRef, .. }, .. }, tail: i_rest }) => {
            let mut txt_1: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_left_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_right.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSIGN { right: i_e_right, left: i_e_left, .. }, tail: i_rest }) => {
            let mut txt_2: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = ExpressionDumpTpl::dumpExp(txt.clone(), i_e_left.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_right.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::REINIT { value: i_e_value, stateVar: i_e_stateVar, .. }, tail: i_rest }) => {
            let mut txt_3: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(")).clone() }))?;
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_e_stateVar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",  ")).clone() }))?;
            txt_3 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_value.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::ASSERT { level: i_e_level, message: i_e_message, condition: i_e_condition, .. }, tail: i_rest }) => {
            let mut txt_6: Tpl::Text;
            let mut txt_5: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert(")).clone() }))?;
            txt_4 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_condition.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_4.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt_5 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_message.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_5.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt_6 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_level.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_6.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::TERMINATE { message: i_e_message, .. }, tail: i_rest }) => {
            let mut txt_7: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("terminate(")).clone() }))?;
            txt_7 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_message.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_7.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")%>);\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::WhenOperator::NORETCALL { exp: i_e_exp, .. }, tail: i_rest }) => {
            let mut txt_8: Tpl::Text;
            let mut l_restbody: Tpl::Text;
            let mut txt = (*txt).clone();
            l_restbody = dumpWhenOps(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("noReturnCall(")).clone() }))?;
            txt_8 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_8.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")%>);\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_restbody.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("CodegenUtilSimulation.tpl")).clone(), 443, 14), (literal!("dumpEqs: Unknown equation")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_76(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_eqs.clone())) {
        (txt, Deref @ "none", a_eqs) => {
            let mut txt = (*txt).clone();
            txt = dumpEqsAlternativeTearingWork(txt.clone(), a_eqs.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEqsAlternativeTearing(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: ArcStr = arcstr::literal!("");
    ret_0 = (Flags::getConfigString(Flags::OBFUSCATE.clone())?).clone();
    out_txt = fun_76(txt.clone(), (ret_0.clone()).clone(), a_eqs.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_78(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { name: i_cr, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<var>")).clone() }))?;
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_cr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</var>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_78(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_78(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_79(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_79(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ SimCode::SimEqSystem::SES_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_0.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_FOR_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_1 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_1.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SimCode::SimEqSystem::SES_GENERIC_RESIDUAL { exp: i_e_exp, .. }) => {
            let mut txt_2: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<residual>")).clone() }))?;
            txt_2 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e_exp.clone(), (literal!("\"")).clone())?;
            txt = CodegenUtil::escapeCComments(txt.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</residual>")).clone() }))?;
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
fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, i32, Arc<SimCode::SimEqSystem>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_i1, i_i2, i_eq), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cell row=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" col=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_i2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_80(txt.clone(), i_eq.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cell>")).clone() }))?;
            txt = lm_81(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_82(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cr, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = ExpressionDumpTpl::dumpCref(txt.clone(), i_cr.clone())?;
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
fn lm_83(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = equationIndex(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_83(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_eq: Arc<SimCode::SimEqSystem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_LINEAR { alternativeTearing: Some(Deref @ SimCode::LinearSystem { simJac: i_at_simJac, beqs: i_at_beqs, vars: i_at_vars, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndexAlternativeTearing(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: LINEAR\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_78(txt.clone(), i_at_vars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<row>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_79(txt.clone(), i_at_beqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</row>\n")).clone(), (literal!("<matrix>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = lm_81(txt.clone(), i_at_simJac.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</matrix>\n")).clone(), (literal!("\n")).clone(), (literal!("This is the alternative tearing set with casual solvability rules.\n")).clone(), (literal!("If it fails, this function will call the strict tearing set.")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ SimCode::SimEqSystem::SES_NONLINEAR { alternativeTearing: Some(Deref @ SimCode::NonlinearSystem { eqs: i_at_eqs, crefs: i_at_crefs, indexNonLinearSystem: i_at_indexNonLinearSystem, .. }), .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equation index: ")).clone() }))?;
            txt = equationIndexAlternativeTearing(txt.clone(), i_e.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("indexNonlinear: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_at_indexNonLinearSystem.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("type: NONLINEAR\n")).clone(), (literal!("\n")).clone(), (literal!("vars: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_82(txt.clone(), i_at_crefs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("eqns: {")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_83(txt.clone(), i_at_eqs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("This is the alternative tearing set with casual solvability rules.\n")).clone(), (literal!("If it fails, this function will call the strict tearing set.")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unknown equation")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_85(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_84(txt.clone(), i_eq.clone())?;
            txt = lm_85(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpEqsAlternativeTearingWork(mut txt: Tpl::Text, mut a_eqs: Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = lm_85(txt.clone(), a_eqs.clone())?;
    Ok(out_txt)
}

