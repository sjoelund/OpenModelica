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
use openmodelica_frontend::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::Tpl;
use openmodelica_util::System;
use openmodelica_util::Util;

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_43(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cl, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = classExternalHeader(txt.clone(), i_cl.clone(), (literal!("")).clone())?;
            txt = lm_43(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn programExternalHeader(mut txt: Tpl::Text, mut a_program: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* Automatically generated header for external MetaModelica functions */\n")).clone(), (literal!("#ifdef __cplusplus\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
    out_txt = lm_43(out_txt.clone(), a_program.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef __cplusplus\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_45(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Var>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Var { name: i_var_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_var_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_45(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_45(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_46(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_fieldsStr: Tpl::Text, mut in_a_nElts: Tpl::Text, mut in_a_omcname: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fieldsStr.clone(), in_a_nElts.clone(), in_a_omcname.clone())) {
        (txt, Deref @ "0", _, _, a_omcname) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD_METARECORD_DEFINITIONS const char* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__desc__fields[1] = {\"no fields\"};")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_fieldsStr, a_nElts, a_omcname) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD_METARECORD_DEFINITIONS const char* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__desc__fields[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nElts.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fieldsStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("};")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_47(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ DAE::Type::T_METARECORD { path: i_path, fields: i_ty_fields, .. }, tail: rest }) => {
            let mut ret_9: ArcStr = arcstr::literal!("");
            let mut str_8: ArcStr = arcstr::literal!("");
            let mut l_fieldsDescription: Tpl::Text;
            let mut ret_6: i32 = 0;
            let mut l_nElts: Tpl::Text;
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut l_omcname: Tpl::Text;
            let mut l_fieldsStr: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fieldsStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_fieldsStr = lm_45(l_fieldsStr.clone(), i_ty_fields.clone())?;
            l_fieldsStr = Tpl::popIter(l_fieldsStr.clone())?;
            ret_2 = (AbsynUtil::pathString(i_path.clone(), (literal!("$")).clone(), false, false)?).clone();
            ret_3 = (System::stringReplace((ret_2.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            ret_4 = (System::stringReplace((ret_3.clone()).clone(), (literal!("$")).clone(), (literal!("_")).clone())?).clone();
            l_omcname = Tpl::writeStr(Tpl::emptyTxt.clone(), (ret_4.clone()).clone())?;
            ret_6 = (i_ty_fields.clone().len() as i32);
            l_nElts = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_6.clone())).clone())?;
            str_8 = (Tpl::textString(l_nElts.clone())?).clone();
            l_fieldsDescription = fun_46(Tpl::emptyTxt.clone(), (str_8.clone()).clone(), l_fieldsStr.clone(), l_nElts.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef ADD_METARECORD_DEFINITIONS\n")).clone(), (literal!("#ifndef ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("__desc_added\n")).clone(), (literal!("#define ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("__desc_added\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fieldsDescription.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD_METARECORD_DEFINITIONS struct record_description ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("__desc = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\n")).clone(), (literal!("\"")).clone()], lastHasNewLine: false }))?;
            ret_9 = (AbsynUtil::pathString(i_path.clone(), (literal!(".")).clone(), false, false)?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_9.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\",\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("__desc__fields\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("#endif\n")).clone(), (literal!("#else /* Only use the file as a header */\n")).clone(), (literal!("extern struct record_description ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("__desc;\n")).clone(), (literal!("#endif")).clone()], lastHasNewLine: false }))?;
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

pub fn programExternalHeaderFromTypes(mut txt: Tpl::Text, mut a_tys: Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* Automatically generated header for bootstrapping MetaModelica */\n")).clone(), (literal!("#ifdef __cplusplus\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_47(out_txt.clone(), a_tys.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef __cplusplus\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_49(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>, mut in_a_c_name: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_c_name.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_elt, tail: rest }, a_c_name) => {
            let mut txt = (*txt).clone();
            txt = elementExternalHeader(txt.clone(), i_elt.clone(), (a_c_name.clone()).clone())?;
            txt = lm_49(txt.clone(), rest.clone(), (a_c_name.clone()).clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_50(mut in_txt: Tpl::Text, mut in_a_cl: Arc<SCode::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cl.clone())) {
        (txt, i_c @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { name: Deref @ Absyn::Path::IDENT { name: i_name }, moved: true, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = elementExternalHeader(txt.clone(), i_c.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::Element::CLASS { name: i_c_name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: i_p_elementLst, .. }, .. }) => {
            let mut txt = (*txt).clone();
            txt = lm_49(txt.clone(), i_p_elementLst.clone(), (i_c_name.clone()).clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn classExternalHeader(mut txt: Tpl::Text, mut a_cl: Arc<SCode::Element>, mut a_pack: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_50(txt.clone(), a_cl.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathString(mut in_txt: Tpl::Text, mut in_a_path: Arc<Absyn::Path>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_path.clone())) {
        (txt, Deref @ Absyn::Path::IDENT { name: i_name }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::QUALIFIED { path: i_path, name: i_name_1 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = pathString(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, Deref @ Absyn::Path::FULLYQUALIFIED { path: i_path }) => {
            let mut txt = (*txt).clone();
            txt = pathString(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (txt, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn metaHelperBoxStart(mut in_txt: Tpl::Text, mut in_a_numVariables: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_numVariables.clone()) {
        (mut txt, mut i_numVariables @ 0) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 1) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 2) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 3) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 4) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 5) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 6) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 7) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 8) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables @ 9) => {
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        (mut txt, mut i_numVariables) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_numVariables.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_54(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: i_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_54(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_54(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_55(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: i_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
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

fn fun_56(mut in_txt: Tpl::Text, mut in_mArg: ArcStr, mut in_a_fieldsStr: Tpl::Text, mut in_a_nElts: Tpl::Text, mut in_a_omcname: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fieldsStr.clone(), in_a_nElts.clone(), in_a_omcname.clone())) {
        (txt, Deref @ "0", _, _, a_omcname) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD_METARECORD_DEFINITIONS const char* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__desc__fields[1] = {\"no fields\"};")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_fieldsStr, a_nElts, a_omcname) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD_METARECORD_DEFINITIONS const char* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__desc__fields[")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_nElts.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fieldsStr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("};")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_57(mut in_txt: Tpl::Text, mut in_a_p_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>, mut in_a_fields: Tpl::Text, mut in_a_omcname: Tpl::Text, mut in_a_ctor: Tpl::Text, mut in_a_fullname: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_p_elementLst.clone(), in_a_fields.clone(), in_a_omcname.clone(), in_a_ctor.clone(), in_a_fullname.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_omcname, a_ctor, a_fullname) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static const MMC_DEFSTRUCTLIT(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__struct,1,")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_ctor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") {&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("__desc}};\n")).clone(), (literal!("static void *")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = MMC_REFSTRUCTLIT(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__struct);")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, i_p_elementLst, a_fields, a_omcname, a_ctor, a_fullname) => {
            let mut ret_1: i32 = 0;
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fields.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(") (mmc_mk_box")).clone() }))?;
            ret_0 = (i_p_elementLst.clone().len() as i32);
            ret_1 = intAdd(1, ret_0.clone());
            txt = metaHelperBoxStart(txt.clone(), ret_1.clone())?;
            txt = Tpl::writeText(txt.clone(), a_ctor.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",&")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__desc,")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fields.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("))")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn elementExternalHeader(mut in_txt: Tpl::Text, mut in_a_elt: Arc<SCode::Element>, mut in_a_pack: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elt.clone(), in_a_pack.clone())) {
        (txt, Deref @ SCode::Element::CLASS { name: i_c_name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: i_p_elementLst, .. }, restriction: SCode::Restriction::R_METARECORD { index: i_r_index, name: i_r_name, moved: true, .. }, .. }, a_pack) => {
            let mut str_11: ArcStr = arcstr::literal!("");
            let mut l_fieldsDescription: Tpl::Text;
            let mut ret_9: i32 = 0;
            let mut l_ctor: Tpl::Text;
            let mut ret_7: ArcStr = arcstr::literal!("");
            let mut l_fullname: Tpl::Text;
            let mut ret_5: i32 = 0;
            let mut l_nElts: Tpl::Text;
            let mut ret_3: ArcStr = arcstr::literal!("");
            let mut l_omcname: Tpl::Text;
            let mut l_fieldsStr: Tpl::Text;
            let mut l_fields: Tpl::Text;
            let mut txt = (*txt).clone();
            l_fields = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_fields = lm_54(l_fields.clone(), i_p_elementLst.clone())?;
            l_fields = Tpl::popIter(l_fields.clone())?;
            l_fieldsStr = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_fieldsStr = lm_55(l_fieldsStr.clone(), i_p_elementLst.clone())?;
            l_fieldsStr = Tpl::popIter(l_fieldsStr.clone())?;
            l_omcname = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_pack.clone()).clone())?;
            l_omcname = Tpl::writeTok(l_omcname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            l_omcname = pathString(l_omcname.clone(), i_r_name.clone())?;
            l_omcname = Tpl::writeTok(l_omcname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_3 = (System::stringReplace((i_c_name.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            l_omcname = Tpl::writeStr(l_omcname.clone(), (ret_3.clone()).clone())?;
            ret_5 = (i_p_elementLst.clone().len() as i32);
            l_nElts = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_5.clone())).clone())?;
            l_fullname = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_pack.clone()).clone())?;
            l_fullname = Tpl::writeTok(l_fullname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            ret_7 = (System::stringReplace((i_c_name.clone()).clone(), (literal!("_")).clone(), (literal!("_5f")).clone())?).clone();
            l_fullname = Tpl::writeStr(l_fullname.clone(), (ret_7.clone()).clone())?;
            ret_9 = intAdd(3, i_r_index.clone());
            l_ctor = Tpl::writeStr(Tpl::emptyTxt.clone(), (intString(ret_9.clone())).clone())?;
            str_11 = (Tpl::textString(l_nElts.clone())?).clone();
            l_fieldsDescription = fun_56(Tpl::emptyTxt.clone(), (str_11.clone()).clone(), l_fieldsStr.clone(), l_nElts.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef ADD_METARECORD_DEFINITIONS\n")).clone(), (literal!("#ifndef ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("__desc_added\n")).clone(), (literal!("#define ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("__desc_added\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_fieldsDescription.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("ADD_METARECORD_DEFINITIONS struct record_description ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("__desc = {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\",\n")).clone(), (literal!("\"")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (a_pack.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = pathString(txt.clone(), i_r_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_c_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("\",\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("__desc__fields\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("#endif\n")).clone(), (literal!("#else /* Only use the file as a header */\n")).clone(), (literal!("extern struct record_description ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("__desc;\n")).clone(), (literal!("#endif\n")).clone(), (literal!("#define ")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_3dBOX")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_nElts.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ctor.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = fun_57(txt.clone(), i_p_elementLst.clone(), l_fields.clone(), l_omcname.clone(), l_ctor.clone(), l_fullname.clone())?;
            txt.clone()
        },
        (txt, i_elt @ Deref @ SCode::Element::CLASS { name: _, .. }, a_pack) => {
            let mut txt = (*txt).clone();
            txt = classExternalHeader(txt.clone(), i_elt.clone(), (a_pack.clone()).clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn lm_59(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>, mut in_a_buf2: Tpl::Text, mut in_a_buf1: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    (out_txt, out_a_buf2, out_a_buf1) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_buf2.clone(), in_a_buf1.clone())) {
        (txt, Deref @ metamodelica::List::Nil, a_buf2, a_buf1) => {
            (txt.clone(), a_buf2.clone(), a_buf1.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_cl, tail: rest }, a_buf2, a_buf1) => {
            let mut txt = (*txt).clone();
            let mut a_buf2 = (*a_buf2).clone();
            let mut a_buf1 = (*a_buf1).clone();
            (txt, a_buf1, a_buf2) = classExternalHeaderJulia(txt.clone(), a_buf1.clone(), a_buf2.clone(), i_cl.clone(), (literal!("")).clone())?;
            (txt, a_buf2, a_buf1) = lm_59(txt.clone(), rest.clone(), a_buf2.clone(), a_buf1.clone())?;
            (txt.clone(), a_buf2.clone(), a_buf1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_buf2, out_a_buf1))
}

pub fn programExternalHeaderJulia(mut txt: Tpl::Text, mut a_program: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_res: Tpl::Text;
    let mut l_buf2: Tpl::Text;
    let mut l_buf1: Tpl::Text;
    l_buf1 = Tpl::emptyTxt.clone();
    l_buf2 = Tpl::emptyTxt.clone();
    (l_res, l_buf2, l_buf1) = lm_59(Tpl::emptyTxt.clone(), a_program.clone(), l_buf2.clone(), l_buf1.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("/* Automatically generated header for external MetaModelica functions */\n")).clone(), (literal!("#include <julia.h>\n")).clone(), (literal!("#include <assert.h>\n")).clone(), (literal!("#ifdef __cplusplus\n")).clone(), (literal!("extern \"C\" {\n")).clone(), (literal!("#endif\n")).clone(), (literal!("#ifdef ADD_METARECORD_DEFINITIONS\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_buf1.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("void OpenModelica_initAbsynReferences()\n")).clone(), (literal!("{\n")).clone(), (literal!("  /* Note: These values may be garbage collected away? Call this before each file is parsed? */\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_buf2.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("#else\n")).clone(), (literal!("void OpenModelica_initAbsynReferences();\n")).clone(), (literal!("#endif\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_res.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("#ifdef __cplusplus\n")).clone(), (literal!("}\n")).clone(), (literal!("#endif\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
    Ok(out_txt)
}

fn fun_61(mut in_txt: Tpl::Text, mut in_a_cl: Arc<SCode::Element>, mut in_a_buf1: Tpl::Text, mut in_a_buf2: Tpl::Text, mut in_a_pack: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    (out_txt, out_a_buf1, out_a_buf2) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cl.clone(), in_a_buf1.clone(), in_a_buf2.clone(), in_a_pack.clone())) {
        (txt, i_c @ Deref @ SCode::Element::CLASS { name: i_c_name, .. }, a_buf1, a_buf2, a_pack) => {
            let mut txt = (*txt).clone();
            let mut a_buf1 = (*a_buf1).clone();
            let mut a_buf2 = (*a_buf2).clone();
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jl_eval_string(\"using ")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\");\n")).clone(), (literal!("jl_module_t* ")).clone()], lastHasNewLine: false }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = (jl_module_t *) jl_eval_string(\"")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\");\n")).clone(), (literal!("if (!")).clone()], lastHasNewLine: false }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(")\n")).clone(), (literal!("{\n")).clone()], lastHasNewLine: true }))?;
            a_buf2 = Tpl::pushBlock(a_buf2.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("fprintf(stderr, \"module ")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" not loaded, load it via using.\");\n")).clone(), (literal!("fflush(NULL);\n")).clone()], lastHasNewLine: true }))?;
            a_buf2 = Tpl::popBlock(a_buf2.clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("}\n")).clone(), (literal!("assert(jl_is_module(")).clone()], lastHasNewLine: false }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("));")).clone() }))?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt, a_buf1, a_buf2) = classExternalHeaderJuliaWork(txt.clone(), a_buf1.clone(), a_buf2.clone(), i_c.clone(), (a_pack.clone()).clone())?;
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        (txt, _, a_buf1, a_buf2, _) => {
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_buf1, out_a_buf2))
}

