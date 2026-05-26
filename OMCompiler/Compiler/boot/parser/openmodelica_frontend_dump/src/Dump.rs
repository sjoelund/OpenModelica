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

use crate::AbsynDumpTpl;
use openmodelica_ast::Absyn;
use openmodelica_susan::Tpl;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::File::Escape;
use openmodelica_util::File;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Print;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DumpOptions {
    pub fileName: ArcStr,
}

impl Default for DumpOptions {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
        }
    }
}

pub type DUMPOPTIONS = DumpOptions;


pub static defaultDumpOptions: DumpOptions = DumpOptions { fileName: literal!("") };

pub fn boolUnparseFileFromInfo(mut info: SourceInfo, mut options: DumpOptions) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((options.clone(), info.clone())) {
        (DumpOptions { fileName: Deref @ "" }, _) => true,
        (DumpOptions { .. }, SourceInfo { .. }) => options.fileName.clone() == info.fileName.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(b)
}

pub fn unparseStr(mut inProgram: Absyn::Program, mut markup: bool, mut options: DumpOptions) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString2(Arc::new(AbsynDumpTpl::dump), inProgram.clone(), options.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseClassList(mut inClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString2(Arc::new(AbsynDumpTpl::dump), Absyn::Program { classes: inClasses.clone(), within_: openmodelica_ast::Absyn::Within::TOP }, defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseClassStr(mut inClass: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString3(Arc::new(AbsynDumpTpl::dumpClass), inClass.clone(), (literal!("")).clone(), defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseWithin(mut inWithin: Absyn::Within) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpWithin), inWithin.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseClassAttributesStr(mut inClass: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { restriction: r, encapsulatedPrefix: e, finalPrefix: f, partialPrefix: p, .. } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s2_1: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            s1 = (if (p.clone()) {literal!("partial ")} else {literal!("")}).clone();
            s2 = (if (f.clone()) {literal!("final ")} else {literal!("")}).clone();
            s2_1 = (if (e.clone()) {literal!("encapsulated ")} else {literal!("")}).clone();
            s3 = (unparseRestrictionStr(r.clone())?).clone();
            r#str = stringAppendList(list![(s2_1.clone()).clone(), (s1.clone()).clone(), (s2.clone()).clone(), (s3.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn unparseCommentOption(mut inComment: Option<Arc<Absyn::Comment>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpCommentOpt), inComment.clone())?).clone();
    Ok(outString)
}

pub fn unparseRestrictionStr(mut inRestriction: Absyn::Restriction) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpRestriction), inRestriction.clone())?).clone();
    Ok(outString)
}

