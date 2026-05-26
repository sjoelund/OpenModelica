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
use openmodelica_ast::Absyn;
use openmodelica_frontend::Types;
use openmodelica_frontend::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_susan::Tpl;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_44(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcResultType: i_ty_funcResultType, funcArg: i_ty_funcArg, path: i_path, .. }, tail: rest }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (AbsynUtil::pathLastIdent(i_path.clone())?).clone();
            txt = getCevalScriptInterfaceFunc(txt.clone(), (ret_0.clone()).clone(), i_ty_funcArg.clone(), i_ty_funcResultType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = lm_44(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_44(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn getCevalScriptInterface(mut txt: Tpl::Text, mut a_tys: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_funcs: Tpl::Text;
    l_funcs = lm_44(Tpl::emptyTxt.clone(), a_tys.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("import Absyn;\n")).clone(), (literal!("import AbsynUtil;\n")).clone(), (literal!("import CevalScript;\n")).clone(), (literal!("import Parser;\n")).clone(), (literal!("\n")).clone(), (literal!("protected\n")).clone(), (literal!("\n")).clone(), (literal!("import Values;\n")).clone(), (literal!("import ValuesMake;\n")).clone(), (literal!("import ValuesUtil;\n")).clone(), (literal!("constant Absyn.Msg dummyMsg = Absyn.MSG(SOURCEINFO(\"<interactive>\",false,1,1,1,1,0.0));\n")).clone(), (literal!("\n")).clone(), (literal!("public\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_funcs.clone())?;
    Ok(out_txt)
}

pub fn getInType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Integer")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Boolean")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Real")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list<")).clone() }))?;
            txt = getInType(txt.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("String")).clone() }))?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getInType failed for ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 72, 16), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_47(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.STRING(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.INTEGER(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.BOOL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.REAL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }, a_name) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ValuesMake.makeArray(list(")).clone() }))?;
            txt_0 = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_iter")).clone() }))?;
            (txt, txt_0) = getInValue(txt.clone(), txt_0.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_iter in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.CODE(Absyn.C_TYPENAME(Parser.stringPath(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")))")).clone() }))?;
            txt.clone()
        },
        (txt, i_ty, _) => {
            let mut txt_1: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getInValue failed for ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_1 = Tpl::writeStr(txt_1.clone(), (ret_1.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 84, 16), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getInValue(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    out_txt = fun_47(txt.clone(), a_ty.clone(), a_name.clone())?;
    out_a_name = a_name.clone();
    Ok((out_txt, out_a_name))
}

fn fun_49(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text, mut in_a_varDecl: Tpl::Text, mut in_a_postMatch: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    let mut out_a_postMatch: Tpl::Text;
    (out_txt, out_a_varDecl, out_a_postMatch) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone(), in_a_varDecl.clone(), in_a_postMatch.clone())) {
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name, a_varDecl, a_postMatch) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.STRING(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_varDecl.clone(), a_postMatch.clone())
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name, a_varDecl, a_postMatch) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.INTEGER(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_varDecl.clone(), a_postMatch.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name, a_varDecl, a_postMatch) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.BOOL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_varDecl.clone(), a_postMatch.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name, a_varDecl, a_postMatch) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.REAL(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt.clone(), a_varDecl.clone(), a_postMatch.clone())
        },
        (txt, i_aty @ Deref @ DAE::Type::T_ARRAY { ty: _, .. }, a_name, a_varDecl, a_postMatch) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            let mut a_postMatch = (*a_postMatch).clone();
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.Value ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_arr;")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_postMatch = Tpl::writeText(a_postMatch.clone(), a_name.clone())?;
            a_postMatch = Tpl::writeTok(a_postMatch.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" := ")).clone() }))?;
            txt_0 = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_arr")).clone() }))?;
            (a_postMatch, txt_0) = getOutValueArray(a_postMatch.clone(), txt_0.clone(), i_aty.clone())?;
            a_postMatch = Tpl::writeTok(a_postMatch.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            a_postMatch = Tpl::writeTok(a_postMatch.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_arr")).clone() }))?;
            (txt.clone(), a_varDecl.clone(), a_postMatch.clone())
        },
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name, a_varDecl, a_postMatch) => {
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            let mut a_postMatch = (*a_postMatch).clone();
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Absyn.Path ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_path;")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_postMatch = Tpl::writeText(a_postMatch.clone(), a_name.clone())?;
            a_postMatch = Tpl::writeTok(a_postMatch.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" := AbsynUtil.pathString(")).clone() }))?;
            a_postMatch = Tpl::writeText(a_postMatch.clone(), a_name.clone())?;
            a_postMatch = Tpl::writeTok(a_postMatch.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_path);")).clone() }))?;
            a_postMatch = Tpl::writeTok(a_postMatch.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.CODE(Absyn.C_TYPENAME(path=")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_path))")).clone() }))?;
            (txt.clone(), a_varDecl.clone(), a_postMatch.clone())
        },
        (txt, i_ty, _, a_varDecl, a_postMatch) => {
            let mut txt_1: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getOutValue failed for ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_1 = Tpl::writeStr(txt_1.clone(), (ret_1.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 103, 16), (Tpl::textString(txt_1.clone())?).clone())?;
            (txt.clone(), a_varDecl.clone(), a_postMatch.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecl, out_a_postMatch))
}

pub fn getOutValue(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_varDecl: Tpl::Text, mut a_postMatch: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    let mut out_a_postMatch: Tpl::Text;
    (out_txt, out_a_varDecl, out_a_postMatch) = fun_49(txt.clone(), a_ty.clone(), a_name.clone(), a_varDecl.clone(), a_postMatch.clone())?;
    out_a_name = a_name.clone();
    Ok((out_txt, out_a_name, out_a_varDecl, out_a_postMatch))
}

