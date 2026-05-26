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

use crate::Tpl;
use crate::TplAbsyn;
use crate::TplCodegen;
use crate::TplParser;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Print;
use openmodelica_util::System;

pub static emptyTxt: std::sync::LazyLock<Tpl::Text> = std::sync::LazyLock::new(|| { Tpl::Text::MEM_TEXT { tokens: metamodelica::nil(), blocksStack: metamodelica::nil() } });

pub static dsi: std::sync::LazyLock<SourceInfo> = std::sync::LazyLock::new(|| { TplAbsyn::dummySourceInfo.clone() });

pub fn main(mut inFile: ArcStr) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &(inFile.clone()) {
        Deref @ "SusanTest.tpl" => {
            tplMainTest((literal!("a")).clone())?;
            ()
        },
        file => {
            let mut strErrBuf: ArcStr = arcstr::literal!("");
            Print::clearBuf();
            translateFile((file.clone()).clone())?;
            strErrBuf = (Print::getErrorString()?).clone();
            strErrBuf = (if (strErrBuf.clone() == literal!("")) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Error Buffer ###\n")); __mm_s.push_str(&*strErrBuf.clone()); __mm_s.push_str(&*literal!("\n### End of Error Buffer ###\n")); ArcStr::from(__mm_s) }}).clone();
            println!("{}", (strErrBuf.clone()).clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub fn translateFile(mut inFile: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else { bail!("nomatch") };
            let mut destFile: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut txt: Tpl::Text;
            let mut tplPackage: TplAbsyn::TemplPackage;
            let mut mmPckg: TplAbsyn::MMPackage;
            let mut nErrors: i32 = 0;
            let mut wasError: bool = false;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nProcessing file '")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("'\n")); ArcStr::from(__mm_s) }).clone());
            nErrors = Error::getNumErrorMessages();
            destFile = (System::stringReplace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("*")); ArcStr::from(__mm_s) }).clone(), (literal!(".tpl*")).clone(), (literal!(".mo")).clone())?).clone();
            let false = (stringEq((file.clone()).clone(), (destFile.clone()).clone())) else { bail!("pattern mismatch") };
            tplPackage = TplParser::templPackageFromFile((file.clone()).clone())?;
            mmPckg = TplAbsyn::transformAST(tplPackage.clone())?;
            txt = emptyTxt.clone();
            txt = TplCodegen::mmPackage(txt.clone(), mmPckg.clone())?;
            res = (Tpl::textString(txt.clone())?).clone();
            wasError = nErrors.clone() < Error::getNumErrorMessages();
            destFile = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*destFile.clone()); __mm_s.push_str(&*if (wasError.clone()) {literal!(".err.mo")} else {literal!("")}); ArcStr::from(__mm_s) }).clone();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nWriting result to file '")); __mm_s.push_str(&*destFile.clone()); __mm_s.push_str(&*literal!("'\n")); ArcStr::from(__mm_s) }).clone());
            System::writeFile((destFile.clone()).clone(), (res.clone()).clone())?;
            let false = (wasError.clone()) else { bail!("pattern mismatch") };
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let mut file = __mc_input.clone() else { bail!("nomatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n### translation of file '")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("' failed!  ###\n")); ArcStr::from(__mm_s) }).clone());
            println!("{}", (literal!("### Error Buffer ###\n")).clone());
            println!("{}", (Print::getErrorString()?).clone());
            println!("{}", (literal!("\n### End of Error Buffer ###\n")).clone());
            Print::clearErrorBuf();
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

