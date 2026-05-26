// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::CodegenUtil;
use crate::SimCode;
use crate::SimCodeVar;
use openmodelica_frontend::Expression;
use openmodelica_frontend::ExpressionDump;
use openmodelica_frontend_dump::ExpressionDumpTpl;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Util;

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_47(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: SimCodeVar::SimVar { name: i_v_name, .. }, tail: rest }) => {
            let mut x_index0: i32 = 0;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            x_index0 = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_index0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": ")).clone() }))?;
            txt_0 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_v_name.clone())?;
            ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_0.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_47(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_47(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpVarsShort(mut txt: Tpl::Text, mut a_vars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_varsString: Tpl::Text;
    l_varsString = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_varsString = lm_47(l_varsString.clone(), a_vars.clone())?;
    l_varsString = Tpl::popIter(l_varsString.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_varsString.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    Ok(out_txt)
}

pub fn dumpAlias(mut in_txt: Tpl::Text, mut in_a_alias: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_alias.clone()) {
        (mut txt, SimCodeVar::AliasVariable::ALIAS { varName: ref i_varName }) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt_0: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<alias>")).clone() }))?;
            txt_0 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_0.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</alias>")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCodeVar::AliasVariable::NEGATEDALIAS { varName: ref i_varName }) => {
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut txt_2: Tpl::Text;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<alias negated=\"true\">")).clone() }))?;
            txt_2 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_varName.clone())?;
            ret_3 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_2.clone())?).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</alias>")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn printExpStrEscaped(mut txt: Tpl::Text, mut a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: ArcStr = arcstr::literal!("");
    let mut txt_0: Tpl::Text;
    txt_0 = ExpressionDumpTpl::dumpExp(Tpl::emptyTxt.clone(), a_exp.clone(), (literal!("\"")).clone())?;
    ret_1 = (Util::escapeModelicaStringToXmlString((Tpl::textString(txt_0.clone())?).clone())?).clone();
    out_txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
    Ok(out_txt)
}