fn fun_51(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" case Values.STRING() then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".string; end match")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" case Values.INTEGER() then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".integer; end match")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" case Values.BOOL() then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".boolean; end match")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("match ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" case Values.REAL() then ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".real; end match")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }, a_name) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("list(")).clone() }))?;
            txt_0 = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_iter")).clone() }))?;
            (txt, txt_0) = getOutValueArray(txt.clone(), txt_0.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" for ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_iter in ValuesUtil.arrayValues(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ValuesUtil.valString(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_ty, _) => {
            let mut txt_1: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getOutValueArray failed for ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_1 = Tpl::writeStr(txt_1.clone(), (ret_1.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 117, 16), (Tpl::textString(txt_1.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getOutValueArray(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    out_txt = fun_51(txt.clone(), a_ty.clone(), a_name.clone())?;
    out_a_name = a_name.clone();
    Ok((out_txt, out_a_name))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_53(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { ty: i_arg_ty, name: i_arg_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            (txt, _) = getInValue(txt.clone(), Tpl::stringText((i_arg_name.clone()).clone()), i_arg_ty.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_53(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_53(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_54(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_postMatch: Tpl::Text, mut in_a_varDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_postMatch: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    (out_txt, out_a_postMatch, out_a_varDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_postMatch.clone(), in_a_varDecl.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_postMatch, a_varDecl) => {
            (txt.clone(), a_postMatch.clone(), a_varDecl.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }, a_postMatch, a_varDecl) => {
            let mut x_i: i32 = 0;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_postMatch = (*a_postMatch).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("res")).clone() }))?;
            txt_0 = Tpl::writeStr(txt_0.clone(), (intString(x_i.clone())).clone())?;
            (txt, txt_0, a_varDecl, a_postMatch) = getOutValue(txt.clone(), txt_0.clone(), i_ty.clone(), a_varDecl.clone(), a_postMatch.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_postMatch, a_varDecl) = lm_54(txt.clone(), rest.clone(), a_postMatch.clone(), a_varDecl.clone())?;
            (txt.clone(), a_postMatch.clone(), a_varDecl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_postMatch, out_a_varDecl))
}

fn fun_55(mut in_txt: Tpl::Text, mut in_a_res: Arc<DAE::Type>, mut in_a_postMatch: Tpl::Text, mut in_a_varDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_postMatch: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    (out_txt, out_a_postMatch, out_a_varDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_res.clone(), in_a_postMatch.clone(), in_a_varDecl.clone())) {
        (txt, Deref @ DAE::Type::T_TUPLE { types: i_types, .. }, a_postMatch, a_varDecl) => {
            let mut txt = (*txt).clone();
            let mut a_postMatch = (*a_postMatch).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.TUPLE({")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_postMatch, a_varDecl) = lm_54(txt.clone(), i_types.clone(), a_postMatch.clone(), a_varDecl.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("})")).clone() }))?;
            (txt.clone(), a_postMatch.clone(), a_varDecl.clone())
        },
        (txt, Deref @ DAE::Type::T_NORETCALL, a_postMatch, a_varDecl) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Values.NORETCALL()")).clone() }))?;
            (txt.clone(), a_postMatch.clone(), a_varDecl.clone())
        },
        (txt, i_res, a_postMatch, a_varDecl) => {
            let mut txt = (*txt).clone();
            let mut a_postMatch = (*a_postMatch).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            (txt, _, a_varDecl, a_postMatch) = getOutValue(txt.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("res")).clone() })), i_res.clone(), a_varDecl.clone(), a_postMatch.clone())?;
            (txt.clone(), a_postMatch.clone(), a_varDecl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_postMatch, out_a_varDecl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_56(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { name: i_arg_name, ty: i_arg_ty, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("input ")).clone() }))?;
            txt = getInType(txt.clone(), i_arg_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_arg_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_56(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_57(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output ")).clone() }))?;
            txt = getInType(txt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" res")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_i.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_57(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_58(mut in_txt: Tpl::Text, mut in_a_res: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_res.clone())) {
        (txt, Deref @ DAE::Type::T_TUPLE { types: i_types, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_57(txt.clone(), i_types.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_NORETCALL) => {
            txt.clone()
        },
        (txt, i_res) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("output ")).clone() }))?;
            txt = getInType(txt.clone(), i_res.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" res;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_59(mut in_txt: Tpl::Text, mut in_a_varDecl: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_varDecl.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, i_varDecl) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("protected\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), i_varDecl.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getCevalScriptInterfaceFunc(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_args: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut a_res: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_outVals: Tpl::Text;
    let mut l_inVals: Tpl::Text;
    let mut l_postMatch: Tpl::Text;
    let mut l_varDecl: Tpl::Text;
    l_varDecl = Tpl::emptyTxt.clone();
    l_postMatch = Tpl::emptyTxt.clone();
    l_inVals = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_inVals = lm_53(l_inVals.clone(), a_args.clone())?;
    l_inVals = Tpl::popIter(l_inVals.clone())?;
    (l_outVals, l_postMatch, l_varDecl) = fun_55(Tpl::emptyTxt.clone(), a_res.clone(), l_postMatch.clone(), l_varDecl.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_56(out_txt.clone(), a_args.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = fun_58(out_txt.clone(), a_res.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = fun_59(out_txt.clone(), l_varDecl.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("algorithm\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(_,")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_outVals.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") := CevalScript.cevalInteractiveFunctions2(FCore.emptyCache(), FGraph.empty(), \"")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", {")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_inVals.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}, dummyMsg);\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_postMatch.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_61(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_classNameWithColons: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_classNameWithColons.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcResultType: i_ty_funcResultType, funcArg: i_ty_funcArg, path: i_path, .. }, tail: rest }, a_classNameWithColons) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (AbsynUtil::pathLastIdent(i_path.clone())?).clone();
            txt = getQtInterfaceFunc(txt.clone(), (ret_0.clone()).clone(), i_ty_funcArg.clone(), i_ty_funcResultType.clone(), (a_classNameWithColons.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = lm_61(txt.clone(), rest.clone(), (a_classNameWithColons.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_classNameWithColons) => {
            let mut txt = (*txt).clone();
            txt = lm_61(txt.clone(), rest.clone(), (a_classNameWithColons.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn getQtInterface(mut txt: Tpl::Text, mut a_tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut a_classNameWithColons: ArcStr, mut a_className: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_funcs: Tpl::Text;
    l_funcs = lm_61(Tpl::emptyTxt.clone(), a_tys.clone(), (a_classNameWithColons.clone()).clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* generated by OpenModelica */\n")).clone(), (literal!("\n")).clone(), (literal!("#include <stdexcept>\n")).clone(), (literal!("#include \"OpenModelicaScriptingAPIQt.h\"\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_classNameWithColons.clone()).clone())?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_className.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("(threadData_t *td)\n")).clone(), (literal!("  : threadData(td)\n")).clone(), (literal!("{\n")).clone(), (literal!("}\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_funcs.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_63(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_className: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_className.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcResultType: i_ty_funcResultType, funcArg: i_ty_funcArg, path: i_path, .. }, tail: rest }, a_className) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = (AbsynUtil::pathLastIdent(i_path.clone())?).clone();
            txt = getQtInterfaceHeader(txt.clone(), (ret_0.clone()).clone(), (literal!("")).clone(), i_ty_funcArg.clone(), i_ty_funcResultType.clone(), (a_className.clone()).clone(), true)?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = lm_63(txt.clone(), rest.clone(), (a_className.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_className) => {
            let mut txt = (*txt).clone();
            txt = lm_63(txt.clone(), rest.clone(), (a_className.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn getQtInterfaceHeaders(mut txt: Tpl::Text, mut a_tys: Arc<metamodelica::List<Arc<DAE::Type>>>, mut a_className: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_heads: Tpl::Text;
    l_heads = lm_63(Tpl::emptyTxt.clone(), a_tys.clone(), (a_className.clone()).clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* generated by OpenModelica */\n")).clone(), (literal!("#ifndef OpenModelicaScriptingAPIQt__H\n")).clone(), (literal!("#define OpenModelicaScriptingAPIQt__H\n")).clone(), (literal!("\n")).clone(), (literal!("#include <QOpenGLContext> // must be first include to fix undefined GLDEBUGPROC\n")).clone(), (literal!("#include <QtCore>\n")).clone(), (literal!("/* import the scripting here */\n")).clone(), (literal!("#define IMPORT_INTO\n")).clone(), (literal!("#include \"OpenModelicaScriptingAPI.h\"\n")).clone(), (literal!("\n")).clone(), (literal!("class ")).clone()], lastHasNewLine: false }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_className.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" : public QObject\n")).clone(), (literal!("{\n")).clone(), (literal!("  Q_OBJECT\n")).clone(), (literal!("public:\n")).clone(), (literal!("  threadData_t *threadData;\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_className.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("(threadData_t *td);\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_heads.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("signals:\n")).clone(), (literal!("  void logCommand(QString command);\n")).clone(), (literal!("  // elapsed time in seconds\n")).clone(), (literal!("  void logResponse(QString command, QString response, double elapsed);\n")).clone(), (literal!("  void throwException(QString exception);\n")).clone(), (literal!("};\n")).clone(), (literal!("\n")).clone(), (literal!("#endif /* OpenModelicaScriptingAPIQt__H */")).clone()], lastHasNewLine: false }))?;
    Ok(out_txt)
}

pub fn getQtType(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone())) {
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("QString")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }) => {
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
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("QList<")).clone() }))?;
            txt = getQtType(txt.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" >")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("QString")).clone() }))?;
            txt.clone()
        },
        (txt, i_ty) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getQtType failed for ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 211, 16), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getQtTupleTypeOutputNameHelper(mut in_txt: Tpl::Text, mut in_a_names: Option<Arc<metamodelica::List<ArcStr>>>, mut in_a_index: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_names.clone(), in_a_index.clone())) {
        (txt, Some(i_lst), a_index) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            ret_0 = ((i_lst.clone()).get(a_index.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_index) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("res")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getQtTupleTypeOutputName(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_index: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_index.clone())) {
        (txt, Deref @ DAE::Type::T_TUPLE { names: i_names, .. }, a_index) => {
            let mut txt = (*txt).clone();
            txt = getQtTupleTypeOutputNameHelper(txt.clone(), i_names.clone(), a_index.clone())?;
            txt.clone()
        },
        (txt, _, a_index) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("res")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_index.clone())).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_68(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_res: Arc<DAE::Type>, mut in_a_index: i32, mut in_a_name: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    (out_txt, out_a_name) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_res.clone(), in_a_index.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_res, a_index, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(")).clone() }))?;
            txt = getQtTupleTypeOutputName(txt.clone(), a_res.clone(), a_index.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            (txt.clone(), a_name.clone())
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_res, a_index, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(\"\\\"\" + ")).clone() }))?;
            txt = getQtTupleTypeOutputName(txt.clone(), a_res.clone(), a_index.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + \"\\\"\");")).clone() }))?;
            (txt.clone(), a_name.clone())
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_res, a_index, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(QString::number(")).clone() }))?;
            txt = getQtTupleTypeOutputName(txt.clone(), a_res.clone(), a_index.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            (txt.clone(), a_name.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_res, a_index, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(QString::number(")).clone() }))?;
            txt = getQtTupleTypeOutputName(txt.clone(), a_res.clone(), a_index.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            (txt.clone(), a_name.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_res, a_index, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(")).clone() }))?;
            txt = getQtTupleTypeOutputName(txt.clone(), a_res.clone(), a_index.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ? \"true\" : \"false\");")).clone() }))?;
            (txt.clone(), a_name.clone())
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }, a_res, a_index, a_name) => {
            let mut l_counter: Tpl::Text;
            let mut l_elt: Tpl::Text;
            let mut l_varName: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_name = (*a_name).clone();
            l_varName = getQtTupleTypeOutputName(Tpl::emptyTxt.clone(), a_res.clone(), a_index.clone())?;
            l_elt = Tpl::writeText(Tpl::emptyTxt.clone(), l_varName.clone())?;
            l_elt = Tpl::writeTok(l_elt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_elt")).clone() }))?;
            l_counter = Tpl::writeText(Tpl::emptyTxt.clone(), l_varName.clone())?;
            l_counter = Tpl::writeTok(l_counter.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_i")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".append(\"{\");\n")).clone(), (literal!("int ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" = 0;\n")).clone(), (literal!("foreach(")).clone()], lastHasNewLine: false }))?;
            txt = getQtType(txt.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".append(\",\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            (txt, l_elt, a_name) = getQtResponseLogText(txt.clone(), l_elt.clone(), i_aty_ty.clone(), a_name.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("++;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(\"}\");")).clone() }))?;
            (txt.clone(), a_name.clone())
        },
        (txt, i_ty, _, _, a_name) => {
            let mut txt_3: Tpl::Text;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_3 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("structToString failed for ")).clone() }))?;
            ret_3 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_3 = Tpl::writeStr(txt_3.clone(), (ret_3.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 256, 16), (Tpl::textString(txt_3.clone())?).clone())?;
            (txt.clone(), a_name.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_name))
}