pub fn classExternalHeaderJulia(mut txt: Tpl::Text, mut a_buf1: Tpl::Text, mut a_buf2: Tpl::Text, mut a_cl: Arc<SCode::Element>, mut a_pack: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    (out_txt, out_a_buf1, out_a_buf2) = fun_61(txt.clone(), a_cl.clone(), a_buf1.clone(), a_buf2.clone(), (a_pack.clone()).clone())?;
    Ok((out_txt, out_a_buf1, out_a_buf2))
}

fn lm_63(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>, mut in_a_c_name: ArcStr, mut in_a_buf2: Tpl::Text, mut in_a_buf1: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    (out_txt, out_a_buf2, out_a_buf1) = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_c_name.clone(), in_a_buf2.clone(), in_a_buf1.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, a_buf2, a_buf1) => {
            (txt.clone(), a_buf2.clone(), a_buf1.clone())
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_elt, tail: rest }, a_c_name, a_buf2, a_buf1) => {
            let mut txt = (*txt).clone();
            let mut a_buf2 = (*a_buf2).clone();
            let mut a_buf1 = (*a_buf1).clone();
            (txt, a_buf1, a_buf2) = elementExternalHeaderJulia(txt.clone(), a_buf1.clone(), a_buf2.clone(), i_elt.clone(), (a_c_name.clone()).clone())?;
            (txt, a_buf2, a_buf1) = lm_63(txt.clone(), rest.clone(), (a_c_name.clone()).clone(), a_buf2.clone(), a_buf1.clone())?;
            (txt.clone(), a_buf2.clone(), a_buf1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((out_txt, out_a_buf2, out_a_buf1))
}

fn fun_64(mut in_txt: Tpl::Text, mut in_a_cl: Arc<SCode::Element>, mut in_a_buf1: Tpl::Text, mut in_a_buf2: Tpl::Text) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    (out_txt, out_a_buf1, out_a_buf2) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cl.clone(), in_a_buf1.clone(), in_a_buf2.clone())) {
        (txt, Deref @ SCode::Element::CLASS { name: i_c_name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: i_p_elementLst, .. }, .. }, a_buf1, a_buf2) => {
            let mut txt = (*txt).clone();
            let mut a_buf1 = (*a_buf1).clone();
            let mut a_buf2 = (*a_buf2).clone();
            (txt, a_buf2, a_buf1) = lm_63(txt.clone(), i_p_elementLst.clone(), (i_c_name.clone()).clone(), a_buf2.clone(), a_buf1.clone())?;
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        (txt, _, a_buf1, a_buf2) => {
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_buf1, out_a_buf2))
}

