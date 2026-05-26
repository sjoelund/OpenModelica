// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::VisualXML;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;
use openmodelica_util::Util;

pub fn dumpVisXML(mut txt: Tpl::Text, mut a_vis: metamodelica::Array<VisualXML::Visualization>, mut a_fileName: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut txt_0: Tpl::Text;
    txt_0 = dumpVisXML1(Tpl::emptyTxt.clone(), a_vis.clone())?;
    Tpl::textFile(txt_0.clone(), (a_fileName.clone()).clone())?;
    out_txt = txt.clone();
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_9(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<VisualXML::Visualization>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_vis, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpVisualization(txt.clone(), i_vis.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_9(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpVisXML1(mut txt: Tpl::Text, mut a_visArr: metamodelica::Array<VisualXML::Visualization>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_1: Arc<metamodelica::List<VisualXML::Visualization>> = metamodelica::nil();
    let mut l_visDump: Tpl::Text;
    ret_1 = Arc::new(a_visArr.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
    l_visDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_visDump = lm_9(l_visDump.clone(), ret_1.clone())?;
    l_visDump = Tpl::popIter(l_visDump.clone())?;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n")).clone(), (literal!("<visualization>\n")).clone()], lastHasNewLine: true }))?;
    out_txt = Tpl::writeText(out_txt.clone(), l_visDump.clone())?;
    out_txt = Tpl::softNewLine(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</visualization>")).clone() }))?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_11(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_T0, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpVecExp(txt.clone(), i_T0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_11(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_12(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_T0, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpVecExp(txt.clone(), i_T0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_12(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_13(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_T0, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpVecExp(txt.clone(), i_T0.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_13(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpVisualization(mut in_txt: Tpl::Text, mut in_a_vis: VisualXML::Visualization) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_vis.clone())) {
        (txt, VisualXML::Visualization::SHAPE { specularCoeff: i_specularCoeff, extra: i_extra, height: i_height, width: i_width, length: i_length, ident: i_ident, color: i_color, widthDir: i_widthDir, lengthDir: i_lengthDir, r_shape: i_r__shape, r: i_r, T: i_T, shapeType: Deref @ DAE::Exp::SCONST { string: i_svalue } }) => {
            let mut ret_12: ArcStr = arcstr::literal!("");
            let mut ret_11: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut l_colorDump: Tpl::Text;
            let mut ret_9: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut l_wDirDump: Tpl::Text;
            let mut ret_7: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut l_lDirDump: Tpl::Text;
            let mut ret_5: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut l_r__shapeDump: Tpl::Text;
            let mut ret_3: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut l_rDump: Tpl::Text;
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut l_TDump: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(i_T.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_TDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_TDump = lm_11(l_TDump.clone(), ret_1.clone())?;
            l_TDump = Tpl::popIter(l_TDump.clone())?;
            ret_3 = Arc::new(i_r.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_rDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_3.clone())?;
            ret_5 = Arc::new(i_r__shape.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_r__shapeDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_5.clone())?;
            ret_7 = Arc::new(i_lengthDir.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_lDirDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_7.clone())?;
            ret_9 = Arc::new(i_widthDir.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_wDirDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_9.clone())?;
            ret_11 = Arc::new(i_color.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_colorDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_11.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <shape>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<ident>")).clone() }))?;
            ret_12 = (ComponentReferenceBasics::printComponentRefStr(i_ident.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_12.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</ident>\n")).clone(), (literal!("<type>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeStr(txt.clone(), (i_svalue.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</type>\n")).clone(), (literal!("<T>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_TDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</T>\n")).clone(), (literal!("<r>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_rDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</r>\n")).clone(), (literal!("<r_shape>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_r__shapeDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</r_shape>\n")).clone(), (literal!("<lengthDir>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_lDirDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</lengthDir>\n")).clone(), (literal!("<widthDir>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_wDirDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</widthDir>\n")).clone(), (literal!("<length>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_length.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</length>\n")).clone(), (literal!("<width>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_width.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</width>\n")).clone(), (literal!("<height>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_height.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</height>\n")).clone(), (literal!("<extra>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_extra.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</extra>\n")).clone(), (literal!("<color>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_colorDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</color>\n")).clone(), (literal!("<specCoeff>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_specularCoeff.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</specCoeff>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </shape>")).clone() }))?;
            txt.clone()
        },
        (txt, VisualXML::Visualization::VECTOR { twoHeadedArrow: i_twoHeadedArrow, headAtOrigin: i_headAtOrigin, quantity: i_quantity, specularCoeff: i_specularCoeff, ident: i_ident, color: i_color, coordinates: i_coordinates, r: i_r, T: i_T }) => {
            let mut ret_18: ArcStr = arcstr::literal!("");
            let mut ret_17: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ret_16: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut l_coordDump: Tpl::Text;
            let mut ret_14: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ret_13: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut l_colorDump: Tpl::Text;
            let mut l_rDump: Tpl::Text;
            let mut l_TDump: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_13 = Arc::new(i_T.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_TDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_TDump = lm_12(l_TDump.clone(), ret_13.clone())?;
            l_TDump = Tpl::popIter(l_TDump.clone())?;
            ret_14 = Arc::new(i_r.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_rDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_14.clone())?;
            ret_16 = Arc::new(i_coordinates.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_coordDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_16.clone())?;
            ret_17 = Arc::new(i_color.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_colorDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_17.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <vector>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<ident>")).clone() }))?;
            ret_18 = (ComponentReferenceBasics::printComponentRefStr(i_ident.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_18.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</ident>\n")).clone(), (literal!("<T>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_TDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</T>\n")).clone(), (literal!("<r>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_rDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</r>\n")).clone(), (literal!("<coordinates>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_coordDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</coordinates>\n")).clone(), (literal!("<color>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_colorDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</color>\n")).clone(), (literal!("<specCoeff>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_specularCoeff.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</specCoeff>\n")).clone(), (literal!("<quantity>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_quantity.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</quantity>\n")).clone(), (literal!("<headAtOrigin>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_headAtOrigin.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</headAtOrigin>\n")).clone(), (literal!("<twoHeadedArrow>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_twoHeadedArrow.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</twoHeadedArrow>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </vector>")).clone() }))?;
            txt.clone()
        },
        (txt, VisualXML::Visualization::SURFACE { transparency: i_transparency, specularCoeff: i_specularCoeff, multiColored: i_multiColored, wireframe: i_wireframe, nv: i_nv, nu: i_nu, ident: i_ident, color: i_color, r_0: i_r__0, T: i_T }) => {
            let mut ret_23: ArcStr = arcstr::literal!("");
            let mut ret_22: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut ret_21: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut l_r__0Dump: Tpl::Text;
            let mut ret_19: Arc<metamodelica::List<Arc<metamodelica::List<Arc<DAE::Exp>>>>> = metamodelica::nil();
            let mut l_colorDump: Tpl::Text;
            let mut l_TDump: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_19 = Arc::new(i_T.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_TDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_TDump = lm_13(l_TDump.clone(), ret_19.clone())?;
            l_TDump = Tpl::popIter(l_TDump.clone())?;
            ret_21 = Arc::new(i_r__0.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_r__0Dump = dumpVecExp(Tpl::emptyTxt.clone(), ret_21.clone())?;
            ret_22 = Arc::new(i_color.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_colorDump = dumpVecExp(Tpl::emptyTxt.clone(), ret_22.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("  <surface>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 6 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<ident>")).clone() }))?;
            ret_23 = (ComponentReferenceBasics::printComponentRefStr(i_ident.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_23.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</ident>\n")).clone(), (literal!("<T>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_TDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</T>\n")).clone(), (literal!("<r>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_r__0Dump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</r>\n")).clone(), (literal!("<nu>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_nu.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</nu>\n")).clone(), (literal!("<nv>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_nv.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</nv>\n")).clone(), (literal!("<wireframe>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_wireframe.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</wireframe>\n")).clone(), (literal!("<multiColored>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_multiColored.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</multiColored>\n")).clone(), (literal!("<color>\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeText(txt.clone(), l_colorDump.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</color>\n")).clone(), (literal!("<specCoeff>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_specularCoeff.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</specCoeff>\n")).clone(), (literal!("<transparency>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_transparency.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</transparency>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("  </surface>")).clone() }))?;
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
fn lm_15(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_vec, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_vec.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_15(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpVecExp(mut txt: Tpl::Text, mut a_vector: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_vecDump: Tpl::Text;
    l_vecDump = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    l_vecDump = lm_15(l_vecDump.clone(), a_vector.clone())?;
    l_vecDump = Tpl::popIter(l_vecDump.clone())?;
    out_txt = Tpl::writeText(txt.clone(), l_vecDump.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_17(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<DAE::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = dumpExp(txt.clone(), i_e.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_17(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpExp(mut in_txt: Tpl::Text, mut in_a_expIn: Arc<DAE::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_expIn.clone())) {
        (txt, Deref @ DAE::Exp::ENUM_LITERAL { index: i_index, .. }) => {
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<enum>")).clone() }))?;
            ret_0 = (intString(i_index.clone())).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</enum>")).clone() }))?;
            txt.clone()
        },
        (txt, i_expIn @ Deref @ DAE::Exp::BCONST { bool: _ }) => {
            let mut ret_1: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<bconst>")).clone() }))?;
            ret_1 = (ExpressionBasics::printExpStr(i_expIn.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_1.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</bconst>")).clone() }))?;
            txt.clone()
        },
        (txt, i_expIn @ Deref @ DAE::Exp::CREF { componentRef: _, .. }) => {
            let mut ret_2: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cref>")).clone() }))?;
            ret_2 = (ExpressionBasics::printExpStr(i_expIn.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_2.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</cref>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::BINARY { exp2: i_exp2, operator: i_operator, exp1: i_exp1 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<binary>\n")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<op>")).clone() }))?;
            txt = dumpOperator(txt.clone(), i_operator.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</op>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = dumpExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</binary>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::UNARY { exp: i_exp, operator: i_operator }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<unary>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<op>")).clone() }))?;
            txt = dumpOperator(txt.clone(), i_operator.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</op>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</unary>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LBINARY { exp2: i_exp2, operator: i_operator, exp1: i_exp1 }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<lbinary>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = dumpExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<op>")).clone() }))?;
            txt = dumpOperator(txt.clone(), i_operator.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</op>\n")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</lbinary>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::LUNARY { exp: i_exp, operator: i_operator }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<lunary>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<op>")).clone() }))?;
            txt = dumpOperator(txt.clone(), i_operator.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</op>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = dumpExp(txt.clone(), i_exp.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</lunary>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::RELATION { exp2: i_exp2, operator: i_operator, exp1: i_exp1, .. }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<relation>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp1>")).clone() }))?;
            txt = dumpExp(txt.clone(), i_exp1.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</exp1>\n")).clone(), (literal!("<op>")).clone()], lastHasNewLine: false }))?;
            txt = dumpOperator(txt.clone(), i_operator.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</op>\n")).clone(), (literal!("<exp2>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_exp2.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</exp2>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</relation>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::IFEXP { expElse: i_expElse, expThen: i_expThen, expCond: i_expCond }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<ifexp>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<cond>")).clone() }))?;
            txt = dumpExp(txt.clone(), i_expCond.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</cond>\n")).clone(), (literal!("<then>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_expThen.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</then>\n")).clone(), (literal!("<else>")).clone()], lastHasNewLine: false }))?;
            txt = dumpExp(txt.clone(), i_expElse.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</else>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</ifexp>")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ DAE::Exp::CALL { path: i_path, expLst: i_expLst, .. }) => {
            let mut ret_4: ArcStr = arcstr::literal!("");
            let mut l_elist: Tpl::Text;
            let mut txt = (*txt).clone();
            l_elist = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elist = lm_17(l_elist.clone(), i_expLst.clone())?;
            l_elist = Tpl::popIter(l_elist.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("<call>\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 4 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<path>")).clone() }))?;
            ret_4 = (AbsynUtil::pathString(i_path.clone(), (literal!(".")).clone(), true, false)?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_4.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("</path>\n")).clone(), (literal!("<expLst>")).clone()], lastHasNewLine: false }))?;
            txt = Tpl::writeText(txt.clone(), l_elist.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("</expLst>\n")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</call>")).clone() }))?;
            txt.clone()
        },
        (txt, i_expIn) => {
            let mut ret_5: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<exp>")).clone() }))?;
            ret_5 = (ExpressionBasics::printExpStr(i_expIn.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_5.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("</exp>")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpOperator(mut in_txt: Tpl::Text, mut in_a_op: DAE::Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_op.clone()) {
        (mut txt, DAE::Operator::ADD { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("add")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("sub")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mul")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("div")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pow")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("uminus")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::UMINUS_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("uminus_arr")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("add_arr")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("sub_arr")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mul_arr")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("siv_arr")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mul_array_scalar")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::ADD_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("add_array_scalar")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::SUB_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("sub_scalar_array")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mul_scalar_product")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("mul_matrix_product")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("div_array_scalar")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("siv_scalar_array")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARRAY_SCALAR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pow_array_scalar")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_SCALAR_ARRAY { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pow_scalar_array")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("pow_arr")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::POW_ARR2 { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("por_arr2")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::AND { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("and")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::OR { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("or")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NOT { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("not")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESS { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("less")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::LESSEQ { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("lesseq")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATER { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("greater")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::GREATEREQ { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("greatereq")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::EQUAL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("equal")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::NEQUAL { ty: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("nequal")).clone() }))?;
            txt.clone()
        },
        (mut txt, DAE::Operator::USERDEFINED { fqName: _ }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("userdefined")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("-unknown operator-")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