pub fn structToString(mut txt: Tpl::Text, mut a_res: Arc<DAE::Type>, mut a_ty: Arc<DAE::Type>, mut a_index: i32, mut a_name: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    (out_txt, out_a_name) = fun_68(txt.clone(), a_ty.clone(), a_res.clone(), a_index.clone(), a_name.clone())?;
    Ok((out_txt, out_a_name))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_70(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { name: i_arg_name, ty: i_arg_ty, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = getQtType(txt.clone(), i_arg_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_arg_name.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_70(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_70(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_71(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_res: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_res.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }, a_res) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            txt = getQtType(txt.clone(), i_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = getQtTupleTypeOutputName(txt.clone(), a_res.clone(), x_i.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_71(txt.clone(), rest.clone(), a_res.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_72(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_res: Arc<DAE::Type>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_res.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_ty, tail: rest }, a_res) => {
            let mut x_i: i32 = 0;
            let mut txt = (*txt).clone();
            x_i = Tpl::getIteri_i0(txt.clone())?;
            (txt, _) = structToString(txt.clone(), a_res.clone(), i_ty.clone(), x_i.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("resultBuffer")).clone() })))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_72(txt.clone(), rest.clone(), a_res.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_73(mut in_txt: Tpl::Text, mut in_a_addStructs: bool, mut in_a_res: Arc<DAE::Type>, mut in_a_types: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_name: ArcStr, mut in_a_prefix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_addStructs.clone(), in_a_res.clone(), in_a_types.clone(), in_a_name.clone(), in_a_prefix.clone())) {
        (txt, false, _, _, a_name, a_prefix) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_prefix.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_res")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_res, a_types, a_name, _) => {
            let mut ret_1: Arc<Tpl::StringToken> = Arc::new(Tpl::StringToken::ST_NEW_LINE);
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("typedef struct ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_res {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_71(txt.clone(), a_types.clone(), a_res.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("QString toString() {\n")).clone(), (literal!("  QString resultBuffer = \"(\";\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("resultBuffer.append(\",\");")).clone() }))?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            ret_1 = Tpl::textStrTok(txt_0.clone())?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(ret_1.clone()), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_72(txt.clone(), a_types.clone(), a_res.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("resultBuffer.append(\")\");\n")).clone(), (literal!("return resultBuffer;\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("} ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_res;\n")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_res")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_74(mut in_txt: Tpl::Text, mut in_a_res: Arc<DAE::Type>, mut in_a_name: ArcStr, mut in_a_prefix: ArcStr, mut in_a_addStructs: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_res.clone(), in_a_name.clone(), in_a_prefix.clone(), in_a_addStructs.clone())) {
        (txt, i_res @ Deref @ DAE::Type::T_TUPLE { types: i_types, .. }, a_name, a_prefix, a_addStructs) => {
            let mut txt = (*txt).clone();
            txt = fun_73(txt.clone(), a_addStructs.clone(), i_res.clone(), i_types.clone(), (a_name.clone()).clone(), (a_prefix.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_NORETCALL, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void")).clone() }))?;
            txt.clone()
        },
        (txt, i_res, _, _, _) => {
            let mut txt = (*txt).clone();
            txt = getQtType(txt.clone(), i_res.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getQtInterfaceHeader(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_prefix: ArcStr, mut a_args: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut a_res: Arc<DAE::Type>, mut a_className: ArcStr, mut a_addStructs: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_outType: Tpl::Text;
    let mut l_inTypes: Tpl::Text;
    l_inTypes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_inTypes = lm_70(l_inTypes.clone(), a_args.clone())?;
    l_inTypes = Tpl::popIter(l_inTypes.clone())?;
    l_outType = fun_74(Tpl::emptyTxt.clone(), a_res.clone(), (a_name.clone()).clone(), (a_prefix.clone()).clone(), a_addStructs.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_outType.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_prefix.clone()).clone())?;
    out_txt = Tpl::writeStr(out_txt.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_inTypes.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
    Ok(out_txt)
}

fn fun_76(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text, mut in_a_varDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    (out_txt, out_a_varDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone(), in_a_varDecl.clone())) {
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name, a_varDecl) => {
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("QByteArray ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_utf8 = ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".toUtf8();")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_scon(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_utf8.constData())")).clone() }))?;
            (txt.clone(), a_varDecl.clone())
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name, a_varDecl) => {
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("QByteArray ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_utf8 = ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".toUtf8();")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_scon(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_utf8.constData())")).clone() }))?;
            (txt.clone(), a_varDecl.clone())
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name, a_varDecl) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            (txt.clone(), a_varDecl.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name, a_varDecl) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            (txt.clone(), a_varDecl.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name, a_varDecl) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            (txt.clone(), a_varDecl.clone())
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }, a_name, a_varDecl) => {
            let mut l_i: Tpl::Text;
            let mut txt_3: Tpl::Text;
            let mut l_body: Tpl::Text;
            let mut l_elt: Tpl::Text;
            let mut l_varDecl2: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            l_varDecl2 = Tpl::emptyTxt.clone();
            l_elt = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            l_elt = Tpl::writeTok(l_elt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_elt")).clone() }))?;
            (txt_3, l_elt, l_varDecl2) = getQtInArg(Tpl::emptyTxt.clone(), l_elt.clone(), i_aty_ty.clone(), l_varDecl2.clone())?;
            (l_body, txt_3) = getQtInArgBoxed(Tpl::emptyTxt.clone(), txt_3.clone(), i_aty_ty.clone())?;
            l_i = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            l_i = Tpl::writeTok(l_i.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_i")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void *")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("_lst = mmc_mk_nil();\n")).clone(), (literal!("for (int ")).clone()], lastHasNewLine: false }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), l_i.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".size()-1; ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), l_i.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">=0; ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), l_i.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("--) {\n")).clone() }))?;
            a_varDecl = Tpl::pushBlock(a_varDecl.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_varDecl = getQtType(a_varDecl.clone(), i_aty_ty.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), l_elt.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), l_i.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("];\n")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), l_varDecl2.clone())?;
            a_varDecl = Tpl::softNewLine(a_varDecl.clone())?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_lst = mmc_mk_cons(")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), l_body.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_name.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_lst);\n")).clone() }))?;
            a_varDecl = Tpl::popBlock(a_varDecl.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_lst")).clone() }))?;
            (txt.clone(), a_varDecl.clone())
        },
        (txt, i_ty, _, a_varDecl) => {
            let mut txt_5: Tpl::Text;
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_5 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getQtInArg failed for ")).clone() }))?;
            ret_5 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_5 = Tpl::writeStr(txt_5.clone(), (ret_5.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 311, 16), (Tpl::textString(txt_5.clone())?).clone())?;
            (txt.clone(), a_varDecl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_varDecl))
}