// ********** Tests ****************
pub fn testStringEquality(mut inStringReturned: ArcStr, mut inStringShouldBe: ArcStr, mut inPrintResult: bool, mut inPrintErrorBuffer: bool, mut inTestLabel: ArcStr, mut inNotPassedCnt: i32) -> Result<i32> {
    let mut outNotPassedCnt: i32 = 0;
    outNotPassedCnt = 'mc: {
        let __mc_input = (inStringReturned.clone(), inStringShouldBe.clone(), inPrintResult.clone(), inPrintErrorBuffer.clone(), inTestLabel.clone(), inNotPassedCnt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut strRet, mut strShouldBe, mut printResult, mut printErrBuf, mut strLabel, mut notPassedCnt) = __mc_input.clone() else { bail!("nomatch") };
            let mut strRes: ArcStr = arcstr::literal!("");
            let mut strErrBuf: ArcStr = arcstr::literal!("");
            let true = (stringEq((strRet.clone()).clone(), (strShouldBe.clone()).clone())) else { bail!("pattern mismatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n**************************************************\n")); __mm_s.push_str(&*strLabel.clone()); ArcStr::from(__mm_s) }).clone());
            strRes = (if (printResult.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  returned <<\n")); __mm_s.push_str(&*strRet.clone()); __mm_s.push_str(&*literal!(">>\n")); ArcStr::from(__mm_s) }} else {literal!("\n result not shown \n")}).clone();
            println!("{}", (strRes.clone()).clone());
            strErrBuf = (Print::getErrorString()?).clone();
            strErrBuf = (if (strErrBuf.clone() == literal!("")) {literal!("")} else {if (printErrBuf.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Error Buffer ###\n")); __mm_s.push_str(&*strErrBuf.clone()); __mm_s.push_str(&*literal!("\n### End of Error Buffer ###\n")); ArcStr::from(__mm_s) }} else {literal!("### Error Buffer is NOT empty - not shown ###\n")}}).clone();
            println!("{}", (strErrBuf.clone()).clone());
            println!("{}", (literal!("*** OK ***\n")).clone());
            Print::clearErrorBuf();
            Ok(notPassedCnt.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut strRet, mut strShouldBe, mut printResult, mut printErrBuf, mut strLabel, mut notPassedCnt) = __mc_input.clone() else { bail!("nomatch") };
            let mut strRes: ArcStr = arcstr::literal!("");
            let mut strErrBuf: ArcStr = arcstr::literal!("");
            let false = (stringEq((strRet.clone()).clone(), (strShouldBe.clone()).clone())) else { bail!("pattern mismatch") };
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n##################################################\n")); __mm_s.push_str(&*strLabel.clone()); ArcStr::from(__mm_s) }).clone());
            strRes = (if (printResult.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  returned <<\n")); __mm_s.push_str(&*strRet.clone()); __mm_s.push_str(&*literal!(">>\nshould be <<\n")); __mm_s.push_str(&*strShouldBe.clone()); __mm_s.push_str(&*literal!(">>\n")); ArcStr::from(__mm_s) }} else {literal!("\n result not shown \n")}).clone();
            println!("{}", (strRes.clone()).clone());
            strErrBuf = (Print::getErrorString()?).clone();
            strErrBuf = (if (strErrBuf.clone() == literal!("")) {literal!("")} else {if (printErrBuf.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("### Error Buffer ###\n")); __mm_s.push_str(&*strErrBuf.clone()); __mm_s.push_str(&*literal!("\n### End of Error Buffer ###\n")); ArcStr::from(__mm_s) }} else {literal!("### Error Buffer is NOT empty - not shown ###\n")}}).clone();
            println!("{}", (strErrBuf.clone()).clone());
            println!("{}", (literal!("### NOT Passed ###\n")).clone());
            Print::clearErrorBuf();
            Ok(notPassedCnt.clone() + 1)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
            Debug::trace((literal!("-!!!Tpl.tplMainTest failed.\n")).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNotPassedCnt)
}

pub fn testTranslateTplFile(mut inFile: ArcStr, mut inPrintResult: bool, mut inPrintErrorBuffer: bool, mut inNotPassedCnt: i32) -> Result<i32> {
    let mut outNotPassedCnt: i32 = 0;
    outNotPassedCnt = 'mc: {
        let __mc_input = (inFile.clone(), inPrintResult.clone(), inPrintErrorBuffer.clone(), inNotPassedCnt.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut file, mut printRes, mut printErrBuf, mut notPassedCnt) = __mc_input.clone() else { bail!("nomatch") };
            let mut res: ArcStr = arcstr::literal!("");
            let mut resToBe: ArcStr = arcstr::literal!("");
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone(), (literal!("Test failed.")).clone())?;
            translateFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(".tpl")); ArcStr::from(__mm_s) }).clone())?;
            res = (System::stringReplace((System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone())?).clone(), intStringChar(13), (literal!("")).clone())?).clone();
            resToBe = (System::stringReplace((System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("__testShouldBe.mo")); ArcStr::from(__mm_s) }).clone())?).clone(), intStringChar(13), (literal!("")).clone())?).clone();
            notPassedCnt = testStringEquality((res.clone()).clone(), (resToBe.clone()).clone(), printRes.clone(), printErrBuf.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("translateFile ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(".tpl")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
            Ok(notPassedCnt.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut file, mut printRes, mut printErrBuf, mut notPassedCnt) = __mc_input.clone() else { bail!("nomatch") };
            let mut res: ArcStr = arcstr::literal!("");
            let mut resToBe: ArcStr = arcstr::literal!("");
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone(), (literal!("Test failed.")).clone())?;
            res = (System::stringReplace((System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone())?).clone(), intStringChar(13), (literal!("")).clone())?).clone();
            resToBe = (System::stringReplace((System::readFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!("__testShouldBe.mo")); ArcStr::from(__mm_s) }).clone())?).clone(), intStringChar(13), (literal!("")).clone())?).clone();
            notPassedCnt = testStringEquality((res.clone()).clone(), (resToBe.clone()).clone(), printRes.clone(), printErrBuf.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("translateFile ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(".tpl")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
            Ok(notPassedCnt.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNotPassedCnt)
}

pub fn tplMainTest(mut inFile: ArcStr) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inFile.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "a" => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut strOut: ArcStr = arcstr::literal!("");
                    let mut ident: ArcStr = arcstr::literal!("");
                    let mut cval: ArcStr = arcstr::literal!("");
                    let mut chars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut txt: Tpl::Text;
                    let mut tequal: bool = false;
                    let mut tplPackage: TplAbsyn::TemplPackage;
                    let mut mmPckg: TplAbsyn::MMPackage;
                    let mut pid: Arc<TplAbsyn::PathIdent>;
                    let mut ts: Arc<TplAbsyn::TypeSignature> = Arc::new(TplAbsyn::TypeSignature::BOOLEAN_TYPE);
                    let mut astDefs: Arc<metamodelica::List<TplAbsyn::ASTDef>> = metamodelica::nil();
                    let mut expB: Arc<TplAbsyn::ExpressionBase> = Arc::new(TplAbsyn::ExpressionBase::ERROR_EXP);
                    let mut tok: Arc<Tpl::StringToken> = Arc::new(Tpl::StringToken::ST_NEW_LINE);
                    let mut tstart: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut lnum: i32 = 0;
                    let mut colnum: i32 = 0;
                    let mut llen: i32 = 0;
                    let mut notPassedCnt: i32 = 0;
                    notPassedCnt = 0;
                    Print::clearErrorBuf();
                    println!("{}", (literal!("\n A Test:\n")).clone());
                    tstart = clock();
                    txt = Tpl::writeStr(emptyTxt.clone(), (literal!("Ahoj Susan")).clone())?;
                    txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_ANCHOR { offset: 0 }))?;
                    txt = Tpl::writeStr(txt.clone(), (literal!("Ahoj Susan")).clone())?;
                    txt = Tpl::newLine(txt.clone())?;
                    txt = Tpl::writeStr(txt.clone(), (literal!("Ahoj Susan")).clone())?;
                    txt = Tpl::popBlock(txt.clone())?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("Ahoj SusanAhoj Susan\n          Ahoj Susan")).clone(), true, true, (literal!("Anchor")).clone(), notPassedCnt.clone())?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::pathIdent(txt.clone(), Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Susan")).clone() }))?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("Susan")).clone(), true, true, (literal!("PathIdent IDENT")).clone(), notPassedCnt.clone())?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::pathIdent(txt.clone(), Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Hej")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Susan")).clone() }) }))?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("Hej.Susan")).clone(), true, true, (literal!("PathIdent PATH_IDENT")).clone(), notPassedCnt.clone())?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::typedIdents(txt.clone(), list![(literal!("Hej"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)), (literal!("Susan"), Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Pa")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Li")).clone() }) }) }) }))])?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("Tpl.Text Hej;\nlist<Pa.Li> Susan;")).clone(), true, true, (literal!("typedIdents")).clone(), notPassedCnt.clone())?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::typedIdentsEx(txt.clone(), list![(literal!("Hej"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)), (literal!("Susan"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Pa")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Li")).clone() }) }) }))], (literal!("input")).clone(), (literal!("in")).clone())?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("input Tpl.Text inHej;\ninput Pa.Li inSusan;")).clone(), true, true, (literal!("typedIdentsEx")).clone(), notPassedCnt.clone())?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::mmPackage(txt.clone(), TplAbsyn::MMPackage { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Susan")).clone() }), mmDeclarations: list![TplAbsyn::MMDeclaration::MM_IMPORT { isPublic: true, packageName: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Pa")).clone(), path: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Li")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Ko")).clone() }) }) }) }, TplAbsyn::MMDeclaration::MM_STR_TOKEN_DECL { isPublic: true, name: (literal!("strTokConst")).clone(), value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Susan")).clone(), (literal!("is")).clone(), (literal!("beautiful\n")).clone()], lastHasNewLine: true }) }, TplAbsyn::MMDeclaration::MM_LITERAL_DECL { isPublic: false, name: (literal!("c_literalValueConst")).clone(), value: (literal!("123")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE) }, TplAbsyn::MMDeclaration::MM_FUN { isPublic: true, name: (literal!("MuchFun")).clone(), inArgs: list![(literal!("txt"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)), (literal!("laughLevel"), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE)), (literal!("jokes"), Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) }))], outArgs: list![(literal!("txt"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE))], locals: list![(literal!("txt"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)), (literal!("laughLevel"), Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE)), (literal!("jokes"), Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) }))], statements: list![Arc::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![(literal!("out_txt")).clone()], rhs: Arc::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("writeStr")).clone() }) }), args: list![Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("txt")).clone() }) }), Arc::new(TplAbsyn::MMExp::MM_STRING { value: (literal!("Susan")).clone() })] }) }), Arc::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![(literal!("out_txt")).clone()], rhs: Arc::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("writeTok")).clone() }) }), args: list![Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("out_txt")).clone() }) }), Arc::new(TplAbsyn::MMExp::MM_STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_LINE { line: (literal!("Susan is cosmic!\n")).clone() }) })] }) })], genInfoOpt: crate::TplAbsyn::GenInfo::GI_TEMPL_FUN }, TplAbsyn::MMDeclaration::MM_FUN { isPublic: true, name: (literal!("MoreFun")).clone(), inArgs: list![(literal!("txt"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE)), (literal!("v_laughLevel"), Arc::new(TplAbsyn::TypeSignature::OPTION_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) })), (literal!("v_jokes"), Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) }))], outArgs: list![(literal!("txt"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE))], locals: list![(literal!("txt"), Arc::new(crate::TplAbsyn::TypeSignature::TEXT_TYPE))], statements: list![Arc::new(TplAbsyn::MMExp::MM_MATCH { matchCases: list![(list![Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (literal!("txt")).clone() }), Arc::new(TplAbsyn::MatchingExp::SOME_MATCH { value: Arc::new(TplAbsyn::MatchingExp::BIND_AS_MATCH { bindIdent: (literal!("v_hej")).clone(), matchingExp: Arc::new(TplAbsyn::MatchingExp::STRING_MATCH { value: (literal!("Hej")).clone() }) }) }), Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (literal!("v_jokes")).clone() })], list![Arc::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![(literal!("txt")).clone()], rhs: Arc::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("writeStr")).clone() }) }), args: list![Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("txt")).clone() }) }), Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("v_hej")).clone() }) })] }) })]), (list![Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (literal!("txt")).clone() }), Arc::new(TplAbsyn::MatchingExp::SOME_MATCH { value: Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (literal!("v_hej")).clone() }) }), Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH)], list![Arc::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![(literal!("txt")).clone()], rhs: Arc::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("writeStr")).clone() }) }), args: list![Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("txt")).clone() }) }), Arc::new(TplAbsyn::MMExp::MM_STRING { value: (literal!("Not hej:")).clone() })] }) }), Arc::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![(literal!("txt")).clone()], rhs: Arc::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("writeStr")).clone() }) }), args: list![Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("txt")).clone() }) }), Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("v_hej")).clone() }) })] }) })]), (list![Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (literal!("txt")).clone() }), Arc::new(crate::TplAbsyn::MatchingExp::NONE_MATCH), Arc::new(crate::TplAbsyn::MatchingExp::REST_MATCH)], list![Arc::new(TplAbsyn::MMExp::MM_ASSIGN { lhsArgs: list![(literal!("txt")).clone()], rhs: Arc::new(TplAbsyn::MMExp::MM_FN_CALL { fnName: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("writeStr")).clone() }) }), args: list![Arc::new(TplAbsyn::MMExp::MM_IDENT { ident: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("txt")).clone() }) }), Arc::new(TplAbsyn::MMExp::MM_STRING { value: (literal!("NONE at all")).clone() })] }) })])] })], genInfoOpt: crate::TplAbsyn::GenInfo::GI_TEMPL_FUN }], annotationFooter: (literal!("")).clone() })?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("package Susan\n\npublic import Tpl;\n\npublic import Pa.Li.Ko;\n\npublic constant Tpl.StringToken strTokConst = Tpl.ST_STRING_LIST({\n                                                                                  \"Susan\",\n                                                                                  \"is\",\n                                                                                  \"beautiful\\n\"\n                                                                      }, true);\n\nprotected constant Integer c_literalValueConst = 123;\n\npublic function MuchFun\n  input Tpl.Text txt;\n  input Integer laughLevel;\n  input list<String> jokes;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt := Tpl.writeStr(txt, \"Susan\");\n  out_txt := Tpl.writeTok(out_txt, Tpl.ST_LINE(\"Susan is cosmic!\\n\"));\nend MuchFun;\n\npublic function MoreFun\n  input Tpl.Text in_txt;\n  input Option<String> in_v_laughLevel;\n  input list<String> in_v_jokes;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt :=\n  matchcontinue(in_txt, in_v_laughLevel, in_v_jokes)\n    local\n      Tpl.Text txt;\n\n    case ( txt,\n           SOME((v_hej as \"Hej\")),\n           v_jokes )\n      local\n        String v_hej;\n        list<String> v_jokes;\n      algorithm\n        txt = Tpl.writeStr(txt, v_hej);\n      then txt;\n\n    case ( txt,\n           SOME(v_hej),\n           _ )\n      local\n        String v_hej;\n      algorithm\n        txt = Tpl.writeStr(txt, \"Not hej:\");\n        txt = Tpl.writeStr(txt, v_hej);\n      then txt;\n\n    case ( txt,\n           NONE(),\n           _ )\n      algorithm\n        txt = Tpl.writeStr(txt, \"NONE at all\");\n      then txt;\n  end matchcontinue;\nend MoreFun;\n\nend Susan;")).clone(), false, false, (literal!("mmPackage")).clone(), notPassedCnt.clone())?;
                    tplPackage = TplAbsyn::TemplPackage { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Susan")).clone() }), astDefs: list![TplAbsyn::ASTDef { importPackage: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("TplAbsyn")).clone() }), isDefault: true, types: list![(literal!("Ident"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) }), (literal!("TypedIdents"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: list![Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Ident")).clone() }) }), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("PathIdent")).clone() }) })] }) }) }), (literal!("PathIdent"), TplAbsyn::TypeInfo::TI_UNION_TYPE { recTags: list![(literal!("IDENT"), list![(literal!("ident"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Ident")).clone() }) }))]), (literal!("PATH_IDENT"), list![(literal!("ident"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Ident")).clone() }) })), (literal!("path"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("PathIdent")).clone() }) }))])] })] }], templateDefs: list![(literal!("pathIdent"), TplAbsyn::TemplateDef::TEMPLATE_DEF { args: list![(literal!("it"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("PathIdent")).clone() }) }))], lesc: (literal!("")).clone(), resc: (literal!("")).clone(), exp: (Arc::new(TplAbsyn::ExpressionBase::MATCH { matchExp: (Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("it")).clone() }) }), dsi.clone()), cases: list![(Arc::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("IDENT")).clone() }), fieldMatchings: metamodelica::nil() }), (Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("ident")).clone() }) }), dsi.clone())), (Arc::new(TplAbsyn::MatchingExp::RECORD_MATCH { tagName: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("PATH_IDENT")).clone() }), fieldMatchings: metamodelica::nil() }), (Arc::new(TplAbsyn::ExpressionBase::TEMPLATE { items: list![(Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("ident")).clone() }) }), dsi.clone()), (Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(".")).clone() }) }), dsi.clone()), (Arc::new(TplAbsyn::ExpressionBase::FUN_CALL { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("pathIdent")).clone() }), args: list![(Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("path")).clone() }) }), dsi.clone())] }), dsi.clone())], lquote: (literal!("\"")).clone(), rquote: (literal!("\"")).clone() }), dsi.clone()))] }), dsi.clone()) }), (literal!("typedIdents"), TplAbsyn::TemplateDef::TEMPLATE_DEF { args: list![(literal!("decls"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("TypedIdents")).clone() }) }))], lesc: (literal!("")).clone(), resc: (literal!("")).clone(), exp: (Arc::new(TplAbsyn::ExpressionBase::ESCAPED { exp: (Arc::new(TplAbsyn::ExpressionBase::MAP { argExp: (Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("decls")).clone() }) }), dsi.clone()), ofBinding: Arc::new(TplAbsyn::MatchingExp::TUPLE_MATCH { tupleArgs: list![Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (literal!("id")).clone() }), Arc::new(TplAbsyn::MatchingExp::BIND_MATCH { bindIdent: (literal!("pid")).clone() })] }), mapExp: (Arc::new(TplAbsyn::ExpressionBase::TEMPLATE { items: list![(Arc::new(TplAbsyn::ExpressionBase::FUN_CALL { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("pathIdent")).clone() }), args: list![(Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("pid")).clone() }) }), dsi.clone())] }), dsi.clone()), (Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }) }), dsi.clone()), (Arc::new(TplAbsyn::ExpressionBase::BOUND_VALUE { boundPath: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("id")).clone() }) }), dsi.clone()), (Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }) }), dsi.clone())], lquote: (literal!("\"")).clone(), rquote: (literal!("\"")).clone() }), dsi.clone()), hasIndexIdentOpt: None }), dsi.clone()), options: list![(literal!("separator"), Some((Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }), dsi.clone())))] }), dsi.clone()) })], annotationFooter: (literal!("")).clone() };
                    mmPckg = TplAbsyn::transformAST(tplPackage.clone())?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::mmPackage(txt.clone(), mmPckg.clone())?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("package Susan\n\npublic import Tpl;\n\npublic import TplAbsyn;\n\npublic function pathIdent\n  input Tpl.Text in_txt;\n  input TplAbsyn.PathIdent in_i_it;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt :=\n  matchcontinue(in_txt, in_i_it)\n    local\n      Tpl.Text txt;\n\n    case ( txt,\n           TplAbsyn.IDENT(ident = i_ident) )\n      local\n        TplAbsyn.Ident i_ident;\n      algorithm\n        txt = Tpl.writeStr(txt, i_ident);\n      then txt;\n\n    case ( txt,\n           TplAbsyn.PATH_IDENT(ident = i_ident, path = i_path) )\n      local\n        TplAbsyn.PathIdent i_path;\n        TplAbsyn.Ident i_ident;\n      algorithm\n        txt = Tpl.writeStr(txt, i_ident);\n        txt = Tpl.writeTok(txt, Tpl.ST_STRING(\".\"));\n        txt = pathIdent(txt, i_path);\n      then txt;\n\n    else in_txt;\n  end matchcontinue;\nend pathIdent;\n\nprotected function lm_2\n  input Tpl.Text in_txt;\n  input TplAbsyn.TypedIdents in_items;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt :=\n  matchcontinue(in_txt, in_items)\n    local\n      Tpl.Text txt;\n\n    case ( txt,\n           {} )\n      then txt;\n\n    case ( txt,\n           (i_id, i_pid) :: rest )\n      local\n        TplAbsyn.TypedIdents rest;\n        TplAbsyn.PathIdent i_pid;\n        TplAbsyn.Ident i_id;\n      algorithm\n        txt = pathIdent(txt, i_pid);\n        txt = Tpl.writeTok(txt, Tpl.ST_STRING(\" \"));\n        txt = Tpl.writeStr(txt, i_id);\n        txt = Tpl.writeTok(txt, Tpl.ST_STRING(\";\"));\n        txt = Tpl.nextIter(txt);\n        txt = lm_2(txt, rest);\n      then txt;\n\n    case ( txt,\n           _ :: rest )\n      local\n        TplAbsyn.TypedIdents rest;\n      algorithm\n        txt = lm_2(txt, rest);\n      then txt;\n  end matchcontinue;\nend lm_2;\n\npublic function typedIdents\n  input Tpl.Text txt;\n  input TplAbsyn.TypedIdents i_decls;\n\n  output Tpl.Text out_txt;\nalgorithm\n  out_txt := Tpl.pushIter(txt, Tpl.ITER_OPTIONS(0, NONE(), SOME(Tpl.ST_NEW_LINE()), 0, 0, Tpl.ST_NEW_LINE(), 0, Tpl.ST_NEW_LINE()));\n  out_txt := lm_2(out_txt, i_decls);\n  out_txt := Tpl.popIter(out_txt);\nend typedIdents;\n\nend Susan;")).clone(), false, false, (literal!("transformAST - pathIdent() + typedIdents()")).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("// Hej Susan\n/*this is another dance with Susan */\n/* event I will /*nest*/ into */ //and still comment\n      Susan lives!")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _) = TplParser::interleave(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?)?;
                    strOut = (stringCharListString(chars.clone())).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("Susan lives!")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.interleave \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("(Susan)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    TplParser::afterKeyword(chars.clone())?;
                    strOut = (stringCharListString(chars.clone())).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("(Susan)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.afterKeyword \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("Susan2:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, ident) = TplParser::identifier(chars.clone())?;
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("*Susan2*:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.identifier \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("Susan:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _, pid) = TplParser::pathIdent(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?)?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::pathIdent(txt.clone(), pid.clone())?;
                    ident = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("*Susan*:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.pathIdent \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("Susan./*comment*/ Susan2 . tpl3_h4:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _, pid) = TplParser::pathIdent(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?)?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::pathIdent(txt.clone(), pid.clone())?;
                    ident = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("*Susan.Susan2.tpl3_h4*:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.pathIdent \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("Tpl.Susan:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _, ts) = TplParser::typeSig(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?)?;
                    tequal = ts.clone() == Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Tpl")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Susan")).clone() }) }) });
                    txt = emptyTxt.clone();
                    txt = TplCodegen::typeSig(txt.clone(), ts.clone())?;
                    ident = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*Tpl.Susan*:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.typeSig \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("list< tuple<Hej.Susan,list <String>,Option< /*uáá*/Integer>> >:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _, ts) = TplParser::typeSig(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?)?;
                    tequal = ts.clone() == Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: list![Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::PATH_IDENT { ident: (literal!("Hej")).clone(), path: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Susan")).clone() }) }) }), Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) }), Arc::new(TplAbsyn::TypeSignature::OPTION_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE) })] }) });
                    txt = emptyTxt.clone();
                    txt = TplCodegen::typeSig(txt.clone(), ts.clone())?;
                    ident = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*list<tuple<Hej.Susan, list<String>, Option<Integer>>>*:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.typeSig \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\ninterface package Susan\n  package TplAbsyn\n    type Ident = String;\n    type TypedIdents = list<tuple<Ident, PathIdent>>;\n\n    uniontype PathIdent\n      record IDENT\n        Ident ident;\n      end IDENT;\n\n      record PATH_IDENT\n        Ident ident;\n        PathIdent path;\n      end PATH_IDENT;\n    end PathIdent;\n  end TplAbsyn;\nend Susan;:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _, pid, astDefs) = TplParser::interfacePackage(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, metamodelica::nil())?;
                    tequal = astDefs.clone() == list![TplAbsyn::ASTDef { importPackage: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("TplAbsyn")).clone() }), isDefault: true, types: list![(literal!("Ident"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) }), (literal!("TypedIdents"), TplAbsyn::TypeInfo::TI_ALIAS_TYPE { aliasType: Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(TplAbsyn::TypeSignature::TUPLE_TYPE { ofTypes: list![Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Ident")).clone() }) }), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("PathIdent")).clone() }) })] }) }) }), (literal!("PathIdent"), TplAbsyn::TypeInfo::TI_UNION_TYPE { recTags: list![(literal!("IDENT"), list![(literal!("ident"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Ident")).clone() }) }))]), (literal!("PATH_IDENT"), list![(literal!("ident"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Ident")).clone() }) })), (literal!("path"), Arc::new(TplAbsyn::TypeSignature::NAMED_TYPE { name: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("PathIdent")).clone() }) }))])] })] }];
                    txt = emptyTxt.clone();
                    txt = TplCodegen::pathIdent(txt.clone(), pid.clone())?;
                    ident = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*ident.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*Susan*:)")).clone(), true, true, (literal!("TplParser.templPackage - absyn - type Ident, TypedIdents, PathIdent \n")).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\ninterface package Susan\npackage builtin\n  function stringListStringChar\n    input String inString;\n    output list<String> outStringList;\n  end stringListStringChar;\nend builtin;\nend Susan;:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _, pid, astDefs) = TplParser::interfacePackage(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, metamodelica::nil())?;
                    tequal = astDefs.clone() == list![TplAbsyn::ASTDef { importPackage: Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("builtin")).clone() }), isDefault: true, types: list![(literal!("stringListStringChar"), TplAbsyn::TypeInfo::TI_FUN_TYPE { inArgs: list![(literal!("inString"), Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE))], outArgs: list![(literal!("outStringList"), Arc::new(TplAbsyn::TypeSignature::LIST_TYPE { ofType: Arc::new(crate::TplAbsyn::TypeSignature::STRING_TYPE) }))], tyVars: metamodelica::nil() })] }] && pid.clone() == Arc::new(TplAbsyn::PathIdent::IDENT { ident: (literal!("Susan")).clone() });
                    txt = emptyTxt.clone();
                    txt = TplCodegen::pathIdent(txt.clone(), pid.clone())?;
                    let _ = Tpl::textString(txt.clone())?;
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*:)")).clone(), true, true, (literal!("TplParser.templPackage - function stringListStringChar\n")).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\ninterface package Susan\npackage builtin\n  function stringListStringChar\n    input String inString;\n    output list<String> outStringList;\n  end stringListStringChar;\nend builtin;\n\n\nprotected package Tpl\n  uniontype StringToken\n    record ST_NEW_LINE \"Always outputs the new-line char.\"  end ST_NEW_LINE;\n\n    record ST_STRING \"A string without new-lines in it.\"\n      String value;\n    end ST_STRING;\n\n    record ST_LINE \"A (non-empty) string with new-line at the end.\"\n      String line;\n    end ST_LINE;\n\n    record ST_STRING_LIST \"Every string in the list can have a new-line at its end (but does not have to).\"\n      list<String> strList;\n      Boolean lastHasNewLine \"True when the last string in the list has new-line at the end.\";\n    end ST_STRING_LIST;\n  end StringToken;\nend Tpl;\n\n\npackage TplAbsyn\n  type Ident = String;\n  type TypedIdents = list<tuple<Ident, TypeSignature>>;\n  type StringToken = Tpl.StringToken;\n\n  uniontype PathIdent\n    record IDENT\n      Ident ident;\n    end IDENT;\n\n    record PATH_IDENT\n      Ident ident;\n      PathIdent path;\n    end PATH_IDENT;\n  end PathIdent;\n\n  uniontype TypeSignature\n    record LIST_TYPE\n      TypeSignature ofType;\n    end LIST_TYPE;\n\n    record ARRAY_TYPE  // one-dimensional arrays --> with only (safe) list behaviour\n      TypeSignature ofType;\n    end ARRAY_TYPE;\n\n    record OPTION_TYPE\n      TypeSignature ofType;\n    end OPTION_TYPE;\n\n    record TUPLE_TYPE\n      list<TypeSignature> ofTypes;\n    end TUPLE_TYPE;\n\n    record NAMED_TYPE \"key/path to a TypeInfo list from an AST definition\"\n      PathIdent name;\n    end NAMED_TYPE;\n\n    record STRING_TYPE  end STRING_TYPE;\n    record TEXT_TYPE    end TEXT_TYPE;\n    record STRING_TOKEN_TYPE \"Used only for internal string constants.\" end STRING_TOKEN_TYPE;\n\n    record INTEGER_TYPE end INTEGER_TYPE;\n    record REAL_TYPE    end REAL_TYPE;\n    record BOOLEAN_TYPE end BOOLEAN_TYPE;\n\n    record UNRESOLVED_TYPE \"Errorneous resolving type. Only used during elaboration phase.\"\n      String reason;\n    end UNRESOLVED_TYPE;\n  end TypeSignature;\n\n\n  uniontype MatchingExp\n    record BIND_AS_MATCH\n      Ident bindIdent;\n      MatchingExp matchingExp;\n    end BIND_AS_MATCH;\n\n    record BIND_MATCH\n      Ident bindIdent;\n    end BIND_MATCH;\n\n    record RECORD_MATCH\n      PathIdent tagName;\n      list<tuple<Ident, MatchingExp>> fieldMatchings;\n    end RECORD_MATCH;\n\n    record SOME_MATCH\n      MatchingExp value;\n    end SOME_MATCH;\n\n    record NONE_MATCH end NONE_MATCH;\n\n    record TUPLE_MATCH\n      list<MatchingExp> tupleArgs;\n    end TUPLE_MATCH;\n\n    record LIST_MATCH //non-empty list\n      list<MatchingExp> listElts;\n    end LIST_MATCH;\n\n    record LIST_CONS_MATCH\n      MatchingExp head;\n      MatchingExp rest;\n    end LIST_CONS_MATCH;\n\n    record STRING_MATCH\n      String value;\n    end STRING_MATCH;\n\n    record LITERAL_MATCH\n      String value;\n      TypeSignature litType; // only INTEGER_TYPE, REAL_TYPE or BOOLEAN_TYPE\n    end LITERAL_MATCH;\n\n    record REST_MATCH end REST_MATCH;\n  end MatchingExp;\n\n\n  // **** the (core) output AST\n\n  uniontype MMPackage\n    record MM_PACKAGE\n      PathIdent name;\n      list<MMDeclaration> mmDeclarations;\n    end MM_PACKAGE;\n  end MMPackage;\n\n  type MMMatchCase = tuple<list<MatchingExp>, TypedIdents, list<MMExp>>;\n\n  uniontype MMDeclaration\n    record MM_IMPORT\n      Boolean isPublic;\n      PathIdent packageName;\n    end MM_IMPORT;\n\n    record MM_STR_TOKEN_DECL\n      Boolean isPublic;\n      Ident name;\n      StringToken value;\n    end MM_STR_TOKEN_DECL;\n\n    record MM_LITERAL_DECL\n      Boolean isPublic;\n      Ident name;\n      String value;\n      TypeSignature litType;\n    end MM_LITERAL_DECL;\n\n\n    record MM_FUN\n      Boolean isPublic;\n      Ident name;\n      TypedIdents inArgs; //inTxt inclusive\n      TypedIdents outArgs; // outTxt + extra Texts\n      TypedIdents locals;\n      list<MMExp> statements;\n    end MM_FUN;\n  end MMDeclaration;\n\n  uniontype MMExp\n    record MM_ASSIGN\n      list<Ident> lhsArgs;\n      MMExp rhs;\n    end MM_ASSIGN;\n\n    record MM_FN_CALL\n      PathIdent fnName;\n      list<MMExp> args;\n    end MM_FN_CALL;\n\n    record MM_IDENT\n      PathIdent ident;\n    end MM_IDENT;\n\n    record MM_STR_TOKEN \"constructor of type StringToken\"\n      StringToken value;\n    end MM_STR_TOKEN;\n\n    record MM_STRING \"to pass a string constant as parameter of type String\"\n      String value;\n    end MM_STRING;\n\n    record MM_LITERAL \"to pass a literal constant as parameter of type Integer, Real or Boolean\"\n      String value;\n    end MM_LITERAL;\n\n    record MM_MATCH\n      list<MMMatchCase> matchCases;\n    end MM_MATCH;\n  end MMExp;\nend TplAbsyn;\nend Susan;:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    (chars, _, pid, _) = TplParser::interfacePackage(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, metamodelica::nil())?;
                    txt = emptyTxt.clone();
                    txt = TplCodegen::pathIdent(txt.clone(), pid.clone())?;
                    let _ = Tpl::textString(txt.clone())?;
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("parsed*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("parsed*:)")).clone(), true, true, (literal!("TplParser.templPackage - all types for Susan's backend\n")).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\"Susan\"~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa0, _, (__pa1, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa0.clone();
                    expB = __pa1.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Susan")).clone() }) });
                    let __pa2 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa2 } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa2.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*Susan*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\"\\n\"~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa3, _, (__pa4, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa3.clone();
                    expB = __pa4.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) });
                    let __pa5 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa5 } => __pa5.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa5.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*\n*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\",\\n\"~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa6, _, (__pa7, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa6.clone();
                    expB = __pa7.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(",\n")).clone() }) });
                    let __pa8 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa8 } => __pa8.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa8.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*,\n*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\"Susan\nis\\nfantastic!\"~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa9, _, (__pa10, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa9.clone();
                    expB = __pa10.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("Susan\n")).clone(), (literal!("is\n")).clone(), (literal!("fantastic!")).clone()], lastHasNewLine: false }) });
                    let __pa11 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa11 } => __pa11.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa11.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*Susan\nis\nfantastic!*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\"\nSusan\nis\\n new lined!\n\"~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa12, _, (__pa13, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa12.clone();
                    expB = __pa13.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\n")).clone(), (literal!("Susan\n")).clone(), (literal!("is\n")).clone(), (literal!(" new lined!\n")).clone()], lastHasNewLine: true }) });
                    let __pa14 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa14 } => __pa14.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa14.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*\nSusan\nis\n new lined!\n*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n>")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("<\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("1234567~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa15, _, (__pa16, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa15.clone();
                    expB = __pa16.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::LITERAL { value: (literal!("1234567")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE) });
                    let __pa17 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa17, litType: _ } => __pa17.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cval = __pa17.clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*1234567*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("- 1234567~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa18, _, (__pa19, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa18.clone();
                    expB = __pa19.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::LITERAL { value: (literal!("-1234567")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::INTEGER_TYPE) });
                    let __pa20 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa20, litType: _ } => __pa20.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cval = __pa20.clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*-1234567*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("- 1234567.0123e-12~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa21, _, (__pa22, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa21.clone();
                    expB = __pa22.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::LITERAL { value: (literal!("-1234567.0123e-12")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE) });
                    let __pa23 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa23, litType: _ } => __pa23.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cval = __pa23.clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*-1234567.0123e-12*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!(".0123E12~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa24, _, (__pa25, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa24.clone();
                    expB = __pa25.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::LITERAL { value: (literal!(".0123E12")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::REAL_TYPE) });
                    let __pa26 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa26, litType: _ } => __pa26.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cval = __pa26.clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*.0123E12*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("true~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa27, _, (__pa28, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa27.clone();
                    expB = __pa28.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::LITERAL { value: (literal!("true")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::BOOLEAN_TYPE) });
                    let __pa29 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa29, litType: _ } => __pa29.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cval = __pa29.clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*true*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("false~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa30, _, (__pa31, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa30.clone();
                    expB = __pa31.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::LITERAL { value: (literal!("false")).clone(), litType: Arc::new(crate::TplAbsyn::TypeSignature::BOOLEAN_TYPE) });
                    let __pa32 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::LITERAL { value: __pa32, litType: _ } => __pa32.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cval = __pa32.clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*cval.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*false*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\\n~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa33, _, (__pa34, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa33.clone();
                    expB = __pa34.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) });
                    let __pa35 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa35 } => __pa35.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa35.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*\n*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("\\\"\\n\\n\\ ~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa36, _, (__pa37, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa36.clone();
                    expB = __pa37.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING_LIST { strList: list![(literal!("\"\n")).clone(), (literal!("\n")).clone(), (literal!(" ")).clone()], lastHasNewLine: false }) });
                    let __pa38 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa38 } => __pa38.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa38.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*\"\n\n *~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("'Susan'~:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    let (__pa39, _, (__pa40, _)) = TplParser::expression(chars.clone(), TplParser::makeStartLineInfo(chars.clone(), (literal!("in memory test")).clone())?, (literal!("<")).clone(), (literal!(">")).clone(), false)?;
                    chars = __pa39.clone();
                    expB = __pa40.clone();
                    tequal = expB.clone() == Arc::new(TplAbsyn::ExpressionBase::STR_TOKEN { value: Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("Susan")).clone() }) });
                    let __pa41 = ::match_deref::match_deref! { match &(expB.clone()) {
                        Deref @ TplAbsyn::ExpressionBase::STR_TOKEN { value: __pa41 } => __pa41.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    tok = __pa41.clone();
                    txt = emptyTxt.clone();
                    txt = Tpl::writeTok(txt.clone(), tok.clone())?;
                    strOut = (Tpl::textString(txt.clone())?).clone();
                    strOut = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::booleanString(tequal.clone())); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*strOut.clone()); __mm_s.push_str(&*literal!("*")); __mm_s.push_str(&*stringCharListString(chars.clone())); ArcStr::from(__mm_s) }).clone();
                    notPassedCnt = testStringEquality((strOut.clone()).clone(), (literal!("true*Susan*~:)")).clone(), true, true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TplParser.expression \n\"")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\"\n")); ArcStr::from(__mm_s) }).clone(), notPassedCnt.clone())?;
                    r#str = (literal!("Susan:)")).clone();
                    chars = stringListStringChar((r#str.clone()).clone());
                    llen = TplParser::charsTillEndOfLine(chars.clone(), 1)?;
                    let __pa42 = ::match_deref::match_deref! { match &(chars.clone()) {
                        Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: _, tail: __pa42 } } => __pa42.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    chars = __pa42.clone();
                    (lnum, colnum) = TplParser::getPosition(chars.clone(), TplParser::LineInfo { parseInfo: TplParser::ParseInfo { fileName: (literal!("test - no file")).clone(), errors: metamodelica::nil(), wasFatalError: false }, lineNumber: 11, lineLength: llen.clone(), startOfLineChars: chars.clone() })?;
                    notPassedCnt = testStringEquality(({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(lnum.clone())); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*intString(colnum.clone())); __mm_s.push_str(&*literal!(" of ")); __mm_s.push_str(&*intString(llen.clone())); ArcStr::from(__mm_s) }).clone(), (literal!("11,3 of 8")).clone(), true, true, (literal!("TplParser.charsTillEndOfLine and getPosition \n")).clone(), notPassedCnt.clone())?;
                    txt = emptyTxt.clone();
                    txt = statement(txt.clone(), Arc::new(Statement::WHILE { condition: Arc::new(Exp::BINARY { lhs: Arc::new(Exp::VARIABLE { name: (literal!("x")).clone() }), op: crate::TplMain::Operator::LESS, rhs: Arc::new(Exp::ICONST { value: 20 }) }), statements: list![Arc::new(Statement::ASSIGN { lhs: Arc::new(Exp::VARIABLE { name: (literal!("x")).clone() }), rhs: Arc::new(Exp::BINARY { lhs: Arc::new(Exp::VARIABLE { name: (literal!("x")).clone() }), op: crate::TplMain::Operator::PLUS, rhs: Arc::new(Exp::BINARY { lhs: Arc::new(Exp::VARIABLE { name: (literal!("y")).clone() }), op: crate::TplMain::Operator::TIMES, rhs: Arc::new(Exp::ICONST { value: 2 }) }) }) })] }))?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("while((x < 20)) {\n  x = (x + (y * 2));\n}")).clone(), true, true, (literal!("Paper Example statement()")).clone(), notPassedCnt.clone())?;
                    txt = emptyTxt.clone();
                    txt = intMatrix(txt.clone(), list![list![1, 2, 3, 4, 5], list![6, 7, 8, 9, 10], list![11, 12, 13, 14, 15]])?;
                    r#str = (Tpl::textString(txt.clone())?).clone();
                    notPassedCnt = testStringEquality((r#str.clone()).clone(), (literal!("[ 1, 2, 3, 4, 5;\n  6, 7, 8, 9, 10;\n  11, 12, 13, 14, 15 ]")).clone(), true, true, (literal!("intMatrix() from test.tpl")).clone(), notPassedCnt.clone())?;
                    notPassedCnt = testTranslateTplFile((literal!("TplCodegen")).clone(), false, false, notPassedCnt.clone())?;
                    notPassedCnt = testTranslateTplFile((literal!("paper")).clone(), false, false, notPassedCnt.clone())?;
                    notPassedCnt = testTranslateTplFile((literal!("test")).clone(), false, true, notPassedCnt.clone())?;
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("All tests took ")); __mm_s.push_str(&*realString(clock() - tstart.clone())); __mm_s.push_str(&*literal!(" seconds.\n")); ArcStr::from(__mm_s) }).clone());
                    r#str = (if (notPassedCnt.clone() == 0) {literal!("\n ***** All a) tests OK *****\n\n")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n #### ")); __mm_s.push_str(&*intString(notPassedCnt.clone())); __mm_s.push_str(&*literal!(" test")); __mm_s.push_str(&*if (notPassedCnt.clone() > 1) {literal!("s")} else {literal!("")}); __mm_s.push_str(&*literal!(" DID NOT passed ####\n\n")); ArcStr::from(__mm_s) }}).clone();
                    println!("{}", (r#str.clone()).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                r#str => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n######## tplMainTest '")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("' (fatally) failed!  ########\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", (literal!("### Error Buffer ###\n")).clone());
                    println!("{}", (Print::getErrorString()?).clone());
                    println!("{}", (literal!("\n### End of Error Buffer ###\n")).clone());
                    Print::clearErrorBuf();
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

/* the paper example */
/// Algorithmic stmts
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Statement {
    /// An assignment stmt
    ASSIGN {
        lhs: Arc<Exp>,
        rhs: Arc<Exp>,
    },
    /// A while statement
    WHILE {
        condition: Arc<Exp>,
        statements: Arc<metamodelica::List<Arc<Statement>>>,
    },
}
impl Default for Statement {
    fn default() -> Self {
        Self::ASSIGN {
            lhs: Default::default(),
            rhs: Default::default(),
        }
    }
}
pub use self::Statement::{ASSIGN,WHILE};

/// Expression nodes
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exp {
    /// Integer constant value
    ICONST {
        value: i32,
    },
    /// Variable reference
    VARIABLE {
        name: ArcStr,
    },
    /// Binary ops
    BINARY {
        lhs: Arc<Exp>,
        op: Operator,
        rhs: Arc<Exp>,
    },
}
impl Default for Exp {
    fn default() -> Self {
        Self::ICONST {
            value: Default::default(),
        }
    }
}
pub use self::Exp::{ICONST,VARIABLE,BINARY};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operator {
    PLUS,
    TIMES,
    LESS,
}
impl Default for Operator {
    fn default() -> Self { Self::PLUS }
}
pub use self::Operator::{PLUS,TIMES,LESS};

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_1(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<Statement>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), in_items.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Nil) => {
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = statement(txt.clone(), i_it.clone())?;
                    txt = Tpl::nextIter(txt.clone())?;
                    txt = lm_1(txt.clone(), rest.clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = lm_1(txt.clone(), rest.clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_txt)
}

pub fn statement(mut in_txt: Tpl::Text, mut in_i_it: Arc<Statement>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), in_i_it.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Statement::ASSIGN { rhs: i_rhs, lhs: i_lhs }) => {
                    let mut txt = (*txt).clone();
                    txt = exp(txt.clone(), i_lhs.clone())?;
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" = ")).clone() }))?;
                    txt = exp(txt.clone(), i_rhs.clone())?;
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(";")).clone() }))?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Statement::WHILE { statements: i_statements, condition: i_condition }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("while(")).clone() }))?;
                    txt = exp(txt.clone(), i_condition.clone())?;
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(") {\n")).clone() }))?;
                    txt = Tpl::pushBlock(txt.clone(), Arc::new(Tpl::BlockType::BT_INDENT { width: 2 }))?;
                    txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(crate::Tpl::StringToken::ST_NEW_LINE)), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
                    txt = lm_1(txt.clone(), i_statements.clone())?;
                    txt = Tpl::popIter(txt.clone())?;
                    txt = Tpl::softNewLine(txt.clone())?;
                    txt = Tpl::popBlock(txt.clone())?;
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("}")).clone() }))?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, _) => {
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_txt)
}

