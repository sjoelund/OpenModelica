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
use crate::SCode;
use crate::SCodeDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, PartialEq)]
pub struct SCodeDumpOptions {
    pub stripAlgorithmSections: bool,
    pub stripProtectedImports: bool,
    pub stripProtectedClasses: bool,
    pub stripProtectedComponents: bool,
    pub stripMetaRecords: bool,
    pub stripGraphicalAnnotations: bool,
    pub stripStringComments: bool,
    pub stripExternalDecl: bool,
    pub stripOutputBindings: bool,
}

pub type OPTIONS = SCodeDumpOptions;


pub fn classDefStr(cd: Arc<SCode::ClassDef>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpClassDef), cd.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn connectorTypeStr(inConnectorType: SCode::ConnectorType) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match inConnectorType.clone() {
        SCode::POTENTIAL => literal!(""),
        SCode::FLOW => literal!("flow"),
        SCode::STREAM => literal!("stream"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub static defaultOptions: SCodeDumpOptions = SCodeDumpOptions { stripAlgorithmSections: false, stripProtectedImports: false, stripProtectedClasses: false, stripProtectedComponents: false, stripMetaRecords: true, stripGraphicalAnnotations: true, stripStringComments: false, stripExternalDecl: false, stripOutputBindings: false };

pub fn eachStr(inEach: SCode::Each) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match inEach.clone() {
        SCode::EACH => literal!("each "),
        SCode::NOT_EACH => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn encapsulatedStr(inEncapsulated: SCode::Encapsulated) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match inEncapsulated.clone() {
        SCode::ENCAPSULATED => literal!("encapsulated "),
        SCode::NOT_ENCAPSULATED => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn equationStr(inEquation: Arc<SCode::Equation>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpEquation), inEquation.clone(), options.clone())?).clone();
    Ok(outString)
}

fn filterElement(element: Arc<SCode::Element>, options: SCodeDumpOptions) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &((element.clone(), options.clone())) {
        (Deref @ SCode::IMPORT { visibility: SCode::PROTECTED, .. }, OPTIONS { stripProtectedImports: true, .. }) => false,
        (Deref @ SCode::CLASS { prefixes: Deref @ SCode::PREFIXES { visibility: SCode::PROTECTED, .. }, .. }, OPTIONS { stripProtectedClasses: true, .. }) => false,
        (Deref @ SCode::COMPONENT { prefixes: Deref @ SCode::PREFIXES { visibility: SCode::PROTECTED, .. }, .. }, OPTIONS { stripProtectedComponents: true, .. }) => false,
        (Deref @ SCode::CLASS { restriction: SCode::R_METARECORD { moved: true, .. }, .. }, OPTIONS { stripMetaRecords: true, .. }) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn filterElements(elements: Arc<metamodelica::List<Arc<SCode::Element>>>, options: SCodeDumpOptions) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut outElements: Arc<metamodelica::List<Arc<SCode::Element>>>;
    outElements = List::select1(elements.clone(), Arc::new(fnptr!(filterElement, Arc<SCode::Element>, SCodeDumpOptions)), options.clone());
    outElements
}

pub fn finalStr(inFinal: SCode::Final) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match inFinal.clone() {
        SCode::FINAL => literal!("final "),
        SCode::NOT_FINAL => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn generateOptions(stripAlgorithmSections: bool, stripProtectedImports: bool, stripProtectedClasses: bool, stripProtectedComponents: bool, stripMetaRecords: bool, stripGraphicalAnnotations: bool, stripStringComments: bool, stripExternalDecl: bool, stripOutputBindings: bool) -> SCodeDumpOptions {
    let mut options: SCodeDumpOptions;
    options = SCodeDumpOptions { stripAlgorithmSections: stripAlgorithmSections.clone(), stripProtectedImports: stripProtectedImports.clone(), stripProtectedClasses: stripProtectedClasses.clone(), stripProtectedComponents: stripProtectedComponents.clone(), stripMetaRecords: stripMetaRecords.clone(), stripGraphicalAnnotations: stripGraphicalAnnotations.clone(), stripStringComments: stripStringComments.clone(), stripExternalDecl: stripExternalDecl.clone(), stripOutputBindings: stripOutputBindings.clone() };
    options
}

pub fn innerouterString(innerOuter: Absyn::InnerOuter) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match innerOuter.clone() {
        Absyn::INNER_OUTER => literal!("INNER/OUTER"),
        Absyn::INNER => literal!("INNER"),
        Absyn::OUTER => literal!("OUTER"),
        Absyn::NOT_INNER_OUTER => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub const noEachStr: &'static str = "";

pub fn parallelismString(inParallelism: SCode::Parallelism) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inParallelism.clone() {
        SCode::PARGLOBAL => literal!("PARGLOBAL"),
        SCode::PARLOCAL => literal!("PARLOCAL"),
        SCode::NON_PARALLEL => literal!("NON_PARALLEL"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn partialStr(inPartial: SCode::Partial) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match inPartial.clone() {
        SCode::PARTIAL => literal!("partial "),
        SCode::NOT_PARTIAL => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn prefixesStr(prefixes: Arc<SCode::Prefixes>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((::match_deref::match_deref! { match &(prefixes.clone()) {
        Deref @ SCode::PREFIXES { visibility: v, redeclarePrefix: rd, finalPrefix: f, innerOuter: io, replaceablePrefix: rpl } => {
            let mut s: ArcStr;
            s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*visibilityStr(v.clone())?); __mm_s.push_str(&*redeclareStr(rd.clone())?); __mm_s.push_str(&*finalStr(f.clone())?); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpl.clone())?); ArcStr::from(__mm_s) }).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn printAnnotationStr(inComment: Arc<SCode::Comment>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &((inComment.clone(), options.clone())) {
        (Deref @ SCode::COMMENT { annotation_, .. }, _) => Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpAnnotationOpt), annotation_.clone(), options.clone())?,
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn printCommentAndAnnotationStr(inComment: Arc<SCode::Comment>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpComment), inComment.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn printCommentStr(inComment: Arc<SCode::Comment>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inComment.clone()) {
        Deref @ SCode::COMMENT { comment, .. } => Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpCommentStr), comment.clone(), options.clone())?,
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn printEnumStr(en: Arc<SCode::Enum>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((::match_deref::match_deref! { match &(en.clone()) {
        Deref @ SCode::ENUM { literal: s, comment: _ } => s.clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn printInitialStr(initial_: SCode::Initial) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match initial_.clone() {
        SCode::INITIAL => literal!("initial"),
        SCode::NON_INITIAL => literal!("non initial"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn printModStr(inMod: Arc<SCode::Mod>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpModifier), inMod.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn programStr(inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpProgram), inProgram.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn redeclareStr(inRedeclare: SCode::Redeclare) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match inRedeclare.clone() {
        SCode::REDECLARE => literal!("redeclare "),
        SCode::NOT_REDECLARE => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn replaceableConstrainClassStr(inReplaceable: Arc<SCode::Replaceable>) -> Result<ArcStr> {
    let mut strReplaceable: ArcStr;
    (_, strReplaceable) = replaceableStr(inReplaceable.clone())?;
    Ok(strReplaceable)
}

pub fn replaceablePrefixStr(inReplaceable: Arc<SCode::Replaceable>) -> Result<ArcStr> {
    let mut strReplaceable: ArcStr;
    strReplaceable = ((::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::REPLACEABLE { cc: _ } => literal!("replaceable "),
        Deref @ SCode::NOT_REPLACEABLE => literal!(""),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(strReplaceable)
}

pub fn replaceableStr(inReplaceable: Arc<SCode::Replaceable>) -> Result<(ArcStr, ArcStr)> {
    let mut strReplaceable: ArcStr;
    let mut strConstraint: ArcStr;
    (strReplaceable, strConstraint) = (::match_deref::match_deref! { match &(inReplaceable.clone()) {
        Deref @ SCode::REPLACEABLE { cc: Some(SCode::CONSTRAINCLASS { modifier: r#mod, constrainingClass: path, .. }) } => {
            let mut path_str: ArcStr;
            let mut mod_str: ArcStr;
            path_str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            mod_str = (printModStr(r#mod.clone(), defaultOptions.clone())?).clone();
            (literal!("replaceable "), { let mut __mm_s = String::new(); __mm_s.push_str(&*path_str.clone()); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*mod_str.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })
        },
        Deref @ SCode::REPLACEABLE { cc: None } => (literal!("replaceable "), literal!("")),
        Deref @ SCode::NOT_REPLACEABLE => (literal!(""), literal!("")),
        _ => bail!("match: no arm matched"),
    } });
    Ok((strReplaceable, strConstraint))
}

pub fn restrString(inRestriction: SCode::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inRestriction.clone() {
        SCode::R_CLASS => literal!("class"),
        SCode::R_OPTIMIZATION => literal!("optimization"),
        SCode::R_MODEL => literal!("model"),
        SCode::R_RECORD { isOperator: false } => literal!("record"),
        SCode::R_RECORD { isOperator: true } => literal!("operator record"),
        SCode::R_BLOCK => literal!("block"),
        SCode::R_CONNECTOR { isExpandable: false } => literal!("connector"),
        SCode::R_CONNECTOR { isExpandable: true } => literal!("expandable connector"),
        SCode::R_OPERATOR => literal!("operator"),
        SCode::R_FUNCTION { .. } => (match var_field!(inRestriction.functionRestriction, SCode::Restriction::R_FUNCTION).clone() {
        SCode::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::PURE { .. } } => literal!("pure function"),
        SCode::FR_NORMAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => literal!("impure function"),
        SCode::FR_OPERATOR_FUNCTION => literal!("operator function"),
        SCode::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::PURE { .. } } => literal!("pure external function"),
        SCode::FR_EXTERNAL_FUNCTION { purity: Absyn::FunctionPurity::IMPURE { .. } } => literal!("impure external function"),
        SCode::FR_RECORD_CONSTRUCTOR => literal!("record constructor"),
        SCode::FR_PARALLEL_FUNCTION => literal!("parallel function"),
        SCode::FR_KERNEL_FUNCTION => literal!("kernel function"),
        _ => literal!("function"),
    }),
        SCode::R_TYPE => literal!("type"),
        SCode::R_PACKAGE => literal!("package"),
        SCode::R_ENUMERATION => literal!("enumeration"),
        SCode::R_METARECORD { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("metarecord ")); __mm_s.push_str(&*AbsynUtil::pathString(var_field!(inRestriction.name, SCode::Restriction::R_METARECORD).clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) },
        SCode::R_UNIONTYPE { .. } => literal!("uniontype"),
        SCode::R_PREDEFINED_INTEGER => literal!("Integer"),
        SCode::R_PREDEFINED_REAL => literal!("Real"),
        SCode::R_PREDEFINED_STRING => literal!("String"),
        SCode::R_PREDEFINED_BOOLEAN => literal!("Boolean"),
        SCode::R_PREDEFINED_CLOCK => literal!("Clock"),
        SCode::R_PREDEFINED_ENUMERATION => literal!("enumeration"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn restrictionStringPP(inRestriction: SCode::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString(Arc::new(SCodeDumpTpl::dumpRestriction), inRestriction.clone())?).clone();
    Ok(outString)
}

pub fn shortElementStr(inElement: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::EXTENDS { modifications: r#mod, baseClassPath: path, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut n: ArcStr;
            let mut ioStr: ArcStr;
            let mut imp: Absyn::Import;
            let mut io: Absyn::InnerOuter;
            let mut rdp: SCode::Redeclare;
            let mut rpp: Arc<SCode::Replaceable>;
            let mut pp: SCode::Partial;
            r#str = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*printModStr(r#mod.clone(), defaultOptions.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(literal!("extends ")).clone(), (r#str.clone()).clone(), (literal!(";")).clone()]);
            res.clone()
        },
        Deref @ SCode::COMPONENT { .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut n: ArcStr;
            let mut ioStr: ArcStr;
            let mut r#mod: Arc<SCode::Mod>;
            let mut path: Arc<Absyn::Path>;
            let mut imp: Absyn::Import;
            let mut io: Absyn::InnerOuter;
            let mut rdp: SCode::Redeclare;
            let mut rpp: Arc<SCode::Replaceable>;
            let mut pp: SCode::Partial;
            res = (unparseElementStr(inElement.clone(), defaultOptions.clone())?).clone();
            res.clone()
        },
        Deref @ SCode::CLASS { classDef: Deref @ SCode::DERIVED { .. }, prefixes: Deref @ SCode::PREFIXES { .. }, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut n: ArcStr;
            let mut ioStr: ArcStr;
            let mut r#mod: Arc<SCode::Mod>;
            let mut path: Arc<Absyn::Path>;
            let mut imp: Absyn::Import;
            let mut io: Absyn::InnerOuter;
            let mut rdp: SCode::Redeclare;
            let mut rpp: Arc<SCode::Replaceable>;
            let mut pp: SCode::Partial;
            res = (unparseElementStr(inElement.clone(), defaultOptions.clone())?).clone();
            res.clone()
        },
        Deref @ SCode::CLASS { classDef: Deref @ SCode::CLASS_EXTENDS { .. }, prefixes: Deref @ SCode::PREFIXES { replaceablePrefix: rpp, redeclarePrefix: rdp, innerOuter: io, .. }, partialPrefix: pp, name: n, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut ioStr: ArcStr;
            let mut r#mod: Arc<SCode::Mod>;
            let mut path: Arc<Absyn::Path>;
            let mut imp: Absyn::Import;
            ioStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*redeclareStr(rdp.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpp.clone())?); __mm_s.push_str(&*partialStr(pp.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(ioStr.clone()).clone(), (literal!("class extends ")).clone(), (n.clone()).clone(), (literal!(";")).clone()]);
            res.clone()
        },
        Deref @ SCode::CLASS { classDef: Deref @ SCode::ENUMERATION { .. }, prefixes: Deref @ SCode::PREFIXES { replaceablePrefix: rpp, redeclarePrefix: rdp, innerOuter: io, .. }, partialPrefix: pp, name: n, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut ioStr: ArcStr;
            let mut r#mod: Arc<SCode::Mod>;
            let mut path: Arc<Absyn::Path>;
            let mut imp: Absyn::Import;
            ioStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*redeclareStr(rdp.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpp.clone())?); __mm_s.push_str(&*partialStr(pp.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(ioStr.clone()).clone(), (literal!("class ")).clone(), (n.clone()).clone(), (literal!(" enumeration;")).clone()]);
            res.clone()
        },
        Deref @ SCode::CLASS { prefixes: Deref @ SCode::PREFIXES { replaceablePrefix: rpp, redeclarePrefix: rdp, innerOuter: io, .. }, partialPrefix: pp, name: n, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut ioStr: ArcStr;
            let mut r#mod: Arc<SCode::Mod>;
            let mut path: Arc<Absyn::Path>;
            let mut imp: Absyn::Import;
            ioStr = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseInnerOuterStr(io.clone())?); __mm_s.push_str(&*redeclareStr(rdp.clone())?); __mm_s.push_str(&*replaceablePrefixStr(rpp.clone())?); __mm_s.push_str(&*partialStr(pp.clone())?); ArcStr::from(__mm_s) }).clone();
            res = stringAppendList(list![(ioStr.clone()).clone(), (literal!("class ")).clone(), (n.clone()).clone(), (literal!(";")).clone()]);
            res.clone()
        },
        Deref @ SCode::IMPORT { imp, .. } => {
            let mut r#str: ArcStr;
            let mut res: ArcStr;
            let mut n: ArcStr;
            let mut ioStr: ArcStr;
            let mut r#mod: Arc<SCode::Mod>;
            let mut path: Arc<Absyn::Path>;
            let mut io: Absyn::InnerOuter;
            let mut rdp: SCode::Redeclare;
            let mut rpp: Arc<SCode::Replaceable>;
            let mut pp: SCode::Partial;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("import ")); __mm_s.push_str(&*AbsynUtil::printImportString(imp.clone())?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn statementStr(stmt: Arc<SCode::Statement>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString2(Arc::new(SCodeDumpTpl::dumpStatement), stmt.clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn unparseElementStr(inElement: Arc<SCode::Element>, options: SCodeDumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (Tpl::tplString3(Arc::new(SCodeDumpTpl::dumpElement), inElement.clone(), (arcstr::literal!(noEachStr)).clone(), options.clone())?).clone();
    Ok(outString)
}

pub fn unparseVariability(inVariability: SCode::Variability) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inVariability.clone() {
        SCode::VAR => literal!(""),
        SCode::DISCRETE => literal!("discrete"),
        SCode::PARAM => literal!("parameter"),
        SCode::CONST => literal!("constant"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn variabilityString(inVariability: SCode::Variability) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inVariability.clone() {
        SCode::VAR => literal!("VAR"),
        SCode::DISCRETE => literal!("DISCRETE"),
        SCode::PARAM => literal!("PARAM"),
        SCode::CONST => literal!("CONST"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn visibilityStr(inVisibility: SCode::Visibility) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match inVisibility.clone() {
        SCode::PUBLIC => literal!("public "),
        SCode::PROTECTED => literal!("protected "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