pub fn getQtInArg(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_varDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    (out_txt, out_a_varDecl) = fun_76(txt.clone(), a_ty.clone(), a_name.clone(), a_varDecl.clone())?;
    out_a_name = a_name.clone();
    Ok((out_txt, out_a_name, out_a_varDecl))
}

fn fun_78(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone())) {
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: _, .. }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_icon(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mmc_mk_rcon(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, i_ty, _) => {
            let mut txt_0: Tpl::Text;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getQtInArgBoxed failed for ")).clone() }))?;
            ret_0 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_0 = Tpl::writeStr(txt_0.clone(), (ret_0.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 323, 16), (Tpl::textString(txt_0.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getQtInArgBoxed(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    out_txt = fun_78(txt.clone(), a_ty.clone(), a_name.clone())?;
    out_a_name = a_name.clone();
    Ok((out_txt, out_a_name))
}

fn fun_80(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text, mut in_a_commandLog: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_commandLog: Tpl::Text;
    (out_txt, out_a_commandLog) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone(), in_a_commandLog.clone())) {
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name, a_commandLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_commandLog.clone())
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name, a_commandLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(\"\\\"\" + ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + \"\\\"\");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_commandLog.clone())
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name, a_commandLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(QString::number(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_commandLog.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name, a_commandLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(QString::number(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_commandLog.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name, a_commandLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ? \"true\" : \"false\");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_commandLog.clone())
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }, a_name, a_commandLog) => {
            let mut l_counter: Tpl::Text;
            let mut l_elt: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_commandLog = (*a_commandLog).clone();
            l_elt = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            l_elt = Tpl::writeTok(l_elt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_elt")).clone() }))?;
            l_counter = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            l_counter = Tpl::writeTok(l_counter.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_i")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".append(\"{\");\n")).clone(), (literal!("int ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" = 0;\n")).clone(), (literal!("foreach(")).clone()], lastHasNewLine: false }))?;
            txt = getQtType(txt.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".append(\",\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            (txt, l_elt, a_commandLog) = getQtCommandLogText(txt.clone(), l_elt.clone(), i_aty_ty.clone(), a_commandLog.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("++;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(\"}\");")).clone() }))?;
            (txt.clone(), a_commandLog.clone())
        },
        (txt, i_ty, _, a_commandLog) => {
            let mut txt_2: Tpl::Text;
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getQtCommandLogText failed for ")).clone() }))?;
            ret_2 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_2 = Tpl::writeStr(txt_2.clone(), (ret_2.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 349, 16), (Tpl::textString(txt_2.clone())?).clone())?;
            (txt.clone(), a_commandLog.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_commandLog))
}

pub fn getQtCommandLogText(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_commandLog: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    let mut out_a_commandLog: Tpl::Text;
    (out_txt, out_a_commandLog) = fun_80(txt.clone(), a_ty.clone(), a_name.clone(), a_commandLog.clone())?;
    out_a_name = a_name.clone();
    Ok((out_txt, out_a_name, out_a_commandLog))
}

fn fun_82(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text, mut in_a_shortName: Tpl::Text, mut in_a_varDecl: Tpl::Text, mut in_a_postCall: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    let mut out_a_shortName: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    let mut out_a_postCall: Tpl::Text;
    (out_txt, out_a_name, out_a_shortName, out_a_varDecl, out_a_postCall) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone(), in_a_shortName.clone(), in_a_varDecl.clone(), in_a_postCall.clone())) {
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name, a_shortName, a_varDecl, a_postCall) => {
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            let mut a_postCall = (*a_postCall).clone();
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void *")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_shortName.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm = NULL;")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_postCall = Tpl::writeText(a_postCall.clone(), a_name.clone())?;
            a_postCall = Tpl::writeTok(a_postCall.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = QString::fromUtf8((char*)MMC_STRINGDATA(")).clone() }))?;
            a_postCall = Tpl::writeText(a_postCall.clone(), a_shortName.clone())?;
            a_postCall = Tpl::writeTok(a_postCall.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm));")).clone() }))?;
            a_postCall = Tpl::writeTok(a_postCall.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_shortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm")).clone() }))?;
            (txt.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name, a_shortName, a_varDecl, a_postCall) => {
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            let mut a_postCall = (*a_postCall).clone();
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void *")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_shortName.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm = NULL;")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_postCall = Tpl::writeText(a_postCall.clone(), a_name.clone())?;
            a_postCall = Tpl::writeTok(a_postCall.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = QString::fromUtf8((char*)MMC_STRINGDATA(")).clone() }))?;
            a_postCall = Tpl::writeText(a_postCall.clone(), a_shortName.clone())?;
            a_postCall = Tpl::writeTok(a_postCall.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm));")).clone() }))?;
            a_postCall = Tpl::writeTok(a_postCall.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), a_shortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm")).clone() }))?;
            (txt.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name, a_shortName, a_varDecl, a_postCall) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            (txt.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name, a_shortName, a_varDecl, a_postCall) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            (txt.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name, a_shortName, a_varDecl, a_postCall) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            (txt.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())
        },
        (txt, i_aty @ Deref @ DAE::Type::T_ARRAY { ty: _, .. }, a_name, a_shortName, a_varDecl, a_postCall) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_name = (*a_name).clone();
            let mut a_shortName = (*a_shortName).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            let mut a_postCall = (*a_postCall).clone();
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("void *")).clone() }))?;
            a_varDecl = Tpl::writeText(a_varDecl.clone(), a_shortName.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm = NULL;")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt_0 = Tpl::writeText(Tpl::emptyTxt.clone(), a_shortName.clone())?;
            txt_0 = Tpl::writeTok(txt_0.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm")).clone() }))?;
            (a_postCall, a_name, a_shortName, txt_0) = getQtOutArgArray(a_postCall.clone(), a_name.clone(), a_shortName.clone(), txt_0.clone(), i_aty.clone())?;
            txt = Tpl::writeText(txt.clone(), a_shortName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_mm")).clone() }))?;
            (txt.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())
        },
        (txt, i_ty, a_name, a_shortName, a_varDecl, a_postCall) => {
            let mut txt_1: Tpl::Text;
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getQtOutArg failed for ")).clone() }))?;
            ret_1 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_1 = Tpl::writeStr(txt_1.clone(), (ret_1.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 367, 16), (Tpl::textString(txt_1.clone())?).clone())?;
            (txt.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_name, out_a_shortName, out_a_varDecl, out_a_postCall))
}

pub fn getQtOutArg(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_shortName: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_varDecl: Tpl::Text, mut a_postCall: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    let mut out_a_shortName: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    let mut out_a_postCall: Tpl::Text;
    (out_txt, out_a_name, out_a_shortName, out_a_varDecl, out_a_postCall) = fun_82(txt.clone(), a_ty.clone(), a_name.clone(), a_shortName.clone(), a_varDecl.clone(), a_postCall.clone())?;
    Ok((out_txt, out_a_name, out_a_shortName, out_a_varDecl, out_a_postCall))
}

fn fun_84(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text, mut in_a_shortName: Tpl::Text, mut in_a_mm: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone(), in_a_shortName.clone(), in_a_mm.clone())) {
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name, _, a_mm) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MMC_STRINGDATA(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name, _, a_mm) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MMC_STRINGDATA(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name, _, a_mm) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = mmc_unbox_integer(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name, _, a_mm) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = mmc_unbox_boolean(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name, _, a_mm) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = mmc_unbox_real(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }, a_name, a_shortName, a_mm) => {
            let mut txt_1: Tpl::Text;
            let mut l_elt: Tpl::Text;
            let mut txt = (*txt).clone();
            l_elt = Tpl::writeText(Tpl::emptyTxt.clone(), a_shortName.clone())?;
            l_elt = Tpl::writeTok(l_elt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_elt")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".clear();\n")).clone(), (literal!("while (!listEmpty(")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(")) {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = getQtType(txt.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() }))?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("MMC_CAR(")).clone() }))?;
            txt_1 = Tpl::writeText(txt_1.clone(), a_mm.clone())?;
            txt_1 = Tpl::writeTok(txt_1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            (txt, _, l_elt, txt_1) = getQtOutArgArray(txt.clone(), l_elt.clone(), l_elt.clone(), txt_1.clone(), i_aty_ty.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".push_back(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MMC_CDR(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_mm.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, i_ty, _, _, _) => {
            let mut txt_2: Tpl::Text;
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getOutValueArray failed for ")).clone() }))?;
            ret_2 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_2 = Tpl::writeStr(txt_2.clone(), (ret_2.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 389, 16), (Tpl::textString(txt_2.clone())?).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getQtOutArgArray(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_shortName: Tpl::Text, mut a_mm: Tpl::Text, mut a_ty: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    let mut out_a_shortName: Tpl::Text;
    let mut out_a_mm: Tpl::Text;
    out_txt = fun_84(txt.clone(), a_ty.clone(), a_name.clone(), a_shortName.clone(), a_mm.clone())?;
    out_a_name = a_name.clone();
    out_a_shortName = a_shortName.clone();
    out_a_mm = a_mm.clone();
    Ok((out_txt, out_a_name, out_a_shortName, out_a_mm))
}

fn fun_86(mut in_txt: Tpl::Text, mut in_a_ty: Arc<DAE::Type>, mut in_a_name: Tpl::Text, mut in_a_responseLog: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_responseLog: Tpl::Text;
    (out_txt, out_a_responseLog) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ty.clone(), in_a_name.clone(), in_a_responseLog.clone())) {
        (txt, Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME }, a_name, a_responseLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_responseLog.clone())
        },
        (txt, Deref @ DAE::Type::T_STRING { varLst: _ }, a_name, a_responseLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(\"\\\"\" + ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" + \"\\\"\");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_responseLog.clone())
        },
        (txt, Deref @ DAE::Type::T_INTEGER { varLst: _ }, a_name, a_responseLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(QString::number(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_responseLog.clone())
        },
        (txt, Deref @ DAE::Type::T_REAL { varLst: _ }, a_name, a_responseLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(QString::number(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_responseLog.clone())
        },
        (txt, Deref @ DAE::Type::T_BOOL { varLst: _ }, a_name, a_responseLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ? \"true\" : \"false\");")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_responseLog.clone())
        },
        (txt, Deref @ DAE::Type::T_TUPLE { types: _, .. }, a_name, a_responseLog) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".toString());")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_responseLog.clone())
        },
        (txt, Deref @ DAE::Type::T_ARRAY { ty: i_aty_ty, .. }, a_name, a_responseLog) => {
            let mut l_counter: Tpl::Text;
            let mut l_elt: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_responseLog = (*a_responseLog).clone();
            l_elt = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            l_elt = Tpl::writeTok(l_elt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_elt")).clone() }))?;
            l_counter = Tpl::writeText(Tpl::emptyTxt.clone(), a_name.clone())?;
            l_counter = Tpl::writeTok(l_counter.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_i")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(".append(\"{\");\n")).clone(), (literal!("int ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" = 0;\n")).clone(), (literal!("foreach(")).clone()], lastHasNewLine: false }))?;
            txt = getQtType(txt.clone(), i_aty_ty.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_elt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if (")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(".append(\",\");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            (txt, l_elt, a_responseLog) = getQtResponseLogText(txt.clone(), l_elt.clone(), i_aty_ty.clone(), a_responseLog.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_counter.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("++;\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("}\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_responseLog.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".append(\"}\");")).clone() }))?;
            (txt.clone(), a_responseLog.clone())
        },
        (txt, i_ty, _, a_responseLog) => {
            let mut txt_2: Tpl::Text;
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt_2 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("getQtResponseLogText failed for ")).clone() }))?;
            ret_2 = (TypesDump::unparseType(i_ty.clone())?).clone();
            txt_2 = Tpl::writeStr(txt_2.clone(), (ret_2.clone()).clone())?;
            txt = CodegenUtil::error(txt.clone(), Tpl::sourceInfo((literal!("GenerateAPIFunctionsTpl.tpl")).clone(), 416, 16), (Tpl::textString(txt_2.clone())?).clone())?;
            (txt.clone(), a_responseLog.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_responseLog))
}

