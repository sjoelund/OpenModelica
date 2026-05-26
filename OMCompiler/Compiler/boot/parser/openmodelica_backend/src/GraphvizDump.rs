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
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use crate::CodegenUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_susan::Tpl;

pub fn dumpBackendDAE(mut in_txt: Tpl::Text, mut in_a_backendDAE: Arc<BackendDAE::BackendDAE>, mut in_a_suffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_backendDAE.clone(), in_a_suffix.clone())) {
        (txt, i_dae @ Deref @ DAE { shared: BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: i_info_fileNamePrefix, .. }, .. }, .. }, a_suffix) => {
            let mut txt_3: Tpl::Text;
            let mut txt_2: Tpl::Text;
            let mut l_0___1: Tpl::Text;
            let mut l_0__: Tpl::Text;
            l_0__ = dumpAdjacencyMatrix(Tpl::emptyTxt.clone(), i_dae.clone(), (a_suffix.clone()).clone())?;
            txt_2 = dumpMatching(Tpl::emptyTxt.clone(), i_dae.clone(), (a_suffix.clone()).clone())?;
            txt_3 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_info_fileNamePrefix.clone()).clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt_3 = Tpl::writeStr(txt_3.clone(), (a_suffix.clone()).clone())?;
            txt_3 = Tpl::writeTok(txt_3.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_matching.dot")).clone() }))?;
            Tpl::textFile(txt_2.clone(), (Tpl::textString(txt_3.clone())?).clone())?;
            l_0___1 = Tpl::emptyTxt.clone();
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAdjacencyMatrix(mut in_txt: Tpl::Text, mut in_a_backendDAE: Arc<BackendDAE::BackendDAE>, mut in_a_suffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_backendDAE.clone(), in_a_suffix.clone())) {
        (txt, i_dae @ Deref @ DAE { shared: BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: i_info_fileNamePrefix, .. }, .. }, .. }, a_suffix) => {
            let mut txt_2: Tpl::Text;
            let mut txt_1: Tpl::Text;
            let mut l_0__: Tpl::Text;
            txt_1 = dumpDependence(Tpl::emptyTxt.clone(), i_dae.clone(), (a_suffix.clone()).clone())?;
            txt_2 = Tpl::writeStr(Tpl::emptyTxt.clone(), (i_info_fileNamePrefix.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt_2 = Tpl::writeStr(txt_2.clone(), (a_suffix.clone()).clone())?;
            txt_2 = Tpl::writeTok(txt_2.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_dependence.dot")).clone() }))?;
            Tpl::textFile(txt_1.clone(), (Tpl::textString(txt_2.clone())?).clone())?;
            l_0__ = Tpl::emptyTxt.clone();
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_11(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_var_varName: Arc<DAE::ComponentRef>, mut in_a_varID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_var_varName.clone(), in_a_varID.clone(), in_a_clusterID.clone())) {
        (txt, false, a_var_varName, a_varID, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), a_var_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", shape=\"box\"]")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_var_varName, a_varID, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"der(")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), a_var_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")\", shape=\"box\"]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_12(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<BackendDAE::Var>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var @ BackendDAE::Var { varName: i_var_varName, .. }, tail: rest }, a_clusterID) => {
            let mut x_varID: i32 = 0;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            x_varID = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = BackendVariable::isStateVar(i_var.clone());
            txt = fun_11(txt.clone(), ret_0.clone(), i_var_varName.clone(), x_varID.clone(), a_clusterID.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_12(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = lm_12(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_13(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_clusterID) => {
            let mut x_eqID: i32 = 0;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            x_eqID = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"")).clone() }))?;
            ret_0 = (BackendDump::equationString(i_eq.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", shape=\"box\"]")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_13(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_14(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { m: i_eqSystem_m, orderedEqs: i_eqSystem_orderedEqs, orderedVars: i_eqSystem_orderedVars, .. }, tail: rest }) => {
            let mut x_clusterID: i32 = 0;
            let mut ret_3: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut l_eqDeclaration: Tpl::Text;
            let mut ret_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut l_varDeclaration: Tpl::Text;
            let mut txt = (*txt).clone();
            x_clusterID = Tpl::getIteri_i0(txt.clone())?;
            ret_1 = BackendVariable::varList(i_eqSystem_orderedVars.clone())?;
            l_varDeclaration = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_varDeclaration = lm_12(l_varDeclaration.clone(), ret_1.clone(), x_clusterID.clone())?;
            l_varDeclaration = Tpl::popIter(l_varDeclaration.clone())?;
            ret_3 = BackendEquation::equationList(i_eqSystem_orderedEqs.clone());
            l_eqDeclaration = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_eqDeclaration = lm_13(l_eqDeclaration.clone(), ret_3.clone(), x_clusterID.clone())?;
            l_eqDeclaration = Tpl::popIter(l_eqDeclaration.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("subgraph cluster_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label = \"system #")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("color=white\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_varDeclaration.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_eqDeclaration.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = dumpDependence2(txt.clone(), x_clusterID.clone(), i_eqSystem_m.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_14(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_14(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpDependence(mut in_txt: Tpl::Text, mut in_a_backendDAE: Arc<BackendDAE::BackendDAE>, mut in_a_suffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_backendDAE.clone(), in_a_suffix.clone())) {
        (txt, Deref @ DAE { shared: BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: i_info_fileNamePrefix, .. }, .. }, eqs: i_eqs, .. }, a_suffix) => {
            let mut l_systems: Tpl::Text;
            let mut txt = (*txt).clone();
            l_systems = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_systems = lm_14(l_systems.clone(), i_eqs.clone())?;
            l_systems = Tpl::popIter(l_systems.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("digraph G {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_info_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_suffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" - dependence]\";\n")).clone(), (literal!("rankdir=LR;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_systems.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_16(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_eqID: i32, mut in_a_varID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_eqID.clone(), in_a_varID.clone(), in_a_clusterID.clone()) {
        (mut txt, false, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_eqID, mut a_varID, mut a_clusterID) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -> eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [style=\"dashed\", arrowhead=\"none\"];")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_17(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_eqID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_eqID.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_varID, tail: rest }, a_eqID, a_clusterID) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intGt(i_varID.clone(), 0);
            txt = fun_16(txt.clone(), ret_0.clone(), a_eqID.clone(), i_varID.clone(), a_clusterID.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_17(txt.clone(), rest.clone(), a_eqID.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_18(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_varList, tail: rest }, a_clusterID) => {
            let mut x_eqID: i32 = 0;
            let mut l_foo: Tpl::Text;
            let mut txt = (*txt).clone();
            x_eqID = Tpl::getIteri_i0(txt.clone())?;
            l_foo = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_foo = lm_17(l_foo.clone(), i_varList.clone(), x_eqID.clone(), a_clusterID.clone())?;
            l_foo = Tpl::popIter(l_foo.clone())?;
            txt = Tpl::writeText(txt.clone(), l_foo.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_18(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_19(mut in_txt: Tpl::Text, mut in_a_m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_m.clone(), in_a_clusterID.clone()) {
        (mut txt, Some(mut i_incMatrix), mut a_clusterID) => {
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut l_incNodes: Tpl::Text;
            ret_1 = Arc::new(i_incMatrix.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_incNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_incNodes = lm_18(l_incNodes.clone(), ret_1.clone(), a_clusterID.clone())?;
            l_incNodes = Tpl::popIter(l_incNodes.clone())?;
            txt = Tpl::writeText(txt.clone(), l_incNodes.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("// no adjacency matrix")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpDependence2(mut txt: Tpl::Text, mut a_clusterID: i32, mut a_m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_19(txt.clone(), a_m.clone(), a_clusterID.clone())?;
    Ok(out_txt)
}

fn fun_21(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_var_varName: Arc<DAE::ComponentRef>, mut in_a_varID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_var_varName.clone(), in_a_varID.clone(), in_a_clusterID.clone())) {
        (txt, false, a_var_varName, a_varID, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), a_var_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", shape=\"box\"]")).clone() }))?;
            txt.clone()
        },
        (txt, _, a_var_varName, a_varID, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"der(")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), a_var_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")\", shape=\"box\"]")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_22(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<BackendDAE::Var>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_var @ BackendDAE::Var { varName: i_var_varName, .. }, tail: rest }, a_clusterID) => {
            let mut x_varID: i32 = 0;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            x_varID = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = BackendVariable::isStateVar(i_var.clone());
            txt = fun_21(txt.clone(), ret_0.clone(), i_var_varName.clone(), x_varID.clone(), a_clusterID.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_22(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = lm_22(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_23(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_clusterID) => {
            let mut x_eqID: i32 = 0;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            x_eqID = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"")).clone() }))?;
            ret_0 = (BackendDump::equationString(i_eq.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", shape=\"box\"]")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_23(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_24(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { m: i_eqSystem_m, matching: i_eqSystem_matching, orderedEqs: i_eqSystem_orderedEqs, orderedVars: i_eqSystem_orderedVars, .. }, tail: rest }) => {
            let mut x_clusterID: i32 = 0;
            let mut ret_3: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut l_eqDeclaration: Tpl::Text;
            let mut ret_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut l_varDeclaration: Tpl::Text;
            let mut txt = (*txt).clone();
            x_clusterID = Tpl::getIteri_i0(txt.clone())?;
            ret_1 = BackendVariable::varList(i_eqSystem_orderedVars.clone())?;
            l_varDeclaration = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_varDeclaration = lm_22(l_varDeclaration.clone(), ret_1.clone(), x_clusterID.clone())?;
            l_varDeclaration = Tpl::popIter(l_varDeclaration.clone())?;
            ret_3 = BackendEquation::equationList(i_eqSystem_orderedEqs.clone());
            l_eqDeclaration = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_eqDeclaration = lm_23(l_eqDeclaration.clone(), ret_3.clone(), x_clusterID.clone())?;
            l_eqDeclaration = Tpl::popIter(l_eqDeclaration.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("subgraph cluster_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label = \"system #")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("color=white\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_varDeclaration.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeText(txt.clone(), l_eqDeclaration.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = connections(txt.clone(), x_clusterID.clone(), i_eqSystem_matching.clone(), i_eqSystem_m.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_24(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_24(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpMatching(mut in_txt: Tpl::Text, mut in_a_backendDAE: Arc<BackendDAE::BackendDAE>, mut in_a_suffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_backendDAE.clone(), in_a_suffix.clone())) {
        (txt, Deref @ DAE { shared: BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: i_info_fileNamePrefix, .. }, .. }, eqs: i_eqs, .. }, a_suffix) => {
            let mut l_systems: Tpl::Text;
            let mut txt = (*txt).clone();
            l_systems = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_systems = lm_24(l_systems.clone(), i_eqs.clone())?;
            l_systems = Tpl::popIter(l_systems.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("digraph G {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_info_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_suffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" - matching]\";\n")).clone(), (literal!("rankdir=LR;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_systems.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_26(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_eqID: i32, mut in_a_varID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_eqID.clone(), in_a_varID.clone(), in_a_clusterID.clone()) {
        (mut txt, false, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_eqID, mut a_varID, mut a_clusterID) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -> eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [style=\"dashed\", arrowhead=\"none\"];")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_27(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_eqID: i32, mut in_a_clusterID: i32, mut in_a_varID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_eqID.clone(), in_a_clusterID.clone(), in_a_varID.clone()) {
        (mut txt, false, mut a_eqID, mut a_clusterID, mut a_varID) => {
            let mut ret_0: bool = false;
            ret_0 = intGt(a_varID.clone(), 0);
            txt = fun_26(txt.clone(), ret_0.clone(), a_eqID.clone(), a_varID.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_eqID, mut a_clusterID, mut a_varID) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -> eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [style=\"bold\", arrowhead=\"none\"];")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_28(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_clusterID: i32, mut in_a_eqID: i32, mut in_a_ass2: metamodelica::Array<i32>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone(), in_a_eqID.clone(), in_a_ass2.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_varID, tail: rest }, a_clusterID, a_eqID, a_ass2) => {
            let mut ret_2: bool = false;
            let mut ret_1: i32 = 0;
            let mut ret_0: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut txt = (*txt).clone();
            ret_0 = Arc::new(a_ass2.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            ret_1 = (ret_0.clone()).get(a_eqID.clone())?;
            ret_2 = intEq(ret_1.clone(), i_varID.clone());
            txt = fun_27(txt.clone(), ret_2.clone(), a_eqID.clone(), a_clusterID.clone(), i_varID.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_28(txt.clone(), rest.clone(), a_clusterID.clone(), a_eqID.clone(), a_ass2.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_29(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut in_a_clusterID: i32, mut in_a_ass2: metamodelica::Array<i32>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone(), in_a_ass2.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_varList, tail: rest }, a_clusterID, a_ass2) => {
            let mut x_eqID: i32 = 0;
            let mut l_foo: Tpl::Text;
            let mut txt = (*txt).clone();
            x_eqID = Tpl::getIteri_i0(txt.clone())?;
            l_foo = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_foo = lm_28(l_foo.clone(), i_varList.clone(), a_clusterID.clone(), x_eqID.clone(), a_ass2.clone())?;
            l_foo = Tpl::popIter(l_foo.clone())?;
            txt = Tpl::writeText(txt.clone(), l_foo.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_29(txt.clone(), rest.clone(), a_clusterID.clone(), a_ass2.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_30(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_eqID: i32, mut in_a_varID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_eqID.clone(), in_a_varID.clone(), in_a_clusterID.clone()) {
        (mut txt, false, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_eqID, mut a_varID, mut a_clusterID) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -> eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [style=\"dashed\", arrowhead=\"none\"];")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_31(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_eqID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_eqID.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_varID, tail: rest }, a_eqID, a_clusterID) => {
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            ret_0 = intGt(i_varID.clone(), 0);
            txt = fun_30(txt.clone(), ret_0.clone(), a_eqID.clone(), i_varID.clone(), a_clusterID.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_31(txt.clone(), rest.clone(), a_eqID.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_32(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_varList, tail: rest }, a_clusterID) => {
            let mut x_eqID: i32 = 0;
            let mut l_foo: Tpl::Text;
            let mut txt = (*txt).clone();
            x_eqID = Tpl::getIteri_i0(txt.clone())?;
            l_foo = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_foo = lm_31(l_foo.clone(), i_varList.clone(), x_eqID.clone(), a_clusterID.clone())?;
            l_foo = Tpl::popIter(l_foo.clone())?;
            txt = Tpl::writeText(txt.clone(), l_foo.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_32(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_33(mut in_txt: Tpl::Text, mut in_a_matching: Arc<BackendDAE::Matching>, mut in_a_clusterID: i32, mut in_a_incMatrix: metamodelica::Array<Arc<metamodelica::List<i32>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matching.clone(), in_a_clusterID.clone(), in_a_incMatrix.clone())) {
        (txt, Deref @ BackendDAE::Matching::MATCHING { ass2: i_ass2, .. }, a_clusterID, a_incMatrix) => {
            let mut ret_1: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut l_incNodes: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(a_incMatrix.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_incNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_incNodes = lm_29(l_incNodes.clone(), ret_1.clone(), a_clusterID.clone(), i_ass2.clone())?;
            l_incNodes = Tpl::popIter(l_incNodes.clone())?;
            txt = Tpl::writeText(txt.clone(), l_incNodes.clone())?;
            txt.clone()
        },
        (txt, _, a_clusterID, a_incMatrix) => {
            let mut ret_2: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>> = metamodelica::nil();
            let mut l_incNodes: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_2 = Arc::new(a_incMatrix.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_incNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_incNodes = lm_32(l_incNodes.clone(), ret_2.clone(), a_clusterID.clone())?;
            l_incNodes = Tpl::popIter(l_incNodes.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("// no matching\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_incNodes.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_34(mut in_txt: Tpl::Text, mut in_mArg: bool, mut in_a_eqID: i32, mut in_a_varID: i32, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_eqID.clone(), in_a_varID.clone(), in_a_clusterID.clone()) {
        (mut txt, false, _, _, _) => {
            txt.clone()
        },
        (mut txt, _, mut a_eqID, mut a_varID, mut a_clusterID) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -> eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [style=\"bold\", arrowhead=\"none\"];")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_35(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_varID, tail: rest }, a_clusterID) => {
            let mut x_eqID: i32 = 0;
            let mut ret_0: bool = false;
            let mut txt = (*txt).clone();
            x_eqID = Tpl::getIteri_i0(txt.clone())?;
            ret_0 = intGt(i_varID.clone(), 0);
            txt = fun_34(txt.clone(), ret_0.clone(), x_eqID.clone(), i_varID.clone(), a_clusterID.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_35(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_36(mut in_txt: Tpl::Text, mut in_a_matching: Arc<BackendDAE::Matching>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matching.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ BackendDAE::Matching::MATCHING { ass2: i_ass2, .. }, a_clusterID) => {
            let mut ret_1: Arc<metamodelica::List<i32>> = metamodelica::nil();
            let mut l_matchedNodes: Tpl::Text;
            let mut txt = (*txt).clone();
            ret_1 = Arc::new(i_ass2.clone().borrow().iter().cloned().collect::<metamodelica::List<_>>());
            l_matchedNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_matchedNodes = lm_35(l_matchedNodes.clone(), ret_1.clone(), a_clusterID.clone())?;
            l_matchedNodes = Tpl::popIter(l_matchedNodes.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("// no adjacency matrix\n")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_matchedNodes.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("// no adjacency matrix\n")).clone(), (literal!("// no matching")).clone()], lastHasNewLine: false }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_37(mut in_txt: Tpl::Text, mut in_a_m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>, mut in_a_clusterID: i32, mut in_a_matching: Arc<BackendDAE::Matching>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_m.clone(), in_a_clusterID.clone(), in_a_matching.clone())) {
        (txt, Some(i_incMatrix), a_clusterID, a_matching) => {
            let mut txt = (*txt).clone();
            txt = fun_33(txt.clone(), a_matching.clone(), a_clusterID.clone(), i_incMatrix.clone())?;
            txt.clone()
        },
        (txt, _, a_clusterID, a_matching) => {
            let mut txt = (*txt).clone();
            txt = fun_36(txt.clone(), a_matching.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn connections(mut txt: Tpl::Text, mut a_clusterID: i32, mut a_matching: Arc<BackendDAE::Matching>, mut a_m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_37(txt.clone(), a_m.clone(), a_clusterID.clone(), a_matching.clone())?;
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_39(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<BackendDAE::Var>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: BackendDAE::Var { varName: i_var_varName, .. }, tail: rest }, a_clusterID) => {
            let mut x_varID: i32 = 0;
            let mut txt = (*txt).clone();
            x_varID = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_varID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"")).clone() }))?;
            txt = CodegenUtil::crefStr(txt.clone(), i_var_varName.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", shape=\"box\"]")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_39(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = lm_39(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_40(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_clusterID) => {
            let mut x_eqID: i32 = 0;
            let mut ret_0: ArcStr = arcstr::literal!("");
            let mut txt = (*txt).clone();
            x_eqID = Tpl::getIteri_i0(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("eq")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_eqID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [label=\"")).clone() }))?;
            ret_0 = (BackendDump::equationString(i_eq.clone())?).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\", shape=\"box\"]")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_40(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_41(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: Deref @ BackendDAE::EqSystem { matching: i_eqSystem_matching, orderedEqs: i_eqSystem_orderedEqs, orderedVars: i_eqSystem_orderedVars, .. }, tail: rest }) => {
            let mut x_clusterID: i32 = 0;
            let mut ret_3: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
            let mut l_eqDeclaration: Tpl::Text;
            let mut ret_1: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
            let mut l_varDeclaration: Tpl::Text;
            let mut txt = (*txt).clone();
            x_clusterID = Tpl::getIteri_i0(txt.clone())?;
            ret_1 = BackendVariable::varList(i_eqSystem_orderedVars.clone())?;
            l_varDeclaration = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_varDeclaration = lm_39(l_varDeclaration.clone(), ret_1.clone(), x_clusterID.clone())?;
            l_varDeclaration = Tpl::popIter(l_varDeclaration.clone())?;
            ret_3 = BackendEquation::equationList(i_eqSystem_orderedEqs.clone());
            l_eqDeclaration = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_eqDeclaration = lm_40(l_eqDeclaration.clone(), ret_3.clone(), x_clusterID.clone())?;
            l_eqDeclaration = Tpl::popIter(l_eqDeclaration.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("subgraph cluster_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label = \"system #")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(x_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\";\n")).clone(), (literal!("color=white\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_varDeclaration.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = dumpStrongComponent(txt.clone(), x_clusterID.clone(), i_eqSystem_matching.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_41(txt.clone(), rest.clone())?;
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = lm_41(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

pub fn dumpSorting(mut in_txt: Tpl::Text, mut in_a_backendDAE: Arc<BackendDAE::BackendDAE>, mut in_a_suffix: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_backendDAE.clone(), in_a_suffix.clone())) {
        (txt, Deref @ DAE { shared: BackendDAE::Shared { info: BackendDAE::ExtraInfo { fileNamePrefix: i_info_fileNamePrefix, .. }, .. }, eqs: i_eqs, .. }, a_suffix) => {
            let mut l_systems: Tpl::Text;
            let mut txt = (*txt).clone();
            l_systems = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_systems = lm_41(l_systems.clone(), i_eqs.clone())?;
            l_systems = Tpl::popIter(l_systems.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("digraph G {\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("label=\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_info_fileNamePrefix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" [")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_suffix.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!(" - sorting]\";\n")).clone(), (literal!("rankdir=LR;\n")).clone(), (literal!("\n")).clone()], lastHasNewLine: true }))?;
            txt = Tpl::writeText(txt.clone(), l_systems.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
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
fn lm_43(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_v, tail: rest }, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_v.clone())).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_43(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_44(mut in_txt: Tpl::Text, mut in_a_comp: Arc<BackendDAE::StrongComponent>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comp.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ BackendDAE::StrongComponent::SINGLEEQUATION { var: i_c_var, .. }, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("var")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(a_clusterID.clone())).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("_")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (intString(i_c_var.clone())).clone())?;
            txt.clone()
        },
        (txt, Deref @ BackendDAE::StrongComponent::EQUATIONSYSTEM { vars: i_c_vars, .. }, a_clusterID) => {
            let mut l_foo: Tpl::Text;
            let mut txt = (*txt).clone();
            l_foo = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" <-> ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_foo = lm_43(l_foo.clone(), i_c_vars.clone(), a_clusterID.clone())?;
            l_foo = Tpl::popIter(l_foo.clone())?;
            txt = Tpl::writeText(txt.clone(), l_foo.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("asd")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_45(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<BackendDAE::StrongComponent>>>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            txt.clone()
        },
        (txt, Deref @ metamodelica::List::Cons { head: i_comp, tail: rest }, a_clusterID) => {
            let mut txt = (*txt).clone();
            txt = fun_44(txt.clone(), i_comp.clone(), a_clusterID.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_45(txt.clone(), rest.clone(), a_clusterID.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn fun_46(mut in_txt: Tpl::Text, mut in_a_matching: Arc<BackendDAE::Matching>, mut in_a_clusterID: i32) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_matching.clone(), in_a_clusterID.clone())) {
        (txt, Deref @ BackendDAE::Matching::MATCHING { comps: i_comps, .. }, a_clusterID) => {
            let mut l_cmpNodes: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmpNodes = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" -> ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_cmpNodes = lm_45(l_cmpNodes.clone(), i_comps.clone(), a_clusterID.clone())?;
            l_cmpNodes = Tpl::popIter(l_cmpNodes.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmpNodes.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpStrongComponent(mut txt: Tpl::Text, mut a_clusterID: i32, mut a_matching: Arc<BackendDAE::Matching>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_46(txt.clone(), a_matching.clone(), a_clusterID.clone())?;
    Ok(out_txt)
}