pub fn exp(mut in_txt: Tpl::Text, mut in_i_it: Arc<Exp>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), in_i_it.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Exp::ICONST { value: i_value }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeStr(txt.clone(), (intString(i_value.clone())).clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Exp::VARIABLE { name: i_name }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeStr(txt.clone(), (i_name.clone()).clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ Exp::BINARY { rhs: i_rhs, op: i_op, lhs: i_lhs }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("(")).clone() }))?;
                    txt = exp(txt.clone(), i_lhs.clone())?;
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
                    txt = oper(txt.clone(), i_op.clone())?;
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ")).clone() }))?;
                    txt = exp(txt.clone(), i_rhs.clone())?;
                    txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(")")).clone() }))?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(in_txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_txt)
}

pub fn oper(mut in_txt: Tpl::Text, mut in_i_it: Operator) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), in_i_it.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, Operator::PLUS { .. }) = __mc_input.clone() else { bail!("nomatch") };
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("+")).clone() }))?;
            Ok(txt.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, Operator::TIMES { .. }) = __mc_input.clone() else { bail!("nomatch") };
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("*")).clone() }))?;
            Ok(txt.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut txt, Operator::LESS { .. }) = __mc_input.clone() else { bail!("nomatch") };
            txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("<")).clone() }))?;
            Ok(txt.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(in_txt.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_txt)
}