pub fn getQtResponseLogText(mut txt: Tpl::Text, mut a_name: Tpl::Text, mut a_ty: Arc<DAE::Type>, mut a_responseLog: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_name: Tpl::Text;
    let mut out_a_responseLog: Tpl::Text;
    (out_txt, out_a_responseLog) = fun_86(txt.clone(), a_ty.clone(), a_name.clone(), a_responseLog.clone())?;
    out_a_name = a_name.clone();
    Ok((out_txt, out_a_name, out_a_responseLog))
}

fn lm_88(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut in_a_varDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    (out_txt, out_a_varDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_varDecl.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_varDecl) => {
            (txt.clone(), a_varDecl.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { ty: i_arg_ty, name: i_arg_name, .. }, tail: rest }, a_varDecl) => {
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            (txt, _, a_varDecl) = getQtInArg(txt.clone(), Tpl::stringText((i_arg_name.clone()).clone()), i_arg_ty.clone(), a_varDecl.clone())?;
            (txt, a_varDecl) = lm_88(txt.clone(), rest.clone(), a_varDecl.clone())?;
            (txt.clone(), a_varDecl.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_varDecl) => {
            let mut txt = (*txt).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            (txt, a_varDecl) = lm_88(txt.clone(), rest.clone(), a_varDecl.clone())?;
            (txt.clone(), a_varDecl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_varDecl))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_89(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::FuncArg>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::FuncArg { ty: i_arg_ty, name: i_arg_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            (txt, _, _) = getQtCommandLogText(txt.clone(), Tpl::stringText((i_arg_name.clone()).clone()), i_arg_ty.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("commandLog")).clone() })))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_89(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_89(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn lm_90(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>, mut in_a_postCall: Tpl::Text, mut in_a_varDecl: Tpl::Text, mut in_a_res: Arc<DAE::Type>) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_postCall: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    (out_txt, out_a_postCall, out_a_varDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_postCall.clone(), in_a_varDecl.clone(), in_a_res.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_postCall, a_varDecl, _) => {
            (txt.clone(), a_postCall.clone(), a_varDecl.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_t, tail: rest }, a_postCall, a_varDecl, a_res) => {
            let mut x_i1: i32 = 0;
            let mut txt_1: Tpl::Text;
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_postCall = (*a_postCall).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", &")).clone() }))?;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("result.")).clone() }))?;
            txt_0 = getQtTupleTypeOutputName(txt_0.clone(), a_res.clone(), x_i1.clone())?;
            txt_1 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out")).clone() }))?;
            txt_1 = Tpl::writeStr(txt_1.clone(), (intString(x_i1.clone())).clone())?;
            (txt, txt_0, txt_1, a_varDecl, a_postCall) = getQtOutArg(txt.clone(), txt_0.clone(), txt_1.clone(), i_t.clone(), a_varDecl.clone(), a_postCall.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            (txt, a_postCall, a_varDecl) = lm_90(txt.clone(), rest.clone(), a_postCall.clone(), a_varDecl.clone(), a_res.clone())?;
            (txt.clone(), a_postCall.clone(), a_varDecl.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_postCall, out_a_varDecl))
}

