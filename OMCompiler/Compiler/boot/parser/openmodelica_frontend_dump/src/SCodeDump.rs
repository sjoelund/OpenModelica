// Auto-generated from MetaModelica source
/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
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
use crate::SCodeDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::SCode;
use openmodelica_susan::Tpl;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SCodeDumpOptions {
    pub stripAlgorithmSections: bool,
    pub stripProtectedImports: bool,
    pub stripProtectedClasses: bool,
    pub stripProtectedComponents: bool,
    /// The automatically generated records that change scope from uniontype to the package
    pub stripMetaRecords: bool,
    pub stripGraphicalAnnotations: bool,
    pub stripStringComments: bool,
    pub stripExternalDecl: bool,
    pub stripOutputBindings: bool,
}

impl Default for SCodeDumpOptions {
    fn default() -> Self {
        Self {
            stripAlgorithmSections: Default::default(),
            stripProtectedImports: Default::default(),
            stripProtectedClasses: Default::default(),
            stripProtectedComponents: Default::default(),
            stripMetaRecords: Default::default(),
            stripGraphicalAnnotations: Default::default(),
            stripStringComments: Default::default(),
            stripExternalDecl: Default::default(),
            stripOutputBindings: Default::default(),
        }
    }
}

pub type OPTIONS = SCodeDumpOptions;


pub static defaultOptions: SCodeDumpOptions = SCodeDumpOptions { stripAlgorithmSections: false, stripProtectedImports: false, stripProtectedClasses: false, stripProtectedComponents: false, stripMetaRecords: true, stripGraphicalAnnotations: true, stripStringComments: false, stripExternalDecl: false, stripOutputBindings: false };

pub fn generateOptions(mut stripAlgorithmSections: bool, mut stripProtectedImports: bool, mut stripProtectedClasses: bool, mut stripProtectedComponents: bool, mut stripMetaRecords: bool, mut stripGraphicalAnnotations: bool, mut stripStringComments: bool, mut stripExternalDecl: bool, mut stripOutputBindings: bool) -> SCodeDumpOptions {
    let mut options: SCodeDumpOptions;
    options = SCodeDumpOptions { stripAlgorithmSections: stripAlgorithmSections.clone(), stripProtectedImports: stripProtectedImports.clone(), stripProtectedClasses: stripProtectedClasses.clone(), stripProtectedComponents: stripProtectedComponents.clone(), stripMetaRecords: stripMetaRecords.clone(), stripGraphicalAnnotations: stripGraphicalAnnotations.clone(), stripStringComments: stripStringComments.clone(), stripExternalDecl: stripExternalDecl.clone(), stripOutputBindings: stripOutputBindings.clone() };
    options
}

