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
use crate::SimCodeFunctionUtil;
use crate::SimCodeUtil;
use crate::SimCodeVar;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::FMI;
use openmodelica_util::Flags;
use openmodelica_util::MMath;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

fn fun_52(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canGetAndSetFMUstate=\"false\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canGetAndSetFMUstate=\"true\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_53(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canSerializeFMUstate=\"false\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("canSerializeFMUstate=\"true\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_54(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("providesDirectionalDerivative=\"false\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("providesDirectionalDerivative=\"true\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn ModelExchange(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simCode.clone(), in_a_sourceFiles.clone())) {
        (txt, i_simCode @ SimCode::SimCode { modelInfo: _, .. }, a_sourceFiles) => {
            let mut ret_3: bool = false;
            let mut ret_2: bool = false;
            let mut ret_1: bool = false;
            let mut l_modelIdentifier: Tpl::Text;
            let mut txt = (*txt).clone();
            l_modelIdentifier = CodegenUtilSimulation::modelNamePrefix(Tpl::emptyTxt.clone(), i_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ModelExchange\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("modelIdentifier=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_modelIdentifier.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("needsExecutionTool=\"false\"\n")).clone(), (literal!("completedIntegratorStepNotNeeded=\"false\"\n")).clone(), (literal!("canBeInstantiatedOnlyOncePerProcess=\"false\"\n")).clone(), (literal!("canNotUseMemoryManagementFunctions=\"false\"\n")).clone()], lastHasNewLine: true }))?;
            ret_1 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_52(txt.clone(), ret_1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_2 = Flags::isSet(Flags::FMU_EXPERIMENTAL.clone())?;
            txt = fun_53(txt.clone(), ret_2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_3 = SimCodeUtil::providesDirectionalDerivative(i_simCode.clone());
            txt = fun_54(txt.clone(), ret_3.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = SourceFiles(txt.clone(), a_sourceFiles.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ModelExchange>")).clone() }))?;
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
fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_file, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<File name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_file.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" />")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn SourceFiles(mut in_txt: Tpl::Text, mut in_a_sourceFiles: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_sourceFiles.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_sourceFiles) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<SourceFiles>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_56(txt.clone(), i_sourceFiles.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</SourceFiles>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_58(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_58(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_59(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_59(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_60(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_60(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_61(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_61(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_62(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_62(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_63(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_63(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_64(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_64(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_65(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_65(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_66(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_66(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_67(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_67(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_68(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_68(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_69(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_69(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_70(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_70(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_71(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_71(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_72(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_72(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_73(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_FMUVersion: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modelInfo.clone(), in_a_FMUVersion.clone(), in_a_simCode.clone())) {
        (txt, i_modelInfo @ SimCode::ModelInfo { vars: SimCodeVar::SimVars { stringAliasVars: i_vars_stringAliasVars, stringParamVars: i_vars_stringParamVars, stringAlgVars: i_vars_stringAlgVars, boolAliasVars: i_vars_boolAliasVars, boolParamVars: i_vars_boolParamVars, boolAlgVars: i_vars_boolAlgVars, intAliasVars: i_vars_intAliasVars, intParamVars: i_vars_intParamVars, intAlgVars: i_vars_intAlgVars, aliasVars: i_vars_aliasVars, paramVars: i_vars_paramVars, discreteAlgVars: i_vars_discreteAlgVars, algVars: i_vars_algVars, derivativeVars: i_vars_derivativeVars, stateVars: i_vars_stateVars @ i_stateVars, .. }, .. }, a_FMUVersion, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ModelVariables>\n")).clone() }))?;
            System::tmpTickReset(0);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_58(txt.clone(), i_vars_stateVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_59(txt.clone(), i_vars_derivativeVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_60(txt.clone(), i_vars_algVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_61(txt.clone(), i_vars_discreteAlgVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_62(txt.clone(), i_vars_paramVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_63(txt.clone(), i_vars_aliasVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickReset(0);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_64(txt.clone(), i_vars_intAlgVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_65(txt.clone(), i_vars_intParamVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_66(txt.clone(), i_vars_intAliasVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickReset(0);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_67(txt.clone(), i_vars_boolAlgVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_68(txt.clone(), i_vars_boolParamVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_69(txt.clone(), i_vars_boolAliasVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickReset(0);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_70(txt.clone(), i_vars_stringAlgVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_71(txt.clone(), i_vars_stringParamVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_72(txt.clone(), i_vars_stringAliasVars.clone(), (a_FMUVersion.clone()).clone(), i_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            System::tmpTickReset(0);
            txt = Tpl::softNewLine(txt.clone())?;
            txt = externalFunctions(txt.clone(), i_modelInfo.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ModelVariables>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn fmiModelVariables(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: ref i_modelInfo, .. }, mut a_FMUVersion) => {
            txt = fun_73(txt.clone(), i_modelInfo.clone(), (a_FMUVersion.clone()).clone(), i_simCode.clone())?;
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
fn lm_75(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone(), in_a_stateVars.clone(), in_a_simCode.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion, a_stateVars, a_simCode) => {
            let mut txt = (*txt).clone();
            txt = ScalarVariable(txt.clone(), i_var.clone(), a_simCode.clone(), a_stateVars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_75(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_76(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_stateVars.clone(), in_a_simCode.clone(), in_a_simVar.clone())) {
        (txt, false, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_stateVars, a_simCode, a_simVar) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<!-- Index of variable = \"")).clone() }))?;
            ret_0 = SimCodeUtil::getVariableFMIIndex(a_simVar.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\" -->\n")).clone(), (literal!("<ScalarVariable\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ScalarVariableAttribute2(txt.clone(), a_simVar.clone(), a_simCode.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = ScalarVariableType2(txt.clone(), a_simVar.clone(), a_stateVars.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ScalarVariable>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_77(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode, mut in_a_exportVar: Option<Arc<DAE::ComponentRef>>, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_stateVars.clone(), in_a_simCode.clone(), in_a_exportVar.clone(), in_a_simVar.clone())) {
        (txt, false, _, _, _, a_simVar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ScalarVariable\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ScalarVariableAttribute(txt.clone(), a_simVar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = ScalarVariableType(txt.clone(), a_simVar.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ScalarVariable>")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_stateVars, a_simCode, a_exportVar, a_simVar) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = Util::isSome(a_exportVar.clone());
            txt = fun_76(txt.clone(), ret_0.clone(), a_stateVars.clone(), a_simCode.clone(), a_simVar.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_78(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode, mut in_a_exportVar: Option<Arc<DAE::ComponentRef>>, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_stateVars.clone(), in_a_simCode.clone(), in_a_exportVar.clone(), in_a_simVar.clone(), in_a_FMUVersion.clone())) {
        (txt, false, a_stateVars, a_simCode, a_exportVar, a_simVar, a_FMUVersion) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_77(txt.clone(), ret_0.clone(), a_stateVars.clone(), a_simCode.clone(), a_exportVar.clone(), a_simVar.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_79(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simCode: SimCode::SimCode, mut in_a_exportVar: Option<Arc<DAE::ComponentRef>>, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_FMUVersion: ArcStr, mut in_a_name: Arc<DAE::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_stateVars.clone(), in_a_simCode.clone(), in_a_exportVar.clone(), in_a_simVar.clone(), in_a_FMUVersion.clone(), in_a_name.clone())) {
        (txt, false, a_stateVars, a_simCode, a_exportVar, a_simVar, a_FMUVersion, a_name) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), a_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("der($dummy)")).clone());
            txt = fun_78(txt.clone(), ret_1.clone(), a_stateVars.clone(), a_simCode.clone(), a_exportVar.clone(), a_simVar.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariable(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone(), in_a_stateVars.clone(), in_a_FMUVersion.clone())) {
        (txt, i_simVar @ SimCodeVar::SimVar { type_: Deref @ DAE::Type::T_ARRAY { ty: _, .. }, .. }, a_simCode, a_stateVars, a_FMUVersion) => {
            let mut ret_0: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            ret_0 = SimCodeUtil::getScalarElements(i_simVar.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_75(txt.clone(), ret_0.clone(), (a_FMUVersion.clone()).clone(), a_stateVars.clone(), a_simCode.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, i_simVar @ SimCodeVar::SimVar { exportVar: i_exportVar, name: i_name, .. }, a_simCode, a_stateVars, a_FMUVersion) => {
            let mut ret_2: bool = false;
            let mut txt_1: Tpl::Text;
            let mut txt = (*txt).clone();
            txt_1 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_2 = stringEq((Tpl::textString(txt_1.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_79(txt.clone(), ret_2.clone(), a_stateVars.clone(), a_simCode.clone(), i_exportVar.clone(), i_simVar.clone(), (a_FMUVersion.clone()).clone(), i_name.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_81(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_comment) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("description=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_comment.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableAttribute(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone()) {
        (mut txt, SimCodeVar::SimVar { name: ref i_name, causality: mut i_causality, aliasvar: mut i_aliasvar, comment: mut i_comment, variability: mut i_variability, .. }) => {
            let mut ret_7: ArcStr = arcstr::literal!("");
            let mut txt_6: Tpl::Text;
            let mut l_caus: Tpl::Text;
            let mut l_alias: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut l_variability__: Tpl::Text;
            let mut ret_1: i32 = 0;
            let mut l_valueReference: Tpl::Text;
            ret_1 = System::tmpTick();
            l_valueReference = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_1.clone())).clone())?;
            l_variability__ = getVariability(Tpl::emptyTxt.clone(), i_variability.clone())?;
            l_description = fun_81(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            l_alias = getAliasVar(Tpl::emptyTxt.clone(), i_aliasvar.clone())?;
            l_caus = getCausality(Tpl::emptyTxt.clone(), i_causality.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("name=\"")).clone() }))?;
            txt_6 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_7 = (System::stringReplace((Tpl::textString(txt_6.clone())?).clone(), (literal!("$")).clone(), (literal!("_D_")).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_7.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("valueReference=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_valueReference.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_description.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("variability=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_variability__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("causality=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_caus.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("alias=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_alias.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getCausality(mut in_txt: Tpl::Text, mut in_a_c: Option<SimCodeVar::Causality>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_c.clone()) {
        (mut txt, Some(SimCodeVar::Causality::NONECAUS)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("none")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::OUTPUT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::INPUT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("internal")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getVariability(mut in_txt: Tpl::Text, mut in_a_variability__: Option<SimCodeVar::Variability>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_variability__.clone()) {
        (mut txt, Some(SimCodeVar::Variability::DISCRETE)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Variability::FIXED)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Variability::CONSTANT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Variability::CONTINUOUS)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continuous")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continuous")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getAliasVar(mut in_txt: Tpl::Text, mut in_a_aliasvar: SimCodeVar::AliasVariable) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_aliasvar.clone()) {
        (mut txt, SimCodeVar::AliasVariable::NOALIAS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("noAlias")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("noAlias")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_simvar.clone())) {
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Integer")).clone() }))?;
            txt = StartString(txt.clone(), a_simvar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Real")).clone() }))?;
            txt = StartString(txt.clone(), a_simvar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Boolean")).clone() }))?;
            txt = StartString(txt.clone(), a_simvar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<String")).clone() }))?;
            txt = StartString(txt.clone(), a_simvar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { path: i_path, .. }, a_simvar) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Enumeration declaredType=\"")).clone() }))?;
            ret_0 = (AbsynUtil::pathString(i_path.clone(), (literal!(".")).clone(), false, false)?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = StartString(txt.clone(), a_simvar.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKOWN_TYPE")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableType(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, ref i_simvar @ SimCodeVar::SimVar { type_: ref i_type__, .. }) => {
            txt = fun_86(txt.clone(), i_type__.clone(), i_simvar.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_88(mut in_txt: Tpl::Text, mut in_a_causality: Option<SimCodeVar::Causality>, mut in_a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_causality.clone(), in_a_type__.clone())) {
        (txt, Some(SimCodeVar::Causality::INPUT), a_type__) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initDefaultValXml(txt.clone(), a_type__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
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

fn fun_89(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>, mut in_a_type__: Arc<DAE::Type>, mut in_a_causality: Option<SimCodeVar::Causality>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone(), in_a_type__.clone(), in_a_causality.clone())) {
        (txt, Some(i_e @ Deref @ DAE::Exp::ICONST { integer: _ }), _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::RCONST { real: _ }), _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::SCONST { string: _ }), _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::BCONST { bool: _ }), _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. }), _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, a_type__, a_causality) => {
            let mut txt = (*txt).clone();
            txt = fun_88(txt.clone(), a_causality.clone(), a_type__.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn StartString(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, SimCodeVar::SimVar { type_: ref i_type__, causality: mut i_causality, initialValue: mut i_initialValue, .. }) => {
            txt = fun_89(txt.clone(), i_initialValue.clone(), i_type__.clone(), i_causality.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_91(mut in_txt: Tpl::Text, mut in_a_unit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_unit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_unit) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_unit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_92(mut in_txt: Tpl::Text, mut in_a_displayUnit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_displayUnit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_displayUnit) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("displayUnit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_displayUnit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableTypeRealAttribute(mut txt: Tpl::Text, mut a_unit: ArcStr, mut a_displayUnit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_displayUnit__: Tpl::Text;
    let mut l_unit__: Tpl::Text;
    l_unit__ = fun_91(Tpl::emptyTxt.clone(), (a_unit.clone()).clone())?;
    l_displayUnit__ = fun_92(Tpl::emptyTxt.clone(), (a_displayUnit.clone()).clone())?;
    out_txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_unit__.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_displayUnit__.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_94(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = externalFunction(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_94(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn externalFunctions(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { functions: ref i_functions, .. }) => {
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_94(txt.clone(), i_functions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn externalFunction(mut in_txt: Tpl::Text, mut in_a_fn: Arc<SimCodeFunction::Function::Function>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn.clone())) {
        (txt, Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { language: i_language, extName: i_extName, dynamicLoad: true, .. }) => {
            let mut ret_1: i32 = 0;
            let mut l_fname: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fname = CodegenCFunctions::extFunctionName(Tpl::emptyTxt.clone(), (i_extName.clone()).clone(), (i_language.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ExternalFunction\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("valueReference=\"")).clone()], lastHasNewLine: false }))?;
            ret_1 = System::tmpTick();
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
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

pub fn Implementation(mut txt: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<Implementation>\n")).clone(), (literal!("  <CoSimulation_StandAlone>\n")).clone(), (literal!("    <Capabilities\n")).clone(), (literal!("      canHandleVariableCommunicationStepSize=\"true\"\n")).clone(), (literal!("      canHandleEvents=\"true\"\n")).clone(), (literal!("      canBeInstantiatedOnlyOncePerProcess=\"false\"\n")).clone(), (literal!("      canInterpolateInputs=\"true\"\n")).clone(), (literal!("      maxOutputDerivativeOrder=\"0\"/>\n")).clone(), (literal!("  </CoSimulation_StandAlone>\n")).clone(), (literal!("</Implementation>")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

fn fun_98(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fmistruct_fmiDiscreteStates: SimCode::FmiDiscreteStates) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fmistruct_fmiDiscreteStates.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_fmistruct_fmiDiscreteStates) => {
            txt = ModelStructureDiscreteStates(txt.clone(), a_fmistruct_fmiDiscreteStates.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn ModelStructure(mut in_txt: Tpl::Text, mut in_a_fmiModelStructure: Option<SimCode::FmiModelStructure>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiModelStructure.clone()) {
        (mut txt, Some(SimCode::FmiModelStructure { fmiInitialUnknowns: mut i_fmistruct_fmiInitialUnknowns, fmiDiscreteStates: mut i_fmistruct_fmiDiscreteStates, fmiDerivatives: mut i_fmistruct_fmiDerivatives, fmiOutputs: mut i_fmistruct_fmiOutputs, .. })) => {
            let mut ret_0: bool = false;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ModelStructure>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ModelStructureOutputs(txt.clone(), i_fmistruct_fmiOutputs.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = ModelStructureDerivatives(txt.clone(), i_fmistruct_fmiDerivatives.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_0 = Flags::getConfigBool(Flags::EXPORT_CLOCKS_IN_MODELDESCRIPTION.clone())?;
            txt = fun_98(txt.clone(), ret_0.clone(), i_fmistruct_fmiDiscreteStates.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = ModelStructureInitialUnknowns(txt.clone(), i_fmistruct_fmiInitialUnknowns.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ModelStructure>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<ModelStructure>\n")).clone(), (literal!("</ModelStructure>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_100(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_snom: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_snom.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_snom) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("shiftCounter=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_snom.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_101(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_sres: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_sres.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_sres) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("resolution=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_sres.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_102(mut in_txt: Tpl::Text, mut in_a_subPartition: SimCode::SubPartition, mut in_a_bi: metamodelica::Real) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_subPartition.clone(), in_a_bi.clone()) {
        (mut txt, SimCode::SubPartition { subClock: BackendDAE::SubClock::SUBCLOCK { shift: MMath::Rational { denom: mut i_sres, nom: mut i_snom }, factor: MMath::Rational { denom: mut i_fsuper, nom: mut i_fsub }, .. }, .. }, mut a_bi) => {
            let mut ret_5: bool = false;
            let mut ret_4: bool = false;
            let mut ret_3: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ret_2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ret_1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            let mut ret_0: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<Clock><Inferred\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("interval=\"")).clone() }))?;
            ret_0 = intReal(i_fsub.clone());
            ret_1 = intReal(i_fsuper.clone());
            ret_2 = realDiv(ret_0.clone(), ret_1.clone());
            ret_3 = (a_bi.clone()) * (ret_2.clone());
            txt = Tpl::writeStr(txt.clone(), (realString(ret_3.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"\n")).clone() }))?;
            ret_4 = intGt(i_snom.clone(), 0);
            txt = fun_100(txt.clone(), ret_4.clone(), i_snom.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_5 = intGt(i_sres.clone(), 1);
            txt = fun_101(txt.clone(), ret_5.clone(), i_sres.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/></Clock>")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
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
fn lm_103(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCode::SubPartition>>, mut in_a_bi: metamodelica::Real) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_bi.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_subPartition, tail: rest }, a_bi) => {
            let mut txt = (*txt).clone();
            txt = fun_102(txt.clone(), i_subPartition.clone(), a_bi.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_103(txt.clone(), rest.clone(), a_bi.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_104(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_fsuper: i32, mut in_a_resi: i32, mut in_a_snom: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_fsuper.clone(), in_a_resi.clone(), in_a_snom.clone()) {
        (mut txt, false, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_fsuper, mut a_resi, mut a_snom) => {
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("shiftCounter=\"")).clone() }))?;
            ret_0 = intMul(a_snom.clone(), a_resi.clone());
            ret_1 = intMul(ret_0.clone(), a_fsuper.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_105(mut in_txt: Tpl::Text, mut in_a_subPartition: SimCode::SubPartition, mut in_a_resi: i32, mut in_a_bic: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_subPartition.clone(), in_a_resi.clone(), in_a_bic.clone()) {
        (mut txt, SimCode::SubPartition { subClock: BackendDAE::SubClock::SUBCLOCK { shift: MMath::Rational { denom: mut i_sres, nom: mut i_snom }, factor: MMath::Rational { denom: mut i_fsuper, nom: mut i_fsub }, .. }, .. }, mut a_resi, mut a_bic) => {
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut ret_2: bool = false;
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<Clock><Inferred\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 8 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("intervalCounter=\"")).clone() }))?;
            ret_0 = intMul(a_bic.clone(), i_fsub.clone());
            ret_1 = intMul(ret_0.clone(), i_sres.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"\n")).clone() }))?;
            ret_2 = intGt(i_snom.clone(), 0);
            txt = fun_104(txt.clone(), ret_2.clone(), i_fsuper.clone(), a_resi.clone(), i_snom.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("resolution=\"")).clone() }))?;
            ret_3 = intMul(a_resi.clone(), i_sres.clone());
            ret_4 = intMul(ret_3.clone(), i_fsuper.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_4.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("/></Clock>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::popBlock(txt.clone())?;
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
fn lm_106(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCode::SubPartition>>, mut in_a_resi: i32, mut in_a_bic: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_resi.clone(), in_a_bic.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_subPartition, tail: rest }, a_resi, a_bic) => {
            let mut txt = (*txt).clone();
            txt = fun_105(txt.clone(), i_subPartition.clone(), a_resi.clone(), a_bic.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_106(txt.clone(), rest.clone(), a_resi.clone(), a_bic.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_107(mut in_txt: Tpl::Text, mut in_a_baseClock: Arc<DAE::ClockKind>, mut in_a_subPartitions: Arc<metamodelica::List<SimCode::SubPartition>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_baseClock.clone(), in_a_subPartitions.clone())) {
        (txt, Deref @ DAE::ClockKind::REAL_CLOCK { interval: Deref @ DAE::Exp::RCONST { real: i_bi } }, a_subPartitions) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_103(txt.clone(), a_subPartitions.clone(), i_bi.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::RATIONAL_CLOCK { resolution: Deref @ DAE::Exp::ICONST { integer: i_resi }, intervalCounter: Deref @ DAE::Exp::ICONST { integer: i_bic } }, a_subPartitions) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_106(txt.clone(), a_subPartitions.clone(), i_resi.clone(), i_bic.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::REAL_CLOCK { interval: _ }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Clock><Inferred/></Clock>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: _, .. }, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Clock><Inferred/></Clock>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::ClockKind::INFERRED_CLOCK, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Clock><Inferred/></Clock>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Clock><Triggered/></Clock>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_108(mut in_txt: Tpl::Text, mut in_a_partition: SimCode::ClockedPartition) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_partition.clone()) {
        (mut txt, SimCode::ClockedPartition { subPartitions: ref i_subPartitions, baseClock: ref i_baseClock }) => {
            txt = fun_107(txt.clone(), i_baseClock.clone(), i_subPartitions.clone())?;
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
fn lm_109(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCode::ClockedPartition>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_partition, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_108(txt.clone(), i_partition.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_109(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_110(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_clocks: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_clocks.clone())) {
        (txt, Deref @ "", _) => {
            txt.clone()
        },
        (txt, _, a_clocks) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<Clocks>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_clocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</Clocks>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn TypeDefinitionsClocks(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { clockedPartitions: ref i_clockedPartitions, modelInfo: SimCode::ModelInfo { name: _, .. }, .. }) => {
            let mut str_1: ArcStr = arcstr::literal!("");
            let mut l_clocks: Tpl::Text;
            l_clocks = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_clocks = lm_109(l_clocks.clone(), i_clockedPartitions.clone())?;
            l_clocks = Tpl::popIter(l_clocks.clone())?;
            str_1 = (Tpl::textString(l_clocks.clone())?).clone();
            txt = fun_110(txt.clone(), (str_1.clone()).clone(), l_clocks.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn ModelStructureOutputs(mut in_txt: Tpl::Text, mut in_a_fmiOutputs: SimCode::FmiOutputs) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiOutputs.clone())) {
        (txt, SimCode::FmiOutputs { fmiUnknownsList: Deref @ metamodelica::List::Nil }) => {
            txt.clone()
        },
        (txt, SimCode::FmiOutputs { fmiUnknownsList: i_fmiUnknownsList }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<Outputs>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ModelStructureUnknowns(txt.clone(), i_fmiUnknownsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</Outputs>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ModelStructureDerivatives(mut in_txt: Tpl::Text, mut in_a_fmiDerivatives: SimCode::FmiDerivatives) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiDerivatives.clone())) {
        (txt, SimCode::FmiDerivatives { fmiUnknownsList: Deref @ metamodelica::List::Nil }) => {
            txt.clone()
        },
        (txt, SimCode::FmiDerivatives { fmiUnknownsList: i_fmiUnknownsList }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<Derivatives>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ModelStructureUnknowns(txt.clone(), i_fmiUnknownsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</Derivatives>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ModelStructureDiscreteStates(mut in_txt: Tpl::Text, mut in_a_fmiDiscreteStates: SimCode::FmiDiscreteStates) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiDiscreteStates.clone())) {
        (txt, SimCode::FmiDiscreteStates { fmiUnknownsList: Deref @ metamodelica::List::Nil }) => {
            txt.clone()
        },
        (txt, SimCode::FmiDiscreteStates { fmiUnknownsList: i_fmiUnknownsList }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<DiscreteStates>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ModelStructureUnknowns(txt.clone(), i_fmiUnknownsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</DiscreteStates>")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ModelStructureInitialUnknowns(mut in_txt: Tpl::Text, mut in_a_fmiInitialUnknowns: SimCode::FmiInitialUnknowns) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fmiInitialUnknowns.clone())) {
        (txt, SimCode::FmiInitialUnknowns { fmiUnknownsList: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, SimCode::FmiInitialUnknowns { fmiUnknownsList: i_fmiUnknownsList, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<InitialUnknowns>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = ModelStructureUnknowns(txt.clone(), i_fmiUnknownsList.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</InitialUnknowns>")).clone() }))?;
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
fn lm_116(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCode::FmiUnknown>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_fmiUnknown, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = FmiUnknownAttributes(txt.clone(), i_fmiUnknown.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_116(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn ModelStructureUnknowns(mut txt: Tpl::Text, mut a_fmiUnknownsList: Arc<metamodelica::List<SimCode::FmiUnknown>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_116(out_txt.clone(), a_fmiUnknownsList.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn FmiUnknownAttributes(mut in_txt: Tpl::Text, mut in_a_fmiUnknown: SimCode::FmiUnknown) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_fmiUnknown.clone()) {
        (mut txt, SimCode::FmiUnknown { dependenciesKind: ref i_dependenciesKind, dependencies: ref i_dependencies, index: mut i_index }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Unknown index=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_index.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = FmiUnknownDependencies(txt.clone(), i_dependencies.clone())?;
            txt = FmiUnknownDependenciesKind(txt.clone(), i_dependenciesKind.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" />")).clone() }))?;
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
fn lm_119(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dependency, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_dependency.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_119(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn FmiUnknownDependencies(mut txt: Tpl::Text, mut a_dependencies: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dependencies=\"")).clone() }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_119(out_txt.clone(), a_dependencies.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_121(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_dependencyKind, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_dependencyKind.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_121(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn FmiUnknownDependenciesKind(mut txt: Tpl::Text, mut a_dependenciesKind: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("dependenciesKind=\"")).clone() }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_121(out_txt.clone(), a_dependenciesKind.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    Ok(out_txt)
}

fn fun_123(mut in_txt: Tpl::Text, mut in_a_comment: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_comment) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("description=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_comment.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_124(mut in_txt: Tpl::Text, mut in_mArg: Option<i32>, mut in_a_variability: Option<SimCodeVar::Variability>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_variability.clone()) {
        (mut txt, None, mut a_variability) => {
            txt = getVariability2(txt.clone(), a_variability.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_125(mut in_txt: Tpl::Text, mut in_mArg: Option<i32>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, Some(mut i_val)) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_val.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_126(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_varKind.clone(), in_a_simCode.clone()) {
        (mut txt, BackendDAE::VarKind::CLOCKED_STATE { previousName: ref i_previousName, .. }, mut a_simCode) => {
            let mut ret_1: i32 = 0;
            let mut ret_0: SimCodeVar::SimVar = <SimCodeVar::SimVar as ::std::default::Default>::default();
            ret_0 = SimCodeUtil::cref2simvar(i_previousName.clone(), a_simCode.clone())?;
            ret_1 = SimCodeUtil::getVariableFMIIndex(ret_0.clone());
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_127(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_variability__: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_variability__.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_variability__) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("variability=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_variability__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_128(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_caus: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_caus.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_caus) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("causality=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_caus.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_129(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_clockIndex: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_clockIndex.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_clockIndex) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("clockIndex=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_clockIndex.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_130(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_previous: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_previous.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_previous) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("previous=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_previous.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_131(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_initial: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_initial.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_initial) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("initial=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_initial.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn ScalarVariableAttribute2(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_simCode.clone()) {
        (mut txt, ref i_simVar @ SimCodeVar::SimVar { causality: ref i_causality, varKind: ref i_varKind, variability: ref i_variability, comment: ref i_comment, exportVar: ref i_exportVar, .. }, mut a_simCode) => {
            let mut ret_31: bool = false;
            let mut ret_30: bool = false;
            let mut ret_29: bool = false;
            let mut ret_28: bool = false;
            let mut ret_27: bool = false;
            let mut ret_26: bool = false;
            let mut ret_25: bool = false;
            let mut ret_24: bool = false;
            let mut ret_23: bool = false;
            let mut ret_22: bool = false;
            let mut ret_21: bool = false;
            let mut ret_20: bool = false;
            let mut ret_19: bool = false;
            let mut ret_18: bool = false;
            let mut ret_17: ArcStr = arcstr::literal!("");
            let mut l_initial: Tpl::Text;
            let mut l_caus: Tpl::Text;
            let mut l_previous: Tpl::Text;
            let mut ret_13: Option<i32> = None;
            let mut l_clockIndex: Tpl::Text;
            let mut ret_11: Option<i32> = None;
            let mut l_variability__: Tpl::Text;
            let mut l_description: Tpl::Text;
            let mut ret_8: ArcStr = arcstr::literal!("");
            let mut l_valueReference: Tpl::Text;
            let mut ret_6: i32 = 0;
            let mut l_defaultValueReference: Tpl::Text;
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut txt_2: Tpl::Text;
            let mut ret_1: Arc<DAE::ComponentRef> = Arc::new(DAE::ComponentRef::WILD);
            let mut l_name: Tpl::Text;
            ret_1 = Util::getOption(i_exportVar.clone())?;
            txt_2 = CodegenUtil::crefStrNoUnderscore(Tpl::emptyTxt.clone(), ret_1.clone())?;
            ret_3 = (System::stringReplace((Tpl::textString(txt_2.clone())?).clone(), (literal!("$")).clone(), (literal!("_D_")).clone())?).clone();
            ret_4 = (Util::escapeModelicaStringToXmlString((ret_3.clone()).clone())?).clone();
            l_name = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_4.clone()).clone())?;
            ret_6 = System::tmpTick();
            l_defaultValueReference = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_6.clone())).clone())?;
            ret_8 = (SimCodeUtil::getValueReference(i_simVar.clone(), a_simCode.clone(), false)?).clone();
            l_valueReference = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_8.clone()).clone())?;
            l_description = fun_123(Tpl::emptyTxt.clone(), (i_comment.clone()).clone())?;
            ret_11 = SimCodeUtil::getClockIndex(i_simVar.clone(), a_simCode.clone())?;
            l_variability__ = fun_124(Tpl::emptyTxt.clone(), ret_11.clone(), i_variability.clone())?;
            ret_13 = SimCodeUtil::getClockIndex(i_simVar.clone(), a_simCode.clone())?;
            l_clockIndex = fun_125(Tpl::emptyTxt.clone(), ret_13.clone())?;
            l_previous = fun_126(Tpl::emptyTxt.clone(), i_varKind.clone(), a_simCode.clone())?;
            l_caus = getCausality2(Tpl::emptyTxt.clone(), i_causality.clone())?;
            ret_17 = (SimCodeUtil::getFmiInitialAttributeStr(i_simVar.clone())?).clone();
            l_initial = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_17.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("name=\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("valueReference=\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_valueReference.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\"\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_description.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_18 = stringEq((Tpl::textString(l_variability__.clone())?).clone(), (literal!("")).clone());
            ret_19 = boolNot(ret_18.clone());
            txt = fun_127(txt.clone(), ret_19.clone(), l_variability__.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_20 = stringEq((Tpl::textString(l_caus.clone())?).clone(), (literal!("")).clone());
            ret_21 = boolNot(ret_20.clone());
            txt = fun_128(txt.clone(), ret_21.clone(), l_caus.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_22 = stringEq((Tpl::textString(l_clockIndex.clone())?).clone(), (literal!("")).clone());
            ret_23 = boolNot(ret_22.clone());
            ret_24 = Flags::getConfigBool(Flags::EXPORT_CLOCKS_IN_MODELDESCRIPTION.clone())?;
            ret_25 = boolAnd(ret_23.clone(), ret_24.clone());
            txt = fun_129(txt.clone(), ret_25.clone(), l_clockIndex.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_26 = stringEq((Tpl::textString(l_previous.clone())?).clone(), (literal!("")).clone());
            ret_27 = boolNot(ret_26.clone());
            ret_28 = Flags::getConfigBool(Flags::EXPORT_CLOCKS_IN_MODELDESCRIPTION.clone())?;
            ret_29 = boolAnd(ret_27.clone(), ret_28.clone());
            txt = fun_130(txt.clone(), ret_29.clone(), l_previous.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            ret_30 = stringEq((Tpl::textString(l_initial.clone())?).clone(), (literal!("")).clone());
            ret_31 = boolNot(ret_30.clone());
            txt = fun_131(txt.clone(), ret_31.clone(), l_initial.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_133(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continuous")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getVariability2(mut in_txt: Tpl::Text, mut in_a_variability: Option<SimCodeVar::Variability>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_variability.clone()) {
        (mut txt, Some(SimCodeVar::Variability::DISCRETE)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Variability::FIXED)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fixed")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Variability::CONSTANT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Variability::CONTINUOUS)) => {
            let mut ret_0: bool = false;
            ret_0 = Flags::isSet(Flags::DUMP_FORCE_FMI_ATTRIBUTES.clone())?;
            txt = fun_133(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Variability::TUNABLE)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("tunable")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_135(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("local")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn getCausality2(mut in_txt: Tpl::Text, mut in_a_c: Option<SimCodeVar::Causality>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_c.clone()) {
        (mut txt, Some(SimCodeVar::Causality::NONECAUS)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("none")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::OUTPUT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::INPUT)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::LOCAL)) => {
            let mut ret_0: bool = false;
            ret_0 = Flags::isSet(Flags::DUMP_FORCE_FMI_ATTRIBUTES.clone())?;
            txt = fun_135(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::PARAMETER)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Causality::CALCULATED_PARAMETER)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("calculatedParameter")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_137(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_vi_numZeroCrossings: i32, mut in_a_zeroCrossings: Arc<metamodelica::List<BackendDAE::ZeroCrossing>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_vi_numZeroCrossings.clone(), in_a_zeroCrossings.clone())) {
        (txt, Deref @ "Cpp", _, a_zeroCrossings) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            ret_0 = (a_zeroCrossings.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt.clone()
        },
        (txt, _, a_vi_numZeroCrossings, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(a_vi_numZeroCrossings.clone())).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getNumberOfEventIndicators(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: SimCode::ModelInfo { varInfo: SimCode::VarInfo { numZeroCrossings: mut i_vi_numZeroCrossings, .. }, .. }, zeroCrossings: ref i_zeroCrossings, .. }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            ret_0 = (Config::simCodeTarget()?).clone();
            txt = fun_137(txt.clone(), (ret_0.clone()).clone(), i_vi_numZeroCrossings.clone(), i_zeroCrossings.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_139(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_stateVars.clone(), in_a_simvar.clone())) {
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_stateVars, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Real")).clone() }))?;
            txt = ScalarVariableTypeCommonAttribute2(txt.clone(), a_simvar.clone(), a_stateVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_stateVars, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Integer")).clone() }))?;
            txt = ScalarVariableTypeCommonAttribute2(txt.clone(), a_simvar.clone(), a_stateVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_stateVars, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Boolean")).clone() }))?;
            txt = ScalarVariableTypeCommonAttribute2(txt.clone(), a_simvar.clone(), a_stateVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_stateVars, a_simvar) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<String")).clone() }))?;
            txt = ScalarVariableTypeCommonAttribute2(txt.clone(), a_simvar.clone(), a_stateVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ENUMERATION { path: i_path, .. }, a_stateVars, a_simvar) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Enumeration declaredType=\"")).clone() }))?;
            ret_0 = (AbsynUtil::pathString(i_path.clone(), (literal!(".")).clone(), false, false)?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = ScalarVariableTypeCommonAttribute2(txt.clone(), a_simvar.clone(), a_stateVars.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("UNKOWN_TYPE")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableType2(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simvar.clone(), in_a_stateVars.clone())) {
        (txt, i_simvar @ SimCodeVar::SimVar { type_: i_type__, .. }, a_stateVars) => {
            let mut txt = (*txt).clone();
            txt = fun_139(txt.clone(), i_type__.clone(), a_stateVars.clone(), i_simvar.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn ScalarVariableTypeCommonAttribute2(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simvar.clone(), in_a_stateVars.clone())) {
        (txt, i_simvar @ SimCodeVar::SimVar { name: _, .. }, a_stateVars) => {
            let mut l_extraAttributes: Tpl::Text;
            let mut l_startString: Tpl::Text;
            let mut txt = (*txt).clone();
            l_startString = StartString2(Tpl::emptyTxt.clone(), i_simvar.clone())?;
            l_extraAttributes = DerivativeVarIndex(Tpl::emptyTxt.clone(), i_simvar.clone(), a_stateVars.clone())?;
            l_extraAttributes = MinString2(l_extraAttributes.clone(), i_simvar.clone())?;
            l_extraAttributes = MaxString2(l_extraAttributes.clone(), i_simvar.clone())?;
            l_extraAttributes = NominalString2(l_extraAttributes.clone(), i_simvar.clone())?;
            l_extraAttributes = UnitString2(l_extraAttributes.clone(), i_simvar.clone())?;
            l_extraAttributes = relativeQuantity(l_extraAttributes.clone(), i_simvar.clone())?;
            txt = Tpl::writeText(txt.clone(), l_startString.clone())?;
            txt = Tpl::writeText(txt.clone(), l_extraAttributes.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_142(mut in_txt: Tpl::Text, mut in_a_varKind: BackendDAE::VarKind, mut in_a_index: i32, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varKind.clone(), in_a_index.clone(), in_a_stateVars.clone())) {
        (txt, BackendDAE::VarKind::STATE_DER, a_index, a_stateVars) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("derivative=\"")).clone() }))?;
            ret_0 = SimCodeUtil::getStateSimVarIndexFromIndex(a_stateVars.clone(), a_index.clone())?;
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn DerivativeVarIndex(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar, mut in_a_stateVars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simvar.clone(), in_a_stateVars.clone())) {
        (txt, SimCodeVar::SimVar { index: i_index, varKind: i_varKind, .. }, a_stateVars) => {
            let mut txt = (*txt).clone();
            txt = fun_142(txt.clone(), i_varKind.clone(), i_index.clone(), a_stateVars.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_144(mut in_txt: Tpl::Text, mut in_a_initial__: Option<SimCodeVar::Initial>, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_initial__.clone(), in_a_simvar.clone()) {
        (mut txt, Some(SimCodeVar::Initial::EXACT), mut a_simvar) => {
            txt = startString3(txt.clone(), a_simvar.clone())?;
            txt.clone()
        },
        (mut txt, Some(SimCodeVar::Initial::APPROX), mut a_simvar) => {
            txt = startString3(txt.clone(), a_simvar.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn StartString2(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_simvar.clone())) {
        (txt, SimCodeVar::SimVar { aliasvar: SimCodeVar::AliasVariable::ALIAS { varName: _ }, .. }) => {
            txt.clone()
        },
        (txt, SimCodeVar::SimVar { initialValue: None, .. }) => {
            txt.clone()
        },
        (txt, i_simvar @ SimCodeVar::SimVar { causality: Some(SimCodeVar::Causality::INPUT), .. }) => {
            let mut txt = (*txt).clone();
            txt = startString3(txt.clone(), i_simvar.clone())?;
            txt.clone()
        },
        (txt, i_simvar @ SimCodeVar::SimVar { initial_: i_initial__, .. }) => {
            let mut txt = (*txt).clone();
            txt = fun_144(txt.clone(), i_initial__.clone(), i_simvar.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_146(mut in_txt: Tpl::Text, mut in_a_initialValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_initialValue.clone())) {
        (txt, Some(i_e @ Deref @ DAE::Exp::ICONST { integer: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::RCONST { real: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::SCONST { string: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::BCONST { bool: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("start=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
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

pub fn startString3(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, SimCodeVar::SimVar { initialValue: mut i_initialValue, .. }) => {
            txt = fun_146(txt.clone(), i_initialValue.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn startString2Helper(mut in_txt: Tpl::Text, mut in_a_exp: Option<Arc<DAE::Exp>>, mut in_a_type__: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone(), in_a_type__.clone())) {
        (txt, Some(i_e), _) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt.clone()
        },
        (txt, _, a_type__) => {
            let mut txt = (*txt).clone();
            txt = CodegenUtil::initDefaultValXml(txt.clone(), a_type__.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_149(mut in_txt: Tpl::Text, mut in_a_minValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_minValue.clone())) {
        (txt, Some(i_e @ Deref @ DAE::Exp::ICONST { integer: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("min=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::RCONST { real: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("min=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::SCONST { string: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("min=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::BCONST { bool: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("min=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("min=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
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

pub fn MinString2(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, SimCodeVar::SimVar { minValue: mut i_minValue, .. }) => {
            txt = fun_149(txt.clone(), i_minValue.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_151(mut in_txt: Tpl::Text, mut in_a_maxValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_maxValue.clone())) {
        (txt, Some(i_e @ Deref @ DAE::Exp::ICONST { integer: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("max=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::RCONST { real: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("max=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::SCONST { string: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("max=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::BCONST { bool: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("max=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Some(i_e @ Deref @ DAE::Exp::ENUM_LITERAL { name: _, .. })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("max=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
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

pub fn MaxString2(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, SimCodeVar::SimVar { maxValue: mut i_maxValue, .. }) => {
            txt = fun_151(txt.clone(), i_maxValue.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_153(mut in_txt: Tpl::Text, mut in_a_nominalValue: Option<Arc<DAE::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_nominalValue.clone())) {
        (txt, Some(i_e @ Deref @ DAE::Exp::RCONST { real: _ })) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("nominal=\"")).clone() }))?;
            txt = CodegenUtil::initValXml(txt.clone(), i_e.clone(), (literal!("")).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
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

pub fn NominalString2(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, SimCodeVar::SimVar { nominalValue: mut i_nominalValue, .. }) => {
            txt = fun_153(txt.clone(), i_nominalValue.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_155(mut in_txt: Tpl::Text, mut in_a_unit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_unit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_unit) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("unit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_unit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_156(mut in_txt: Tpl::Text, mut in_a_displayUnit: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_displayUnit.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_displayUnit) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("displayUnit=\"")).clone() }))?;
            ret_0 = (Util::escapeModelicaStringToXmlString((i_displayUnit.clone()).clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn UnitString2(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, SimCodeVar::SimVar { displayUnit: mut i_displayUnit, unit: mut i_unit, .. }) => {
            let mut l_displayUnitString: Tpl::Text;
            let mut l_unitString: Tpl::Text;
            l_unitString = fun_155(Tpl::emptyTxt.clone(), (i_unit.clone()).clone())?;
            l_displayUnitString = fun_156(Tpl::emptyTxt.clone(), (i_displayUnit.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_unitString.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_158(mut in_txt: Tpl::Text, mut in_a_relativeQuantity: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_relativeQuantity.clone()) {
        (mut txt, true) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" relativeQuantity=\"true\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn relativeQuantity(mut in_txt: Tpl::Text, mut in_a_simvar: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simvar.clone()) {
        (mut txt, SimCodeVar::SimVar { relativeQuantity: mut i_relativeQuantity, .. }) => {
            txt = fun_158(txt.clone(), i_relativeQuantity.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_160(mut in_txt: Tpl::Text, mut in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("1")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("0")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_161(mut in_txt: Tpl::Text, mut in_a_var: SimCodeVar::SimVar) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, SimCodeVar::SimVar { name: ref i_name, .. }) => {
            let mut ret_1: bool = false;
            let mut txt_0: Tpl::Text;
            txt_0 = CodegenUtil::crefStr(Tpl::emptyTxt.clone(), i_name.clone())?;
            ret_1 = stringEq((Tpl::textString(txt_0.clone())?).clone(), (literal!("$dummy")).clone());
            txt = fun_160(txt.clone(), ret_1.clone())?;
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
fn lm_162(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = fun_161(txt.clone(), i_var.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_162(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn statesnumwithDummy(mut txt: Tpl::Text, mut a_vars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_162(out_txt.clone(), a_vars.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn xsdateTime(mut in_txt: Tpl::Text, mut in_a_dt: Util::DateTime) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_dt.clone()) {
        (mut txt, Util::DateTime { sec: mut i_sec, min: mut i_min, hour: mut i_hour, mday: mut i_mday, mon: mut i_mon, year: mut i_year }) => {
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut ret_0: ArcStr = arcstr::literal!("");
            txt = Tpl::writeStr(txt.clone(), (intString(i_year.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            ret_0 = (SimCodeFunctionUtil::twodigit(i_mon.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            ret_1 = (SimCodeFunctionUtil::twodigit(i_mday.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("T")).clone() }))?;
            ret_2 = (SimCodeFunctionUtil::twodigit(i_hour.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            ret_3 = (SimCodeFunctionUtil::twodigit(i_min.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_3.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            ret_4 = (SimCodeFunctionUtil::twodigit(i_sec.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Z")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_165(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone()) {
        (mut txt, SimCode::ModelInfo { unitDefinitions: ref i_unitDefinitions, .. }) => {
            txt = UnitDefinitionsHelper(txt.clone(), i_unitDefinitions.clone())?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn UnitDefinitions(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::SimCode { modelInfo: mut i_modelInfo, .. }) => {
            txt = fun_165(txt.clone(), i_modelInfo.clone())?;
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
fn lm_167(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCode::UnitDefinition>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_unitDefinition, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = UnitDefinitionsHelper1(txt.clone(), i_unitDefinition.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_167(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn UnitDefinitionsHelper(mut in_txt: Tpl::Text, mut in_a_unitDefinitions: Arc<metamodelica::List<SimCode::UnitDefinition>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_unitDefinitions.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, i_unitDefinitions) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<UnitDefinitions>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_167(txt.clone(), i_unitDefinitions.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</UnitDefinitions>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn UnitDefinitionsHelper1(mut in_txt: Tpl::Text, mut in_a_unitDefinition: SimCode::UnitDefinition) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_unitDefinition.clone()) {
        (mut txt, SimCode::UnitDefinition { baseUnit: mut i_baseUnit, name: mut i_name }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Unit ")).clone() }))?;
            txt = unitDefinitionAttribute(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(">\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = baseUnitAttributes(txt.clone(), i_baseUnit.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</Unit>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_170(mut in_txt: Tpl::Text, mut in_a_unitName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_unitName.clone())) {
        (txt, Deref @ "") => {
            txt.clone()
        },
        (txt, i_unitName) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_unitName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn unitDefinitionAttribute(mut txt: Tpl::Text, mut a_unitName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_unitString: Tpl::Text;
    l_unitString = fun_170(Tpl::emptyTxt.clone(), (a_unitName.clone()).clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_unitString.clone())?;
    Ok(out_txt)
}

fn fun_172(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_s: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_s.clone()) {
        (mut txt, false, mut a_s) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("s=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_s.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_173(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_m: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_m.clone()) {
        (mut txt, false, mut a_m) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("m=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_m.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_174(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_kg: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_kg.clone()) {
        (mut txt, false, mut a_kg) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("kg=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_kg.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_175(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_A: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_A.clone()) {
        (mut txt, false, mut a_A) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("A=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_A.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_176(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_K: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_K.clone()) {
        (mut txt, false, mut a_K) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("K=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_K.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_177(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_mol: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_mol.clone()) {
        (mut txt, false, mut a_mol) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mol=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_mol.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_178(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_cd: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_cd.clone()) {
        (mut txt, false, mut a_cd) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("cd=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_cd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_179(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_factor: metamodelica::Real) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_factor.clone()) {
        (mut txt, false, mut a_factor) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("factor=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_factor.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_180(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_offset: metamodelica::Real) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_offset.clone()) {
        (mut txt, false, mut a_offset) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("offset=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_offset.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn baseUnitAttributes(mut in_txt: Tpl::Text, mut in_a_baseUnit: SimCode::BaseUnit) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_baseUnit.clone()) {
        (mut txt, SimCode::BaseUnit::BASEUNIT { offset: mut i_offset, factor: mut i_factor, cd: mut i_cd, mol: mut i_mol, K: mut i_K, A: mut i_A, kg: mut i_kg, m: mut i_m, s: mut i_s }) => {
            let mut ret_17: bool = false;
            let mut l_offset__Value: Tpl::Text;
            let mut ret_15: bool = false;
            let mut l_factor__Value: Tpl::Text;
            let mut ret_13: bool = false;
            let mut l_cd__Value: Tpl::Text;
            let mut ret_11: bool = false;
            let mut l_mol__Value: Tpl::Text;
            let mut ret_9: bool = false;
            let mut l_K__Value: Tpl::Text;
            let mut ret_7: bool = false;
            let mut l_A__Value: Tpl::Text;
            let mut ret_5: bool = false;
            let mut l_kg__Value: Tpl::Text;
            let mut ret_3: bool = false;
            let mut l_m__Value: Tpl::Text;
            let mut ret_1: bool = false;
            let mut l_s__Value: Tpl::Text;
            ret_1 = intEq(i_s.clone(), 0);
            l_s__Value = fun_172(Tpl::emptyTxt.clone(), ret_1.clone(), i_s.clone())?;
            ret_3 = intEq(i_m.clone(), 0);
            l_m__Value = fun_173(Tpl::emptyTxt.clone(), ret_3.clone(), i_m.clone())?;
            ret_5 = intEq(i_kg.clone(), 0);
            l_kg__Value = fun_174(Tpl::emptyTxt.clone(), ret_5.clone(), i_kg.clone())?;
            ret_7 = intEq(i_A.clone(), 0);
            l_A__Value = fun_175(Tpl::emptyTxt.clone(), ret_7.clone(), i_A.clone())?;
            ret_9 = intEq(i_K.clone(), 0);
            l_K__Value = fun_176(Tpl::emptyTxt.clone(), ret_9.clone(), i_K.clone())?;
            ret_11 = intEq(i_mol.clone(), 0);
            l_mol__Value = fun_177(Tpl::emptyTxt.clone(), ret_11.clone(), i_mol.clone())?;
            ret_13 = intEq(i_cd.clone(), 0);
            l_cd__Value = fun_178(Tpl::emptyTxt.clone(), ret_13.clone(), i_cd.clone())?;
            ret_15 = realAlmostEq(i_factor.clone(), metamodelica::OrderedFloat(1.0_f64), metamodelica::OrderedFloat(1e-6_f64));
            l_factor__Value = fun_179(Tpl::emptyTxt.clone(), ret_15.clone(), i_factor.clone())?;
            ret_17 = realAlmostEq(i_offset.clone(), metamodelica::OrderedFloat(0.0_f64), metamodelica::OrderedFloat(1e-6_f64));
            l_offset__Value = fun_180(Tpl::emptyTxt.clone(), ret_17.clone(), i_offset.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<BaseUnit ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_mol__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cd__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_m__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_s__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_A__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_K__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_kg__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_factor__Value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_offset__Value.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (mut txt, SimCode::BaseUnit::NOBASEUNIT) => {
            txt.clone()
        },
        (mut txt, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_182(mut in_txt: Tpl::Text, mut in_a_modelInfo: SimCode::ModelInfo, mut in_a_FMUVersion: ArcStr, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modelInfo.clone(), in_a_FMUVersion.clone(), in_a_simCode.clone()) {
        (mut txt, SimCode::ModelInfo { vars: mut i_vars @ SimCodeVar::SimVars { stateVars: _, .. }, .. }, mut a_FMUVersion, mut a_simCode) => {
            let mut ret_0: Arc<metamodelica::List<SimCodeVar::SimVar>> = metamodelica::nil();
            ret_0 = SimCodeUtil::getEnumerationTypes(i_vars.clone());
            txt = TypeDefinitionsHelper(txt.clone(), a_simCode.clone(), ret_0.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn fmiTypeDefinitions(mut in_txt: Tpl::Text, mut in_a_simCode: SimCode::SimCode, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simCode.clone(), in_a_FMUVersion.clone()) {
        (mut txt, ref i_simCode @ SimCode::SimCode { modelInfo: ref i_modelInfo, .. }, mut a_FMUVersion) => {
            txt = fun_182(txt.clone(), i_modelInfo.clone(), (a_FMUVersion.clone()).clone(), i_simCode.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_184(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_simCode.clone()) {
        (mut txt, false, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_simCode) => {
            txt = TypeDefinitionsClocks(txt.clone(), a_simCode.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_185(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_simCode: SimCode::SimCode) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_simCode.clone()) {
        (mut txt, false, mut a_simCode) => {
            let mut ret_0: bool = false;
            ret_0 = Flags::getConfigBool(Flags::EXPORT_CLOCKS_IN_MODELDESCRIPTION.clone())?;
            txt = fun_184(txt.clone(), ret_0.clone(), a_simCode.clone())?;
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
fn lm_186(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_FMUVersion.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }, a_FMUVersion) => {
            let mut txt = (*txt).clone();
            txt = TypeDefinition(txt.clone(), i_var.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_186(txt.clone(), rest.clone(), (a_FMUVersion.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_187(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_clocks: Tpl::Text, mut in_a_FMUVersion: ArcStr, mut in_a_vars: Arc<metamodelica::List<SimCodeVar::SimVar>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_clocks.clone(), in_a_FMUVersion.clone(), in_a_vars.clone())) {
        (txt, false, _, _, _) => {
            txt.clone()
        },
        (txt, _, a_clocks, a_FMUVersion, a_vars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<TypeDefinitions>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_186(txt.clone(), a_vars.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_clocks.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</TypeDefinitions>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn TypeDefinitionsHelper(mut txt: Tpl::Text, mut a_simCode: SimCode::SimCode, mut a_vars: Arc<metamodelica::List<SimCodeVar::SimVar>>, mut a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_6: bool = false;
    let mut ret_5: bool = false;
    let mut ret_4: bool = false;
    let mut ret_3: bool = false;
    let mut ret_2: i32 = 0;
    let mut ret_1: bool = false;
    let mut l_clocks: Tpl::Text;
    ret_1 = FMI::isFMIVersion10((a_FMUVersion.clone()).clone());
    l_clocks = fun_185(Tpl::emptyTxt.clone(), ret_1.clone(), a_simCode.clone())?;
    ret_2 = (a_vars.clone().len() as i32);
    ret_3 = intGt(ret_2.clone(), 0);
    ret_4 = stringEq((Tpl::textString(l_clocks.clone())?).clone(), (literal!("")).clone());
    ret_5 = boolNot(ret_4.clone());
    ret_6 = boolOr(ret_3.clone(), ret_5.clone());
    out_txt = fun_187(txt.clone(), ret_6.clone(), l_clocks.clone(), (a_FMUVersion.clone()).clone(), a_vars.clone())?;
    Ok(out_txt)
}

pub fn TypeDefinition(mut in_txt: Tpl::Text, mut in_a_simVar: SimCodeVar::SimVar, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simVar.clone(), in_a_FMUVersion.clone()) {
        (mut txt, SimCodeVar::SimVar { type_: ref i_type__, .. }, mut a_FMUVersion) => {
            txt = TypeDefinitionType(txt.clone(), i_type__.clone(), (a_FMUVersion.clone()).clone())?;
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
fn lm_190(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_name, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Item name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_190(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_191(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_name, tail: rest }) => {
            let mut x_i0: i32 = 0;
            let mut txt = (*txt).clone();
            x_i0 = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Item name=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" value=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"/>")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_191(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_192(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_names: Arc<metamodelica::List<ArcStr>>, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_names.clone(), in_a_path.clone())) {
        (txt, false, a_names, a_path) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<Type name=\"")).clone() }))?;
            ret_0 = (AbsynUtil::pathString(a_path.clone(), (literal!(".")).clone(), false, false)?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <EnumerationType>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_190(txt.clone(), a_names.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </EnumerationType>\n")).clone(), (literal!("</Type>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        (txt, _, a_names, a_path) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<SimpleType name=\"")).clone() }))?;
            ret_1 = (AbsynUtil::pathString(a_path.clone(), (literal!(".")).clone(), false, false)?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\">\n")).clone(), (literal!("  <Enumeration>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_191(txt.clone(), a_names.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("  </Enumeration>\n")).clone(), (literal!("</SimpleType>")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn TypeDefinitionType(mut in_txt: Tpl::Text, mut in_a_type__: Arc<DAE::Type>, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_type__.clone(), in_a_FMUVersion.clone())) {
        (txt, Deref @ DAE::Type::T_ENUMERATION { names: i_names, path: i_path, .. }, a_FMUVersion) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_192(txt.clone(), ret_0.clone(), i_names.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn DefaultExperiment(mut in_txt: Tpl::Text, mut in_a_simulationSettingsOpt: Option<SimCode::SimulationSettings>, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simulationSettingsOpt.clone(), in_a_FMUVersion.clone()) {
        (mut txt, Some(mut i_v), mut a_FMUVersion) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<DefaultExperiment ")).clone() }))?;
            txt = DefaultExperimentAttribute(txt.clone(), i_v.clone(), (a_FMUVersion.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/>")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_195(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_stepSize: metamodelica::Real, mut in_a_tolerance: metamodelica::Real, mut in_a_stopTime: metamodelica::Real, mut in_a_startTime: metamodelica::Real) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_stepSize.clone(), in_a_tolerance.clone(), in_a_stopTime.clone(), in_a_startTime.clone()) {
        (mut txt, false, _, mut a_tolerance, mut a_stopTime, mut a_startTime) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("startTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" stopTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_stopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" tolerance=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_tolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_stepSize, mut a_tolerance, mut a_stopTime, mut a_startTime) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("startTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_startTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" stopTime=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_stopTime.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" tolerance=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_tolerance.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\" stepSize=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(a_stepSize.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn DefaultExperimentAttribute(mut in_txt: Tpl::Text, mut in_a_simulationSettings: SimCode::SimulationSettings, mut in_a_FMUVersion: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_simulationSettings.clone(), in_a_FMUVersion.clone()) {
        (mut txt, SimCode::SimulationSettings { stepSize: mut i_stepSize, tolerance: mut i_tolerance, stopTime: mut i_stopTime, startTime: mut i_startTime, .. }, mut a_FMUVersion) => {
            let mut ret_0: bool = false;
            ret_0 = FMI::isFMIVersion20((a_FMUVersion.clone()).clone());
            txt = fun_195(txt.clone(), ret_0.clone(), i_stepSize.clone(), i_tolerance.clone(), i_stopTime.clone(), i_startTime.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt.clone()
        },
    });
    Ok(out_txt)
}