fn fun_91(mut in_txt: Tpl::Text, mut in_a_res: Arc<DAE::Type>, mut in_a_postCall: Tpl::Text, mut in_a_outArg: Tpl::Text, mut in_a_responseLog: Tpl::Text, mut in_a_name: ArcStr, mut in_a_varDecl: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_postCall: Tpl::Text;
    let mut out_a_outArg: Tpl::Text;
    let mut out_a_responseLog: Tpl::Text;
    let mut out_a_varDecl: Tpl::Text;
    (out_txt, out_a_postCall, out_a_outArg, out_a_responseLog, out_a_varDecl) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_res.clone(), in_a_postCall.clone(), in_a_outArg.clone(), in_a_responseLog.clone(), in_a_name.clone(), in_a_varDecl.clone())) {
        (txt, Deref @ DAE::Type::T_NORETCALL, a_postCall, a_outArg, a_responseLog, _, a_varDecl) => {
            (txt.clone(), a_postCall.clone(), a_outArg.clone(), a_responseLog.clone(), a_varDecl.clone())
        },
        (txt, i_t @ Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::List::Cons { head: i_type1, tail: i_types2 }, .. }, a_postCall, a_outArg, a_responseLog, a_name, a_varDecl) => {
            let mut txt_0: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_postCall = (*a_postCall).clone();
            let mut a_outArg = (*a_outArg).clone();
            let mut a_responseLog = (*a_responseLog).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            a_varDecl = Tpl::writeStr(a_varDecl.clone(), (a_name.clone()).clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_res result;")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (a_responseLog, _, _) = getQtResponseLogText(a_responseLog.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("result")).clone() })), i_t.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("responseLog")).clone() })))?;
            txt_0 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("result.")).clone() }))?;
            txt_0 = getQtTupleTypeOutputName(txt_0.clone(), i_t.clone(), 1)?;
            (a_outArg, txt_0, _, a_varDecl, a_postCall) = getQtOutArg(a_outArg.clone(), txt_0.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("out1")).clone() })), i_type1.clone(), a_varDecl.clone(), a_postCall.clone())?;
            a_outArg = Tpl::writeTok(a_outArg.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 2, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            (txt, a_postCall, a_varDecl) = lm_90(txt.clone(), i_types2.clone(), a_postCall.clone(), a_varDecl.clone(), i_t.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            (txt.clone(), a_postCall.clone(), a_outArg.clone(), a_responseLog.clone(), a_varDecl.clone())
        },
        (txt, i_res, a_postCall, a_outArg, a_responseLog, _, a_varDecl) => {
            let mut a_postCall = (*a_postCall).clone();
            let mut a_outArg = (*a_outArg).clone();
            let mut a_responseLog = (*a_responseLog).clone();
            let mut a_varDecl = (*a_varDecl).clone();
            a_varDecl = getQtType(a_varDecl.clone(), i_res.clone())?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" result;")).clone() }))?;
            a_varDecl = Tpl::writeTok(a_varDecl.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (a_responseLog, _, _) = getQtResponseLogText(a_responseLog.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("result")).clone() })), i_res.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("responseLog")).clone() })))?;
            (a_outArg, _, _, a_varDecl, a_postCall) = getQtOutArg(a_outArg.clone(), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("result")).clone() })), Tpl::strTokText(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("result")).clone() })), i_res.clone(), a_varDecl.clone(), a_postCall.clone())?;
            a_outArg = Tpl::writeTok(a_outArg.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            (txt.clone(), a_postCall.clone(), a_outArg.clone(), a_responseLog.clone(), a_varDecl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_postCall, out_a_outArg, out_a_responseLog, out_a_varDecl))
}