pub fn programStr(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpProgram), inProgram.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn classDefStr(mut cd: Arc<SCode::ClassDef>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpClassDef), cd.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn statementStr(mut stmt: Arc<SCode::Statement>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpStatement), stmt.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn equationStr(mut inEquation: Arc<SCode::Equation>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpEquation), inEquation.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn printModStr(mut inMod: Arc<SCode::Mod>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpModifier), inMod.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn printCommentAndAnnotationStr(mut inComment: Arc<SCode::Comment>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpComment), inComment.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn printCommentStr(mut inComment: Arc<SCode::Comment>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inComment.clone()) {
        Deref @ SCode::Comment { comment, .. } => {
            Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpCommentStr), comment.clone(), options.clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn printAnnotationStr(mut inComment: Arc<SCode::Comment>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inComment.clone(), options.clone())) {
        (Deref @ SCode::Comment { annotation_, .. }, _) => {
            Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpAnnotationOpt), annotation_.clone(), options.clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn restrString(mut inRestriction: SCode::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inRestriction.clone() {
        SCode::Restriction::R_CLASS => literal!("class"),
        SCode::Restriction::R_OPTIMIZATION => literal!("optimization"),
        SCode::Restriction::R_MODEL => literal!("model"),
        SCode::Restriction::R_RECORD { isOperator: false } => literal!("record"),
        SCode::Restriction::R_RECORD { isOperator: true } => literal!("operator record"),
        SCode::Restriction::R_BLOCK => literal!("block"),
        SCode::Restriction::R_CONNECTOR { isExpandable: false } => literal!("connector"),
        SCode::Restriction::R_CONNECTOR { isExpandable: true } => literal!("expandable connector"),
        SCode::Restriction::R_OPERATOR => literal!("operator"),
        SCode::Restriction::R_FUNCTION { .. } => (match var_field!(inRestriction.functionRestriction, SCode::Restriction::R_FUNCTION).clone() {
        SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::PURE { .. } } => literal!("pure function"),
        SCode::FunctionRestriction::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => literal!("impure function"),
        SCode::FunctionRestriction::FR_OPERATOR_FUNCTION => literal!("operator function"),
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::PURE { .. } } => literal!("pure external function"),
        SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => literal!("impure external function"),
        SCode::FunctionRestriction::FR_RECORD_CONSTRUCTOR => literal!("record constructor"),
        SCode::FunctionRestriction::FR_PARALLEL_FUNCTION => literal!("parallel function"),
        SCode::FunctionRestriction::FR_KERNEL_FUNCTION => literal!("kernel function"),
        _ => literal!("function"),
    }),
        SCode::Restriction::R_TYPE => literal!("type"),
        SCode::Restriction::R_PACKAGE => literal!("package"),
        SCode::Restriction::R_ENUMERATION => literal!("enumeration"),
        SCode::Restriction::R_METARECORD { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("metarecord ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!(inRestriction.name, SCode::Restriction::R_METARECORD).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) },
        SCode::Restriction::R_UNIONTYPE { .. } => literal!("uniontype"),
        SCode::Restriction::R_PREDEFINED_INTEGER => literal!("Integer"),
        SCode::Restriction::R_PREDEFINED_REAL => literal!("Real"),
        SCode::Restriction::R_PREDEFINED_STRING => literal!("String"),
        SCode::Restriction::R_PREDEFINED_BOOLEAN => literal!("Boolean"),
        SCode::Restriction::R_PREDEFINED_CLOCK => literal!("Clock"),
        SCode::Restriction::R_PREDEFINED_ENUMERATION => literal!("enumeration"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn restrictionStringPP(mut inRestriction: SCode::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(SCodeDumpTpl::dumpRestriction), inRestriction.clone())?).clone();
    Ok(outString)
}

pub const noEachStr: &'static str = "";

pub fn unparseElementStr(mut inElement: Arc<SCode::Element>, mut options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString3(Arc::new(SCodeDumpTpl::dumpElement), inElement.clone(), (arcstr::literal!(noEachStr)).clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn shortElementStr(mut inElement: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::EXTENDS { modifications: r#mod, baseClassPath: path, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            r#str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*printModStr(r#mod.clone(), defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(literal!("extends ")).clone(), (r#str.clone()).clone(), (literal!(";")).clone()]);
            res.clone()
        },
        Deref @ SCode::Element::COMPONENT { .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (unparseElementStr(inElement.clone(), defaultOptions.clone())?).clone();
            res.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::DERIVED { .. }, prefixes: Deref @ SCode::Prefixes { .. }, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (unparseElementStr(inElement.clone(), defaultOptions.clone())?).clone();
            res.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rpp, redeclarePrefix: rdp, innerOuter: io, .. }, partialPrefix: pp, name: n, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            let mut ioStr: ArcStr = arcstr::literal!("");
            ioStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*redeclareStr(rdp.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpp.clone())?); __mm_s.push_str(&*partialStr(pp.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(ioStr.clone()).clone(), (literal!("class extends ")).clone(), (n.clone()).clone(), (literal!(";")).clone()]);
            res.clone()
        },
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::ENUMERATION { .. }, prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rpp, redeclarePrefix: rdp, innerOuter: io, .. }, partialPrefix: pp, name: n, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            let mut ioStr: ArcStr = arcstr::literal!("");
            ioStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*redeclareStr(rdp.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpp.clone())?); __mm_s.push_str(&*partialStr(pp.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(ioStr.clone()).clone(), (literal!("class ")).clone(), (n.clone()).clone(), (literal!(" enumeration;")).clone()]);
            res.clone()
        },
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { replaceablePrefix: rpp, redeclarePrefix: rdp, innerOuter: io, .. }, partialPrefix: pp, name: n, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            let mut ioStr: ArcStr = arcstr::literal!("");
            ioStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*redeclareStr(rdp.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpp.clone())?); __mm_s.push_str(&*partialStr(pp.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(ioStr.clone()).clone(), (literal!("class ")).clone(), (n.clone()).clone(), (literal!(";")).clone()]);
            res.clone()
        },
        Deref @ SCode::Element::IMPORT { imp, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("import ")); __mm_s.push_str(&*AbsynUtil::printImportString(imp.clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printEnumStr(mut en: Arc<SCode::Enum>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(en.clone()) {
        Deref @ SCode::Enum { literal: s, comment: _ } => {
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn variabilityString(mut inVariability: SCode::Variability) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVariability.clone() {
        SCode::Variability::VAR => literal!("VAR"),
        SCode::Variability::DISCRETE => literal!("DISCRETE"),
        SCode::Variability::PARAM => literal!("PARAM"),
        SCode::Variability::CONST => literal!("CONST"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn parallelismString(mut inParallelism: SCode::Parallelism) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inParallelism.clone() {
        SCode::Parallelism::PARGLOBAL => literal!("PARGLOBAL"),
        SCode::Parallelism::PARLOCAL => literal!("PARLOCAL"),
        SCode::Parallelism::NON_PARALLEL => literal!("NON_PARALLEL"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn innerouterString(mut innerOuter: Absyn::InnerOuter) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match innerOuter.clone() {
        Absyn::InnerOuter::INNER_OUTER => literal!("INNER/OUTER"),
        Absyn::InnerOuter::INNER => literal!("INNER"),
        Absyn::InnerOuter::OUTER => literal!("OUTER"),
        Absyn::InnerOuter::NOT_INNER_OUTER => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseVariability(mut inVariability: SCode::Variability) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVariability.clone() {
        SCode::Variability::VAR => literal!(""),
        SCode::Variability::DISCRETE => literal!("discrete"),
        SCode::Variability::PARAM => literal!("parameter"),
        SCode::Variability::CONST => literal!("constant"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn printInitialStr(mut initial_: SCode::Initial) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match initial_.clone() {
        SCode::Initial::INITIAL => literal!("initial"),
        SCode::Initial::NON_INITIAL => literal!("non initial"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn connectorTypeStr(mut inConnectorType: SCode::ConnectorType) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inConnectorType.clone() {
        SCode::ConnectorType::POTENTIAL => literal!(""),
        SCode::ConnectorType::FLOW => literal!("flow"),
        SCode::ConnectorType::STREAM => literal!("stream"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn encapsulatedStr(mut inEncapsulated: SCode::Encapsulated) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inEncapsulated.clone() {
        SCode::Encapsulated::ENCAPSULATED => literal!("encapsulated "),
        SCode::Encapsulated::NOT_ENCAPSULATED => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn partialStr(mut inPartial: SCode::Partial) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inPartial.clone() {
        SCode::Partial::PARTIAL => literal!("partial "),
        SCode::Partial::NOT_PARTIAL => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn visibilityStr(mut inVisibility: SCode::Visibility) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inVisibility.clone() {
        SCode::Visibility::PUBLIC => literal!("public "),
        SCode::Visibility::PROTECTED => literal!("protected "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn finalStr(mut inFinal: SCode::Final) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inFinal.clone() {
        SCode::Final::FINAL => literal!("final "),
        SCode::Final::NOT_FINAL => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn eachStr(mut inEach: SCode::Each) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inEach.clone() {
        SCode::Each::EACH => literal!("each "),
        SCode::Each::NOT_EACH => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn redeclareStr(mut inRedeclare: SCode::Redeclare) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match inRedeclare.clone() {
        SCode::Redeclare::REDECLARE => literal!("redeclare "),
        SCode::Redeclare::NOT_REDECLARE => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn replaceableStr(mut inReplaceable: Arc<SCode::Replaceable>) -> Result<(ArcStr, ArcStr)> {
    let mut strReplaceable: ArcStr = arcstr::literal!("");
    let mut strConstraint: ArcStr = arcstr::literal!("");
    (strReplaceable, strConstraint) = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: Some(Deref @ SCode::ConstrainClass { modifier: r#mod, constrainingClass: path, .. }) } => {
            let mut path_str: ArcStr = arcstr::literal!("");
            let mut mod_str: ArcStr = arcstr::literal!("");
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            mod_str = (printModStr(r#mod.clone(), defaultOptions.clone())?).clone();
            (literal!("replaceable "), { let mut __mm_s = String::new(); __mm_s.push_str(&*path_str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*mod_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        },
        Deref @ SCode::Replaceable::REPLACEABLE { cc: None } => {
            (literal!("replaceable "), literal!(""))
        },
        Deref @ SCode::Replaceable::NOT_REPLACEABLE => {
            (literal!(""), literal!(""))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((strReplaceable, strConstraint))
}

pub fn replaceablePrefixStr(mut inReplaceable: Arc<SCode::Replaceable>) -> Result<ArcStr> {
    let mut strReplaceable: ArcStr = arcstr::literal!("");
    strReplaceable = ((::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::Replaceable::REPLACEABLE { cc: _ } => literal!("replaceable "),
        Deref @ SCode::Replaceable::NOT_REPLACEABLE => literal!(""),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(strReplaceable)
}

pub fn replaceableConstrainClassStr(mut inReplaceable: Arc<SCode::Replaceable>) -> Result<ArcStr> {
    let mut strReplaceable: ArcStr = arcstr::literal!("");
    (_, strReplaceable) = replaceableStr(inReplaceable.clone())?;
    Ok(strReplaceable)
}

pub fn prefixesStr(mut prefixes: Arc<SCode::Prefixes>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((::match_deref::match_deref! { match &(prefixes.clone()) {
        Deref @ SCode::Prefixes { visibility: v, redeclarePrefix: rd, finalPrefix: f, innerOuter: io, replaceablePrefix: rpl } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*visibilityStr(v.clone())?); __mm_s.push_str(&*redeclareStr(rd.clone())?); __mm_s.push_str(&*finalStr(f.clone())?); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpl.clone())?); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn filterElements(mut elements: Arc<metamodelica::List<Arc<SCode::Element>>>, mut options: SCodeDumpOptions) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    outElements = List::select1(elements.clone(), Arc::new(fnptr!(filterElement, Arc<SCode::Element>, SCodeDumpOptions)), options.clone());
    outElements
}

fn filterElement(mut element: Arc<SCode::Element>, mut options: SCodeDumpOptions) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((element.clone(), options.clone())) {
        (Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PROTECTED, .. }, SCodeDumpOptions { stripProtectedImports: true, .. }) => false,
        (Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED, .. }, .. }, SCodeDumpOptions { stripProtectedClasses: true, .. }) => false,
        (Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PROTECTED, .. }, .. }, SCodeDumpOptions { stripProtectedComponents: true, .. }) => false,
        (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { moved: true, .. }, .. }, SCodeDumpOptions { stripMetaRecords: true, .. }) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

