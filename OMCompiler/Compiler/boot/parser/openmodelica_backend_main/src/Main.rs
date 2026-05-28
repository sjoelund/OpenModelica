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

use crate::BackendInterfaceImplementation;
use crate::CevalScript;
use crate::CevalScriptBackend;
use crate::Interactive;
use crate::InteractiveUtil;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_backend::SymbolTable;
use openmodelica_dump_extra::AbsynJLDumpTpl;
use openmodelica_dump_extra::DumpGraphviz;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::Parser;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_program_util::ProgramUtil;
use openmodelica_susan::Tpl;
use openmodelica_susan::TplMain;
use openmodelica_util::Autoconf;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Config;
use openmodelica_util::Corba;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::Socket;
use openmodelica_util::StackOverflow;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util::ZeroMQ;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

fn makeDebugResult(mut inFlag: Flags::DebugFlag, mut res: ArcStr) -> Result<ArcStr> {
    let mut res_1: ArcStr = arcstr::literal!("");
    res_1 = ('mc: {
        let __mc_input = inFlag.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let Flags::DebugFlag { name: mut flagstr, .. } = __mc_input.clone() else { bail!("nomatch") };
            let mut debugstr: ArcStr = arcstr::literal!("");
            let mut res_with_debug: ArcStr = arcstr::literal!("");
            let true = (Flags::isSet(inFlag.clone())?) else { bail!("pattern mismatch") };
            debugstr = (Print::getString()?).clone();
            res_with_debug = stringAppendList(list![(res.clone()).clone(), (literal!("\n---DEBUG(")).clone(), (flagstr.clone()).clone(), (literal!(")---\n")).clone(), (debugstr.clone()).clone(), (literal!("\n---/DEBUG(")).clone(), (flagstr.clone()).clone(), (literal!(")---\n")).clone()]);
            Ok(res_with_debug.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(res.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(res_1)
}

fn parseCommand(mut inCommand: ArcStr) -> Result<(Option<GlobalScript::Statements>, Option<Absyn::Program>)> {
    let mut outStatements: Option<GlobalScript::Statements> = None;
    let mut outProgram: Option<Absyn::Program> = None;
    (outStatements, outProgram) = 'mc: {
        let __mc_input = inCommand.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut stmts: GlobalScript::Statements = <GlobalScript::Statements as ::std::default::Default>::default();
            ErrorExt::setCheckpoint((literal!("parsestring")).clone());
            stmts = Parser::parsestringexp((inCommand.clone()).clone(), (literal!("<interactive>")).clone())?;
            ErrorExt::delCheckpoint((literal!("parsestring")).clone());
            Ok((Some(stmts.clone()), None))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut prog: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            ErrorExt::rollBack((literal!("parsestring")).clone());
            prog = Parser::parsestring((inCommand.clone()).clone(), (literal!("<interactive>")).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
            Ok((None, Some(prog.clone())))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok((None, None))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStatements, outProgram))
}

pub fn handleCommand(mut inCommand: ArcStr) -> Result<(bool, ArcStr)> {
    let mut outContinue: bool = false;
    let mut outResult: ArcStr = arcstr::literal!("");
    let mut stmts: Option<GlobalScript::Statements> = None;
    let mut prog: Option<Absyn::Program> = None;
    Print::clearBuf();
    if Util::strncmp((literal!("quit()")).clone(), (inCommand.clone()).clone(), 6) {
        outContinue = false;
        outResult = (literal!("Ok\n")).clone();
    } else {
        outContinue = true;
        (stmts, prog) = parseCommand((inCommand.clone()).clone())?;
        outResult = (handleCommand2(stmts.clone(), prog.clone(), (inCommand.clone()).clone())?).clone();
        outResult = (makeDebugResult(Flags::DUMP.clone(), (outResult.clone()).clone())?).clone();
        outResult = (makeDebugResult(Flags::DUMP_GRAPHVIZ.clone(), (outResult.clone()).clone())?).clone();
    }
    Ok((outContinue, outResult))
}

fn handleCommand2(mut inStatements: Option<GlobalScript::Statements>, mut inProgram: Option<Absyn::Program>, mut inCommand: ArcStr) -> Result<ArcStr> {
    let mut outResult: ArcStr = arcstr::literal!("");
    outResult = ('mc: {
        let __mc_input = (inStatements.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (Some(mut stmts), None) = __mc_input.clone() else { bail!("nomatch") };
            let mut result: ArcStr = arcstr::literal!("");
            result = (Interactive::evaluate(stmts.clone(), false)?).clone();
            Ok(result.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, Some(mut prog)) = __mc_input.clone() else { bail!("nomatch") };
            let mut prog2: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut ast: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut result: ArcStr = arcstr::literal!("");
            let mut vars: Arc<metamodelica::List</* ? */>> = metamodelica::nil();
            let mut table: Arc<SymbolTable::SymbolTable>;
            table = SymbolTable::get();
            ast = table.ast.clone();
            vars = table.vars.clone();
            prog2 = Interactive::addScope(prog.clone(), vars.clone())?;
            prog2 = ProgramUtil::updateProgram(prog2.clone(), ast.clone(), false)?;
            if Flags::isSet(Flags::DUMP.clone())? {
                Debug::trace((literal!("\n--------------- Parsed program ---------------\n")).clone())?;
                Print::printBuf((Dump::unparseStr(prog2.clone(), false, Dump::defaultDumpOptions.clone())?).clone())?;
            }
            if Flags::isSet(Flags::DUMP_GRAPHVIZ.clone())? {
                DumpGraphviz::dump(prog2.clone())?;
            }
            result = (makeClassDefResult(prog.clone())?).clone();
            SymbolTable::setAbsyn(prog2.clone())?;
            Ok(result.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (None, None) = __mc_input.clone() else { bail!("nomatch") };
            let mut result: ArcStr = arcstr::literal!("");
            Print::printBuf((literal!("Error occurred building AST\n")).clone())?;
            result = (Print::getString()?).clone();
            result = (stringAppend((result.clone()).clone(), (literal!("Syntax Error\n")).clone())).clone();
            result = (stringAppend((result.clone()).clone(), (Error::printMessagesStr(false)).clone())).clone();
            Ok(result.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut result: ArcStr = arcstr::literal!("");
            let true = (isSome(inStatements.clone()) || Util::isSome(inProgram.clone())) else { bail!("pattern mismatch") };
            result = (Error::printMessagesStr(false)).clone();
            Ok(result.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let true = (isSome(inStatements.clone()) || Util::isSome(inProgram.clone())) else { bail!("pattern mismatch") };
            Error::addMessage(Error::STACK_OVERFLOW.clone(), list![(inCommand.clone()).clone()])?;
            Ok(literal!(""))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outResult)
}

fn makeClassDefResult(mut p: Absyn::Program) -> Result<ArcStr> {
    let mut res: ArcStr = arcstr::literal!("");
    res = ((match p.clone() {
        Absyn::Program { within_: Absyn::Within::WITHIN { path: ref scope }, classes: ref cls } => {
            let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            names = ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut c in (cls.clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::Path::IDENT { name: (AbsynUtil::className(c.clone())?).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            names = List::map1(names.clone(), (std::sync::Arc::new(AbsynUtil::joinPaths) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>), scope.clone());
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (names.clone()).into_iter().cloned() {
            let __x = AbsynUtil::pathString(n.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        Absyn::Program { within_: Absyn::Within::TOP, classes: ref cls } => {
            let mut names: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            names = ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut c in (cls.clone()).into_iter().cloned() {
            let __x = Arc::new(Absyn::Path::IDENT { name: (AbsynUtil::className(c.clone())?).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*stringDelimitList(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (names.clone()).into_iter().cloned() {
            let __x = AbsynUtil::pathString(n.clone(), (literal!(".")).clone(), true, false)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (literal!(",")).clone())); __mm_s.push_str(&*literal!("}\n")); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(res)
}

fn isModelicaFile(mut inFilename: ArcStr) -> Result<bool> {
    let mut outIsModelicaFile: bool = false;
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut file_ext: ArcStr = arcstr::literal!("");
    lst = System::strtok((inFilename.clone()).clone(), (literal!(".")).clone());
    if lst.clone().is_empty() {
        outIsModelicaFile = false;
    } else {
        file_ext = (List::last(lst.clone())?).clone();
        outIsModelicaFile = file_ext.clone() == literal!("mo") || file_ext.clone() == literal!("mof");
    }
    Ok(outIsModelicaFile)
}

fn isEmptyOrFirstIsModelicaFile(mut libs: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(libs.clone()) {
        Deref @ metamodelica::List::Nil => {
            ()
        },
        Deref @ metamodelica::List::Cons { head: f, tail: _ } => {
            let true = (isModelicaFile((f.clone()).clone())?) else { bail!("pattern mismatch") };
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn isFlatModelicaFile(mut filename: ArcStr) -> Result<()> {
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut last: ArcStr = arcstr::literal!("");
    lst = System::strtok((filename.clone()).clone(), (literal!(".")).clone());
    let __pa0 = ::match_deref::match_deref! { match &(lst.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    last = __pa0.clone();
    let true = (stringEq((last.clone()).clone(), (literal!("mof")).clone())?) else { bail!("pattern mismatch") };
    Ok(())
}

fn isModelicaScriptFile(mut filename: ArcStr) -> Result<()> {
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut last: ArcStr = arcstr::literal!("");
    let true = (System::regularFileExists((filename.clone()).clone())) else { bail!("pattern mismatch") };
    lst = System::strtok((filename.clone()).clone(), (literal!(".")).clone());
    let __pa0 = ::match_deref::match_deref! { match &(lst.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    last = __pa0.clone();
    let true = (stringEq((last.clone()).clone(), (literal!("mos")).clone())?) else { bail!("pattern mismatch") };
    Ok(())
}

fn isCodegenTemplateFile(mut filename: ArcStr) -> Result<()> {
    let mut lst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut last: ArcStr = arcstr::literal!("");
    lst = System::strtok((filename.clone()).clone(), (literal!(".")).clone());
    let __pa0 = ::match_deref::match_deref! { match &(lst.clone().reverse()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    last = __pa0.clone();
    let true = (stringEq((last.clone()).clone(), (literal!("tpl")).clone())?) else { bail!("pattern mismatch") };
    Ok(())
}

fn showErrors(mut errorString: ArcStr, mut errorMessages: ArcStr) -> () {
    if errorString.clone() != literal!("") {
        System::fflush();
        System::fputs((errorString.clone()).clone(), System::StreamType::STDERR.clone());
        System::fputs((literal!("\n")).clone(), System::StreamType::STDERR.clone());
        System::fflush();
    }
    if errorMessages.clone() != literal!("") {
        System::fflush();
        System::fputs((errorMessages.clone()).clone(), System::StreamType::STDERR.clone());
        System::fputs((literal!("\n")).clone(), System::StreamType::STDERR.clone());
        System::fflush();
    }
    ()
}

fn loadLib(mut inLib: ArcStr) -> Result<()> {
    let mut is_modelica_file: bool = false;
    is_modelica_file = isModelicaFile((inLib.clone()).clone())?;
    let () = 'mc: {
        let __mc_input = is_modelica_file.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else { bail!("nomatch") };
            let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            p = SymbolTable::getAbsyn();
            pnew = CevalScript::loadFile((inLib.clone()).clone(), (literal!("UTF-8")).clone(), p.clone(), true, true, false, true)?;
            SymbolTable::setAbsyn(pnew.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else { bail!("nomatch") };
            let mut mp: ArcStr = arcstr::literal!("");
            let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut path: Arc<Absyn::Path>;
            path = AbsynUtil::stringPath((inLib.clone()).clone())?;
            mp = (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone();
            p = SymbolTable::getAbsyn();
            let (__pa0, true) = (CevalScript::loadModel(list![(path.clone(), literal!("command-line argument"), list![(literal!("default")).clone()], false)], (mp.clone()).clone(), p.clone(), true, true, true, false, false, (literal!("")).clone())?) else { bail!("pattern mismatch") };
            pnew = __pa0.clone();
            SymbolTable::setAbsyn(pnew.clone())?;
            Ok(())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let false = __mc_input.clone() else { bail!("nomatch") };
            Print::printErrorBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to load library: ")); __mm_s.push_str(&*inLib.clone()); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let true = __mc_input.clone() else { bail!("nomatch") };
            Print::printErrorBuf(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to parse file: ")); __mm_s.push_str(&*inLib.clone()); __mm_s.push_str(&*literal!("!\n")); ArcStr::from(__mm_s) }).clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn translateFile(mut inStringLst: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut f: ArcStr = arcstr::literal!("");
    let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut cname: Arc<Absyn::Path>;
    let mut runBackend: bool = false;
    let mut runSilent: bool = false;
    let mut stmts: GlobalScript::Statements = <GlobalScript::Statements as ::std::default::Default>::default();
    let mut cls: ArcStr = arcstr::literal!("");
    let mut fileNamePrefix: ArcStr = arcstr::literal!("");
    if !(stringEmpty((Flags::getConfigString(Flags::EXECUTE_COMMAND.clone())?).clone())) {
        stmts = Parser::parsestringexp((Flags::getConfigString(Flags::EXECUTE_COMMAND.clone())?).clone(), (literal!("<interactive>")).clone())?;
        showErrors((Print::getErrorString()?).clone(), (ErrorExt::printMessagesStr(false)).clone());
        Interactive::evaluateToStdOut(stmts.clone(), true)?;
        if inStringLst.clone().is_empty() && stringEmpty((Config::classToInstantiate()?).clone()) {
            return Ok(());
        }
    }
    let () = 'mc: {
        let __mc_input = inStringLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                libs => {
                    let mut fileNamePrefix: ArcStr = fileNamePrefix.clone();
                    let mut cname: Arc<Absyn::Path>;
                    let mut runBackend: bool = runBackend.clone();
                    let mut runSilent: bool = runSilent.clone();
                    let mut cls: ArcStr = cls.clone();
                    isEmptyOrFirstIsModelicaFile(libs.clone())?;
                    execStatReset()?;
                    for mut lib in &*libs.clone() {
                        let mut lib = lib.clone();
                        loadLib((lib.clone()).clone())?;
                    }
                    if Flags::isSet(Flags::DUMP.clone())? {
                        Debug::trace((literal!("\n--------------- Parsed program ---------------\n")).clone())?;
                        Dump::unparseStr(SymbolTable::getAbsyn(), false, Dump::defaultDumpOptions.clone())?;
                        println!("{}", (Print::getString()?).clone());
                    }
                    if Flags::isSet(Flags::DUMP_JL.clone())? {
                        Debug::trace((literal!("\n--------------- Julia representation of the parsed program ---------------\n")).clone())?;
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Tpl::tplString((std::sync::Arc::new(AbsynJLDumpTpl::dump) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Absyn::Program) -> Result<Tpl::Text> + 'static>), SymbolTable::getAbsyn())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    if Flags::isSet(Flags::DUMP_GRAPHVIZ.clone())? {
                        DumpGraphviz::dump(SymbolTable::getAbsyn())?;
                    }
                    execStat((literal!("Parsed file")).clone())?;
                    cls = (Config::classToInstantiate()?).clone();
                    cname = if (stringEmpty((cls.clone()).clone())) {AbsynUtil::lastClassname(SymbolTable::getAbsyn())?} else {AbsynUtil::stringPath((cls.clone()).clone())?};
                    fileNamePrefix = (Util::stringReplaceChar((AbsynUtil::pathString(cname.clone(), (literal!(".")).clone(), true, false)?).clone(), (literal!(".")).clone(), (literal!("_")).clone())?).clone();
                    runBackend = Config::simulationCg()? || Config::simulation()?;
                    runSilent = Config::silent()?;
                    CevalScriptBackend::translateModel(FCore::emptyCache(), FGraph::empty(), cname.clone(), (fileNamePrefix.clone()).clone(), runBackend.clone(), runSilent.clone(), None)?;
                    showErrors((Print::getErrorString()?).clone(), (ErrorExt::printMessagesStr(false)).clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: f, tail: libs } => {
                    let mut stmts: GlobalScript::Statements = stmts.clone();
                    isModelicaScriptFile((f.clone()).clone())?;
                    for mut lib in &*libs.clone() {
                        let mut lib = lib.clone();
                        loadLib((lib.clone()).clone())?;
                    }
                    stmts = Parser::parseexp((f.clone()).clone())?;
                    showErrors((Print::getErrorString()?).clone(), (ErrorExt::printMessagesStr(false)).clone());
                    Interactive::evaluateToStdOut(stmts.clone(), true)?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: f, tail: Deref @ metamodelica::List::Nil } => {
                    isCodegenTemplateFile((f.clone()).clone())?;
                    TplMain::main((f.clone()).clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: f, tail: _ } => {
                    if System::regularFileExists((f.clone()).clone()) {
                        println!("{}", (literal!("Error processing file: ")).clone());
                    } else {
                        println!("{}", (System::gettext((literal!("File does not exist: ")).clone())).clone());
                    }
                    println!("{}", (f.clone()).clone());
                    println!("{}", (literal!("\n")).clone());
                    System::fflush();
                    showErrors((Print::getErrorString()?).clone(), (ErrorExt::printMessagesStr(false)).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn interactivemode() -> Result<()> {
    let mut shandle: i32 = 0;
    let mut b: bool = false;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut replystr: ArcStr = arcstr::literal!("");
    shandle = Socket::waitforconnect(29500);
    if shandle.clone() == -1 {
        bail!("fail");
    }
    loop {
        r#str = (Socket::handlerequest(shandle.clone())).clone();
        if Flags::isSet(Flags::INTERACTIVE_DUMP.clone())? {
            Debug::trace((literal!("------- Recieved Data from client -----\n")).clone())?;
            Debug::trace((r#str.clone()).clone())?;
            Debug::trace((literal!("------- End recieved Data-----\n")).clone())?;
        }
        (b, replystr) = handleCommand((r#str.clone()).clone())?;
        replystr = (if (b.clone()) {replystr.clone()} else {literal!("quit requested, shutting server down\n")}).clone();
        Socket::sendreply(shandle.clone(), (replystr.clone()).clone());
        if !(b.clone()) {
            Socket::close(shandle.clone());
            Socket::cleanup();
            break;
        }
    }
    Ok(())
}

fn interactivemodeCorba() -> Result<()> {
    if '__try0: {
        unwrap_break_err!(Corba::initialize(), '__try0);
        unwrap_break_err!(serverLoopCorba(), '__try0);
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        Print::printBuf((literal!("Failed to initialize Corba! Is another OMC already running?\n")).clone())?;
        Print::printBuf((literal!("Exiting!\n")).clone())?;
    }
    Ok(())
}

fn interactivemodeZMQ() -> Result<()> {
    let mut zmqSocket: Option<i32> = None;
    let mut b: bool = false;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut replystr: ArcStr = arcstr::literal!("");
    let mut suffix: ArcStr = arcstr::literal!("");
    suffix = (Flags::getConfigString(Flags::ZEROMQ_FILE_SUFFIX.clone())?).clone();
    zmqSocket = ZeroMQ::initialize((if (suffix.clone() == literal!("")) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*suffix.clone()); ArcStr::from(__mm_s) }}).clone(), Flags::isSet(Flags::ZMQ_LISTEN_TO_ALL.clone())?, Flags::getConfigInt(Flags::INTERACTIVE_PORT.clone())?);
    let false = (Some(0) == zmqSocket.clone()) else { bail!("pattern mismatch") };
    loop {
        r#str = (ZeroMQ::handleRequest(zmqSocket.clone())).clone();
        if Flags::isSet(Flags::INTERACTIVE_DUMP.clone())? {
            Debug::trace((literal!("------- Recieved Data from client -----\n")).clone())?;
            Debug::trace((r#str.clone()).clone())?;
            Debug::trace((literal!("------- End recieved Data-----\n")).clone())?;
        }
        (b, replystr) = handleCommand((r#str.clone()).clone())?;
        replystr = (if (b.clone()) {replystr.clone()} else {literal!("quit requested, shutting server down\n")}).clone();
        ZeroMQ::sendReply(zmqSocket.clone(), (replystr.clone()).clone());
        if !(b.clone()) {
            ZeroMQ::close(zmqSocket.clone());
            break;
        }
    }
    Ok(())
}

fn serverLoopCorba() -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut reply_str: ArcStr = arcstr::literal!("");
    let mut cont: bool = false;
    cont = true;
    loop {
        r#str = (Corba::waitForCommand()?).clone();
        (cont, reply_str) = handleCommand((r#str.clone()).clone())?;
        if cont.clone() {
            Corba::sendreply((reply_str.clone()).clone())?;
        } else {
            break;
        }
    }
    Corba::sendreply((literal!("quit requested, shutting server down\n")).clone())?;
    Corba::close()?;
    Ok(())
}

pub fn readSettings(mut inArguments: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut settings_file: ArcStr = arcstr::literal!("");
    settings_file = (Util::flagValue((literal!("-s")).clone(), inArguments.clone())?).clone();
    if settings_file.clone() != literal!("") {
        settings_file = (System::trim((settings_file.clone()).clone(), (literal!(" \"")).clone())).clone();
        readSettingsFile((settings_file.clone()).clone())?;
    }
    Ok(())
}

fn readSettingsFile(mut filePath: ArcStr) -> Result<()> {
    let mut command: ArcStr = arcstr::literal!("");
    if System::regularFileExists((filePath.clone()).clone()) {
        command = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("runScript(\"")); __mm_s.push_str(&*filePath.clone()); __mm_s.push_str(&*literal!("\")")); ArcStr::from(__mm_s) }).clone();
        handleCommand((command.clone()).clone())?;
    }
    Ok(())
}

pub fn setWindowsPaths(mut inOMHome: ArcStr) -> Result<()> {
    let () = (match inOMHome.clone() {
        mut omHome => {
            let mut oldPath: ArcStr = arcstr::literal!("");
            let mut newPath: ArcStr = arcstr::literal!("");
            let mut omdevPath: ArcStr = arcstr::literal!("");
            let mut msysPath: ArcStr = arcstr::literal!("");
            let mut mingwDir: ArcStr = arcstr::literal!("");
            let mut binDir: ArcStr = arcstr::literal!("");
            let mut libBinDir: ArcStr = arcstr::literal!("");
            let mut msysBinDir: ArcStr = arcstr::literal!("");
            let mut hasBinDir: bool = false;
            let mut hasLibBinDir: bool = false;
            System::setEnv((literal!("OPENMODELICAHOME")).clone(), (omHome.clone()).clone(), true);
            omdevPath = (Util::makeValueOrDefault((std::sync::Arc::new(System::readEnv) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>), (literal!("OMDEV")).clone(), (literal!("")).clone())).clone();
            if stringEq((omdevPath.clone()).clone(), (literal!("")).clone())? {
                omdevPath = (omHome.clone()).clone();
            }
            msysPath = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*omdevPath.clone()); __mm_s.push_str(&*literal!("\\tools\\msys")); ArcStr::from(__mm_s) }).clone();
            mingwDir = (System::openModelicaPlatform()).clone();
            msysBinDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msysPath.clone()); __mm_s.push_str(&*literal!("\\usr\\bin")); ArcStr::from(__mm_s) }).clone();
            binDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msysPath.clone()); __mm_s.push_str(&*literal!("\\")); __mm_s.push_str(&*mingwDir.clone()); __mm_s.push_str(&*literal!("\\bin")); ArcStr::from(__mm_s) }).clone();
            if System::getCCompiler() == literal!("gcc") {
                libBinDir = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msysPath.clone()); __mm_s.push_str(&*literal!("\\")); __mm_s.push_str(&*mingwDir.clone()); __mm_s.push_str(&*literal!("\\lib\\gcc\\")); __mm_s.push_str(&*System::gccDumpMachine()); __mm_s.push_str(&*literal!("\\")); __mm_s.push_str(&*System::gccVersion()); ArcStr::from(__mm_s) }).clone();
            } else {
                libBinDir = (binDir.clone()).clone();
            }
            hasBinDir = System::directoryExists((binDir.clone()).clone());
            hasLibBinDir = System::directoryExists((libBinDir.clone()).clone());
            if hasBinDir.clone() && hasLibBinDir.clone() {
                oldPath = (System::readEnv((literal!("PATH")).clone())?).clone();
                newPath = stringAppendList(list![(omHome.clone()).clone(), (literal!("\\bin;")).clone(), (omHome.clone()).clone(), (literal!("\\lib;")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*binDir.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*libBinDir.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*msysBinDir.clone()); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }).clone()]);
                newPath = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::stringReplace((newPath.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?); __mm_s.push_str(&*oldPath.clone()); ArcStr::from(__mm_s) }).clone();
                System::setEnv((literal!("PATH")).clone(), (newPath.clone()).clone(), true);
            } else {
                if !(Flags::isSet(Flags::DISABLE_WINDOWS_PATH_CHECK_WARNING.clone())?) {
                    println!("{}", (literal!("We could not find some needed MINGW paths in $OPENMODELICAHOME or $OMDEV. Searched for paths:\n")).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*binDir.clone()); __mm_s.push_str(&*if (hasBinDir.clone()) {literal!(" [found] ")} else {literal!(" [not found] ")}); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\t")); __mm_s.push_str(&*libBinDir.clone()); __mm_s.push_str(&*if (hasLibBinDir.clone()) {literal!(" [found] ")} else {literal!(" [not found] ")}); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                }
            }
            ()
        },
    });
    Ok(())
}

fn setDefaultCC() -> () {
    if '__try0: {
        System::setCCompiler((System::readEnv((literal!("CC")).clone()).unwrap()).clone());
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    ()
}

pub fn init(mut args: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut args_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    System::setEnv((literal!("G_SLICE")).clone(), (literal!("always-malloc")).clone(), true);
    System::initGarbageCollector();
    if true {
        GCExt::setForceUnmapOnGcollect(arcstr::literal!(Autoconf::os) == literal!("Windows_NT"));
    } else {
        GCExt::expandHeap(metamodelica::OrderedFloat((if (arcstr::literal!(Autoconf::os) == literal!("Windows_NT")) {1024 * 1024 * 150} else {1024 * 1024 * 300}) as f64));
    }
    Global::initialize();
    ErrorExt::registerModelicaFormatError();
    ErrorExt::initAssertionFunctions();
    System::realtimeTick(ClockIndexes::RT_CLOCK_SIMULATE_TOTAL.clone())?;
    args_1 = FlagsUtil::new(args.clone())?;
    System::gettextInit((if (Testsuite::isRunning()?) {literal!("C")} else {Flags::getConfigString(Flags::LOCALE_FLAG.clone())?}).clone());
    setDefaultCC();
    SymbolTable::reset()?;
    BackendInterfaceImplementation::initializeBackendInterface();
    Ok(args_1)
}

pub fn main(mut args: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut args_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut seconds: i32 = 0;
    execStatReset()?;
    if '__try0: {
        match '__try1: {
            args_1 = unwrap_break_err!(init(args.clone()), '__try1);
            if unwrap_break_err!(Flags::isSet(Flags::GC_PROF.clone()), '__try1) {
                println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(GCExt::profStatsStr(GCExt::getProfStats(), (literal!("GC stats after initialization:")).clone(), (literal!("\n  ")).clone()), '__try1)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            }
            seconds = unwrap_break_err!(Flags::getConfigInt(Flags::ALARM.clone()), '__try1);
            if seconds.clone() > 0 {
                System::alarm(seconds.clone());
            }
            unwrap_break_err!(main2(args_1.clone()), '__try1);
            Ok::<_, anyhow::Error>((args_1.clone(), seconds.clone()))
        } {
            Ok((__try1_o0, __try1_o1)) => {
                args_1 = __try1_o0;
                seconds = __try1_o1;
            }
            Err(_) => {
                ErrorExt::clearMessages();
                if '__try2: {
                    unwrap_break_err!(FlagsUtil::new(args.clone()), '__try2);
                    Ok::<(), anyhow::Error>(())
                }.is_ok() { bail!("failure(): body succeeded") }
                println!("{}", (ErrorExt::printMessagesStr(false)).clone());
                println!("{}", (literal!("\n")).clone());
                break '__try0 Err::<_, _>(anyhow::anyhow!("fail"));
            }
        }
        if unwrap_break_err!(Flags::isSet(Flags::GC_PROF.clone()), '__try0) {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*unwrap_break_err!(GCExt::profStatsStr(GCExt::getProfStats(), (literal!("GC stats at end of program:")).clone(), (literal!("\n  ")).clone()), '__try0)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow detected and was not caught.\n")); __mm_s.push_str(&*literal!("Send us a bug report at https://trac.openmodelica.org/OpenModelica/newticket\n")); __mm_s.push_str(&*literal!("    Include the following trace:\n")); ArcStr::from(__mm_s) }).clone());
        for mut s in &*StackOverflow::readableStacktraceMessages()? {
            let mut s = s.clone();
            println!("{}", (s.clone()).clone());
            println!("{}", (literal!("\n")).clone());
        }
    }
    Ok(())
}

fn main2(mut args: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut interactiveMode: ArcStr = arcstr::literal!("");
    if Config::versionRequest()? {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getVersionNr()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        return Ok(());
    }
    interactiveMode = (Flags::getConfigString(Flags::INTERACTIVE.clone())?).clone();
    if System::userIsRoot() && (interactiveMode.clone() == literal!("corba") || interactiveMode.clone() == literal!("tcp") || interactiveMode.clone() == literal!("zmq")) {
        Error::addMessage(Error::ROOT_USER_INTERACTIVE.clone(), metamodelica::nil())?;
        println!("{}", (ErrorExt::printMessagesStr(false)).clone());
        bail!("fail");
    }
    if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
        setWindowsPaths((Settings::getInstallationDirectoryPath()?).clone())?;
    }
    if '__try0: {
        unwrap_break_err!(Settings::getInstallationDirectoryPath(), '__try0);
        unwrap_break_err!(readSettings(args.clone()), '__try0);
        if interactiveMode.clone() == literal!("tcp") {
            unwrap_break_err!(interactivemode(), '__try0);
        } else if interactiveMode.clone() == literal!("corba") {
            unwrap_break_err!(interactivemodeCorba(), '__try0);
        } else if interactiveMode.clone() == literal!("zmq") {
            unwrap_break_err!(interactivemodeZMQ(), '__try0);
        } else {
            unwrap_break_err!(translateFile(args.clone()), '__try0);
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
        if args.clone().is_empty() && Config::classToInstantiate()? == literal!("") {
            if !(Config::helpRequest()?) {
                println!("{}", (FlagsUtil::printUsage()?).clone());
                System::fflush();
            }
            return Ok(());
        }
        if '__try1: {
            unwrap_break_err!(Settings::getInstallationDirectoryPath(), '__try1);
            println!("{}", (literal!("# Error encountered! Exiting...\n")).clone());
            System::fflush();
            println!("{}", (literal!("# Please check the error message and the flags.\n")).clone());
            System::fflush();
            unwrap_break_err!(Print::printBuf((literal!("\n\n----\n\nError buffer:\n\n")).clone()), '__try1);
            System::fflush();
            println!("{}", (unwrap_break_err!(Print::getErrorString(), '__try1)).clone());
            System::fflush();
            println!("{}", (ErrorExt::printMessagesStr(false)).clone());
            System::fflush();
            println!("{}", (literal!("\n")).clone());
            System::fflush();
            Ok::<(), anyhow::Error>(())
        }.is_err() {
            println!("{}", (literal!("Error: Failed to retrieve the installation directory path!\n")).clone());
            System::fflush();
        }
        bail!("fail");
    }
    Ok(())
}

