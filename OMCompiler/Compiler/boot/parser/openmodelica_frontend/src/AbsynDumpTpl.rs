// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::AbsynUtil;
use crate::Dump;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;

pub fn dump(in_txt: Tpl::Text, in_a_program: Absyn::Program, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_program.clone(), in_a_options.clone())) {
        (txt, Absyn::PROGRAM { classes: Deref @ metamodelica::List::Nil, .. }, _) => txt.clone(),
        (txt, Absyn::PROGRAM { classes: i_classes, within_: i_within__ }, a_options) => {
            let mut l_cls__str: Tpl::Text;
            let mut l_within__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_within__str = dumpWithin(Tpl::emptyTxt.clone(), i_within__.clone())?;
            l_cls__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\n")).clone(), (literal!("\\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_cls__str = lm_9(l_cls__str.clone(), i_classes.clone(), a_options.clone())?;
            l_cls__str = Tpl::popIter(l_cls__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_within__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cls__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgorithm(in_txt: Tpl::Text, in_a_alg: Arc<Absyn::Algorithm>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone())) {
        (txt, Deref @ Absyn::ALG_ASSIGN { value: i_value, assignComponent: i_assignComponent }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpLhsExp(Tpl::emptyTxt.clone(), i_assignComponent.clone())?;
            l_rhs__str = dumpExp(Tpl::emptyTxt.clone(), i_value.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" := ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_IF { elseBranch: i_elseBranch, elseIfAlgorithmBranch: i_elseIfAlgorithmBranch, trueBranch: i_trueBranch, ifExp: i_ifExp }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_if__str = dumpAlgorithmBranch(Tpl::emptyTxt.clone(), i_ifExp.clone(), i_trueBranch.clone(), (literal!("if")).clone(), (literal!("then")).clone())?;
            l_elseif__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elseif__str = lm_137(l_elseif__str.clone(), i_elseIfAlgorithmBranch.clone())?;
            l_elseif__str = Tpl::popIter(l_elseif__str.clone())?;
            l_else__branch__str = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_elseBranch.clone())?;
            l_else__str = fun_138(Tpl::emptyTxt.clone(), l_else__branch__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_if__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elseif__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end if")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_FOR { forBody: i_forBody, iterators: i_iterators }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_iter__str = dumpForIterators(Tpl::emptyTxt.clone(), i_iterators.clone())?;
            l_body__str = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_forBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_PARFOR { parforBody: i_parforBody, iterators: i_iterators }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_iter__str = dumpForIterators(Tpl::emptyTxt.clone(), i_iterators.clone())?;
            l_body__str = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_parforBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parfor ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end parfor")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_WHILE { whileBody: i_whileBody, boolExpr: i_boolExpr }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_while__str = dumpAlgorithmBranch(Tpl::emptyTxt.clone(), i_boolExpr.clone(), i_whileBody.clone(), (literal!("while")).clone(), (literal!("loop")).clone())?;
            txt = Tpl::writeText(txt.clone(), l_while__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end while")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_WHEN_A { elseWhenAlgorithmBranch: i_elseWhenAlgorithmBranch, whenBody: i_whenBody, boolExpr: i_boolExpr }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_when__str = dumpAlgorithmBranch(Tpl::emptyTxt.clone(), i_boolExpr.clone(), i_whenBody.clone(), (literal!("when")).clone(), (literal!("then")).clone())?;
            l_elsewhen__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elsewhen__str = lm_139(l_elsewhen__str.clone(), i_elseWhenAlgorithmBranch.clone())?;
            l_elsewhen__str = Tpl::popIter(l_elsewhen__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_when__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elsewhen__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_NORETCALL { functionArgs: i_functionArgs, functionCall: i_functionCall }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = dumpCref(Tpl::emptyTxt.clone(), i_functionCall.clone())?;
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_RETURN) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_BREAK) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("break")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_FAILURE { equ: i_equ }) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arg__str = fun_140(Tpl::emptyTxt.clone(), i_equ.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("failure(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_TRY { elseBody: i_elseBody, body: i_body }) => {
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arg1 = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_body.clone())?;
            l_arg2 = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_elseBody.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("try\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_arg1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_arg2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end try;")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALG_CONTINUE) => {
            let mut i_elseBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionCall: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_whenBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_whileBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_boolExpr: Arc<Absyn::Exp>;
            let mut i_parforBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_forBody: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_elseBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_elseIfAlgorithmBranch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut i_trueBranch: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_arg2: Tpl::Text;
            let mut l_arg1: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_while__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continue")).clone() }))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgorithmBranch(txt: Tpl::Text, a_cond: Arc<Absyn::Exp>, a_body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, a_header: ArcStr, a_exec__str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_body__str: Tpl::Text;
    let mut l_cond__str: Tpl::Text;
    l_cond__str = dumpExp(Tpl::emptyTxt.clone(), a_cond.clone())?;
    l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_body__str = lm_142(l_body__str.clone(), a_body.clone())?;
    l_body__str = Tpl::popIter(l_body__str.clone())?;
    out_txt = Tpl::writeStr(txt.clone(), (a_header.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_cond__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_exec__str.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_body__str.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpAlgorithmItem(in_txt: Tpl::Text, in_a_alg: Arc<Absyn::AlgorithmItem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_alg.clone())) {
        (txt, Deref @ Absyn::ALGORITHMITEM { comment: i_comment, algorithm_: i_algorithm__, .. }) => {
            let mut i_comment_1: ArcStr;
            let mut ret_2: ArcStr;
            let mut l_cmt__str: Tpl::Text;
            let mut l_alg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_alg__str = dumpAlgorithm(Tpl::emptyTxt.clone(), i_algorithm__.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_alg__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALGORITHMITEMCOMMENT { comment: i_comment_1 }) => {
            let mut i_comment: Option<Arc<Absyn::Comment>>;
            let mut i_algorithm__: Arc<Absyn::Algorithm>;
            let mut ret_2: ArcStr;
            let mut l_cmt__str: Tpl::Text;
            let mut l_alg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ABS_INDENT { width: 0 }))?;
            ret_2 = (System::trimWhitespace((i_comment_1.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgorithmItems(txt: Tpl::Text, a_algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_134(out_txt.clone(), a_algs.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpAnnotation(in_txt: Tpl::Text, in_a_ann: Arc<Absyn::Annotation>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann.clone())) {
        (txt, Deref @ Absyn::ANNOTATION { elementArgs: Deref @ metamodelica::List::Nil }) => {
            let mut i_elementArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("annotation()")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ANNOTATION { elementArgs: i_elementArgs }) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("annotation(\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() }))?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt, txt_0) = dumpElementArgList(txt.clone(), i_elementArgs.clone(), txt_0.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAnnotationOpt(in_txt: Tpl::Text, in_a_oann: Option<Arc<Absyn::Annotation>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_oann.clone()) {
        (mut txt, Some(mut i_ann)) => {
            txt = dumpAnnotation(txt.clone(), i_ann.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpAnnotationOptSpace(in_txt: Tpl::Text, in_a_oann: Option<Arc<Absyn::Annotation>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_oann.clone()) {
        (mut txt, Some(mut i_ann)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = dumpAnnotation(txt.clone(), i_ann.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpArrayDimOpt(in_txt: Tpl::Text, in_a_arraydim: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_arraydim.clone()) {
        (mut txt, Some(mut i_ad)) => {
            txt = dumpSubscripts(txt.clone(), i_ad.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpClass(txt: Tpl::Text, a_cls: Arc<Absyn::Class>, a_sc: ArcStr, a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = dumpClassElement(txt.clone(), a_cls.clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), (literal!("")).clone(), (a_sc.clone()).clone(), a_options.clone())?;
    Ok(out_txt)
}

pub fn dumpClassDef(in_txt: Tpl::Text, in_a_cdef: Arc<Absyn::ClassDef>, in_a_cls__name: ArcStr, in_a_commentsBeforeEnd: Arc<metamodelica::List<ArcStr>>, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cdef.clone(), in_a_cls__name.clone(), in_a_commentsBeforeEnd.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::PARTS { classParts: i_classParts, comment: i_comment, ann: i_ann, typeVars: i_typeVars, .. }, a_cls__name, a_commentsBeforeEnd, a_options) => {
            let mut i_vars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionName: Arc<Absyn::Path>;
            let mut i_functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_enumLiterals: Arc<Absyn::EnumDef>;
            let mut i_baseClassName: ArcStr;
            let mut i_modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_comment_1: Option<Arc<Absyn::Comment>>;
            let mut i_arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut l_vars__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut l_funcs__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_mod__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tvs__str = fun_21(Tpl::emptyTxt.clone(), i_typeVars.clone())?;
            ret_2 = i_ann.clone().reverse();
            l_ann__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\\n")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_ann__str = lm_22(l_ann__str.clone(), ret_2.clone())?;
            l_ann__str = Tpl::popIter(l_ann__str.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_23(l_body__str.clone(), i_classParts.clone(), a_options.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cls__name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_tvs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = fun_24(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_25(txt.clone(), a_commentsBeforeEnd.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_cls__name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::DERIVED { comment: i_comment_1, arguments: i_arguments, typeSpec: i_typeSpec, attributes: i_attributes }, a_cls__name, a_commentsBeforeEnd, _) => {
            let mut a_options: Dump::DumpOptions;
            let mut i_vars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionName: Arc<Absyn::Path>;
            let mut i_functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_enumLiterals: Arc<Absyn::EnumDef>;
            let mut i_baseClassName: ArcStr;
            let mut i_modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_comment: Option<ArcStr>;
            let mut i_ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut l_vars__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut l_funcs__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_mod__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_attr__str = dumpElementAttr(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            l_ty__str = dumpTypeSpec(Tpl::emptyTxt.clone(), i_typeSpec.clone())?;
            l_mod__str = fun_26(Tpl::emptyTxt.clone(), i_arguments.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment_1.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cls__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_attr__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = lm_27(txt.clone(), a_commentsBeforeEnd.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CLASS_EXTENDS { baseClassName: i_baseClassName, ann: i_ann, comment: i_comment, modifications: i_modifications, parts: i_parts }, a_cls__name, a_commentsBeforeEnd, a_options) => {
            let mut i_vars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionName: Arc<Absyn::Path>;
            let mut i_functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_enumLiterals: Arc<Absyn::EnumDef>;
            let mut i_comment_1: Option<Arc<Absyn::Comment>>;
            let mut i_arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut i_classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut l_vars__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut l_funcs__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_mod__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_28(l_body__str.clone(), i_parts.clone(), a_options.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_mod__str = fun_29(Tpl::emptyTxt.clone(), i_modifications.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            ret_8 = i_ann.clone().reverse();
            l_ann__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\\n")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_ann__str = lm_30(l_ann__str.clone(), ret_8.clone())?;
            l_ann__str = Tpl::popIter(l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("extends ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_baseClassName.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = fun_31(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = lm_32(txt.clone(), a_commentsBeforeEnd.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_cls__name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ENUMERATION { comment: i_comment_1, enumLiterals: i_enumLiterals }, a_cls__name, a_commentsBeforeEnd, _) => {
            let mut a_options: Dump::DumpOptions;
            let mut i_vars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionName: Arc<Absyn::Path>;
            let mut i_functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_baseClassName: ArcStr;
            let mut i_modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut i_classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_comment: Option<ArcStr>;
            let mut i_ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut l_vars__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut l_funcs__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_mod__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_enum__str = dumpEnumDef(Tpl::emptyTxt.clone(), i_enumLiterals.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment_1.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cls__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = enumeration(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_enum__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = lm_33(txt.clone(), a_commentsBeforeEnd.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::OVERLOAD { comment: i_comment_1, functionNames: i_functionNames }, a_cls__name, a_commentsBeforeEnd, _) => {
            let mut a_options: Dump::DumpOptions;
            let mut i_vars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionName: Arc<Absyn::Path>;
            let mut i_enumLiterals: Arc<Absyn::EnumDef>;
            let mut i_baseClassName: ArcStr;
            let mut i_modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut i_classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_comment: Option<ArcStr>;
            let mut i_ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut l_vars__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut l_funcs__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_mod__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_funcs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_funcs__str = lm_34(l_funcs__str.clone(), i_functionNames.clone())?;
            l_funcs__str = Tpl::popIter(l_funcs__str.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment_1.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cls__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = $overload(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_funcs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = lm_35(txt.clone(), a_commentsBeforeEnd.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::PDER { vars: i_vars, functionName: i_functionName, .. }, a_cls__name, a_commentsBeforeEnd, _) => {
            let mut a_options: Dump::DumpOptions;
            let mut i_functionNames: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_enumLiterals: Arc<Absyn::EnumDef>;
            let mut i_baseClassName: ArcStr;
            let mut i_modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_comment_1: Option<Arc<Absyn::Comment>>;
            let mut i_arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut i_classParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            let mut i_comment: Option<ArcStr>;
            let mut i_ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut l_vars__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut l_funcs__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut ret_8: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_mod__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut ret_2: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fn__str = dumpPath(Tpl::emptyTxt.clone(), i_functionName.clone())?;
            l_vars__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_vars__str = lm_36(l_vars__str.clone(), i_vars.clone())?;
            l_vars__str = Tpl::popIter(l_vars__str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cls__name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = der(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fn__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_vars__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = lm_37(txt.clone(), a_commentsBeforeEnd.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassDefSpacing(in_txt: Tpl::Text, in_a_cdef: Arc<Absyn::ClassDef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cdef.clone())) {
        (txt, Deref @ Absyn::PARTS { typeVars: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CLASS_EXTENDS { baseClassName: _, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassElement(in_txt: Tpl::Text, in_a_cls: Arc<Absyn::Class>, in_a_final__str: ArcStr, in_a_redecl__str: ArcStr, in_a_repl__str: ArcStr, in_a_io__str: ArcStr, in_a_sc: ArcStr, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cls.clone(), in_a_final__str.clone(), in_a_redecl__str.clone(), in_a_repl__str.clone(), in_a_io__str.clone(), in_a_sc.clone(), in_a_options.clone())) {
        (txt, i_cls @ Deref @ Absyn::CLASS { commentsAfterEnd: i_commentsAfterEnd, commentsBeforeClass: i_commentsBeforeClass, commentsBeforeEnd: i_commentsBeforeEnd, name: i_name, body: i_body, .. }, a_final__str, a_redecl__str, a_repl__str, a_io__str, a_sc, a_options) => {
            let mut txt_3: Tpl::Text;
            let mut l_beforeComment: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_header__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_header__str = dumpClassHeader(Tpl::emptyTxt.clone(), i_cls.clone(), (a_final__str.clone()).clone(), (a_redecl__str.clone()).clone(), (a_repl__str.clone()).clone(), (a_io__str.clone()).clone())?;
            l_body__str = dumpClassDef(Tpl::emptyTxt.clone(), i_body.clone(), (i_name.clone()).clone(), i_commentsBeforeEnd.clone(), a_options.clone())?;
            l_beforeComment = fun_15(Tpl::emptyTxt.clone(), i_commentsBeforeClass.clone())?;
            txt = Tpl::writeText(txt.clone(), l_beforeComment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_header__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_sc.clone()).clone())?;
            txt_3 = fun_17(Tpl::emptyTxt.clone(), i_commentsAfterEnd.clone())?;
            txt = smf_18(txt.clone(), txt_3.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassHeader(in_txt: Tpl::Text, in_a_cls: Arc<Absyn::Class>, in_a_final__str: ArcStr, in_a_redecl__str: ArcStr, in_a_repl__str: ArcStr, in_a_io__str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cls.clone(), in_a_final__str.clone(), in_a_redecl__str.clone(), in_a_repl__str.clone(), in_a_io__str.clone())) {
        (txt, i_cls @ Deref @ Absyn::CLASS { restriction: i_restriction, .. }, a_final__str, a_redecl__str, a_repl__str, a_io__str) => {
            let mut l_pref__str: Tpl::Text;
            let mut l_res__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_res__str = dumpRestriction(Tpl::emptyTxt.clone(), i_restriction.clone())?;
            l_pref__str = dumpClassPrefixes(Tpl::emptyTxt.clone(), i_cls.clone(), (a_final__str.clone()).clone(), (a_redecl__str.clone()).clone(), (a_repl__str.clone()).clone(), (a_io__str.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_pref__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_res__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassPart(in_txt: Tpl::Text, in_a_class__part: Arc<Absyn::ClassPart>, in_a_idx: i32, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_class__part.clone(), in_a_idx.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::PUBLIC { contents: i_contents }, a_idx, a_options) => {
            let mut i_externalDecl: Arc<Absyn::ExternalDecl>;
            let mut i_annotation__: Option<Arc<Absyn::Annotation>>;
            let mut i_contents_3: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_contents_2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_contents_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_section__str = fun_51(Tpl::emptyTxt.clone(), a_idx.clone())?;
            l_el__str = dumpElementItems(Tpl::emptyTxt.clone(), i_contents.clone(), (literal!("")).clone(), true, a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_section__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::PROTECTED { contents: i_contents }, _, a_options) => {
            let mut a_idx: i32;
            let mut i_externalDecl: Arc<Absyn::ExternalDecl>;
            let mut i_annotation__: Option<Arc<Absyn::Annotation>>;
            let mut i_contents_3: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_contents_2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_contents_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_el__str = dumpElementItems(Tpl::emptyTxt.clone(), i_contents.clone(), (literal!("")).clone(), true, a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("protected\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CONSTRAINTS { contents: i_contents_1 }, _, _) => {
            let mut a_idx: i32;
            let mut a_options: Dump::DumpOptions;
            let mut i_externalDecl: Arc<Absyn::ExternalDecl>;
            let mut i_annotation__: Option<Arc<Absyn::Annotation>>;
            let mut i_contents_3: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_contents_2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("constraint\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_52(txt.clone(), i_contents_1.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQUATIONS { contents: i_contents_2 }, _, _) => {
            let mut a_idx: i32;
            let mut a_options: Dump::DumpOptions;
            let mut i_externalDecl: Arc<Absyn::ExternalDecl>;
            let mut i_annotation__: Option<Arc<Absyn::Annotation>>;
            let mut i_contents_3: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_contents_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("equation\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_53(txt.clone(), i_contents_2.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::INITIALEQUATIONS { contents: i_contents_2 }, _, _) => {
            let mut a_idx: i32;
            let mut a_options: Dump::DumpOptions;
            let mut i_externalDecl: Arc<Absyn::ExternalDecl>;
            let mut i_annotation__: Option<Arc<Absyn::Annotation>>;
            let mut i_contents_3: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_contents_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("initial equation\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_54(txt.clone(), i_contents_2.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALGORITHMS { contents: i_contents_3 }, _, _) => {
            let mut a_idx: i32;
            let mut a_options: Dump::DumpOptions;
            let mut i_externalDecl: Arc<Absyn::ExternalDecl>;
            let mut i_annotation__: Option<Arc<Absyn::Annotation>>;
            let mut i_contents_2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_contents_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("algorithm\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_55(txt.clone(), i_contents_3.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::INITIALALGORITHMS { contents: i_contents_3 }, _, _) => {
            let mut a_idx: i32;
            let mut a_options: Dump::DumpOptions;
            let mut i_externalDecl: Arc<Absyn::ExternalDecl>;
            let mut i_annotation__: Option<Arc<Absyn::Annotation>>;
            let mut i_contents_2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_contents_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("initial algorithm\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_56(txt.clone(), i_contents_3.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EXTERNAL { externalDecl: i_externalDecl, annotation_: i_annotation__ }, _, _) => {
            let mut a_idx: i32;
            let mut a_options: Dump::DumpOptions;
            let mut i_contents_3: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_contents_2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_contents_1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_contents: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            let mut l_ann__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_section__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ann__str = fun_57(Tpl::emptyTxt.clone(), i_annotation__.clone())?;
            txt = fun_64(txt.clone(), i_externalDecl.clone(), l_ann__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassPrefixes(txt: Tpl::Text, a_cls: Arc<Absyn::Class>, a_final__str: ArcStr, a_redecl__str: ArcStr, a_repl__str: ArcStr, a_io__str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_44(txt.clone(), a_cls.clone(), (a_redecl__str.clone()).clone(), (a_repl__str.clone()).clone(), (a_io__str.clone()).clone())?;
    Ok(out_txt)
}

pub fn dumpCodeNode(in_txt: Tpl::Text, in_a_code: Arc<Absyn::CodeNode>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_code.clone())) {
        (txt, Deref @ Absyn::C_TYPENAME { path: i_path }) => {
            let mut i_modification: Arc<Absyn::Modification>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_element: Arc<Absyn::Element>;
            let mut i_algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_boolean: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::C_VARIABLENAME { componentRef: i_componentRef }) => {
            let mut i_modification: Arc<Absyn::Modification>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_element: Arc<Absyn::Element>;
            let mut i_algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_boolean: bool;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::C_CONSTRAINTSECTION { equationItemLst: i_equationItemLst, boolean: i_boolean }) => {
            let mut i_modification: Arc<Absyn::Modification>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_element: Arc<Absyn::Element>;
            let mut i_algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_initial__str = fun_169(Tpl::emptyTxt.clone(), i_boolean.clone())?;
            l_eql__str = dumpEquationItems(Tpl::emptyTxt.clone(), i_equationItemLst.clone())?;
            txt = Tpl::writeText(txt.clone(), l_initial__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("constraint\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::C_EQUATIONSECTION { equationItemLst: i_equationItemLst, boolean: i_boolean }) => {
            let mut i_modification: Arc<Absyn::Modification>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_element: Arc<Absyn::Element>;
            let mut i_algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_initial__str = fun_170(Tpl::emptyTxt.clone(), i_boolean.clone())?;
            l_eql__str = dumpEquationItems(Tpl::emptyTxt.clone(), i_equationItemLst.clone())?;
            txt = Tpl::writeText(txt.clone(), l_initial__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("equation\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::C_ALGORITHMSECTION { algorithmItemLst: i_algorithmItemLst, boolean: i_boolean }) => {
            let mut i_modification: Arc<Absyn::Modification>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_element: Arc<Absyn::Element>;
            let mut i_equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_initial__str = fun_171(Tpl::emptyTxt.clone(), i_boolean.clone())?;
            l_algs__str = dumpAlgorithmItems(Tpl::emptyTxt.clone(), i_algorithmItemLst.clone())?;
            txt = Tpl::writeText(txt.clone(), l_initial__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("algorithm\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_algs__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::C_ELEMENT { element: i_element }) => {
            let mut i_modification: Arc<Absyn::Modification>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_boolean: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpElement(txt.clone(), i_element.clone(), Dump::defaultDumpOptions.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::C_EXPRESSION { exp: i_exp }) => {
            let mut i_modification: Arc<Absyn::Modification>;
            let mut i_element: Arc<Absyn::Element>;
            let mut i_algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_boolean: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::C_MODIFICATION { modification: i_modification }) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_element: Arc<Absyn::Element>;
            let mut i_algorithmItemLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut i_equationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_boolean: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_algs__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_initial__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpModification(txt.clone(), i_modification.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpComment(in_txt: Tpl::Text, in_a_cmt: Arc<Absyn::Comment>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cmt.clone())) {
        (txt, Deref @ Absyn::COMMENT { annotation_: i_annotation__, comment: i_comment }) => {
            let mut txt = (*txt).clone();
            txt = dumpStringCommentOption(txt.clone(), i_comment.clone())?;
            txt = dumpAnnotationOptSpace(txt.clone(), i_annotation__.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCommentOpt(in_txt: Tpl::Text, in_a_ocmt: Option<Arc<Absyn::Comment>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_ocmt.clone()) {
        (mut txt, Some(mut i_cmt)) => {
            txt = dumpComment(txt.clone(), i_cmt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpComponent(in_txt: Tpl::Text, in_a_comp: Absyn::Component) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_comp.clone()) {
        (mut txt, Absyn::COMPONENT { name: mut i_name, modification: mut i_modification, arrayDim: ref i_arrayDim }) => {
            let mut l_mod__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            l_dim__str = dumpSubscripts(Tpl::emptyTxt.clone(), i_arrayDim.clone())?;
            l_mod__str = fun_119(Tpl::emptyTxt.clone(), i_modification.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpComponentCondition(in_txt: Tpl::Text, in_a_cond: Option<Arc<Absyn::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_cond.clone()) {
        (mut txt, Some(mut i_cexp)) => {
            let mut l_exp__str: Tpl::Text;
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_cexp.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpComponentItem(in_txt: Tpl::Text, in_a_comp: Arc<Absyn::ComponentItem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comp.clone())) {
        (txt, Deref @ Absyn::COMPONENTITEM { comment: i_comment, condition: i_condition, component: i_component }) => {
            let mut l_cmt: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_comp__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_comp__str = dumpComponent(Tpl::emptyTxt.clone(), i_component.clone())?;
            l_cond__str = dumpComponentCondition(Tpl::emptyTxt.clone(), i_condition.clone())?;
            l_cmt = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_comp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpConstrainClass(in_txt: Tpl::Text, in_a_cc: Arc<Absyn::ConstrainClass>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cc.clone())) {
        (txt, Deref @ Absyn::CONSTRAINCLASS { comment: i_comment, elementSpec: Deref @ Absyn::EXTENDS { elementArg: i_el, path: i_p, .. } }) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_p.clone())?;
            l_el__str = fun_116(Tpl::emptyTxt.clone(), i_el.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constrainedby ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_el__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpCref(in_txt: Tpl::Text, in_a_cref: Arc<Absyn::ComponentRef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cref.clone())) {
        (txt, Deref @ Absyn::CREF_QUAL { componentRef: i_componentRef, subscripts: i_subscripts, name: i_name }) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpSubscripts(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CREF_IDENT { subscripts: i_subscripts, name: i_name }) => {
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = dumpSubscripts(txt.clone(), i_subscripts.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: i_componentRef }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_name: ArcStr;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::WILD) => {
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_name: ArcStr;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Config::acceptMetaModelicaGrammar()?;
            txt = fun_186(txt.clone(), ret_0.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALLWILD) => {
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_name: ArcStr;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpDirection(in_txt: Tpl::Text, in_a_dir: Absyn::Direction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_dir.clone()) {
        (mut txt, Absyn::BIDIR) => txt.clone(),
        (mut txt, Absyn::INPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::OUTPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::INPUT_OUTPUT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input output ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpEach(in_txt: Tpl::Text, in_a_each: Absyn::Each) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_each.clone()) {
        (mut txt, Absyn::EACH) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("each ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpElement(in_txt: Tpl::Text, in_a_elem: Arc<Absyn::Element>, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elem.clone(), in_a_options.clone())) {
        (txt, i_elem @ Deref @ Absyn::ELEMENT { constrainClass: i_constrainClass, specification: i_specification, innerOuter: i_innerOuter, redeclareKeywords: i_redeclareKeywords, finalPrefix: i_finalPrefix, info: i_info }, a_options) => {
            let mut i_string: ArcStr;
            let mut i_optName: Option<ArcStr>;
            let mut i_name: ArcStr;
            let mut i_args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut ret_5: bool;
            let mut l_args__str: Tpl::Text;
            let mut ret_3: bool;
            let mut ret_2: bool;
            let mut ret_1: bool;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Dump::boolUnparseFileFromInfo(i_info.clone(), a_options.clone())?;
            ret_1 = AbsynUtil::isClassdef(i_elem.clone());
            ret_2 = boolNot(ret_1.clone());
            ret_3 = boolOr(ret_0.clone(), ret_2.clone());
            txt = fun_78(txt.clone(), ret_3.clone(), i_constrainClass.clone(), a_options.clone(), i_specification.clone(), i_innerOuter.clone(), i_redeclareKeywords.clone(), i_finalPrefix.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::DEFINEUNIT { name: i_name, args: i_args, .. }, _) => {
            let mut a_options: Dump::DumpOptions;
            let mut i_string: ArcStr;
            let mut i_optName: Option<ArcStr>;
            let mut i_constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            let mut i_specification: Arc<Absyn::ElementSpec>;
            let mut i_innerOuter: Absyn::InnerOuter;
            let mut i_redeclareKeywords: Option<Absyn::RedeclareKeywords>;
            let mut i_finalPrefix: bool;
            let mut i_elem: Arc<Absyn::Element>;
            let mut i_info: SourceInfo;
            let mut ret_5: bool;
            let mut l_args__str: Tpl::Text;
            let mut ret_3: bool;
            let mut ret_2: bool;
            let mut ret_1: bool;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            l_args__str = fun_80(Tpl::emptyTxt.clone(), i_args.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("defineunit ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::TEXT { string: i_string, optName: i_optName, info: i_info }, a_options) => {
            let mut i_name: ArcStr;
            let mut i_args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut i_constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            let mut i_specification: Arc<Absyn::ElementSpec>;
            let mut i_innerOuter: Absyn::InnerOuter;
            let mut i_redeclareKeywords: Option<Absyn::RedeclareKeywords>;
            let mut i_finalPrefix: bool;
            let mut i_elem: Arc<Absyn::Element>;
            let mut ret_5: bool;
            let mut l_args__str: Tpl::Text;
            let mut ret_3: bool;
            let mut ret_2: bool;
            let mut ret_1: bool;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_5 = Dump::boolUnparseFileFromInfo(i_info.clone(), a_options.clone())?;
            txt = fun_82(txt.clone(), ret_5.clone(), (i_string.clone()).clone(), i_info.clone(), i_optName.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementArg(in_txt: Tpl::Text, in_a_earg: Arc<Absyn::ElementArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_earg.clone())) {
        (txt, Deref @ Absyn::MODIFICATION { comment: i_comment, modification: i_modification, path: i_path, finalPrefix: i_finalPrefix, eachPrefix: i_eachPrefix, .. }) => {
            let mut i_comment_1: ArcStr;
            let mut i_constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            let mut i_elementSpec: Arc<Absyn::ElementSpec>;
            let mut i_redeclareKeywords: Absyn::RedeclareKeywords;
            let mut l_cc__str: Tpl::Text;
            let mut l_elem__str: Tpl::Text;
            let mut l_eredecl__str: Tpl::Text;
            let mut l_repl__str: Tpl::Text;
            let mut l_redecl__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut l_each__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_mod__str = fun_94(Tpl::emptyTxt.clone(), i_modification.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_each__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_final__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::REDECLARATION { constrainClass: i_constrainClass, elementSpec: i_elementSpec, redeclareKeywords: i_redeclareKeywords, finalPrefix: i_finalPrefix, eachPrefix: i_eachPrefix, .. }) => {
            let mut i_comment_1: ArcStr;
            let mut i_comment: Option<ArcStr>;
            let mut i_modification: Option<Arc<Absyn::Modification>>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_cc__str: Tpl::Text;
            let mut l_elem__str: Tpl::Text;
            let mut l_eredecl__str: Tpl::Text;
            let mut l_repl__str: Tpl::Text;
            let mut l_redecl__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut l_each__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_redecl__str = dumpRedeclare(Tpl::emptyTxt.clone(), i_redeclareKeywords.clone())?;
            l_repl__str = dumpReplaceable(Tpl::emptyTxt.clone(), i_redeclareKeywords.clone())?;
            l_eredecl__str = Tpl::writeText(Tpl::emptyTxt.clone(), l_redecl__str.clone())?;
            l_eredecl__str = Tpl::writeText(l_eredecl__str.clone(), l_each__str.clone())?;
            l_elem__str = dumpElementSpec(Tpl::emptyTxt.clone(), i_elementSpec.clone(), (Tpl::textString(l_final__str.clone())?).clone(), (Tpl::textString(l_eredecl__str.clone())?).clone(), (Tpl::textString(l_repl__str.clone())?).clone(), (literal!("")).clone(), Dump::defaultDumpOptions.clone())?;
            l_cc__str = fun_95(Tpl::emptyTxt.clone(), i_constrainClass.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elem__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cc__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ELEMENTARGCOMMENT { comment: i_comment_1 }) => {
            let mut i_constrainClass: Option<Arc<Absyn::ConstrainClass>>;
            let mut i_elementSpec: Arc<Absyn::ElementSpec>;
            let mut i_redeclareKeywords: Absyn::RedeclareKeywords;
            let mut i_comment: Option<ArcStr>;
            let mut i_modification: Option<Arc<Absyn::Modification>>;
            let mut i_path: Arc<Absyn::Path>;
            let mut i_finalPrefix: bool;
            let mut i_eachPrefix: Absyn::Each;
            let mut l_cc__str: Tpl::Text;
            let mut l_elem__str: Tpl::Text;
            let mut l_eredecl__str: Tpl::Text;
            let mut l_repl__str: Tpl::Text;
            let mut l_redecl__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut l_each__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ABS_INDENT { width: 0 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_comment_1.clone()).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementArgList(txt: Tpl::Text, a_elementArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, a_separator: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_separator: Tpl::Text;
    let mut ret_0: Arc<metamodelica::List<(Arc<Absyn::ElementArg>, bool)>>;
    ret_0 = Dump::shouldSeparateAfterElementArg(a_elementArgs.clone());
    out_txt = lm_92(txt.clone(), ret_0.clone(), a_separator.clone())?;
    out_a_separator = a_separator.clone();
    Ok((out_txt, out_a_separator))
}

pub fn dumpElementAttr(in_txt: Tpl::Text, in_a_attr: Absyn::ElementAttributes) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attr.clone()) {
        (mut txt, Absyn::ATTR { direction: mut i_direction, variability: mut i_variability, isField: mut i_isField, parallelism: mut i_parallelism, streamPrefix: mut i_streamPrefix, flowPrefix: mut i_flowPrefix, .. }) => {
            let mut l_dir__str: Tpl::Text;
            let mut l_var__str: Tpl::Text;
            let mut l_field__str: Tpl::Text;
            let mut l_par__str: Tpl::Text;
            let mut l_stream__str: Tpl::Text;
            let mut l_flow__str: Tpl::Text;
            l_flow__str = fun_108(Tpl::emptyTxt.clone(), i_flowPrefix.clone())?;
            l_stream__str = fun_109(Tpl::emptyTxt.clone(), i_streamPrefix.clone())?;
            l_par__str = dumpParallelism(Tpl::emptyTxt.clone(), i_parallelism.clone())?;
            l_field__str = dumpIsField(Tpl::emptyTxt.clone(), i_isField.clone())?;
            l_var__str = dumpVariability(Tpl::emptyTxt.clone(), i_variability.clone())?;
            l_dir__str = dumpDirection(Tpl::emptyTxt.clone(), i_direction.clone())?;
            txt = Tpl::writeText(txt.clone(), l_flow__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_stream__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_par__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_field__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dir__str.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpElementAttrDim(in_txt: Tpl::Text, in_a_attr: Absyn::ElementAttributes) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attr.clone()) {
        (mut txt, Absyn::ATTR { arrayDim: ref i_arrayDim, .. }) => {
            txt = dumpSubscripts(txt.clone(), i_arrayDim.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpElementItem(in_txt: Tpl::Text, in_a_eitem: Arc<Absyn::ElementItem>, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eitem.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::ELEMENTITEM { element: i_element }, a_options) => {
            let mut i_comment: ArcStr;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            txt = dumpElement(txt.clone(), i_element.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::LEXER_COMMENT { comment: i_comment }, _) => {
            let mut a_options: Dump::DumpOptions;
            let mut i_element: Arc<Absyn::Element>;
            let mut ret_0: ArcStr;
            let mut txt = (*txt).clone();
            ret_0 = (System::trimWhitespace((i_comment.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementItemPreSpacing(txt: Tpl::Text, a_curSpacing: ArcStr, a_prevSpacing: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_70(txt.clone(), (a_prevSpacing.clone()).clone(), (a_curSpacing.clone()).clone())?;
    Ok(out_txt)
}

pub fn dumpElementItemSpacing(in_txt: Tpl::Text, in_a_item: Arc<Absyn::ElementItem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_item.clone())) {
        (txt, Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { class_: Deref @ Absyn::CLASS { body: i_cdef, .. }, .. }, .. } }) => {
            let mut txt = (*txt).clone();
            txt = dumpClassDefSpacing(txt.clone(), i_cdef.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementItems(in_txt: Tpl::Text, in_a_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, in_a_prevSpacing: ArcStr, in_a_first: bool, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_items.clone(), in_a_prevSpacing.clone(), in_a_first.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_item, tail: i_rest__items }, a_prevSpacing, a_first, a_options) => {
            let mut l_post__spacing: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_item__str: Tpl::Text;
            let mut l_pre__spacing: Tpl::Text;
            let mut l_spacing: Tpl::Text;
            let mut txt = (*txt).clone();
            l_spacing = dumpElementItemSpacing(Tpl::emptyTxt.clone(), i_item.clone())?;
            l_pre__spacing = fun_66(Tpl::emptyTxt.clone(), a_first.clone(), (a_prevSpacing.clone()).clone(), l_spacing.clone())?;
            l_item__str = dumpElementItem(Tpl::emptyTxt.clone(), i_item.clone(), a_options.clone())?;
            l_rest__str = dumpElementItems(Tpl::emptyTxt.clone(), i_rest__items.clone(), (Tpl::textString(l_spacing.clone())?).clone(), false, a_options.clone())?;
            l_post__spacing = fun_67(Tpl::emptyTxt.clone(), l_rest__str.clone(), l_spacing.clone())?;
            txt = Tpl::writeText(txt.clone(), l_pre__spacing.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_item__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_post__spacing.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = fun_68(txt.clone(), l_rest__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementSpec(in_txt: Tpl::Text, in_a_elem: Arc<Absyn::ElementSpec>, in_a_final: ArcStr, in_a_redecl: ArcStr, in_a_repl: ArcStr, in_a_io: ArcStr, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elem.clone(), in_a_final.clone(), in_a_redecl.clone(), in_a_repl.clone(), in_a_io.clone(), in_a_options.clone())) {
        (txt, Deref @ Absyn::CLASSDEF { class_: i_class__, .. }, a_final, a_redecl, a_repl, a_io, a_options) => {
            let mut i_comment: Option<Arc<Absyn::Comment>>;
            let mut i_import__: Absyn::Import;
            let mut i_components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_annotationOpt: Option<Arc<Absyn::Annotation>>;
            let mut i_elementArg: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_path: Arc<Absyn::Path>;
            let mut l_imp__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            let mut l_comps__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_bc__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpClassElement(txt.clone(), i_class__.clone(), (a_final.clone()).clone(), (a_redecl.clone()).clone(), (a_repl.clone()).clone(), (a_io.clone()).clone(), (literal!("")).clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EXTENDS { annotationOpt: i_annotationOpt, elementArg: i_elementArg, path: i_path }, _, _, _, _, _) => {
            let mut a_final: ArcStr;
            let mut a_redecl: ArcStr;
            let mut a_repl: ArcStr;
            let mut a_io: ArcStr;
            let mut a_options: Dump::DumpOptions;
            let mut i_comment: Option<Arc<Absyn::Comment>>;
            let mut i_import__: Absyn::Import;
            let mut i_components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_class__: Arc<Absyn::Class>;
            let mut l_imp__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            let mut l_comps__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_bc__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_bc__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            (l_args__str, _) = dumpElementArgList(Tpl::emptyTxt.clone(), i_elementArg.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })))?;
            l_mod__str = fun_105(Tpl::emptyTxt.clone(), l_args__str.clone())?;
            l_ann__str = dumpAnnotationOptSpace(Tpl::emptyTxt.clone(), i_annotationOpt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("extends ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_bc__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::COMPONENTS { components: i_components, attributes: i_attributes, typeSpec: i_typeSpec }, a_final, a_redecl, a_repl, a_io, _) => {
            let mut a_options: Dump::DumpOptions;
            let mut i_comment: Option<Arc<Absyn::Comment>>;
            let mut i_import__: Absyn::Import;
            let mut i_annotationOpt: Option<Arc<Absyn::Annotation>>;
            let mut i_elementArg: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_path: Arc<Absyn::Path>;
            let mut i_class__: Arc<Absyn::Class>;
            let mut l_imp__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            let mut l_comps__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_bc__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpTypeSpec(Tpl::emptyTxt.clone(), i_typeSpec.clone())?;
            l_attr__str = dumpElementAttr(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            l_dim__str = dumpElementAttrDim(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            l_comps__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_comps__str = lm_106(l_comps__str.clone(), i_components.clone())?;
            l_comps__str = Tpl::popIter(l_comps__str.clone())?;
            l_prefix__str = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_redecl.clone()).clone())?;
            l_prefix__str = Tpl::writeStr(l_prefix__str.clone(), (a_final.clone()).clone())?;
            l_prefix__str = Tpl::writeStr(l_prefix__str.clone(), (a_io.clone()).clone())?;
            l_prefix__str = Tpl::writeStr(l_prefix__str.clone(), (a_repl.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_prefix__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_attr__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dim__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_comps__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::IMPORT { comment: i_comment, import_: i_import__, .. }, _, _, _, _, _) => {
            let mut a_final: ArcStr;
            let mut a_redecl: ArcStr;
            let mut a_repl: ArcStr;
            let mut a_io: ArcStr;
            let mut a_options: Dump::DumpOptions;
            let mut i_components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            let mut i_attributes: Absyn::ElementAttributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_annotationOpt: Option<Arc<Absyn::Annotation>>;
            let mut i_elementArg: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut i_path: Arc<Absyn::Path>;
            let mut i_class__: Arc<Absyn::Class>;
            let mut l_imp__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            let mut l_comps__str: Tpl::Text;
            let mut l_dim__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_bc__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_imp__str = dumpImport(Tpl::emptyTxt.clone(), i_import__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_imp__str.clone())?;
            txt = dumpCommentOpt(txt.clone(), i_comment.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElseIfExp(txt: Tpl::Text, a_else__if: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_167(out_txt.clone(), a_else__if.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpEnumDef(in_txt: Tpl::Text, in_a_enum__def: Arc<Absyn::EnumDef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_enum__def.clone())) {
        (txt, Deref @ Absyn::ENUMLITERALS { enumLiterals: i_enumLiterals }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_39(txt.clone(), i_enumLiterals.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ENUM_COLON) => {
            let mut i_enumLiterals: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEnumLiteral(in_txt: Tpl::Text, in_a_lit: Arc<Absyn::EnumLiteral>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lit.clone())) {
        (txt, Deref @ Absyn::ENUMLITERAL { literal: i_literal, comment: i_comment }) => {
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_literal.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEqMod(in_txt: Tpl::Text, in_a_eqmod: Arc<Absyn::EqMod>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eqmod.clone())) {
        (txt, Deref @ Absyn::EQMOD { exp: i_exp, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEquation(in_txt: Tpl::Text, in_a_eq: Arc<Absyn::Equation>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ Absyn::EQ_IF { equationElseItems: i_equationElseItems, elseIfBranches: i_elseIfBranches, equationTrueItems: i_equationTrueItems, ifExp: i_ifExp }) => {
            let mut i_equ: Arc<Absyn::EquationItem>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionName: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_whenExp: Arc<Absyn::Exp>;
            let mut i_forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_connector2: Arc<Absyn::ComponentRef>;
            let mut i_connector1: Arc<Absyn::ComponentRef>;
            let mut i_domain: Arc<Absyn::ComponentRef>;
            let mut i_rightSide: Arc<Absyn::Exp>;
            let mut i_leftSide: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_if__str = dumpEquationBranch(Tpl::emptyTxt.clone(), i_ifExp.clone(), i_equationTrueItems.clone(), (literal!("if")).clone())?;
            l_elseif__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elseif__str = lm_128(l_elseif__str.clone(), i_elseIfBranches.clone())?;
            l_elseif__str = Tpl::popIter(l_elseif__str.clone())?;
            l_else__branch__str = dumpEquationItems(Tpl::emptyTxt.clone(), i_equationElseItems.clone())?;
            l_else__str = fun_129(Tpl::emptyTxt.clone(), l_else__branch__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_if__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elseif__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end if")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQ_EQUALS { rightSide: i_rightSide, leftSide: i_leftSide }) => {
            let mut i_equ: Arc<Absyn::EquationItem>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionName: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_whenExp: Arc<Absyn::Exp>;
            let mut i_forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_connector2: Arc<Absyn::ComponentRef>;
            let mut i_connector1: Arc<Absyn::ComponentRef>;
            let mut i_domain: Arc<Absyn::ComponentRef>;
            let mut i_equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs = dumpLhsExp(Tpl::emptyTxt.clone(), i_leftSide.clone())?;
            l_rhs = dumpExp(Tpl::emptyTxt.clone(), i_rightSide.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQ_PDE { domain: i_domain, rightSide: i_rightSide, leftSide: i_leftSide }) => {
            let mut i_equ: Arc<Absyn::EquationItem>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionName: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_whenExp: Arc<Absyn::Exp>;
            let mut i_forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_connector2: Arc<Absyn::ComponentRef>;
            let mut i_connector1: Arc<Absyn::ComponentRef>;
            let mut i_equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs = dumpLhsExp(Tpl::emptyTxt.clone(), i_leftSide.clone())?;
            l_rhs = dumpExp(Tpl::emptyTxt.clone(), i_rightSide.clone())?;
            l_domain__str = dumpCref(Tpl::emptyTxt.clone(), i_domain.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" indomain ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_domain__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQ_CONNECT { connector2: i_connector2, connector1: i_connector1 }) => {
            let mut i_equ: Arc<Absyn::EquationItem>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionName: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_whenExp: Arc<Absyn::Exp>;
            let mut i_forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_domain: Arc<Absyn::ComponentRef>;
            let mut i_rightSide: Arc<Absyn::Exp>;
            let mut i_leftSide: Arc<Absyn::Exp>;
            let mut i_equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_c1__str = dumpCref(Tpl::emptyTxt.clone(), i_connector1.clone())?;
            l_c2__str = dumpCref(Tpl::emptyTxt.clone(), i_connector2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("connect(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_c1__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_c2__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQ_FOR { forEquations: i_forEquations, iterators: i_iterators }) => {
            let mut i_equ: Arc<Absyn::EquationItem>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionName: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_whenExp: Arc<Absyn::Exp>;
            let mut i_connector2: Arc<Absyn::ComponentRef>;
            let mut i_connector1: Arc<Absyn::ComponentRef>;
            let mut i_domain: Arc<Absyn::ComponentRef>;
            let mut i_rightSide: Arc<Absyn::Exp>;
            let mut i_leftSide: Arc<Absyn::Exp>;
            let mut i_equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_iter__str = dumpForIterators(Tpl::emptyTxt.clone(), i_iterators.clone())?;
            l_body__str = dumpEquationItems(Tpl::emptyTxt.clone(), i_forEquations.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQ_WHEN_E { elseWhenEquations: i_elseWhenEquations, whenEquations: i_whenEquations, whenExp: i_whenExp }) => {
            let mut i_equ: Arc<Absyn::EquationItem>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionName: Arc<Absyn::ComponentRef>;
            let mut i_forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_connector2: Arc<Absyn::ComponentRef>;
            let mut i_connector1: Arc<Absyn::ComponentRef>;
            let mut i_domain: Arc<Absyn::ComponentRef>;
            let mut i_rightSide: Arc<Absyn::Exp>;
            let mut i_leftSide: Arc<Absyn::Exp>;
            let mut i_equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_when__str = dumpEquationBranch(Tpl::emptyTxt.clone(), i_whenExp.clone(), i_whenEquations.clone(), (literal!("when")).clone())?;
            l_elsewhen__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elsewhen__str = lm_130(l_elsewhen__str.clone(), i_elseWhenEquations.clone())?;
            l_elsewhen__str = Tpl::popIter(l_elsewhen__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_when__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elsewhen__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQ_NORETCALL { functionArgs: i_functionArgs, functionName: i_functionName }) => {
            let mut i_equ: Arc<Absyn::EquationItem>;
            let mut i_elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_whenExp: Arc<Absyn::Exp>;
            let mut i_forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_connector2: Arc<Absyn::ComponentRef>;
            let mut i_connector1: Arc<Absyn::ComponentRef>;
            let mut i_domain: Arc<Absyn::ComponentRef>;
            let mut i_rightSide: Arc<Absyn::Exp>;
            let mut i_leftSide: Arc<Absyn::Exp>;
            let mut i_equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_name__str = dumpCref(Tpl::emptyTxt.clone(), i_functionName.clone())?;
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQ_FAILURE { equ: i_equ }) => {
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_functionName: Arc<Absyn::ComponentRef>;
            let mut i_elseWhenEquations: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_whenEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_whenExp: Arc<Absyn::Exp>;
            let mut i_forEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_connector2: Arc<Absyn::ComponentRef>;
            let mut i_connector1: Arc<Absyn::ComponentRef>;
            let mut i_domain: Arc<Absyn::ComponentRef>;
            let mut i_rightSide: Arc<Absyn::Exp>;
            let mut i_leftSide: Arc<Absyn::Exp>;
            let mut i_equationElseItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_elseIfBranches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut i_equationTrueItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut i_ifExp: Arc<Absyn::Exp>;
            let mut l_eq__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_iter__str: Tpl::Text;
            let mut l_c2__str: Tpl::Text;
            let mut l_c1__str: Tpl::Text;
            let mut l_domain__str: Tpl::Text;
            let mut l_rhs: Tpl::Text;
            let mut l_lhs: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_eq__str = dumpEquationItem(Tpl::emptyTxt.clone(), i_equ.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("failure(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEquationBranch(txt: Tpl::Text, a_cond: Arc<Absyn::Exp>, a_body: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, a_header: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_body__str: Tpl::Text;
    let mut l_cond__str: Tpl::Text;
    l_cond__str = dumpExp(Tpl::emptyTxt.clone(), a_cond.clone())?;
    l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_body__str = lm_132(l_body__str.clone(), a_body.clone())?;
    l_body__str = Tpl::popIter(l_body__str.clone())?;
    out_txt = Tpl::writeStr(txt.clone(), (a_header.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_cond__str.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_body__str.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpEquationItem(in_txt: Tpl::Text, in_a_eq: Arc<Absyn::EquationItem>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eq.clone())) {
        (txt, Deref @ Absyn::EQUATIONITEM { comment: i_comment, equation_: i_equation__, .. }) => {
            let mut i_comment_1: ArcStr;
            let mut ret_2: ArcStr;
            let mut l_cmt__str: Tpl::Text;
            let mut l_eq__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_eq__str = dumpEquation(Tpl::emptyTxt.clone(), i_equation__.clone())?;
            l_cmt__str = dumpCommentOpt(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EQUATIONITEMCOMMENT { comment: i_comment_1 }) => {
            let mut i_comment: Option<Arc<Absyn::Comment>>;
            let mut i_equation__: Arc<Absyn::Equation>;
            let mut ret_2: ArcStr;
            let mut l_cmt__str: Tpl::Text;
            let mut l_eq__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ABS_INDENT { width: 0 }))?;
            ret_2 = (System::trimWhitespace((i_comment_1.clone()).clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEquationItems(txt: Tpl::Text, a_eql: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_126(out_txt.clone(), a_eql.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpExp(in_txt: Tpl::Text, in_a_exp: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ Absyn::INTEGER { value: i_value }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (intString(i_value.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::REAL { value: i_value_1 }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_value_1.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CREF { componentRef: i_componentRef }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpCref(txt.clone(), i_componentRef.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::STRING { value: i_value_1 }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ABS_INDENT { width: 0 }))?;
            txt = Tpl::writeStr(txt.clone(), (i_value_1.clone()).clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::BOOL { value: i_value_2 }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (Tpl::booleanString(i_value_2.clone())).clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::BINARY { op: i_op, exp2: i_exp2, exp1: i_exp1 }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::UNARY { op: i_op, exp: i_exp }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::LBINARY { op: i_op, exp2: i_exp2, exp1: i_exp1 }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::LUNARY { op: i_op, exp: i_exp }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::RELATION { op: i_op, exp2: i_exp2, exp1: i_exp1 }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp1.clone(), i_e.clone(), true)?;
            l_rhs__str = dumpOperand(Tpl::emptyTxt.clone(), i_exp2.clone(), i_e.clone(), false)?;
            l_op__str = dumpOperator(Tpl::emptyTxt.clone(), i_op.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ Absyn::IFEXP { ifExp: _, .. }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpIfExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CALL { functionArgs: i_functionArgs, function_: Deref @ Absyn::CREF_IDENT { name: Deref @ "$array", .. }, .. }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CALL { typeVars: i_typeVars, functionArgs: i_functionArgs, function_: i_function__ }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = dumpCref(Tpl::emptyTxt.clone(), i_function__.clone())?;
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            l_tvs__str = fun_155(Tpl::emptyTxt.clone(), i_typeVars.clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_tvs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::PARTEVALFUNCTION { functionArgs: i_functionArgs, function_: i_function__ }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = dumpCref(Tpl::emptyTxt.clone(), i_function__.clone())?;
            l_args__str = dumpFunctionArgs(Tpl::emptyTxt.clone(), i_functionArgs.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ARRAY { arrayExp: i_arrayExp }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_array__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_array__str = lm_156(l_array__str.clone(), i_arrayExp.clone())?;
            l_array__str = Tpl::popIter(l_array__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_array__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::MATRIX { matrix: i_matrix }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_matrix__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("; ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_matrix__str = lm_158(l_matrix__str.clone(), i_matrix.clone())?;
            l_matrix__str = Tpl::popIter(l_matrix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_matrix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::RANGE { stop: i_stop, start: i_start, step: Some(i_step) }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false)?;
            l_step__str = dumpOperand(Tpl::emptyTxt.clone(), i_step.clone(), i_e.clone(), false)?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_step__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt.clone()
        },
        (txt, i_e @ Deref @ Absyn::RANGE { stop: i_stop, start: i_start, step: None }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_start__str = dumpOperand(Tpl::emptyTxt.clone(), i_start.clone(), i_e.clone(), false)?;
            l_stop__str = dumpOperand(Tpl::emptyTxt.clone(), i_stop.clone(), i_e.clone(), false)?;
            txt = Tpl::writeText(txt.clone(), l_start__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_stop__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::TUPLE { expressions: i_expressions }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_tuple__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("")).clone() })), separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_tuple__str = lm_159(l_tuple__str.clone(), i_expressions.clone())?;
            l_tuple__str = Tpl::popIter(l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_tuple__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::END) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CODE { code: i_code }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("$Code(")).clone() }))?;
            txt = dumpCodeNode(txt.clone(), i_code.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::AS { id: i_id, exp: i_exp }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_id.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" as ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::CONS { rest: i_rest, head: i_head }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_head__str = dumpExp(Tpl::emptyTxt.clone(), i_head.clone())?;
            l_rest__str = dumpExp(Tpl::emptyTxt.clone(), i_rest.clone())?;
            txt = Tpl::writeText(txt.clone(), l_head__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" :: ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rest__str.clone())?;
            txt.clone()
        },
        (txt, i_exp @ Deref @ Absyn::MATCHEXP { matchTy: _, .. }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpMatchExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::LIST { exps: i_exps }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_list__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_list__str = lm_160(l_list__str.clone(), i_exps.clone())?;
            l_list__str = Tpl::popIter(l_list__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_list__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::DOT { index: i_index, exp: i_exp }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpExp(txt.clone(), i_index.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::EXPRESSIONCOMMENT { commentsAfter: i_commentsAfter, exp: i_exp, commentsBefore: i_commentsBefore }) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ABS_INDENT { width: 0 }))?;
            txt = lm_161(txt.clone(), i_commentsBefore.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ABS_INDENT { width: 0 }))?;
            txt = lm_162(txt.clone(), i_commentsAfter.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::SUBSCRIPTED_EXP { subscripts: i_subscripts, exp: i_exp }) => {
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")[")).clone() }))?;
            txt = dumpSubscripts(txt.clone(), i_subscripts.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("]")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::BREAK) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("break")).clone() }))?;
            txt.clone()
        },
        (txt, _) => {
            let mut i_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut i_commentsAfter: Arc<metamodelica::List<ArcStr>>;
            let mut i_commentsBefore: Arc<metamodelica::List<ArcStr>>;
            let mut i_index: Arc<Absyn::Exp>;
            let mut i_exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_rest: Arc<Absyn::Exp>;
            let mut i_head: Arc<Absyn::Exp>;
            let mut i_id: ArcStr;
            let mut i_code: Arc<Absyn::CodeNode>;
            let mut i_expressions: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_stop: Arc<Absyn::Exp>;
            let mut i_step: Arc<Absyn::Exp>;
            let mut i_start: Arc<Absyn::Exp>;
            let mut i_matrix: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut i_arrayExp: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut i_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_function__: Arc<Absyn::ComponentRef>;
            let mut i_functionArgs: Arc<Absyn::FunctionArgs>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_op: Absyn::Operator;
            let mut i_exp2: Arc<Absyn::Exp>;
            let mut i_e: Arc<Absyn::Exp>;
            let mut i_exp1: Arc<Absyn::Exp>;
            let mut i_value_2: bool;
            let mut i_componentRef: Arc<Absyn::ComponentRef>;
            let mut i_value_1: ArcStr;
            let mut i_value: i32;
            let mut l_list__str: Tpl::Text;
            let mut l_rest__str: Tpl::Text;
            let mut l_head__str: Tpl::Text;
            let mut l_tuple__str: Tpl::Text;
            let mut l_stop__str: Tpl::Text;
            let mut l_step__str: Tpl::Text;
            let mut l_start__str: Tpl::Text;
            let mut l_matrix__str: Tpl::Text;
            let mut l_array__str: Tpl::Text;
            let mut l_tvs__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_op__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* AbsynDumpTpl.dumpExp: UNHANDLED Abyn.Exp */")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFinal(in_txt: Tpl::Text, in_a_final: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_final.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("final ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpForIterator(in_txt: Tpl::Text, in_a_iterator: Arc<Absyn::ForIterator>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_iterator.clone())) {
        (txt, Deref @ Absyn::ITERATOR { name: i_name, guardExp: i_guardExp, range: i_range }) => {
            let mut l_guard__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_range__str = fun_198(Tpl::emptyTxt.clone(), i_range.clone())?;
            l_guard__str = fun_199(Tpl::emptyTxt.clone(), i_guardExp.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_guard__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpForIterators(txt: Tpl::Text, a_iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_196(out_txt.clone(), a_iters.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpFunctionArgs(in_txt: Tpl::Text, in_a_args: Arc<Absyn::FunctionArgs>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args.clone())) {
        (txt, Deref @ Absyn::FUNCTIONARGS { argNames: i_argNames, args: i_args }) => {
            let mut i_iterType: Absyn::ReductionIterType;
            let mut i_iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut l_iter__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_separator: Tpl::Text;
            let mut l_namedargs__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_args__str = lm_188(l_args__str.clone(), i_args.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            l_namedargs__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_namedargs__str = lm_189(l_namedargs__str.clone(), i_argNames.clone())?;
            l_namedargs__str = Tpl::popIter(l_namedargs__str.clone())?;
            l_separator = fun_191(Tpl::emptyTxt.clone(), l_args__str.clone(), i_argNames.clone())?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_separator.clone())?;
            txt = Tpl::writeText(txt.clone(), l_namedargs__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::FOR_ITER_FARG { iterType: i_iterType, iterators: i_iterators, exp: i_exp }) => {
            let mut i_argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut i_args: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut l_iter__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_separator: Tpl::Text;
            let mut l_namedargs__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_iter__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_iter__str = lm_192(l_iter__str.clone(), i_iterators.clone())?;
            l_iter__str = Tpl::popIter(l_iter__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = fun_193(txt.clone(), i_iterType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_iter__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpGroupImport(in_txt: Tpl::Text, in_a_gimp: Absyn::GroupImport) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_gimp.clone()) {
        (mut txt, Absyn::GROUP_IMPORT_NAME { name: mut i_name }) => {
            let mut i_rename: ArcStr;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, Absyn::GROUP_IMPORT_RENAME { name: mut i_name, rename: mut i_rename }) => {
            txt = Tpl::writeStr(txt.clone(), (i_rename.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpIfExp(in_txt: Tpl::Text, in_a_if__exp: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_if__exp.clone())) {
        (txt, Deref @ Absyn::IFEXP { elseIfBranch: i_elseIfBranch, elseBranch: i_elseBranch, trueBranch: i_trueBranch, ifExp: i_ifExp }) => {
            let mut l_else__if__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_true__branch__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), i_ifExp.clone())?;
            l_true__branch__str = dumpExp(Tpl::emptyTxt.clone(), i_trueBranch.clone())?;
            l_else__branch__str = dumpExp(Tpl::emptyTxt.clone(), i_elseBranch.clone())?;
            l_else__if__str = dumpElseIfExp(Tpl::emptyTxt.clone(), i_elseIfBranch.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_true__branch__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__if__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" else ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__branch__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpImport(in_txt: Tpl::Text, in_a_imp: Absyn::Import) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_imp.clone()) {
        (mut txt, Absyn::NAMED_IMPORT { path: ref i_path, name: mut i_name }) => {
            let mut i_groups: Arc<metamodelica::List<Absyn::GroupImport>>;
            let mut i_prefix: Arc<Absyn::Path>;
            let mut l_groups__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::QUAL_IMPORT { path: ref i_path }) => {
            let mut i_groups: Arc<metamodelica::List<Absyn::GroupImport>>;
            let mut i_prefix: Arc<Absyn::Path>;
            let mut i_name: ArcStr;
            let mut l_groups__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::UNQUAL_IMPORT { path: ref i_path }) => {
            let mut i_groups: Arc<metamodelica::List<Absyn::GroupImport>>;
            let mut i_prefix: Arc<Absyn::Path>;
            let mut i_name: ArcStr;
            let mut l_groups__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".*")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::GROUP_IMPORT { groups: ref i_groups, prefix: ref i_prefix }) => {
            let mut i_path: Arc<Absyn::Path>;
            let mut i_name: ArcStr;
            let mut l_groups__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            l_prefix__str = dumpPath(Tpl::emptyTxt.clone(), i_prefix.clone())?;
            l_groups__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_groups__str = lm_122(l_groups__str.clone(), i_groups.clone())?;
            l_groups__str = Tpl::popIter(l_groups__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_prefix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".{")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_groups__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpInfo(in_txt: Tpl::Text, in_a_info: SourceInfo) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_info.clone()) {
        (mut txt, SourceInfo { columnNumberEnd: mut i_columnNumberEnd, lineNumberEnd: mut i_lineNumberEnd, columnNumberStart: mut i_columnNumberStart, lineNumberStart: mut i_lineNumberStart, fileName: mut i_fileName, isReadOnly: mut i_isReadOnly, .. }) => {
            let mut l_rm__str: Tpl::Text;
            l_rm__str = fun_84(Tpl::emptyTxt.clone(), i_isReadOnly.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("SOURCEINFO(\\\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_fileName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rm__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberStart.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_lineNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_columnNumberEnd.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")\\\\n")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpInnerOuter(in_txt: Tpl::Text, in_a_io: Absyn::InnerOuter) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_io.clone()) {
        (mut txt, Absyn::INNER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("inner ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::OUTER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("outer ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::INNER_OUTER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("inner outer ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpIsField(in_txt: Tpl::Text, in_a_isField: Absyn::IsField) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isField.clone()) {
        (mut txt, Absyn::NONFIELD) => txt.clone(),
        (mut txt, Absyn::FIELD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("field ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpLhsExp(in_txt: Tpl::Text, in_a_lhs: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_lhs.clone())) {
        (txt, i_lhs @ Deref @ Absyn::IFEXP { ifExp: _, .. }) => {
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

pub fn dumpMatchCase(in_txt: Tpl::Text, in_a_c: Arc<Absyn::Case>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_c.clone())) {
        (txt, Deref @ Absyn::CASE { comment: i_comment, result: i_result, classPart: i_classPart, patternGuard: i_patternGuard, pattern: i_pattern, .. }) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_then__str: Tpl::Text;
            let mut l_result__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_guard__str: Tpl::Text;
            let mut l_pattern__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_pattern__str = dumpExp(Tpl::emptyTxt.clone(), i_pattern.clone())?;
            l_guard__str = fun_181(Tpl::emptyTxt.clone(), i_patternGuard.clone())?;
            l_eql__str = dumpMatchEquations(Tpl::emptyTxt.clone(), i_classPart.clone())?;
            l_result__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone())?;
            l_then__str = fun_182(Tpl::emptyTxt.clone(), l_eql__str.clone(), l_result__str.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("case ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_pattern__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_guard__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_then__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ELSE { comment: i_comment, result: i_result, classPart: i_classPart, .. }) => {
            let mut i_patternGuard: Option<Arc<Absyn::Exp>>;
            let mut i_pattern: Arc<Absyn::Exp>;
            let mut l_cmt__str: Tpl::Text;
            let mut l_then__str: Tpl::Text;
            let mut l_result__str: Tpl::Text;
            let mut l_eql__str: Tpl::Text;
            let mut l_guard__str: Tpl::Text;
            let mut l_pattern__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_eql__str = dumpMatchEquations(Tpl::emptyTxt.clone(), i_classPart.clone())?;
            l_result__str = dumpExp(Tpl::emptyTxt.clone(), i_result.clone())?;
            l_then__str = fun_183(Tpl::emptyTxt.clone(), l_eql__str.clone(), l_result__str.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("else ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eql__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_then__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpMatchEquations(in_txt: Tpl::Text, in_a_cp: Arc<Absyn::ClassPart>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cp.clone())) {
        (txt, Deref @ Absyn::EQUATIONS { contents: Deref @ metamodelica::List::Nil }) => txt.clone(),
        (txt, Deref @ Absyn::EQUATIONS { contents: i_eql }) => {
            let mut i_algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\n")).clone(), (literal!("  equation\\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_178(txt.clone(), i_eql.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::ALGORITHMS { contents: Deref @ metamodelica::List::Nil }) => txt.clone(),
        (txt, Deref @ Absyn::ALGORITHMS { contents: i_algs }) => {
            let mut i_eql: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\n")).clone(), (literal!("  algorithm\\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_179(txt.clone(), i_algs.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpMatchExp(in_txt: Tpl::Text, in_a_match__exp: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_match__exp.clone())) {
        (txt, Deref @ Absyn::MATCHEXP { comment: i_comment, cases: i_cases, localDecls: i_localDecls, inputExp: i_inputExp, matchTy: i_matchTy }) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_cases__str: Tpl::Text;
            let mut l_locals__str: Tpl::Text;
            let mut l_input__str: Tpl::Text;
            let mut l_ty__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ty__str = dumpMatchType(Tpl::emptyTxt.clone(), i_matchTy.clone())?;
            l_input__str = dumpExp(Tpl::emptyTxt.clone(), i_inputExp.clone())?;
            l_locals__str = dumpMatchLocals(Tpl::emptyTxt.clone(), i_localDecls.clone())?;
            l_cases__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\n")).clone(), (literal!("\\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_cases__str = lm_173(l_cases__str.clone(), i_cases.clone())?;
            l_cases__str = Tpl::popIter(l_cases__str.clone())?;
            l_cmt__str = dumpStringCommentOption(Tpl::emptyTxt.clone(), i_comment.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_input__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_locals__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_cases__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpMatchLocals(in_txt: Tpl::Text, in_a_locals: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_locals.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_locals) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  local\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_176(txt.clone(), i_locals.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpMatchType(in_txt: Tpl::Text, in_a_match__type: Absyn::MatchType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_match__type.clone()) {
        (mut txt, Absyn::MATCH) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::MATCHCONTINUE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("matchcontinue")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpModification(in_txt: Tpl::Text, in_a_mod: Arc<Absyn::Modification>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_mod.clone())) {
        (txt, Deref @ Absyn::CLASSMOD { eqMod: i_eqMod, elementArgLst: i_elementArgLst }) => {
            let mut l_eq__str: Tpl::Text;
            let mut l_arg__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_arg__str = fun_102(Tpl::emptyTxt.clone(), i_elementArgLst.clone())?;
            l_eq__str = dumpEqMod(Tpl::emptyTxt.clone(), i_eqMod.clone())?;
            txt = Tpl::writeText(txt.clone(), l_arg__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpNamedArg(in_txt: Tpl::Text, in_a_narg: Arc<Absyn::NamedArg>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_narg.clone())) {
        (txt, Deref @ Absyn::NAMEDARG { argValue: i_argValue, argName: i_argName }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_argName.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_argValue.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpOperand(txt: Tpl::Text, a_operand: Arc<Absyn::Exp>, a_operation: Arc<Absyn::Exp>, a_lhs: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_op__str: Tpl::Text;
    l_op__str = dumpExp(Tpl::emptyTxt.clone(), a_operand.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_op__str.clone())?;
    Ok(out_txt)
}

pub fn dumpOperator(in_txt: Tpl::Text, in_a_op: Absyn::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, Absyn::ADD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::SUB) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" - ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::MUL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::DIV) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::POW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("^")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::UPLUS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::UMINUS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::ADD_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" .+ ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::SUB_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" .- ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::MUL_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".*")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::DIV_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("./")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::POW_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".^")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::UPLUS_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" .+ ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::UMINUS_EW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" .- ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::AND) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" and ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::OR) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" or ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::NOT) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("not")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::LESS) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" < ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::LESSEQ) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <= ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::GREATER) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" > ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::GREATEREQ) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" >= ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::EQUAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" == ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::NEQUAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <> ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpParallelism(in_txt: Tpl::Text, in_a_par: Absyn::Parallelism) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_par.clone()) {
        (mut txt, Absyn::PARGLOBAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parglobal ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::PARLOCAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parlocal ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::NON_PARALLEL) => txt.clone(),
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpPath(in_txt: Tpl::Text, in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::FULLYQUALIFIED { path: i_path }) => {
            let mut i_name: ArcStr;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::QUALIFIED { path: i_path, name: i_name }) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
            txt = fun_144(txt.clone(), ret_0.clone(), i_path.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::IDENT { name: i_name }) => {
            let mut i_path: Arc<Absyn::Path>;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            let mut i_name: ArcStr;
            let mut i_path: Arc<Absyn::Path>;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpPath: Unknown path.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpPathNoQual(in_txt: Tpl::Text, in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = dumpPath(txt.clone(), i_path.clone())?;
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

pub fn dumpPurity(in_txt: Tpl::Text, in_a_purity: Absyn::FunctionPurity) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_purity.clone()) {
        (mut txt, Absyn::PURE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pure ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::IMPURE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("impure ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::NO_PURITY) => txt.clone(),
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpRedeclare(in_txt: Tpl::Text, in_a_redecl: Absyn::RedeclareKeywords) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_redecl.clone()) {
        (mut txt, Absyn::REDECLARE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("redeclare ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::REDECLARE_REPLACEABLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("redeclare ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpReplaceable(in_txt: Tpl::Text, in_a_repl: Absyn::RedeclareKeywords) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_repl.clone()) {
        (mut txt, Absyn::REPLACEABLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("replaceable ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::REDECLARE_REPLACEABLE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("replaceable ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpRestriction(in_txt: Tpl::Text, in_a_restriction: Absyn::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_restriction.clone()) {
        (mut txt, Absyn::R_CLASS) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("class")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_OPTIMIZATION) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("optimization")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_MODEL) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("model")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_RECORD) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("record")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_BLOCK) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("block")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_CONNECTOR) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("connector")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_EXP_CONNECTOR) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expandable connector")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_TYPE) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("type")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_PACKAGE) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("package")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_FUNCTION { functionRestriction: mut i_functionRestriction }) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut l_prefix__str: Tpl::Text;
            l_prefix__str = fun_47(Tpl::emptyTxt.clone(), i_functionRestriction.clone())?;
            txt = Tpl::writeText(txt.clone(), l_prefix__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_OPERATOR) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("operator")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_OPERATOR_RECORD) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("operator record")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_ENUMERATION) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("enumeration")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_PREDEFINED_INTEGER) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_PREDEFINED_REAL) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_PREDEFINED_STRING) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_PREDEFINED_BOOLEAN) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_PREDEFINED_ENUMERATION) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("enumeration(:)")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_UNIONTYPE) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("uniontype")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::R_METARECORD { typeVars: ref i_typeVars, .. }) => {
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("metarecord")).clone() }))?;
            txt = fun_49(txt.clone(), i_typeVars.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::R_UNKNOWN) => {
            let mut i_typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionRestriction: Absyn::FunctionRestriction;
            let mut l_prefix__str: Tpl::Text;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*unknown*")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpStringCommentOption(in_txt: Tpl::Text, in_a_cmt: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_cmt.clone()) {
        (mut txt, Some(mut i_str)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_str.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpSubscript(in_txt: Tpl::Text, in_a_subscript: Arc<Absyn::Subscript>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscript.clone())) {
        (txt, Deref @ Absyn::NOSUB) => {
            let mut i_subscript: Arc<Absyn::Exp>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ Absyn::SUBSCRIPT { subscript: i_subscript }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_subscript.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpSubscripts(in_txt: Tpl::Text, in_a_subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subscripts.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_subscripts) => {
            let mut l_sub__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_sub__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_sub__str = lm_151(l_sub__str.clone(), i_subscripts.clone())?;
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

pub fn dumpTypeSpec(in_txt: Tpl::Text, in_a_typeSpec: Arc<Absyn::TypeSpec>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeSpec.clone())) {
        (txt, Deref @ Absyn::TPATH { arrayDim: i_arrayDim, path: i_path }) => {
            let mut i_typeSpecs: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>;
            let mut l_ty__str: Tpl::Text;
            let mut l_arraydim__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_arraydim__str = dumpArrayDimOpt(Tpl::emptyTxt.clone(), i_arrayDim.clone())?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_arraydim__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::TCOMPLEX { arrayDim: i_arrayDim, typeSpecs: i_typeSpecs, path: i_path }) => {
            let mut l_ty__str: Tpl::Text;
            let mut l_arraydim__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            l_ty__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_ty__str = lm_148(l_ty__str.clone(), i_typeSpecs.clone())?;
            l_ty__str = Tpl::popIter(l_ty__str.clone())?;
            l_arraydim__str = dumpArrayDimOpt(Tpl::emptyTxt.clone(), i_arrayDim.clone())?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ty__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_arraydim__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpVariability(in_txt: Tpl::Text, in_a_var: Absyn::Variability) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_var.clone()) {
        (mut txt, Absyn::VAR) => txt.clone(),
        (mut txt, Absyn::DISCRETE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::PARAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::CONST) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpWithin(in_txt: Tpl::Text, in_a_within: Absyn::Within) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_within.clone()) {
        (mut txt, Absyn::TOP) => txt.clone(),
        (mut txt, Absyn::WITHIN { path: ref i_path }) => {
            let mut l_path__str: Tpl::Text;
            l_path__str = dumpPath(Tpl::emptyTxt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("within ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\\n")).clone(), (literal!("\\n")).clone()], lastHasNewLine: true }))?;
            txt.clone()
        },
        (mut txt, _) => {
            let mut i_path: Arc<Absyn::Path>;
            let mut l_path__str: Tpl::Text;
            Tpl::addSourceTemplateError((literal!("Unknown operation")).clone(), Tpl::sourceInfo((literal!("AbsynDumpTpl.tpl")).clone(), 64, 56))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn errorMsg(txt: Tpl::Text, a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    Tpl::addTemplateError((a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeStr(txt.clone(), (a_errMessage.clone()).clone())?;
    Ok(out_txt)
}

fn fun_102(in_txt: Tpl::Text, in_a_elementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elementArgLst.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_elementArgLst) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            (txt, _) = dumpElementArgList(txt.clone(), i_elementArgLst.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_105(in_txt: Tpl::Text, in_a_args__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_args__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_108(in_txt: Tpl::Text, in_a_flowPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_flowPrefix.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flow ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_109(in_txt: Tpl::Text, in_a_streamPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_streamPrefix.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("stream ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_116(in_txt: Tpl::Text, in_a_el: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_el.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_el) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            (txt, _) = dumpElementArgList(txt.clone(), i_el.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_119(in_txt: Tpl::Text, in_a_modification: Option<Arc<Absyn::Modification>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modification.clone()) {
        (mut txt, Some(mut i_mod)) => {
            txt = dumpModification(txt.clone(), i_mod.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_129(in_txt: Tpl::Text, in_a_else__branch__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__branch__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_else__branch__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_else__branch__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_138(in_txt: Tpl::Text, in_a_else__branch__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_else__branch__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_else__branch__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_else__branch__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_140(in_txt: Tpl::Text, in_a_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equ.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            let mut i_equ: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("...")).clone() }))?;
            txt.clone()
        },
        (txt, i_equ) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmItems(txt.clone(), i_equ.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_144(in_txt: Tpl::Text, in_mArg: bool, in_a_path: Arc<Absyn::Path>, in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_path.clone(), in_a_name.clone())) {
        (txt, false, a_path, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = dumpPath(txt.clone(), a_path.clone())?;
            txt.clone()
        },
        (txt, _, a_path, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            txt = dumpPath(txt.clone(), a_path.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_15(in_txt: Tpl::Text, in_a_commentsBeforeClass: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_commentsBeforeClass.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_commentsBeforeClass) => {
            let mut txt = (*txt).clone();
            txt = lm_14(txt.clone(), i_commentsBeforeClass.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_155(in_txt: Tpl::Text, in_a_typeVars: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_typeVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_154(txt.clone(), i_typeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_169(in_txt: Tpl::Text, in_a_boolean: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_boolean.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("initial ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_17(in_txt: Tpl::Text, in_a_commentsAfterEnd: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_commentsAfterEnd.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_commentsAfterEnd) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = lm_16(txt.clone(), i_commentsAfterEnd.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_170(in_txt: Tpl::Text, in_a_boolean: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_boolean.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("initial ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_171(in_txt: Tpl::Text, in_a_boolean: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_boolean.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("initial ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_181(in_txt: Tpl::Text, in_a_patternGuard: Option<Arc<Absyn::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_patternGuard.clone()) {
        (mut txt, Some(mut i_g)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("guard ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_g.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_182(in_txt: Tpl::Text, in_a_eql__str: Tpl::Text, in_a_result__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eql__str.clone(), in_a_result__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_result__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_result__str.clone())?;
            txt.clone()
        },
        (txt, _, a_result__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\n")).clone(), (literal!("  then\\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_result__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_183(in_txt: Tpl::Text, in_a_eql__str: Tpl::Text, in_a_result__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_eql__str.clone(), in_a_result__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_result__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_result__str.clone())?;
            txt.clone()
        },
        (txt, _, a_result__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\\n")).clone(), (literal!("  then\\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), a_result__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_186(in_txt: Tpl::Text, in_mArg: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_190(in_txt: Tpl::Text, in_a_argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_argNames.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_191(in_txt: Tpl::Text, in_a_args__str: Tpl::Text, in_a_argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone(), in_a_argNames.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => txt.clone(),
        (txt, _, a_argNames) => {
            let mut txt = (*txt).clone();
            txt = fun_190(txt.clone(), a_argNames.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_193(in_txt: Tpl::Text, in_a_iterType: Absyn::ReductionIterType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_iterType.clone()) {
        (mut txt, Absyn::THREAD) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("threaded ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_198(in_txt: Tpl::Text, in_a_range: Option<Arc<Absyn::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_range.clone()) {
        (mut txt, Some(mut i_r)) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("in ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_r.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_199(in_txt: Tpl::Text, in_a_guardExp: Option<Arc<Absyn::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_guardExp.clone()) {
        (mut txt, Some(mut i_g)) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("guard ")).clone() }))?;
            txt = dumpExp(txt.clone(), i_g.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_21(in_txt: Tpl::Text, in_a_typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_typeVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_20(txt.clone(), i_typeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_24(in_txt: Tpl::Text, in_a_ann__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_ann__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_26(in_txt: Tpl::Text, in_a_arguments: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_arguments.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_arguments) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            (txt, _) = dumpElementArgList(txt.clone(), i_arguments.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_29(in_txt: Tpl::Text, in_a_modifications: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modifications.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_modifications) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            (txt, _) = dumpElementArgList(txt.clone(), i_modifications.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_31(in_txt: Tpl::Text, in_a_ann__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_ann__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_42(in_txt: Tpl::Text, in_a_encapsulatedPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_encapsulatedPrefix.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("encapsulated ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_43(in_txt: Tpl::Text, in_a_partialPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_partialPrefix.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("partial ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_44(in_txt: Tpl::Text, in_a_cls: Arc<Absyn::Class>, in_a_redecl__str: ArcStr, in_a_repl__str: ArcStr, in_a_io__str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cls.clone(), in_a_redecl__str.clone(), in_a_repl__str.clone(), in_a_io__str.clone())) {
        (txt, Deref @ Absyn::CLASS { finalPrefix: i_finalPrefix, partialPrefix: i_partialPrefix, encapsulatedPrefix: i_encapsulatedPrefix, .. }, a_redecl__str, a_repl__str, a_io__str) => {
            let mut l_fin__str: Tpl::Text;
            let mut l_partial__str: Tpl::Text;
            let mut l_enc__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_enc__str = fun_42(Tpl::emptyTxt.clone(), i_encapsulatedPrefix.clone())?;
            l_partial__str = fun_43(Tpl::emptyTxt.clone(), i_partialPrefix.clone())?;
            l_fin__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_redecl__str.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_fin__str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_io__str.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_repl__str.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_enc__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_partial__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_47(in_txt: Tpl::Text, in_a_functionRestriction: Absyn::FunctionRestriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_functionRestriction.clone()) {
        (mut txt, Absyn::FR_NORMAL_FUNCTION { purity: mut i_purity }) => {
            txt = dumpPurity(txt.clone(), i_purity.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::FR_OPERATOR_FUNCTION) => {
            let mut i_purity: Absyn::FunctionPurity;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("operator ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::FR_PARALLEL_FUNCTION) => {
            let mut i_purity: Absyn::FunctionPurity;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parallel ")).clone() }))?;
            txt.clone()
        },
        (mut txt, Absyn::FR_KERNEL_FUNCTION) => {
            let mut i_purity: Absyn::FunctionPurity;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("kernel ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_49(in_txt: Tpl::Text, in_a_typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_typeVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_48(txt.clone(), i_typeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_51(in_txt: Tpl::Text, in_a_idx: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_idx.clone()) {
        (mut txt, 0) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("public")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_57(in_txt: Tpl::Text, in_a_annotation__: Option<Arc<Absyn::Annotation>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_annotation__.clone()) {
        (mut txt, Some(mut i_ann)) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = dumpAnnotation(txt.clone(), i_ann.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_58(in_txt: Tpl::Text, in_a_funcName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_funcName.clone()) {
        (mut txt, Some(mut i_fn)) => {
            txt = Tpl::writeStr(txt.clone(), (i_fn.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_59(in_txt: Tpl::Text, in_a_lang: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_lang.clone()) {
        (mut txt, Some(mut i_l)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_l.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\" ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_60(in_txt: Tpl::Text, in_a_output__: Option<Arc<Absyn::ComponentRef>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_output__.clone()) {
        (mut txt, Some(mut i_o)) => {
            txt = dumpCref(txt.clone(), i_o.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_61(in_txt: Tpl::Text, in_a_fn__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_fn__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("()")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_63(in_txt: Tpl::Text, in_a_args: Arc<metamodelica::List<Arc<Absyn::Exp>>>, in_a_fn__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args.clone(), in_a_fn__str.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_fn__str) => {
            let mut i_args: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut txt = (*txt).clone();
            txt = fun_61(txt.clone(), a_fn__str.clone())?;
            txt.clone()
        },
        (txt, i_args, _) => {
            let mut a_fn__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_62(txt.clone(), i_args.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_64(in_txt: Tpl::Text, in_a_externalDecl: Arc<Absyn::ExternalDecl>, in_a_ann__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_externalDecl.clone(), in_a_ann__str.clone())) {
        (txt, Deref @ Absyn::EXTERNALDECL { annotation_: i_annotation__, args: i_args, output_: i_output__, lang: i_lang, funcName: i_funcName }, a_ann__str) => {
            let mut l_ann2__str: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_output__str: Tpl::Text;
            let mut l_lang__str: Tpl::Text;
            let mut l_fn__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fn__str = fun_58(Tpl::emptyTxt.clone(), i_funcName.clone())?;
            l_lang__str = fun_59(Tpl::emptyTxt.clone(), i_lang.clone())?;
            l_output__str = fun_60(Tpl::emptyTxt.clone(), i_output__.clone())?;
            l_args__str = fun_63(Tpl::emptyTxt.clone(), i_args.clone(), l_fn__str.clone())?;
            l_ann2__str = dumpAnnotationOptSpace(Tpl::emptyTxt.clone(), i_annotation__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("external ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lang__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_output__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_fn__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann2__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ann__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_66(in_txt: Tpl::Text, in_a_first: bool, in_a_prevSpacing: ArcStr, in_a_spacing: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_first.clone(), in_a_prevSpacing.clone(), in_a_spacing.clone()) {
        (mut txt, false, mut a_prevSpacing, mut a_spacing) => {
            txt = dumpElementItemPreSpacing(txt.clone(), (Tpl::textString(a_spacing.clone())?).clone(), (a_prevSpacing.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_67(in_txt: Tpl::Text, in_a_rest__str: Tpl::Text, in_a_spacing: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rest__str.clone(), in_a_spacing.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => txt.clone(),
        (txt, _, a_spacing) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_spacing.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_68(in_txt: Tpl::Text, in_a_rest__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_rest__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_rest__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), i_rest__str.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_70(in_txt: Tpl::Text, in_a_prevSpacing: ArcStr, in_a_curSpacing: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_prevSpacing.clone(), in_a_curSpacing.clone())) {
        (txt, Deref @ "", a_curSpacing) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_curSpacing.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_75(in_txt: Tpl::Text, in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_redeclareKeywords.clone()) {
        (mut txt, Some(mut i_re)) => {
            txt = dumpRedeclare(txt.clone(), i_re.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_76(in_txt: Tpl::Text, in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_redeclareKeywords.clone()) {
        (mut txt, Some(mut i_re)) => {
            txt = dumpReplaceable(txt.clone(), i_re.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_77(in_txt: Tpl::Text, in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_constrainClass.clone()) {
        (mut txt, Some(mut i_cc)) => {
            txt = dumpConstrainClass(txt.clone(), i_cc.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_78(in_txt: Tpl::Text, in_mArg: bool, in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>, in_a_options: Dump::DumpOptions, in_a_specification: Arc<Absyn::ElementSpec>, in_a_innerOuter: Absyn::InnerOuter, in_a_redeclareKeywords: Option<Absyn::RedeclareKeywords>, in_a_finalPrefix: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_constrainClass.clone(), in_a_options.clone(), in_a_specification.clone(), in_a_innerOuter.clone(), in_a_redeclareKeywords.clone(), in_a_finalPrefix.clone())) {
        (txt, false, _, _, _, _, _, _) => txt.clone(),
        (txt, _, a_constrainClass, a_options, a_specification, a_innerOuter, a_redeclareKeywords, a_finalPrefix) => {
            let mut l_cc__str: Tpl::Text;
            let mut l_ec__str: Tpl::Text;
            let mut l_io__str: Tpl::Text;
            let mut l_repl__str: Tpl::Text;
            let mut l_redecl__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), a_finalPrefix.clone())?;
            l_redecl__str = fun_75(Tpl::emptyTxt.clone(), a_redeclareKeywords.clone())?;
            l_repl__str = fun_76(Tpl::emptyTxt.clone(), a_redeclareKeywords.clone())?;
            l_io__str = dumpInnerOuter(Tpl::emptyTxt.clone(), a_innerOuter.clone())?;
            l_ec__str = dumpElementSpec(Tpl::emptyTxt.clone(), a_specification.clone(), (Tpl::textString(l_final__str.clone())?).clone(), (Tpl::textString(l_redecl__str.clone())?).clone(), (Tpl::textString(l_repl__str.clone())?).clone(), (Tpl::textString(l_io__str.clone())?).clone(), a_options.clone())?;
            l_cc__str = fun_77(Tpl::emptyTxt.clone(), a_constrainClass.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ec__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cc__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_80(in_txt: Tpl::Text, in_a_args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_args) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = lm_79(txt.clone(), i_args.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_81(in_txt: Tpl::Text, in_a_optName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_optName.clone()) {
        (mut txt, Some(mut i_name)) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_82(in_txt: Tpl::Text, in_mArg: bool, in_a_string: ArcStr, in_a_info: SourceInfo, in_a_optName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_string.clone(), in_a_info.clone(), in_a_optName.clone()) {
        (mut txt, false, _, _, _) => txt.clone(),
        (mut txt, _, mut a_string, mut a_info, mut a_optName) => {
            let mut l_info__str: Tpl::Text;
            let mut l_name__str: Tpl::Text;
            l_name__str = fun_81(Tpl::emptyTxt.clone(), a_optName.clone())?;
            l_info__str = dumpInfo(Tpl::emptyTxt.clone(), a_info.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("/* Absyn.TEXT(SOME(\\\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"), \\\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_string.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\", \\\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_info__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"); */")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_84(in_txt: Tpl::Text, in_a_isReadOnly: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isReadOnly.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("writable")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("readonly")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_91(in_txt: Tpl::Text, in_a_b: bool, in_a_separator: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_b.clone(), in_a_separator.clone()) {
        (mut txt, false, _) => txt.clone(),
        (mut txt, _, mut a_separator) => {
            txt = Tpl::writeText(txt.clone(), a_separator.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_94(in_txt: Tpl::Text, in_a_modification: Option<Arc<Absyn::Modification>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_modification.clone()) {
        (mut txt, Some(mut i_mod)) => {
            txt = dumpModification(txt.clone(), i_mod.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_95(in_txt: Tpl::Text, in_a_constrainClass: Option<Arc<Absyn::ConstrainClass>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_constrainClass.clone()) {
        (mut txt, Some(mut i_cc)) => {
            txt = dumpConstrainClass(txt.clone(), i_cc.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_106(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_comp, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpComponentItem(txt.clone(), i_comp.clone())?;
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
fn lm_122(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Absyn::GroupImport>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_group, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpGroupImport(txt.clone(), i_group.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_122(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_126(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationItem(txt.clone(), i_eq.clone())?;
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
fn lm_128(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationBranch(txt.clone(), i_c.clone(), i_b.clone(), (literal!("elseif")).clone())?;
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
fn lm_130(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationBranch(txt.clone(), i_c.clone(), i_b.clone(), (literal!("elsewhen")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_130(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_132(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationItem(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_132(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_134(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_alg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmItem(txt.clone(), i_alg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_134(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_137(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmBranch(txt.clone(), i_c.clone(), i_b.clone(), (literal!("elseif")).clone(), (literal!("then")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_137(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_139(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_c, i_b), tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmBranch(txt.clone(), i_c.clone(), i_b.clone(), (literal!("elsewhen")).clone(), (literal!("then")).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_139(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_14(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_14(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_142(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmItem(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_142(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_148(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpTypeSpec(txt.clone(), i_ty.clone())?;
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
fn lm_151(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpSubscript(txt.clone(), i_s.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_151(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_154(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpPath(txt.clone(), i_v.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_154(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_156(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_156(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_157(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_157(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_158(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_row, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_157(txt.clone(), i_row.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_158(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_159(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_159(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_16(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = lm_16(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_160(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_160(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_161(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_161(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_162(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_162(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_167(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_cond, i_branch), tail: rest }) => {
            let mut l_branch__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = dumpExp(Tpl::emptyTxt.clone(), i_cond.clone())?;
            l_branch__str = dumpExp(Tpl::emptyTxt.clone(), i_branch.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elseif ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_branch__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_167(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_173(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Case>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_c, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpMatchCase(txt.clone(), i_c.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_173(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_176(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_decl, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpElementItem(txt.clone(), i_decl.clone(), Dump::defaultDumpOptions.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_176(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_178(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationItem(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_178(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_179(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_alg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmItem(txt.clone(), i_alg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_179(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_188(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_188(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_189(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_narg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpNamedArg(txt.clone(), i_narg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_189(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_192(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpForIterator(txt.clone(), i_i.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_192(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_196(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_i, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpForIterator(txt.clone(), i_i.clone())?;
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
fn lm_20(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_typevar, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_typevar.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_20(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_22(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Annotation>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_a, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAnnotation(txt.clone(), i_a.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_22(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_23(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_class__part, tail: rest }, a_options) => {
            let mut x_idx: i32;
            let mut txt = (*txt).clone();
            x_idx = Tpl::getIteri_i0(txt.clone())?;
            txt = dumpClassPart(txt.clone(), i_class__part.clone(), x_idx.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_23(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_25(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_25(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_27(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_27(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_28(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_class__part, tail: rest }, a_options) => {
            let mut x_idx: i32;
            let mut txt = (*txt).clone();
            x_idx = Tpl::getIteri_i0(txt.clone())?;
            txt = dumpClassPart(txt.clone(), i_class__part.clone(), x_idx.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_28(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_30(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Annotation>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_a, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAnnotation(txt.clone(), i_a.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_30(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_32(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_32(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_33(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_33(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_34(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_fn, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpPath(txt.clone(), i_fn.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_34(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_35(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_35(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_36(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_var, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_var.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_36(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_37(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cmt, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_cmt.clone()).clone())?;
            txt = lm_37(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_39(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::EnumLiteral>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_lit, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEnumLiteral(txt.clone(), i_lit.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_39(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_48(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_tv, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_tv.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_48(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_52(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_exp, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_52(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_53(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationItem(txt.clone(), i_eq.clone())?;
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
fn lm_54(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpEquationItem(txt.clone(), i_eq.clone())?;
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
fn lm_55(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmItem(txt.clone(), i_eq.clone())?;
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
fn lm_56(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmItem(txt.clone(), i_eq.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_62(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_arg.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_62(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_79(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpNamedArg(txt.clone(), i_arg.clone())?;
            txt = lm_79(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_9(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Class>>>, in_a_options: Dump::DumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_cls, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpClass(txt.clone(), i_cls.clone(), (literal!(";")).clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_9(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_92(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::ElementArg>, bool)>>, in_a_separator: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_separator.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_earg, i_b), tail: rest }, a_separator) => {
            let mut txt = (*txt).clone();
            txt = dumpElementArg(txt.clone(), i_earg.clone())?;
            txt = fun_91(txt.clone(), i_b.clone(), a_separator.clone())?;
            txt = lm_92(txt.clone(), rest.clone(), a_separator.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn smf_18(in_txt: Tpl::Text, in_it: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_it.clone()) {
        (mut txt, mut i_cmt) => {
            txt = Tpl::writeText(txt.clone(), i_cmt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