/* **************************/
/* intMatrix from test.tpl */
/* **************************/
// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_54(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<i32>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), in_items.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Nil) => {
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Cons { head: i_it, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::writeStr(txt.clone(), (intString(i_it.clone())).clone())?;
                    txt = Tpl::nextIter(txt.clone())?;
                    txt = lm_54(txt.clone(), rest.clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = lm_54(txt.clone(), rest.clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_txt)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn lm_55(mut in_txt: Tpl::Text, mut in_items: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = 'mc: {
        let __mc_input = (in_txt.clone(), in_items.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Nil) => {
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Cons { head: i_intLst, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = Tpl::pushIter(txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(", ")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
                    txt = lm_54(txt.clone(), i_intLst.clone())?;
                    txt = Tpl::popIter(txt.clone())?;
                    txt = Tpl::nextIter(txt.clone())?;
                    txt = lm_55(txt.clone(), rest.clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (txt, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut txt = (*txt).clone();
                    txt = lm_55(txt.clone(), rest.clone())?;
                    Ok(txt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_txt)
}

pub fn intMatrix(mut txt: Tpl::Text, mut i_lstOfLst: Arc<metamodelica::List<Arc<metamodelica::List<i32>>>>) -> Result<Tpl::Text> {
    let mut out_txt: Tpl::Text;
    out_txt = Tpl::writeTok(txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!("[ ")).clone() }))?;
    out_txt = Tpl::pushBlock(out_txt.clone(), Arc::new(Tpl::BlockType::BT_ANCHOR { offset: 0 }))?;
    out_txt = Tpl::pushIter(out_txt.clone(), Arc::new(Tpl::IterOptions { startIndex0: 0, empty: None, separator: Some(Arc::new(Tpl::StringToken::ST_LINE { line: (literal!(";\n")).clone() })), alignNum: 0, alignOfset: 0, alignSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE), wrapWidth: 0, wrapSeparator: Arc::new(crate::Tpl::StringToken::ST_NEW_LINE) }))?;
    out_txt = lm_55(out_txt.clone(), i_lstOfLst.clone())?;
    out_txt = Tpl::popIter(out_txt.clone())?;
    out_txt = Tpl::popBlock(out_txt.clone())?;
    out_txt = Tpl::writeTok(out_txt.clone(), Arc::new(Tpl::StringToken::ST_STRING { value: (literal!(" ]")).clone() }))?;
    Ok(out_txt)
}

/* **************************/
/* end of intMatrix from test.tpl */
/* **************************/
// !!! weird type behavior of MM
/*
public
function MuchFun2
  input Tpl.Text txt;
  input Integer inlaughLevel;
  input list<String> injokes;

  output Integer txt;

  Tpl.Text txt1;
  Integer laughLevel;
  list<String> jokes;
algorithm
(txt) := Tpl.writeStr(txt, "Susan");
txt := 1;
//(txt1) := Tpl.writeStr(txt, "Susan");

//(txt) := Tpl.writeTok(txt, Tpl.ST_LINE("Susan is cosmic!\n"));
end MuchFun2;
*/
