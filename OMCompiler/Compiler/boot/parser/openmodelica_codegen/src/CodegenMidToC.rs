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
use openmodelica_backend::MidCode;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_44(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Function>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_44(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn genProgram(mut in_txt: Tpl::Text, mut in_a_p: MidCode::Program) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_p.clone()) {
        (mut txt, MidCode::Program { name: mut i_name, functions: ref i_functions }) => {
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// number of functions: ")).clone() }))?;
            ret_0 = (i_functions.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#include \"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".h\"\n")).clone(), (literal!("#include \"util/modelica.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("#include \"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_includes.h\"\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_44(txt.clone(), i_functions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_46(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarType(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_47(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_i.clone())?;
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
fn lm_48(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" *outPtr_")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_48(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn smf_49(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_it.clone())) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_50(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn smf_51(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

pub fn genFunction(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fn.clone()) {
        (mut txt, ref i_fn @ MidCode::Function { body: ref i_body, localBufPtrs: ref i_localBufPtrs, localBufs: ref i_localBufs, locals: ref i_locals, name: ref i_name, inputs: ref i_inputs, outputs: ref i_outputs, .. }) => {
            let mut txt_4: Tpl::Text;
            let mut ret_4: Arc<metamodelica::List<MidCode::Var>> = metamodelica::nil();
            let mut txt_3: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut ret_1: bool = false;
            let mut l_returnType: Tpl::Text;
            ret_1 = i_outputs.clone().is_empty();
            l_returnType = fun_46(Tpl::emptyTxt.clone(), ret_1.clone(), i_outputs.clone())?;
            txt_3 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_3 = lm_47(txt_3.clone(), i_inputs.clone())?;
            txt_3 = Tpl::popIter(txt_3.clone())?;
            ret_4 = List::restOrEmpty(i_outputs.clone())?;
            txt_4 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_4 = lm_48(txt_4.clone(), ret_4.clone())?;
            txt_4 = Tpl::popIter(txt_4.clone())?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_arguments = smf_49(l_arguments.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData_t *threadData")).clone() }))?;
            l_arguments = smf_50(l_arguments.clone(), txt_3.clone())?;
            l_arguments = smf_51(l_arguments.clone(), txt_4.clone())?;
            l_arguments = Tpl::popIter(l_arguments.clone())?;
            txt = Tpl::writeText(txt.clone(), l_returnType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 3 }))?;
            txt = genLocalDecls(txt.clone(), i_fn.clone(), i_locals.clone(), i_localBufs.clone(), i_localBufPtrs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genEntry(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genBlocks(txt.clone(), i_fn.clone(), i_body.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genExit(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = genInFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genBoxPtrFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
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
fn lm_53(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_53(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_54(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_54(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_55(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(")).clone() }))?;
            txt = varModelicaRead(txt.clone(), i_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") return 1;")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_55(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = varModelicaWrite(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
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
fn lm_58(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_58(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_59(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_59(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn smf_60(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_it.clone())) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_61(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn smf_62(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_63(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_noretcall(outVar);")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genInFunction(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fn.clone()) {
        (mut txt, MidCode::Function { name: ref i_name, outputs: ref i_outputs, inputs: ref i_inputs, .. }) => {
            let mut ret_10: bool = false;
            let mut txt_8: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<MidCode::Var>> = metamodelica::nil();
            let mut txt_7: Tpl::Text;
            let mut l_callargs: Tpl::Text;
            let mut ret_5: bool = false;
            let mut l_callretval: Tpl::Text;
            let mut l_outputLines: Tpl::Text;
            let mut l_inputLines: Tpl::Text;
            let mut l_outputDefs: Tpl::Text;
            let mut l_inputDefs: Tpl::Text;
            l_inputDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_inputDefs = lm_53(l_inputDefs.clone(), i_inputs.clone())?;
            l_inputDefs = Tpl::popIter(l_inputDefs.clone())?;
            l_outputDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_outputDefs = lm_54(l_outputDefs.clone(), i_outputs.clone())?;
            l_outputDefs = Tpl::popIter(l_outputDefs.clone())?;
            l_inputLines = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_inputLines = lm_55(l_inputLines.clone(), i_inputs.clone())?;
            l_inputLines = Tpl::popIter(l_inputLines.clone())?;
            l_outputLines = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_outputLines = lm_56(l_outputLines.clone(), i_outputs.clone())?;
            l_outputLines = Tpl::popIter(l_outputLines.clone())?;
            ret_5 = i_outputs.clone().is_empty();
            l_callretval = fun_57(Tpl::emptyTxt.clone(), ret_5.clone(), i_outputs.clone())?;
            txt_7 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_7 = lm_58(txt_7.clone(), i_inputs.clone())?;
            txt_7 = Tpl::popIter(txt_7.clone())?;
            ret_8 = List::restOrEmpty(i_outputs.clone())?;
            txt_8 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_8 = lm_59(txt_8.clone(), ret_8.clone())?;
            txt_8 = Tpl::popIter(txt_8.clone())?;
            l_callargs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_callargs = smf_60(l_callargs.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData")).clone() }))?;
            l_callargs = smf_61(l_callargs.clone(), txt_7.clone())?;
            l_callargs = smf_62(l_callargs.clone(), txt_8.clone())?;
            l_callargs = Tpl::popIter(l_callargs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("int in_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(threadData_t *threadData, type_description *inArgs, type_description *outVar)\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_inputDefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_outputDefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_inputLines.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("MMC_TRY_TOP_INTERNAL()\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_callretval.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_callargs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("MMC_CATCH_TOP(return 1)\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_outputLines.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_10 = i_outputs.clone().is_empty();
            txt = fun_63(txt.clone(), ret_10.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("fflush(NULL);\n")).clone(), (literal!("return 0;\n")).clone()], lastHasNewLine: true }))?;
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

fn fun_65(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_66(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype ")).clone() }))?;
            txt = genVarName(txt.clone(), i_i.clone())?;
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
fn lm_67(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype *out_")).clone() }))?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_67(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn smf_68(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_it.clone())) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_69(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn smf_70(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_71(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_i: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_i.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_i) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" unbox_")).clone() }))?;
            txt = genVarName(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_72(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_i.clone())?;
            txt = fun_71(txt.clone(), txt_0.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_72(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_73(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_o: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_o.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_o) => {
            let mut txt = (*txt).clone();
            txt = genVarType(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_74(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_o.clone())?;
            txt = fun_73(txt.clone(), txt_0.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_74(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_75(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype out_")).clone() }))?;
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
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

fn fun_76(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_i: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_i.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_i) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox_")).clone() }))?;
            txt = genVarName(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = varUnbox2(txt.clone(), a_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_77(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_i.clone())?;
            txt = fun_76(txt.clone(), txt_0.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_77(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out_")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, false, a_outputs) => {
            let mut ret_2: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt_1: Tpl::Text;
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt_1 = varBoxType(Tpl::emptyTxt.clone(), ret_0.clone())?;
            txt = fun_78(txt.clone(), txt_1.clone())?;
            ret_2 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_80(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unbox_")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_81(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_i.clone())?;
            txt = fun_80(txt.clone(), txt_0.clone())?;
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_81(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_82(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out_")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_83(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_o.clone())?;
            txt = fun_82(txt.clone(), txt_0.clone())?;
            txt = genVarName(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_83(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn smf_84(mut in_txt: Tpl::Text, mut in_it: Arc<Tpl::StringToken>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_it.clone())) {
        (txt, i_it) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), i_it.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn smf_85(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn smf_86(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_87(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_outputs) => {
            let mut ret_1: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out_")).clone() }))?;
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            ret_1 = listHead(a_outputs.clone())?;
            txt = varBox(txt.clone(), ret_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, false, a_outputs) => {
            let mut txt_1: Tpl::Text;
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt_1 = varBoxType(Tpl::emptyTxt.clone(), ret_0.clone())?;
            txt = fun_87(txt.clone(), txt_1.clone(), a_outputs.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_89(mut in_txt: Tpl::Text, mut in_mArg: Tpl::Text, mut in_a_o: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_o.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => {
            txt.clone()
        },
        (txt, _, a_o) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if(out_")).clone() }))?;
            txt = genVarName(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") *out_")).clone() }))?;
            txt = genVarName(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = varBox(txt.clone(), a_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_90(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = varBoxType(Tpl::emptyTxt.clone(), i_o.clone())?;
            txt = fun_89(txt.clone(), txt_0.clone(), i_o.clone())?;
            txt = lm_90(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
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

fn fun_93(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return out_")).clone() }))?;
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn genBoxPtrFunction(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fn.clone()) {
        (mut txt, MidCode::Function { name: ref i_name, inputs: ref i_inputs, outputs: ref i_outputs, .. }) => {
            let mut ret_22: bool = false;
            let mut txt_20: Tpl::Text;
            let mut ret_20: Arc<metamodelica::List<MidCode::Var>> = metamodelica::nil();
            let mut txt_18: Tpl::Text;
            let mut ret_18: bool = false;
            let mut l_boxes: Tpl::Text;
            let mut txt_15: Tpl::Text;
            let mut ret_15: Arc<metamodelica::List<MidCode::Var>> = metamodelica::nil();
            let mut txt_14: Tpl::Text;
            let mut l_callvars: Tpl::Text;
            let mut ret_12: bool = false;
            let mut l_callretval: Tpl::Text;
            let mut l_unboxes: Tpl::Text;
            let mut ret_9: bool = false;
            let mut l_boxDefs: Tpl::Text;
            let mut l_callOutDefs: Tpl::Text;
            let mut l_unboxDefs: Tpl::Text;
            let mut txt_4: Tpl::Text;
            let mut ret_4: Arc<metamodelica::List<MidCode::Var>> = metamodelica::nil();
            let mut txt_3: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut ret_1: bool = false;
            let mut l_returnType: Tpl::Text;
            ret_1 = i_outputs.clone().is_empty();
            l_returnType = fun_65(Tpl::emptyTxt.clone(), ret_1.clone())?;
            txt_3 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_3 = lm_66(txt_3.clone(), i_inputs.clone())?;
            txt_3 = Tpl::popIter(txt_3.clone())?;
            ret_4 = List::restOrEmpty(i_outputs.clone())?;
            txt_4 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_4 = lm_67(txt_4.clone(), ret_4.clone())?;
            txt_4 = Tpl::popIter(txt_4.clone())?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_arguments = smf_68(l_arguments.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData_t *threadData")).clone() }))?;
            l_arguments = smf_69(l_arguments.clone(), txt_3.clone())?;
            l_arguments = smf_70(l_arguments.clone(), txt_4.clone())?;
            l_arguments = Tpl::popIter(l_arguments.clone())?;
            l_unboxDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_unboxDefs = lm_72(l_unboxDefs.clone(), i_inputs.clone())?;
            l_unboxDefs = Tpl::popIter(l_unboxDefs.clone())?;
            l_callOutDefs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_callOutDefs = lm_74(l_callOutDefs.clone(), i_outputs.clone())?;
            l_callOutDefs = Tpl::popIter(l_callOutDefs.clone())?;
            ret_9 = i_outputs.clone().is_empty();
            l_boxDefs = fun_75(Tpl::emptyTxt.clone(), ret_9.clone(), i_outputs.clone())?;
            l_unboxes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_unboxes = lm_77(l_unboxes.clone(), i_inputs.clone())?;
            l_unboxes = Tpl::popIter(l_unboxes.clone())?;
            ret_12 = i_outputs.clone().is_empty();
            l_callretval = fun_79(Tpl::emptyTxt.clone(), ret_12.clone(), i_outputs.clone())?;
            txt_14 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_14 = lm_81(txt_14.clone(), i_inputs.clone())?;
            txt_14 = Tpl::popIter(txt_14.clone())?;
            ret_15 = List::restOrEmpty(i_outputs.clone())?;
            txt_15 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_15 = lm_83(txt_15.clone(), ret_15.clone())?;
            txt_15 = Tpl::popIter(txt_15.clone())?;
            l_callvars = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_callvars = smf_84(l_callvars.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData")).clone() }))?;
            l_callvars = smf_85(l_callvars.clone(), txt_14.clone())?;
            l_callvars = smf_86(l_callvars.clone(), txt_15.clone())?;
            l_callvars = Tpl::popIter(l_callvars.clone())?;
            ret_18 = i_outputs.clone().is_empty();
            txt_18 = fun_88(Tpl::emptyTxt.clone(), ret_18.clone(), i_outputs.clone())?;
            ret_20 = List::restOrEmpty(i_outputs.clone())?;
            txt_20 = lm_90(Tpl::emptyTxt.clone(), ret_20.clone())?;
            l_boxes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_boxes = smf_91(l_boxes.clone(), txt_18.clone())?;
            l_boxes = smf_92(l_boxes.clone(), txt_20.clone())?;
            l_boxes = Tpl::popIter(l_boxes.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#undef boxptr_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_returnType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" boxptr_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_unboxDefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_callOutDefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_boxDefs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_unboxes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_callretval.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_callvars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_boxes.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_22 = i_outputs.clone().is_empty();
            txt = fun_93(txt.clone(), ret_22.clone(), i_outputs.clone())?;
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
fn lm_95(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fn.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_local, tail: rest }, a_fn) => {
            let mut txt = (*txt).clone();
            txt = genLocalDecl(txt.clone(), a_fn.clone(), i_local.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_95(txt.clone(), rest.clone(), a_fn.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_96(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::VarBuf>>, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fn.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_local, tail: rest }, a_fn) => {
            let mut txt = (*txt).clone();
            txt = genLocalBufDecl(txt.clone(), a_fn.clone(), i_local.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_96(txt.clone(), rest.clone(), a_fn.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_97(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::VarBufPtr>>, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fn.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_local, tail: rest }, a_fn) => {
            let mut txt = (*txt).clone();
            txt = genLocalBufPtrDecl(txt.clone(), a_fn.clone(), i_local.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_97(txt.clone(), rest.clone(), a_fn.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn genLocalDecls(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_locals: Arc<metamodelica::List<MidCode::Var>>, mut a_localBufs: Arc<metamodelica::List<MidCode::VarBuf>>, mut a_localBufPtrs: Arc<metamodelica::List<MidCode::VarBufPtr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_95(out_txt.clone(), a_locals.clone(), a_fn.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_96(out_txt.clone(), a_localBufs.clone(), a_fn.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_97(out_txt.clone(), a_localBufPtrs.clone(), a_fn.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_99(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { volatile: true, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("volatile ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genLocalDecl(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_99(txt.clone(), a_var.clone())?;
    out_txt = genVarType(out_txt.clone(), a_var.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = genVarName(out_txt.clone(), a_var.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub fn genLocalBufDecl(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_var: MidCode::VarBuf) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jmp_buf ")).clone() }))?;
    out_txt = genVarBufName(out_txt.clone(), a_var.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub fn genLocalBufPtrDecl(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_var: MidCode::VarBufPtr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jmp_buf *")).clone() }))?;
    out_txt = genVarBufPtrName(out_txt.clone(), a_var.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    Ok(out_txt)
}

pub fn genEntry(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fn.clone()) {
        (mut txt, MidCode::Function { entryId: mut i_entryId, .. }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_entryId.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_104(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_outputs: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_outputs.clone())) {
        (txt, false, a_outputs) => {
            let mut ret_0: MidCode::Var = <MidCode::Var as ::std::default::Default>::default();
            let mut txt = (*txt).clone();
            ret_0 = listHead(a_outputs.clone())?;
            txt = genVarName(txt.clone(), ret_0.clone())?;
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
fn lm_105(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: rest }) => {
            let mut l_outPtrName: Tpl::Text;
            let mut txt = (*txt).clone();
            l_outPtrName = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("outPtr_")).clone() }))?;
            l_outPtrName = genVarName(l_outPtrName.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_outPtrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" != NULL)\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_outPtrName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = genVarName(txt.clone(), i_v.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_105(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn genExit(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fn.clone()) {
        (mut txt, MidCode::Function { exitId: mut i_exitId, outputs: ref i_outputs, .. }) => {
            let mut ret_2: Arc<metamodelica::List<MidCode::Var>> = metamodelica::nil();
            let mut ret_1: bool = false;
            let mut l_returnString: Tpl::Text;
            ret_1 = i_outputs.clone().is_empty();
            l_returnString = fun_104(Tpl::emptyTxt.clone(), ret_1.clone(), i_outputs.clone())?;
            txt = genLabel(txt.clone(), i_exitId.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(": // exit block\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            ret_2 = List::restOrEmpty(i_outputs.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_105(txt.clone(), ret_2.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_returnString.clone())?;
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
fn lm_107(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Block>>, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fn.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_block, tail: rest }, a_fn) => {
            let mut txt = (*txt).clone();
            txt = genBlock(txt.clone(), a_fn.clone(), i_block.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_107(txt.clone(), rest.clone(), a_fn.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn genBlocks(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_body: Arc<metamodelica::List<MidCode::Block>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_107(out_txt.clone(), a_body.clone(), a_fn.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_109(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Stmt>>, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_fn.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_stmt, tail: rest }, a_fn) => {
            let mut txt = (*txt).clone();
            txt = genStmt(txt.clone(), a_fn.clone(), i_stmt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_109(txt.clone(), rest.clone(), a_fn.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_110(mut in_txt: Tpl::Text, mut in_a_block: MidCode::Block, mut in_a_fn: MidCode::Function) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_block.clone(), in_a_fn.clone()) {
        (mut txt, MidCode::Block { terminator: mut i_terminator, stmts: ref i_stmts, id: mut i_id }, mut a_fn) => {
            txt = genLabel(txt.clone(), i_id.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(":\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_109(txt.clone(), i_stmts.clone(), a_fn.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = genTerminator(txt.clone(), a_fn.clone(), i_terminator.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genBlock(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_block: MidCode::Block) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_110(txt.clone(), a_block.clone(), a_fn.clone())?;
    Ok(out_txt)
}

pub fn genLabel(mut txt: Tpl::Text, mut a_i: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label_")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (intString(a_i.clone())).clone())?;
    Ok(out_txt)
}

pub fn genVarName(mut in_txt: Tpl::Text, mut in_a_v: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_v.clone()) {
        (mut txt, MidCode::Var { name: mut i_name, .. }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genVarBufName(mut in_txt: Tpl::Text, mut in_a_v: MidCode::VarBuf) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_v.clone()) {
        (mut txt, MidCode::VarBuf { name: mut i_name }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genVarBufPtrName(mut in_txt: Tpl::Text, mut in_a_v: MidCode::VarBufPtr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_v.clone()) {
        (mut txt, MidCode::VarBufPtr { name: mut i_name }) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_116(mut in_txt: Tpl::Text, mut in_a_stmt: MidCode::Stmt) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_stmt.clone()) {
        (mut txt, MidCode::Stmt::NOP) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; // NOP")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Stmt::ASSIGN { src: mut i_rvalue, dest: MidCode::Var { ty: _, name: mut i_dest__name, .. } }) => {
            txt = Tpl::writeStr(txt.clone(), (i_dest__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = genRValue(txt.clone(), i_rvalue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genStmt(mut txt: Tpl::Text, mut a_fn: MidCode::Function, mut a_stmt: MidCode::Stmt) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_116(txt.clone(), a_stmt.clone())?;
    Ok(out_txt)
}

fn fun_118(mut in_txt: Tpl::Text, mut in_a_value: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_value.clone()) {
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

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_119(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_element, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genVarName(txt.clone(), i_element.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_119(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
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

fn smf_126(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_127(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_elementargs: Tpl::Text, mut in_a_metatypeCtor: Tpl::Text, mut in_a_metatypeSlots: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_elementargs.clone(), in_a_metatypeCtor.clone(), in_a_metatypeSlots.clone())) {
        (txt, i_ty @ Deref @ DAE::Type::T_METARECORD { path: _, .. }, a_elementargs, a_metatypeCtor, a_metatypeSlots) => {
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeText(Tpl::emptyTxt.clone(), a_metatypeSlots.clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+1")).clone() }))?;
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt_2 = genTypeUnderscorePath(txt_2.clone(), i_ty.clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__desc")).clone() }))?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_arguments = smf_120(l_arguments.clone(), txt_1.clone())?;
            l_arguments = smf_121(l_arguments.clone(), a_metatypeCtor.clone())?;
            l_arguments = smf_122(l_arguments.clone(), txt_2.clone())?;
            l_arguments = smf_123(l_arguments.clone(), a_elementargs.clone())?;
            l_arguments = Tpl::popIter(l_arguments.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_box(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_elementargs, a_metatypeCtor, a_metatypeSlots) => {
            let mut l_arguments: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_arguments = smf_124(l_arguments.clone(), a_metatypeSlots.clone())?;
            l_arguments = smf_125(l_arguments.clone(), a_metatypeCtor.clone())?;
            l_arguments = smf_126(l_arguments.clone(), a_elementargs.clone())?;
            l_arguments = Tpl::popIter(l_arguments.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_box(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn genRValue(mut in_txt: Tpl::Text, mut in_a_rvalue: MidCode::RValue) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rvalue.clone())) {
        (txt, MidCode::RValue::VARIABLE { src: MidCode::Var { ty: _, name: i_src__name, .. } }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_src__name.clone()).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::BINARYOP { rsrc: i_rsrc @ MidCode::Var { ty: Deref @ DAE::Type::T_STRING { varLst: _ }, name: _, .. }, lsrc: i_lsrc @ MidCode::Var { ty: Deref @ DAE::Type::T_STRING { varLst: _ }, name: _, .. }, op: i_op }) => {
            let mut txt = (*txt).clone();
            txt = genStringBinaryop(txt.clone(), i_op.clone(), i_lsrc.clone(), i_rsrc.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::BINARYOP { rsrc: MidCode::Var { ty: _, name: i_rsrc__name, .. }, lsrc: MidCode::Var { ty: _, name: i_lsrc__name, .. }, op: MidCode::BinaryOp::POW }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pow(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_lsrc__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_rsrc__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::BINARYOP { rsrc: MidCode::Var { ty: _, name: i_rsrc__name, .. }, lsrc: MidCode::Var { ty: _, name: i_lsrc__name, .. }, op: i_op }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_lsrc__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = binaryopToString(txt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_rsrc__name.clone()).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNARYOP { src: i_src, op: MidCode::UnaryOp::BOX }) => {
            let mut txt = (*txt).clone();
            txt = varBox(txt.clone(), i_src.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNARYOP { src: i_src, op: MidCode::UnaryOp::UNBOX }) => {
            let mut txt = (*txt).clone();
            txt = varUnbox(txt.clone(), i_src.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNARYOP { src: i_src, op: i_op_1 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = genVarType(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") ")).clone() }))?;
            txt = unaryopToString(txt.clone(), i_op_1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALINTEGER { value: i_value }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_value.clone())).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALBOOLEAN { value: i_value_1 }) => {
            let mut txt = (*txt).clone();
            txt = fun_118(txt.clone(), i_value_1.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALREAL { value: i_value_2 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (realString(i_value_2.clone())).clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALSTRING { value: i_value_3 }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_scon(\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToCString((i_value_3.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\")")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::LITERALMETATYPE { ty: i_ty, elements: i_elements }) => {
            let mut l_elementargs: Tpl::Text;
            let mut l_metatypeCtor: Tpl::Text;
            let mut ret_2: i32 = 0;
            let mut l_metatypeSlots: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = (i_elements.clone().len() as i32);
            l_metatypeSlots = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_2.clone())).clone())?;
            l_metatypeCtor = genTypeCtorIndex(Tpl::emptyTxt.clone(), i_elements.clone(), i_ty.clone())?;
            l_elementargs = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elementargs = lm_119(l_elementargs.clone(), i_elements.clone())?;
            l_elementargs = Tpl::popIter(l_elementargs.clone())?;
            txt = fun_127(txt.clone(), i_ty.clone(), l_elementargs.clone(), l_metatypeCtor.clone(), l_metatypeSlots.clone())?;
            txt.clone()
        },
        (txt, MidCode::RValue::METAFIELD { ty: _, index: i_index, src: i_src }) => {
            let mut ret_5: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MMC_FETCH(MMC_OFFSET(MMC_UNTAGPTR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("),")).clone() }))?;
            ret_5 = intAdd(i_index.clone(), 1);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_5.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::UNIONTYPEVARIANT { src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(MMC_HDRCTOR(MMC_GETHDR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")) - 3)")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::ISSOME { src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(0==MMC_HDRSLOTS(MMC_GETHDR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")) ? 0 : 1)")).clone() }))?;
            txt.clone()
        },
        (txt, MidCode::RValue::ISCONS { src: i_src }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(MMC_GETHDR(")).clone() }))?;
            txt = genVarName(txt.clone(), i_src.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") == MMC_CONSHDR)")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn binaryopToString(mut in_txt: Tpl::Text, mut in_a_op: MidCode::BinaryOp) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, MidCode::BinaryOp::ADD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::SUB) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::MUL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::DIV) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::LESS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::LESSEQ) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<=")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::GREATER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::GREATEREQ) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">=")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::EQUAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("==")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::BinaryOp::NEQUAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!=")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn unaryopToString(mut in_txt: Tpl::Text, mut in_a_op: MidCode::UnaryOp) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, MidCode::UnaryOp::MOVE) => {
            txt.clone()
        },
        (mut txt, MidCode::UnaryOp::UMINUS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::UnaryOp::NOT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("!")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genStringBinaryop(mut in_txt: Tpl::Text, mut in_a_op: MidCode::BinaryOp, mut in_a_lsrc: MidCode::Var, mut in_a_rsrc: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone(), in_a_lsrc.clone(), in_a_rsrc.clone()) {
        (mut txt, MidCode::BinaryOp::ADD, mut a_lsrc, mut a_rsrc) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("stringAppend(")).clone() }))?;
            txt = genVarName(txt.clone(), a_lsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = genVarName(txt.clone(), a_rsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_op, mut a_lsrc, mut a_rsrc) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0 ")).clone() }))?;
            txt = binaryopToString(txt.clone(), i_op.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" stringCompare(")).clone() }))?;
            txt = genVarName(txt.clone(), a_lsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = genVarName(txt.clone(), a_rsrc.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_132(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<(i32, i32)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: (i_from, i_to), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_from.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(": goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_to.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_132(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_133(mut in_txt: Tpl::Text, mut in_a_outputs: Arc<metamodelica::List<MidCode::OutVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_outputs.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: MidCode::OutVar::OUT_WILD, tail: _ }) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: MidCode::OutVar::OUT_VAR { var: i_var }, tail: _ }) => {
            let mut txt = (*txt).clone();
            txt = genVarName(txt.clone(), i_var.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_134(mut in_txt: Tpl::Text, mut in_a_builtin: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_builtin.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threadData")).clone() }))?;
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
fn lm_135(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = genVarName(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_135(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_136(mut in_txt: Tpl::Text, mut in_a_o: MidCode::OutVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_o.clone()) {
        (mut txt, MidCode::OutVar::OUT_VAR { var: mut i_var }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("&")).clone() }))?;
            txt = genVarName(txt.clone(), i_var.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("NULL")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_137(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<MidCode::OutVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_o, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_136(txt.clone(), i_o.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_137(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn smf_138(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn smf_139(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn smf_140(mut in_txt: Tpl::Text, mut in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn fun_141(mut in_txt: Tpl::Text, mut in_a_builtin: bool, mut in_a_func: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_builtin.clone(), in_a_func.clone())) {
        (txt, false, a_func) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_")).clone() }))?;
            txt = underscorePath(txt.clone(), a_func.clone())?;
            txt.clone()
        },
        (txt, _, a_func) => {
            let mut txt = (*txt).clone();
            txt = identBuiltinCall(txt.clone(), a_func.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_142(mut in_txt: Tpl::Text, mut in_a_terminator: MidCode::Terminator, mut in_a_exitId: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_terminator.clone(), in_a_exitId.clone()) {
        (mut txt, MidCode::Terminator::RETURN, mut a_exitId) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), a_exitId.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; // exit label")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::GOTO { next: mut i_label }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_label.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::BRANCH { onFalse: mut i_labelFalse, onTrue: mut i_labelTrue, condition: mut i_condition }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = genVarName(txt.clone(), i_condition.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_labelTrue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("goto ")).clone() }))?;
            txt = genLabel(txt.clone(), i_labelFalse.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::SWITCH { cases: ref i_cases, condition: mut i_condition }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("switch (")).clone() }))?;
            txt = genVarName(txt.clone(), i_condition.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("){\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_132(txt.clone(), i_cases.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::CALL { next: mut i_next, outputs: ref i_outputs, inputs: ref i_inputs, builtin: mut i_builtin, func: ref i_func }, _) => {
            let mut txt_4: Tpl::Text;
            let mut ret_4: Arc<metamodelica::List<MidCode::OutVar>> = metamodelica::nil();
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut l_arguments: Tpl::Text;
            let mut l_returnAssignment: Tpl::Text;
            l_returnAssignment = fun_133(Tpl::emptyTxt.clone(), i_outputs.clone())?;
            txt_2 = fun_134(Tpl::emptyTxt.clone(), i_builtin.clone())?;
            txt_3 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_3 = lm_135(txt_3.clone(), i_inputs.clone())?;
            txt_3 = Tpl::popIter(txt_3.clone())?;
            ret_4 = List::restOrEmpty(i_outputs.clone())?;
            txt_4 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt_4 = lm_137(txt_4.clone(), ret_4.clone())?;
            txt_4 = Tpl::popIter(txt_4.clone())?;
            l_arguments = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(",\n")).clone(), (literal!("    ")).clone()], lastHasNewLine: false })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_arguments = smf_138(l_arguments.clone(), txt_2.clone())?;
            l_arguments = smf_139(l_arguments.clone(), txt_3.clone())?;
            l_arguments = smf_140(l_arguments.clone(), txt_4.clone())?;
            l_arguments = Tpl::popIter(l_arguments.clone())?;
            txt = Tpl::writeText(txt.clone(), l_returnAssignment.clone())?;
            txt = fun_141(txt.clone(), i_builtin.clone(), i_func.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arguments.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("goto ")).clone()], lastHasNewLine: false }))?;
            txt = genLabel(txt.clone(), i_next.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::LONGJMP, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("longjmp(*threadData->mmc_jumper,1);")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::PUSHJMP { next: mut i_next, new_buf: mut i_new__buf, old_buf: mut i_old__buf }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("// PUSHJMP\n")).clone() }))?;
            txt = genVarBufPtrName(txt.clone(), i_old__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" = threadData->mmc_jumper;\n")).clone(), (literal!("threadData->mmc_jumper = &")).clone()], lastHasNewLine: false }))?;
            txt = genVarBufName(txt.clone(), i_new__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("setjmp(&")).clone()], lastHasNewLine: false }))?;
            txt = genVarBufName(txt.clone(), i_new__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(");\n")).clone(), (literal!("goto ")).clone()], lastHasNewLine: false }))?;
            txt = genLabel(txt.clone(), i_next.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, MidCode::Terminator::POPJMP { next: mut i_next, old_buf: mut i_old__buf }, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// POPJMP\n")).clone(), (literal!("threadData->mmc_jumper = ")).clone()], lastHasNewLine: false }))?;
            txt = genVarBufPtrName(txt.clone(), i_old__buf.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("goto ")).clone()], lastHasNewLine: false }))?;
            txt = genLabel(txt.clone(), i_next.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn genTerminator(mut in_txt: Tpl::Text, mut in_a_fn: MidCode::Function, mut in_a_terminator: MidCode::Terminator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fn.clone(), in_a_terminator.clone()) {
        (mut txt, MidCode::Function { exitId: mut i_exitId, entryId: _, body: _, outputs: _, inputs: _, locals: _, .. }, mut a_terminator) => {
            txt = fun_142(txt.clone(), a_terminator.clone(), i_exitId.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_144(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_boolean")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_real")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_string")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METARECORD { path: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATYPE { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_UNKNOWN) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unknown")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAPOLYMORPHIC { name: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_ metapolymorphic")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_ metauniontype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ANYTYPE { anyClassType: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_anytype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_TUPLE { types: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error_tuple")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("notimplemented")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn genVarType(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { ty: ref i_ty, name: _, .. }) => {
            txt = fun_144(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_mArg: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, 0) => {
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

fn fun_147(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_elements: Arc<metamodelica::List<MidCode::Var>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_elements.clone())) {
        (txt, Deref @ DAE::Type::T_METARECORD { index: i_index, .. }, _) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = intAdd(i_index.clone(), 3);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("2")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }, a_elements) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            ret_1 = (a_elements.clone().len() as i32);
            txt = fun_146(txt.clone(), ret_1.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn genTypeCtorIndex(mut txt: Tpl::Text, mut a_elements: Arc<metamodelica::List<MidCode::Var>>, mut a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_147(txt.clone(), a_ty.clone(), a_elements.clone())?;
    Ok(out_txt)
}

pub fn genTypeUnderscorePath(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_METARECORD { path: i_path, .. }) => {
            let mut txt = (*txt).clone();
            txt = underscorePath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { varLst: _, complexClassType: ClassInf::State::RECORD { path: i_path }, .. }) => {
            let mut txt = (*txt).clone();
            txt = underscorePath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("error: genTypeUnderscorePath")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_150(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_metatype")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn varBoxType(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { ty: ref i_ty, name: _, .. }) => {
            txt = fun_150(txt.clone(), i_ty.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_152(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_rcon(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_string(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_box(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn varBox(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { ty: ref i_ty, name: mut i_name, .. }) => {
            txt = fun_152(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_154(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_INTEGER { varLst: _ } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_ENUMERATION { index: _, .. } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_BOOL { varLst: _ } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METABOXED { ty: Deref @ DAE::Type::T_REAL { varLst: _ } }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_real(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_string(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn varUnbox(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { ty: ref i_ty, name: mut i_name, .. }) => {
            txt = fun_154(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_156(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_real(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_unbox_string(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn varUnbox2(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { ty: ref i_ty, name: mut i_name, .. }) => {
            txt = fun_156(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_158(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_integer(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_integer(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_real(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_string(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_integer(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("read_modelica_metatype(&inArgs, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
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

pub fn varModelicaRead(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { ty: ref i_ty, name: mut i_name, .. }) => {
            txt = fun_158(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_160(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_integer(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_integer(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_real(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_string(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { index: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_integer(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_COMPLEX { complexClassType: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAUNIONTYPE { paths: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METALIST { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAARRAY { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METAOPTION { ty: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_METATUPLE { types: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("write_modelica_metatype(outVar, &")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
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

pub fn varModelicaWrite(mut in_txt: Tpl::Text, mut in_a_var: MidCode::Var) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, MidCode::Var { ty: ref i_ty, name: mut i_name, .. }) => {
            txt = fun_160(txt.clone(), i_ty.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn identName(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn identBuiltinCall(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "clock" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_clock")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "anyString" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_anyString")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "fail" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MMC_THROW_INTERNAL")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: Deref @ "intMod" }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelica_mod_integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn replaceDotAndUnderscore(mut in_txt: Tpl::Text, mut in_a_str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_str.clone()) {
        (mut txt, mut i_name) => {
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_str__underscores: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut l_str__dots: Tpl::Text;
            ret_1 = (System::stringReplace((i_name.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
            l_str__dots = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_1.clone()).clone())?;
            ret_3 = (System::stringReplace((Tpl::textString(l_str__dots.clone())?).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            l_str__underscores = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_3.clone()).clone())?;
            ret_4 = (System::unquoteIdentifier((Tpl::textString(l_str__underscores.clone())?).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn underscorePath(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::QUALIFIED { path: i_path, name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = replaceDotAndUnderscore(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = underscorePath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::IDENT { name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            txt = replaceDotAndUnderscore(txt.clone(), (i_name_1.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = underscorePath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

