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
use crate::Dump;
use crate::SCode;
use crate::SCodeDump;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util::Util;

pub fn dumpAlgorithmSection(in_txt: Tpl::Text, in_a_algorithm: Arc<SCode::AlgorithmSection>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_algorithm.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ALGORITHM { statements: i_statements }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpStatements(txt.clone(), i_statements.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAlgorithmSections(in_txt: Tpl::Text, in_a_algorithms: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, in_a_label: ArcStr, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_algorithms.clone(), in_a_label.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => txt.clone(),
        (txt, i_algorithms, a_label, a_options) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_label.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_80(txt.clone(), i_algorithms.clone(), a_options.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAnnotation(in_txt: Tpl::Text, in_a_annotation: Arc<SCode::Annotation>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annotation.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ANNOTATION { modification: i_modification }, a_options) => {
            let mut l_modifStr: Tpl::Text;
            let mut txt = (*txt).clone();
            l_modifStr = dumpAnnotationModifier(Tpl::emptyTxt.clone(), i_modification.clone(), a_options.clone())?;
            txt = fun_131(txt.clone(), l_modifStr.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAnnotationElement(txt: Tpl::Text, a_annotation: Arc<SCode::Annotation>, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_annstr: Tpl::Text;
    l_annstr = dumpAnnotation(Tpl::emptyTxt.clone(), a_annotation.clone(), a_options.clone())?;
    out_txt = fun_133(txt.clone(), l_annstr.clone())?;
    Ok(out_txt)
}

pub fn dumpAnnotationModifier(in_txt: Tpl::Text, in_a_modifier: Arc<SCode::Mod>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modifier.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::MOD { subModLst: i_subModLst, binding: i_binding, .. }, a_options) => {
            let mut l_submod__str: Tpl::Text;
            let mut l_text: Tpl::Text;
            let mut l_binding__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_binding__str = dumpModifierBinding(Tpl::emptyTxt.clone(), i_binding.clone())?;
            l_text = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_text = lm_114(l_text.clone(), i_subModLst.clone(), a_options.clone())?;
            l_text = Tpl::popIter(l_text.clone())?;
            l_submod__str = fun_115(Tpl::emptyTxt.clone(), l_text.clone())?;
            txt = Tpl::writeText(txt.clone(), l_submod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_binding__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAnnotationOpt(in_txt: Tpl::Text, in_a_annotation: Option<Arc<SCode::Annotation>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_annotation.clone(), in_a_options.clone()) {
        (mut txt, Some(mut i_ann), mut a_options) => {
            txt = dumpAnnotation(txt.clone(), i_ann.clone(), a_options.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpAnnotationSubModifier(in_txt: Tpl::Text, in_a_submod: Arc<SCode::SubMod>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_submod.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::NAMEMOD { ident: i_ident, r#mod: i_nameMod @ i_mod @ Deref @ SCode::MOD { finalPrefix: _, .. } }, a_options) => {
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            ret_0 = Config::showAnnotations()?;
            txt = fun_122(txt.clone(), ret_0.clone(), i_mod.clone(), a_options.clone(), i_nameMod.clone(), (i_ident.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::NAMEMOD { r#mod: i_mod @ Deref @ SCode::REDECL { finalPrefix: _, .. }, .. }, a_options) => {
            let mut i_nameMod: Arc<SCode::Mod>;
            let mut i_ident: ArcStr;
            let mut ret_0: bool;
            let mut txt = (*txt).clone();
            txt = dumpRedeclModifier(txt.clone(), i_mod.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAssertionLevel(in_txt: Tpl::Text, in_a_exp: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_exp.clone())) {
        (txt, Deref @ Absyn::CREF { componentRef: Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: Deref @ Absyn::CREF_QUAL { componentRef: Deref @ Absyn::CREF_IDENT { name: Deref @ "error", .. }, name: Deref @ "AssertionLevel", .. } } }) => txt.clone(),
        (txt, Deref @ Absyn::CREF { componentRef: Deref @ Absyn::CREF_QUAL { componentRef: Deref @ Absyn::CREF_IDENT { name: Deref @ "error", .. }, name: Deref @ "AssertionLevel", .. } }) => txt.clone(),
        (txt, i_exp) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = AbsynDumpTpl::dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpAttributeDim(in_txt: Tpl::Text, in_a_attributes: SCode::Attributes) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attributes.clone()) {
        (mut txt, SCode::ATTR { arrayDims: ref i_arrayDims, .. }) => {
            txt = AbsynDumpTpl::dumpSubscripts(txt.clone(), i_arrayDims.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpAttributes(in_txt: Tpl::Text, in_a_attributes: SCode::Attributes) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attributes.clone()) {
        (mut txt, SCode::ATTR { direction: mut i_direction, variability: mut i_variability, parallelism: mut i_parallelism, connectorType: mut i_connectorType, .. }) => {
            let mut l_dir__str: Tpl::Text;
            let mut l_var__str: Tpl::Text;
            let mut l_prl__str: Tpl::Text;
            let mut l_ct__str: Tpl::Text;
            l_ct__str = dumpConnectorType(Tpl::emptyTxt.clone(), i_connectorType.clone())?;
            l_prl__str = dumpParallelism(Tpl::emptyTxt.clone(), i_parallelism.clone())?;
            l_var__str = dumpVariability(Tpl::emptyTxt.clone(), i_variability.clone())?;
            l_dir__str = dumpDirection(Tpl::emptyTxt.clone(), i_direction.clone())?;
            txt = Tpl::writeText(txt.clone(), l_prl__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_var__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_dir__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ct__str.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpClass(in_txt: Tpl::Text, in_a_class: Arc<SCode::Element>, in_a_each: ArcStr, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_class.clone(), in_a_each.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::CLASS { name: i_name, cmt: i_cmt, classDef: i_classDef, restriction: i_restriction, partialPrefix: i_partialPrefix, encapsulatedPrefix: i_encapsulatedPrefix, prefixes: i_prefixes, .. }, a_each, a_options) => {
            let mut l_footer__str: Tpl::Text;
            let mut l_header__str: Tpl::Text;
            let mut l_cc__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_prefixes__str: Tpl::Text;
            let mut l_res__str: Tpl::Text;
            let mut l_partial__str: Tpl::Text;
            let mut l_enc__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_prefix__str = dumpPrefixes(Tpl::emptyTxt.clone(), i_prefixes.clone(), (a_each.clone()).clone())?;
            l_enc__str = dumpEncapsulated(Tpl::emptyTxt.clone(), i_encapsulatedPrefix.clone())?;
            l_partial__str = dumpPartial(Tpl::emptyTxt.clone(), i_partialPrefix.clone())?;
            l_res__str = dumpRestriction(Tpl::emptyTxt.clone(), i_restriction.clone())?;
            l_prefixes__str = Tpl::writeText(Tpl::emptyTxt.clone(), l_prefix__str.clone())?;
            l_prefixes__str = Tpl::writeText(l_prefixes__str.clone(), l_enc__str.clone())?;
            l_prefixes__str = Tpl::writeText(l_prefixes__str.clone(), l_partial__str.clone())?;
            l_prefixes__str = Tpl::writeText(l_prefixes__str.clone(), l_res__str.clone())?;
            l_cdef__str = dumpClassDef(Tpl::emptyTxt.clone(), i_classDef.clone(), a_options.clone())?;
            l_cmt__str = dumpClassComment(Tpl::emptyTxt.clone(), i_cmt.clone(), a_options.clone())?;
            l_ann__str = dumpClassAnnotation(Tpl::emptyTxt.clone(), i_cmt.clone(), a_options.clone())?;
            l_cc__str = dumpReplaceableConstrainClass(Tpl::emptyTxt.clone(), i_prefixes.clone(), a_options.clone())?;
            l_header__str = dumpClassHeader(Tpl::emptyTxt.clone(), i_classDef.clone(), (i_name.clone()).clone(), i_restriction.clone(), (Tpl::textString(l_cmt__str.clone())?).clone(), a_options.clone())?;
            l_footer__str = dumpClassFooter(Tpl::emptyTxt.clone(), i_classDef.clone(), (Tpl::textString(l_cdef__str.clone())?).clone(), (i_name.clone()).clone(), (Tpl::textString(l_cmt__str.clone())?).clone(), (Tpl::textString(l_ann__str.clone())?).clone(), (Tpl::textString(l_cc__str.clone())?).clone())?;
            txt = Tpl::writeText(txt.clone(), l_prefixes__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_header__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_footer__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassAnnotation(in_txt: Tpl::Text, in_a_comment: Arc<SCode::Comment>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::COMMENT { annotation_: i_annotation__, .. }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpAnnotationOpt(txt.clone(), i_annotation__.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassComment(in_txt: Tpl::Text, in_a_comment: Arc<SCode::Comment>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::COMMENT { comment: i_comment, .. }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpCommentStr(txt.clone(), i_comment.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassDef(in_txt: Tpl::Text, in_a_classDef: Arc<SCode::ClassDef>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classDef.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::PARTS { externalDecl: i_p_externalDecl, initialAlgorithmLst: i_p_initialAlgorithmLst, normalAlgorithmLst: i_p_normalAlgorithmLst, initialEquationLst: i_initialEquationLst, normalEquationLst: i_normalEquationLst, elementLst: i_elementLst, .. }, a_options) => {
            let mut i_pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_derivedVariables: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionPath: Arc<Absyn::Path>;
            let mut i_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut i_attributes: SCode::Attributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_composition: Arc<SCode::ClassDef>;
            let mut i_modifications: Arc<SCode::Mod>;
            let mut l_func__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_extdecl__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_nal__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_neq__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_el__str = dumpElements(Tpl::emptyTxt.clone(), i_elementLst.clone(), true, a_options.clone())?;
            l_neq__str = dumpEquations(Tpl::emptyTxt.clone(), i_normalEquationLst.clone(), (literal!("equation")).clone(), a_options.clone())?;
            l_ieq__str = dumpEquations(Tpl::emptyTxt.clone(), i_initialEquationLst.clone(), (literal!("initial equation")).clone(), a_options.clone())?;
            l_nal__str = fun_37(Tpl::emptyTxt.clone(), a_options.clone(), i_p_normalAlgorithmLst.clone())?;
            l_ial__str = fun_38(Tpl::emptyTxt.clone(), a_options.clone(), i_p_initialAlgorithmLst.clone())?;
            l_extdecl__str = dumpExternalDeclOpt(Tpl::emptyTxt.clone(), i_p_externalDecl.clone(), a_options.clone())?;
            l_cdef__str = Tpl::writeText(Tpl::emptyTxt.clone(), l_el__str.clone())?;
            l_cdef__str = Tpl::softNewLine(l_cdef__str.clone())?;
            l_cdef__str = Tpl::writeText(l_cdef__str.clone(), l_ieq__str.clone())?;
            l_cdef__str = Tpl::softNewLine(l_cdef__str.clone())?;
            l_cdef__str = Tpl::writeText(l_cdef__str.clone(), l_ial__str.clone())?;
            l_cdef__str = Tpl::softNewLine(l_cdef__str.clone())?;
            l_cdef__str = Tpl::writeText(l_cdef__str.clone(), l_neq__str.clone())?;
            l_cdef__str = Tpl::softNewLine(l_cdef__str.clone())?;
            l_cdef__str = Tpl::writeText(l_cdef__str.clone(), l_nal__str.clone())?;
            l_cdef__str = Tpl::softNewLine(l_cdef__str.clone())?;
            l_cdef__str = Tpl::pushBlock(l_cdef__str.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            l_cdef__str = Tpl::writeText(l_cdef__str.clone(), l_extdecl__str.clone())?;
            l_cdef__str = Tpl::popBlock(l_cdef__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cdef__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::CLASS_EXTENDS { composition: i_composition, modifications: i_modifications }, a_options) => {
            let mut i_pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_derivedVariables: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionPath: Arc<Absyn::Path>;
            let mut i_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut i_attributes: SCode::Attributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_p_externalDecl: Option<Arc<SCode::ExternalDecl>>;
            let mut i_p_initialAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_p_normalAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_initialEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_normalEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut l_func__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_extdecl__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_nal__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_neq__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_mod__str = dumpModifier(Tpl::emptyTxt.clone(), i_modifications.clone(), a_options.clone())?;
            l_cdef__str = dumpClassDef(Tpl::emptyTxt.clone(), i_composition.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cdef__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::DERIVED { attributes: i_attributes, modifications: i_modifications, typeSpec: i_typeSpec }, a_options) => {
            let mut i_pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_derivedVariables: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionPath: Arc<Absyn::Path>;
            let mut i_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut i_composition: Arc<SCode::ClassDef>;
            let mut i_p_externalDecl: Option<Arc<SCode::ExternalDecl>>;
            let mut i_p_initialAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_p_normalAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_initialEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_normalEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut l_func__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_extdecl__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_nal__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_neq__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_type__str = AbsynDumpTpl::dumpTypeSpec(Tpl::emptyTxt.clone(), i_typeSpec.clone())?;
            l_mod__str = dumpModifier(Tpl::emptyTxt.clone(), i_modifications.clone(), a_options.clone())?;
            l_attr__str = dumpAttributes(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_attr__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_type__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::ENUMERATION { enumLst: i_enumLst }, a_options) => {
            let mut i_pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_derivedVariables: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionPath: Arc<Absyn::Path>;
            let mut i_attributes: SCode::Attributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_composition: Arc<SCode::ClassDef>;
            let mut i_modifications: Arc<SCode::Mod>;
            let mut i_p_externalDecl: Option<Arc<SCode::ExternalDecl>>;
            let mut i_p_initialAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_p_normalAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_initialEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_normalEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut l_func__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_extdecl__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_nal__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_neq__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_enum__str = fun_40(Tpl::emptyTxt.clone(), i_enumLst.clone(), a_options.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= enumeration(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_enum__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::PDER { derivedVariables: i_derivedVariables, functionPath: i_functionPath }, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut i_attributes: SCode::Attributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_composition: Arc<SCode::ClassDef>;
            let mut i_modifications: Arc<SCode::Mod>;
            let mut i_p_externalDecl: Option<Arc<SCode::ExternalDecl>>;
            let mut i_p_initialAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_p_normalAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_initialEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_normalEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut l_func__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_extdecl__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_nal__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_neq__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_functionPath.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= der(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_41(txt.clone(), i_derivedVariables.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::OVERLOAD { pathLst: i_pathLst }, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_derivedVariables: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionPath: Arc<Absyn::Path>;
            let mut i_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut i_attributes: SCode::Attributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_composition: Arc<SCode::ClassDef>;
            let mut i_modifications: Arc<SCode::Mod>;
            let mut i_p_externalDecl: Option<Arc<SCode::ExternalDecl>>;
            let mut i_p_initialAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_p_normalAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_initialEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_normalEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut l_func__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_extdecl__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_nal__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_neq__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= overload(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_42(txt.clone(), i_pathLst.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_pathLst: Arc<metamodelica::List<Arc<Absyn::Path>>>;
            let mut i_derivedVariables: Arc<metamodelica::List<ArcStr>>;
            let mut i_functionPath: Arc<Absyn::Path>;
            let mut i_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut i_attributes: SCode::Attributes;
            let mut i_typeSpec: Arc<Absyn::TypeSpec>;
            let mut i_composition: Arc<SCode::ClassDef>;
            let mut i_modifications: Arc<SCode::Mod>;
            let mut i_p_externalDecl: Option<Arc<SCode::ExternalDecl>>;
            let mut i_p_initialAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_p_normalAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>;
            let mut i_initialEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_normalEquationLst: Arc<metamodelica::List<Arc<SCode::Equation>>>;
            let mut i_elementLst: Arc<metamodelica::List<Arc<SCode::Element>>>;
            let mut l_func__str: Tpl::Text;
            let mut l_enum__str: Tpl::Text;
            let mut l_attr__str: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_cdef__str: Tpl::Text;
            let mut l_extdecl__str: Tpl::Text;
            let mut l_ial__str: Tpl::Text;
            let mut l_nal__str: Tpl::Text;
            let mut l_ieq__str: Tpl::Text;
            let mut l_neq__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpClassDef: Unknown class definition.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn dumpClassDefSpacing(in_txt: Tpl::Text, in_a_classDef: Arc<SCode::ClassDef>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classDef.clone())) {
        (txt, Deref @ SCode::CLASS_EXTENDS { composition: i_composition, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpClassDefSpacing(txt.clone(), i_composition.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::PARTS { elementLst: _, .. }) => {
            let mut i_composition: Arc<SCode::ClassDef>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassFooter(in_txt: Tpl::Text, in_a_classDef: Arc<SCode::ClassDef>, in_a_cdefStr: ArcStr, in_a_name: ArcStr, in_a_cmt: ArcStr, in_a_ann: ArcStr, in_a_cc__str: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classDef.clone(), in_a_cdefStr.clone(), in_a_name.clone(), in_a_cmt.clone(), in_a_ann.clone(), in_a_cc__str.clone())) {
        (txt, Deref @ SCode::DERIVED { typeSpec: _, .. }, a_cdefStr, _, a_cmt, a_ann, a_cc__str) => {
            let mut a_name: ArcStr;
            let mut l_annstr: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_cdefStr.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cmt.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_ann.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cc__str.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::ENUMERATION { enumLst: _ }, a_cdefStr, _, a_cmt, a_ann, a_cc__str) => {
            let mut a_name: ArcStr;
            let mut l_annstr: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_cdefStr.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cmt.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_ann.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cc__str.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::PDER { functionPath: _, .. }, a_cdefStr, _, _, _, _) => {
            let mut a_name: ArcStr;
            let mut a_cmt: ArcStr;
            let mut a_ann: ArcStr;
            let mut a_cc__str: ArcStr;
            let mut l_annstr: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_cdefStr.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_cdefStr, a_name, _, a_ann, a_cc__str) => {
            let mut a_cmt: ArcStr;
            let mut l_annstr: Tpl::Text;
            let mut txt = (*txt).clone();
            l_annstr = fun_44(Tpl::emptyTxt.clone(), (a_ann.clone()).clone())?;
            txt = fun_46(txt.clone(), (a_cdefStr.clone()).clone(), (a_cc__str.clone()).clone(), (a_name.clone()).clone(), l_annstr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpClassHeader(in_txt: Tpl::Text, in_a_classDef: Arc<SCode::ClassDef>, in_a_name: ArcStr, in_a_restr: SCode::Restriction, in_a_cmt: ArcStr, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_classDef.clone(), in_a_name.clone(), in_a_restr.clone(), in_a_cmt.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::CLASS_EXTENDS { modifications: i_modifications, .. }, a_name, _, a_cmt, a_options) => {
            let mut a_restr: SCode::Restriction;
            let mut l_mod__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_mod__str = dumpModifier(Tpl::emptyTxt.clone(), i_modifications.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("extends ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cmt.clone()).clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::PARTS { elementLst: _, .. }, a_name, a_restr, a_cmt, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_modifications: Arc<SCode::Mod>;
            let mut l_mod__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = dumpRestrictionTypeVars(txt.clone(), a_restr.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cmt.clone()).clone())?;
            txt.clone()
        },
        (txt, _, a_name, _, _, _) => {
            let mut a_restr: SCode::Restriction;
            let mut a_cmt: ArcStr;
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_modifications: Arc<SCode::Mod>;
            let mut l_mod__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpComment(in_txt: Tpl::Text, in_a_comment: Arc<SCode::Comment>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_comment.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::COMMENT { comment: i_comment, annotation_: i_annotation__ }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ann__str = dumpAnnotationOpt(Tpl::emptyTxt.clone(), i_annotation__.clone(), a_options.clone())?;
            l_cmt__str = dumpCommentStr(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpCommentOpt(in_txt: Tpl::Text, in_a_comment: Option<Arc<SCode::Comment>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_comment.clone(), in_a_options.clone()) {
        (mut txt, Some(mut i_cmt), mut a_options) => {
            txt = dumpComment(txt.clone(), i_cmt.clone(), a_options.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpCommentStr(txt: Tpl::Text, a_comment: Option<ArcStr>, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_148(txt.clone(), a_options.clone(), a_comment.clone())?;
    Ok(out_txt)
}

pub fn dumpComponent(in_txt: Tpl::Text, in_a_component: Arc<SCode::Element>, in_a_each: ArcStr, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_component.clone(), in_a_each.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::COMPONENT { name: i_name, comment: i_comment, condition: i_condition, modifications: i_modifications, typeSpec: i_typeSpec, attributes: i_attributes, prefixes: i_prefixes, .. }, a_each, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_mod__str1: Tpl::Text;
            let mut l_type__str: Tpl::Text;
            let mut l_attr__dim__str: Tpl::Text;
            let mut l_attr__pre__str: Tpl::Text;
            let mut l_cc__str: Tpl::Text;
            let mut l_prefix__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_prefix__str = dumpPrefixes(Tpl::emptyTxt.clone(), i_prefixes.clone(), (a_each.clone()).clone())?;
            l_cc__str = dumpReplaceableConstrainClass(Tpl::emptyTxt.clone(), i_prefixes.clone(), a_options.clone())?;
            l_attr__pre__str = dumpAttributes(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            l_attr__dim__str = dumpAttributeDim(Tpl::emptyTxt.clone(), i_attributes.clone())?;
            l_type__str = AbsynDumpTpl::dumpTypeSpec(Tpl::emptyTxt.clone(), i_typeSpec.clone())?;
            l_mod__str1 = dumpModifier(Tpl::emptyTxt.clone(), i_modifications.clone(), a_options.clone())?;
            l_mod__str = fun_51(Tpl::emptyTxt.clone(), a_options.clone(), i_attributes.clone(), l_mod__str1.clone())?;
            l_cond__str = fun_52(Tpl::emptyTxt.clone(), i_condition.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_prefix__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_attr__pre__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_type__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_attr__dim__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cc__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpConnectorType(in_txt: Tpl::Text, in_a_connectorType: SCode::ConnectorType) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_connectorType.clone()) {
        (mut txt, SCode::FLOW) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("flow ")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::STREAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("stream ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpDefineUnit(in_txt: Tpl::Text, in_a_defineUnit: Arc<SCode::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_defineUnit.clone())) {
        (txt, Deref @ SCode::DEFINEUNIT { name: i_name, weight: i_weight, exp: i_exp, visibility: i_visibility, .. }) => {
            let mut l_pe: Tpl::Text;
            let mut l_pb: Tpl::Text;
            let mut l_args__str: Tpl::Text;
            let mut l_weight__str: Tpl::Text;
            let mut l_exp__str: Tpl::Text;
            let mut l_vis__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_vis__str = dumpVisibility(Tpl::emptyTxt.clone(), i_visibility.clone())?;
            l_exp__str = fun_54(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_weight__str = fun_55(Tpl::emptyTxt.clone(), i_weight.clone())?;
            l_args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_args__str = smf_56(l_args__str.clone(), l_exp__str.clone())?;
            l_args__str = smf_57(l_args__str.clone(), l_weight__str.clone())?;
            l_args__str = Tpl::popIter(l_args__str.clone())?;
            l_pb = fun_58(Tpl::emptyTxt.clone(), l_args__str.clone())?;
            l_pe = fun_59(Tpl::emptyTxt.clone(), l_args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("defineunit ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_pb.clone())?;
            txt = Tpl::writeText(txt.clone(), l_args__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_pe.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpDirection(in_txt: Tpl::Text, in_a_direction: Absyn::Direction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_direction.clone()) {
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

pub fn dumpEach(in_txt: Tpl::Text, in_a_each: SCode::Each) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_each.clone()) {
        (mut txt, SCode::EACH) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("each ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpElement(in_txt: Tpl::Text, in_a_element: Arc<SCode::Element>, in_a_each: ArcStr, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_element.clone(), in_a_each.clone(), in_a_options.clone())) {
        (txt, i_element @ Deref @ SCode::IMPORT { visibility: i_visibility, .. }, _, a_options) => {
            let mut a_each: ArcStr;
            let mut txt = (*txt).clone();
            txt = fun_26(txt.clone(), i_visibility.clone(), i_element.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_element @ Deref @ SCode::EXTENDS { baseClassPath: _, .. }, _, a_options) => {
            let mut a_each: ArcStr;
            let mut i_visibility: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpExtends(txt.clone(), i_element.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_element @ Deref @ SCode::CLASS { name: _, .. }, a_each, a_options) => {
            let mut i_visibility: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpClass(txt.clone(), i_element.clone(), (a_each.clone()).clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_element @ Deref @ SCode::COMPONENT { name: _, .. }, a_each, a_options) => {
            let mut i_visibility: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpComponent(txt.clone(), i_element.clone(), (a_each.clone()).clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_element @ Deref @ SCode::DEFINEUNIT { name: _, .. }, _, _) => {
            let mut a_each: ArcStr;
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_visibility: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpDefineUnit(txt.clone(), i_element.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => {
            let mut a_each: ArcStr;
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_element: Arc<SCode::Element>;
            let mut i_visibility: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpElement: Unknown element.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementSpacing(in_txt: Tpl::Text, in_a_element: Arc<SCode::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_element.clone())) {
        (txt, Deref @ SCode::CLASS { classDef: i_classDef, .. }) => {
            let mut txt = (*txt).clone();
            txt = dumpClassDefSpacing(txt.clone(), i_classDef.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElementVisibility(in_txt: Tpl::Text, in_a_element: Arc<SCode::Element>, in_a_inPublicSection: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_element.clone(), in_a_inPublicSection.clone())) {
        (txt, Deref @ SCode::IMPORT { visibility: i_visibility, .. }, a_inPublicSection) => {
            let mut i_vis: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpSectionVisibility(txt.clone(), i_visibility.clone(), a_inPublicSection.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::EXTENDS { visibility: i_visibility, .. }, a_inPublicSection) => {
            let mut i_vis: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpSectionVisibility(txt.clone(), i_visibility.clone(), a_inPublicSection.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::CLASS { prefixes: Deref @ SCode::PREFIXES { visibility: i_vis, .. }, .. }, a_inPublicSection) => {
            let mut i_visibility: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpSectionVisibility(txt.clone(), i_vis.clone(), a_inPublicSection.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::COMPONENT { prefixes: Deref @ SCode::PREFIXES { visibility: i_vis, .. }, .. }, a_inPublicSection) => {
            let mut i_visibility: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpSectionVisibility(txt.clone(), i_vis.clone(), a_inPublicSection.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::DEFINEUNIT { visibility: i_visibility, .. }, a_inPublicSection) => {
            let mut i_vis: SCode::Visibility;
            let mut txt = (*txt).clone();
            txt = dumpSectionVisibility(txt.clone(), i_visibility.clone(), a_inPublicSection.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElements(txt: Tpl::Text, a_elements: Arc<metamodelica::List<Arc<SCode::Element>>>, a_indent: bool, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_0: Arc<metamodelica::List<Arc<SCode::Element>>>;
    ret_0 = SCodeDump::filterElements(a_elements.clone(), a_options.clone());
    out_txt = dumpElements2(txt.clone(), ret_0.clone(), a_indent.clone(), a_options.clone())?;
    Ok(out_txt)
}

pub fn dumpElements2(txt: Tpl::Text, a_elements: Arc<metamodelica::List<Arc<SCode::Element>>>, a_indent: bool, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut ret_2: metamodelica::Array<bool>;
    let mut ret_1: metamodelica::Array<bool>;
    let mut ret_0: i32;
    ret_0 = (a_elements.clone().len() as i32);
    ret_1 = Util::makeStatefulBoolean(false);
    ret_2 = Util::makeStatefulBoolean(true);
    out_txt = dumpElements3(txt.clone(), a_elements.clone(), ret_0.clone(), ret_1.clone(), a_indent.clone(), ret_2.clone(), a_options.clone())?;
    Ok(out_txt)
}

pub fn dumpElements3(txt: Tpl::Text, a_elements: Arc<metamodelica::List<Arc<SCode::Element>>>, a_numElements: i32, a_prevSpacing: metamodelica::Array<bool>, a_indent: bool, a_inPublicSection: metamodelica::Array<bool>, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 1, empty: None, separator: None, alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_19(out_txt.clone(), a_elements.clone(), a_indent.clone(), a_numElements.clone(), a_inPublicSection.clone(), a_options.clone(), a_prevSpacing.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpElseIfEquation(in_txt: Tpl::Text, in_a_condition: Arc<metamodelica::List<Arc<Absyn::Exp>>>, in_a_branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_condition.clone(), in_a_branches.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_cond, tail: i_rest__conds }, a_branches, a_options) => {
            let mut txt = (*txt).clone();
            txt = fun_70(txt.clone(), a_branches.clone(), i_rest__conds.clone(), a_options.clone(), i_cond.clone())?;
            txt.clone()
        },
        (txt, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpElseIfStatements(txt: Tpl::Text, a_else__if: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_87(out_txt.clone(), a_else__if.clone(), a_options.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpEncapsulated(in_txt: Tpl::Text, in_a_encapsulated: SCode::Encapsulated) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_encapsulated.clone()) {
        (mut txt, SCode::ENCAPSULATED) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("encapsulated ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpEnumLiteral(in_txt: Tpl::Text, in_a_enum: Arc<SCode::Enum>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_enum.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ENUM { literal: i_literal, comment: i_comment }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_literal.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEquation(in_txt: Tpl::Text, in_a_equation: Arc<SCode::Equation>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equation.clone(), in_a_options.clone())) {
        (txt, i_equation @ Deref @ SCode::EQ_IF { condition: _, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpIfEquation(txt.clone(), i_equation.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::EQ_EQUALS { comment: i_comment, expRight: i_expRight, expLeft: i_expLeft, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_equation: Arc<SCode::Equation>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = AbsynDumpTpl::dumpLhsExp(Tpl::emptyTxt.clone(), i_expLeft.clone())?;
            l_rhs__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_expRight.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::EQ_CONNECT { comment: i_comment, crefRight: i_crefRight, crefLeft: i_crefLeft, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut i_equation: Arc<SCode::Equation>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = AbsynDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_crefLeft.clone())?;
            l_rhs__str = AbsynDumpTpl::dumpCref(Tpl::emptyTxt.clone(), i_crefRight.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("connect(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, i_equation @ Deref @ SCode::EQ_FOR { index: _, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpForEquation(txt.clone(), i_equation.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_equation @ Deref @ SCode::EQ_WHEN { condition: _, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpWhenEquation(txt.clone(), i_equation.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::EQ_ASSERT { comment: i_comment, level: i_level, message: i_message, condition: i_condition, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut i_equation: Arc<SCode::Equation>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_condition.clone())?;
            l_msg__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_message.clone())?;
            l_lvl__str = dumpAssertionLevel(Tpl::emptyTxt.clone(), i_level.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_msg__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lvl__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::EQ_TERMINATE { comment: i_comment, message: i_message, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut i_equation: Arc<SCode::Equation>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_msg__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_message.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("terminate(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_msg__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::EQ_REINIT { comment: i_comment, expReinit: i_expReinit, cref: i_cref, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut i_equation: Arc<SCode::Equation>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cref__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_cref.clone())?;
            l_exp__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_expReinit.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cref__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::EQ_NORETCALL { comment: i_comment, exp: i_exp, .. }, a_options) => {
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut i_equation: Arc<SCode::Equation>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_expReinit: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_crefRight: Arc<Absyn::ComponentRef>;
            let mut i_crefLeft: Arc<Absyn::ComponentRef>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_expRight: Arc<Absyn::Exp>;
            let mut i_expLeft: Arc<Absyn::Exp>;
            let mut i_equation: Arc<SCode::Equation>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cref__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpEquation: Unknown Equation.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpEquations(in_txt: Tpl::Text, in_a_equations: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_label: ArcStr, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_equations.clone(), in_a_label.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _) => txt.clone(),
        (txt, i_equations, a_label, a_options) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (a_label.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_62(txt.clone(), i_equations.clone(), a_options.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpExtends(in_txt: Tpl::Text, in_a_extends: Arc<SCode::Element>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_extends.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::EXTENDS { ann: i_ann, modifications: i_modifications, visibility: i_visibility, baseClassPath: i_baseClassPath, .. }, a_options) => {
            let mut l_ann__str: Tpl::Text;
            let mut l_mod__str: Tpl::Text;
            let mut l_visibility__str: Tpl::Text;
            let mut l_bc__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_bc__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_baseClassPath.clone())?;
            l_visibility__str = dumpVisibility(Tpl::emptyTxt.clone(), i_visibility.clone())?;
            l_mod__str = dumpModifier(Tpl::emptyTxt.clone(), i_modifications.clone(), a_options.clone())?;
            l_ann__str = dumpAnnotationOpt(Tpl::emptyTxt.clone(), i_ann.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_visibility__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("extends ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_bc__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpExternalDecl(txt: Tpl::Text, a_externalDecl: Arc<SCode::ExternalDecl>, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    let mut l_res: Tpl::Text;
    l_res = fun_141(Tpl::emptyTxt.clone(), a_externalDecl.clone(), a_options.clone())?;
    out_txt = fun_143(txt.clone(), a_externalDecl.clone(), a_options.clone(), l_res.clone())?;
    Ok(out_txt)
}

pub fn dumpExternalDeclOpt(in_txt: Tpl::Text, in_a_externalDecl: Option<Arc<SCode::ExternalDecl>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_externalDecl.clone(), in_a_options.clone()) {
        (mut txt, Some(mut i_extdecl), mut a_options) => {
            txt = dumpExternalDecl(txt.clone(), i_extdecl.clone(), a_options.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpFinal(in_txt: Tpl::Text, in_a_final: SCode::Final) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_final.clone()) {
        (mut txt, SCode::FINAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("final ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpForEquation(in_txt: Tpl::Text, in_a_for__equation: Arc<SCode::Equation>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_for__equation.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::EQ_FOR { index: i_index, comment: i_comment, eEquationLst: i_eEquationLst, range: Some(i_range), .. }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_eq__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_range__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_range.clone())?;
            l_eq__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_eq__str = lm_72(l_eq__str.clone(), i_eEquationLst.clone(), a_options.clone())?;
            l_eq__str = Tpl::popIter(l_eq__str.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_index.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::EQ_FOR { index: i_index, comment: i_comment, eEquationLst: i_eEquationLst, .. }, a_options) => {
            let mut i_range: Arc<Absyn::Exp>;
            let mut l_cmt__str: Tpl::Text;
            let mut l_eq__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_eq__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_eq__str = lm_73(l_eq__str.clone(), i_eEquationLst.clone(), a_options.clone())?;
            l_eq__str = Tpl::popIter(l_eq__str.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_index.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_eq__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpForStatement(in_txt: Tpl::Text, in_a_for__statement: Arc<SCode::Statement>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_for__statement.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ALG_FOR { index: i_index, comment: i_comment, forBody: i_forBody, range: Some(i_e), .. }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_range__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_e.clone())?;
            l_body__str = dumpStatements(Tpl::emptyTxt.clone(), i_forBody.clone(), a_options.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_index.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" in ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_range__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_FOR { index: i_index, comment: i_comment, forBody: i_forBody, .. }, a_options) => {
            let mut i_e: Arc<Absyn::Exp>;
            let mut l_cmt__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_range__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_body__str = dumpStatements(Tpl::emptyTxt.clone(), i_forBody.clone(), a_options.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("for ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_index.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end for")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpFunctionRestriction(in_txt: Tpl::Text, in_a_funcRest: SCode::FunctionRestriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_funcRest.clone()) {
        (mut txt, SCode::FR_NORMAL_FUNCTION { purity: mut i_purity }) => {
            txt = AbsynDumpTpl::dumpPurity(txt.clone(), i_purity.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::FR_EXTERNAL_FUNCTION { purity: mut i_purity }) => {
            txt = AbsynDumpTpl::dumpPurity(txt.clone(), i_purity.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::FR_OPERATOR_FUNCTION) => {
            let mut i_purity: Absyn::FunctionPurity;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("operator function")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::FR_RECORD_CONSTRUCTOR) => {
            let mut i_purity: Absyn::FunctionPurity;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("function")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            let mut i_purity: Absyn::FunctionPurity;
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpFunctionRestriction: Unknown Function restriction.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpIfEquation(in_txt: Tpl::Text, in_a_ifequation: Arc<SCode::Equation>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ifequation.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::EQ_IF { comment: i_comment, elseBranch: i_elseBranch, thenBranch: Deref @ metamodelica::List::Cons { head: i_if__branch, tail: i_elseif__branches }, condition: Deref @ metamodelica::List::Cons { head: i_if__cond, tail: i_elseif__conds }, .. }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_elseif__str: Tpl::Text;
            let mut l_if__branch__str: Tpl::Text;
            let mut l_if__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_if__cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_if__cond.clone())?;
            l_if__branch__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_if__branch__str = lm_65(l_if__branch__str.clone(), i_if__branch.clone(), a_options.clone())?;
            l_if__branch__str = Tpl::popIter(l_if__branch__str.clone())?;
            l_elseif__str = dumpElseIfEquation(Tpl::emptyTxt.clone(), i_elseif__conds.clone(), i_elseif__branches.clone(), a_options.clone())?;
            l_else__str = fun_67(Tpl::emptyTxt.clone(), i_elseBranch.clone(), a_options.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_if__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_if__branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elseif__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end if")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpIfStatement(in_txt: Tpl::Text, in_a_if__statement: Arc<SCode::Statement>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_if__statement.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ALG_IF { comment: i_comment, elseBranch: i_elseBranch, elseIfBranch: i_elseIfBranch, trueBranch: i_trueBranch, boolExpr: i_boolExpr, .. }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_else__branch__str: Tpl::Text;
            let mut l_else__if__str: Tpl::Text;
            let mut l_true__branch__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_boolExpr.clone())?;
            l_true__branch__str = dumpStatements(Tpl::emptyTxt.clone(), i_trueBranch.clone(), a_options.clone())?;
            l_else__if__str = dumpElseIfStatements(Tpl::emptyTxt.clone(), i_elseIfBranch.clone(), a_options.clone())?;
            l_else__branch__str = dumpStatements(Tpl::emptyTxt.clone(), i_elseBranch.clone(), a_options.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_true__branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__if__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_else__branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end if")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpImport(in_txt: Tpl::Text, in_a_import: Arc<SCode::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_import.clone())) {
        (txt, Deref @ SCode::IMPORT { imp: i_imp, visibility: i_visibility, .. }) => {
            let mut l_import__str: Tpl::Text;
            let mut l_visibility__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_visibility__str = dumpVisibility(Tpl::emptyTxt.clone(), i_visibility.clone())?;
            l_import__str = fun_32(Tpl::emptyTxt.clone(), i_imp.clone())?;
            txt = Tpl::writeText(txt.clone(), l_visibility__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_import__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpInnerOuter(in_txt: Tpl::Text, in_a_innerOuter: Absyn::InnerOuter) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_innerOuter.clone()) {
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

pub fn dumpModifier(in_txt: Tpl::Text, in_a_modifier: Arc<SCode::Mod>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modifier.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::MOD { subModLst: i_subModLst, binding: i_binding, .. }, a_options) => {
            let mut l_submod__str: Tpl::Text;
            let mut l_binding__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_binding__str = dumpModifierBinding(Tpl::emptyTxt.clone(), i_binding.clone())?;
            l_submod__str = fun_112(Tpl::emptyTxt.clone(), i_subModLst.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_submod__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_binding__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpModifierBinding(in_txt: Tpl::Text, in_a_binding: Option<Arc<Absyn::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_binding.clone()) {
        (mut txt, Some(mut i_exp)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("= ")).clone() }))?;
            txt = AbsynDumpTpl::dumpExp(txt.clone(), i_exp.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpModifierPrefix(in_txt: Tpl::Text, in_a_modifier: Arc<SCode::Mod>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modifier.clone())) {
        (txt, Deref @ SCode::MOD { eachPrefix: i_eachPrefix, finalPrefix: i_finalPrefix, .. }) => {
            let mut l_each__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            txt = Tpl::writeText(txt.clone(), l_each__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_final__str.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::REDECL { eachPrefix: i_eachPrefix, finalPrefix: i_finalPrefix, .. }) => {
            let mut l_each__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            txt = Tpl::writeText(txt.clone(), l_each__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_final__str.clone())?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpParallelism(in_txt: Tpl::Text, in_a_parallelism: SCode::Parallelism) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_parallelism.clone()) {
        (mut txt, SCode::PARGLOBAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parglobal ")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::PARLOCAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parlocal ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpPartial(in_txt: Tpl::Text, in_a_partial: SCode::Partial) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_partial.clone()) {
        (mut txt, SCode::PARTIAL) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("partial ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpPreElementSpacing(txt: Tpl::Text, a_curSpacing: ArcStr, a_prevSpacing: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = fun_21(txt.clone(), a_prevSpacing.clone(), (a_curSpacing.clone()).clone())?;
    Ok(out_txt)
}

pub fn dumpPrefixes(in_txt: Tpl::Text, in_a_prefixes: Arc<SCode::Prefixes>, in_a_each: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_prefixes.clone(), in_a_each.clone())) {
        (txt, Deref @ SCode::PREFIXES { replaceablePrefix: i_replaceablePrefix, innerOuter: i_innerOuter, finalPrefix: i_finalPrefix, redeclarePrefix: i_redeclarePrefix, .. }, a_each) => {
            let mut l_replaceable__str: Tpl::Text;
            let mut l_io__str: Tpl::Text;
            let mut l_final__str: Tpl::Text;
            let mut l_redeclare__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_redeclare__str = dumpRedeclare(Tpl::emptyTxt.clone(), i_redeclarePrefix.clone())?;
            l_final__str = dumpFinal(Tpl::emptyTxt.clone(), i_finalPrefix.clone())?;
            l_io__str = dumpInnerOuter(Tpl::emptyTxt.clone(), i_innerOuter.clone())?;
            l_replaceable__str = dumpReplaceable(Tpl::emptyTxt.clone(), i_replaceablePrefix.clone())?;
            txt = Tpl::writeText(txt.clone(), l_redeclare__str.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_each.clone()).clone())?;
            txt = Tpl::writeText(txt.clone(), l_final__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_io__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_replaceable__str.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpProgram(txt: Tpl::Text, a_program: Arc<metamodelica::List<Arc<SCode::Element>>>, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = dumpElements(txt.clone(), a_program.clone(), false, a_options.clone())?;
    Ok(out_txt)
}

pub fn dumpRedeclModifier(in_txt: Tpl::Text, in_a_modifier: Arc<SCode::Mod>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modifier.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::REDECL { element: i_element, eachPrefix: i_eachPrefix, .. }, a_options) => {
            let mut l_each__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_each__str = dumpEach(Tpl::emptyTxt.clone(), i_eachPrefix.clone())?;
            txt = dumpElement(txt.clone(), i_element.clone(), (Tpl::textString(l_each__str.clone())?).clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpRedeclare(in_txt: Tpl::Text, in_a_redeclare: SCode::Redeclare) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_redeclare.clone()) {
        (mut txt, SCode::REDECLARE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("redeclare ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpReplaceable(in_txt: Tpl::Text, in_a_replaceable: Arc<SCode::Replaceable>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_replaceable.clone())) {
        (txt, Deref @ SCode::REPLACEABLE { cc: _ }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("replaceable ")).clone() }))?;
            txt.clone()
        },
        (txt, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpReplaceableConstrainClass(in_txt: Tpl::Text, in_a_replaceable: Arc<SCode::Prefixes>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_replaceable.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::PREFIXES { replaceablePrefix: Deref @ SCode::REPLACEABLE { cc: Some(SCode::CONSTRAINCLASS { modifier: i_cc__mod, constrainingClass: i_cc__path, .. }) }, .. }, a_options) => {
            let mut l_mod__str: Tpl::Text;
            let mut l_path__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_path__str = AbsynDumpTpl::dumpPath(Tpl::emptyTxt.clone(), i_cc__path.clone())?;
            l_mod__str = dumpModifier(Tpl::emptyTxt.clone(), i_cc__mod.clone(), a_options.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constrainedby ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_path__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_mod__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpRestriction(in_txt: Tpl::Text, in_a_restriction: SCode::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_restriction.clone()) {
        (mut txt, SCode::R_CLASS) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("class")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_OPTIMIZATION) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("optimization")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_MODEL) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("model")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_RECORD { isOperator: mut i_isOperator }) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            txt = fun_104(txt.clone(), i_isOperator.clone())?;
            txt.clone()
        },
        (mut txt, SCode::R_OPERATOR) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("operator")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_BLOCK) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("block")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_CONNECTOR { isExpandable: mut i_isExpandable }) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isOperator: bool;
            txt = fun_105(txt.clone(), i_isExpandable.clone())?;
            txt.clone()
        },
        (mut txt, SCode::R_OPERATOR) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("operator")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_TYPE) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("type")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_PACKAGE) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("package")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_FUNCTION { functionRestriction: mut i_functionRestriction }) => {
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = dumpFunctionRestriction(txt.clone(), i_functionRestriction.clone())?;
            txt.clone()
        },
        (mut txt, SCode::R_ENUMERATION) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("enumeration")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_PREDEFINED_INTEGER) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("IntegerType")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_PREDEFINED_REAL) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("RealType")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_PREDEFINED_STRING) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("StringType")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_PREDEFINED_BOOLEAN) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("BooleanType")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_PREDEFINED_ENUMERATION) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("EnumType")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_METARECORD { name: _, .. }) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("record")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::R_UNIONTYPE { typeVars: _ }) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("uniontype")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            let mut i_functionRestriction: SCode::FunctionRestriction;
            let mut i_isExpandable: bool;
            let mut i_isOperator: bool;
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpRestriction: Unknown restriction.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

pub fn dumpRestrictionTypeVars(in_txt: Tpl::Text, in_a_restriction: SCode::Restriction) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_restriction.clone()) {
        (mut txt, SCode::R_UNIONTYPE { typeVars: ref i_typeVars }) => {
            txt = fun_108(txt.clone(), i_typeVars.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpSectionVisibility(in_txt: Tpl::Text, in_a_visibility: SCode::Visibility, in_a_inPublicSection: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_visibility.clone(), in_a_inPublicSection.clone()) {
        (mut txt, SCode::PUBLIC, mut a_inPublicSection) => {
            txt = fun_29(txt.clone(), a_inPublicSection.clone())?;
            txt.clone()
        },
        (mut txt, SCode::PROTECTED, mut a_inPublicSection) => {
            txt = fun_30(txt.clone(), a_inPublicSection.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpStatement(in_txt: Tpl::Text, in_a_statement: Arc<SCode::Statement>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_statement.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ALG_ASSIGN { comment: i_comment, value: i_value, assignComponent: i_assignComponent, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_lhs__str = AbsynDumpTpl::dumpLhsExp(Tpl::emptyTxt.clone(), i_assignComponent.clone())?;
            l_rhs__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_value.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lhs__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" := ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_rhs__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, i_statement @ Deref @ SCode::ALG_IF { boolExpr: _, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpIfStatement(txt.clone(), i_statement.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_statement @ Deref @ SCode::ALG_FOR { index: _, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpForStatement(txt.clone(), i_statement.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_statement @ Deref @ SCode::ALG_WHILE { boolExpr: _, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpWhileStatement(txt.clone(), i_statement.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, i_statement @ Deref @ SCode::ALG_WHEN_A { branches: _, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpWhenStatement(txt.clone(), i_statement.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_ASSERT { level: i_level, message: i_message, condition: i_condition, .. }, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_condition.clone())?;
            l_msg__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_message.clone())?;
            l_lvl__str = dumpAssertionLevel(Tpl::emptyTxt.clone(), i_level.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("assert(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_msg__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_lvl__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_TERMINATE { message: i_message, .. }, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_msg__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_message.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("terminate(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_msg__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_REINIT { newValue: i_newValue, cref: i_cref, .. }, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cr__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_cref.clone())?;
            l_exp__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_newValue.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("reinit(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cr__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(");")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_NORETCALL { comment: i_comment, exp: i_exp, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_exp__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_exp.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeText(txt.clone(), l_exp__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_RETURN { comment: i_comment, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("return")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_BREAK { comment: i_comment, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("break")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_FAILURE { comment: i_comment, stmts: Deref @ metamodelica::List::Cons { head: i_stmt, tail: Deref @ metamodelica::List::Nil }, .. }, a_options) => {
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("failure(")).clone() }))?;
            txt = dumpStatement(txt.clone(), i_stmt.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, i_statement @ Deref @ SCode::ALG_TRY { body: _, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = dumpTryStatement(txt.clone(), i_statement.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::ALG_CONTINUE { comment: i_comment, .. }, a_options) => {
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("continue")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_stmt: Arc<SCode::Statement>;
            let mut i_exp: Arc<Absyn::Exp>;
            let mut i_newValue: Arc<Absyn::Exp>;
            let mut i_cref: Arc<Absyn::Exp>;
            let mut i_level: Arc<Absyn::Exp>;
            let mut i_message: Arc<Absyn::Exp>;
            let mut i_condition: Arc<Absyn::Exp>;
            let mut i_statement: Arc<SCode::Statement>;
            let mut i_comment: Arc<SCode::Comment>;
            let mut i_value: Arc<Absyn::Exp>;
            let mut i_assignComponent: Arc<Absyn::Exp>;
            let mut l_exp__str: Tpl::Text;
            let mut l_cr__str: Tpl::Text;
            let mut l_lvl__str: Tpl::Text;
            let mut l_msg__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut l_rhs__str: Tpl::Text;
            let mut l_lhs__str: Tpl::Text;
            let mut txt = (*txt).clone();
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpStatement: Unknown statement.")).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpStatements(txt: Tpl::Text, a_statements: Arc<metamodelica::List<Arc<SCode::Statement>>>, a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_83(out_txt.clone(), a_statements.clone(), a_options.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    Ok(out_txt)
}

pub fn dumpSubModifier(in_txt: Tpl::Text, in_a_submod: Arc<SCode::SubMod>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_submod.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::NAMEMOD { ident: i_ident, r#mod: i_mod @ Deref @ SCode::MOD { finalPrefix: _, .. } }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpModifierPrefix(txt.clone(), i_mod.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = dumpModifier(txt.clone(), i_mod.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, Deref @ SCode::NAMEMOD { r#mod: i_mod @ Deref @ SCode::REDECL { finalPrefix: _, .. }, .. }, a_options) => {
            let mut i_ident: ArcStr;
            let mut txt = (*txt).clone();
            txt = dumpRedeclModifier(txt.clone(), i_mod.clone(), a_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpTryStatement(in_txt: Tpl::Text, in_a_try__statement: Arc<SCode::Statement>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_try__statement.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ALG_TRY { elseBody: i_elseBody, body: i_body, comment: i_comment, .. }, a_options) => {
            let mut l_algs2: Tpl::Text;
            let mut l_algs1: Tpl::Text;
            let mut l_cmt__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            l_algs1 = dumpStatements(Tpl::emptyTxt.clone(), i_body.clone(), a_options.clone())?;
            l_algs2 = dumpStatements(Tpl::emptyTxt.clone(), i_elseBody.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("try\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_algs1.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_algs2.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end try")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpVariability(in_txt: Tpl::Text, in_a_variability: SCode::Variability) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_variability.clone()) {
        (mut txt, SCode::DISCRETE) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("discrete ")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::PARAM) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("parameter ")).clone() }))?;
            txt.clone()
        },
        (mut txt, SCode::CONST) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("constant ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpVisibility(in_txt: Tpl::Text, in_a_visibility: SCode::Visibility) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_visibility.clone()) {
        (mut txt, SCode::PROTECTED) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("protected ")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

pub fn dumpWhenEquation(in_txt: Tpl::Text, in_a_when__equation: Arc<SCode::Equation>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_when__equation.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::EQ_WHEN { comment: i_comment, elseBranches: i_elseBranches, eEquationLst: i_eEquationLst, condition: i_condition, .. }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_else__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_condition.clone())?;
            l_body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_body__str = lm_75(l_body__str.clone(), i_eEquationLst.clone(), a_options.clone())?;
            l_body__str = Tpl::popIter(l_body__str.clone())?;
            l_else__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_else__str = lm_77(l_else__str.clone(), i_elseBranches.clone(), a_options.clone())?;
            l_else__str = Tpl::popIter(l_else__str.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_else__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpWhenStatement(in_txt: Tpl::Text, in_a_when__statement: Arc<SCode::Statement>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_when__statement.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ALG_WHEN_A { comment: i_comment, branches: Deref @ metamodelica::List::Cons { head: (i_when__cond, i_when__body), tail: i_elsewhens }, .. }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_elsewhen__str: Tpl::Text;
            let mut l_when__body__str: Tpl::Text;
            let mut l_when__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_when__cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_when__cond.clone())?;
            l_when__body__str = dumpStatements(Tpl::emptyTxt.clone(), i_when__body.clone(), a_options.clone())?;
            l_elsewhen__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_elsewhen__str = lm_91(l_elsewhen__str.clone(), i_elsewhens.clone(), a_options.clone())?;
            l_elsewhen__str = Tpl::popIter(l_elsewhen__str.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("when ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_when__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_when__body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_elsewhen__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end when")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn dumpWhileStatement(in_txt: Tpl::Text, in_a_while__statement: Arc<SCode::Statement>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_while__statement.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::ALG_WHILE { comment: i_comment, whileBody: i_whileBody, boolExpr: i_boolExpr, .. }, a_options) => {
            let mut l_cmt__str: Tpl::Text;
            let mut l_body__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_boolExpr.clone())?;
            l_body__str = dumpStatements(Tpl::emptyTxt.clone(), i_whileBody.clone(), a_options.clone())?;
            l_cmt__str = dumpComment(Tpl::emptyTxt.clone(), i_comment.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("while ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" loop\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end while")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cmt__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

pub fn errorMsg(txt: Tpl::Text, a_errMessage: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    Tpl::addTemplateError((a_errMessage.clone()).clone())?;
    out_txt = Tpl::writeStr(txt.clone(), (a_errMessage.clone()).clone())?;
    Ok(out_txt)
}

fn fun_104(in_txt: Tpl::Text, in_a_isOperator: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isOperator.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("record")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("operator record")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_105(in_txt: Tpl::Text, in_a_isExpandable: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_isExpandable.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("connector")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("expandable connector")).clone() }))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_108(in_txt: Tpl::Text, in_a_typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_typeVars.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, i_typeVars) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(",")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_107(txt.clone(), i_typeVars.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(">")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_112(in_txt: Tpl::Text, in_a_subModLst: Arc<metamodelica::List<Arc<SCode::SubMod>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_subModLst.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, i_subModLst, a_options) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_111(txt.clone(), i_subModLst.clone(), a_options.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_115(in_txt: Tpl::Text, in_a_text: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_text.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_text) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_text.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_121(in_txt: Tpl::Text, in_a_ident: ArcStr, in_a_options: SCodeDump::SCodeDumpOptions, in_a_nameMod: Arc<SCode::Mod>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ident.clone(), in_a_options.clone(), in_a_nameMod.clone())) {
        (txt, Deref @ "choices", _, _) => txt.clone(),
        (txt, Deref @ "Documentation", _, _) => txt.clone(),
        (txt, Deref @ "Dialog", _, _) => txt.clone(),
        (txt, Deref @ "Diagram", _, _) => txt.clone(),
        (txt, Deref @ "Icon", _, _) => txt.clone(),
        (txt, Deref @ "Line", _, _) => txt.clone(),
        (txt, Deref @ "Placement", _, _) => txt.clone(),
        (txt, Deref @ "preferredView", _, _) => txt.clone(),
        (txt, Deref @ "conversion", _, _) => txt.clone(),
        (txt, Deref @ "defaultComponentName", _, _) => txt.clone(),
        (txt, Deref @ "revisionId", _, _) => txt.clone(),
        (txt, Deref @ "uses", _, _) => txt.clone(),
        (txt, i_ident, a_options, a_nameMod) => {
            let mut txt = (*txt).clone();
            txt = dumpModifierPrefix(txt.clone(), a_nameMod.clone())?;
            txt = Tpl::writeStr(txt.clone(), (i_ident.clone()).clone())?;
            txt = dumpAnnotationModifier(txt.clone(), a_nameMod.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_122(in_txt: Tpl::Text, in_mArg: bool, in_a_mod: Arc<SCode::Mod>, in_a_options: SCodeDump::SCodeDumpOptions, in_a_nameMod: Arc<SCode::Mod>, in_a_ident: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_mArg.clone(), in_a_mod.clone(), in_a_options.clone(), in_a_nameMod.clone(), in_a_ident.clone())) {
        (txt, false, _, a_options, a_nameMod, a_ident) => {
            let mut a_mod: Arc<SCode::Mod>;
            let mut txt = (*txt).clone();
            txt = fun_121(txt.clone(), (a_ident.clone()).clone(), a_options.clone(), a_nameMod.clone())?;
            txt.clone()
        },
        (txt, _, a_mod, a_options, a_nameMod, a_ident) => {
            let mut txt = (*txt).clone();
            txt = dumpModifierPrefix(txt.clone(), a_mod.clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_ident.clone()).clone())?;
            txt = dumpAnnotationModifier(txt.clone(), a_nameMod.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_131(in_txt: Tpl::Text, in_a_modifStr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_modifStr.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_modifStr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("annotation")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_modifStr.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_133(in_txt: Tpl::Text, in_a_annstr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annstr.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, i_annstr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), i_annstr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_136(in_txt: Tpl::Text, in_a_funcName: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_funcName.clone()) {
        (mut txt, Some(mut i_name)) => {
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_138(in_txt: Tpl::Text, in_a_func__name__str: Tpl::Text, in_a_func__args__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_func__name__str.clone(), in_a_func__args__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => txt.clone(),
        (txt, i_func__name__str, a_func__args__str) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeText(txt.clone(), i_func__name__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_func__args__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_139(in_txt: Tpl::Text, in_a_lang: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_lang.clone()) {
        (mut txt, Some(mut i_l)) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_l.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_14(in_txt: Tpl::Text, in_mArg: bool, in_a_prevSpacing: metamodelica::Array<bool>, in_a_spacing: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_prevSpacing.clone(), in_a_spacing.clone()) {
        (mut txt, false, _, _) => txt.clone(),
        (mut txt, _, mut a_prevSpacing, mut a_spacing) => {
            let mut ret_0: bool;
            ret_0 = Util::getStatefulBoolean(a_prevSpacing.clone());
            txt = dumpPreElementSpacing(txt.clone(), (Tpl::textString(a_spacing.clone())?).clone(), ret_0.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_140(in_txt: Tpl::Text, in_a_output__: Option<Arc<Absyn::ComponentRef>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_output__.clone()) {
        (mut txt, Some(mut i_name)) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = AbsynDumpTpl::dumpCref(txt.clone(), i_name.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" =")).clone() }))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_141(in_txt: Tpl::Text, in_a_externalDecl: Arc<SCode::ExternalDecl>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_externalDecl.clone(), in_a_options.clone())) {
        (txt, Deref @ SCode::EXTERNALDECL { output_: i_output__, annotation_: i_annotation__, lang: i_lang, args: i_args, funcName: i_funcName }, a_options) => {
            let mut l_output__str: Tpl::Text;
            let mut l_ann__str: Tpl::Text;
            let mut l_lang__str: Tpl::Text;
            let mut l_func__str: Tpl::Text;
            let mut l_func__args__str: Tpl::Text;
            let mut l_func__name__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_func__name__str = fun_136(Tpl::emptyTxt.clone(), i_funcName.clone())?;
            l_func__args__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_func__args__str = lm_137(l_func__args__str.clone(), i_args.clone())?;
            l_func__args__str = Tpl::popIter(l_func__args__str.clone())?;
            l_func__str = fun_138(Tpl::emptyTxt.clone(), l_func__name__str.clone(), l_func__args__str.clone())?;
            l_lang__str = fun_139(Tpl::emptyTxt.clone(), i_lang.clone())?;
            l_ann__str = dumpAnnotationOpt(Tpl::emptyTxt.clone(), i_annotation__.clone(), a_options.clone())?;
            l_output__str = fun_140(Tpl::emptyTxt.clone(), i_output__.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("external")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_lang__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_output__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_func__str.clone())?;
            txt = Tpl::writeText(txt.clone(), l_ann__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_142(in_txt: Tpl::Text, in_a_options: SCodeDump::SCodeDumpOptions, in_a_res: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_options.clone(), in_a_res.clone()) {
        (mut txt, SCodeDump::OPTIONS { stripExternalDecl: false, .. }, mut a_res) => {
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_143(in_txt: Tpl::Text, in_a_externalDecl: Arc<SCode::ExternalDecl>, in_a_options: SCodeDump::SCodeDumpOptions, in_a_res: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_externalDecl.clone(), in_a_options.clone(), in_a_res.clone())) {
        (txt, Deref @ SCode::EXTERNALDECL { lang: Some(Deref @ "builtin"), .. }, _, a_res) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_res.clone())?;
            txt.clone()
        },
        (txt, _, a_options, a_res) => {
            let mut txt = (*txt).clone();
            txt = fun_142(txt.clone(), a_options.clone(), a_res.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_147(in_txt: Tpl::Text, in_a_comment: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_comment.clone()) {
        (mut txt, Some(mut i_cmt)) => {
            let mut ret_0: ArcStr;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            ret_0 = (System::escapedString((i_cmt.clone()).clone(), false)).clone();
            txt = Tpl::writeStr(txt.clone(), (ret_0.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_148(in_txt: Tpl::Text, in_a_options: SCodeDump::SCodeDumpOptions, in_a_comment: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_options.clone(), in_a_comment.clone()) {
        (mut txt, SCodeDump::OPTIONS { stripStringComments: false, .. }, mut a_comment) => {
            txt = fun_147(txt.clone(), a_comment.clone())?;
            txt.clone()
        },
        (mut txt, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_15(in_txt: Tpl::Text, in_a_vis__str: Tpl::Text, in_a_inPublicSection: metamodelica::Array<bool>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_vis__str.clone(), in_a_inPublicSection.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, _) => txt.clone(),
        (txt, _, a_inPublicSection) => {
            let mut ret_1: bool;
            let mut ret_0: bool;
            ret_0 = Util::getStatefulBoolean(a_inPublicSection.clone());
            ret_1 = boolNot(ret_0.clone());
            Util::setStatefulBoolean(a_inPublicSection.clone(), ret_1.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_16(in_txt: Tpl::Text, in_a_spacing: Tpl::Text, in_a_prevSpacing: metamodelica::Array<bool>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_spacing.clone(), in_a_prevSpacing.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }, a_prevSpacing) => {
            let mut i_spacing: Tpl::Text;
            Util::setStatefulBoolean(a_prevSpacing.clone(), false)?;
            txt.clone()
        },
        (txt, i_spacing, a_prevSpacing) => {
            let mut txt = (*txt).clone();
            Util::setStatefulBoolean(a_prevSpacing.clone(), true)?;
            txt = Tpl::writeText(txt.clone(), i_spacing.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_17(in_txt: Tpl::Text, in_mArg: bool, in_a_spacing: Tpl::Text, in_a_prevSpacing: metamodelica::Array<bool>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_mArg.clone(), in_a_spacing.clone(), in_a_prevSpacing.clone()) {
        (mut txt, false, _, mut a_prevSpacing) => {
            let mut a_spacing: Tpl::Text;
            Util::setStatefulBoolean(a_prevSpacing.clone(), false)?;
            txt.clone()
        },
        (mut txt, _, mut a_spacing, mut a_prevSpacing) => {
            txt = fun_16(txt.clone(), a_spacing.clone(), a_prevSpacing.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_18(in_txt: Tpl::Text, in_a_indent: bool, in_a_post__spacing: Tpl::Text, in_a_el__str: Tpl::Text, in_a_vis__str: Tpl::Text, in_a_pre__spacing: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_indent.clone(), in_a_post__spacing.clone(), in_a_el__str.clone(), in_a_vis__str.clone(), in_a_pre__spacing.clone()) {
        (mut txt, false, mut a_post__spacing, mut a_el__str, mut a_vis__str, mut a_pre__spacing) => {
            txt = Tpl::writeText(txt.clone(), a_pre__spacing.clone())?;
            txt = Tpl::writeText(txt.clone(), a_vis__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), a_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_post__spacing.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, _, mut a_post__spacing, mut a_el__str, mut a_vis__str, mut a_pre__spacing) => {
            txt = Tpl::writeText(txt.clone(), a_pre__spacing.clone())?;
            txt = Tpl::writeText(txt.clone(), a_vis__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), a_el__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), a_post__spacing.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_21(in_txt: Tpl::Text, in_a_prevSpacing: bool, in_a_curSpacing: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_prevSpacing.clone(), in_a_curSpacing.clone()) {
        (mut txt, false, mut a_curSpacing) => {
            txt = Tpl::writeStr(txt.clone(), (a_curSpacing.clone()).clone())?;
            txt.clone()
        },
        (mut txt, _, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_25(in_txt: Tpl::Text, in_a_options: SCodeDump::SCodeDumpOptions, in_a_element: Arc<SCode::Element>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_options.clone(), in_a_element.clone())) {
        (txt, SCodeDump::OPTIONS { stripProtectedImports: true, .. }, _) => txt.clone(),
        (txt, _, a_element) => {
            let mut txt = (*txt).clone();
            txt = dumpImport(txt.clone(), a_element.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_26(in_txt: Tpl::Text, in_a_visibility: SCode::Visibility, in_a_element: Arc<SCode::Element>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_visibility.clone(), in_a_element.clone(), in_a_options.clone())) {
        (txt, SCode::PROTECTED, a_element, a_options) => {
            let mut txt = (*txt).clone();
            txt = fun_25(txt.clone(), a_options.clone(), a_element.clone())?;
            txt.clone()
        },
        (txt, _, a_element, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut txt = (*txt).clone();
            txt = dumpImport(txt.clone(), a_element.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_29(in_txt: Tpl::Text, in_a_inPublicSection: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_inPublicSection.clone()) {
        (mut txt, false) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("public")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_30(in_txt: Tpl::Text, in_a_inPublicSection: bool) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_inPublicSection.clone()) {
        (mut txt, false) => txt.clone(),
        (mut txt, _) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("protected")).clone() }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_32(in_txt: Tpl::Text, in_a_imp: Absyn::Import) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_imp.clone()) {
        (mut txt, Absyn::NAMED_IMPORT { path: ref i_path, name: mut i_name }) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::QUAL_IMPORT { path: ref i_path }) => {
            let mut i_name: ArcStr;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt.clone()
        },
        (mut txt, Absyn::UNQUAL_IMPORT { path: ref i_path }) => {
            let mut i_name: ArcStr;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("import ")).clone() }))?;
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".*")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => {
            let mut i_path: Arc<Absyn::Path>;
            let mut i_name: ArcStr;
            txt = errorMsg(txt.clone(), (literal!("SCodeDump.dumpImport: Unknown import.")).clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_37(in_txt: Tpl::Text, in_a_options: SCodeDump::SCodeDumpOptions, in_a_p_normalAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_options.clone(), in_a_p_normalAlgorithmLst.clone())) {
        (txt, i_options @ SCodeDump::OPTIONS { stripAlgorithmSections: false, .. }, a_p_normalAlgorithmLst) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmSections(txt.clone(), a_p_normalAlgorithmLst.clone(), (literal!("algorithm")).clone(), i_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_38(in_txt: Tpl::Text, in_a_options: SCodeDump::SCodeDumpOptions, in_a_p_initialAlgorithmLst: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_options.clone(), in_a_p_initialAlgorithmLst.clone())) {
        (txt, i_options @ SCodeDump::OPTIONS { stripAlgorithmSections: false, .. }, a_p_initialAlgorithmLst) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmSections(txt.clone(), a_p_initialAlgorithmLst.clone(), (literal!("initial algorithm")).clone(), i_options.clone())?;
            txt.clone()
        },
        (txt, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_40(in_txt: Tpl::Text, in_a_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_enumLst.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => {
            let mut a_options: SCodeDump::SCodeDumpOptions;
            let mut i_enumLst: Arc<metamodelica::List<Arc<SCode::Enum>>>;
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(":")).clone() }))?;
            txt.clone()
        },
        (txt, i_enumLst, a_options) => {
            let mut txt = (*txt).clone();
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_39(txt.clone(), i_enumLst.clone(), a_options.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_44(in_txt: Tpl::Text, in_a_ann: ArcStr) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_ann.clone())) {
        (txt, Deref @ "") => txt.clone(),
        (txt, i_ann) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_ann.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_45(in_txt: Tpl::Text, in_a_annstr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_annstr.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_46(in_txt: Tpl::Text, in_a_cdefStr: ArcStr, in_a_cc__str: ArcStr, in_a_name: ArcStr, in_a_annstr: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_cdefStr.clone(), in_a_cc__str.clone(), in_a_name.clone(), in_a_annstr.clone())) {
        (txt, Deref @ "", a_cc__str, a_name, a_annstr) => {
            let mut i_cdefStr: ArcStr;
            let mut txt = (*txt).clone();
            txt = Tpl::writeText(txt.clone(), a_annstr.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" end ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cc__str.clone()).clone())?;
            txt.clone()
        },
        (txt, i_cdefStr, a_cc__str, a_name, a_annstr) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE))?;
            txt = Tpl::writeStr(txt.clone(), (i_cdefStr.clone()).clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = fun_45(txt.clone(), a_annstr.clone())?;
            txt = Tpl::writeText(txt.clone(), a_annstr.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("end ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (a_name.clone()).clone())?;
            txt = Tpl::writeStr(txt.clone(), (a_cc__str.clone()).clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_50(in_txt: Tpl::Text, in_a_attributes: SCode::Attributes, in_a_mod__str1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_attributes.clone(), in_a_mod__str1.clone()) {
        (mut txt, SCode::ATTR { direction: Absyn::OUTPUT, .. }, _) => txt.clone(),
        (mut txt, _, mut a_mod__str1) => {
            txt = Tpl::writeText(txt.clone(), a_mod__str1.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_51(in_txt: Tpl::Text, in_a_options: SCodeDump::SCodeDumpOptions, in_a_attributes: SCode::Attributes, in_a_mod__str1: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_options.clone(), in_a_attributes.clone(), in_a_mod__str1.clone()) {
        (mut txt, SCodeDump::OPTIONS { stripOutputBindings: false, .. }, _, mut a_mod__str1) => {
            let mut a_attributes: SCode::Attributes;
            txt = Tpl::writeText(txt.clone(), a_mod__str1.clone())?;
            txt.clone()
        },
        (mut txt, _, mut a_attributes, mut a_mod__str1) => {
            txt = fun_50(txt.clone(), a_attributes.clone(), a_mod__str1.clone())?;
            txt.clone()
        },
    });
    Ok(out_txt)
}

fn fun_52(in_txt: Tpl::Text, in_a_condition: Option<Arc<Absyn::Exp>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_condition.clone()) {
        (mut txt, Some(mut i_cond)) => {
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 1 }))?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("if ")).clone() }))?;
            txt = AbsynDumpTpl::dumpExp(txt.clone(), i_cond.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_54(in_txt: Tpl::Text, in_a_exp: Option<ArcStr>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_exp.clone()) {
        (mut txt, Some(mut i_e)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("exp = \\\"")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (i_e.clone()).clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("\\\"")).clone() }))?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_55(in_txt: Tpl::Text, in_a_weight: Option<f64>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (match (in_txt.clone(), in_a_weight.clone()) {
        (mut txt, Some(mut i_w)) => {
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("weight = ")).clone() }))?;
            txt = Tpl::writeStr(txt.clone(), (realString(i_w.clone())).clone())?;
            txt.clone()
        },
        (mut txt, _) => txt.clone(),
    });
    Ok(out_txt)
}

fn fun_58(in_txt: Tpl::Text, in_a_args__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_59(in_txt: Tpl::Text, in_a_args__str: Tpl::Text) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_args__str.clone())) {
        (txt, Tpl::MEM_TEXT { tokens: Deref @ metamodelica::List::Nil, .. }) => txt.clone(),
        (txt, _) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_67(in_txt: Tpl::Text, in_a_elseBranch: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_elseBranch.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, i_elseBranch, a_options) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("else\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            txt = lm_66(txt.clone(), i_elseBranch.clone(), a_options.clone())?;
            txt = Tpl::popIter(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

fn fun_70(in_txt: Tpl::Text, in_a_branches: Arc<metamodelica::List<Arc<metamodelica::List<Arc<SCode::Equation>>>>>, in_a_rest__conds: Arc<metamodelica::List<Arc<Absyn::Exp>>>, in_a_options: SCodeDump::SCodeDumpOptions, in_a_cond: Arc<Absyn::Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_a_branches.clone(), in_a_rest__conds.clone(), in_a_options.clone(), in_a_cond.clone())) {
        (txt, Deref @ metamodelica::List::Cons { head: i_branch, tail: i_rest__branches }, a_rest__conds, a_options, a_cond) => {
            let mut l_rest__str: Tpl::Text;
            let mut l_branch__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), a_cond.clone())?;
            l_branch__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_branch__str = lm_69(l_branch__str.clone(), i_branch.clone(), a_options.clone())?;
            l_branch__str = Tpl::popIter(l_branch__str.clone())?;
            l_rest__str = dumpElseIfEquation(Tpl::emptyTxt.clone(), a_rest__conds.clone(), i_rest__branches.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elseif ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_branch__str.clone())?;
            txt = Tpl::softNewLine(txt.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::writeText(txt.clone(), l_rest__str.clone())?;
            txt.clone()
        },
        (txt, _, _, _, _) => txt.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_107(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_tv, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_tv.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_107(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_111(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::SubMod>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_submod, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpSubModifier(txt.clone(), i_submod.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_111(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_114(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::SubMod>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_submod, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpAnnotationSubModifier(txt.clone(), i_submod.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_114(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_137(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_arg, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpExp(txt.clone(), i_arg.clone())?;
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
fn lm_19(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Element>>>, in_a_indent: bool, in_a_numElements: i32, in_a_inPublicSection: metamodelica::Array<bool>, in_a_options: SCodeDump::SCodeDumpOptions, in_a_prevSpacing: metamodelica::Array<bool>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_indent.clone(), in_a_numElements.clone(), in_a_inPublicSection.clone(), in_a_options.clone(), in_a_prevSpacing.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _, _, _, _, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_el, tail: rest }, a_indent, a_numElements, a_inPublicSection, a_options, a_prevSpacing) => {
            let mut x_i1: i32;
            let mut ret_10: bool;
            let mut ret_9: bool;
            let mut l_post__spacing: Tpl::Text;
            let mut l_dummyTxt: Tpl::Text;
            let mut ret_6: bool;
            let mut l_vis__str: Tpl::Text;
            let mut l_el__str: Tpl::Text;
            let mut ret_3: bool;
            let mut ret_2: bool;
            let mut l_pre__spacing: Tpl::Text;
            let mut l_spacing: Tpl::Text;
            let mut txt = (*txt).clone();
            x_i1 = Tpl::getIteri_i0(txt.clone())?;
            l_spacing = dumpElementSpacing(Tpl::emptyTxt.clone(), i_el.clone())?;
            ret_2 = intEq(1, x_i1.clone());
            ret_3 = boolNot(ret_2.clone());
            l_pre__spacing = fun_14(Tpl::emptyTxt.clone(), ret_3.clone(), a_prevSpacing.clone(), l_spacing.clone())?;
            l_el__str = dumpElement(Tpl::emptyTxt.clone(), i_el.clone(), (literal!("")).clone(), a_options.clone())?;
            ret_6 = Util::getStatefulBoolean(a_inPublicSection.clone());
            l_vis__str = dumpElementVisibility(Tpl::emptyTxt.clone(), i_el.clone(), ret_6.clone())?;
            l_dummyTxt = fun_15(Tpl::emptyTxt.clone(), l_vis__str.clone(), a_inPublicSection.clone())?;
            ret_9 = intEq(x_i1.clone(), a_numElements.clone());
            ret_10 = boolNot(ret_9.clone());
            l_post__spacing = fun_17(Tpl::emptyTxt.clone(), ret_10.clone(), l_spacing.clone(), a_prevSpacing.clone())?;
            txt = fun_18(txt.clone(), a_indent.clone(), l_post__spacing.clone(), l_el__str.clone(), l_vis__str.clone(), l_pre__spacing.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_19(txt.clone(), rest.clone(), a_indent.clone(), a_numElements.clone(), a_inPublicSection.clone(), a_options.clone(), a_prevSpacing.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_39(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Enum>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_enum, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEnumLiteral(txt.clone(), i_enum.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_39(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_41(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<ArcStr>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = Tpl::writeStr(txt.clone(), (i_it.clone()).clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_41(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_42(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone())) {
        (txt, Deref @ metamodelica::List::Nil) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_path, tail: rest }) => {
            let mut txt = (*txt).clone();
            txt = AbsynDumpTpl::dumpPath(txt.clone(), i_path.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_42(txt.clone(), rest.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_62(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_eq, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_eq.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_62(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_65(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_e.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_65(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_66(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_e.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_66(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_69(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_e.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_69(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_72(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_e.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_72(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_73(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_e.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_73(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_75(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_e.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_75(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_76(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Equation>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_e, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpEquation(txt.clone(), i_e.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_76(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_77(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Equation>>>)>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_else__cond, i_else__body), tail: rest }, a_options) => {
            let mut l_else__body__str: Tpl::Text;
            let mut l_else__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_else__cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_else__cond.clone())?;
            l_else__body__str = Tpl::pushIter(Tpl::emptyTxt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(openmodelica_susan::Tpl::StringToken::ST_NEW_LINE) }))?;
            l_else__body__str = lm_76(l_else__body__str.clone(), i_else__body.clone(), a_options.clone())?;
            l_else__body__str = Tpl::popIter(l_else__body__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elsewhen ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_else__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_else__body__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_77(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_80(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::AlgorithmSection>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_al, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpAlgorithmSection(txt.clone(), i_al.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_80(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_83(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<Arc<SCode::Statement>>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: i_s, tail: rest }, a_options) => {
            let mut txt = (*txt).clone();
            txt = dumpStatement(txt.clone(), i_s.clone(), a_options.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_83(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_87(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_cond, i_body), tail: rest }, a_options) => {
            let mut l_body__str: Tpl::Text;
            let mut l_cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_cond.clone())?;
            l_body__str = dumpStatements(Tpl::emptyTxt.clone(), i_body.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elseif ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_body__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_87(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_91(in_txt: Tpl::Text, in_items: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<SCode::Statement>>>)>>, in_a_options: SCodeDump::SCodeDumpOptions) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = (::match_deref::match_deref! { match &((in_txt.clone(), in_items.clone(), in_a_options.clone())) {
        (txt, Deref @ metamodelica::List::Nil, _) => txt.clone(),
        (txt, Deref @ metamodelica::List::Cons { head: (i_ew__cond, i_ew__body), tail: rest }, a_options) => {
            let mut l_ew__body__str: Tpl::Text;
            let mut l_ew__cond__str: Tpl::Text;
            let mut txt = (*txt).clone();
            l_ew__cond__str = AbsynDumpTpl::dumpExp(Tpl::emptyTxt.clone(), i_ew__cond.clone())?;
            l_ew__body__str = dumpStatements(Tpl::emptyTxt.clone(), i_ew__body.clone(), a_options.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("elsewhen ")).clone() }))?;
            txt = Tpl::writeText(txt.clone(), l_ew__cond__str.clone())?;
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(" then\\n")).clone() }))?;
            txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
            txt = Tpl::writeText(txt.clone(), l_ew__body__str.clone())?;
            txt = Tpl::popBlock(txt.clone())?;
            txt = Tpl::nextIter(txt.clone())?;
            txt = lm_91(txt.clone(), rest.clone(), a_options.clone())?;
            txt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_txt)
}

fn smf_56(in_txt: Tpl::Text, in_it: Tpl::Text) -> Result<Tpl::Text> {
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

fn smf_57(in_txt: Tpl::Text, in_it: Tpl::Text) -> Result<Tpl::Text> {
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