pub fn unparseEachStr(mut inEach: Absyn::Each) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inEach.clone() {
        Absyn::Each::EACH => literal!("each "),
        Absyn::Each::NON_EACH => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseElementArgStr(mut inElementArg: Arc<Absyn::ElementArg>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpElementArg), inElementArg.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn shouldSeparateAfterElementArg(mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<(Arc<Absyn::ElementArg>, bool)>> {
    let mut outArgs: Arc<metamodelica::List<(Arc<Absyn::ElementArg>, bool)>> = metamodelica::nil();
    let mut numNonComment: i32 = 0;
    let mut cur: i32 = 0;
    let mut b: bool = false;
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        numNonComment = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => numNonComment.clone(),
        _ => numNonComment.clone() + 1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outArgs = metamodelica::nil();
    for mut arg in &*args.clone() {
        let mut arg = arg.clone();
        b = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. } => false,
        _ => {
            cur = cur.clone() + 1;
            cur.clone() < numNonComment.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outArgs = cons((arg.clone(), b.clone()), outArgs.clone());
    }
    outArgs = outArgs.clone().reverse();
    outArgs
}

pub fn unparseElementItemStr(mut inElementItem: Arc<Absyn::ElementItem>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString2(Arc::new(AbsynDumpTpl::dumpElementItem), inElementItem.clone(), defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseAnnotation(mut inAnnotation: Arc<Absyn::Annotation>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpAnnotation), inAnnotation.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseAnnotationOption(mut inAbsynAnnotation: Option<Arc<Absyn::Annotation>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inAbsynAnnotation.clone()) {
        Some(ann) => {
            unparseAnnotation(ann.clone())?
        },
        _ => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

pub fn unparseInnerOuterStr(mut inInnerOuter: Absyn::InnerOuter) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inInnerOuter.clone() {
        Absyn::InnerOuter::INNER => literal!("inner "),
        Absyn::InnerOuter::OUTER => literal!("outer "),
        Absyn::InnerOuter::INNER_OUTER => literal!("inner outer "),
        Absyn::InnerOuter::NOT_INNER_OUTER => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn unparseGroupImport(mut gimp: Absyn::GroupImport) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match gimp.clone() {
        Absyn::GroupImport::GROUP_IMPORT_NAME { name: mut name } => {
            name.clone()
        },
        Absyn::GroupImport::GROUP_IMPORT_RENAME { name: mut name, rename: mut rename } => {
            { let mut __mm_s = String::new(); __mm_s.push_str(&*rename.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn unparseImportStr(mut inImport: Absyn::Import) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpImport), inImport.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

fn unparseVariabilitySymbolStr(mut inVariability: Absyn::Variability) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inVariability.clone() {
        Absyn::Variability::VAR => literal!(""),
        Absyn::Variability::DISCRETE => literal!("discrete "),
        Absyn::Variability::PARAM => literal!("parameter "),
        Absyn::Variability::CONST => literal!("constant "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseDirectionSymbolStr(mut inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inDirection.clone() {
        Absyn::Direction::BIDIR => literal!(""),
        Absyn::Direction::INPUT => literal!("input "),
        Absyn::Direction::OUTPUT => literal!("output "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn directionSymbol(mut inDirection: Absyn::Direction) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inDirection.clone() {
        Absyn::Direction::BIDIR => literal!(""),
        Absyn::Direction::INPUT => literal!("input"),
        Absyn::Direction::OUTPUT => literal!("output"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseParallelismSymbolStr(mut inParallelism: Absyn::Parallelism) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inParallelism.clone() {
        Absyn::Parallelism::NON_PARALLEL => literal!(""),
        Absyn::Parallelism::PARGLOBAL => literal!("parglobal "),
        Absyn::Parallelism::PARLOCAL => literal!("parlocal "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn unparseComponentCondition(mut inComponentCondition: Option<Arc<Absyn::Exp>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpComponentCondition), inComponentCondition.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn printArraydimStr(mut s: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (printSubscriptsStr(s.clone())?).clone();
    Ok(r#str)
}

pub fn printSubscriptStr(mut inSubscript: Arc<Absyn::Subscript>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ Absyn::Subscript::NOSUB => {
            literal!(":")
        },
        Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e1 } => {
            let mut s: ArcStr = arcstr::literal!("");
            s = (printExpStr(e1.clone())?).clone();
            s.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn unparseModificationStr(mut inModification: Arc<Absyn::Modification>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpModification), inModification.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn equationName(mut eq: Arc<Absyn::Equation>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::Equation::EQ_IF { .. } => literal!("if"),
        Deref @ Absyn::Equation::EQ_EQUALS { .. } => literal!("equals"),
        Deref @ Absyn::Equation::EQ_PDE { .. } => literal!("pde"),
        Deref @ Absyn::Equation::EQ_CONNECT { .. } => literal!("connect"),
        Deref @ Absyn::Equation::EQ_WHEN_E { .. } => literal!("when"),
        Deref @ Absyn::Equation::EQ_NORETCALL { .. } => literal!("function call"),
        Deref @ Absyn::Equation::EQ_FAILURE { .. } => literal!("failure"),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(name)
}

pub fn unparseClassPart(mut classPart: Arc<Absyn::ClassPart>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString3(Arc::new(AbsynDumpTpl::dumpClassPart), classPart.clone(), 0, defaultDumpOptions.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseEquationStr(mut inEquation: Arc<Absyn::Equation>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpEquation), inEquation.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseEquationItemStr(mut inEquation: Arc<Absyn::EquationItem>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpEquationItem), inEquation.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn unparseEquationItemStrLst(mut inEquationItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut inSeparator: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = stringDelimitList(List::map(inEquationItems.clone(), Arc::new(unparseEquationItemStr)), (inSeparator.clone()).clone());
    outString
}

pub fn unparseAlgorithmStrLst(mut inAlgorithmItems: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut inSeparator: ArcStr) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = stringDelimitList(List::map(inAlgorithmItems.clone(), Arc::new(unparseAlgorithmStr)), (inSeparator.clone()).clone());
    outString
}

pub fn unparseAlgorithmStr(mut inAlgorithmItem: Arc<Absyn::AlgorithmItem>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpAlgorithmItem), inAlgorithmItem.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn printComponentRefStr(mut inComponentRef: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: subs, name: s } => {
            let mut subsstr: ArcStr = arcstr::literal!("");
            let mut s_1: ArcStr = arcstr::literal!("");
            subsstr = (printSubscriptsStr(subs.clone())?).clone();
            s_1 = (stringAppend((s.clone()).clone(), (subsstr.clone()).clone())).clone();
            s_1.clone()
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: cr, subscripts: subs, name: s } => {
            let mut subsstr: ArcStr = arcstr::literal!("");
            let mut s_1: ArcStr = arcstr::literal!("");
            let mut crs: ArcStr = arcstr::literal!("");
            let mut s_2: ArcStr = arcstr::literal!("");
            let mut s_3: ArcStr = arcstr::literal!("");
            crs = (printComponentRefStr(cr.clone())?).clone();
            subsstr = (printSubscriptsStr(subs.clone())?).clone();
            s_1 = (stringAppend((s.clone()).clone(), (subsstr.clone()).clone())).clone();
            s_2 = (stringAppend((s_1.clone()).clone(), (literal!(".")).clone())).clone();
            s_3 = (stringAppend((s_2.clone()).clone(), (crs.clone()).clone())).clone();
            s_3.clone()
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
            let mut crs: ArcStr = arcstr::literal!("");
            let mut s_3: ArcStr = arcstr::literal!("");
            crs = (printComponentRefStr(cr.clone())?).clone();
            s_3 = (stringAppend((literal!(".")).clone(), (crs.clone()).clone())).clone();
            s_3.clone()
        },
        Deref @ Absyn::ComponentRef::ALLWILD => {
            literal!("__")
        },
        Deref @ Absyn::ComponentRef::WILD => {
            if (Config::acceptMetaModelicaGrammar()?) {literal!("_")} else {literal!("")}
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printSubscriptsStr(mut inAbsynSubscriptLst: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inAbsynSubscriptLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                l => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s = (printListStr(l.clone(), Arc::new(printSubscriptStr), (literal!(",")).clone())?).clone();
                    s_1 = (stringAppend((literal!("[")).clone(), (s.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (literal!("]")).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn printFunctionArgsStr(mut inFunctionArgs: Arc<Absyn::FunctionArgs>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inFunctionArgs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs @ Deref @ metamodelica::List::Cons { head: _, tail: _ }, args: expargs @ Deref @ metamodelica::List::Cons { head: _, tail: _ } } => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut s3: ArcStr = arcstr::literal!("");
                    let mut r#str: ArcStr = arcstr::literal!("");
                    s1 = (printListStr(expargs.clone(), Arc::new(printExpStr), (literal!(", ")).clone())?).clone();
                    s2 = (stringAppend((s1.clone()).clone(), (literal!(", ")).clone())).clone();
                    s3 = (printListStr(nargs.clone(), Arc::new(printNamedArgStr), (literal!(", ")).clone())?).clone();
                    r#str = (stringAppend((s2.clone()).clone(), (s3.clone()).clone())).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs, args: Deref @ metamodelica::List::Nil } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (printListStr(nargs.clone(), Arc::new(printNamedArgStr), (literal!(", ")).clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: expargs } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (printListStr(expargs.clone(), Arc::new(printExpStr), (literal!(", ")).clone())?).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { iterators, exp, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut estr: ArcStr = arcstr::literal!("");
                    let mut istr: ArcStr = arcstr::literal!("");
                    estr = (printExpStr(exp.clone())?).clone();
                    istr = (printIteratorsStr(iterators.clone())?).clone();
                    r#str = stringAppendList(list![(estr.clone()).clone(), (literal!(" for ")).clone(), (istr.clone()).clone()]);
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn printIteratorsStr(mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>) -> Result<ArcStr> {
    let mut iteratorsStr: ArcStr = arcstr::literal!("");
    iteratorsStr = ('mc: {
        let __mc_input = iterators.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: Some(guardExp), range: Some(exp) }, tail: Deref @ metamodelica::List::Nil } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (printExpStr(exp.clone())?).clone();
                    s2 = (printExpStr(guardExp.clone())?).clone();
                    s = stringAppendList(list![(id.clone()).clone(), (literal!(" guard ")).clone(), (s2.clone()).clone(), (literal!(" in ")).clone(), (s1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: Some(exp) }, tail: Deref @ metamodelica::List::Nil } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (printExpStr(exp.clone())?).clone();
                    s = stringAppendList(list![(id.clone()).clone(), (literal!(" in ")).clone(), (s1.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ForIterator { name: id, guardExp: None, range: None }, tail: Deref @ metamodelica::List::Nil } => {
                    Ok(id.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: x, tail: rest } => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    s1 = (printIteratorsStr(list![x.clone()])?).clone();
                    s2 = (printIteratorsStr(rest.clone())?).clone();
                    s = stringAppendList(list![(s1.clone()).clone(), (literal!(", ")).clone(), (s2.clone()).clone()]);
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(iteratorsStr)
}

pub fn printNamedArgStr(mut inNamedArg: Arc<Absyn::NamedArg>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inNamedArg.clone()) {
        Deref @ Absyn::NamedArg { argValue: e, argName: ident } => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            s1 = (stringAppend((ident.clone()).clone(), (literal!(" = ")).clone())).clone();
            s2 = (printExpStr(e.clone())?).clone();
            r#str = (stringAppend((s1.clone()).clone(), (s2.clone()).clone())).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printNamedArgValueStr(mut inNamedArg: Arc<Absyn::NamedArg>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inNamedArg.clone()) {
        Deref @ Absyn::NamedArg { argValue: e, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = (printExpStr(e.clone())?).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn shouldParenthesize(mut inOperand: Arc<Absyn::Exp>, mut inOperator: Arc<Absyn::Exp>, mut inLhs: bool) -> Result<bool> {
    let mut outShouldParenthesize: bool = false;
    outShouldParenthesize = (::match_deref::match_deref! { match &(inOperand.clone()) {
        Deref @ Absyn::Exp::UNARY { .. } => {
            true
        },
        _ => {
            let mut diff: i32 = 0;
            diff = Util::intCompare(expPriority(inOperand.clone(), inLhs.clone())?, expPriority(inOperator.clone(), inLhs.clone())?);
            shouldParenthesize2(diff.clone(), inOperand.clone(), inLhs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outShouldParenthesize)
}

fn shouldParenthesize2(mut inPrioDiff: i32, mut inOperand: Arc<Absyn::Exp>, mut inLhs: bool) -> bool {
    let mut outShouldParenthesize: bool = false;
    outShouldParenthesize = (match inPrioDiff.clone() {
        1 => true,
        0 => if (inLhs.clone()) {isNonAssociativeExp(inOperand.clone())} else {!(isAssociativeExp(inOperand.clone()))},
        _ => false,
    });
    outShouldParenthesize
}

fn isAssociativeExp(mut inExp: Arc<Absyn::Exp>) -> bool {
    let mut outIsAssociative: bool = false;
    outIsAssociative = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::BINARY { op, .. } => {
            isAssociativeOp(op.clone())
        },
        Deref @ Absyn::Exp::LBINARY { .. } => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsAssociative
}

fn isAssociativeOp(mut inOperator: Absyn::Operator) -> bool {
    let mut outIsAssociative: bool = false;
    outIsAssociative = (match inOperator.clone() {
        Absyn::Operator::ADD => true,
        Absyn::Operator::ADD_EW => true,
        Absyn::Operator::MUL_EW => true,
        _ => false,
    });
    outIsAssociative
}

fn isNonAssociativeExp(mut exp: Arc<Absyn::Exp>) -> bool {
    let mut isNonAssociative: bool = false;
    isNonAssociative = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::BINARY { .. } => isNonAssociativeOp(var_field!((*exp).op, Absyn::Exp::BINARY).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isNonAssociative
}

fn isNonAssociativeOp(mut operator: Absyn::Operator) -> bool {
    let mut isNonAssociative: bool = false;
    isNonAssociative = (match operator.clone() {
        Absyn::Operator::POW => true,
        Absyn::Operator::POW_EW => true,
        _ => false,
    });
    isNonAssociative
}

pub fn expPriority(mut inExp: Arc<Absyn::Exp>, mut inLhs: bool) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (::match_deref::match_deref! { match &((inExp.clone(), inLhs.clone())) {
        (Deref @ Absyn::Exp::BINARY { op, .. }, false) => {
            priorityBinopRhs(op.clone())?
        },
        (Deref @ Absyn::Exp::BINARY { op, .. }, true) => {
            priorityBinopLhs(op.clone())?
        },
        (Deref @ Absyn::Exp::UNARY { .. }, _) => {
            4
        },
        (Deref @ Absyn::Exp::LBINARY { op, .. }, _) => {
            priorityLBinop(op.clone())?
        },
        (Deref @ Absyn::Exp::LUNARY { .. }, _) => {
            7
        },
        (Deref @ Absyn::Exp::RELATION { .. }, _) => {
            6
        },
        (Deref @ Absyn::Exp::RANGE { .. }, _) => {
            10
        },
        (Deref @ Absyn::Exp::IFEXP { .. }, _) => {
            11
        },
        _ => {
            0
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPriority)
}

fn priorityBinopLhs(mut inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (match inOp.clone() {
        Absyn::Operator::ADD => 5,
        Absyn::Operator::SUB => 5,
        Absyn::Operator::MUL => 2,
        Absyn::Operator::DIV => 2,
        Absyn::Operator::POW => 1,
        Absyn::Operator::ADD_EW => 5,
        Absyn::Operator::SUB_EW => 5,
        Absyn::Operator::MUL_EW => 2,
        Absyn::Operator::DIV_EW => 2,
        Absyn::Operator::POW_EW => 1,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityBinopRhs(mut inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (match inOp.clone() {
        Absyn::Operator::ADD => 6,
        Absyn::Operator::SUB => 5,
        Absyn::Operator::MUL => 2,
        Absyn::Operator::DIV => 2,
        Absyn::Operator::POW => 1,
        Absyn::Operator::ADD_EW => 6,
        Absyn::Operator::SUB_EW => 5,
        Absyn::Operator::MUL_EW => 3,
        Absyn::Operator::DIV_EW => 2,
        Absyn::Operator::POW_EW => 1,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

fn priorityLBinop(mut inOp: Absyn::Operator) -> Result<i32> {
    let mut outPriority: i32 = 0;
    outPriority = (match inOp.clone() {
        Absyn::Operator::AND => 8,
        Absyn::Operator::OR => 9,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPriority)
}

fn printOperandStr(mut inOperand: Arc<Absyn::Exp>, mut inOperation: Arc<Absyn::Exp>, mut inLhs: bool) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inOperand.clone(), inOperation.clone(), inLhs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let mut op_str: ArcStr = arcstr::literal!("");
                    let true = (shouldParenthesize(inOperand.clone(), inOperation.clone(), inLhs.clone())?) else { bail!("pattern mismatch") };
                    op_str = (printExpStr(inOperand.clone())?).clone();
                    op_str = stringAppendList(list![(literal!("(")).clone(), (op_str.clone()).clone(), (literal!(")")).clone()]);
                    Ok(op_str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(printExpStr(inOperand.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn printExpLstStr(mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = stringDelimitList(List::map(expl.clone(), Arc::new(printExpStr)), (literal!(", ")).clone());
    outString
}

pub fn printExpStr(mut inExp: Arc<Absyn::Exp>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpExp), inExp.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn printCodeStr(mut inCode: Arc<Absyn::CodeNode>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpCodeNode), inCode.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

fn printListStr<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>, mut inString: ArcStr) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone> = fn(Type_a) -> Result<ArcStr>;

    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inTypeALst.clone(), inFuncTypeTypeAToString.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, r, _) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    s = r(h.clone())?;
                    Ok(s.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, r, sep) => {
                    let mut s: ArcStr = arcstr::literal!("");
                    let mut srest: ArcStr = arcstr::literal!("");
                    let mut s_1: ArcStr = arcstr::literal!("");
                    let mut s_2: ArcStr = arcstr::literal!("");
                    s = r(h.clone())?;
                    srest = (printListStr(t.clone(), r.clone(), (sep.clone()).clone())?).clone();
                    s_1 = (stringAppend((s.clone()).clone(), (sep.clone()).clone())).clone();
                    s_2 = (stringAppend((s_1.clone()).clone(), (srest.clone()).clone())).clone();
                    Ok(s_2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn opSymbol(mut inOperator: Absyn::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        Absyn::Operator::ADD => literal!(" + "),
        Absyn::Operator::SUB => literal!(" - "),
        Absyn::Operator::MUL => literal!(" * "),
        Absyn::Operator::DIV => literal!(" / "),
        Absyn::Operator::POW => literal!(" ^ "),
        Absyn::Operator::UMINUS => literal!("-"),
        Absyn::Operator::UPLUS => literal!("+"),
        Absyn::Operator::ADD_EW => literal!(" .+ "),
        Absyn::Operator::SUB_EW => literal!(" .- "),
        Absyn::Operator::MUL_EW => literal!(" .* "),
        Absyn::Operator::DIV_EW => literal!(" ./ "),
        Absyn::Operator::POW_EW => literal!(" .^ "),
        Absyn::Operator::UMINUS_EW => literal!(" .-"),
        Absyn::Operator::UPLUS_EW => literal!(" .+"),
        Absyn::Operator::AND => literal!(" and "),
        Absyn::Operator::OR => literal!(" or "),
        Absyn::Operator::NOT => literal!("not "),
        Absyn::Operator::LESS => literal!(" < "),
        Absyn::Operator::LESSEQ => literal!(" <= "),
        Absyn::Operator::GREATER => literal!(" > "),
        Absyn::Operator::GREATEREQ => literal!(" >= "),
        Absyn::Operator::EQUAL => literal!(" == "),
        Absyn::Operator::NEQUAL => literal!(" <> "),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn opSymbolCompact(mut inOperator: Absyn::Operator) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inOperator.clone() {
        Absyn::Operator::ADD => literal!("+"),
        Absyn::Operator::SUB => literal!("-"),
        Absyn::Operator::MUL => literal!("*"),
        Absyn::Operator::DIV => literal!("/"),
        Absyn::Operator::POW => literal!("^"),
        Absyn::Operator::UMINUS => literal!("-"),
        Absyn::Operator::UPLUS => literal!("+"),
        Absyn::Operator::ADD_EW => literal!("+"),
        Absyn::Operator::SUB_EW => literal!("-"),
        Absyn::Operator::MUL_EW => literal!("*"),
        Absyn::Operator::DIV_EW => literal!("/"),
        Absyn::Operator::POW_EW => literal!("^"),
        Absyn::Operator::UMINUS_EW => literal!("-"),
        Absyn::Operator::AND => literal!("and"),
        Absyn::Operator::OR => literal!("or"),
        Absyn::Operator::NOT => literal!("not"),
        Absyn::Operator::LESS => literal!("<"),
        Absyn::Operator::LESSEQ => literal!("<="),
        Absyn::Operator::GREATER => literal!(">"),
        Absyn::Operator::GREATEREQ => literal!(">="),
        Absyn::Operator::EQUAL => literal!("=="),
        Absyn::Operator::NEQUAL => literal!("<>"),
        _ => bail!("fail"),
    })).clone();
    Ok(outString)
}

/*
 *
 * Utility functions
 * These are utility functions used in some of the other functions.
 *
 */
pub fn printOption<Type_a: Clone + 'static>(mut inTypeAOption: Option<Type_a>, mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    let _ = (match inTypeAOption.clone() {
        None => {
            Print::printBuf((literal!("NONE()")).clone())?;
            ()
        },
        Some(mut x) => {
            Print::printBuf((literal!("SOME(")).clone())?;
            inFuncTypeTypeATo(x.clone())?;
            Print::printBuf((literal!(")")).clone())?;
            ()
        },
    });
    Ok(())
}

pub fn printList<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, mut inString: ArcStr) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    let _ = 'mc: {
        let __mc_input = (inTypeALst.clone(), inFuncTypeTypeATo.clone(), inString.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: Deref @ metamodelica::List::Nil }, r, _) => {
                    r(h.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: h, tail: t }, r, sep) => {
                    r(h.clone())?;
                    Print::printBuf((sep.clone()).clone())?;
                    printList(t.clone(), r.clone(), (sep.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printStringCommentOption(mut inStringOption: Option<ArcStr>) -> Result<()> {
    let () = (match inStringOption.clone() {
        None => {
            Print::printBuf((literal!("NONE()")).clone())?;
            ()
        },
        Some(mut s) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = stringAppendList(list![(literal!("SOME(\"")).clone(), (s.clone()).clone(), (literal!("\")")).clone()]);
            Print::printBuf((r#str.clone()).clone())?;
            ()
        },
    });
    Ok(())
}

pub fn unparseTypeSpec(mut inTypeSpec: Arc<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut status: bool = false;
    status = Flags::getConfigBool(Flags::MODELICA_OUTPUT.clone())?;
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), false)?;
    outString = (Tpl::tplString(Arc::new(AbsynDumpTpl::dumpTypeSpec), inTypeSpec.clone())?).clone();
    FlagsUtil::setConfigBool(Flags::MODELICA_OUTPUT.clone(), status.clone())?;
    Ok(outString)
}

pub fn printTypeSpec(mut typeSpec: Arc<Absyn::TypeSpec>) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (unparseTypeSpec(typeSpec.clone())?).clone();
    println!("{}", (r#str.clone()).clone());
    Ok(())
}

pub fn stdout() -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (Print::getString()?).clone();
    println!("{}", (r#str.clone()).clone());
    Print::clearBuf();
    Ok(())
}

pub fn getAstAsCorbaString(mut program: Absyn::Program) -> Result<()> {
    let _ = (match program.clone() {
        Absyn::Program { within_: mut within_, classes: mut classes } => {
            Print::printBuf((literal!("record Absyn.PROGRAM\nclasses = ")).clone())?;
            printListAsCorbaString(classes.clone(), Arc::new(printClassAsCorbaString), (literal!(",\n")).clone())?;
            Print::printBuf((literal!(",\nwithin_ = ")).clone())?;
            printWithinAsCorbaString(within_.clone())?;
            Print::printBuf((literal!("\nend Absyn.PROGRAM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printPathAsCorbaString(mut inPath: Arc<Absyn::Path>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::QUALIFIED { path: p, name: s } => {
            Print::printBuf((literal!("record Absyn.QUALIFIED name = \"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\", path = ")).clone())?;
            printPathAsCorbaString(p.clone())?;
            Print::printBuf((literal!(" end Absyn.QUALIFIED;")).clone())?;
            ()
        },
        Deref @ Absyn::Path::IDENT { name: s } => {
            Print::printBuf((literal!("record Absyn.IDENT name = \"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\" end Absyn.IDENT;")).clone())?;
            ()
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path: p } => {
            Print::printBuf((literal!("record Absyn.FULLYQUALIFIED path = \"")).clone())?;
            printPathAsCorbaString(p.clone())?;
            Print::printBuf((literal!("\" end Absyn.FULLYQUALIFIED;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printComponentRefAsCorbaString(mut cref: Arc<Absyn::ComponentRef>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_QUAL { componentRef: p, subscripts, name: s } => {
            Print::printBuf((literal!("record Absyn.CREF_QUAL name = \"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\", subscripts = ")).clone())?;
            printListAsCorbaString(subscripts.clone(), Arc::new(printSubscriptAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", componentRef = ")).clone())?;
            printComponentRefAsCorbaString(p.clone())?;
            Print::printBuf((literal!(" end Absyn.CREF_QUAL;")).clone())?;
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts, name: s } => {
            Print::printBuf((literal!("record Absyn.CREF_IDENT name = \"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\", subscripts = ")).clone())?;
            printListAsCorbaString(subscripts.clone(), Arc::new(printSubscriptAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.CREF_IDENT;")).clone())?;
            ()
        },
        Deref @ Absyn::ComponentRef::ALLWILD => {
            Print::printBuf((literal!("record Absyn.ALLWILD end Absyn.ALLWILD;")).clone())?;
            ()
        },
        Deref @ Absyn::ComponentRef::WILD => {
            Print::printBuf((literal!("record Absyn.WILD end Absyn.WILD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printWithinAsCorbaString(mut within_: Absyn::Within) -> Result<()> {
    let _ = (match within_.clone() {
        Absyn::Within::WITHIN { path: mut path } => {
            Print::printBuf((literal!("record Absyn.WITHIN path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.WITHIN;")).clone())?;
            ()
        },
        Absyn::Within::TOP => {
            Print::printBuf((literal!("record Absyn.TOP end Absyn.TOP;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printClassAsCorbaString(mut cl: Arc<Absyn::Class>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { name, partialPrefix, finalPrefix, encapsulatedPrefix, restriction, body, commentsBeforeClass: _, commentsBeforeEnd: _, commentsAfterEnd: _, info } => {
            Print::printBuf((literal!("record Absyn.CLASS name = \"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\", partialPrefix = ")).clone())?;
            Print::printBuf((boolString(partialPrefix.clone())).clone())?;
            Print::printBuf((literal!(", finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(", encapsulatedPrefix = ")).clone())?;
            Print::printBuf((boolString(encapsulatedPrefix.clone())).clone())?;
            Print::printBuf((literal!(", restriction = ")).clone())?;
            printRestrictionAsCorbaString(restriction.clone())?;
            Print::printBuf((literal!(", body = ")).clone())?;
            printClassDefAsCorbaString(body.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.CLASS;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printClassAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printInfoAsCorbaString(mut info: SourceInfo) -> Result<()> {
    let _ = (match info.clone() {
        SourceInfo { fileName: mut fileName, isReadOnly: mut isReadOnly, lineNumberStart: mut lineNumberStart, columnNumberStart: mut columnNumberStart, lineNumberEnd: mut lineNumberEnd, columnNumberEnd: mut columnNumberEnd, lastModification: mut lastModified } => {
            Print::printBuf((literal!("record SOURCEINFO fileName = \"")).clone())?;
            Print::printBuf((fileName.clone()).clone())?;
            Print::printBuf((literal!("\", isReadOnly = ")).clone())?;
            Print::printBuf((boolString(isReadOnly.clone())).clone())?;
            Print::printBuf((literal!(", lineNumberStart = ")).clone())?;
            Print::printBuf((intString(lineNumberStart.clone())).clone())?;
            Print::printBuf((literal!(", columnNumberStart = ")).clone())?;
            Print::printBuf((intString(columnNumberStart.clone())).clone())?;
            Print::printBuf((literal!(", lineNumberEnd = ")).clone())?;
            Print::printBuf((intString(lineNumberEnd.clone())).clone())?;
            Print::printBuf((literal!(", columnNumberEnd = ")).clone())?;
            Print::printBuf((intString(columnNumberEnd.clone())).clone())?;
            Print::printBuf((literal!(", lastModified = ")).clone())?;
            Print::printBuf((realString(lastModified.clone())).clone())?;
            Print::printBuf((literal!(" end SOURCEINFO;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printInfoAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
    });
    Ok(())
}

fn printClassDefAsCorbaString(mut classDef: Arc<Absyn::ClassDef>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs: _, classParts, ann, comment: optString } => {
            Print::printBuf((literal!("record Absyn.PARTS typeVars = {")).clone())?;
            Print::printBuf(stringDelimitList(typeVars.clone(), (literal!(",")).clone()))?;
            Print::printBuf((literal!("}, classParts = ")).clone())?;
            printListAsCorbaString(classParts.clone(), Arc::new(printClassPartAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", ann = ")).clone())?;
            printListAsCorbaString(ann.clone(), Arc::new(printAnnotationAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(optString.clone())?;
            Print::printBuf((literal!(" end Absyn.PARTS;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { typeSpec, attributes, arguments, comment } => {
            Print::printBuf((literal!("record Absyn.DERIVED typeSpec = ")).clone())?;
            printTypeSpecAsCorbaString(typeSpec.clone())?;
            Print::printBuf((literal!(", attributes = ")).clone())?;
            printElementAttributesAsCorbaString(attributes.clone())?;
            Print::printBuf((literal!(", arguments = ")).clone())?;
            printListAsCorbaString(arguments.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.DERIVED;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassDef::ENUMERATION { enumLiterals, comment } => {
            Print::printBuf((literal!("record Absyn.ENUMERATION enumLiterals = ")).clone())?;
            printEnumDefAsCorbaString(enumLiterals.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.ENUMERATION;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassDef::OVERLOAD { functionNames, comment } => {
            Print::printBuf((literal!("record Absyn.OVERLOAD functionNames = ")).clone())?;
            printListAsCorbaString(functionNames.clone(), Arc::new(printPathAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.OVERLOAD;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName, modifications, comment: optString, parts: classParts, ann } => {
            Print::printBuf((literal!("record Absyn.CLASS_EXTENDS baseClassName = \"")).clone())?;
            Print::printBuf((baseClassName.clone()).clone())?;
            Print::printBuf((literal!("\", modifications = ")).clone())?;
            printListAsCorbaString(modifications.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(optString.clone())?;
            Print::printBuf((literal!(", parts = ")).clone())?;
            printListAsCorbaString(classParts.clone(), Arc::new(printClassPartAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", ann = ")).clone())?;
            printListAsCorbaString(ann.clone(), Arc::new(printAnnotationAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!("end Absyn.CLASS_EXTENDS;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassDef::PDER { functionName, vars, comment } => {
            Print::printBuf((literal!("record Absyn.PDER functionName = ")).clone())?;
            printPathAsCorbaString(functionName.clone())?;
            Print::printBuf((literal!(", vars = ")).clone())?;
            printListAsCorbaString(vars.clone(), Arc::new(printStringAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.PDER;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printClassDefAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printEnumDefAsCorbaString(mut enumDef: Arc<Absyn::EnumDef>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(enumDef.clone()) {
        Deref @ Absyn::EnumDef::ENUMLITERALS { enumLiterals } => {
            Print::printBuf((literal!("record Absyn.ENUMLITERALS enumLiterals = ")).clone())?;
            printListAsCorbaString(enumLiterals.clone(), Arc::new(printEnumLiteralAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!("end Absyn.ENUMLITERALS;")).clone())?;
            ()
        },
        Deref @ Absyn::EnumDef::ENUM_COLON => {
            Print::printBuf((literal!("record Absyn.ENUM_COLON end Absyn.ENUM_COLON;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printEnumDefAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printEnumLiteralAsCorbaString(mut enumLit: Arc<Absyn::EnumLiteral>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(enumLit.clone()) {
        Deref @ Absyn::EnumLiteral { literal, comment } => {
            Print::printBuf((literal!("record Absyn.ENUMLITERAL literal = \"")).clone())?;
            Print::printBuf((literal.clone()).clone())?;
            Print::printBuf((literal!("\", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.ENUMLITERAL;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printEnumLiteralAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printRestrictionAsCorbaString(mut r: Absyn::Restriction) -> Result<()> {
    let _ = (match r.clone() {
        Absyn::Restriction::R_CLASS => {
            Print::printBuf((literal!("record Absyn.R_CLASS end Absyn.R_CLASS;")).clone())?;
            ()
        },
        Absyn::Restriction::R_OPTIMIZATION => {
            Print::printBuf((literal!("record Absyn.R_OPTIMIZATION end Absyn.R_OPTIMIZATION;")).clone())?;
            ()
        },
        Absyn::Restriction::R_MODEL => {
            Print::printBuf((literal!("record Absyn.R_MODEL end Absyn.R_MODEL;")).clone())?;
            ()
        },
        Absyn::Restriction::R_RECORD => {
            Print::printBuf((literal!("record Absyn.R_RECORD end Absyn.R_RECORD;")).clone())?;
            ()
        },
        Absyn::Restriction::R_BLOCK => {
            Print::printBuf((literal!("record Absyn.R_BLOCK end Absyn.R_BLOCK;")).clone())?;
            ()
        },
        Absyn::Restriction::R_CONNECTOR => {
            Print::printBuf((literal!("record Absyn.R_CONNECTOR end Absyn.R_CONNECTOR;")).clone())?;
            ()
        },
        Absyn::Restriction::R_EXP_CONNECTOR => {
            Print::printBuf((literal!("record Absyn.R_EXP_CONNECTOR end Absyn.R_EXP_CONNECTOR;")).clone())?;
            ()
        },
        Absyn::Restriction::R_TYPE => {
            Print::printBuf((literal!("record Absyn.R_TYPE end Absyn.R_TYPE;")).clone())?;
            ()
        },
        Absyn::Restriction::R_PACKAGE => {
            Print::printBuf((literal!("record Absyn.R_PACKAGE end Absyn.R_PACKAGE;")).clone())?;
            ()
        },
        Absyn::Restriction::R_FUNCTION { functionRestriction: mut functionRestriction } => {
            Print::printBuf((literal!("record Absyn.R_FUNCTION functionRestriction = ")).clone())?;
            printFunctionRestrictionAsCorbaString(functionRestriction.clone())?;
            Print::printBuf((literal!("end Absyn.R_FUNCTION;")).clone())?;
            ()
        },
        Absyn::Restriction::R_OPERATOR => {
            Print::printBuf((literal!("record Absyn.R_OPERATOR end Absyn.R_OPERATOR;")).clone())?;
            ()
        },
        Absyn::Restriction::R_ENUMERATION => {
            Print::printBuf((literal!("record Absyn.R_ENUMERATION end Absyn.R_ENUMERATION;")).clone())?;
            ()
        },
        Absyn::Restriction::R_PREDEFINED_INTEGER => {
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_INTEGER end Absyn.R_PREDEFINED_INTEGER;")).clone())?;
            ()
        },
        Absyn::Restriction::R_PREDEFINED_REAL => {
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_REAL end Absyn.R_PREDEFINED_REAL;")).clone())?;
            ()
        },
        Absyn::Restriction::R_PREDEFINED_STRING => {
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_STRING end Absyn.R_PREDEFINED_STRING;")).clone())?;
            ()
        },
        Absyn::Restriction::R_PREDEFINED_BOOLEAN => {
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_BOOLEAN end Absyn.R_PREDEFINED_BOOLEAN;")).clone())?;
            ()
        },
        Absyn::Restriction::R_PREDEFINED_CLOCK => {
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_CLOCK end Absyn.R_PREDEFINED_CLOCK;")).clone())?;
            ()
        },
        Absyn::Restriction::R_PREDEFINED_ENUMERATION => {
            Print::printBuf((literal!("record Absyn.R_PREDEFINED_ENUMERATION end Absyn.R_PREDEFINED_ENUMERATION;")).clone())?;
            ()
        },
        Absyn::Restriction::R_UNIONTYPE => {
            Print::printBuf((literal!("record Absyn.R_UNIONTYPE end Absyn.R_UNIONTYPE;")).clone())?;
            ()
        },
        Absyn::Restriction::R_METARECORD { index: mut i, name: ref path, .. } => {
            Print::printBuf((literal!("record Absyn.R_METARECORD name = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", index = ")).clone())?;
            Print::printBuf((intString(i.clone())).clone())?;
            Print::printBuf((literal!(" end Absyn.R_METARECORD;")).clone())?;
            ()
        },
        Absyn::Restriction::R_UNKNOWN => {
            Print::printBuf((literal!("record Absyn.R_UNKNOWN end Absyn.R_UNKNOWN;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printRestrictionAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
    });
    Ok(())
}

fn printFunctionRestrictionAsCorbaString(mut functionRestriction: Absyn::FunctionRestriction) -> Result<()> {
    let _ = (match functionRestriction.clone() {
        Absyn::FunctionRestriction::FR_NORMAL_FUNCTION { purity: mut purity } => {
            Print::printBuf((literal!("record Absyn.FR_NORMAL_FUNCTION purity = ")).clone())?;
            printFunctionPurityAsCorbaString(purity.clone())?;
            Print::printBuf((literal!(" end Absyn.FR_NORMAL_FUNCTION;")).clone())?;
            ()
        },
        Absyn::FunctionRestriction::FR_OPERATOR_FUNCTION => {
            Print::printBuf((literal!("record Absyn.FR_OPERATOR_FUNCTION end Absyn.FR_OPERATOR_FUNCTION;")).clone())?;
            ()
        },
        Absyn::FunctionRestriction::FR_PARALLEL_FUNCTION => {
            Print::printBuf((literal!("record Absyn.FR_PARALLEL_FUNCTION end Absyn.FR_PARALLEL_FUNCTION;")).clone())?;
            ()
        },
        Absyn::FunctionRestriction::FR_KERNEL_FUNCTION => {
            Print::printBuf((literal!("record Absyn.FR_KERNEL_FUNCTION end Absyn.FR_KERNEL_FUNCTION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printFunctionPurityAsCorbaString(mut functionPurity: Absyn::FunctionPurity) -> Result<()> {
    let _ = (match functionPurity.clone() {
        Absyn::FunctionPurity::PURE => {
            Print::printBuf((literal!("record Absyn.PURE end Absyn.PURE;")).clone())?;
            ()
        },
        Absyn::FunctionPurity::IMPURE => {
            Print::printBuf((literal!("record Absyn.IMPURE end Absyn.IMPURE;")).clone())?;
            ()
        },
        Absyn::FunctionPurity::NO_PURITY => {
            Print::printBuf((literal!("record Absyn.NO_PURITY end Absyn.NO_PURITY;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printClassPartAsCorbaString(mut classPart: Arc<Absyn::ClassPart>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(classPart.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { contents } => {
            Print::printBuf((literal!("\nrecord Absyn.PUBLIC contents = ")).clone())?;
            printListAsCorbaString(contents.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.PUBLIC;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { contents } => {
            Print::printBuf((literal!("\nrecord Absyn.PROTECTED contents = ")).clone())?;
            printListAsCorbaString(contents.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.PROTECTED;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { contents: eqContents } => {
            Print::printBuf((literal!("\nrecord Absyn.EQUATIONS contents = ")).clone())?;
            printListAsCorbaString(eqContents.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQUATIONS;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { contents: eqContents } => {
            Print::printBuf((literal!("\nrecord Absyn.INITIALEQUATIONS contents = ")).clone())?;
            printListAsCorbaString(eqContents.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.INITIALEQUATIONS;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassPart::ALGORITHMS { contents: algContents } => {
            Print::printBuf((literal!("\nrecord Absyn.ALGORITHMS contents = ")).clone())?;
            printListAsCorbaString(algContents.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALGORITHMS;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { contents: algContents } => {
            Print::printBuf((literal!("\nrecord Absyn.INITIALALGORITHMS contents = ")).clone())?;
            printListAsCorbaString(algContents.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.INITIALALGORITHMS;")).clone())?;
            ()
        },
        Deref @ Absyn::ClassPart::EXTERNAL { externalDecl, annotation_ } => {
            Print::printBuf((literal!("\nrecord Absyn.EXTERNAL externalDecl = ")).clone())?;
            printExternalDeclAsCorbaString(externalDecl.clone())?;
            Print::printBuf((literal!(", annotation_ = ")).clone())?;
            printOption(annotation_.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.EXTERNAL;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printClassPartAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printExternalDeclAsCorbaString(mut decl: Arc<Absyn::ExternalDecl>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(decl.clone()) {
        Deref @ Absyn::ExternalDecl { funcName, lang, output_, args, annotation_ } => {
            Print::printBuf((literal!("record Absyn.EXTERNALDECL funcName = ")).clone())?;
            printStringCommentOption(funcName.clone())?;
            Print::printBuf((literal!(", lang = ")).clone())?;
            printStringCommentOption(lang.clone())?;
            Print::printBuf((literal!(", output_ = ")).clone())?;
            printOption(output_.clone(), Arc::new(printComponentRefAsCorbaString))?;
            Print::printBuf((literal!(", args = ")).clone())?;
            printListAsCorbaString(args.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", annotation_ = ")).clone())?;
            printOption(annotation_.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.EXTERNALDECL;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printExternalDeclAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printElementItemAsCorbaString(mut el: Arc<Absyn::ElementItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element } => {
            Print::printBuf((literal!("record Absyn.ELEMENTITEM element = ")).clone())?;
            printElementAsCorbaString(element.clone())?;
            Print::printBuf((literal!(" end Absyn.ELEMENTITEM;")).clone())?;
            ()
        },
        Deref @ Absyn::ElementItem::LEXER_COMMENT { comment: cmt } => {
            Print::printBuf((literal!("record Absyn.ELEMENTITEM element = \"")).clone())?;
            Print::printBuf((cmt.clone()).clone())?;
            Print::printBuf((literal!("\" end Absyn.ELEMENTITEM;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printElementItemAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printElementAsCorbaString(mut el: Arc<Absyn::Element>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::Element::ELEMENT { finalPrefix, redeclareKeywords, innerOuter, specification, info, constrainClass } => {
            Print::printBuf((literal!("\nrecord Absyn.ELEMENT finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(",redeclareKeywords = ")).clone())?;
            printOption(redeclareKeywords.clone(), Arc::new(printRedeclareKeywordsAsCorbaString))?;
            Print::printBuf((literal!(",innerOuter = ")).clone())?;
            printInnerOuterAsCorbaString(innerOuter.clone())?;
            Print::printBuf((literal!(",specification = ")).clone())?;
            printElementSpecAsCorbaString(specification.clone())?;
            Print::printBuf((literal!(",info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(",constrainClass = ")).clone())?;
            printOption(constrainClass.clone(), Arc::new(printConstrainClassAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.ELEMENT;")).clone())?;
            ()
        },
        Deref @ Absyn::Element::DEFINEUNIT { name, args, .. } => {
            Print::printBuf((literal!("\nrecord Absyn.DEFINEUNIT name = \"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\", args = ")).clone())?;
            printListAsCorbaString(args.clone(), Arc::new(printNamedArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.DEFINEUNIT;")).clone())?;
            ()
        },
        Deref @ Absyn::Element::TEXT { optName, string, info } => {
            Print::printBuf((literal!("\nrecord Absyn.TEXT optName = ")).clone())?;
            printStringCommentOption(optName.clone())?;
            Print::printBuf((literal!(", string = \"")).clone())?;
            Print::printBuf((string.clone()).clone())?;
            Print::printBuf((literal!("\", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.TEXT;")).clone())?;
            ()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("printElementAsCorbaString failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn printInnerOuterAsCorbaString(mut innerOuter: Absyn::InnerOuter) -> Result<()> {
    let _ = (match innerOuter.clone() {
        Absyn::InnerOuter::INNER => {
            Print::printBuf((literal!("record Absyn.INNER end Absyn.INNER;")).clone())?;
            ()
        },
        Absyn::InnerOuter::OUTER => {
            Print::printBuf((literal!("record Absyn.OUTER end Absyn.OUTER;")).clone())?;
            ()
        },
        Absyn::InnerOuter::INNER_OUTER => {
            Print::printBuf((literal!("record Absyn.INNER_OUTER end Absyn.INNER_OUTER;")).clone())?;
            ()
        },
        Absyn::InnerOuter::NOT_INNER_OUTER => {
            Print::printBuf((literal!("record Absyn.NOT_INNER_OUTER end Absyn.NOT_INNER_OUTER;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printRedeclareKeywordsAsCorbaString(mut redeclareKeywords: Absyn::RedeclareKeywords) -> Result<()> {
    let _ = (match redeclareKeywords.clone() {
        Absyn::RedeclareKeywords::REDECLARE => {
            Print::printBuf((literal!("record Absyn.REDECLARE end Absyn.REDECLARE;")).clone())?;
            ()
        },
        Absyn::RedeclareKeywords::REPLACEABLE => {
            Print::printBuf((literal!("record Absyn.REPLACEABLE end Absyn.REPLACEABLE;")).clone())?;
            ()
        },
        Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE => {
            Print::printBuf((literal!("record Absyn.REDECLARE_REPLACEABLE end Absyn.REDECLARE_REPLACEABLE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printConstrainClassAsCorbaString(mut constrainClass: Arc<Absyn::ConstrainClass>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(constrainClass.clone()) {
        Deref @ Absyn::ConstrainClass { elementSpec, comment } => {
            Print::printBuf((literal!("record Absyn.CONSTRAINCLASS elementSpec = ")).clone())?;
            printElementSpecAsCorbaString(elementSpec.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.CONSTRAINCLASS;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printElementSpecAsCorbaString(mut spec: Arc<Absyn::ElementSpec>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_, class_ } => {
            Print::printBuf((literal!("record Absyn.CLASSDEF replaceable_ = ")).clone())?;
            Print::printBuf((boolString(replaceable_.clone())).clone())?;
            Print::printBuf((literal!(", class_ = ")).clone())?;
            printClassAsCorbaString(class_.clone())?;
            Print::printBuf((literal!(" end Absyn.CLASSDEF;")).clone())?;
            ()
        },
        Deref @ Absyn::ElementSpec::EXTENDS { path, elementArg, annotationOpt } => {
            Print::printBuf((literal!("record Absyn.EXTENDS path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", elementArg = ")).clone())?;
            printListAsCorbaString(elementArg.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", annotationOpt = ")).clone())?;
            printOption(annotationOpt.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.EXTENDS;")).clone())?;
            ()
        },
        Deref @ Absyn::ElementSpec::IMPORT { import_, comment, info } => {
            Print::printBuf((literal!("record Absyn.IMPORT import_ = ")).clone())?;
            printImportAsCorbaString(import_.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.IMPORT;")).clone())?;
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { attributes, typeSpec, components } => {
            Print::printBuf((literal!("record Absyn.COMPONENTS attributes = ")).clone())?;
            printElementAttributesAsCorbaString(attributes.clone())?;
            Print::printBuf((literal!(", typeSpec = ")).clone())?;
            printTypeSpecAsCorbaString(typeSpec.clone())?;
            Print::printBuf((literal!(", components = ")).clone())?;
            printListAsCorbaString(components.clone(), Arc::new(printComponentItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.COMPONENTS;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printComponentItemAsCorbaString(mut componentItem: Arc<Absyn::ComponentItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(componentItem.clone()) {
        Deref @ Absyn::ComponentItem { component, condition, comment } => {
            Print::printBuf((literal!("record Absyn.COMPONENTITEM component = ")).clone())?;
            printComponentAsCorbaString(component.clone())?;
            Print::printBuf((literal!(", condition = ")).clone())?;
            printOption(condition.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.COMPONENTITEM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printComponentAsCorbaString(mut component: Absyn::Component) -> Result<()> {
    let _ = (match component.clone() {
        Absyn::Component { name: mut name, arrayDim: mut arrayDim, modification: mut modification } => {
            Print::printBuf((literal!("record Absyn.COMPONENT name = \"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\", arrayDim = ")).clone())?;
            printArrayDimAsCorbaString(arrayDim.clone())?;
            Print::printBuf((literal!(", modification = ")).clone())?;
            printOption(modification.clone(), Arc::new(printModificationAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.COMPONENT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printModificationAsCorbaString(mut r#mod: Arc<Absyn::Modification>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::Modification { elementArgLst, eqMod } => {
            Print::printBuf((literal!("record Absyn.CLASSMOD elementArgLst = ")).clone())?;
            printListAsCorbaString(elementArgLst.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", eqMod = ")).clone())?;
            printEqModAsCorbaString(eqMod.clone())?;
            Print::printBuf((literal!(" end Absyn.CLASSMOD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEqModAsCorbaString(mut eqMod: Arc<Absyn::EqMod>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::EqMod::NOMOD => {
            Print::printBuf((literal!("record Absyn.NOMOD end Absyn.NOMOD;")).clone())?;
            ()
        },
        Deref @ Absyn::EqMod::EQMOD { exp, info } => {
            Print::printBuf((literal!("record Absyn.EQMOD exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.EQMOD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEquationItemAsCorbaString(mut el: Arc<Absyn::EquationItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_, comment, info } => {
            Print::printBuf((literal!("\nrecord Absyn.EQUATIONITEM equation_ = ")).clone())?;
            printEquationAsCorbaString(equation_.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.EQUATIONITEM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printEquationAsCorbaString(mut eq: Arc<Absyn::Equation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::Equation::EQ_IF { ifExp, equationTrueItems, elseIfBranches, equationElseItems } => {
            Print::printBuf((literal!("record Absyn.EQ_IF ifExp = ")).clone())?;
            printExpAsCorbaString(ifExp.clone())?;
            Print::printBuf((literal!(", equationTrueItems = ")).clone())?;
            printListAsCorbaString(equationTrueItems.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseIfBranches = ")).clone())?;
            printListAsCorbaString(elseIfBranches.clone(), Arc::new(printEquationBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", equationElseItems = ")).clone())?;
            printListAsCorbaString(equationElseItems.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_IF;")).clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_EQUALS { leftSide, rightSide } => {
            Print::printBuf((literal!("record Absyn.EQ_EQUALS leftSide = ")).clone())?;
            printExpAsCorbaString(leftSide.clone())?;
            Print::printBuf((literal!(", rightSide = ")).clone())?;
            printExpAsCorbaString(rightSide.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_EQUALS;")).clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_PDE { leftSide, rightSide, domain: cr } => {
            Print::printBuf((literal!("record Absyn.EQ_PDE leftSide = ")).clone())?;
            printExpAsCorbaString(leftSide.clone())?;
            Print::printBuf((literal!(", rightSide = ")).clone())?;
            printExpAsCorbaString(rightSide.clone())?;
            Print::printBuf((literal!(", domain = ")).clone())?;
            printComponentRefAsCorbaString(cr.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_PDE;")).clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_CONNECT { connector1, connector2 } => {
            Print::printBuf((literal!("record Absyn.EQ_CONNECT connector1 = ")).clone())?;
            printComponentRefAsCorbaString(connector1.clone())?;
            Print::printBuf((literal!(", connector2 = ")).clone())?;
            printComponentRefAsCorbaString(connector2.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_CONNECT;")).clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_FOR { iterators, forEquations } => {
            Print::printBuf((literal!("record Absyn.EQ_FOR iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", forEquations = ")).clone())?;
            printListAsCorbaString(forEquations.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_FOR;")).clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { whenExp, whenEquations, elseWhenEquations } => {
            Print::printBuf((literal!("record Absyn.EQ_WHEN_E whenExp = ")).clone())?;
            printExpAsCorbaString(whenExp.clone())?;
            Print::printBuf((literal!(", whenEquations = ")).clone())?;
            printListAsCorbaString(whenEquations.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseWhenEquations = ")).clone())?;
            printListAsCorbaString(elseWhenEquations.clone(), Arc::new(printEquationBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_WHEN_E;")).clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { functionName, functionArgs } => {
            Print::printBuf((literal!("record Absyn.EQ_NORETCALL functionName = ")).clone())?;
            printComponentRefAsCorbaString(functionName.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_NORETCALL;")).clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_FAILURE { equ } => {
            Print::printBuf((literal!("record Absyn.EQ_FAILURE equ = ")).clone())?;
            printEquationItemAsCorbaString(equ.clone())?;
            Print::printBuf((literal!(" end Absyn.EQ_FAILURE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printAlgorithmItemAsCorbaString(mut el: Arc<Absyn::AlgorithmItem>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_, comment, info } => {
            Print::printBuf((literal!("\nrecord Absyn.ALGORITHMITEM algorithm_ = ")).clone())?;
            printAlgorithmAsCorbaString(algorithm_.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printOption(comment.clone(), Arc::new(printCommentAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.ALGORITHMITEM;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printAlgorithmAsCorbaString(mut alg: Arc<Absyn::Algorithm>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { assignComponent, value } => {
            Print::printBuf((literal!("record Absyn.ALG_ASSIGN assignComponent = ")).clone())?;
            printExpAsCorbaString(assignComponent.clone())?;
            Print::printBuf((literal!(", value = ")).clone())?;
            printExpAsCorbaString(value.clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_ASSIGN;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_IF { ifExp, trueBranch, elseIfAlgorithmBranch, elseBranch } => {
            Print::printBuf((literal!("record Absyn.ALG_IF ifExp = ")).clone())?;
            printExpAsCorbaString(ifExp.clone())?;
            Print::printBuf((literal!(", trueBranch = ")).clone())?;
            printListAsCorbaString(trueBranch.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseIfAlgorithmBranch = ")).clone())?;
            printListAsCorbaString(elseIfAlgorithmBranch.clone(), Arc::new(printAlgorithmBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseBranch = ")).clone())?;
            printListAsCorbaString(elseBranch.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_IF;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_FOR { iterators, forBody } => {
            Print::printBuf((literal!("record Absyn.ALG_FOR iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", forBody = ")).clone())?;
            printListAsCorbaString(forBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_FOR;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_PARFOR { iterators, parforBody: forBody } => {
            Print::printBuf((literal!("record Absyn.ALG_PARFOR iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", parforBody = ")).clone())?;
            printListAsCorbaString(forBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_PARFOR;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { boolExpr, whileBody } => {
            Print::printBuf((literal!("record Absyn.ALG_WHILE boolExpr = ")).clone())?;
            printExpAsCorbaString(boolExpr.clone())?;
            Print::printBuf((literal!(", whileBody = ")).clone())?;
            printListAsCorbaString(whileBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_WHILE;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { boolExpr, whenBody, elseWhenAlgorithmBranch } => {
            Print::printBuf((literal!("record Absyn.ALG_WHEN_A boolExpr = ")).clone())?;
            printExpAsCorbaString(boolExpr.clone())?;
            Print::printBuf((literal!(", whenBody = ")).clone())?;
            printListAsCorbaString(whenBody.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", elseWhenAlgorithmBranch = ")).clone())?;
            printListAsCorbaString(elseWhenAlgorithmBranch.clone(), Arc::new(printAlgorithmBranchAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_WHEN_A;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { functionCall, functionArgs } => {
            Print::printBuf((literal!("record Absyn.ALG_NORETCALL functionCall = ")).clone())?;
            printComponentRefAsCorbaString(functionCall.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_NORETCALL;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_RETURN => {
            Print::printBuf((literal!("record Absyn.ALG_RETURN end Absyn.ALG_RETURN;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_BREAK => {
            Print::printBuf((literal!("record Absyn.ALG_BREAK end Absyn.ALG_BREAK;")).clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_FAILURE { equ: body } => {
            Print::printBuf((literal!("record Absyn.ALG_FAILURE body = ")).clone())?;
            printListAsCorbaString(body.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ALG_FAILURE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printAlgorithmBranchAsCorbaString(mut inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)) -> Result<()> {
    printTupleAsCorbaString(inBranch.clone(), Arc::new(printExpAsCorbaString), Arc::new(printAlgorithmItemListAsCorbaString))?;
    Ok(())
}

fn printAlgorithmItemListAsCorbaString(mut inLst: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) -> Result<()> {
    printListAsCorbaString(inLst.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

fn printEquationBranchAsCorbaString(mut inBranch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)) -> Result<()> {
    printTupleAsCorbaString(inBranch.clone(), Arc::new(printExpAsCorbaString), Arc::new(printEquationItemListAsCorbaString))?;
    Ok(())
}

fn printEquationItemListAsCorbaString(mut inLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<()> {
    printListAsCorbaString(inLst.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

fn printAnnotationAsCorbaString(mut annotation_: Arc<Absyn::Annotation>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(annotation_.clone()) {
        Deref @ Absyn::Annotation { elementArgs } => {
            Print::printBuf((literal!("record Absyn.ANNOTATION elementArgs = ")).clone())?;
            printListAsCorbaString(elementArgs.clone(), Arc::new(printElementArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ANNOTATION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printCommentAsCorbaString(mut inComment: Arc<Absyn::Comment>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inComment.clone()) {
        Deref @ Absyn::Comment { annotation_, comment } => {
            Print::printBuf((literal!("record Absyn.COMMENT annotation_ = ")).clone())?;
            printOption(annotation_.clone(), Arc::new(printAnnotationAsCorbaString))?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(" end Absyn.COMMENT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printTypeSpecAsCorbaString(mut typeSpec: Arc<Absyn::TypeSpec>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(typeSpec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { path, arrayDim } => {
            Print::printBuf((literal!("record Absyn.TPATH path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", arrayDim = ")).clone())?;
            printOption(arrayDim.clone(), Arc::new(printArrayDimAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.TPATH;")).clone())?;
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { path, typeSpecs, arrayDim } => {
            Print::printBuf((literal!("record Absyn.TPATH path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(", typeSpecs = ")).clone())?;
            printListAsCorbaString(typeSpecs.clone(), Arc::new(printTypeSpecAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", arrayDim = ")).clone())?;
            printOption(arrayDim.clone(), Arc::new(printArrayDimAsCorbaString))?;
            Print::printBuf((literal!(" end Absyn.TPATH;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printArrayDimAsCorbaString(mut arrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<()> {
    printListAsCorbaString(arrayDim.clone(), Arc::new(printSubscriptAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

fn printSubscriptAsCorbaString(mut subscript: Arc<Absyn::Subscript>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Absyn::Subscript::NOSUB => {
            Print::printBuf((literal!("record Absyn.NOSUB end Absyn.NOSUB;")).clone())?;
            ()
        },
        Deref @ Absyn::Subscript::SUBSCRIPT { subscript: sub } => {
            Print::printBuf((literal!("record Absyn.SUBSCRIPT subscript = ")).clone())?;
            printExpAsCorbaString(sub.clone())?;
            Print::printBuf((literal!(" end Absyn.SUBSCRIPT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printImportAsCorbaString(mut import_: Absyn::Import) -> Result<()> {
    let _ = (match import_.clone() {
        Absyn::Import::NAMED_IMPORT { name: mut name, path: mut path } => {
            Print::printBuf((literal!("record Absyn.NAMED_IMPORT name = \"")).clone())?;
            Print::printBuf((name.clone()).clone())?;
            Print::printBuf((literal!("\", path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.NAMED_IMPORT;")).clone())?;
            ()
        },
        Absyn::Import::QUAL_IMPORT { path: mut path } => {
            Print::printBuf((literal!("record Absyn.QUAL_IMPORT path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.QUAL_IMPORT;")).clone())?;
            ()
        },
        Absyn::Import::UNQUAL_IMPORT { path: mut path } => {
            Print::printBuf((literal!("record Absyn.UNQUAL_IMPORT path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.UNQUAL_IMPORT;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printElementAttributesAsCorbaString(mut attr: Absyn::ElementAttributes) -> Result<()> {
    let _ = (match attr.clone() {
        Absyn::ElementAttributes { flowPrefix: mut flowPrefix, streamPrefix: mut streamPrefix, parallelism: mut parallelism, variability: mut variability, direction: mut direction, isField: mut isField, arrayDim: mut arrayDim } => {
            Print::printBuf((literal!("record Absyn.ATTR flowPrefix = ")).clone())?;
            Print::printBuf((boolString(flowPrefix.clone())).clone())?;
            Print::printBuf((literal!(", streamPrefix = ")).clone())?;
            Print::printBuf((boolString(streamPrefix.clone())).clone())?;
            Print::printBuf((literal!(", parallelism = ")).clone())?;
            printParallelismAsCorbaString(parallelism.clone())?;
            Print::printBuf((literal!(", variability = ")).clone())?;
            printVariabilityAsCorbaString(variability.clone())?;
            Print::printBuf((literal!(", direction = ")).clone())?;
            printDirectionAsCorbaString(direction.clone())?;
            if intEq(Flags::getConfigEnum(Flags::GRAMMAR.clone())?, Flags::PDEMODELICA.clone()) {
                Print::printBuf((literal!(", isField = ")).clone())?;
                printIsFieldAsCorbaString(isField.clone())?;
            }
            Print::printBuf((literal!(", arrayDim = ")).clone())?;
            printArrayDimAsCorbaString(arrayDim.clone())?;
            Print::printBuf((literal!(" end Absyn.ATTR;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printParallelismAsCorbaString(mut parallelism: Absyn::Parallelism) -> Result<()> {
    let _ = (match parallelism.clone() {
        Absyn::Parallelism::PARGLOBAL => {
            Print::printBuf((literal!("record Absyn.PARGLOBAL end Absyn.PARGLOBAL;")).clone())?;
            ()
        },
        Absyn::Parallelism::PARLOCAL => {
            Print::printBuf((literal!("record Absyn.PARLOCAL end Absyn.PARLOCAL;")).clone())?;
            ()
        },
        Absyn::Parallelism::NON_PARALLEL => {
            Print::printBuf((literal!("record Absyn.NON_PARALLEL end Absyn.NON_PARALLEL;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printVariabilityAsCorbaString(mut var: Absyn::Variability) -> Result<()> {
    let _ = (match var.clone() {
        Absyn::Variability::VAR => {
            Print::printBuf((literal!("record Absyn.VAR end Absyn.VAR;")).clone())?;
            ()
        },
        Absyn::Variability::DISCRETE => {
            Print::printBuf((literal!("record Absyn.DISCRETE end Absyn.DISCRETE;")).clone())?;
            ()
        },
        Absyn::Variability::PARAM => {
            Print::printBuf((literal!("record Absyn.PARAM end Absyn.PARAM;")).clone())?;
            ()
        },
        Absyn::Variability::CONST => {
            Print::printBuf((literal!("record Absyn.CONST end Absyn.CONST;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printDirectionAsCorbaString(mut dir: Absyn::Direction) -> Result<()> {
    let _ = (match dir.clone() {
        Absyn::Direction::INPUT => {
            Print::printBuf((literal!("record Absyn.INPUT end Absyn.INPUT;")).clone())?;
            ()
        },
        Absyn::Direction::OUTPUT => {
            Print::printBuf((literal!("record Absyn.OUTPUT end Absyn.OUTPUT;")).clone())?;
            ()
        },
        Absyn::Direction::BIDIR => {
            Print::printBuf((literal!("record Absyn.BIDIR end Absyn.BIDIR;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printIsFieldAsCorbaString(mut isf: Absyn::IsField) -> Result<()> {
    let _ = (match isf.clone() {
        Absyn::IsField::NONFIELD => {
            Print::printBuf((literal!("record Absyn.NONFIELD end Absyn.NONFIELD;")).clone())?;
            ()
        },
        Absyn::IsField::FIELD => {
            Print::printBuf((literal!("record Absyn.FIELD end Absyn.FIELD;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printElementArgAsCorbaString(mut arg: Arc<Absyn::ElementArg>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { finalPrefix, eachPrefix, path: p, modification, comment, info } => {
            Print::printBuf((literal!("record Absyn.MODIFICATION finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(", eachPrefix = ")).clone())?;
            printEachAsCorbaString(eachPrefix.clone())?;
            Print::printBuf((literal!(", path = ")).clone())?;
            printPathAsCorbaString(p.clone())?;
            Print::printBuf((literal!(", modification = ")).clone())?;
            printOption(modification.clone(), Arc::new(printModificationAsCorbaString))?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.MODIFICATION;")).clone())?;
            ()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { finalPrefix, redeclareKeywords, eachPrefix, elementSpec, constrainClass, info } => {
            Print::printBuf((literal!("record Absyn.REDECLARATION finalPrefix = ")).clone())?;
            Print::printBuf((boolString(finalPrefix.clone())).clone())?;
            Print::printBuf((literal!(", redeclareKeywords = ")).clone())?;
            printRedeclareKeywordsAsCorbaString(redeclareKeywords.clone())?;
            Print::printBuf((literal!(", eachPrefix = ")).clone())?;
            printEachAsCorbaString(eachPrefix.clone())?;
            Print::printBuf((literal!(", elementSpec = ")).clone())?;
            printElementSpecAsCorbaString(elementSpec.clone())?;
            Print::printBuf((literal!(", constrainClass = ")).clone())?;
            printOption(constrainClass.clone(), Arc::new(printConstrainClassAsCorbaString))?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.REDECLARATION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printFunctionArgsAsCorbaString(mut fargs: Arc<Absyn::FunctionArgs>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(fargs.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args, argNames } => {
            Print::printBuf((literal!("record Absyn.FUNCTIONARGS args = ")).clone())?;
            printListAsCorbaString(args.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", argNames = ")).clone())?;
            printListAsCorbaString(argNames.clone(), Arc::new(printNamedArgAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.FUNCTIONARGS;")).clone())?;
            ()
        },
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { exp, iterType: _, iterators } => {
            Print::printBuf((literal!("record Absyn.FOR_ITER_FARG exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(", iterators = ")).clone())?;
            printListAsCorbaString(iterators.clone(), Arc::new(printForIteratorAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.FOR_ITER_FARG;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printForIteratorAsCorbaString(mut iter: Arc<Absyn::ForIterator>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(iter.clone()) {
        Deref @ Absyn::ForIterator { name: id, guardExp, range } => {
            Print::printBuf((literal!("record Absyn.ITERATOR name = \"")).clone())?;
            Print::printBuf((id.clone()).clone())?;
            Print::printBuf((literal!("\", guardExp = ")).clone())?;
            printOption(guardExp.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", range = ")).clone())?;
            printOption(range.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!("end Absyn.ITERATOR;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printNamedArgAsCorbaString(mut arg: Arc<Absyn::NamedArg>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::NamedArg { argName, argValue } => {
            Print::printBuf((literal!("record Absyn.NAMEDARG argName = \"")).clone())?;
            Print::printBuf((argName.clone()).clone())?;
            Print::printBuf((literal!("\", argValue = ")).clone())?;
            printExpAsCorbaString(argValue.clone())?;
            Print::printBuf((literal!(" end Absyn.NAMEDARG;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printExpAsCorbaString(mut inExp: Arc<Absyn::Exp>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::INTEGER { value: i } => {
            Print::printBuf((literal!("record Absyn.INTEGER value = ")).clone())?;
            Print::printBuf((intString(i.clone())).clone())?;
            Print::printBuf((literal!(" end Absyn.INTEGER;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::REAL { value: s } => {
            Print::printBuf((literal!("record Absyn.REAL value = ")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!(" end Absyn.REAL;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::CREF { componentRef } => {
            Print::printBuf((literal!("record Absyn.CREF componentRef = ")).clone())?;
            printComponentRefAsCorbaString(componentRef.clone())?;
            Print::printBuf((literal!(" end Absyn.CREF;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::STRING { value: s } => {
            Print::printBuf((literal!("record Absyn.STRING value = \"")).clone())?;
            Print::printBuf((s.clone()).clone())?;
            Print::printBuf((literal!("\" end Absyn.STRING;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::BOOL { value: b } => {
            Print::printBuf((literal!("record Absyn.BOOL value = ")).clone())?;
            Print::printBuf((boolString(b.clone())).clone())?;
            Print::printBuf((literal!(" end Absyn.BOOL;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::BINARY { exp1, op, exp2 } => {
            Print::printBuf((literal!("record Absyn.BINARY exp1 = ")).clone())?;
            printExpAsCorbaString(exp1.clone())?;
            Print::printBuf((literal!(", op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp2 = ")).clone())?;
            printExpAsCorbaString(exp2.clone())?;
            Print::printBuf((literal!(" end Absyn.BINARY;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::UNARY { op, exp } => {
            Print::printBuf((literal!("record Absyn.UNARY op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.UNARY;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::LBINARY { exp1, op, exp2 } => {
            Print::printBuf((literal!("record Absyn.LBINARY exp1 = ")).clone())?;
            printExpAsCorbaString(exp1.clone())?;
            Print::printBuf((literal!(", op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp2 = ")).clone())?;
            printExpAsCorbaString(exp2.clone())?;
            Print::printBuf((literal!(" end Absyn.LBINARY;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::LUNARY { op, exp } => {
            Print::printBuf((literal!("record Absyn.LUNARY op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.LUNARY;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::RELATION { exp1, op, exp2 } => {
            Print::printBuf((literal!("record Absyn.RELATION exp1 = ")).clone())?;
            printExpAsCorbaString(exp1.clone())?;
            Print::printBuf((literal!(", op = ")).clone())?;
            printOperatorAsCorbaString(op.clone())?;
            Print::printBuf((literal!(", exp2 = ")).clone())?;
            printExpAsCorbaString(exp2.clone())?;
            Print::printBuf((literal!(" end Absyn.RELATION;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::IFEXP { ifExp, trueBranch, elseBranch, elseIfBranch } => {
            Print::printBuf((literal!("record Absyn.IFEXP ifExp = ")).clone())?;
            printExpAsCorbaString(ifExp.clone())?;
            Print::printBuf((literal!(", trueBranch = ")).clone())?;
            printExpAsCorbaString(trueBranch.clone())?;
            Print::printBuf((literal!(", elseBranch = ")).clone())?;
            printExpAsCorbaString(elseBranch.clone())?;
            Print::printBuf((literal!(", elseIfBranch = ")).clone())?;
            printListAsCorbaString(elseIfBranch.clone(), Arc::new(printTupleExpExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.IFEXP;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::CALL { function_, functionArgs, .. } => {
            Print::printBuf((literal!("record Absyn.CALL function_ = ")).clone())?;
            printComponentRefAsCorbaString(function_.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.CALL;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_, functionArgs } => {
            Print::printBuf((literal!("record Absyn.PARTEVALFUNCTION function_ = ")).clone())?;
            printComponentRefAsCorbaString(function_.clone())?;
            Print::printBuf((literal!(", functionArgs = ")).clone())?;
            printFunctionArgsAsCorbaString(functionArgs.clone())?;
            Print::printBuf((literal!(" end Absyn.PARTEVALFUNCTION;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::ARRAY { arrayExp } => {
            Print::printBuf((literal!("record Absyn.ARRAY arrayExp = ")).clone())?;
            printListAsCorbaString(arrayExp.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.ARRAY;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::MATRIX { matrix } => {
            Print::printBuf((literal!("record Absyn.MATRIX matrix = ")).clone())?;
            printListAsCorbaString(matrix.clone(), Arc::new(printListExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.MATRIX;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::RANGE { start, step, stop } => {
            Print::printBuf((literal!("record Absyn.RANGE start = ")).clone())?;
            printExpAsCorbaString(start.clone())?;
            Print::printBuf((literal!(", step = ")).clone())?;
            printOption(step.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", stop = ")).clone())?;
            printExpAsCorbaString(stop.clone())?;
            Print::printBuf((literal!(" end Absyn.RANGE;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::TUPLE { expressions } => {
            Print::printBuf((literal!("record Absyn.TUPLE expressions = ")).clone())?;
            printListAsCorbaString(expressions.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.TUPLE;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::END => {
            Print::printBuf((literal!("record Absyn.END end Absyn.END;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::CODE { code } => {
            Print::printBuf((literal!("record Absyn.CODE code = ")).clone())?;
            printCodeAsCorbaString(code.clone())?;
            Print::printBuf((literal!(" end Absyn.CODE;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::AS { id, exp } => {
            Print::printBuf((literal!("record Absyn.AS id = \"")).clone())?;
            Print::printBuf((id.clone()).clone())?;
            Print::printBuf((literal!("\", exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.AS;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::CONS { head, rest } => {
            Print::printBuf((literal!("record Absyn.CONS head = ")).clone())?;
            printExpAsCorbaString(head.clone())?;
            Print::printBuf((literal!(", rest = ")).clone())?;
            printExpAsCorbaString(rest.clone())?;
            Print::printBuf((literal!(" end Absyn.CONS;")).clone())?;
            ()
        },
        Deref @ Absyn::Exp::MATCHEXP { matchTy, inputExp, localDecls, cases, comment } => {
            Print::printBuf((literal!("record Absyn.MATCHEXP matchTy = ")).clone())?;
            printMatchTypeAsCorbaString(matchTy.clone())?;
            Print::printBuf((literal!(", inputExp = ")).clone())?;
            printExpAsCorbaString(inputExp.clone())?;
            Print::printBuf((literal!(", localDecls = ")).clone())?;
            printListAsCorbaString(localDecls.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",\n")).clone())?;
            Print::printBuf((literal!(", cases = ")).clone())?;
            printListAsCorbaString(cases.clone(), Arc::new(printCaseAsCorbaString), (literal!(",\n")).clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(" end Absyn.MATCHEXP;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printMatchTypeAsCorbaString(mut matchTy: Absyn::MatchType) -> Result<()> {
    let _ = (match matchTy.clone() {
        Absyn::MatchType::MATCH => {
            Print::printBuf((literal!("record Absyn.MATCH end Absyn.MATCH;")).clone())?;
            ()
        },
        Absyn::MatchType::MATCHCONTINUE => {
            Print::printBuf((literal!("record Absyn.MATCHCONTINUE end Absyn.MATCHCONTINUE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printCaseAsCorbaString(mut case_: Arc<Absyn::Case>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(case_.clone()) {
        Deref @ Absyn::Case::CASE { pattern, patternGuard, patternInfo, localDecls, classPart, result, resultInfo, comment, info } => {
            Print::printBuf((literal!("record Absyn.CASE pattern = ")).clone())?;
            printExpAsCorbaString(pattern.clone())?;
            Print::printBuf((literal!(", patternGuard = ")).clone())?;
            printOption(patternGuard.clone(), Arc::new(printExpAsCorbaString))?;
            Print::printBuf((literal!(", patternInfo = ")).clone())?;
            printInfoAsCorbaString(patternInfo.clone())?;
            Print::printBuf((literal!(", localDecls = ")).clone())?;
            printListAsCorbaString(localDecls.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", classPart = ")).clone())?;
            printClassPartAsCorbaString(classPart.clone())?;
            Print::printBuf((literal!(", result = ")).clone())?;
            printExpAsCorbaString(result.clone())?;
            Print::printBuf((literal!(", resultInfo = ")).clone())?;
            printInfoAsCorbaString(resultInfo.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.CASE;")).clone())?;
            ()
        },
        Deref @ Absyn::Case::ELSE { localDecls, classPart, result, resultInfo, comment, info } => {
            Print::printBuf((literal!("record Absyn.ELSE localDecls = ")).clone())?;
            printListAsCorbaString(localDecls.clone(), Arc::new(printElementItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(", classPart = ")).clone())?;
            printClassPartAsCorbaString(classPart.clone())?;
            Print::printBuf((literal!(", result = ")).clone())?;
            printExpAsCorbaString(result.clone())?;
            Print::printBuf((literal!(", resultInfo = ")).clone())?;
            printInfoAsCorbaString(resultInfo.clone())?;
            Print::printBuf((literal!(", comment = ")).clone())?;
            printStringCommentOption(comment.clone())?;
            Print::printBuf((literal!(", info = ")).clone())?;
            printInfoAsCorbaString(info.clone())?;
            Print::printBuf((literal!(" end Absyn.ELSE;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printCodeAsCorbaString(mut code: Arc<Absyn::CodeNode>) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(code.clone()) {
        Deref @ Absyn::CodeNode::C_TYPENAME { path } => {
            Print::printBuf((literal!("record Absyn.C_TYPENAME path = ")).clone())?;
            printPathAsCorbaString(path.clone())?;
            Print::printBuf((literal!(" end Absyn.C_TYPENAME;")).clone())?;
            ()
        },
        Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef } => {
            Print::printBuf((literal!("record Absyn.C_VARIABLENAME componentRef = ")).clone())?;
            printComponentRefAsCorbaString(componentRef.clone())?;
            Print::printBuf((literal!(" end Absyn.C_VARIABLENAME;")).clone())?;
            ()
        },
        Deref @ Absyn::CodeNode::C_EQUATIONSECTION { boolean, equationItemLst } => {
            Print::printBuf((literal!("record Absyn.C_EQUATIONSECTION boolean = ")).clone())?;
            Print::printBuf((boolString(boolean.clone())).clone())?;
            Print::printBuf((literal!(", equationItemLst = ")).clone())?;
            printListAsCorbaString(equationItemLst.clone(), Arc::new(printEquationItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.C_EQUATIONSECTION;")).clone())?;
            ()
        },
        Deref @ Absyn::CodeNode::C_ALGORITHMSECTION { boolean, algorithmItemLst } => {
            Print::printBuf((literal!("record Absyn.C_ALGORITHMSECTION boolean = ")).clone())?;
            Print::printBuf((boolString(boolean.clone())).clone())?;
            Print::printBuf((literal!(", algorithmItemLst = ")).clone())?;
            printListAsCorbaString(algorithmItemLst.clone(), Arc::new(printAlgorithmItemAsCorbaString), (literal!(",")).clone())?;
            Print::printBuf((literal!(" end Absyn.C_ALGORITHMSECTION;")).clone())?;
            ()
        },
        Deref @ Absyn::CodeNode::C_ELEMENT { element } => {
            Print::printBuf((literal!("record Absyn.C_ELEMENT element = ")).clone())?;
            printElementAsCorbaString(element.clone())?;
            Print::printBuf((literal!(" end Absyn.C_ELEMENT;")).clone())?;
            ()
        },
        Deref @ Absyn::CodeNode::C_EXPRESSION { exp } => {
            Print::printBuf((literal!("record Absyn.C_EXPRESSION exp = ")).clone())?;
            printExpAsCorbaString(exp.clone())?;
            Print::printBuf((literal!(" end Absyn.C_EXPRESSION;")).clone())?;
            ()
        },
        Deref @ Absyn::CodeNode::C_MODIFICATION { modification } => {
            Print::printBuf((literal!("record Absyn.C_MODIFICATION modification = ")).clone())?;
            printModificationAsCorbaString(modification.clone())?;
            Print::printBuf((literal!(" end Absyn.C_MODIFICATION;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn printListExpAsCorbaString(mut inLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<()> {
    printListAsCorbaString(inLst.clone(), Arc::new(printExpAsCorbaString), (literal!(",")).clone())?;
    Ok(())
}

fn printListAsCorbaString<Type_a: Clone + 'static>(mut inTypeALst: Arc<metamodelica::List<Type_a>>, mut inFuncTypeTypeATo: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, mut inString: ArcStr) -> Result<()> {
    pub type FuncTypeType_aTo<Type_a: Clone> = fn(Type_a) -> Result<()>;

    Print::printBuf((literal!("{")).clone())?;
    printList(inTypeALst.clone(), inFuncTypeTypeATo.clone(), (inString.clone()).clone())?;
    Print::printBuf((literal!("}")).clone())?;
    Ok(())
}

fn printTupleAsCorbaString<Type_a: Clone + 'static, Type_b: Clone + 'static>(mut inTpl: (Type_a, Type_b), mut fnA: Arc<dyn ::std::ops::Fn(Type_a) -> Result<()> + 'static>, mut fnB: Arc<dyn ::std::ops::Fn(Type_b) -> Result<()> + 'static>) -> Result<()> {
    pub type FuncTypeType_a<Type_a: Clone> = fn(Type_a) -> Result<()>;

    pub type FuncTypeType_b<Type_b: Clone> = fn(Type_b) -> Result<()>;

    let _ = (match (inTpl.clone(), fnA.clone(), fnB.clone()) {
        ((mut a, mut b), _, _) => {
            Print::printBuf((literal!("(")).clone())?;
            fnA(a.clone())?;
            Print::printBuf((literal!(",")).clone())?;
            fnB(b.clone())?;
            Print::printBuf((literal!(")")).clone())?;
            ()
        },
    });
    Ok(())
}

fn printOperatorAsCorbaString(mut op: Absyn::Operator) -> Result<()> {
    let _ = (match op.clone() {
        Absyn::Operator::ADD => {
            Print::printBuf((literal!("record Absyn.ADD end Absyn.ADD;")).clone())?;
            ()
        },
        Absyn::Operator::SUB => {
            Print::printBuf((literal!("record Absyn.SUB end Absyn.SUB;")).clone())?;
            ()
        },
        Absyn::Operator::MUL => {
            Print::printBuf((literal!("record Absyn.MUL end Absyn.MUL;")).clone())?;
            ()
        },
        Absyn::Operator::DIV => {
            Print::printBuf((literal!("record Absyn.DIV end Absyn.DIV;")).clone())?;
            ()
        },
        Absyn::Operator::POW => {
            Print::printBuf((literal!("record Absyn.POW end Absyn.POW;")).clone())?;
            ()
        },
        Absyn::Operator::UPLUS => {
            Print::printBuf((literal!("record Absyn.UPLUS end Absyn.UPLUS;")).clone())?;
            ()
        },
        Absyn::Operator::UMINUS => {
            Print::printBuf((literal!("record Absyn.UMINUS end Absyn.UMINUS;")).clone())?;
            ()
        },
        Absyn::Operator::ADD_EW => {
            Print::printBuf((literal!("record Absyn.ADD_EW end Absyn.ADD_EW;")).clone())?;
            ()
        },
        Absyn::Operator::SUB_EW => {
            Print::printBuf((literal!("record Absyn.SUB_EW end Absyn.SUB_EW;")).clone())?;
            ()
        },
        Absyn::Operator::MUL_EW => {
            Print::printBuf((literal!("record Absyn.MUL_EW end Absyn.MUL_EW;")).clone())?;
            ()
        },
        Absyn::Operator::DIV_EW => {
            Print::printBuf((literal!("record Absyn.DIV_EW end Absyn.DIV_EW;")).clone())?;
            ()
        },
        Absyn::Operator::UPLUS_EW => {
            Print::printBuf((literal!("record Absyn.UPLUS_EW end Absyn.UPLUS_EW;")).clone())?;
            ()
        },
        Absyn::Operator::UMINUS_EW => {
            Print::printBuf((literal!("record Absyn.UMINUS_EW end Absyn.UMINUS_EW;")).clone())?;
            ()
        },
        Absyn::Operator::AND => {
            Print::printBuf((literal!("record Absyn.AND end Absyn.AND;")).clone())?;
            ()
        },
        Absyn::Operator::OR => {
            Print::printBuf((literal!("record Absyn.OR end Absyn.OR;")).clone())?;
            ()
        },
        Absyn::Operator::NOT => {
            Print::printBuf((literal!("record Absyn.NOT end Absyn.NOT;")).clone())?;
            ()
        },
        Absyn::Operator::LESS => {
            Print::printBuf((literal!("record Absyn.LESS end Absyn.LESS;")).clone())?;
            ()
        },
        Absyn::Operator::LESSEQ => {
            Print::printBuf((literal!("record Absyn.LESSEQ end Absyn.LESSEQ;")).clone())?;
            ()
        },
        Absyn::Operator::GREATER => {
            Print::printBuf((literal!("record Absyn.GREATER end Absyn.GREATER;")).clone())?;
            ()
        },
        Absyn::Operator::GREATEREQ => {
            Print::printBuf((literal!("record Absyn.GREATEREQ end Absyn.GREATEREQ;")).clone())?;
            ()
        },
        Absyn::Operator::EQUAL => {
            Print::printBuf((literal!("record Absyn.EQUAL end Absyn.EQUAL;")).clone())?;
            ()
        },
        Absyn::Operator::NEQUAL => {
            Print::printBuf((literal!("record Absyn.NEQUAL end Absyn.NEQUAL;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printEachAsCorbaString(mut each_: Absyn::Each) -> Result<()> {
    let _ = (match each_.clone() {
        Absyn::Each::EACH => {
            Print::printBuf((literal!("record Absyn.EACH end Absyn.EACH;")).clone())?;
            ()
        },
        Absyn::Each::NON_EACH => {
            Print::printBuf((literal!("record Absyn.NON_EACH end Absyn.NON_EACH;")).clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printTupleExpExpAsCorbaString(mut tpl: (Arc<Absyn::Exp>, Arc<Absyn::Exp>)) -> Result<()> {
    printTupleAsCorbaString(tpl.clone(), Arc::new(printExpAsCorbaString), Arc::new(printExpAsCorbaString))?;
    Ok(())
}

fn printStringAsCorbaString(mut s: ArcStr) -> Result<()> {
    Print::printBuf((literal!("\"")).clone())?;
    Print::printBuf((s.clone()).clone())?;
    Print::printBuf((literal!("\"")).clone())?;
    Ok(())
}

pub fn writePath(mut file: File::File, mut path: Arc<Absyn::Path>, mut escape: Escape, mut delimiter: ArcStr, mut initialDot: bool) -> Result<()> {
    let mut p: Arc<Absyn::Path> = path.clone();
    while true {
        p = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::IDENT).clone()).clone(), escape.clone());
            return Ok(());
            bail!("fail")
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            File::writeEscape(file.clone(), (var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone(), escape.clone());
            File::writeEscape(file.clone(), (delimiter.clone()).clone(), escape.clone());
            var_field!((*p).path, Absyn::Path::QUALIFIED).clone()
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
            if initialDot.clone() {
                File::writeEscape(file.clone(), (delimiter.clone()).clone(), escape.clone());
            }
            var_field!((*p).path, Absyn::Path::FULLYQUALIFIED).clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    Ok(())
}

