// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;

pub fn dump(mut txt: Tpl::Text, mut a_program: Absyn::Program) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_res: Tpl::Text;
    let mut l_preText: Tpl::Text;
    System::tmpTickReset(0);
    l_preText = Tpl::emptyTxt.clone();
    (l_res, l_preText) = dump2(Tpl::emptyTxt.clone(), l_preText.clone(), a_program.clone(), Dump::defaultDumpOptions.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("using Absyn\n")).clone(), (literal!("using MetaModelica\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_preText.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_res.clone())?;
    Ok(out_txt)
}

fn lm_10(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Class>>>, mut in_a_options: Dump::DumpOptions, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cls, tail: rest }, a_options, a_preText) => {
            let mut l_res: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut l_ix: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            ret_1 = System::tmpTick();
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            (l_res, a_preText) = dumpClass(Tpl::emptyTxt.clone(), a_preText.clone(), i_cls.clone(), a_options.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_ix.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_res.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ix.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_10(txt.clone(), rest.clone(), a_options.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_11(mut in_txt: Tpl::Text, mut in_a_program: Absyn::Program, mut in_a_preText: Tpl::Text, mut in_a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_program.clone(), in_a_preText.clone(), in_a_options.clone())) {
        (txt, Absyn::Program { within_: i_within__, classes: Deref @ metamodelica::List::Nil }, a_preText, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PROGRAM(list(), ")).clone() }))?;
            txt = dumpWithin(txt.clone(), i_within__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Absyn::Program { classes: i_classes, within_: i_within__ }, a_preText, a_options) => {
            let mut l_cls__str: Tpl::Text;
            let mut l_within__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_within__str = dumpWithin(Tpl::emptyTxt.clone(), i_within__.clone())?;
            l_cls__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_cls__str, a_preText) = lm_10(l_cls__str.clone(), i_classes.clone(), a_options.clone(), a_preText.clone())?;
            l_cls__str = Tpl::popIter(l_cls__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PROGRAM(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cls__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_within__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText, _) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dump2(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_program: Absyn::Program, mut a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_11(txt.clone(), a_program.clone(), a_preText.clone(), a_options.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_13(mut in_txt: Tpl::Text, mut in_a_cls: Arc<Absyn::Class>, mut in_a_preText: Tpl::Text, mut in_a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cls.clone(), in_a_preText.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::Class { info: i_info, body: i_body, restriction: i_restriction, encapsulatedPrefix: i_encapsulatedPrefix, finalPrefix: i_finalPrefix, partialPrefix: i_partialPrefix, name: i_name, .. }, a_preText, a_options) => {
            let mut ret_8: i32 = 0;
            let mut l_ix: Tpl::Text;
            let mut l_i: Tpl::Text;
            let mut l_cd: Tpl::Text;
            let mut l_r: Tpl::Text;
            let mut l_ep: Tpl::Text;
            let mut l_fp: Tpl::Text;
            let mut l_pp: Tpl::Text;
            let mut l_n: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_n = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_name.clone()).clone())?;
            l_pp = dumpFinal(Tpl::emptyTxt.clone(), i_partialPrefix.clone())?;
            l_fp = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_ep = dumpFinal(Tpl::emptyTxt.clone(), i_encapsulatedPrefix.clone())?;
            l_r = dumpRestriction(Tpl::emptyTxt.clone(), i_restriction.clone())?;
            (l_cd, a_preText) = dumpClassDef(Tpl::emptyTxt.clone(), a_preText.clone(), i_body.clone(), a_options.clone())?;
            l_i = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            ret_8 = System::tmpTick();
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_8.clone())).clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_ix.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_cd.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CLASS(\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_n.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ,")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ep.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_r.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", tmp")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText, _) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpClass(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_cls: Arc<Absyn::Class>, mut a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_13(txt.clone(), a_cls.clone(), a_preText.clone(), a_options.clone())?;
    Ok((out_txt, out_a_preText))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_15(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_typevar, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_typevar.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_15(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_16(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Annotation>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_a, tail: rest }, a_preText) => {
            let mut l_res: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut l_ix: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            ret_1 = System::tmpTick();
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            (l_res, a_preText) = dumpAnnotation(Tpl::emptyTxt.clone(), a_preText.clone(), i_a.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_ix.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_res.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ix.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_16(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_17(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut in_a_options: Dump::DumpOptions, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_class__part, tail: rest }, a_options, a_preText) => {
            let mut l_res: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut l_ix: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            ret_1 = System::tmpTick();
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            (l_res, a_preText) = dumpClassPart(Tpl::emptyTxt.clone(), a_preText.clone(), i_class__part.clone(), a_options.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_ix.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_res.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ix.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_17(txt.clone(), rest.clone(), a_options.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_18(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpNamedArg(txt.clone(), a_preText.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_18(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_19(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementArg(txt.clone(), a_preText.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_19(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_20(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut in_a_options: Dump::DumpOptions, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_class__part, tail: rest }, a_options, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpClassPart(txt.clone(), a_preText.clone(), i_class__part.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_20(txt.clone(), rest.clone(), a_options.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_21(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_mod, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementArg(txt.clone(), a_preText.clone(), i_mod.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_21(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_22(mut in_txt: Tpl::Text, mut in_a_modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modifications.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, i_modifications, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_preText) = lm_21(txt.clone(), i_modifications.clone(), a_preText.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_23(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Annotation>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_a, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpAnnotation(txt.clone(), a_preText.clone(), i_a.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_23(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_24(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpPath(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_24(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_25(mut in_txt: Tpl::Text, mut in_a_cdef: Arc<Absyn::ClassDef>, mut in_a_preText: Tpl::Text, mut in_a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cdef.clone(), in_a_preText.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::ClassDef::PARTS { classAttrs: i_classAttrs, classParts: i_classParts, comment: i_comment, ann: i_ann, typeVars: i_typeVars }, a_preText, a_options) => {
            let mut l_attr__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
            let mut l_ann__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_tvs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_tvs__str = lm_15(l_tvs__str.clone(), i_typeVars.clone())?;
            l_tvs__str = Tpl::popIter(l_tvs__str.clone())?;
            ret_2 = i_ann.clone().reverse();
            l_ann__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_ann__str, a_preText) = lm_16(l_ann__str.clone(), ret_2.clone(), a_preText.clone())?;
            l_ann__str = Tpl::popIter(l_ann__str.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_body__str, a_preText) = lm_17(l_body__str.clone(), i_classParts.clone(), a_options.clone(), a_preText.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_attr__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_attr__str, a_preText) = lm_18(l_attr__str.clone(), i_classAttrs.clone(), a_preText.clone())?;
            l_attr__str = Tpl::popIter(l_attr__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PARTS(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tvs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_attr__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassDef::DERIVED { comment: i_comment_1, arguments: i_arguments, typeSpec: i_typeSpec, attributes: i_attributes }, a_preText, _) => {
            let mut l_arg__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_attr__str, a_preText) = dumpElementAttr(Tpl::emptyTxt.clone(), a_preText.clone(), i_attributes.clone())?;
            (l_ty__str, a_preText) = dumpTypeSpec(Tpl::emptyTxt.clone(), a_preText.clone(), i_typeSpec.clone())?;
            l_arg__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_arg__str, a_preText) = lm_19(l_arg__str.clone(), i_arguments.clone(), a_preText.clone())?;
            l_arg__str = Tpl::popIter(l_arg__str.clone())?;
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DERIVED(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_attr__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: i_baseClassName, ann: i_ann, comment: i_comment, modifications: i_modifications, parts: i_parts }, a_preText, a_options) => {
            let mut ret_9: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
            let mut l_mod__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_body__str, a_preText) = lm_20(l_body__str.clone(), i_parts.clone(), a_options.clone(), a_preText.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            (l_mod__str, a_preText) = fun_22(Tpl::emptyTxt.clone(), i_modifications.clone(), a_preText.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            ret_9 = i_ann.clone().reverse();
            l_ann__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_ann__str, a_preText) = lm_23(l_ann__str.clone(), ret_9.clone(), a_preText.clone())?;
            l_ann__str = Tpl::popIter(l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CLASS_EXTENDS(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_baseClassName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassDef::ENUMERATION { comment: i_comment_1, enumLiterals: i_enumLiterals }, a_preText, _) => {
            let mut l_enum__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_enum__str, a_preText) = dumpEnumDef(Tpl::emptyTxt.clone(), a_preText.clone(), i_enumLiterals.clone())?;
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ENUMERATION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_enum__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassDef::OVERLOAD { comment: i_comment_1, functionNames: i_functionNames }, a_preText, _) => {
            let mut l_funcs__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_funcs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_funcs__str = lm_24(l_funcs__str.clone(), i_functionNames.clone())?;
            l_funcs__str = Tpl::popIter(l_funcs__str.clone())?;
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OVERLOAD(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_funcs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassDef::PDER { functionName: _, .. }, a_preText, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT SUPPORTED???")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText, _) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpClassDef(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_cdef: Arc<Absyn::ClassDef>, mut a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_25(txt.clone(), a_cdef.clone(), a_preText.clone(), a_options.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_27(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_lit, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpEnumLiteral(txt.clone(), a_preText.clone(), i_lit.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_27(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_28(mut in_txt: Tpl::Text, mut in_a_enum__def: Arc<Absyn::EnumDef>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_enum__def.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::EnumDef::ENUMLITERALS { enumLiterals: i_enumLiterals }, a_preText) => {
            let mut l_els: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_els = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_els, a_preText) = lm_27(l_els.clone(), i_enumLiterals.clone(), a_preText.clone())?;
            l_els = Tpl::popIter(l_els.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ENUMLITERALS(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_els.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::EnumDef::ENUM_COLON, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ENUM_COLON()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpEnumDef(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_enum__def: Arc<Absyn::EnumDef>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_28(txt.clone(), a_enum__def.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_30(mut in_txt: Tpl::Text, mut in_a_lit: Arc<Absyn::EnumLiteral>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lit.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::EnumLiteral { literal: i_literal, comment: i_comment }, a_preText) => {
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ENUMLITERAL(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_literal.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpEnumLiteral(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_lit: Arc<Absyn::EnumLiteral>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_30(txt.clone(), a_lit.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_32(mut in_txt: Tpl::Text, mut in_a_functionRestriction: Absyn::FunctionRestriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_functionRestriction.clone()) {
        (mut txt, Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FR_NORMAL_FUNCTION(IMPURE())")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::PURE }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FR_NORMAL_FUNCTION(PURE())")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::NO_PURITY }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FR_NORMAL_FUNCTION(NO_PURITY())")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FR_OPERATOR_FUNCTION()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::FunctionRestriction::FR_PARALLEL_FUNCTION) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FR_PARALLEL_FUNCTION()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::FunctionRestriction::FR_KERNEL_FUNCTION) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FR_KERNEL_FUNCTION()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpRestriction(mut in_txt: Tpl::Text, mut in_a_restriction: Absyn::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_restriction.clone()) {
        (mut txt, Absyn::Restriction::R_CLASS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_CLASS()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_OPTIMIZATION) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_OPTIMIZATION()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_MODEL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_MODEL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_RECORD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_RECORD()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_BLOCK) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_BLOCK()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_CONNECTOR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_CONNECTOR()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_EXP_CONNECTOR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_EXP_CONNECTOR()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_TYPE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_TYPE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_PACKAGE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_PACKAGE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_FUNCTION { functionRestriction: mut i_functionRestriction }) => {
            let mut l_prefix__str: Tpl::Text;
            l_prefix__str = fun_32(Tpl::emptyTxt.clone(), i_functionRestriction.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_FUNCTION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_OPERATOR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_OPERATOR()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_OPERATOR_RECORD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_OPERATOR_RECORD()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_ENUMERATION) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_ENUMERATION()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_PREDEFINED_INTEGER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_PREDEFINED_INTEGER()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_PREDEFINED_REAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_PREDEFINED_REAL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_PREDEFINED_STRING) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_PREDEFINED_STRING()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_PREDEFINED_BOOLEAN) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_PREDEFINED_BOOLEAN()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_PREDEFINED_ENUMERATION) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_PREDEFINED_ENUMERATION()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_UNIONTYPE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_UNIONTYPE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_METARECORD { name: _, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MR: Does not work")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Restriction::R_UNKNOWN) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("R_UNKNOWN()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_34(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_options: Dump::DumpOptions, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_c, tail: rest }, a_options, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementItem(txt.clone(), a_preText.clone(), i_c.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_34(txt.clone(), rest.clone(), a_options.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_35(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_options: Dump::DumpOptions, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_c, tail: rest }, a_options, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementItem(txt.clone(), a_preText.clone(), i_c.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_35(txt.clone(), rest.clone(), a_options.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_36(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_exp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_36(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_37(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpEquationItem(txt.clone(), a_preText.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_37(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_38(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpEquationItem(txt.clone(), a_preText.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_38(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_39(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpAlgorithmItem(txt.clone(), a_preText.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_39(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_40(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpAlgorithmItem(txt.clone(), a_preText.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_40(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_41(mut in_txt: Tpl::Text, mut in_a_annotation__: Option<Arc<Absyn::Annotation>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annotation__.clone(), in_a_preText.clone())) {
        (txt, Some(i_ann), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpAnnotation(txt.clone(), a_preText.clone(), i_ann.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_42(mut in_txt: Tpl::Text, mut in_a_funcName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_funcName.clone()) {
        (mut txt, Some(mut i_fn)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fn.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_43(mut in_txt: Tpl::Text, mut in_a_lang: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_lang.clone()) {
        (mut txt, Some(mut i_l)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_l.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_44(mut in_txt: Tpl::Text, mut in_a_output__: Option<Arc<Absyn::ComponentRef>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_output__.clone(), in_a_preText.clone())) {
        (txt, Some(i_o), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpCref(txt.clone(), a_preText.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_45(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_45(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_46(mut in_txt: Tpl::Text, mut in_a_externalDecl: Arc<Absyn::ExternalDecl>, mut in_a_ann__str: Tpl::Text, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_externalDecl.clone(), in_a_ann__str.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::ExternalDecl { annotation_: i_annotation__, args: i_args, output_: i_output__, lang: i_lang, funcName: i_funcName }, a_ann__str, a_preText) => {
            let mut l_ann2__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_output__str: Tpl::Text;
            let mut l_lang__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_fn__str = fun_42(Tpl::emptyTxt.clone(), i_funcName.clone())?;
            l_lang__str = fun_43(Tpl::emptyTxt.clone(), i_lang.clone())?;
            (l_output__str, a_preText) = fun_44(Tpl::emptyTxt.clone(), i_output__.clone(), a_preText.clone())?;
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_args__str, a_preText) = lm_45(l_args__str.clone(), i_args.clone(), a_preText.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            (l_ann2__str, a_preText) = dumpAnnotationOptSpace(Tpl::emptyTxt.clone(), a_preText.clone(), i_annotation__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EXTERNAL(EXTERNALDECL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fn__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lang__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_output__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ann2__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_47(mut in_txt: Tpl::Text, mut in_a_class__part: Arc<Absyn::ClassPart>, mut in_a_preText: Tpl::Text, mut in_a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_class__part.clone(), in_a_preText.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::ClassPart::PUBLIC { contents: i_contents }, a_preText, a_options) => {
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_el__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_el__str, a_preText) = lm_34(l_el__str.clone(), i_contents.clone(), a_options.clone(), a_preText.clone())?;
            l_el__str = Tpl::popIter(l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PUBLIC(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::PROTECTED { contents: i_contents }, a_preText, a_options) => {
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_el__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_el__str, a_preText) = lm_35(l_el__str.clone(), i_contents.clone(), a_options.clone(), a_preText.clone())?;
            l_el__str = Tpl::popIter(l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PROTECTED(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::CONSTRAINTS { contents: i_contents_1 }, a_preText, _) => {
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_el__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_el__str, a_preText) = lm_36(l_el__str.clone(), i_contents_1.clone(), a_preText.clone())?;
            l_el__str = Tpl::popIter(l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CONSTRAINTS(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::EQUATIONS { contents: i_contents_2 }, a_preText, _) => {
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_el__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_el__str, a_preText) = lm_37(l_el__str.clone(), i_contents_2.clone(), a_preText.clone())?;
            l_el__str = Tpl::popIter(l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQUATIONS(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: i_contents_2 }, a_preText, _) => {
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_el__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_el__str, a_preText) = lm_38(l_el__str.clone(), i_contents_2.clone(), a_preText.clone())?;
            l_el__str = Tpl::popIter(l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INITIALEQUATIONS(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::ALGORITHMS { contents: i_contents_3 }, a_preText, _) => {
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_el__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_el__str, a_preText) = lm_39(l_el__str.clone(), i_contents_3.clone(), a_preText.clone())?;
            l_el__str = Tpl::popIter(l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALGORITHMS(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: i_contents_3 }, a_preText, _) => {
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_el__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_el__str, a_preText) = lm_40(l_el__str.clone(), i_contents_3.clone(), a_preText.clone())?;
            l_el__str = Tpl::popIter(l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INITIALALGORITHMS(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::EXTERNAL { externalDecl: i_externalDecl, annotation_: i_annotation__ }, a_preText, _) => {
            let mut l_ann__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_ann__str, a_preText) = fun_41(Tpl::emptyTxt.clone(), i_annotation__.clone(), a_preText.clone())?;
            (txt, a_preText) = fun_46(txt.clone(), i_externalDecl.clone(), l_ann__str.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText, _) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpClassPart(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_class__part: Arc<Absyn::ClassPart>, mut a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_47(txt.clone(), a_class__part.clone(), a_preText.clone(), a_options.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn dumpWithin(mut in_txt: Tpl::Text, mut in_a_within: Absyn::Within) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_within.clone()) {
        (mut txt, Absyn::Within::TOP) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TOP()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Within::WITHIN { path: ref i_path }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WITHIN(")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_50(mut in_txt: Tpl::Text, mut in_a_isReadOnly: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isReadOnly.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpInfo(mut in_txt: Tpl::Text, mut in_a_info: SourceInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_info.clone()) {
        (mut txt, SourceInfo { columnNumberEnd: mut i_columnNumberEnd, lineNumberEnd: mut i_lineNumberEnd, columnNumberStart: mut i_columnNumberStart, lineNumberStart: mut i_lineNumberStart, fileName: mut i_fileName, isReadOnly: mut i_isReadOnly, .. }) => {
            let mut l_rm__str: Tpl::Text;
            l_rm__str = fun_50(Tpl::emptyTxt.clone(), i_isReadOnly.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOURCEINFO(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rm__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_52(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_earg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementArg(txt.clone(), a_preText.clone(), i_earg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_52(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_53(mut in_txt: Tpl::Text, mut in_a_ann: Arc<Absyn::Annotation>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Annotation { elementArgs: Deref @ metamodelica::List::Nil }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ANNOTATION(list())")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Annotation { elementArgs: i_elementArgs }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ANNOTATION(list(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_preText) = lm_52(txt.clone(), i_elementArgs.clone(), a_preText.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpAnnotation(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_ann: Arc<Absyn::Annotation>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_53(txt.clone(), a_ann.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_oann: Option<Arc<Absyn::Annotation>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_oann.clone(), in_a_preText.clone())) {
        (txt, Some(i_ann), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpAnnotation(txt.clone(), a_preText.clone(), i_ann.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpAnnotationOpt(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_oann: Option<Arc<Absyn::Annotation>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_55(txt.clone(), a_oann.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_oann: Option<Arc<Absyn::Annotation>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_oann.clone(), in_a_preText.clone())) {
        (txt, Some(i_ann), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpAnnotation(txt.clone(), a_preText.clone(), i_ann.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpAnnotationOptSpace(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_oann: Option<Arc<Absyn::Annotation>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_57(txt.clone(), a_oann.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_59(mut in_txt: Tpl::Text, mut in_a_cmt: Arc<Absyn::Comment>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cmt.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Comment { annotation_: i_annotation__, comment: i_comment }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("COMMENT(")).clone() }))?;
            txt = dumpStringCommentOption(txt.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpAnnotationOptSpace(txt.clone(), a_preText.clone(), i_annotation__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpComment(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_cmt: Arc<Absyn::Comment>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_59(txt.clone(), a_cmt.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_61(mut in_txt: Tpl::Text, mut in_a_ocmt: Option<Arc<Absyn::Comment>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ocmt.clone(), in_a_preText.clone())) {
        (txt, Some(i_cmt), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpComment(txt.clone(), a_preText.clone(), i_cmt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpCommentOpt(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_ocmt: Option<Arc<Absyn::Comment>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_61(txt.clone(), a_ocmt.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_63(mut in_txt: Tpl::Text, mut in_a_modification: Option<Arc<Absyn::Modification>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modification.clone(), in_a_preText.clone())) {
        (txt, Some(i_mod), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpModification(txt.clone(), a_preText.clone(), i_mod.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_64(mut in_txt: Tpl::Text, mut in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_constrainClass.clone(), in_a_preText.clone())) {
        (txt, Some(i_cc), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpConstrainClass(txt.clone(), a_preText.clone(), i_cc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_65(mut in_txt: Tpl::Text, mut in_a_earg: Arc<Absyn::ElementArg>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_earg.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::ElementArg::MODIFICATION { info: i_info, comment: i_comment, modification: i_modification, path: i_path, finalPrefix: i_finalPrefix, eachPrefix: i_eachPrefix }, a_preText) => {
            let mut l_info__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut l_each__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            (l_mod__str, a_preText) = fun_63(Tpl::emptyTxt.clone(), i_modification.clone(), a_preText.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MODIFICATION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_final__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_each__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ElementArg::REDECLARATION { info: i_info, constrainClass: i_constrainClass, elementSpec: i_elementSpec, redeclareKeywords: i_redeclareKeywords, finalPrefix: i_finalPrefix, eachPrefix: i_eachPrefix }, a_preText) => {
            let mut l_cc__str: Tpl::Text;
            let mut l_elem__str: Tpl::Text;
            let mut l_redecl__str: Tpl::Text;
            let mut l_info__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut l_each__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_redecl__str = dumpRedeclare(Tpl::emptyTxt.clone(), i_redeclareKeywords.clone())?;
            (l_elem__str, a_preText) = dumpElementSpec(Tpl::emptyTxt.clone(), a_preText.clone(), i_elementSpec.clone(), Dump::defaultDumpOptions.clone())?;
            (l_cc__str, a_preText) = fun_64(Tpl::emptyTxt.clone(), i_constrainClass.clone(), a_preText.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("REDECLARATION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_final__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_redecl__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_each__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elem__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cc__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpElementArg(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_earg: Arc<Absyn::ElementArg>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_65(txt.clone(), a_earg.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn dumpEach(mut in_txt: Tpl::Text, mut in_a_each: Absyn::Each) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_each.clone()) {
        (mut txt, Absyn::Each::EACH) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EACH()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NON_EACH()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpFinal(mut in_txt: Tpl::Text, mut in_a_final: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_final.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpRedeclare(mut in_txt: Tpl::Text, mut in_a_redecl: Absyn::RedeclareKeywords) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_redecl.clone()) {
        (mut txt, Absyn::RedeclareKeywords::REDECLARE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("REDECLARE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("REPLACEABLE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("REDECLARE_REPLACEABLE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpReplaceable(mut in_txt: Tpl::Text, mut in_a_repl: Absyn::RedeclareKeywords) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_repl.clone()) {
        (mut txt, Absyn::RedeclareKeywords::REPLACEABLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("REPLACEABLE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("REDECLARE_REPLACEABLE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpInnerOuter(mut in_txt: Tpl::Text, mut in_a_io: Absyn::InnerOuter) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_io.clone()) {
        (mut txt, Absyn::InnerOuter::INNER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INNER()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::InnerOuter::OUTER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OUTER()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::InnerOuter::INNER_OUTER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INNER_OUTER()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::InnerOuter::NOT_INNER_OUTER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT_INNER_OUTER()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_72(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_earg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementArg(txt.clone(), a_preText.clone(), i_earg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_72(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_73(mut in_txt: Tpl::Text, mut in_a_mod: Arc<Absyn::Modification>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_mod.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Modification { eqMod: i_eqMod, elementArgLst: i_elementArgLst }, a_preText) => {
            let mut l_eq__str: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_arg__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_arg__str, a_preText) = lm_72(l_arg__str.clone(), i_elementArgLst.clone(), a_preText.clone())?;
            l_arg__str = Tpl::popIter(l_arg__str.clone())?;
            (l_eq__str, a_preText) = dumpEqMod(Tpl::emptyTxt.clone(), a_preText.clone(), i_eqMod.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CLASSMOD(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpModification(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_mod: Arc<Absyn::Modification>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_73(txt.clone(), a_mod.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_75(mut in_txt: Tpl::Text, mut in_a_eqmod: Arc<Absyn::EqMod>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eqmod.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::EqMod::EQMOD { info: i_info, exp: i_exp }, a_preText) => {
            let mut l_info__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_exp__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQMOD(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::EqMod::NOMOD, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOMOD()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpEqMod(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_eqmod: Arc<Absyn::EqMod>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_75(txt.clone(), a_eqmod.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_77(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_earg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementArg(txt.clone(), a_preText.clone(), i_earg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_77(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_78(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_comp, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpComponentItem(txt.clone(), a_preText.clone(), i_comp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_78(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_79(mut in_txt: Tpl::Text, mut in_a_elem: Arc<Absyn::ElementSpec>, mut in_a_preText: Tpl::Text, mut in_a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elem.clone(), in_a_preText.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::ElementSpec::CLASSDEF { class_: i_class__, replaceable_: i_replaceable__ }, a_preText, a_options) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CLASSDEF(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_replaceable__.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpClass(txt.clone(), a_preText.clone(), i_class__.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ElementSpec::EXTENDS { annotationOpt: i_annotationOpt, elementArg: i_elementArg, path: i_path }, a_preText, _) => {
            let mut l_ann__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_bc__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_bc__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_args__str, a_preText) = lm_77(l_args__str.clone(), i_elementArg.clone(), a_preText.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            (l_ann__str, a_preText) = dumpAnnotationOptSpace(Tpl::emptyTxt.clone(), a_preText.clone(), i_annotationOpt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EXTENDS(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_bc__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ElementSpec::COMPONENTS { components: i_components, attributes: i_attributes, typeSpec: i_typeSpec }, a_preText, _) => {
            let mut l_comps__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_ty__str, a_preText) = dumpTypeSpec(Tpl::emptyTxt.clone(), a_preText.clone(), i_typeSpec.clone())?;
            (l_attr__str, a_preText) = dumpElementAttr(Tpl::emptyTxt.clone(), a_preText.clone(), i_attributes.clone())?;
            l_comps__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_comps__str, a_preText) = lm_78(l_comps__str.clone(), i_components.clone(), a_preText.clone())?;
            l_comps__str = Tpl::popIter(l_comps__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("COMPONENTS(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_attr__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_comps__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ElementSpec::IMPORT { import_: i_import__, info: i_info, comment: i_comment }, a_preText, _) => {
            let mut l_info__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("IMPORT(")).clone() }))?;
            txt = dumpImport(txt.clone(), i_import__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText, _) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpElementSpec(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_elem: Arc<Absyn::ElementSpec>, mut a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_79(txt.clone(), a_elem.clone(), a_preText.clone(), a_options.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_81(mut in_txt: Tpl::Text, mut in_a_flowPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flowPrefix.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_a_streamPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_streamPrefix.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_83(mut in_txt: Tpl::Text, mut in_a_attr: Absyn::ElementAttributes, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (match (in_txt.clone(), in_a_attr.clone(), in_a_preText.clone()) {
        (mut txt, Absyn::ElementAttributes { arrayDim: ref i_arrayDim, direction: mut i_direction, variability: mut i_variability, isField: mut i_isField, parallelism: mut i_parallelism, streamPrefix: mut i_streamPrefix, flowPrefix: mut i_flowPrefix }, mut a_preText) => {
            let mut l_array__dim: Tpl::Text;
            let mut l_dir__str: Tpl::Text;
            let mut l_var__str: Tpl::Text;
            let mut l_field__str: Tpl::Text;
            let mut l_par__str: Tpl::Text;
            let mut l_stream__str: Tpl::Text;
            let mut l_flow__str: Tpl::Text;
            l_flow__str = fun_81(Tpl::emptyTxt.clone(), i_flowPrefix.clone())?;
            l_stream__str = fun_82(Tpl::emptyTxt.clone(), i_streamPrefix.clone())?;
            l_par__str = dumpParallelism(Tpl::emptyTxt.clone(), i_parallelism.clone())?;
            l_field__str = dumpIsField(Tpl::emptyTxt.clone(), i_isField.clone())?;
            l_var__str = dumpVariability(Tpl::emptyTxt.clone(), i_variability.clone())?;
            l_dir__str = dumpDirection(Tpl::emptyTxt.clone(), i_direction.clone())?;
            (l_array__dim, a_preText) = dumpArrayDim(Tpl::emptyTxt.clone(), a_preText.clone(), i_arrayDim.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ATTR(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_flow__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stream__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_par__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_var__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_dir__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_field__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_array__dim.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (mut txt, _, mut a_preText) => {
            (txt.clone(), a_preText.clone())
        },
    });
    Ok((out_txt, out_a_preText))
}

pub fn dumpElementAttr(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_attr: Absyn::ElementAttributes) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_83(txt.clone(), a_attr.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn dumpParallelism(mut in_txt: Tpl::Text, mut in_a_par: Absyn::Parallelism) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_par.clone()) {
        (mut txt, Absyn::Parallelism::PARGLOBAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PARGLOBAL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Parallelism::PARLOCAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PARGLOBAL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Parallelism::NON_PARALLEL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NON_PARALLEL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpIsField(mut in_txt: Tpl::Text, mut in_a_isField: Absyn::IsField) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isField.clone()) {
        (mut txt, Absyn::IsField::NONFIELD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONFIELD()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::IsField::FIELD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FIELD()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpVariability(mut in_txt: Tpl::Text, mut in_a_var: Absyn::Variability) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, Absyn::Variability::VAR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("VAR()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Variability::DISCRETE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DISCRETE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Variability::PARAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PARAM()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Variability::CONST) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CONST()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpDirection(mut in_txt: Tpl::Text, mut in_a_dir: Absyn::Direction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_dir.clone()) {
        (mut txt, Absyn::Direction::BIDIR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("BIDIR()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Direction::INPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INPUT()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Direction::OUTPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OUTPUT()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Direction::INPUT_OUTPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INPUT_OUTPUT()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_89(mut in_txt: Tpl::Text, mut in_a_attr: Absyn::ElementAttributes, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (match (in_txt.clone(), in_a_attr.clone(), in_a_preText.clone()) {
        (mut txt, Absyn::ElementAttributes { arrayDim: ref i_arrayDim, .. }, mut a_preText) => {
            (txt, a_preText) = dumpSubscripts(txt.clone(), a_preText.clone(), i_arrayDim.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (mut txt, _, mut a_preText) => {
            (txt.clone(), a_preText.clone())
        },
    });
    Ok((out_txt, out_a_preText))
}

pub fn dumpElementAttrDim(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_attr: Absyn::ElementAttributes) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_89(txt.clone(), a_attr.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_91(mut in_txt: Tpl::Text, mut in_a_cc: Arc<Absyn::ConstrainClass>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cc.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::ConstrainClass { elementSpec: i_elementSpec, comment: i_comment }, a_preText) => {
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CONSTRAINCLASS(")).clone() }))?;
            (txt, a_preText) = dumpElementSpec(txt.clone(), a_preText.clone(), i_elementSpec.clone(), Dump::defaultDumpOptions.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpConstrainClass(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_cc: Arc<Absyn::ConstrainClass>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_91(txt.clone(), a_cc.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_93(mut in_txt: Tpl::Text, mut in_a_comp: Arc<Absyn::ComponentItem>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comp.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::ComponentItem { comment: i_comment, condition: i_condition, component: i_component }, a_preText) => {
            let mut ret_4: i32 = 0;
            let mut l_ix: Tpl::Text;
            let mut l_cmt: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_comp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_comp__str, a_preText) = dumpComponent(Tpl::emptyTxt.clone(), a_preText.clone(), i_component.clone())?;
            (l_cond__str, a_preText) = dumpComponentCondition(Tpl::emptyTxt.clone(), a_preText.clone(), i_condition.clone())?;
            (l_cmt, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment.clone())?;
            ret_4 = System::tmpTick();
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_4.clone())).clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_ix.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_comp__str.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("COMPONENTITEM(tmp")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpComponentItem(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_comp: Arc<Absyn::ComponentItem>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_93(txt.clone(), a_comp.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_95(mut in_txt: Tpl::Text, mut in_a_modification: Option<Arc<Absyn::Modification>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modification.clone(), in_a_preText.clone())) {
        (txt, Some(i_mod), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpModification(txt.clone(), a_preText.clone(), i_mod.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_96(mut in_txt: Tpl::Text, mut in_a_comp: Absyn::Component, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (match (in_txt.clone(), in_a_comp.clone(), in_a_preText.clone()) {
        (mut txt, Absyn::Component { name: mut i_name, modification: mut i_modification, arrayDim: ref i_arrayDim }, mut a_preText) => {
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut l_mod__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            (l_dim__str, a_preText) = dumpSubscripts(Tpl::emptyTxt.clone(), a_preText.clone(), i_arrayDim.clone())?;
            (l_mod__str, a_preText) = fun_95(Tpl::emptyTxt.clone(), i_modification.clone(), a_preText.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("COMPONENT(\"")).clone() }))?;
            ret_2 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_dim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (mut txt, _, mut a_preText) => {
            (txt.clone(), a_preText.clone())
        },
    });
    Ok((out_txt, out_a_preText))
}

pub fn dumpComponent(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_comp: Absyn::Component) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_96(txt.clone(), a_comp.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_98(mut in_txt: Tpl::Text, mut in_a_cond: Option<Arc<Absyn::Exp>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cond.clone(), in_a_preText.clone())) {
        (txt, Some(i_cexp), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_cexp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpComponentCondition(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_cond: Option<Arc<Absyn::Exp>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_98(txt.clone(), a_cond.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_100(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Absyn::GroupImport>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_group, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpGroupImport(txt.clone(), i_group.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_100(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpImport(mut in_txt: Tpl::Text, mut in_a_imp: Absyn::Import) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_imp.clone()) {
        (mut txt, Absyn::Import::NAMED_IMPORT { path: ref i_path, name: mut i_name }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NAMED_IMPORT(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Import::QUAL_IMPORT { path: ref i_path }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("QUAL_IMPORT(")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Import::UNQUAL_IMPORT { path: ref i_path }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNQUAL_IMPORT(")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Import::GROUP_IMPORT { groups: ref i_groups, prefix: ref i_prefix }) => {
            let mut l_groups__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            l_prefix__str = dumpPath(Tpl::emptyTxt.clone(), i_prefix.clone())?;
            l_groups__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_groups__str = lm_100(l_groups__str.clone(), i_groups.clone())?;
            l_groups__str = Tpl::popIter(l_groups__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("GROUP_IMPORT(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_prefix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_groups__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpGroupImport(mut in_txt: Tpl::Text, mut in_a_gimp: Absyn::GroupImport) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_gimp.clone()) {
        (mut txt, Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut i_name }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("GROUP_IMPORT_NAME(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::GroupImport::GROUP_IMPORT_RENAME { name: mut i_name, rename: mut i_rename }) => {
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut ret_1: ArcStr = arcstr::literal!("");
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("GROUP_IMPORT_RENAME(\"")).clone() }))?;
            ret_1 = (Util::escapeModelicaStringToJLString((i_rename.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", \"")).clone() }))?;
            ret_2 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_103(mut in_txt: Tpl::Text, mut in_a_eitem: Arc<Absyn::ElementItem>, mut in_a_preText: Tpl::Text, mut in_a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eitem.clone(), in_a_preText.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::ElementItem::ELEMENTITEM { element: i_element }, a_preText, a_options) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ELEMENTITEM(")).clone() }))?;
            (txt, a_preText) = dumpElement(txt.clone(), a_preText.clone(), i_element.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ElementItem::LEXER_COMMENT { comment: i_comment }, a_preText, _) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LEXER_COMMENT(\"")).clone() }))?;
            ret_0 = (System::trimWhitespace((i_comment.clone()).clone())).clone();
            ret_1 = (Util::escapeModelicaStringToJLString((ret_0.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText, _) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpElementItem(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_eitem: Arc<Absyn::ElementItem>, mut a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_103(txt.clone(), a_eitem.clone(), a_preText.clone(), a_options.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_105(mut in_txt: Tpl::Text, mut in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_redeclareKeywords.clone()) {
        (mut txt, Some(mut i_re)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpRedeclare(txt.clone(), i_re.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_106(mut in_txt: Tpl::Text, mut in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_redeclareKeywords.clone()) {
        (mut txt, Some(mut i_re)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            txt = dumpReplaceable(txt.clone(), i_re.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_107(mut in_txt: Tpl::Text, mut in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_constrainClass.clone(), in_a_preText.clone())) {
        (txt, Some(i_cc), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpConstrainClass(txt.clone(), a_preText.clone(), i_cc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_108(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_info: SourceInfo, mut in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut in_a_options: Dump::DumpOptions, mut in_a_specification: Arc<Absyn::ElementSpec>, mut in_a_preText: Tpl::Text, mut in_a_innerOuter: Absyn::InnerOuter, mut in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>, mut in_a_finalPrefix: bool) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_info.clone(), in_a_constrainClass.clone(), in_a_options.clone(), in_a_specification.clone(), in_a_preText.clone(), in_a_innerOuter.clone(), in_a_redeclareKeywords.clone(), in_a_finalPrefix.clone())) {
        (txt, false, _, _, _, _, a_preText, _, _, _) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_info, a_constrainClass, a_options, a_specification, a_preText, a_innerOuter, a_redeclareKeywords, a_finalPrefix) => {
            let mut l_info__str: Tpl::Text;
            let mut l_cc__str: Tpl::Text;
            let mut l_ec__str: Tpl::Text;
            let mut l_io__str: Tpl::Text;
            let mut l_repl__str: Tpl::Text;
            let mut l_redecl__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), a_finalPrefix.clone())?;
            l_redecl__str = fun_105(Tpl::emptyTxt.clone(), a_redeclareKeywords.clone())?;
            l_repl__str = fun_106(Tpl::emptyTxt.clone(), a_redeclareKeywords.clone())?;
            l_io__str = dumpInnerOuter(Tpl::emptyTxt.clone(), a_innerOuter.clone())?;
            (l_ec__str, a_preText) = dumpElementSpec(Tpl::emptyTxt.clone(), a_preText.clone(), a_specification.clone(), a_options.clone())?;
            (l_cc__str, a_preText) = fun_107(Tpl::emptyTxt.clone(), a_constrainClass.clone(), a_preText.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), a_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ELEMENT(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(a_finalPrefix.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_redecl__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_io__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ec__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cc__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_109(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpNamedArg(txt.clone(), a_preText.clone(), i_arg.clone())?;
            (txt, a_preText) = lm_109(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_110(mut in_txt: Tpl::Text, mut in_a_args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, i_args, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = lm_109(txt.clone(), i_args.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_111(mut in_txt: Tpl::Text, mut in_a_optName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_optName.clone()) {
        (mut txt, Some(_)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(name)")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_112(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_string: ArcStr, mut in_a_info: SourceInfo, mut in_a_optName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_string.clone(), in_a_info.clone(), in_a_optName.clone()) {
        (mut txt, false, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_string, mut a_info, mut a_optName) => {
            let mut l_string__str: Tpl::Text;
            let mut l_info__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            l_name__str = fun_111(Tpl::emptyTxt.clone(), a_optName.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), a_info.clone())?;
            l_string__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_string.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TEXT(\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\",\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_string__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\",")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_113(mut in_txt: Tpl::Text, mut in_a_elem: Arc<Absyn::Element>, mut in_a_preText: Tpl::Text, mut in_a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elem.clone(), in_a_preText.clone(), in_a_options.clone())) {
        (txt, i_elem @ Deref @ Absyn::Element::ELEMENT { constrainClass: i_constrainClass, specification: i_specification, innerOuter: i_innerOuter, redeclareKeywords: i_redeclareKeywords, finalPrefix: i_finalPrefix, info: i_info }, a_preText, a_options) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            ret_0 = Dump::boolUnparseFileFromInfo(i_info.clone(), a_options.clone())?;
            ret_1 = AbsynUtil::isClassdef(i_elem.clone());
            ret_2 = boolNot(ret_1.clone());
            ret_3 = boolOr(ret_0.clone(), ret_2.clone());
            (txt, a_preText) = fun_108(txt.clone(), ret_3.clone(), i_info.clone(), i_constrainClass.clone(), a_options.clone(), i_specification.clone(), a_preText.clone(), i_innerOuter.clone(), i_redeclareKeywords.clone(), i_finalPrefix.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Element::DEFINEUNIT { name: i_name, args: i_args, .. }, a_preText, _) => {
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_args__str, a_preText) = fun_110(Tpl::emptyTxt.clone(), i_args.clone(), a_preText.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DEFINEUNIT(\"")).clone() }))?;
            ret_5 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_5.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Element::TEXT { string: i_string, optName: i_optName, info: i_info }, a_preText, a_options) => {
            let mut ret_6: bool = false;
            let mut txt = (*txt).clone();
            ret_6 = Dump::boolUnparseFileFromInfo(i_info.clone(), a_options.clone())?;
            txt = fun_112(txt.clone(), ret_6.clone(), (i_string.clone()).clone(), i_info.clone(), i_optName.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText, _) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpElement(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_elem: Arc<Absyn::Element>, mut a_options: Dump::DumpOptions) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_113(txt.clone(), a_elem.clone(), a_preText.clone(), a_options.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_115(mut in_txt: Tpl::Text, mut in_a_eq: Arc<Absyn::EquationItem>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::EquationItem::EQUATIONITEM { info: i_info, comment: i_comment, equation_: i_equation__ }, a_preText) => {
            let mut l_info__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_eq__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_eq__str, a_preText) = dumpEquation(Tpl::emptyTxt.clone(), a_preText.clone(), i_equation__.clone())?;
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQUATIONITEM(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { comment: i_comment_1 }, a_preText) => {
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQUATIONITEMCOMMENT(\"")).clone() }))?;
            ret_3 = (System::trimWhitespace((i_comment_1.clone()).clone())).clone();
            ret_4 = (Util::escapeModelicaStringToJLString((ret_3.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpEquationItem(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_eq: Arc<Absyn::EquationItem>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_115(txt.clone(), a_eq.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_117(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpEquationItem(txt.clone(), a_preText.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_117(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpEquationItems(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_eql: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (out_txt, out_a_preText) = lm_117(out_txt.clone(), a_eql.clone(), a_preText.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_119(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tuple(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            (txt, a_preText) = dumpEquationItems(txt.clone(), a_preText.clone(), i_b.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_119(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_120(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tuple(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            (txt, a_preText) = dumpEquationItems(txt.clone(), a_preText.clone(), i_b.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_120(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_121(mut in_txt: Tpl::Text, mut in_a_eq: Arc<Absyn::Equation>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Equation::EQ_IF { equationElseItems: i_equationElseItems, elseIfBranches: i_elseIfBranches, equationTrueItems: i_equationTrueItems, ifExp: i_ifExp }, a_preText) => {
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_eq__true__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_if__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_ifExp.clone())?;
            (l_eq__true__str, a_preText) = dumpEquationItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_equationTrueItems.clone())?;
            l_elseif__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_elseif__str, a_preText) = lm_119(l_elseif__str.clone(), i_elseIfBranches.clone(), a_preText.clone())?;
            l_elseif__str = Tpl::popIter(l_elseif__str.clone())?;
            (l_else__branch__str, a_preText) = dumpEquationItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_equationElseItems.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_IF(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_if__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eq__true__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elseif__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__branch__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Equation::EQ_EQUALS { rightSide: i_rightSide, leftSide: i_leftSide }, a_preText) => {
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_lhs, a_preText) = dumpLhsExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_leftSide.clone())?;
            (l_rhs, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_rightSide.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_EQUALS(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Equation::EQ_PDE { domain: i_domain, rightSide: i_rightSide, leftSide: i_leftSide }, a_preText) => {
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_lhs, a_preText) = dumpLhsExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_leftSide.clone())?;
            (l_rhs, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_rightSide.clone())?;
            (l_domain__str, a_preText) = dumpCref(Tpl::emptyTxt.clone(), a_preText.clone(), i_domain.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_PDE(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_domain__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Equation::EQ_CONNECT { connector2: i_connector2, connector1: i_connector1 }, a_preText) => {
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_c1__str, a_preText) = dumpCref(Tpl::emptyTxt.clone(), a_preText.clone(), i_connector1.clone())?;
            (l_c2__str, a_preText) = dumpCref(Tpl::emptyTxt.clone(), a_preText.clone(), i_connector2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_CONNECT(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_c1__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_c2__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Equation::EQ_FOR { forEquations: i_forEquations, iterators: i_iterators }, a_preText) => {
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_iter__str, a_preText) = dumpForIterators(Tpl::emptyTxt.clone(), a_preText.clone(), i_iterators.clone())?;
            (l_body__str, a_preText) = dumpEquationItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_forEquations.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_FOR(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Equation::EQ_WHEN_E { whenEquations: i_whenEquations, elseWhenEquations: i_elseWhenEquations, whenExp: i_whenExp }, a_preText) => {
            let mut l_when__eqs: Tpl::Text;
            let mut l_elsewhen__eqs__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_when__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_whenExp.clone())?;
            l_elsewhen__eqs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_elsewhen__eqs__str, a_preText) = lm_120(l_elsewhen__eqs__str.clone(), i_elseWhenEquations.clone(), a_preText.clone())?;
            l_elsewhen__eqs__str = Tpl::popIter(l_elsewhen__eqs__str.clone())?;
            (l_when__eqs, a_preText) = dumpEquationItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_whenEquations.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_WHEN_E(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_when__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_when__eqs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("),list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elsewhen__eqs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Equation::EQ_NORETCALL { functionArgs: i_functionArgs, functionName: i_functionName }, a_preText) => {
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_name__str, a_preText) = dumpCref(Tpl::emptyTxt.clone(), a_preText.clone(), i_functionName.clone())?;
            (l_args__str, a_preText) = dumpFunctionArgs(Tpl::emptyTxt.clone(), a_preText.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_NORETCALL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Equation::EQ_FAILURE { equ: i_equ }, a_preText) => {
            let mut l_eq__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_eq__str, a_preText) = dumpEquationItem(Tpl::emptyTxt.clone(), a_preText.clone(), i_equ.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQ_FAILURE(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpEquation(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_eq: Arc<Absyn::Equation>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_121(txt.clone(), a_eq.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_123(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_alg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpAlgorithmItem(txt.clone(), a_preText.clone(), i_alg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_123(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpAlgorithmItems(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    let mut l_items: Tpl::Text;
    l_items = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_items, out_a_preText) = lm_123(l_items.clone(), a_algs.clone(), a_preText.clone())?;
    l_items = Tpl::popIter(l_items.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_items.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    Ok((out_txt, out_a_preText))
}

fn fun_125(mut in_txt: Tpl::Text, mut in_a_alg: Arc<Absyn::AlgorithmItem>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { info: i_info, comment: i_comment, algorithm_: i_algorithm__ }, a_preText) => {
            let mut ret_4: i32 = 0;
            let mut l_ix: Tpl::Text;
            let mut l_info__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_alg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_alg__str, a_preText) = dumpAlgorithm(Tpl::emptyTxt.clone(), a_preText.clone(), i_algorithm__.clone())?;
            (l_cmt__str, a_preText) = dumpCommentOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_comment.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            ret_4 = System::tmpTick();
            l_ix = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_4.clone())).clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tmp")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_ix.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_preText = Tpl::writeText(a_preText.clone(), l_alg__str.clone())?;
            a_preText = Tpl::writeTok(a_preText.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALGORITHMITEM(tmp")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ix.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::AlgorithmItem::ALGORITHMITEMCOMMENT { comment: _ }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALGORITHMITEMCOMMENT(\"I am useless. I am a comment\")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpAlgorithmItem(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_alg: Arc<Absyn::AlgorithmItem>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_125(txt.clone(), a_alg.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_127(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpAlgorithmItems(txt.clone(), a_preText.clone(), i_b.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_127(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_128(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_c.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpAlgorithmItems(txt.clone(), a_preText.clone(), i_b.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_128(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_129(mut in_txt: Tpl::Text, mut in_a_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equ.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, i_equ, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpAlgorithmItems(txt.clone(), a_preText.clone(), i_equ.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_130(mut in_txt: Tpl::Text, mut in_a_alg: Arc<Absyn::Algorithm>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Algorithm::ALG_ASSIGN { value: i_value, assignComponent: i_assignComponent }, a_preText) => {
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_lhs__str, a_preText) = dumpLhsExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_assignComponent.clone())?;
            (l_rhs__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_value.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_ASSIGN(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_IF { elseBranch: i_elseBranch, elseIfAlgorithmBranch: i_elseIfAlgorithmBranch, trueBranch: i_trueBranch, ifExp: i_ifExp }, a_preText) => {
            let mut l_else__branch: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_else__if__alg__branch: Tpl::Text;
            let mut l_true__branch: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_if__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_ifExp.clone())?;
            (l_true__branch, a_preText) = dumpAlgorithmItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_trueBranch.clone())?;
            l_else__if__alg__branch = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_else__if__alg__branch, a_preText) = lm_127(l_else__if__alg__branch.clone(), i_elseIfAlgorithmBranch.clone(), a_preText.clone())?;
            l_else__if__alg__branch = Tpl::popIter(l_else__if__alg__branch.clone())?;
            (l_else__branch__str, a_preText) = dumpAlgorithmItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_elseBranch.clone())?;
            (l_else__branch, a_preText) = dumpAlgorithmItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_elseBranch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_IF(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_if__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_true__branch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__if__alg__branch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__branch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_FOR { forBody: i_forBody, iterators: i_iterators }, a_preText) => {
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_iter__str, a_preText) = dumpForIterators(Tpl::emptyTxt.clone(), a_preText.clone(), i_iterators.clone())?;
            (l_body__str, a_preText) = dumpAlgorithmItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_forBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_FOR(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_PARFOR { parforBody: i_parforBody, iterators: i_iterators }, a_preText) => {
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_iter__str, a_preText) = dumpForIterators(Tpl::emptyTxt.clone(), a_preText.clone(), i_iterators.clone())?;
            (l_body__str, a_preText) = dumpAlgorithmItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_parforBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_PARFOR(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_WHILE { whileBody: i_whileBody, boolExpr: i_boolExpr }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_WHILE(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_boolExpr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpAlgorithmItems(txt.clone(), a_preText.clone(), i_whileBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_WHEN_A { whenBody: i_whenBody, boolExpr: i_boolExpr, elseWhenAlgorithmBranch: i_elseWhenAlgorithmBranch }, a_preText) => {
            let mut l_ewab: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_ewab = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_ewab, a_preText) = lm_128(l_ewab.clone(), i_elseWhenAlgorithmBranch.clone(), a_preText.clone())?;
            l_ewab = Tpl::popIter(l_ewab.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_WHEN_A(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_boolExpr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpAlgorithmItems(txt.clone(), a_preText.clone(), i_whenBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ewab.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_NORETCALL { functionArgs: i_functionArgs, functionCall: i_functionCall }, a_preText) => {
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_name__str, a_preText) = dumpCref(Tpl::emptyTxt.clone(), a_preText.clone(), i_functionCall.clone())?;
            (l_args__str, a_preText) = dumpFunctionArgs(Tpl::emptyTxt.clone(), a_preText.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_NORETCALL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_RETURN, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_RETURN()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_BREAK, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_BREAK()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_FAILURE { equ: i_equ }, a_preText) => {
            let mut l_arg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_arg__str, a_preText) = fun_129(Tpl::emptyTxt.clone(), i_equ.clone(), a_preText.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_FAILURE(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_TRY { elseBody: i_elseBody, body: i_body }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_TRY(")).clone() }))?;
            (txt, a_preText) = dumpAlgorithmItems(txt.clone(), a_preText.clone(), i_body.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpAlgorithmItems(txt.clone(), a_preText.clone(), i_elseBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Algorithm::ALG_CONTINUE, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALG_CONTINUE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpAlgorithm(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_alg: Arc<Absyn::Algorithm>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_130(txt.clone(), a_alg.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_132(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_path: Arc<Absyn::Path>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_path.clone(), in_a_name.clone())) {
        (txt, false, _, a_name) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("IDENT(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToJLString((a_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_path, a_name) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("QUALIFIED(\"")).clone() }))?;
            ret_1 = (Util::escapeModelicaStringToJLString((a_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = dumpPath(txt.clone(), a_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpPath(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FULLYQUALIFIED(")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::QUALIFIED { path: i_path, name: i_name }) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_132(txt.clone(), ret_0.clone(), i_path.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("IDENT(\"")).clone() }))?;
            ret_1 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpPath: Unknown path.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpPathNoQual(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FULLYQUALIFIED(")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_path) => {
            let mut txt = (*txt).clone();
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpStringCommentOption(mut in_txt: Tpl::Text, mut in_a_cmt: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_cmt.clone()) {
        (mut txt, Some(mut i_str)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_136(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpTypeSpec(txt.clone(), a_preText.clone(), i_ty.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_136(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_137(mut in_txt: Tpl::Text, mut in_a_typeSpec: Arc<Absyn::TypeSpec>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeSpec.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::TypeSpec::TPATH { arrayDim: i_arrayDim, path: i_path }, a_preText) => {
            let mut l_arraydim__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            (l_arraydim__str, a_preText) = dumpArrayDimOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_arrayDim.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TPATH(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arraydim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::TypeSpec::TCOMPLEX { arrayDim: i_arrayDim, typeSpecs: i_typeSpecs, path: i_path }, a_preText) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_arraydim__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_ty__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_ty__str, a_preText) = lm_136(l_ty__str.clone(), i_typeSpecs.clone(), a_preText.clone())?;
            l_ty__str = Tpl::popIter(l_ty__str.clone())?;
            (l_arraydim__str, a_preText) = dumpArrayDimOpt(Tpl::emptyTxt.clone(), a_preText.clone(), i_arrayDim.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TCOMPLEX(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arraydim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpTypeSpec(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_typeSpec: Arc<Absyn::TypeSpec>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_137(txt.clone(), a_typeSpec.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_139(mut in_txt: Tpl::Text, mut in_a_arraydim: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arraydim.clone(), in_a_preText.clone())) {
        (txt, Some(i_ad), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpSubscripts(txt.clone(), a_preText.clone(), i_ad.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpArrayDimOpt(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_arraydim: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_139(txt.clone(), a_arraydim.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn dumpArrayDim(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_arraydim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = dumpSubscripts(txt.clone(), a_preText.clone(), a_arraydim.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_142(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpSubscript(txt.clone(), a_preText.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_142(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpSubscripts(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    let mut l_sub__str: Tpl::Text;
    l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_sub__str, out_a_preText) = lm_142(l_sub__str.clone(), a_subscripts.clone(), a_preText.clone())?;
    l_sub__str = Tpl::popIter(l_sub__str.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_sub__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    Ok((out_txt, out_a_preText))
}

fn fun_144(mut in_txt: Tpl::Text, mut in_a_subscript: Arc<Absyn::Subscript>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Subscript::NOSUB, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOSUB()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: i_subscript }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SUBSCRIPT(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_subscript.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpSubscript(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_subscript: Arc<Absyn::Subscript>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_144(txt.clone(), a_subscript.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_146(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_146(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_147(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_147(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_148(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_row, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_preText) = lm_147(txt.clone(), i_row.clone(), a_preText.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_148(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_149(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_149(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_150(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_150(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_151(mut in_txt: Tpl::Text, mut in_a_exp: Arc<Absyn::Exp>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Exp::INTEGER { value: i_value }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("INTEGER(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_value.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::REAL { value: i_value_1 }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("REAL(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_value_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::CREF { componentRef: i_componentRef }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF(")).clone() }))?;
            (txt, a_preText) = dumpCref(txt.clone(), a_preText.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::STRING { value: i_value_1 }, a_preText) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("STRING(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToJLString((i_value_1.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::BOOL { value: i_value_2 }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("BOOL(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_value_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_e @ Deref @ Absyn::Exp::BINARY { op: i_op, exp2: i_exp2, exp1: i_exp1 }, a_preText) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_lhs__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp1.clone(), i_e.clone(), true)?;
            (l_rhs__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("BINARY(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_e @ Deref @ Absyn::Exp::UNARY { op: i_op, exp: i_exp }, a_preText) => {
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_exp__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNARY(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_e @ Deref @ Absyn::Exp::LBINARY { op: i_op, exp2: i_exp2, exp1: i_exp1 }, a_preText) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_lhs__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp1.clone(), i_e.clone(), true)?;
            (l_rhs__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LBINARY(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_e @ Deref @ Absyn::Exp::LUNARY { op: i_op, exp: i_exp }, a_preText) => {
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_exp__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LUNARY(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_e @ Deref @ Absyn::Exp::RELATION { op: i_op, exp2: i_exp2, exp1: i_exp1 }, a_preText) => {
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_lhs__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp1.clone(), i_e.clone(), true)?;
            (l_rhs__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("RELATION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_exp @ Deref @ Absyn::Exp::IFEXP { ifExp: _, .. }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpIfExp(txt.clone(), a_preText.clone(), i_exp.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::CALL { functionArgs: i_functionArgs, function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "$array", .. }, .. }, a_preText) => {
            let mut ret_6: ArcStr = arcstr::literal!("");
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_args__str, a_preText) = dumpFunctionArgs(Tpl::emptyTxt.clone(), a_preText.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CALL(CREF_IDENT(\"")).clone() }))?;
            ret_6 = (Util::escapeModelicaStringToJLString((literal!("array")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_6.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", list()) ,")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::CALL { functionArgs: i_functionArgs, function_: i_function__, .. }, a_preText) => {
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_func__str, a_preText) = dumpCref(Tpl::emptyTxt.clone(), a_preText.clone(), i_function__.clone())?;
            (l_args__str, a_preText) = dumpFunctionArgs(Tpl::emptyTxt.clone(), a_preText.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CALL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::PARTEVALFUNCTION { functionArgs: i_functionArgs, function_: i_function__ }, a_preText) => {
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_func__str, a_preText) = dumpCref(Tpl::emptyTxt.clone(), a_preText.clone(), i_function__.clone())?;
            (l_args__str, a_preText) = dumpFunctionArgs(Tpl::emptyTxt.clone(), a_preText.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("PARTEVALFUNCTION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::ARRAY { arrayExp: i_arrayExp }, a_preText) => {
            let mut l_array__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_array__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_array__str, a_preText) = lm_146(l_array__str.clone(), i_arrayExp.clone(), a_preText.clone())?;
            l_array__str = Tpl::popIter(l_array__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ARRAY(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_array__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::MATRIX { matrix: i_matrix }, a_preText) => {
            let mut l_matrix__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_matrix__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_matrix__str, a_preText) = lm_148(l_matrix__str.clone(), i_matrix.clone(), a_preText.clone())?;
            l_matrix__str = Tpl::popIter(l_matrix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MATRIX(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_matrix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_e @ Deref @ Absyn::Exp::RANGE { stop: i_stop, start: i_start, step: Some(i_step) }, a_preText) => {
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_start__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_start.clone(), i_e.clone(), false)?;
            (l_step__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_step.clone(), i_e.clone(), false)?;
            (l_stop__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("RANGE(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", SOME(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_step__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_e @ Deref @ Absyn::Exp::RANGE { stop: i_stop, start: i_start, step: None }, a_preText) => {
            let mut l_stop__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_start__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_start.clone(), i_e.clone(), false)?;
            (l_stop__str, a_preText) = dumpOperand(Tpl::emptyTxt.clone(), a_preText.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("RANGE(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", NONE(), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::TUPLE { expressions: i_expressions }, a_preText) => {
            let mut l_tuple__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_tuple__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_tuple__str, a_preText) = lm_149(l_tuple__str.clone(), i_expressions.clone(), a_preText.clone())?;
            l_tuple__str = Tpl::popIter(l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("TUPLE(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::END, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("END()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::CODE { code: i_code }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CODE(")).clone() }))?;
            (txt, a_preText) = dumpCodeNode(txt.clone(), a_preText.clone(), i_code.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::AS { id: i_id, exp: i_exp }, a_preText) => {
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_exp__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("AS(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::CONS { rest: i_rest, head: i_head }, a_preText) => {
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_head__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_head.clone())?;
            (l_rest__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_rest.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CONS(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_head__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rest__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_exp @ Deref @ Absyn::Exp::MATCHEXP { matchTy: _, .. }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpMatchExp(txt.clone(), a_preText.clone(), i_exp.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::LIST { exps: i_exps }, a_preText) => {
            let mut l_list__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_list__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_list__str, a_preText) = lm_150(l_list__str.clone(), i_exps.clone(), a_preText.clone())?;
            l_list__str = Tpl::popIter(l_list__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LIST(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_list__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Exp::DOT { index: i_index, exp: i_exp }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DOT(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_index.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* AbsynDumpTpl.dumpExp: UNHANDLED Abyn.Exp */")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpExp(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_exp: Arc<Absyn::Exp>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_151(txt.clone(), a_exp.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_153(mut in_txt: Tpl::Text, mut in_a_lhs: Arc<Absyn::Exp>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lhs.clone(), in_a_preText.clone())) {
        (txt, i_lhs @ Deref @ Absyn::Exp::IFEXP { ifExp: _, .. }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, i_lhs, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_lhs.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpLhsExp(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_lhs: Arc<Absyn::Exp>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_153(txt.clone(), a_lhs.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn dumpOperand(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_operand: Arc<Absyn::Exp>, mut a_operation: Arc<Absyn::Exp>, mut a_lhs: bool) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = dumpExp(txt.clone(), a_preText.clone(), a_operand.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_156(mut in_txt: Tpl::Text, mut in_a_if__exp: Arc<Absyn::Exp>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_if__exp.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Exp::IFEXP { elseIfBranch: i_elseIfBranch, elseBranch: i_elseBranch, trueBranch: i_trueBranch, ifExp: i_ifExp }, a_preText) => {
            let mut l_else__if__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_true__branch__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_cond__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_ifExp.clone())?;
            (l_true__branch__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_trueBranch.clone())?;
            (l_else__branch__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_elseBranch.clone())?;
            (l_else__if__str, a_preText) = dumpElseIfExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_elseIfBranch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("IFEXP(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_true__branch__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__branch__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__if__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpIfExp(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_if__exp: Arc<Absyn::Exp>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_156(txt.clone(), a_if__exp.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_158(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_cond, i_branch), tail: rest }, a_preText) => {
            let mut l_branch__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_cond__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_cond.clone())?;
            (l_branch__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_branch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_branch__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_158(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpElseIfExp(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_else__if: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    let mut l_lst: Tpl::Text;
    l_lst = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (l_lst, out_a_preText) = lm_158(l_lst.clone(), a_else__if.clone(), a_preText.clone())?;
    l_lst = Tpl::popIter(l_lst.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_lst.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    Ok((out_txt, out_a_preText))
}

fn fun_160(mut in_txt: Tpl::Text, mut in_a_boolean: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_boolean.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_161(mut in_txt: Tpl::Text, mut in_a_boolean: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_boolean.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_162(mut in_txt: Tpl::Text, mut in_a_boolean: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_boolean.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("false")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("true")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_163(mut in_txt: Tpl::Text, mut in_a_code: Arc<Absyn::CodeNode>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_code.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::CodeNode::C_TYPENAME { path: i_path }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_TYPENAME(")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: _ }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_VARIABLENAME(dumpCref(componentRef))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::CodeNode::C_CONSTRAINTSECTION { equationItemLst: i_equationItemLst, boolean: i_boolean }, a_preText) => {
            let mut l_equation__is__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_initial__str = fun_160(Tpl::emptyTxt.clone(), i_boolean.clone())?;
            (l_equation__is__str, a_preText) = dumpEquationItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_equationItemLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_CONSTRAINTSECTION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_initial__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_equation__is__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::CodeNode::C_EQUATIONSECTION { equationItemLst: i_equationItemLst, boolean: i_boolean }, a_preText) => {
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_initial__str = fun_161(Tpl::emptyTxt.clone(), i_boolean.clone())?;
            (l_eql__str, a_preText) = dumpEquationItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_equationItemLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_EQUATIONSECTION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_initial__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::CodeNode::C_ALGORITHMSECTION { algorithmItemLst: i_algorithmItemLst, boolean: i_boolean }, a_preText) => {
            let mut l_algs__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_initial__str = fun_162(Tpl::emptyTxt.clone(), i_boolean.clone())?;
            (l_algs__str, a_preText) = dumpAlgorithmItems(Tpl::emptyTxt.clone(), a_preText.clone(), i_algorithmItemLst.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_ALGORITHMSECTION(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_initial__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_algs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::CodeNode::C_EXPRESSION { exp: i_exp }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_EXPRESSION(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::CodeNode::C_MODIFICATION { modification: i_modification }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_MODIFICATION(")).clone() }))?;
            (txt, a_preText) = dumpModification(txt.clone(), a_preText.clone(), i_modification.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::CodeNode::C_ELEMENT { element: i_element }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("C_ELEMENT(")).clone() }))?;
            (txt, a_preText) = dumpElement(txt.clone(), a_preText.clone(), i_element.clone(), Dump::defaultDumpOptions.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpCodeNode(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_code: Arc<Absyn::CodeNode>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_163(txt.clone(), a_code.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_165(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Case>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_c, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpMatchCase(txt.clone(), a_preText.clone(), i_c.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_165(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_166(mut in_txt: Tpl::Text, mut in_a_match__exp: Arc<Absyn::Exp>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_match__exp.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Exp::MATCHEXP { comment: i_comment, cases: i_cases, localDecls: i_localDecls, inputExp: i_inputExp, matchTy: i_matchTy }, a_preText) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_cases__str: Tpl::Text;
            let mut l_locals__str: Tpl::Text;
            let mut l_input__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_ty__str = dumpMatchType(Tpl::emptyTxt.clone(), i_matchTy.clone())?;
            (l_input__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_inputExp.clone())?;
            (l_locals__str, a_preText) = dumpMatchLocals(Tpl::emptyTxt.clone(), a_preText.clone(), i_localDecls.clone())?;
            l_cases__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_cases__str, a_preText) = lm_165(l_cases__str.clone(), i_cases.clone(), a_preText.clone())?;
            l_cases__str = Tpl::popIter(l_cases__str.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MATCHEXP(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_input__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_locals__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cases__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpMatchExp(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_match__exp: Arc<Absyn::Exp>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_166(txt.clone(), a_match__exp.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn dumpMatchType(mut in_txt: Tpl::Text, mut in_a_match__type: Absyn::MatchType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_match__type.clone()) {
        (mut txt, Absyn::MatchType::MATCH) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MATCH()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::MatchType::MATCHCONTINUE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MATCHCONTINUE()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn lm_169(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_decl, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementItem(txt.clone(), a_preText.clone(), i_decl.clone(), Dump::defaultDumpOptions.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_169(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_170(mut in_txt: Tpl::Text, mut in_a_locals: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_locals.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, i_locals, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_preText) = lm_169(txt.clone(), i_locals.clone(), a_preText.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpMatchLocals(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_locals: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_170(txt.clone(), a_locals.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_172(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpEquationItem(txt.clone(), a_preText.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_172(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_173(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_alg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpAlgorithmItem(txt.clone(), a_preText.clone(), i_alg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_173(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_174(mut in_txt: Tpl::Text, mut in_a_cp: Arc<Absyn::ClassPart>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cp.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::ClassPart::EQUATIONS { contents: Deref @ metamodelica::List::Nil }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQUATIONS(list())")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::EQUATIONS { contents: i_eql }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQUATIONS(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_preText) = lm_172(txt.clone(), i_eql.clone(), a_preText.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::ALGORITHMS { contents: Deref @ metamodelica::List::Nil }, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALGORITHMS(list())")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ClassPart::ALGORITHMS { contents: i_algs }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALGORITHMS(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_preText) = lm_173(txt.clone(), i_algs.clone(), a_preText.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpMatchEquations(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_cp: Arc<Absyn::ClassPart>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_174(txt.clone(), a_cp.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_176(mut in_txt: Tpl::Text, mut in_a_patternGuard: Option<Arc<Absyn::Exp>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_patternGuard.clone(), in_a_preText.clone())) {
        (txt, Some(i_g), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_g.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_177(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_d, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementItem(txt.clone(), a_preText.clone(), i_d.clone(), Dump::defaultDumpOptions.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_177(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_178(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_d, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpElementItem(txt.clone(), a_preText.clone(), i_d.clone(), Dump::defaultDumpOptions.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_178(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_179(mut in_txt: Tpl::Text, mut in_a_c: Arc<Absyn::Case>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_c.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::Case::CASE { info: i_info, comment: i_comment, resultInfo: i_resultInfo, result: i_result, classPart: i_classPart, localDecls: i_localDecls, patternInfo: i_patternInfo, patternGuard: i_patternGuard, pattern: i_pattern }, a_preText) => {
            let mut l_i__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_r__i__str: Tpl::Text;
            let mut l_result__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_local__decls__str: Tpl::Text;
            let mut l_p__info__str: Tpl::Text;
            let mut l_guard__str: Tpl::Text;
            let mut l_pattern__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_pattern__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_pattern.clone())?;
            (l_guard__str, a_preText) = fun_176(Tpl::emptyTxt.clone(), i_patternGuard.clone(), a_preText.clone())?;
            l_p__info__str = dumpInfo(Tpl::emptyTxt.clone(), i_patternInfo.clone())?;
            l_local__decls__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_local__decls__str, a_preText) = lm_177(l_local__decls__str.clone(), i_localDecls.clone(), a_preText.clone())?;
            l_local__decls__str = Tpl::popIter(l_local__decls__str.clone())?;
            (l_eql__str, a_preText) = dumpMatchEquations(Tpl::emptyTxt.clone(), a_preText.clone(), i_classPart.clone())?;
            (l_result__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_result.clone())?;
            l_r__i__str = dumpInfo(Tpl::emptyTxt.clone(), i_resultInfo.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_i__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CASE(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pattern__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_guard__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_p__info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_local__decls__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_result__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_r__i__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_i__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::Case::ELSE { info: i_info, resultInfo: i_resultInfo, comment: i_comment, result: i_result, classPart: i_classPart, localDecls: i_localDecls }, a_preText) => {
            let mut l_info__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_r__i__str: Tpl::Text;
            let mut l_result__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_local__decls__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_local__decls__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_local__decls__str, a_preText) = lm_178(l_local__decls__str.clone(), i_localDecls.clone(), a_preText.clone())?;
            l_local__decls__str = Tpl::popIter(l_local__decls__str.clone())?;
            (l_eql__str, a_preText) = dumpMatchEquations(Tpl::emptyTxt.clone(), a_preText.clone(), i_classPart.clone())?;
            (l_result__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_result.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_r__i__str = dumpInfo(Tpl::emptyTxt.clone(), i_resultInfo.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), i_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ELSE(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_local__decls__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_result__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_r__i__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpMatchCase(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_c: Arc<Absyn::Case>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_179(txt.clone(), a_c.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn dumpOperator(mut in_txt: Tpl::Text, mut in_a_op: Absyn::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, Absyn::Operator::ADD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::ADD_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD_EW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::AND) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("AND()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::DIV) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DIV()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::DIV_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("DIV_EW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::EQUAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EQUAL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::GREATER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("GREATER()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::GREATEREQ) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("GREATEREQ()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::LESS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LESS()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::LESSEQ) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("LESSEQ()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::MUL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MUL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::MUL_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MUL_EW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::NEQUAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NEQUAL()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::NOT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NOT()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::OR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("OR()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::POW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("POW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::POW_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("POW_EW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::SUB) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SUB()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::SUB_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SUB_EW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::UMINUS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UMINUS()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::UMINUS_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UMINUS_EW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::UPLUS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UPLUS()")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::Operator::UPLUS_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UPLUS_EW()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_182(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WILD()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("WILD()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_183(mut in_txt: Tpl::Text, mut in_a_cref: Arc<Absyn::ComponentRef>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cref.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: i_componentRef, subscripts: i_subscripts, name: i_name }, a_preText) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_QUAL(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            (txt, a_preText) = dumpSubscripts(txt.clone(), a_preText.clone(), i_subscripts.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, a_preText) = dumpCref(txt.clone(), a_preText.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: i_subscripts, name: i_name }, a_preText) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_IDENT(\"")).clone() }))?;
            ret_1 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            (txt, a_preText) = dumpSubscripts(txt.clone(), a_preText.clone(), i_subscripts.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: i_componentRef }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("CREF_FULLYQUALIFIED(")).clone() }))?;
            (txt, a_preText) = dumpCref(txt.clone(), a_preText.clone(), i_componentRef.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ComponentRef::WILD, a_preText) => {
            let mut ret_2: bool = false;
            let mut txt = (*txt).clone();
            ret_2 = Config::acceptMetaModelicaGrammar()?;
            txt = fun_182(txt.clone(), ret_2.clone())?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::ComponentRef::ALLWILD, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ALLWILD()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpCref(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_cref: Arc<Absyn::ComponentRef>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_183(txt.clone(), a_cref.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_185(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_185(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_186(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_narg, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpNamedArg(txt.clone(), a_preText.clone(), i_narg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_186(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn lm_187(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpForIterator(txt.clone(), a_preText.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_187(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_188(mut in_txt: Tpl::Text, mut in_a_iterType: Absyn::ReductionIterType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_iterType.clone()) {
        (mut txt, Absyn::ReductionIterType::THREAD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("THREAD()")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("COMBINE()")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_189(mut in_txt: Tpl::Text, mut in_a_args: Arc<Absyn::FunctionArgs>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: i_argNames, args: i_args }, a_preText) => {
            let mut l_namedargs__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_args__str, a_preText) = lm_185(l_args__str.clone(), i_args.clone(), a_preText.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            l_namedargs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_namedargs__str, a_preText) = lm_186(l_namedargs__str.clone(), i_argNames.clone(), a_preText.clone())?;
            l_namedargs__str = Tpl::popIter(l_namedargs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FUNCTIONARGS(list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("), list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_namedargs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterType: i_iterType, iterators: i_iterators, exp: i_exp }, a_preText) => {
            let mut l_iter__type__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_exp__str, a_preText) = dumpExp(Tpl::emptyTxt.clone(), a_preText.clone(), i_exp.clone())?;
            l_iter__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (l_iter__str, a_preText) = lm_187(l_iter__str.clone(), i_iterators.clone(), a_preText.clone())?;
            l_iter__str = Tpl::popIter(l_iter__str.clone())?;
            l_iter__type__str = fun_188(Tpl::emptyTxt.clone(), i_iterType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("FOR_ITER_FARG(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__type__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", list(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpFunctionArgs(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_args: Arc<Absyn::FunctionArgs>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_189(txt.clone(), a_args.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_191(mut in_txt: Tpl::Text, mut in_a_narg: Arc<Absyn::NamedArg>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_narg.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::NamedArg { argValue: i_argValue, argName: i_argName }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NAMEDARG(\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_argName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_argValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpNamedArg(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_narg: Arc<Absyn::NamedArg>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_191(txt.clone(), a_narg.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

fn lm_193(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_preText.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }, a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (txt, a_preText) = dumpForIterator(txt.clone(), a_preText.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_preText) = lm_193(txt.clone(), rest.clone(), a_preText.clone())?;
            (txt.clone(), a_preText.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpForIterators(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    (out_txt, out_a_preText) = lm_193(out_txt.clone(), a_iters.clone(), a_preText.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok((out_txt, out_a_preText))
}

fn fun_195(mut in_txt: Tpl::Text, mut in_a_guardExp: Option<Arc<Absyn::Exp>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_guardExp.clone(), in_a_preText.clone())) {
        (txt, Some(i_x), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_x.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_196(mut in_txt: Tpl::Text, mut in_a_range: Option<Arc<Absyn::Exp>>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_range.clone(), in_a_preText.clone())) {
        (txt, Some(i_x), a_preText) => {
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOME(")).clone() }))?;
            (txt, a_preText) = dumpExp(txt.clone(), a_preText.clone(), i_x.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NONE()")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

fn fun_197(mut in_txt: Tpl::Text, mut in_a_iterator: Arc<Absyn::ForIterator>, mut in_a_preText: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iterator.clone(), in_a_preText.clone())) {
        (txt, Deref @ Absyn::ForIterator { name: i_name, range: i_range, guardExp: i_guardExp }, a_preText) => {
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut l_re: Tpl::Text;
            let mut l_ge: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_preText = (*a_preText).clone();
            (l_ge, a_preText) = fun_195(Tpl::emptyTxt.clone(), i_guardExp.clone(), a_preText.clone())?;
            (l_re, a_preText) = fun_196(Tpl::emptyTxt.clone(), i_range.clone(), a_preText.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ITERATOR(\"")).clone() }))?;
            ret_2 = (Util::escapeModelicaStringToJLString((i_name.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ge.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_re.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_preText.clone())
        },
        (txt, _, a_preText) => {
            (txt.clone(), a_preText.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_preText))
}

pub fn dumpForIterator(mut txt: Tpl::Text, mut a_preText: Tpl::Text, mut a_iterator: Arc<Absyn::ForIterator>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_preText: Tpl::Text;
    (out_txt, out_a_preText) = fun_197(txt.clone(), a_iterator.clone(), a_preText.clone())?;
    Ok((out_txt, out_a_preText))
}

pub fn errorMsg(mut txt: Tpl::Text, mut a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    Tpl::addTemplateError((a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeStr(txt.clone(), (a_errMessage.clone()).clone())?;
    Ok(out_txt)
}