pub fn classExternalHeaderJuliaWork(mut txt: Tpl::Text, mut a_buf1: Tpl::Text, mut a_buf2: Tpl::Text, mut a_cl: Arc<SCode::Element>, mut a_pack: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    (out_txt, out_a_buf1, out_a_buf2) = fun_64(txt.clone(), a_cl.clone(), a_buf1.clone(), a_buf2.clone())?;
    Ok((out_txt, out_a_buf1, out_a_buf2))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_66(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: i_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_66(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_66(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_67(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: i_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = lm_67(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_67(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_68(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::COMPONENT { name: i_name, .. }, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jl_value_t *")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_68(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_68(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_69(mut in_txt: Tpl::Text, mut in_mArg: i32, mut in_a_fields1: Tpl::Text, mut in_a_fields2: Tpl::Text, mut in_a_p_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>, mut in_a_fieldsWithType: Tpl::Text, mut in_a_omcname: Tpl::Text, mut in_a_fullname: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_fields1.clone(), in_a_fields2.clone(), in_a_p_elementLst.clone(), in_a_fieldsWithType.clone(), in_a_omcname.clone(), in_a_fullname.clone())) {
        (txt, 0, _, _, _, _, a_omcname, a_fullname) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("#define ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" jl_call0(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        (txt, 1, _, a_fields2, a_p_elementLst, a_fieldsWithType, a_omcname, a_fullname) => {
            let mut ret_0: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static inline jl_value_t* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fieldsWithType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return jl_call")).clone() }))?;
            ret_0 = (a_p_elementLst.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_0.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeText(txt.clone(), a_fields2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, 2, _, a_fields2, a_p_elementLst, a_fieldsWithType, a_omcname, a_fullname) => {
            let mut ret_1: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static inline jl_value_t* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fieldsWithType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return jl_call")).clone() }))?;
            ret_1 = (a_p_elementLst.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_1.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeText(txt.clone(), a_fields2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, 3, _, a_fields2, a_p_elementLst, a_fieldsWithType, a_omcname, a_fullname) => {
            let mut ret_2: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static inline jl_value_t* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fieldsWithType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return jl_call")).clone() }))?;
            ret_2 = (a_p_elementLst.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_2.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeText(txt.clone(), a_fields2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_fields1, _, a_p_elementLst, a_fieldsWithType, a_omcname, a_fullname) => {
            let mut ret_4: i32 = 0;
            let mut ret_3: i32 = 0;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("static inline jl_value_t* ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fullname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fieldsWithType.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jl_value_t *values[")).clone() }))?;
            ret_3 = (a_p_elementLst.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_3.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("] = {")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_fields1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("};\n")).clone(), (literal!("return jl_call(")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), a_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", values, ")).clone() }))?;
            ret_4 = (a_p_elementLst.clone().len() as i32);
            txt = Tpl::writeStr(txt.clone(), (intString(ret_4.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(");\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_70(mut in_txt: Tpl::Text, mut in_a_elt: Arc<SCode::Element>, mut in_a_buf1: Tpl::Text, mut in_a_buf2: Tpl::Text, mut in_a_pack: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    (out_txt, out_a_buf1, out_a_buf2) = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elt.clone(), in_a_buf1.clone(), in_a_buf2.clone(), in_a_pack.clone())) {
        (txt, Deref @ SCode::Element::CLASS { name: i_c_name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: _, .. }, restriction: SCode::Restriction::R_UNIONTYPE { .. }, .. }, a_buf1, a_buf2, a_pack) => {
            let mut l_omcname: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_buf1 = (*a_buf1).clone();
            let mut a_buf2 = (*a_buf2).clone();
            l_omcname = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_pack.clone()).clone())?;
            l_omcname = Tpl::writeTok(l_omcname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            l_omcname = Tpl::writeStr(l_omcname.clone(), (i_c_name.clone()).clone())?;
            a_buf1 = Tpl::writeTok(a_buf1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jl_value_t *")).clone() }))?;
            a_buf1 = Tpl::writeText(a_buf1.clone(), l_omcname.clone())?;
            a_buf1 = Tpl::writeTok(a_buf1.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" = NULL;\n")).clone() }))?;
            a_buf1 = Tpl::writeTok(a_buf1.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert((")).clone() }))?;
            a_buf2 = Tpl::writeText(a_buf2.clone(), l_omcname.clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = jl_get_global(")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (a_pack.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", jl_symbol(\"")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"))));")).clone() }))?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("extern jl_value_t *")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        (txt, Deref @ SCode::Element::CLASS { name: i_c_name, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: i_p_elementLst, .. }, restriction: SCode::Restriction::R_METARECORD { name: i_r_name, moved: true, .. }, .. }, a_buf1, a_buf2, a_pack) => {
            let mut ret_8: i32 = 0;
            let mut ret_7: ArcStr = arcstr::literal!("");
            let mut l_fullname: Tpl::Text;
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut l_funcName: Tpl::Text;
            let mut l_fieldsWithType: Tpl::Text;
            let mut l_fields2: Tpl::Text;
            let mut l_fields1: Tpl::Text;
            let mut l_omcname: Tpl::Text;
            let mut txt = (*txt).clone();
            let mut a_buf1 = (*a_buf1).clone();
            let mut a_buf2 = (*a_buf2).clone();
            l_fields1 = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_fields1 = lm_66(l_fields1.clone(), i_p_elementLst.clone())?;
            l_fields1 = Tpl::popIter(l_fields1.clone())?;
            l_fields2 = lm_67(Tpl::emptyTxt.clone(), i_p_elementLst.clone())?;
            l_fieldsWithType = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_fieldsWithType = lm_68(l_fieldsWithType.clone(), i_p_elementLst.clone())?;
            l_fieldsWithType = Tpl::popIter(l_fieldsWithType.clone())?;
            l_funcName = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_pack.clone()).clone())?;
            l_funcName = Tpl::writeTok(l_funcName.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }))?;
            l_funcName = pathString(l_funcName.clone(), i_r_name.clone())?;
            l_omcname = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_pack.clone()).clone())?;
            l_omcname = Tpl::writeTok(l_omcname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            l_omcname = pathString(l_omcname.clone(), i_r_name.clone())?;
            l_omcname = Tpl::writeTok(l_omcname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            ret_5 = (System::stringReplace((i_c_name.clone()).clone(), (literal!("_")).clone(), (literal!("__")).clone())?).clone();
            l_omcname = Tpl::writeStr(l_omcname.clone(), (ret_5.clone()).clone())?;
            l_fullname = Tpl::writeStr(Tpl::emptyTxt.clone(), (a_pack.clone()).clone())?;
            l_fullname = Tpl::writeTok(l_fullname.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("__")).clone() }))?;
            ret_7 = (System::stringReplace((i_c_name.clone()).clone(), (literal!("_")).clone(), (literal!("_5f")).clone())?).clone();
            l_fullname = Tpl::writeStr(l_fullname.clone(), (ret_7.clone()).clone())?;
            a_buf1 = Tpl::writeTok(a_buf1.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("jl_function_t *")).clone() }))?;
            a_buf1 = Tpl::writeText(a_buf1.clone(), l_omcname.clone())?;
            a_buf1 = Tpl::writeTok(a_buf1.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" = NULL;\n")).clone(), (literal!("jl_value_t *")).clone()], lastHasNewLine: false }))?;
            a_buf1 = Tpl::writeText(a_buf1.clone(), l_omcname.clone())?;
            a_buf1 = Tpl::writeTok(a_buf1.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_type = NULL;\n")).clone() }))?;
            a_buf1 = Tpl::writeTok(a_buf1.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert((")).clone() }))?;
            a_buf2 = Tpl::writeText(a_buf2.clone(), l_omcname.clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = jl_get_function(")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (a_pack.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", \"")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\")));\n")).clone(), (literal!("assert((")).clone()], lastHasNewLine: false }))?;
            a_buf2 = Tpl::writeText(a_buf2.clone(), l_omcname.clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_type = jl_get_global(")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (a_pack.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", jl_symbol(\"")).clone() }))?;
            a_buf2 = Tpl::writeStr(a_buf2.clone(), (i_c_name.clone()).clone())?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\"))));")).clone() }))?;
            a_buf2 = Tpl::writeTok(a_buf2.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("extern jl_function_t *")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(";\n")).clone(), (literal!("extern jl_function_t *")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_omcname.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("_type;\n")).clone() }))?;
            ret_8 = (i_p_elementLst.clone().len() as i32);
            txt = fun_69(txt.clone(), ret_8.clone(), l_fields1.clone(), l_fields2.clone(), i_p_elementLst.clone(), l_fieldsWithType.clone(), l_omcname.clone(), l_fullname.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        (txt, i_elt @ Deref @ SCode::Element::CLASS { name: _, .. }, a_buf1, a_buf2, a_pack) => {
            let mut txt = (*txt).clone();
            let mut a_buf1 = (*a_buf1).clone();
            let mut a_buf2 = (*a_buf2).clone();
            (txt, a_buf1, a_buf2) = classExternalHeaderJuliaWork(txt.clone(), a_buf1.clone(), a_buf2.clone(), i_elt.clone(), (a_pack.clone()).clone())?;
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        (txt, _, a_buf1, a_buf2, _) => {
            (txt.clone(), a_buf1.clone(), a_buf2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((out_txt, out_a_buf1, out_a_buf2))
}

pub fn elementExternalHeaderJulia(mut txt: Tpl::Text, mut a_buf1: Tpl::Text, mut a_buf2: Tpl::Text, mut a_elt: Arc<SCode::Element>, mut a_pack: ArcStr) -> Result<(Tpl::Text, Tpl::Text, Tpl::Text)> {
    let mut out_txt: Tpl::Text;
    let mut out_a_buf1: Tpl::Text;
    let mut out_a_buf2: Tpl::Text;
    (out_txt, out_a_buf1, out_a_buf2) = fun_70(txt.clone(), a_elt.clone(), a_buf1.clone(), a_buf2.clone(), (a_pack.clone()).clone())?;
    Ok((out_txt, out_a_buf1, out_a_buf2))
}