fn fun_92(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_commandLog: Tpl::Text, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_commandLog.clone(), in_a_name.clone()) {
        (mut txt, false, _, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("emit logCommand(\"")).clone() }))?;
            txt = CodegenUtil::replaceDotAndUnderscore(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("()\");")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_commandLog, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("QString commandLog;\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_commandLog.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("emit logCommand(\"")).clone() }))?;
            txt = CodegenUtil::replaceDotAndUnderscore(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(\"+commandLog+\")\");")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_93(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_name.clone()) {
        (mut txt, false, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("emit logResponse(\"")).clone() }))?;
            txt = CodegenUtil::replaceDotAndUnderscore(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("()\", responseLog, elapsed);")).clone() }))?;
            txt.clone()
        },
        (mut txt, _, mut a_name) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("emit logResponse(\"")).clone() }))?;
            txt = CodegenUtil::replaceDotAndUnderscore(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(\"+commandLog+\")\", responseLog, elapsed);")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_94(mut in_txt: Tpl::Text, mut in_a_outArg: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_outArg.clone())) {
        (txt, Tpl::Text::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => {
            txt.clone()
        },
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return result;")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn getQtInterfaceFunc(mut txt: Tpl::Text, mut a_name: ArcStr, mut a_args: Arc<metamodelica::List<Arc<DAE::FuncArg>>>, mut a_res: Arc<DAE::Type>, mut a_className: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_12: bool = false;
    let mut ret_11: i32 = 0;
    let mut ret_10: bool = false;
    let mut ret_9: i32 = 0;
    let mut l_outArgs: Tpl::Text;
    let mut l_outArg: Tpl::Text;
    let mut ret_6: Arc<Tpl::StringToken> = Arc::new(Tpl::StringToken::ST_NEW_LINE);
    let mut txt_5: Tpl::Text;
    let mut l_commandLog: Tpl::Text;
    let mut l_inArgs: Tpl::Text;
    let mut l_postCall: Tpl::Text;
    let mut l_responseLog: Tpl::Text;
    let mut l_varDecl: Tpl::Text;
    l_varDecl = Tpl::emptyTxt.clone();
    l_responseLog = Tpl::emptyTxt.clone();
    l_postCall = Tpl::emptyTxt.clone();
    (l_inArgs, l_varDecl) = lm_88(Tpl::emptyTxt.clone(), a_args.clone(), l_varDecl.clone())?;
    l_commandLog = Tpl::emptyTxt.clone();
    txt_5 = Tpl::writeTok(Tpl::emptyTxt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("commandLog.append(\",\");")).clone() }))?;
    txt_5 = Tpl::writeTok(txt_5.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    ret_6 = Tpl::textStrTok(txt_5.clone())?;
    l_commandLog = Tpl::pushIter(l_commandLog.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(ret_6.clone()), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_commandLog = lm_89(l_commandLog.clone(), a_args.clone())?;
    l_commandLog = Tpl::popIter(l_commandLog.clone())?;
    l_outArg = Tpl::emptyTxt.clone();
    (l_outArgs, l_postCall, l_outArg, l_responseLog, l_varDecl) = fun_91(Tpl::emptyTxt.clone(), a_res.clone(), l_postCall.clone(), l_outArg.clone(), l_responseLog.clone(), (a_name.clone()).clone(), l_varDecl.clone())?;
    out_txt = getQtInterfaceHeader(txt.clone(), (a_name.clone()).clone(), (a_className.clone()).clone(), a_args.clone(), a_res.clone(), (a_className.clone()).clone(), false)?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("{\n")).clone(), (literal!("  QElapsedTimer commandTime;\n")).clone(), (literal!("  commandTime.start();\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    ret_9 = (a_args.clone().len() as i32);
    ret_10 = intGt(ret_9.clone(), 0);
    out_txt = fun_92(out_txt.clone(), ret_10.clone(), l_commandLog.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_varDecl.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("try {\n")).clone(), (literal!("  MMC_TRY_TOP_INTERNAL()\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_outArg.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("omc_OpenModelicaScriptingAPI_")).clone() }))?;
    out_txt = CodegenUtil::replaceDotAndUnderscore(out_txt.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(threadData")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_inArgs.clone())?;
    out_txt = Tpl::writeText(out_txt.clone(), l_outArgs.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_postCall.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("MMC_CATCH_TOP()\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("} catch(std::exception &exception) {\n")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("emit throwException(QString(\"")).clone() }))?;
    out_txt = CodegenUtil::replaceDotAndUnderscore(out_txt.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" failed. %1\").arg(exception.what()));\n")).clone() }))?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("\n")).clone(), (literal!("QString responseLog;\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_responseLog.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("double elapsed = (double)commandTime.elapsed() / 1000.0;\n")).clone() }))?;
    ret_11 = (a_args.clone().len() as i32);
    ret_12 = intGt(ret_11.clone(), 0);
    out_txt = fun_93(out_txt.clone(), ret_12.clone(), (a_name.clone()).clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
    out_txt = fun_94(out_txt.clone(), l_outArg.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
    Ok(out_txt)
}

