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
use crate::DAEDumpTpl;
use crate::Dump;
use crate::ExpressionBasics;
use crate::TypesDump;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;

fn fun_13(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_index.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_index) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_14(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_15(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_attr_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_attr_ty.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_attr_ty) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_attr_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_16(mut in_txt: Tpl::Text, mut in_a_scalar: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_scalar.clone(), in_a_ty.clone())) {
        (txt, false, a_ty) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* non-scalar ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* scalar ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_17(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty.clone(), in_a_scalar.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_ty, a_scalar) => {
            let mut txt = (*txt).clone();
            txt = fun_16(txt.clone(), a_scalar.clone(), a_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_18(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>, mut in_a_scalar: bool, mut in_a_stringDelimiter: ArcStr, mut in_a_array: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty.clone(), in_a_scalar.clone(), in_a_stringDelimiter.clone(), in_a_array.clone())) {
        (txt, false, a_ty, a_scalar, a_stringDelimiter, a_array) => {
            let mut ret_1: bool = false;
            let mut l_expl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl = dumpExpList(Tpl::emptyTxt.clone(), a_array.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_1 = Config::typeinfo()?;
            txt = fun_17(txt.clone(), ret_1.clone(), a_ty.clone(), a_scalar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fill(0,0)")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_19(mut in_txt: Tpl::Text, mut in_a_scalar: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_scalar.clone(), in_a_ty.clone())) {
        (txt, false, a_ty) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* non-scalar ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* scalar ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_20(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty.clone(), in_a_scalar.clone())) {
        (txt, false, _, _) => {
            txt.clone()
        },
        (txt, _, a_ty, a_scalar) => {
            let mut txt = (*txt).clone();
            txt = fun_19(txt.clone(), a_scalar.clone(), a_ty.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_21(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_row, tail: rest }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpExpList(txt.clone(), i_row.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_21(txt.clone(), rest.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_22(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* matrix ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_23(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_e: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_step.clone(), in_a_e.clone())) {
        (txt, Some(i_step), a_e) => {
            let mut txt = (*txt).clone();
            txt = dumpOperand(txt.clone(), i_step.clone(), a_e.clone(), false)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_24(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_25(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_26(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*ASUB*/")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_27(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_28(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_29(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_30(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_31(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*RSUB: ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_32(mut in_txt: Tpl::Text, mut in_a_sz: Option<Arc<DAE::Exp>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sz.clone(), in_a_stringDelimiter.clone())) {
        (txt, Some(i_dim), a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_dim.clone(), (a_stringDelimiter.clone()).clone())?;
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
fn lm_33(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpReductionIterator(txt.clone(), i_it.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_33(txt.clone(), rest.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_34(mut in_txt: Tpl::Text, mut in_a_ri_iterType: Absyn::ReductionIterType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_ri_iterType.clone()) {
        (mut txt, Absyn::ReductionIterType::THREAD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threaded ")).clone() }))?;
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
fn lm_35(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_c, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpMatchCase(txt.clone(), i_c.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_35(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_36(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_index: i32, mut in_a_stringDelimiter: ArcStr, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_index.clone(), in_a_stringDelimiter.clone(), in_a_exp.clone())) {
        (txt, false, _, a_stringDelimiter, a_exp) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), a_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_index, a_stringDelimiter, a_exp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* Shared literal ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt = dumpExp(txt.clone(), a_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_37(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*pattern*/")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpExp(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_stringDelimiter.clone())) {
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
        (txt, Deref @ DAE::Exp::SCONST { string: i_string }, a_stringDelimiter) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_str: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = (System::escapedString((i_string.clone()).clone(), false)).clone();
            l_str = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: i_bool }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_bool.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CLKCONST { clk: i_clk }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpClockKind(txt.clone(), i_clk.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: i_name, index: i_index }, _) => {
            let mut ret_2: bool = false;
            let mut txt = (*txt).clone();
            ret_2 = Config::typeinfo()?;
            txt = fun_13(txt.clone(), ret_2.clone(), i_index.clone())?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, ty: i_ty }, _) => {
            let mut ret_3: bool = false;
            let mut txt = (*txt).clone();
            ret_3 = Config::typeinfo()?;
            txt = fun_14(txt.clone(), ret_3.clone(), i_ty.clone())?;
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::BINARY { operator: i_operator, exp2: i_exp2, exp1: i_exp1 }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpBinOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::UNARY { operator: i_operator, exp: i_exp }, _) => {
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpUnaryOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::LBINARY { operator: i_operator, exp2: i_exp2, exp1: i_exp1 }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpLogicalBinOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::LUNARY { operator: i_operator, exp: i_exp }, _) => {
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpLogicalUnaryOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::RELATION { operator: i_operator, exp2: i_exp2, exp1: i_exp1, .. }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpRelationOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::IFEXP { expElse: i_expElse, expThen: i_expThen, expCond: i_expCond }, a_stringDelimiter) => {
            let mut l_else__str: Tpl::Text;
            let mut l_then__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), i_expCond.clone(), (a_stringDelimiter.clone()).clone())?;
            l_then__str = dumpExp(Tpl::emptyTxt.clone(), i_expThen.clone(), (a_stringDelimiter.clone()).clone())?;
            l_else__str = dumpExp(Tpl::emptyTxt.clone(), i_expElse.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_then__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" else ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: i_expLst, path: i_path, attr: Deref @ DAE::CallAttributes { ty: i_attr_ty, builtin: true, .. } }, a_stringDelimiter) => {
            let mut ret_13: bool = false;
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_13 = Config::typeinfo()?;
            txt = fun_15(txt.clone(), ret_13.clone(), i_attr_ty.clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: i_expLst, path: i_path, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RECORD { exps: i_exps, path: i_path, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_exps.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::PARTEVALFUNCTION { expList: i_expList, path: i_path, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { ty: i_ty, scalar: i_scalar, array: i_array @ Deref @ metamodelica::List::Nil }, a_stringDelimiter) => {
            let mut ret_14: bool = false;
            let mut txt = (*txt).clone();
            ret_14 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_18(txt.clone(), ret_14.clone(), i_ty.clone(), i_scalar.clone(), (a_stringDelimiter.clone()).clone(), i_array.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { ty: i_ty, scalar: i_scalar, array: i_array }, a_stringDelimiter) => {
            let mut ret_16: bool = false;
            let mut l_expl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl = dumpExpList(Tpl::emptyTxt.clone(), i_array.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_16 = Config::typeinfo()?;
            txt = fun_20(txt.clone(), ret_16.clone(), i_ty.clone(), i_scalar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATRIX { ty: i_ty, matrix: i_matrix, .. }, a_stringDelimiter) => {
            let mut ret_18: bool = false;
            let mut l_mat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_mat__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_mat__str = lm_21(l_mat__str.clone(), i_matrix.clone(), (a_stringDelimiter.clone()).clone())?;
            l_mat__str = Tpl::popIter(l_mat__str.clone())?;
            ret_18 = Config::typeinfo()?;
            txt = fun_22(txt.clone(), ret_18.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}}")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::RANGE { stop: i_stop, step: i_step, start: i_start, .. }, _) => {
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false)?;
            l_step__str = fun_23(Tpl::emptyTxt.clone(), i_step.clone(), i_e.clone())?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_step__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: i_PR }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_PR.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CAST { ty: i_ty, exp: i_exp }, a_stringDelimiter) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/*")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*/(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ASUB { sub: i_sub, exp: i_exp }, a_stringDelimiter) => {
            let mut ret_28: bool = false;
            let mut l_sub__str: Tpl::Text;
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_24(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_25(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_sub.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rparen.clone())?;
            ret_28 = Config::typeinfo()?;
            txt = fun_26(txt.clone(), ret_28.clone())?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TSUB { ix: i_ix, exp: i_exp, .. }, a_stringDelimiter) => {
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_27(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_28(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rparen.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_ix.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RSUB { fieldName: i_fieldName, ty: i_ty, exp: i_exp, .. }, a_stringDelimiter) => {
            let mut ret_29: bool = false;
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_29(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_30(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            ret_29 = Config::typeinfo()?;
            txt = fun_31(txt.clone(), ret_29.clone(), i_ty.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rparen.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fieldName.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SIZE { sz: i_sz, exp: i_exp }, a_stringDelimiter) => {
            let mut l_dim__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_dim__str = fun_32(Tpl::emptyTxt.clone(), i_sz.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("size(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CODE { code: i_code, .. }, _) => {
            let mut ret_32: ArcStr = arcstr::literal!("");
            let mut l_code__str: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_32 = (Dump::printCodeStr(i_code.clone())?).clone();
            l_code__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_32.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$Code(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_code__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::EMPTY { tyStr: i_tyStr, scope: i_scope, name: i_name_1, .. }, _) => {
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = dumpCref(Tpl::emptyTxt.clone(), i_name_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<EMPTY(scope: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_scope.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", name: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ty: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_tyStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::REDUCTION { iterators: i_iterators, expr: i_expr, reductionInfo: Deref @ DAE::ReductionInfo { iterType: i_ri_iterType, path: i_ri_path, .. } }, a_stringDelimiter) => {
            let mut l_iter__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_ri_path.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_expr.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_iter__str = lm_33(l_iter__str.clone(), i_iterators.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::popIter(l_iter__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" for ")).clone() }))?;
            txt = fun_34(txt.clone(), i_ri_iterType.clone())?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LIST { valList: i_valList }, a_stringDelimiter) => {
            let mut l_expl__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl__str = dumpExpList(Tpl::emptyTxt.clone(), i_valList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("List(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CONS { cdr: i_cdr, car: i_car }, a_stringDelimiter) => {
            let mut l_cdr__str: Tpl::Text;
            let mut l_car__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_car__str = dumpExp(Tpl::emptyTxt.clone(), i_car.clone(), (a_stringDelimiter.clone()).clone())?;
            l_cdr__str = dumpExp(Tpl::emptyTxt.clone(), i_cdr.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("listCons(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_car__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cdr__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_TUPLE { listExp: i_listExp }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_listExp.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tuple(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: Some(i_exp) }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::METARECORDCALL { args: i_args, path: i_path, .. }, a_stringDelimiter) => {
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_args__str = dumpExpList(Tpl::emptyTxt.clone(), i_args.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATCHEXPRESSION { cases: i_cases, inputs: i_inputs, matchType: i_matchType, .. }, a_stringDelimiter) => {
            let mut l_case__str: Tpl::Text;
            let mut l_inputs__str: Tpl::Text;
            let mut l_match__ty: Tpl::Text;
            let mut txt = (*txt).clone();
            l_match__ty = dumpMatchType(Tpl::emptyTxt.clone(), i_matchType.clone())?;
            l_inputs__str = dumpExpList(Tpl::emptyTxt.clone(), i_inputs.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            l_case__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_case__str = lm_35(l_case__str.clone(), i_cases.clone())?;
            l_case__str = Tpl::popIter(l_case__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_match__ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_inputs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_case__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_match__ty.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BOX { exp: i_exp }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::UNBOX { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { index: i_index, exp: i_exp }, a_stringDelimiter) => {
            let mut ret_42: bool = false;
            let mut txt = (*txt).clone();
            ret_42 = Config::typeinfo()?;
            txt = fun_36(txt.clone(), ret_42.clone(), i_index.clone(), (a_stringDelimiter.clone()).clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::PATTERN { pattern: i_pattern }, _) => {
            let mut ret_43: bool = false;
            let mut txt = (*txt).clone();
            ret_43 = Config::typeinfo()?;
            txt = fun_37(txt.clone(), ret_43.clone())?;
            txt = dumpPattern(txt.clone(), i_pattern.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpExp: Unknown expression.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn parenthesizeSubExp(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { ty: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATRIX { ty: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: _ }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CAST { ty: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SIZE { exp: _, .. }) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::REDUCTION { reductionInfo: _, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("y")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_40(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_40(txt.clone(), rest.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpExpList(mut txt: Tpl::Text, mut a_expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_stringDelimiter: ArcStr, mut a_expDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (a_expDelimiter.clone()).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_40(out_txt.clone(), a_expl.clone(), (a_stringDelimiter.clone()).clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_42(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpExpCrefs(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_42(txt.clone(), rest.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpExpListCrefs(mut txt: Tpl::Text, mut a_expl: Arc<metamodelica::List<Arc<DAE::Exp>>>, mut a_stringDelimiter: ArcStr, mut a_expDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (a_expDelimiter.clone()).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_42(out_txt.clone(), a_expl.clone(), (a_stringDelimiter.clone()).clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpClockKind(mut in_txt: Tpl::Text, mut in_a_clk: Arc<DAE::ClockKind>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_clk.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ DAE::ClockKind::INFERRED_CLOCK, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock()")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::RATIONAL_CLOCK { resolution: i_resolution, intervalCounter: i_intervalCounter }, a_stringDelimiter) => {
            let mut l_re__str: Tpl::Text;
            let mut l_ic__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ic__str = dumpExp(Tpl::emptyTxt.clone(), i_intervalCounter.clone(), (a_stringDelimiter.clone()).clone())?;
            l_re__str = dumpExp(Tpl::emptyTxt.clone(), i_resolution.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ic__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_re__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::REAL_CLOCK { interval: i_interval }, a_stringDelimiter) => {
            let mut l_interval__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_interval__str = dumpExp(Tpl::emptyTxt.clone(), i_interval.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_interval__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::EVENT_CLOCK { startInterval: i_startInterval, condition: i_condition }, a_stringDelimiter) => {
            let mut l_si__str: Tpl::Text;
            let mut l_condition__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_condition__str = dumpExp(Tpl::emptyTxt.clone(), i_condition.clone(), (a_stringDelimiter.clone()).clone())?;
            l_si__str = dumpExp(Tpl::emptyTxt.clone(), i_startInterval.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_condition__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_si__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::SOLVER_CLOCK { solverMethod: i_solverMethod, c: i_c }, a_stringDelimiter) => {
            let mut l_sm__str: Tpl::Text;
            let mut l_clk__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_clk__str = dumpExp(Tpl::emptyTxt.clone(), i_c.clone(), (a_stringDelimiter.clone()).clone())?;
            l_sm__str = dumpExp(Tpl::emptyTxt.clone(), i_solverMethod.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Clock(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_clk__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sm__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_45(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_cref__str: Tpl::Text, mut in_a_sub__str: Tpl::Text, mut in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_cref__str.clone(), in_a_sub__str.clone(), in_a_ident.clone()) {
        (mut txt, false, mut a_cref__str, mut a_sub__str, mut a_ident) => {
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_cref__str.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_cref__str, mut a_sub__str, mut a_ident) => {
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), a_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_cref__str.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpCref(mut in_txt: Tpl::Text, mut in_a_cref: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cref.clone())) {
        (txt, Deref @ DAE::ComponentRef::CREF_IDENT { ident: i_ident, subscriptLst: i_subscriptLst, .. }) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::CREF_QUAL { ident: i_ident, componentRef: i_componentRef, subscriptLst: i_subscriptLst, .. }) => {
            let mut ret_2: bool = false;
            let mut l_cref__str: Tpl::Text;
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_subscriptLst.clone())?;
            l_cref__str = dumpCref(Tpl::emptyTxt.clone(), i_componentRef.clone())?;
            ret_2 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_45(txt.clone(), ret_2.clone(), l_cref__str.clone(), l_sub__str.clone(), (i_ident.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::WILD) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { instant: i_instant, componentRef: i_componentRef }) => {
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_instant.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpCref: unknown cref")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_47(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_sub, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpSubscript(txt.clone(), i_sub.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_47(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_48(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_sub, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpSubscript(txt.clone(), i_sub.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_48(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_49(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_subscripts.clone())) {
        (txt, false, a_subscripts) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_sub__str = lm_47(l_sub__str.clone(), a_subscripts.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_subscripts) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_sub__str = lm_48(l_sub__str.clone(), a_subscripts.clone())?;
            l_sub__str = Tpl::popIter(l_sub__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSubscripts(mut in_txt: Tpl::Text, mut in_a_subscripts: Arc<metamodelica::List<Arc<DAE::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_subscripts) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_49(txt.clone(), ret_0.clone(), i_subscripts.clone())?;
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
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::INDEX { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Subscript::WHOLE_NONEXP { exp: i_exp }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpReductionIterator(mut in_txt: Tpl::Text, mut in_a_iterator: Arc<DAE::ReductionIterator>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iterator.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ DAE::ReductionIterator { id: i_id, exp: i_exp, guardExp: None, .. }, a_stringDelimiter) => {
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ReductionIterator { id: i_id, exp: i_exp, guardExp: Some(i_gexp), .. }, a_stringDelimiter) => {
            let mut l_guard__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_guard__str = dumpExp(Tpl::emptyTxt.clone(), i_gexp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" guard ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_guard__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_op__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_op__str.clone()) {
        (mut txt, false, mut a_op__str) => {
            txt = Tpl::writeText(txt.clone(), a_op__str.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_op__str) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpOperand(mut txt: Tpl::Text, mut a_operand: Arc<DAE::Exp>, mut a_operation: Arc<DAE::Exp>, mut a_lhs: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: bool = false;
    let mut l_op__str: Tpl::Text;
    l_op__str = dumpExp(Tpl::emptyTxt.clone(), a_operand.clone(), (literal!("\"")).clone())?;
    ret_1 = ExpressionBasics::shouldParenthesize(a_operand.clone(), a_operation.clone(), a_lhs.clone())?;
    out_txt = fun_53(txt.clone(), ret_1.clone(), l_op__str.clone())?;
    Ok(out_txt)
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, DAE::Operator::ADD { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("./")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR2 { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("./")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpBinOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_56(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, DAE::Operator::ADD { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+ /* ADD_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("- /* SUB_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".* /* MUL_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("./ /* DIV_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^ /* POW_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR2 { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^ /* POW_ARR2 */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* /* MUL_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".+ /* ADD_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".- /* SUB_SCA_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^ /* POW_SCA_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^ /* POW_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* /* MUL_SCA_PRO */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("* /* MUL_MAT_PRO */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/ /* DIV_SCA_ARR */")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/ /* DIV_ARR_SCA */")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpBinOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_op.clone()) {
        (mut txt, false, mut a_op) => {
            txt = fun_55(txt.clone(), a_op.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_op) => {
            txt = fun_56(txt.clone(), a_op.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpBinOp(mut txt: Tpl::Text, mut a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: bool = false;
    ret_0 = Config::typeinfo()?;
    out_txt = fun_57(txt.clone(), ret_0.clone(), a_op.clone())?;
    Ok(out_txt)
}

pub fn dumpUnaryOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, DAE::Operator::UMINUS { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpUnaryOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpLogicalBinOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, DAE::Operator::AND { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("and")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::OR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("or")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpLogicalBinOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpLogicalUnaryOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, DAE::Operator::NOT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("not")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpLogicalUnaryOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpRelationOp(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, DAE::Operator::LESS { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<=")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">=")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::EQUAL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("==")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NEQUAL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<>")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::USERDEFINED { fqName: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("USERDEFINED")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpRelationOp: Unknown operator.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_63(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpFuncArg(txt.clone(), i_arg.clone())?;
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
fn lm_64(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpType(txt.clone(), i_ty.clone())?;
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
fn lm_65(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpType(txt.clone(), i_ty.clone())?;
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
pub fn dumpType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
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
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Bool")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_ty, dims: i_dims }) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_dim__str = dumpDimensions(Tpl::emptyTxt.clone(), i_dims.clone())?;
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_dim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: i_complexClassType, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpClassState(txt.clone(), i_complexClassType.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: i_complexClassType, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpClassState(txt.clone(), i_complexClassType.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_FUNCTION { funcResultType: i_funcResultType, funcArg: i_funcArg, .. }) => {
            let mut l_ret__str: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arg__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_arg__str = lm_63(l_arg__str.clone(), i_funcArg.clone())?;
            l_arg__str = Tpl::popIter(l_arg__str.clone())?;
            l_ret__str = dumpType(Tpl::emptyTxt.clone(), i_funcResultType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<function>(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") => ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ret__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { functionType: i_functionType }) => {
            let mut txt = (*txt).clone();
            txt = dumpType(txt.clone(), i_functionType.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: i_functionType, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpType(txt.clone(), i_functionType.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_TUPLE { types: i_types, .. }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_ty__str = lm_64(l_ty__str.clone(), i_types.clone())?;
            l_ty__str = Tpl::popIter(l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_CODE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_CODE#")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: i_ty }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: i_types }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_ty__str = lm_65(l_ty__str.clone(), i_types.clone())?;
            l_ty__str = Tpl::popIter(l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tuple<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: i_ty }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Option<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METARECORD { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: i_ty }) => {
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpType(Tpl::emptyTxt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("array<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: i_ty }) => {
            let mut txt = (*txt).clone();
            txt = dumpType(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("polymorphic<")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: i_ty }) => {
            let mut txt = (*txt).clone();
            txt = dumpType(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_UNKNOWN) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_UNKNOWN#")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ANYTYPE { anyClassType: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Any")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_NORETCALL) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#T_NORETCALL#")).clone() }))?;
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
        (txt, Deref @ DAE::FuncArg { name: i_arg_name, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_arg_name.clone()).clone())?;
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
fn lm_68(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dim, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpDimension(txt.clone(), i_dim.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_68(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpDimensions(mut txt: Tpl::Text, mut a_dims: Arc<metamodelica::List<Arc<DAE::Dimension>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_68(out_txt.clone(), a_dims.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
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
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
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

pub fn dumpClassState(mut txt: Tpl::Text, mut a_state: ClassInf::State) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: Arc<Absyn::Path>;
    ret_0 = ClassInfUtil::getStateName(a_state.clone());
    out_txt = AbsynDumpTpl::dumpPath(txt.clone(), ret_0.clone())?;
    Ok(out_txt)
}

pub fn dumpMatchType(mut in_txt: Tpl::Text, mut in_a_ty: DAE::MatchType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, DAE::MatchType::MATCHCONTINUE) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("matchcontinue")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::MatchType::MATCH { switch: None }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match")).clone() }))?;
            txt.clone()
        },
        (txt, DAE::MatchType::MATCH { switch: Some(_) }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match /* switch */")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpMatchCase(mut in_txt: Tpl::Text, mut in_a_mcase: Arc<DAE::MatchCase>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_mcase.clone())) {
        (txt, Deref @ DAE::MatchCase { patterns: i_patterns, result: Some(i_result), body: Deref @ metamodelica::List::Nil, .. }) => {
            let mut l_res__str: Tpl::Text;
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            l_res__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_res__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::MatchCase { patterns: i_patterns, result: None, body: Deref @ metamodelica::List::Nil, .. }) => {
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") then fail();")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::MatchCase { body: i_body, patterns: i_patterns, result: Some(i_result), .. }) => {
            let mut l_body__str: Tpl::Text;
            let mut l_res__str: Tpl::Text;
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            l_res__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone(), (literal!("\"")).clone())?;
            l_body__str = DAEDumpTpl::dumpStatements(Tpl::emptyTxt.clone(), i_body.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("  algorithm\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  then\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_res__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::MatchCase { body: i_body, patterns: i_patterns, .. }) => {
            let mut l_body__str: Tpl::Text;
            let mut l_pat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            l_body__str = DAEDumpTpl::dumpStatements(Tpl::emptyTxt.clone(), i_body.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("  algorithm\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  then\n")).clone(), (literal!("    fail();")).clone()], lastHasNewLine: false }))?;
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
fn lm_74(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_pat, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_74(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpPatterns(mut txt: Tpl::Text, mut a_patterns: Arc<metamodelica::List<Arc<DAE::Pattern>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_74(out_txt.clone(), a_patterns.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_76(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_pat, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpNamedPattern(txt.clone(), i_pat.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_76(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpPattern(mut in_txt: Tpl::Text, mut in_a_pattern: Arc<DAE::Pattern>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_pattern.clone())) {
        (txt, Deref @ DAE::Pattern::PAT_WILD) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_AS { id: i_id, pat: Deref @ DAE::Pattern::PAT_WILD, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { id: i_id, pat: Deref @ DAE::Pattern::PAT_WILD }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_SOME { pat: i_pat }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_META_TUPLE { patterns: i_patterns }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = dumpPatterns(txt.clone(), i_patterns.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_CALL_TUPLE { patterns: i_patterns }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = dumpPatterns(txt.clone(), i_patterns.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_CALL { patterns: i_patterns, name: i_name, .. }) => {
            let mut l_pat__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_pat__str = dumpPatterns(Tpl::emptyTxt.clone(), i_patterns.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_CALL_NAMED { patterns: i_patterns_1, name: i_name }) => {
            let mut l_pat__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_pat__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_pat__str = lm_76(l_pat__str.clone(), i_patterns_1.clone())?;
            l_pat__str = Tpl::popIter(l_pat__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_CONS { tail: i_tail, head: i_head }) => {
            let mut txt = (*txt).clone();
            txt = dumpPattern(txt.clone(), i_head.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("::")).clone() }))?;
            txt = dumpPattern(txt.clone(), i_tail.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_CONSTANT { exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone(), (literal!("\"")).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_AS { pat: i_pat, id: i_id, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" as ")).clone() }))?;
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Pattern::PAT_AS_FUNC_PTR { pat: i_pat, id: i_id }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" as ")).clone() }))?;
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*PATTERN*")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpNamedPattern(mut in_txt: Tpl::Text, mut in_a_pattern: (Arc<DAE::Pattern>, ArcStr, Arc<DAE::Type>)) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_pattern.clone())) {
        (txt, (i_pat, i_id, _)) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpPattern(txt.clone(), i_pat.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_scalar.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* non-scalar */ ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* scalar */ ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_scalar: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_scalar.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_scalar) => {
            txt = fun_79(txt.clone(), a_scalar.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_row, tail: rest }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpExpList(txt.clone(), i_row.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_81(txt.clone(), rest.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_ty.clone())) {
        (txt, false, _) => {
            txt.clone()
        },
        (txt, _, a_ty) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* matrix ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(a_ty.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" */ ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_step: Option<Arc<DAE::Exp>>, mut in_a_e: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_step.clone(), in_a_e.clone())) {
        (txt, Some(i_step), a_e) => {
            let mut txt = (*txt).clone();
            txt = dumpOperand(txt.clone(), i_step.clone(), a_e.clone(), false)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_85(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_87(mut in_txt: Tpl::Text, mut in_a_needs__paren: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_needs__paren.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_a_sz: Option<Arc<DAE::Exp>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sz.clone(), in_a_stringDelimiter.clone())) {
        (txt, Some(i_dim), a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_dim.clone(), (a_stringDelimiter.clone()).clone())?;
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
fn lm_89(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::ReductionIterator>>>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpReductionIterator(txt.clone(), i_it.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_89(txt.clone(), rest.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_90(mut in_txt: Tpl::Text, mut in_a_ri_iterType: Absyn::ReductionIterType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_ri_iterType.clone()) {
        (mut txt, Absyn::ReductionIterType::THREAD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threaded ")).clone() }))?;
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
fn lm_91(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::MatchCase>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_c, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpMatchCase(txt.clone(), i_c.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_91(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpExpCrefs(mut in_txt: Tpl::Text, mut in_a_exp: Arc<DAE::Exp>, mut in_a_stringDelimiter: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_stringDelimiter.clone())) {
        (txt, Deref @ DAE::Exp::ICONST { integer: _ }, _) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RCONST { real: _ }, _) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SCONST { string: _ }, _) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BCONST { bool: _ }, _) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { name: i_name, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CREF { componentRef: i_componentRef, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BINARY { exp2: i_exp2, exp1: i_exp1, .. }, a_stringDelimiter) => {
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp1.clone(), (a_stringDelimiter.clone()).clone())?;
            l_rhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp2.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::UNARY { operator: i_operator, exp: i_exp }, _) => {
            let mut l_op__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpUnaryOp(Tpl::emptyTxt.clone(), i_operator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LBINARY { exp2: i_exp2, exp1: i_exp1, .. }, a_stringDelimiter) => {
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp1.clone(), (a_stringDelimiter.clone()).clone())?;
            l_rhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp2.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LUNARY { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RELATION { exp2: i_exp2, exp1: i_exp1, .. }, a_stringDelimiter) => {
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp1.clone(), (a_stringDelimiter.clone()).clone())?;
            l_rhs__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp2.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::IFEXP { expElse: i_expElse, expThen: i_expThen, expCond: i_expCond }, a_stringDelimiter) => {
            let mut l_else__str: Tpl::Text;
            let mut l_then__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_expCond.clone(), (a_stringDelimiter.clone()).clone())?;
            l_then__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_expThen.clone(), (a_stringDelimiter.clone()).clone())?;
            l_else__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_expElse.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_then__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: i_expLst, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_argl = dumpExpListCrefs(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(" ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_argl.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { expLst: i_expLst, .. }, a_stringDelimiter) => {
            let mut l_argl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_argl = dumpExpListCrefs(Tpl::emptyTxt.clone(), i_expLst.clone(), (a_stringDelimiter.clone()).clone(), (literal!(" ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_argl.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::PARTEVALFUNCTION { expList: i_expList, path: i_path, .. }, a_stringDelimiter) => {
            let mut l_func__str: Tpl::Text;
            let mut l_argl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_argl = dumpExpList(Tpl::emptyTxt.clone(), i_expList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_argl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ARRAY { scalar: i_scalar, array: i_array, .. }, a_stringDelimiter) => {
            let mut ret_10: bool = false;
            let mut l_expl: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl = dumpExpList(Tpl::emptyTxt.clone(), i_array.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            ret_10 = Config::typeinfo()?;
            txt = fun_80(txt.clone(), ret_10.clone(), i_scalar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATRIX { ty: i_ty, matrix: i_matrix, .. }, a_stringDelimiter) => {
            let mut ret_12: bool = false;
            let mut l_mat__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_mat__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}, {")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_mat__str = lm_81(l_mat__str.clone(), i_matrix.clone(), (a_stringDelimiter.clone()).clone())?;
            l_mat__str = Tpl::popIter(l_mat__str.clone())?;
            ret_12 = Config::typeinfo()?;
            txt = fun_82(txt.clone(), ret_12.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mat__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}}")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ DAE::Exp::RANGE { stop: i_stop, step: i_step, start: i_start, .. }, _) => {
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false)?;
            l_step__str = fun_83(Tpl::emptyTxt.clone(), i_step.clone(), i_e.clone())?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_step__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::List::Nil }, _) => {
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TUPLE { PR: i_PR }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_PR.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CAST { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExpCrefs(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::ASUB { sub: i_sub, exp: i_exp }, a_stringDelimiter) => {
            let mut l_sub__str: Tpl::Text;
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_84(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_85(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_sub__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_sub.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rparen.clone())?;
            txt = Tpl::writeText(txt.clone(), l_sub__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::TSUB { ix: i_ix, exp: i_exp, .. }, a_stringDelimiter) => {
            let mut l_rparen: Tpl::Text;
            let mut l_lparen: Tpl::Text;
            let mut l_needs__paren: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_needs__paren = parenthesizeSubExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_lparen = fun_86(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_rparen = fun_87(Tpl::emptyTxt.clone(), l_needs__paren.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_lparen.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rparen.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_ix.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SIZE { sz: i_sz, exp: i_exp }, a_stringDelimiter) => {
            let mut l_dim__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            l_dim__str = fun_88(Tpl::emptyTxt.clone(), i_sz.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("size(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CODE { code: i_code, .. }, _) => {
            let mut ret_23: ArcStr = arcstr::literal!("");
            let mut l_code__str: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_23 = (Dump::printCodeStr(i_code.clone())?).clone();
            l_code__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_23.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$Code(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_code__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::EMPTY { tyStr: i_tyStr, scope: i_scope, name: i_name_1, .. }, _) => {
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = dumpCref(Tpl::emptyTxt.clone(), i_name_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<EMPTY(scope: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_scope.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", name: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ty: ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_tyStr.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::REDUCTION { iterators: i_iterators, expr: i_expr, reductionInfo: Deref @ DAE::ReductionInfo { iterType: i_ri_iterType, path: i_name, .. } }, a_stringDelimiter) => {
            let mut l_iter__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPathNoQual(Tpl::emptyTxt.clone(), i_name.clone())?;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_expr.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_iter__str = lm_89(l_iter__str.clone(), i_iterators.clone(), (a_stringDelimiter.clone()).clone())?;
            l_iter__str = Tpl::popIter(l_iter__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" for ")).clone() }))?;
            txt = fun_90(txt.clone(), i_ri_iterType.clone())?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LIST { valList: i_valList }, a_stringDelimiter) => {
            let mut l_expl__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_expl__str = dumpExpList(Tpl::emptyTxt.clone(), i_valList.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("List(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_expl__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CONS { cdr: i_cdr, car: i_car }, a_stringDelimiter) => {
            let mut l_cdr__str: Tpl::Text;
            let mut l_car__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_car__str = dumpExp(Tpl::emptyTxt.clone(), i_car.clone(), (a_stringDelimiter.clone()).clone())?;
            l_cdr__str = dumpExp(Tpl::emptyTxt.clone(), i_cdr.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("listCons(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_car__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cdr__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_TUPLE { listExp: i_listExp }, a_stringDelimiter) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = dumpExpList(Tpl::emptyTxt.clone(), i_listExp.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Tuple(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: Some(i_exp) }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::META_OPTION { exp: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::METARECORDCALL { args: i_args, path: i_path, .. }, a_stringDelimiter) => {
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_args__str = dumpExpList(Tpl::emptyTxt.clone(), i_args.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::MATCHEXPRESSION { cases: i_cases, inputs: i_inputs, matchType: i_matchType, .. }, a_stringDelimiter) => {
            let mut l_case__str: Tpl::Text;
            let mut l_inputs__str: Tpl::Text;
            let mut l_match__ty: Tpl::Text;
            let mut txt = (*txt).clone();
            l_match__ty = dumpMatchType(Tpl::emptyTxt.clone(), i_matchType.clone())?;
            l_inputs__str = dumpExpList(Tpl::emptyTxt.clone(), i_inputs.clone(), (a_stringDelimiter.clone()).clone(), (literal!(", ")).clone())?;
            l_case__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_case__str = lm_91(l_case__str.clone(), i_cases.clone())?;
            l_case__str = Tpl::popIter(l_case__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_match__ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_inputs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_case__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_match__ty.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BOX { exp: i_exp }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::UNBOX { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::SHARED_LITERAL { exp: i_exp, .. }, a_stringDelimiter) => {
            let mut txt = (*txt).clone();
            txt = dumpExpCrefs(txt.clone(), i_exp.clone(), (a_stringDelimiter.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::PATTERN { pattern: i_pattern }, _) => {
            let mut txt = (*txt).clone();
            txt = dumpPattern(txt.clone(), i_pattern.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("ExpressionDumpTpl.dumpExp: Unknown expression.")).clone())?;
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

fn fun_94(mut in_txt: Tpl::Text, mut in_a_con: Arc<DAE::Constraint>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_con.clone())) {
        (txt, Deref @ DAE::Constraint::CONSTRAINT_DT { localCon: true, constraint: i_c }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_c.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (local)")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Constraint::CONSTRAINT_DT { localCon: false, constraint: i_c }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_c.clone(), (literal!("\"")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" (global)")).clone() }))?;
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
fn lm_95(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_con, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_94(txt.clone(), i_con.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_95(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpConstraints(mut txt: Tpl::Text, mut a_cons: Arc<metamodelica::List<Arc<DAE::Constraint>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_95(out_txt.clone(), a_cons.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

