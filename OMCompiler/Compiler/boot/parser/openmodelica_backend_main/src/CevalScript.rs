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

use crate::CevalScriptBackend;
use crate::Interactive::Access;
use crate::Interactive;
use crate::InteractiveUtil;
use openmodelica_ast::Absyn;
use openmodelica_backend::CevalScriptOMSimulator;
use openmodelica_backend::DAEToMid;
use openmodelica_backend::GenerateAPIFunctionsTpl;
use openmodelica_backend::MidCode;
use openmodelica_backend::SimCode;
use openmodelica_backend::SimCodeFunction;
use openmodelica_backend::SimCodeFunctionUtil;
use openmodelica_backend::SymbolTable;
use openmodelica_backend::Unparsing;
use openmodelica_codegen::CodegenCFunctions;
use openmodelica_codegen::CodegenMidToC;
use openmodelica_frontend::Builtin;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::CevalFunction;
use openmodelica_frontend::ClassLoader;
use openmodelica_frontend::ComponentReference;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::Expression;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::FNode;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::InstFunction;
use openmodelica_frontend::InstHashTable;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::Mod;
use openmodelica_frontend::Parser;
use openmodelica_frontend::Static;
use openmodelica_frontend::Types;
use openmodelica_frontend::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_program_util::ProgramUtil;
use openmodelica_script_util::DynLoad;
use openmodelica_script_util::PackageManagement;
use openmodelica_susan::Tpl;
use openmodelica_util::Autoconf;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Config;
use openmodelica_util::Corba;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::ErrorTypes;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExecStat::execStatReset;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Gettext;
use openmodelica_util::Global;
use openmodelica_util::Graph;
use openmodelica_util::HashSetString;
use openmodelica_util::Print;
use openmodelica_util::SemanticVersion;
use openmodelica_util::Settings;
use openmodelica_util::StackOverflow;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::GCExt;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub fn ceval(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inBoolean: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    pub type ReductionOperator = std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>, Arc<Values::Value>) -> Result<Arc<Values::Value>> + 'static>;

    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inBoolean.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { expLst: expl, path: funcpath, .. }, r#impl, msg) => {
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut newval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    let false = (stringEq((literal!("Connection.isRoot")).clone(), (AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?).clone())?) else { bail!("pattern mismatch") };
                    (cache, vallst) = Ceval::cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    (cache, newval) = cevalCallFunction(cache.clone(), env.clone(), e.clone(), vallst.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ DAE::Exp::CALL { .. }, true, msg) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, value) = cevalInteractiveFunctions(cache.clone(), env.clone(), e.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, r#impl, msg) => {
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, value) = Ceval::ceval(cache.clone(), env.clone(), e.clone(), r#impl.clone(), msg.clone(), numIter.clone() + 1)?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn isCompleteFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inFuncPath: Arc<Absyn::Path>) -> Result<bool> {
    let mut isComplete: bool = false;
    isComplete = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inFuncPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, fpath) => {
                    ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), fpath.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(_), .. }, .. }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    let true = (System::getPartialInstantiation()) else { bail!("pattern mismatch") };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, fpath) => {
                    ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), fpath.clone(), None)?) {
                        (_, Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::PARTIAL, .. }, _) => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(isComplete)
}

pub fn compileModel(mut fileprefix: ArcStr, mut libs: Arc<metamodelica::List<ArcStr>>, mut workingDir: ArcStr, mut makeVars: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut omhome: ArcStr = Settings::getInstallationDirectoryPath()?;
    let mut omhome_1: ArcStr = System::stringReplace((omhome.clone()).clone(), (literal!("\"")).clone(), (literal!("")).clone())?;
    let mut pd: ArcStr = arcstr::literal!(Autoconf::pathDelimiter);
    let mut cdWorkingDir: ArcStr = arcstr::literal!("");
    let mut setMakeVars: ArcStr = arcstr::literal!("");
    let mut libsfilename: ArcStr = arcstr::literal!("");
    let mut libs_str: ArcStr = arcstr::literal!("");
    let mut s_call: ArcStr = arcstr::literal!("");
    let mut winCompileMode: ArcStr = arcstr::literal!("");
    let mut workDir: ArcStr = if (stringEq((workingDir.clone()).clone(), (literal!("")).clone())?) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*workingDir.clone()); __mm_s.push_str(&*pd.clone()); ArcStr::from(__mm_s) }};
    let mut linkType: ArcStr = literal!("dynamic");
    let mut fileDLL: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*workDir.clone()); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) };
    let mut fileEXE: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*workDir.clone()); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::exeExt)); ArcStr::from(__mm_s) };
    let mut fileLOG: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*workDir.clone()); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!(".log")); ArcStr::from(__mm_s) };
    let mut numParallel: i32 = 0;
    let mut isWindows: bool = arcstr::literal!(Autoconf::os) == literal!("Windows_NT");
    let mut makeVarsNoBinding: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    libsfilename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workDir.clone()); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!(".libs")); ArcStr::from(__mm_s) }).clone();
    libs_str = stringDelimitList(libs.clone(), (literal!(" ")).clone());
    makeVarsNoBinding = makeVars.clone();
    System::writeFile((libsfilename.clone()).clone(), (libs_str.clone()).clone())?;
    if isWindows.clone() {
        omhome = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("set OPENMODELICAHOME=")); __mm_s.push_str(&*System::stringReplace((omhome_1.clone()).clone(), (literal!("/")).clone(), (literal!("\\")).clone())?); __mm_s.push_str(&*literal!("&& ")); ArcStr::from(__mm_s) }).clone();
        setMakeVars = (({
        let mut __acc = String::new();
        for mut var in (makeVarsNoBinding.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("set ")); __mm_s.push_str(&*var.clone()); __mm_s.push_str(&*literal!("&& ")); ArcStr::from(__mm_s) };
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    })).clone();
        cdWorkingDir = (if (stringEmpty((workingDir.clone()).clone())) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("cd \"")); __mm_s.push_str(&*workingDir.clone()); __mm_s.push_str(&*literal!("\"&& ")); ArcStr::from(__mm_s) }}).clone();
        winCompileMode = (if (Testsuite::isRunning()?) {literal!("serial")} else {literal!("parallel")}).clone();
        if Flags::getConfigEnum(Flags::LINK_TYPE.clone())? == 1 {
            linkType = (literal!("static")).clone();
        } else if Flags::getConfigEnum(Flags::LINK_TYPE.clone())? == 2 {
            linkType = (literal!("dynamic")).clone();
        }
        linkType = (if (Testsuite::isRunning()?) {literal!("dynamic")} else {linkType.clone()}).clone();
        s_call = stringAppendList(list![(omhome.clone()).clone(), (cdWorkingDir.clone()).clone(), (setMakeVars.clone()).clone(), (literal!("\"")).clone(), (omhome_1.clone()).clone(), (pd.clone()).clone(), (literal!("share")).clone(), (pd.clone()).clone(), (literal!("omc")).clone(), (pd.clone()).clone(), (literal!("scripts")).clone(), (pd.clone()).clone(), (literal!("Compile")).clone(), (literal!("\"")).clone(), (literal!(" ")).clone(), (fileprefix.clone()).clone(), (literal!(" ")).clone(), (Config::simulationCodeTarget()?).clone(), (literal!(" ")).clone(), (System::openModelicaPlatform()).clone(), (literal!(" ")).clone(), (winCompileMode.clone()).clone(), (literal!(" ")).clone(), (linkType.clone()).clone()]);
    } else {
        numParallel = if (Testsuite::isRunning()?) {1} else {Config::noProc()?};
        cdWorkingDir = (if (stringEmpty((workingDir.clone()).clone())) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" -C \"")); __mm_s.push_str(&*workingDir.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }}).clone();
        setMakeVars = (({
        let mut __acc = String::new();
        for mut var in (makeVarsNoBinding.clone()).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*var.clone()); ArcStr::from(__mm_s) };
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    })).clone();
        s_call = stringAppendList(list![(arcstr::literal!(Autoconf::make)).clone(), (literal!(" -j")).clone(), (intString(numParallel.clone())).clone(), (cdWorkingDir.clone()).clone(), (literal!(" -f ")).clone(), (fileprefix.clone()).clone(), (literal!(".makefile")).clone(), (setMakeVars.clone()).clone()]);
    }
    if Flags::isSet(Flags::DYN_LOAD.clone())? {
        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("compileModel: running ")); __mm_s.push_str(&*s_call.clone()); ArcStr::from(__mm_s) }).clone())?;
    }
    if System::regularFileExists((fileEXE.clone()).clone()) {
        let 0 = (System::removeFile((fileEXE.clone()).clone())) else { bail!("pattern mismatch") };
    }
    if System::regularFileExists((fileDLL.clone()).clone()) {
        let 0 = (System::removeFile((fileDLL.clone()).clone())) else { bail!("pattern mismatch") };
    }
    if System::regularFileExists((fileLOG.clone()).clone()) {
        let 0 = (System::removeFile((fileLOG.clone()).clone())) else { bail!("pattern mismatch") };
    }
    if Testsuite::isRunning()? {
        System::appendFile((Testsuite::getTempFilesFile()?).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileEXE.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*fileDLL.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*fileLOG.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!(".o\n")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!(".libs\n")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_records.o\n")); __mm_s.push_str(&*fileprefix.clone()); __mm_s.push_str(&*literal!("_res.mat\n")); ArcStr::from(__mm_s) }).clone())?;
    }
    if System::systemCall((s_call.clone()).clone(), (if (isWindows.clone()) {literal!("")} else {fileLOG.clone()}).clone()) != 0 {
        if System::regularFileExists((fileLOG.clone()).clone()) {
            Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![(System::readFile((fileLOG.clone()).clone())?).clone()])?;
        } else if isWindows.clone() {
            s_call = stringAppendList(list![(omhome_1.clone()).clone(), (pd.clone()).clone(), (literal!("share")).clone(), (pd.clone()).clone(), (literal!("omc")).clone(), (pd.clone()).clone(), (literal!("scripts")).clone(), (pd.clone()).clone(), (literal!("Compile.bat")).clone()]);
            if !(System::regularFileExists((s_call.clone()).clone())) {
                Error::addMessage(Error::SIMULATOR_BUILD_ERROR.clone(), list![stringAppendList(list![(literal!("command ")).clone(), (s_call.clone()).clone(), (literal!(" not found. Check $OPENMODELICAHOME")).clone()])])?;
            }
        }
        if Flags::isSet(Flags::DYN_LOAD.clone())? {
            Debug::trace((literal!("compileModel: failed!\n")).clone())?;
        }
        bail!("fail");
    }
    if Flags::isSet(Flags::DYN_LOAD.clone())? {
        Debug::trace((literal!("compileModel: successful!\n")).clone())?;
    }
    Ok(())
}

pub fn loadFile(mut inName: ArcStr, mut encoding: ArcStr, mut p: Absyn::Program, mut checkUses: bool, mut notifyLoad: bool, mut requireExactVersion: bool, mut allowWithin: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut dir: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = inName.clone();
    let mut filename: ArcStr = arcstr::literal!("");
    let mut cname: ArcStr = arcstr::literal!("");
    let mut prio: ArcStr = arcstr::literal!("");
    let mut mp: ArcStr = arcstr::literal!("");
    let mut msg: ArcStr = arcstr::literal!("");
    let mut rest: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if System::directoryExists((inName.clone()).clone()) {
        name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inName.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter)); __mm_s.push_str(&*literal!("package.mo")); ArcStr::from(__mm_s) }).clone();
    }
    if !(System::regularFileReadable((name.clone()).clone())) {
        if !(System::regularFileExists((name.clone()).clone())) {
            msg = (literal!("file does not exist")).clone();
        } else {
            msg = (literal!("read access denied")).clone();
        }
        Error::addMessage(Error::LOAD_FILE_FAILED.clone(), list![(name.clone()).clone(), (msg.clone()).clone()])?;
        bail!("fail");
    }
    (dir, filename) = Util::getAbsoluteDirectoryAndFile((name.clone()).clone())?;
    if filename.clone() == literal!("package.mo") || filename.clone() == literal!("package.moc") {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok((List::last(System::strtok((dir.clone()).clone(), (literal!("/")).clone()))?).clone(), (literal!(" ")).clone())) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cname = __pa0.clone();
        rest = __pa1.clone();
        prio = stringDelimitList(rest.clone(), (literal!(" ")).clone());
        mp = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*System::realpath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*literal!("/../")); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*arcstr::literal!(Autoconf::groupDelimiter)); __mm_s.push_str(&*Settings::getModelicaPath(Testsuite::isRunning()?)?); ArcStr::from(__mm_s) }).clone();
        let (__pa2, true) = (loadModel(cons((Arc::new(Absyn::Path::IDENT { name: (cname.clone()).clone() }), literal!("loadFile automatically converted to loadModel"), list![(prio.clone()).clone()], true), metamodelica::nil()), (mp.clone()).clone(), p.clone(), true, notifyLoad.clone(), checkUses.clone(), requireExactVersion.clone(), filename.clone() == literal!("package.moc"), (System::realpath((name.clone()).clone())?).clone())?) else { bail!("pattern mismatch") };
        outProgram = __pa2.clone();
        return Ok(outProgram.clone());
    }
    outProgram = Parser::parse((name.clone()).clone(), (encoding.clone()).clone(), (literal!("")).clone(), None, Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
    if !(allowWithin.clone()) {
        checkTopClassWithin(outProgram.clone(), (name.clone()).clone())?;
    }
    ClassLoader::checkOnLoadMessage(outProgram.clone())?;
    if checkDuplicateTopLevelClasses(outProgram.clone())? {
        bail!("fail");
    }
    outProgram = checkUsesAndUpdateProgram(outProgram.clone(), p.clone(), checkUses.clone(), (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone(), notifyLoad.clone(), requireExactVersion.clone(), false)?;
    Ok(outProgram)
}

fn checkDuplicateTopLevelClasses(mut program: Absyn::Program) -> Result<bool> {
    let mut hasDuplicates: bool = false;
    let mut skip: bool = false;
    let mut infos: Arc<metamodelica::List<SourceInfo>> = metamodelica::nil();
    let mut classInfoMap: Arc<UnorderedMap::UnorderedMap<ArcStr, SourceInfo>>;
    let mut optClassInfo: Option<SourceInfo> = None;
    if (program.classes.clone().len() as i32) < 2 {
        return Ok(hasDuplicates.clone());
    }
    classInfoMap = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    for mut cl in &*program.classes.clone() {
        let mut cl = cl.clone();
        let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::Class { info: SourceInfo { .. }, .. } => {
            skip = stringEq(cl.info.fileName.clone(), (literal!("<interactive>")).clone())? || stringEq((System::basename(cl.info.fileName.clone())).clone(), (literal!("ModelicaBuiltin.mo")).clone())? || stringEq((System::basename(cl.info.fileName.clone())).clone(), (literal!("MetaModelicaBuiltin.mo")).clone())?;
            if !(skip.clone()) {
                optClassInfo = UnorderedMap::get((cl.name.clone()).clone(), classInfoMap.clone());
                if isSome(optClassInfo.clone()) {
                    infos = list![Util::getOption(optClassInfo.clone())?, cl.info.clone()];
                    Error::addMultiSourceMessage(Error::DOUBLE_DECLARATION_OF_ELEMENTS.clone(), list![(cl.name.clone()).clone()], infos.clone())?;
                    hasDuplicates = true;
                    return Ok(hasDuplicates.clone());
                } else {
                    UnorderedMap::add((cl.name.clone()).clone(), cl.info.clone(), classInfoMap.clone())?;
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(hasDuplicates)
}

fn checkTopClassWithin(mut program: Absyn::Program, mut filename: ArcStr) -> Result<()> {
    if !(AbsynUtil::withinEqual(program.within_.clone(), openmodelica_ast::Absyn::Within::TOP)) {
        Error::addSourceMessage(Error::LIBRARY_UNEXPECTED_WITHIN.clone(), list![(AbsynUtil::withinString(openmodelica_ast::Absyn::Within::TOP)?).clone(), (AbsynUtil::withinString(program.within_.clone())?).clone()], SourceInfo { fileName: (filename.clone()).clone(), isReadOnly: false, lineNumberStart: 1, columnNumberStart: 0, lineNumberEnd: 1, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat((0) as f64) })?;
        bail!("fail");
    }
    Ok(())
}

fn checkUsesAndUpdateProgram(mut newp: Absyn::Program, mut p: Absyn::Program, mut checkUses: bool, mut modelicaPath: ArcStr, mut notifyLoad: bool, mut requireExactVersion: bool, mut mergeAST: bool) -> Result<Absyn::Program> {
    let mut p: Absyn::Program = p;
    let mut modelsToLoad: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    modelsToLoad = if (checkUses.clone()) {Interactive::getUsesAnnotationOrDefault(newp.clone(), requireExactVersion.clone())?} else {metamodelica::nil()};
    p = ProgramUtil::updateProgram(newp.clone(), p.clone(), mergeAST.clone())?;
    (p, _) = loadModel(modelsToLoad.clone(), (modelicaPath.clone()).clone(), p.clone(), false, notifyLoad.clone(), checkUses.clone(), requireExactVersion.clone(), false, (literal!("")).clone())?;
    Ok(p)
}

pub fn loadModel(mut imodelsToLoad: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>>, mut modelicaPath: ArcStr, mut ip: Absyn::Program, mut forceLoad: bool, mut notifyLoad: bool, mut checkUses: bool, mut requireExactVersion: bool, mut encrypted: bool, mut pathToFile: ArcStr) -> Result<(Absyn::Program, bool)> {
    let mut pnew: Absyn::Program = ip.clone();
    let mut success: bool = true;
    let mut b: bool = false;
    PackageManagement::installCachedPackages()?;
    for mut m in &*imodelsToLoad.clone() {
        let mut m = m.clone();
        (pnew, b) = loadModel1(m.clone(), (modelicaPath.clone()).clone(), forceLoad.clone(), notifyLoad.clone(), checkUses.clone(), requireExactVersion.clone(), encrypted.clone(), (pathToFile.clone()).clone(), pnew.clone())?;
        success = b.clone() && success.clone();
    }
    Ok((pnew, success))
}

fn loadModel1(mut modelToLoad: (Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool), mut modelicaPath: ArcStr, mut forceLoad: bool, mut notifyLoad: bool, mut checkUses: bool, mut requireExactVersion: bool, mut encrypted: bool, mut pathToFile: ArcStr, mut program: Absyn::Program) -> Result<(Absyn::Program, bool)> {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut modelsToLoad: Arc<metamodelica::List<(Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool)>> = metamodelica::nil();
    let mut onlyCheckFirstModelicaPath: bool = false;
    let mut path: Arc<Absyn::Path>;
    let mut versionsLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut pathStr: ArcStr = arcstr::literal!("");
    let mut versions: ArcStr = arcstr::literal!("");
    let mut version: ArcStr = arcstr::literal!("");
    let mut thisModelicaPath: ArcStr = arcstr::literal!("");
    let mut dir: ArcStr = arcstr::literal!("");
    let mut requestedBy: ArcStr = arcstr::literal!("");
    let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut msgTokens: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut cl: Option<Arc<Absyn::Class>> = None;
    (path, requestedBy, versionsLst, onlyCheckFirstModelicaPath) = modelToLoad.clone();
    if onlyCheckFirstModelicaPath.clone() {
        let __pa0 = ::match_deref::match_deref! { match &(System::strtok((modelicaPath.clone()).clone(), (arcstr::literal!(Autoconf::groupDelimiter)).clone())) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        thisModelicaPath = __pa0.clone();
    } else {
        thisModelicaPath = (modelicaPath.clone()).clone();
    }
    match '__try1: {
        if unwrap_break_err!(checkModelLoaded(modelToLoad.clone(), program.clone(), forceLoad.clone(), None), '__try1) {
            pnew = Absyn::Program { classes: metamodelica::nil(), within_: openmodelica_ast::Absyn::Within::TOP };
            version = (literal!("")).clone();
            return Ok((program.clone(), success.clone()));
        } else {
            if pathToFile.clone() == literal!("") {
                pnew = unwrap_break_err!(ClassLoader::loadClass(path.clone(), versionsLst.clone(), (thisModelicaPath.clone()).clone(), None, requireExactVersion.clone(), encrypted.clone()), '__try1);
            } else {
                dir = (System::dirname((pathToFile.clone()).clone())).clone();
                cl = unwrap_break_err!(ClassLoader::loadClassFromMp((AbsynUtil::pathFirstIdent(path.clone())?).clone(), (System::dirname((dir.clone()).clone())).clone(), (System::basename((dir.clone()).clone())).clone(), true, None, encrypted.clone()), '__try1);
                if isSome(cl.clone()) {
                    pnew = Absyn::Program { classes: list![unwrap_break_err!(Util::getOption(cl.clone()), '__try1)], within_: openmodelica_ast::Absyn::Within::TOP };
                } else {
                    pnew = Absyn::Program { classes: metamodelica::nil(), within_: openmodelica_ast::Absyn::Within::TOP };
                }
            }
            checkPatchedModelicaServices((AbsynUtil::pathFirstIdent(path.clone())?).clone(), pnew.clone());
            if notifyLoad.clone() && !(forceLoad.clone()) {
                version = (unwrap_break_err!(getPackageVersion(path.clone(), pnew.clone()), '__try1)).clone();
                msgTokens = list![(unwrap_break_err!(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false), '__try1)).clone(), (version.clone()).clone(), (requestedBy.clone()).clone()];
                unwrap_break_err!(Error::addMessage(Error::NOTIFY_LOAD_MODEL_DUE_TO_USES.clone(), msgTokens.clone()), '__try1);
                System::loadModelCallBack((AbsynUtil::pathFirstIdent(path.clone())?).clone());
            }
        }
        program = unwrap_break_err!(ProgramUtil::updateProgram(pnew.clone(), program.clone(), false), '__try1);
        if checkUses.clone() {
            modelsToLoad = unwrap_break_err!(Interactive::getUsesAnnotationOrDefault(pnew.clone(), requireExactVersion.clone()), '__try1);
            (program, success) = unwrap_break_err!(loadModel(modelsToLoad.clone(), (modelicaPath.clone()).clone(), program.clone(), false, notifyLoad.clone(), checkUses.clone(), requireExactVersion.clone(), false, (literal!("")).clone()), '__try1);
        }
        Ok::<_, anyhow::Error>((pnew.clone(), program.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            pnew = __try1_o0;
            program = __try1_o1;
        }
        Err(_) => {
            pathStr = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
            versions = stringDelimitList(versionsLst.clone(), (literal!(",")).clone());
            msgTokens = list![(pathStr.clone()).clone(), (versions.clone()).clone(), (thisModelicaPath.clone()).clone()];
            if forceLoad.clone() {
                Error::addMessage(Error::LOAD_MODEL_FAILED.clone(), msgTokens.clone())?;
                success = false;
            } else {
                Error::addMessage(Error::NOTIFY_LOAD_MODEL_FAILED.clone(), msgTokens.clone())?;
            }
            bail!("try/else: outputs not set in else branch");
        }
    }
    Ok((program, success))
}

fn checkModelLoaded(mut tpl: (Arc<Absyn::Path>, ArcStr, Arc<metamodelica::List<ArcStr>>, bool), mut p: Absyn::Program, mut forceLoad: bool, mut failNonLoad: Option<ArcStr>) -> Result<bool> {
    let mut loaded: bool = false;
    loaded = 'mc: {
        let __mc_input = (tpl.clone(), forceLoad.clone(), failNonLoad.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((path, requestOrigin, Deref @ metamodelica::List::Cons { head: str1, tail: _ }, _), false, _) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut ostr2: Option<ArcStr> = None;
                    let mut withoutConversion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut withConversion: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    cdef = ProgramUtil::getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    ostr2 = AbsynUtil::getNamedAnnotationInClass(cdef.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("version")).clone() }), (std::sync::Arc::new(Interactive::getAnnotationStringValueOrFail) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<ArcStr> + 'static>))?;
                    (withoutConversion, withConversion) = Interactive::getConversionAnnotation(cdef.clone())?;
                    checkValidVersion(path.clone(), (str1.clone()).clone(), ostr2.clone(), (requestOrigin.clone()).clone(), withConversion.clone(), withoutConversion.clone())?;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, None) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ((path, _, _, _), _, Some(str2)) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    str1 = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::INST_NON_LOADED.clone(), list![(str1.clone()).clone(), (str2.clone()).clone()])?;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(loaded)
}

fn checkValidVersion(mut path: Arc<Absyn::Path>, mut version: ArcStr, mut actualVersion: Option<ArcStr>, mut requestOrigin: ArcStr, mut withConversion: Arc<metamodelica::List<ArcStr>>, mut withoutConversion: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let mut semverWanted: SemanticVersion::Version;
    let mut semverActual: SemanticVersion::Version;
    let mut actualVersionStr: ArcStr = arcstr::literal!("");
    let mut pathStr: ArcStr = arcstr::literal!("");
    semverWanted = SemanticVersion::parse((version.clone()).clone(), false)?;
    actualVersionStr = (Util::getOptionOrDefault(actualVersion.clone(), (literal!("")).clone())).clone();
    pathStr = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
    semverActual = SemanticVersion::parse((actualVersionStr.clone()).clone(), false)?;
    if 0 == SemanticVersion::compare(semverWanted.clone(), semverActual.clone(), false, false)? {
        return Ok(());
    }
    if !(SemanticVersion::isSemVer(semverActual.clone()) && SemanticVersion::isSemVer(semverWanted.clone())) {
        Error::addMessage(Error::LOAD_MODEL_DIFFERENT_VERSIONS.clone(), list![(pathStr.clone()).clone(), (version.clone()).clone(), (actualVersionStr.clone()).clone()])?;
        return Ok(());
    }
    for mut ver in &*withoutConversion.clone() {
        let mut ver = ver.clone();
        if 0 == SemanticVersion::compare(semverWanted.clone(), SemanticVersion::parse((ver.clone()).clone(), false)?, false, false)? {
            Error::addMessage(Error::LOAD_MODEL_DIFFERENT_VERSIONS_WITHOUT_CONVERSION.clone(), list![(requestOrigin.clone()).clone(), (pathStr.clone()).clone(), (version.clone()).clone(), (actualVersionStr.clone()).clone()])?;
            return Ok(());
        }
    }
    for mut ver in &*withConversion.clone() {
        let mut ver = ver.clone();
        if 0 == SemanticVersion::compare(semverWanted.clone(), SemanticVersion::parse((ver.clone()).clone(), false)?, false, false)? {
            Error::addMessage(Error::LOAD_MODEL_DIFFERENT_VERSIONS_WITH_CONVERSION.clone(), list![(requestOrigin.clone()).clone(), (pathStr.clone()).clone(), (version.clone()).clone(), (actualVersionStr.clone()).clone()])?;
            return Ok(());
        }
    }
    if SemanticVersion::compare(semverWanted.clone(), semverActual.clone(), true, false)? > 0 {
        Error::addMessage(Error::LOAD_MODEL_DIFFERENT_VERSIONS_NEWER.clone(), list![(pathStr.clone()).clone(), (version.clone()).clone(), (actualVersionStr.clone()).clone()])?;
    } else {
        Error::addMessage(Error::LOAD_MODEL_DIFFERENT_VERSIONS_OLDER.clone(), list![(pathStr.clone()).clone(), (version.clone()).clone(), (actualVersionStr.clone()).clone()])?;
    }
    Ok(())
}

fn checkPatchedModelicaServices(mut name: ArcStr, mut program: Absyn::Program) -> () {
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut alg: Arc<Absyn::Algorithm> = Arc::new(Absyn::Algorithm::ALG_BREAK);
    let mut r#fn: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    if name.clone() == literal!("ModelicaServices") {
        if '__try0: {
            cls = unwrap_break_err!(ProgramUtil::getPathedClassInProgram(Arc::new(Absyn::Path::QUALIFIED { name: (literal!("ModelicaServices")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("ExternalReferences")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("loadResource")).clone() }) }) }), program.clone(), false, false), '__try0);
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(List::find(AbsynUtil::getClassPartsInClass(cls.clone()), (std::sync::Arc::new(fnptr!(AbsynUtil::isAlgorithmSection, Arc<Absyn::ClassPart>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ClassPart>) -> Result<bool> + 'static>)), '__try0)) {
                Deref @ Absyn::ClassPart::ALGORITHMS { contents: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: __pa1, .. }, tail: Deref @ metamodelica::List::Nil } } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            alg = __pa1.clone();
            let __pa3 = ::match_deref::match_deref! { match &(alg.clone()) {
                Deref @ Absyn::Algorithm::ALG_ASSIGN { value: Deref @ Absyn::Exp::CALL { function_: __pa3, .. }, .. } => __pa3.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            r#fn = __pa3.clone();
            if unwrap_break_err!(AbsynUtil::crefString(r#fn.clone()), '__try0) != literal!("OpenModelica.Scripting.uriToFilename") {
                unwrap_break_err!(Error::addMessage(Error::UNPATCHED_MODELICA_SERVICES.clone(), metamodelica::nil()), '__try0);
            }
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
    }
    ()
}

pub fn cevalInteractiveFunctions(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut msg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::List::Cons { head: exp, tail: Deref @ metamodelica::List::Nil }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "timing" }, .. }) => {
                    let mut t1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut t2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut t: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut cache = (*cache).clone();
                    t1 = System::time();
                    (cache, _) = Ceval::ceval(cache.clone(), env.clone(), exp.clone(), true, msg.clone(), numIter.clone() + 1)?;
                    t2 = System::time();
                    t = t2.clone() - t1.clone();
                    Ok((cache.clone(), Arc::new(Values::Value::REAL { real: t.clone() })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { expLst: eLst, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, path: Deref @ Absyn::Path::IDENT { name } }) => {
                    let mut valLst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut value: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, valLst) = Ceval::cevalList(cache.clone(), env.clone(), eLst.clone(), true, msg.clone(), numIter.clone())?;
                    valLst = List::map1(valLst.clone(), (std::sync::Arc::new(evalCodeTypeName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>, FCore::Graph) -> Result<Arc<Values::Value>> + 'static>), env.clone());
                    (cache, value) = cevalInteractiveFunctions2(cache.clone(), env.clone(), (name.clone()).clone(), valLst.clone(), msg.clone())?;
                    Ok((cache.clone(), value.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn cevalInteractiveFunctions2(mut cache: FCore::Cache, mut env: FCore::Graph, mut functionName: ArcStr, mut args: Arc<metamodelica::List<Arc<Values::Value>>>, mut msg: Absyn::Msg) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = cache.clone();
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    outValue = 'mc: {
        let __mc_input = (functionName.clone(), args.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
                    let mut within_: Absyn::Within = Absyn::Within::TOP;
                    let Absyn::PROGRAM { within_: __pa0, classes: __pa1 } = (Parser::parsestring((str1.clone()).clone(), (str2.clone()).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?) else { bail!("pattern mismatch") };
                    within_ = __pa0.clone();
                    classes = __pa1.clone();
                    paths = ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut c in (classes.clone()).into_iter().cloned() {
                    let __x = Arc::new(Absyn::Path::IDENT { name: (AbsynUtil::className(c.clone())?).clone() });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    paths = List::map1r(paths.clone(), (std::sync::Arc::new(AbsynUtil::joinWithinPath) as std::sync::Arc<dyn ::std::ops::Fn(Absyn::Within, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>), within_.clone());
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseString", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    Error::clearMessages();
                    Print::clearErrorBuf();
                    paths = Interactive::parseFile((str1.clone()).clone(), (encoding.clone()).clone(), false)?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFileInteractiveQualified", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    Error::clearMessages();
                    Print::clearErrorBuf();
                    paths = Interactive::parseFile((str1.clone()).clone(), (encoding.clone()).clone(), true)?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFileInteractive", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    newp = loadFile((str1.clone()).clone(), (encoding.clone()).clone(), SymbolTable::getAbsyn(), b.clone(), b1.clone(), requireExactVersion.clone(), true)?;
                    vals = List::map(Interactive::getTopClassnames(newp.clone())?, (std::sync::Arc::new(fnptr!(ValuesMake::makeCodeTypeName, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<Arc<Values::Value>> + 'static>));
                    SymbolTable::setAbsyn(newp.clone())?;
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getSourceFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Interactive::getSourceFile(path.clone(), SymbolTable::getAbsyn())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setSourceFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut access: Access = Access::hide;
                    let mut b: bool = false;
                    access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                    if access.clone() >= Access::all.clone() {
                        (b, p) = Interactive::setSourceFile(path.clone(), (r#str.clone()).clone(), SymbolTable::getAbsyn())?;
                        SymbolTable::setAbsyn(p.clone())?;
                    } else {
                        Error::addMessage(Error::SAVE_ENCRYPTED_CLASS_ERROR.clone(), metamodelica::nil())?;
                        b = false;
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "basename", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::basename((r#str.clone()).clone())).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "dirname", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::dirname((r#str.clone()).clone())).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "codeToString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: codeNode }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Dump::printCodeStr(codeNode.clone())?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "typeOf", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_VARIABLENAME { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, .. } } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    ty = Interactive::getTypeOfVariable((name.clone()).clone(), SymbolTable::getVars())?;
                    Ok(Arc::new(Values::Value::STRING { string: (TypesDump::unparseType(ty.clone())?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_gcollect_and_unmap", Deref @ metamodelica::List::Nil) => {
                    GCExt::gcollectAndUnmap();
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_expand_hp", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: GCExt::expandHeap(metamodelica::OrderedFloat((i.clone()) as f64)) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_set_max_heap_size", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil }) => {
                    GCExt::setMaxHeapSize(metamodelica::OrderedFloat((i.clone()) as f64));
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "GC_get_prof_stats", Deref @ metamodelica::List::Nil) => {
                    let mut gcStats: GCExt::ProfStats = <GCExt::ProfStats as ::std::default::Default>::default();
                    gcStats = GCExt::getProfStats();
                    Ok(Arc::new(Values::Value::RECORD { record_: Arc::new(Absyn::Path::IDENT { name: (literal!("GC_PROFSTATS")).clone() }), orderd: list![Arc::new(Values::Value::INTEGER { integer: gcStats.heapsize_full.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.free_bytes_full.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.unmapped_bytes.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.bytes_allocd_since_gc.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.allocd_bytes_before_gc.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.bytes_allocd_since_gc.clone() + gcStats.allocd_bytes_before_gc.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.non_gc_bytes.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.gc_no.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.markers_m1.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.bytes_reclaimed_since_gc.clone() }), Arc::new(Values::Value::INTEGER { integer: gcStats.reclaimed_bytes_before_gc.clone() })], comp: list![(literal!("heapsize_full")).clone(), (literal!("free_bytes_full")).clone(), (literal!("unmapped_bytes")).clone(), (literal!("bytes_allocd_since_gc")).clone(), (literal!("allocd_bytes_before_gc")).clone(), (literal!("total_allocd_bytes")).clone(), (literal!("non_gc_bytes")).clone(), (literal!("gc_no")).clone(), (literal!("markers_m1")).clone(), (literal!("bytes_reclaimed_since_gc")).clone(), (literal!("reclaimed_bytes_before_gc")).clone()], index: -1 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clear", Deref @ metamodelica::List::Nil) => {
                    SymbolTable::reset()?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearProgram", Deref @ metamodelica::List::Nil) => {
                    SymbolTable::clearProgram()?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearVariables", Deref @ metamodelica::List::Nil) => {
                    SymbolTable::setVars(metamodelica::nil());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "list", _) => {
                    Ok(listClass(args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "listFile", _) => {
                    Ok(listFile(args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "sortStrings", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = List::map(vals.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    strs = List::sort(strs.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
                    Ok(ValuesMake::makeArray(List::map(strs.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "listVariables", Deref @ metamodelica::List::Nil) => {
                    Ok(ValuesMake::makeArray(getVariableNames(SymbolTable::getVars(), metamodelica::nil())?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setTempDirectoryPath", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::List::Nil }) => {
                    Settings::setTempDirectoryPath((cmd.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTempDirectoryPath", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Settings::getTempDirectoryPath()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setEnvironmentVar", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: System::setEnv((name.clone()).clone(), (r#str.clone()).clone(), true) == 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getEnvironmentVar", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Util::makeValueOrDefault((std::sync::Arc::new(System::readEnv) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>), (name.clone()).clone(), (literal!("")).clone())).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setInstallationDirectoryPath", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::List::Nil }) => {
                    Settings::setInstallationDirectoryPath((cmd.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getInstallationDirectoryPath", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Settings::getInstallationDirectoryPath()?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getModelicaPath", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setModelicaPath", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::List::Nil }) => {
                    Settings::setModelicaPath((cmd.clone()).clone());
                    { let __v = None; openmodelica_util::Globals::packageIndexCacheIndex.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setModelicaPath", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getHomeDirectoryPath", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Settings::getHomeDir(Testsuite::isRunning()?)).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLanguageStandard", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Config::languageStandardString(Config::getLanguageStandard()?)?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reopenStandardStream", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: i, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: System::reopenStandardStream(i.clone() - 1, (filename.clone()).clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "iconv", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: from }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: to }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::iconv((r#str.clone()).clone(), (from.clone()).clone(), (to.clone()).clone())).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCompiler", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::getCCompiler()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCFlags", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::setCFlags((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCFlags", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::getCFlags()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCompiler", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::setCCompiler((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCXXCompiler", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::getCXXCompiler()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCXXCompiler", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::setCXXCompiler((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCompilerFlags", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::setCFlags((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLinker", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::getLinker()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setLinker", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::setLinker((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getLinkerFlags", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::getLDFlags()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setLinkerFlags", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::setLDFlags((r#str.clone()).clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCommandLineOptions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut outCache: FCore::Cache = outCache.clone();
                    b = Flags::isSet(Flags::SCODE_INST.clone())?;
                    strs = System::strtok((r#str.clone()).clone(), (literal!(" ")).clone());
                    ::match_deref::match_deref! { match &(FlagsUtil::readArgs(strs.clone())?) {
                        Deref @ metamodelica::List::Nil => (),
                        _ => bail!("pattern mismatch"),
                    } };
                    outCache = FCore::emptyCache();
                    if b.clone() != Flags::isSet(Flags::SCODE_INST.clone())? {
                        Builtin::clearInitialGraph();
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "setCommandLineOptions", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCommandLineOptions", Deref @ metamodelica::List::Nil) => {
                    Ok(ValuesMake::makeStringArray(FlagsUtil::unparseFlags()?)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getCommandLineOptions", _) => {
                    Ok(Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearCommandLineOptions", Deref @ metamodelica::List::Nil) => {
                    FlagsUtil::resetDebugFlags()?;
                    FlagsUtil::resetConfigFlags()?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearCommandLineOptions", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "enableNewInstantiation", _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    if !(Flags::isSet(Flags::SCODE_INST.clone())?) {
                        Builtin::clearInitialGraph();
                        FlagsUtil::enableDebug(Flags::SCODE_INST.clone())?;
                        outCache = FCore::emptyCache();
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "enableNewInstantiation", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "disableNewInstantiation", _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    if Flags::isSet(Flags::SCODE_INST.clone())? {
                        FlagsUtil::disableDebug(Flags::SCODE_INST.clone())?;
                        outCache = FCore::emptyCache();
                        Builtin::clearInitialGraph();
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "disableNewInstantiation", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearDebugFlags", _) => {
                    FlagsUtil::resetDebugFlags()?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearDebugFlags", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConfigFlagValidOptions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut strs1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut strs2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str = (*r#str).clone();
                    (strs1, r#str, strs2) = FlagsUtil::getValidOptionsAndDescription((r#str.clone()).clone())?;
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeStringArray(strs1.clone())?, Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }), ValuesMake::makeStringArray(strs2.clone())?] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getConfigFlagValidOptions", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: _ }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(metamodelica::nil())?, Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), ValuesMake::makeArray(metamodelica::nil())?] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cd", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "" }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::pwd()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cd", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let 0 = (System::cd((r#str.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(Values::Value::STRING { string: (System::pwd()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "cd", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut res: ArcStr = arcstr::literal!("");
                    let false = (System::directoryExists((r#str.clone()).clone())) else { bail!("pattern mismatch") };
                    res = stringAppendList(list![(literal!("Error, directory ")).clone(), (r#str.clone()).clone(), (literal!(" does not exist,")).clone()]);
                    Ok(Arc::new(Values::Value::STRING { string: (res.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "mkdir", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let true = (System::directoryExists((r#str.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "mkdir", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: Util::createDirectoryTree((r#str.clone()).clone())? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "copy", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: System::copyFile((str1.clone()).clone(), (str2.clone()).clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "remove", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: System::removeDirectory((r#str.clone()).clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getVersion", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: Deref @ "OpenModelica" } } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Settings::getVersionNr()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getVersion", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (getPackageVersion(path.clone(), SymbolTable::getAbsyn())?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTempDirectoryPath", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Settings::getTempDirectoryPath()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "system", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: System::systemCall((r#str.clone()).clone(), (filename.clone()).clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "system_parallel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = List::map(vals.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    Ok(ValuesMake::makeIntArray(System::systemCallParallel(strs.clone(), i.clone()))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerClear", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::realtimeClear(i.clone())?;
                    Ok(Arc::new(openmodelica_frontend_types::Values::Value::NORETCALL))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerTick", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::realtimeTick(i.clone())?;
                    Ok(Arc::new(openmodelica_frontend_types::Values::Value::NORETCALL))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerTock", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil }) => {
                    let true = (System::realtimeNtick(i.clone())? > 0) else { bail!("pattern mismatch") };
                    Ok(Arc::new(Values::Value::REAL { real: System::realtimeTock(i.clone())? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "timerTock", _) => {
                    Ok(Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(-1.0_f64) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (System::readFile((r#str.clone()).clone())?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "readFile", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "writeFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    System::writeFile((r#str.clone()).clone(), (str1.clone()).clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "writeFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: true }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    System::appendFile((r#str.clone()).clone(), (str1.clone()).clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "writeFile", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "deleteFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: System::removeFile((r#str.clone()).clone()) == 0 }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareFiles", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: System::fileContentsEqual((str1.clone()).clone(), (str2.clone()).clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareFilesAndMove", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut b: bool = false;
                    let true = (System::regularFileExists((str1.clone()).clone())) else { bail!("pattern mismatch") };
                    b = System::regularFileExists((str2.clone()).clone()) && System::fileContentsEqual((str1.clone()).clone(), (str2.clone()).clone());
                    b = if (!(b.clone())) {System::rename((str1.clone()).clone(), (str2.clone()).clone())} else {System::removeFile((str1.clone()).clone()) == 0};
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "compareFilesAndMove", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getErrorString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (Error::printMessagesStr(b.clone())).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "countMessages", _) => {
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::INTEGER { integer: Error::getNumMessages() }), Arc::new(Values::Value::INTEGER { integer: Error::getNumErrorMessages() }), Arc::new(Values::Value::INTEGER { integer: ErrorExt::getNumWarningMessages() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "clearMessages", Deref @ metamodelica::List::Nil) => {
                    Error::clearMessages();
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMessagesStringInternal", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: true }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut messages: Arc<metamodelica::List<ErrorTypes::TotalMessage>> = metamodelica::nil();
                    messages = List::unique(Error::getMessages());
                    Ok(ValuesMake::makeArray(List::map(messages.clone(), (std::sync::Arc::new(errorToValue) as std::sync::Arc<dyn ::std::ops::Fn(ErrorTypes::TotalMessage) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMessagesStringInternal", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(ValuesMake::makeArray(List::map(Error::getMessages(), (std::sync::Arc::new(errorToValue) as std::sync::Arc<dyn ::std::ops::Fn(ErrorTypes::TotalMessage) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringTypeName", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath((r#str.clone()).clone())? }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringVariableName", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::CODE { A: Arc::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: Parser::stringCref((r#str.clone()).clone())? }) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "typeNameString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "typeNameStrings", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(ValuesMake::makeArray(List::map(AbsynUtil::pathToStringList(path.clone())?, (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>)))?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateHeader", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Tpl::tplString((std::sync::Arc::new(Unparsing::programExternalHeader) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> + 'static>), SymbolTable::getSCode()?)?).clone();
                    System::writeFile((filename.clone()).clone(), (r#str.clone()).clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateHeader", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateJuliaHeader", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Tpl::tplString((std::sync::Arc::new(Unparsing::programExternalHeaderJulia) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Tpl::Text> + 'static>), SymbolTable::getSCode()?)?).clone();
                    System::writeFile((filename.clone()).clone(), (r#str.clone()).clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateJuliaHeader", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateCode", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let (__pa0, Util::SUCCESS { .. }) = (Static::instantiateDaeFunction(outCache.clone(), env.clone(), path.clone(), false, None, true)?) else { bail!("pattern mismatch") };
                    outCache = __pa0.clone();
                    (outCache, _, _) = cevalGenerateFunction(outCache.clone(), env.clone(), SymbolTable::getAbsyn(), path.clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateCode", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateScriptingAPI", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut str3: ArcStr = arcstr::literal!("");
                    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut ty: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
                    let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cl: Arc<SCode::Element>;
                    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut outCache: FCore::Cache = outCache.clone();
                    sp = SymbolTable::getSCode()?;
                    elts = (::match_deref::match_deref! { match &(FBuiltin::getElementWithPathCheckBuiltin(sp.clone(), className.clone())?) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => elts.clone(),
        cl => {
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(className.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" does not contain SCode.PARTS")); ArcStr::from(__mm_s) }).clone()], SCodeUtil::elementInfo(cl.clone()))?;
                    bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    tys = metamodelica::nil();
                    for mut elt in &*elts.clone() {
                        let mut elt = elt.clone();
                        let () = 'mc: {
        let __mc_input = elt.clone();
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { .. } }, partialPrefix: SCode::Partial::NOT_PARTIAL, .. } => {
                            let mut ty: Arc<DAE::Type>;
                            let mut outCache: FCore::Cache = outCache.clone();
                            let mut tys: Arc<metamodelica::List<Arc<DAE::Type>>> = tys.clone();
                            (outCache, ty, _) = Lookup::lookupType(outCache.clone(), env.clone(), AbsynUtil::suffixPath(className.clone(), (var_field!((*elt).name, SCode::Element::CLASS).clone()).clone())?, None)?;
                            if isSimpleAPIFunction(ty.clone())? {
                                        tys = cons(ty.clone(), tys.clone());
                            }
                            Ok(())
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            Ok(())
                        }
                        _ => bail!("nomatch"),
                    }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
                    }
                    str1 = (Tpl::tplString((std::sync::Arc::new(GenerateAPIFunctionsTpl::getCevalScriptInterface) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> + 'static>), tys.clone())?).clone();
                    str2 = (Tpl::tplString3((std::sync::Arc::new(GenerateAPIFunctionsTpl::getQtInterface) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<metamodelica::List<Arc<DAE::Type>>>, ArcStr, ArcStr) -> Result<Tpl::Text> + 'static>), tys.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("::")); ArcStr::from(__mm_s) }).clone(), (name.clone()).clone())?).clone();
                    str3 = (Tpl::tplString2((std::sync::Arc::new(GenerateAPIFunctionsTpl::getQtInterfaceHeaders) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<metamodelica::List<Arc<DAE::Type>>>, ArcStr) -> Result<Tpl::Text> + 'static>), tys.clone(), (name.clone()).clone())?).clone();
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: true }), Arc::new(Values::Value::STRING { string: (str1.clone()).clone() }), Arc::new(Values::Value::STRING { string: (str2.clone()).clone() }), Arc::new(Values::Value::STRING { string: (str3.clone()).clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateScriptingAPI", _) => {
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::BOOL { boolean: false }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateEntryPoint", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut r#str = (*r#str).clone();
                    r#str = (Tpl::tplString2((std::sync::Arc::new(CodegenCFunctions::generateEntryPoint) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<Absyn::Path>, ArcStr) -> Result<Tpl::Text> + 'static>), path.clone(), (r#str.clone()).clone())?).clone();
                    System::writeFile((filename.clone()).clone(), (r#str.clone()).clone())?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateEntryPoint", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkInterfaceOfPackages", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
                    let mut interfaceType: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cl: Arc<SCode::Element>;
                    let mut interfaceTypeAssoc: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
                    sp = SymbolTable::getSCode()?;
                    cl = SCodeUtil::getElementWithPath(sp.clone(), path.clone())?;
                    interfaceTypeAssoc = List::map1(vals.clone(), (std::sync::Arc::new(getInterfaceTypeAssocElt) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>, SourceInfo) -> Result<(ArcStr, Arc<metamodelica::List<ArcStr>>)> + 'static>), SCodeUtil::elementInfo(cl.clone()));
                    interfaceType = getInterfaceType(cl.clone(), interfaceTypeAssoc.clone())?;
                    List::map1_0(sp.clone(), (std::sync::Arc::new(verifyInterfaceType) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<metamodelica::List<ArcStr>>) -> Result<()> + 'static>), interfaceType.clone());
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkInterfaceOfPackages", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateSeparateCodeDependenciesMakefile", _) => {
                    Ok(generateSeparateCodeDependenciesMakefile(args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateSeparateCodeDependencies", _) => {
                    Ok(generateSeparateCodeDependencies(args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "generateSeparateCode", _) => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (v, outCache) = generateSeparateCode(args.clone(), outCache.clone(), env.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportedNames", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut cvars: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    (vals, cvars) = getImportedNames(ProgramUtil::getPathedClassInProgram(path.clone(), SymbolTable::getAbsyn(), false, false)?)?;
                    v = Arc::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(vals.clone())?, ValuesMake::makeArray(cvars.clone())?] });
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getImportedNames", _) => {
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![ValuesMake::makeArray(metamodelica::nil())?, ValuesMake::makeArray(metamodelica::nil())?] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMMfileTotalDependencies", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = getMMfileTotalDependencies((str1.clone()).clone(), (str2.clone()).clone())?;
                    vals = ({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut s in (strs.clone()).into_iter().cloned() {
                    let __x = Arc::new(Values::Value::STRING { string: (s.clone()).clone() });
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMMfileTotalDependencies", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: cvars, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    let mut pathstr: ArcStr = arcstr::literal!("");
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut b1: bool = false;
                    let mut oldLanguageStd: Config::LanguageStandard = Config::LanguageStandard::_1_x;
                    let mut b = (*b).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    p = SymbolTable::getAbsyn();
                    execStatReset()?;
                    pathstr = (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone();
                    strs = List::map(cvars.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>));
                    oldLanguageStd = Config::getLanguageStandard()?;
                    b1 = !(stringEq((r#str.clone()).clone(), (literal!("")).clone())?);
                    if b1.clone() {
                        Config::setLanguageStandard(Config::versionStringToStd((r#str.clone()).clone()))?;
                    }
                    (p, b) = loadModel(list![(path.clone(), literal!("call to loadModel"), strs.clone(), false)], (pathstr.clone()).clone(), p.clone(), true, b.clone(), true, requireExactVersion.clone(), false, (literal!("")).clone())?;
                    if b1.clone() {
                        Config::setLanguageStandard(oldLanguageStd.clone())?;
                    }
                    Print::clearBuf();
                    SymbolTable::setAbsyn(p.clone())?;
                    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("loadModel(")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                    outCache = FCore::emptyCache();
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadModel", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: _ }) => {
                    let mut pathstr: ArcStr = arcstr::literal!("");
                    pathstr = (AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOAD_MODEL_ERROR.clone(), list![(pathstr.clone()).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFile", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: allowWithin }, tail: _ } } } } } }) => {
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut name = (*name).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    execStatReset()?;
                    name = (Testsuite::friendlyPath((name.clone()).clone())?).clone();
                    newp = loadFile((name.clone()).clone(), (encoding.clone()).clone(), SymbolTable::getAbsyn(), b.clone(), b1.clone(), requireExactVersion.clone(), allowWithin.clone())?;
                    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("loadFile(")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                    SymbolTable::setAbsyn(newp.clone())?;
                    outCache = FCore::emptyCache();
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFile", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFiles", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ARRAY { valueLst: vals, .. }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: allowWithin }, tail: _ } } } } } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut newps: Arc<metamodelica::List<Absyn::Program>> = metamodelica::nil();
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut outCache: FCore::Cache = outCache.clone();
                    strs = List::mapMap(vals.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>), (std::sync::Arc::new(Testsuite::friendlyPath) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>));
                    newps = Parser::parallelParseFilesToProgramList(strs.clone(), (encoding.clone()).clone(), i.clone())?;
                    newp = SymbolTable::getAbsyn();
                    for mut p in &*newps.clone() {
                        let mut p = p.clone();
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(strs.clone()) {
                            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        r#str = __pa0.clone();
                        strs = __pa1.clone();
                        if !(allowWithin.clone()) {
                            checkTopClassWithin(p.clone(), (r#str.clone()).clone())?;
                        }
                        newp = checkUsesAndUpdateProgram(p.clone(), newp.clone(), b.clone(), (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone(), b1.clone(), requireExactVersion.clone(), false)?;
                    }
                    SymbolTable::setAbsyn(newp.clone())?;
                    outCache = FCore::emptyCache();
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadFiles", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseEncryptedPackage", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: _ } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut b: bool = false;
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut filename = (*filename).clone();
                    vals = metamodelica::nil();
                    r#str = (System::pwd()).clone();
                    match '__try0: {
                        filename = (unwrap_break_err!(System::realpath((filename.clone()).clone()), '__try0)).clone();
                        let 0 = (System::cd((System::dirname((filename.clone()).clone())).clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        (b, filename) = unwrap_break_err!(unZipEncryptedPackageAndCheckFile((workdir.clone()).clone(), (filename.clone()).clone(), false), '__try0);
                        if b.clone() {
                            Error::clearMessages();
                            Print::clearErrorBuf();
                            filename = (unwrap_break_err!(Testsuite::friendlyPath((filename.clone()).clone()), '__try0)).clone();
                            paths = unwrap_break_err!(Interactive::parseFile((filename.clone()).clone(), (literal!("UTF-8")).clone(), false), '__try0);
                            vals = List::map(paths.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeCodeTypeName, Arc<Absyn::Path>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>) -> Result<Arc<Values::Value>> + 'static>));
                        }
                        Ok::<_, anyhow::Error>((b.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            b = __try0_o0;
                        }
                        Err(_) => {
                            b = false;
                        }
                    }
                    let 0 = (System::cd((r#str.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(ValuesMake::makeArray(vals.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "parseEncryptedPackage", _) => {
                    Ok(ValuesMake::makeArray(metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadEncryptedPackage", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: workdir }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: bval }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: _ } } } } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut filename = (*filename).clone();
                    let mut b = (*b).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    r#str = (System::pwd()).clone();
                    match '__try0: {
                        filename = (unwrap_break_err!(System::realpath((filename.clone()).clone()), '__try0)).clone();
                        let 0 = (System::cd((System::dirname((filename.clone()).clone())).clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        (b, filename) = unwrap_break_err!(unZipEncryptedPackageAndCheckFile((workdir.clone()).clone(), (filename.clone()).clone(), bval.clone()), '__try0);
                        if b.clone() {
                            unwrap_break_err!(execStatReset(), '__try0);
                            filename = (unwrap_break_err!(Testsuite::friendlyPath((filename.clone()).clone()), '__try0)).clone();
                            p = SymbolTable::getAbsyn();
                            newp = unwrap_break_err!(loadFile((filename.clone()).clone(), (literal!("UTF-8")).clone(), p.clone(), b.clone(), b1.clone(), requireExactVersion.clone(), true), '__try0);
                            unwrap_break_err!(execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("loadFile(")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()), '__try0);
                            unwrap_break_err!(SymbolTable::setAbsyn(newp.clone()), '__try0);
                        }
                        outCache = FCore::emptyCache();
                        Ok::<_, anyhow::Error>((b.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            b = __try0_o0;
                        }
                        Err(_) => {
                            b = false;
                        }
                    }
                    let 0 = (System::cd((r#str.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadEncryptedPackage", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "alarm", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: System::alarm(i.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassNames", _) => {
                    Ok(getClassNames(args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reloadClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut filename: ArcStr = arcstr::literal!("");
                    let mut r1: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let mut r2: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?) {
                        Deref @ Absyn::Class { info: SourceInfo { lastModification: __pa0, fileName: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r2 = __pa0.clone();
                    filename = __pa1.clone();
                    let (true, _, __pa2, _) = (System::stat((filename.clone()).clone())) else { bail!("pattern mismatch") };
                    r1 = __pa2.clone();
                    if !(realEq(r1.clone(), r2.clone())) {
                        reloadClass((filename.clone()).clone(), (encoding.clone()).clone())?;
                    }
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reloadClass", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Nil } }) => {
                    if '__try0: {
                        unwrap_break_err!(ProgramUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    Error::addMessage(Error::LOAD_MODEL_ERROR.clone(), list![(AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone()])?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "reloadClass", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadString", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: name }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: encoding }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: mergeAST }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: requireExactVersion }, tail: _ } } } } } } }) => {
                    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut r#str = (*r#str).clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    r#str = (if (!(encoding.clone() == literal!("UTF-8"))) {System::iconv((r#str.clone()).clone(), (encoding.clone()).clone(), (literal!("UTF-8")).clone())} else {r#str.clone()}).clone();
                    newp = Parser::parsestring((r#str.clone()).clone(), (name.clone()).clone(), Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
                    newp = checkUsesAndUpdateProgram(newp.clone(), SymbolTable::getAbsyn(), b.clone(), (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone(), b1.clone(), requireExactVersion.clone(), mergeAST.clone())?;
                    SymbolTable::setAbsyn(newp.clone())?;
                    outCache = FCore::emptyCache();
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "loadString", _) => {
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "help", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "" }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (FlagsUtil::printUsage()?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "help", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    Ok(Arc::new(Values::Value::STRING { string: (FlagsUtil::printHelp(list![(r#str.clone()).clone()])?).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTimeStamp", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut r: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
                    let __pa0 = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(classpath.clone(), SymbolTable::getAbsyn(), false, false)?) {
                        Deref @ Absyn::Class { info: SourceInfo { lastModification: __pa0, .. }, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r = __pa0.clone();
                    r#str = (System::ctime(r.clone())).clone();
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: r.clone() }), Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getTimeStamp", _) => {
                    Ok(Arc::new(Values::Value::TUPLE { valueLst: list![Arc::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }), Arc::new(Values::Value::STRING { string: (literal!("")).clone() })] }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getClassRestriction", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (Interactive::getClassRestriction(classpath.clone(), SymbolTable::getAbsyn())).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "classAnnotationExists", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut b: bool = false;
                    b = ProgramUtil::getNamedAnnotationExp(classpath.clone(), SymbolTable::getAbsyn(), path.clone(), Some(false), std::sync::Arc::new(fnptr!(isSome, _)))?;
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBooleanClassAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut b: bool = false;
                    let __pa0 = ::match_deref::match_deref! { match &(ProgramUtil::getNamedAnnotationExp(classpath.clone(), SymbolTable::getAbsyn(), path.clone(), None, (std::sync::Arc::new(Interactive::getAnnotationExp) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<Absyn::Exp>> + 'static>))?) {
                        Deref @ Absyn::Exp::BOOL { value: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    b = __pa0.clone();
                    Ok(Arc::new(Values::Value::BOOL { boolean: b.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getBooleanClassAnnotation", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: classpath } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Nil } }) => {
                    Error::addMessage(Error::CLASS_ANNOTATION_DOES_NOT_EXIST.clone(), list![(AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?).clone(), (AbsynUtil::pathString(classpath.clone(), (literal!(".")).clone(), true, false)?).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "strtok", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: token }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = System::strtok((r#str.clone()).clone(), (token.clone()).clone());
                    Ok(ValuesMake::makeStringArray(strs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringSplit", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: token }, tail: Deref @ metamodelica::List::Nil } }) => {
                    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    strs = Util::stringSplitAtChar((r#str.clone()).clone(), (token.clone()).clone())?;
                    Ok(ValuesMake::makeStringArray(strs.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "stringReplace", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: str3 }, tail: Deref @ metamodelica::List::Nil } } }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = (System::stringReplace((str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone())?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "checkSettings", Deref @ metamodelica::List::Nil) => {
                    Ok(checkSettings()?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "echo", Deref @ metamodelica::List::Cons { head: v @ Deref @ Values::Value::BOOL { boolean: bval }, tail: Deref @ metamodelica::List::Nil }) => {
                    Settings::setEcho(if (bval.clone()) {1} else {0});
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "numProcessors", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::INTEGER { integer: Config::noProc()? }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScript", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut r#str = (*r#str).clone();
                    r#str = (Testsuite::friendlyPath((r#str.clone()).clone())?).clone();
                    res = (Interactive::evaluate(Parser::parseexp((r#str.clone()).clone())?, true)?).clone();
                    Ok(Arc::new(Values::Value::STRING { string: (res.clone()).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "runScript", _) => {
                    Ok(Arc::new(Values::Value::STRING { string: (literal!("Failed")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "exit", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::INTEGER { integer: i }, tail: Deref @ metamodelica::List::Nil }) => {
                    System::exit(i.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getMemorySize", Deref @ metamodelica::List::Nil) => {
                    Ok(Arc::new(Values::Value::REAL { real: System::getMemorySize() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getAllSubtypeOf", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: parentClass } }, tail: Deref @ metamodelica::List::Cons { head: _, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: includePartial }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: sort }, tail: Deref @ metamodelica::List::Nil } } } } }) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    paths = InteractiveUtil::getAllSubtypeOf(path.clone(), parentClass.clone(), SymbolTable::getAbsyn(), includePartial.clone(), sort.clone())?;
                    Ok(ValuesMake::makeCodeTypeNameArray(paths.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "getReplaceableChoices", Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: parentClass } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: includePartial }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: sort }, tail: Deref @ metamodelica::List::Nil } } } }) => {
                    Ok(InteractiveUtil::getReplaceableChoices(path.clone(), parentClass.clone(), SymbolTable::getAbsyn(), includePartial.clone(), sort.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(CevalScriptOMSimulator::ceval((functionName.clone()).clone(), args.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, v) = CevalScriptBackend::cevalInteractiveFunctions3(outCache.clone(), env.clone(), (functionName.clone()).clone(), args.clone(), msg.clone())?;
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

pub fn evalCodeTypeName(mut val: Arc<Values::Value>, mut env: FCore::Graph) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    res = 'mc: {
        let __mc_input = val.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: path @ Deref @ Absyn::Path::IDENT { name: _ } } } => {
                    let mut res: Arc<Values::Value> = res.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupVar(FCore::emptyCache(), env.clone(), ComponentReference::pathToCref(path.clone())?)?) {
                        (_, _, _, Deref @ DAE::Binding::VALBOUND { valBound: __pa0 @ Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { .. } }, .. }, _, _, _, _, _) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    res = __pa0.clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(val.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn getPackageVersion(mut path: Arc<Absyn::Path>, mut p: Absyn::Program) -> Result<ArcStr> {
    let mut version: ArcStr = literal!("");
    let mut evalParamAnn: bool = false;
    evalParamAnn = Config::getEvaluateParametersInAnnotations()?;
    Config::setEvaluateParametersInAnnotations(true)?;
    match '__try0: {
        let Absyn::STRING { value: __pa1 } = (unwrap_break_err!(ProgramUtil::getNamedAnnotationExp(path.clone(), p.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("version")).clone() }), Some(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() })), (std::sync::Arc::new(Interactive::getAnnotationExp) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<Arc<Absyn::Exp>> + 'static>)), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        version = __pa1.clone();
        Ok::<_, anyhow::Error>((version.clone(),))
    } {
        Ok((__try0_o0,)) => {
            version = __try0_o0;
        }
        Err(_) => {
            version = (literal!("")).clone();
        }
    }
    Config::setEvaluateParametersInAnnotations(evalParamAnn.clone())?;
    Ok(version)
}

fn errorToValue(mut err: ErrorTypes::TotalMessage) -> Result<Arc<Values::Value>> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = (match err.clone() {
        ErrorTypes::TotalMessage { msg: ErrorTypes::Message { id: mut id, ty: mut ty, severity: mut severity, message: mut message }, info: mut info } => {
            let mut msgpath: Arc<Absyn::Path>;
            let mut tyVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut severityVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut infoVal: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
            let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut msg_str: ArcStr = arcstr::literal!("");
            msg_str = (Gettext::translateContent(message.clone())?).clone();
            msgpath = Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("OpenModelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Scripting")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("ErrorMessage")).clone() }) }) }) });
            tyVal = errorTypeToValue(ty.clone())?;
            severityVal = errorLevelToValue(severity.clone())?;
            infoVal = infoToValue(info.clone())?;
            values = list![infoVal.clone(), Arc::new(Values::Value::STRING { string: (msg_str.clone()).clone() }), tyVal.clone(), severityVal.clone(), Arc::new(Values::Value::INTEGER { integer: id.clone() })];
            Arc::new(Values::Value::RECORD { record_: msgpath.clone(), orderd: values.clone(), comp: list![(literal!("info")).clone(), (literal!("message")).clone(), (literal!("kind")).clone(), (literal!("level")).clone(), (literal!("id")).clone()], index: -1 })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(val)
}

fn infoToValue(mut info: SourceInfo) -> Result<Arc<Values::Value>> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = (match info.clone() {
        SourceInfo { fileName: mut filename, isReadOnly: mut readonly, lineNumberStart: mut ls, columnNumberStart: mut cs, lineNumberEnd: mut le, columnNumberEnd: mut ce, lastModification: _ } => {
            let mut values: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
            let mut infopath: Arc<Absyn::Path>;
            infopath = Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("OpenModelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Scripting")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("SourceInfo")).clone() }) }) }) });
            values = list![Arc::new(Values::Value::STRING { string: (filename.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: readonly.clone() }), Arc::new(Values::Value::INTEGER { integer: ls.clone() }), Arc::new(Values::Value::INTEGER { integer: cs.clone() }), Arc::new(Values::Value::INTEGER { integer: le.clone() }), Arc::new(Values::Value::INTEGER { integer: ce.clone() })];
            Arc::new(Values::Value::RECORD { record_: infopath.clone(), orderd: values.clone(), comp: list![(literal!("filename")).clone(), (literal!("readonly")).clone(), (literal!("lineStart")).clone(), (literal!("columnStart")).clone(), (literal!("lineEnd")).clone(), (literal!("columnEnd")).clone()], index: -1 })
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(val)
}

fn makeErrorEnumLiteral(mut enumName: ArcStr, mut enumField: ArcStr, mut index: i32) -> Arc<Values::Value> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = Arc::new(Values::Value::ENUM_LITERAL { name: Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("OpenModelica")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Scripting")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (enumName.clone()).clone(), path: Arc::new(Absyn::Path::IDENT { name: (enumField.clone()).clone() }) }) }) }) }), index: index.clone() });
    val
}

fn errorTypeToValue(mut ty: ErrorTypes::MessageType) -> Result<Arc<Values::Value>> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = (match ty.clone() {
        ErrorTypes::MessageType::SYNTAX => makeErrorEnumLiteral((literal!("ErrorKind")).clone(), (literal!("syntax")).clone(), 1),
        ErrorTypes::MessageType::GRAMMAR => makeErrorEnumLiteral((literal!("ErrorKind")).clone(), (literal!("grammar")).clone(), 2),
        ErrorTypes::MessageType::TRANSLATION => makeErrorEnumLiteral((literal!("ErrorKind")).clone(), (literal!("translation")).clone(), 3),
        ErrorTypes::MessageType::SYMBOLIC => makeErrorEnumLiteral((literal!("ErrorKind")).clone(), (literal!("symbolic")).clone(), 4),
        ErrorTypes::MessageType::SIMULATION => makeErrorEnumLiteral((literal!("ErrorKind")).clone(), (literal!("runtime")).clone(), 5),
        ErrorTypes::MessageType::SCRIPTING => makeErrorEnumLiteral((literal!("ErrorKind")).clone(), (literal!("scripting")).clone(), 6),
        _ => {
            println!("{}", (literal!("errorTypeToValue failed\n")).clone());
            bail!("fail")
        },
    });
    Ok(val)
}

fn errorLevelToValue(mut severity: ErrorTypes::Severity) -> Result<Arc<Values::Value>> {
    let mut val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    val = (match severity.clone() {
        ErrorTypes::Severity::INTERNAL => makeErrorEnumLiteral((literal!("ErrorLevel")).clone(), (literal!("internal")).clone(), 1),
        ErrorTypes::Severity::ERROR => makeErrorEnumLiteral((literal!("ErrorLevel")).clone(), (literal!("error")).clone(), 2),
        ErrorTypes::Severity::WARNING => makeErrorEnumLiteral((literal!("ErrorLevel")).clone(), (literal!("warning")).clone(), 3),
        ErrorTypes::Severity::NOTIFICATION => makeErrorEnumLiteral((literal!("ErrorLevel")).clone(), (literal!("notification")).clone(), 4),
        _ => {
            println!("{}", (literal!("errorLevelToValue failed\n")).clone());
            bail!("fail")
        },
    });
    Ok(val)
}

fn generateFunctionName(mut functionPath: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut functionName: ArcStr = arcstr::literal!("");
    functionName = (AbsynUtil::pathStringUnquoteReplaceDot(functionPath.clone(), (literal!("_")).clone())?).clone();
    Ok(functionName)
}

fn generateFunctionFileName(mut functionPath: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut functionName: ArcStr = arcstr::literal!("");
    let mut n1: ArcStr = arcstr::literal!("");
    let mut n2: ArcStr = arcstr::literal!("");
    functionName = (AbsynUtil::pathStringUnquoteReplaceDot(functionPath.clone(), (literal!("_")).clone())?).clone();
    if ((functionName.clone()).clone().len() as i32) > Global::maxFunctionFileLength.clone() {
        n1 = (AbsynUtil::pathFirstIdent(functionPath.clone())?).clone();
        n2 = (AbsynUtil::pathLastIdent(functionPath.clone())?).clone();
        functionName = (System::unquoteIdentifier(({ let mut __mm_s = String::new(); __mm_s.push_str(&*n1.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*n2.clone()); ArcStr::from(__mm_s) }).clone())).clone();
        functionName = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*functionName.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(tick())); ArcStr::from(__mm_s) }).clone();
    }
    Ok(functionName)
}

pub fn getFunctionDependencies(mut cache: FCore::Cache, mut functionName: Arc<Absyn::Path>) -> Result<(DAE::Function, Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<AvlTreePathFunction::Tree>)> {
    let mut mainFunction: DAE::Function;
    let mut dependencies: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    funcs = FCore::getFunctionTree(cache.clone());
    mainFunction = DAEUtil::getNamedFunction(functionName.clone(), funcs.clone())?;
    dependencies = SimCodeFunction::getCalledFunctionsInFunction(functionName.clone(), funcs.clone())?;
    Ok((mainFunction, dependencies, funcs))
}

pub fn collectDependencies(mut inCache: FCore::Cache, mut env: FCore::Graph, mut functionName: Arc<Absyn::Path>) -> Result<(FCore::Cache, DAE::Function, Arc<metamodelica::List<DAE::Function>>, Arc<metamodelica::List<Arc<DAE::Type>>>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut mainFunction: DAE::Function;
    let mut dependencies: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
    let mut metarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
    let mut uniontypePaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
    (mainFunction, paths, funcs) = getFunctionDependencies(inCache.clone(), functionName.clone())?;
    dependencies = List::map1(paths.clone(), (std::sync::Arc::new(DAEUtil::getNamedFunction) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<AvlTreePathFunction::Tree>) -> Result<DAE::Function> + 'static>), funcs.clone());
    dependencies = List::setDifference(dependencies.clone(), list![mainFunction.clone()])?;
    uniontypePaths = DAEUtil::getUniontypePaths(dependencies.clone(), metamodelica::nil())?;
    (outCache, metarecordTypes) = Lookup::lookupMetarecordsRecursive(inCache.clone(), env.clone(), uniontypePaths.clone())?;
    Ok((outCache, mainFunction, dependencies, metarecordTypes))
}

pub fn cevalGenerateFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut program: Absyn::Program, mut inPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, ArcStr, ArcStr)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut functionName: ArcStr = arcstr::literal!("");
    let mut functionFileName: ArcStr = arcstr::literal!("");
    (outCache, functionName, functionFileName) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inPath.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, path) => {
                    if !((Flags::isSet(Flags::GEN.clone())? && !(Flags::isSet(Flags::GENERATE_CODE_CHEAT.clone())?))) { bail!("guard") }
                    let mut pathstr: ArcStr = arcstr::literal!("");
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut mainFunction: DAE::Function;
                    let mut dependencies: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut metarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    (cache, mainFunction, dependencies, metarecordTypes) = collectDependencies(cache.clone(), env.clone(), path.clone())?;
                    pathstr = (generateFunctionName(path.clone())?).clone();
                    fileName = (generateFunctionFileName(path.clone())?).clone();
                    FCore::getFunctionTree(cache.clone());
                    translateFunctions(program.clone(), (fileName.clone()).clone(), Some(mainFunction.clone()), dependencies.clone(), metarecordTypes.clone(), metamodelica::nil())?;
                    compileModel((fileName.clone()).clone(), metamodelica::nil(), (literal!("")).clone(), metamodelica::nil())?;
                    Ok((cache.clone(), pathstr.clone(), fileName.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, path) => {
                    if !((Flags::isSet(Flags::GEN.clone())? && Flags::isSet(Flags::GENERATE_CODE_CHEAT.clone())?)) { bail!("guard") }
                    let mut pathstr: ArcStr = arcstr::literal!("");
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut dependencies: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
                    let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
                    funcs = FCore::getFunctionTree(cache.clone());
                    pathstr = (generateFunctionName(path.clone())?).clone();
                    fileName = (generateFunctionFileName(path.clone())?).clone();
                    dependencies = DAEUtil::getFunctionList(funcs.clone(), false)?;
                    translateFunctions(program.clone(), (fileName.clone()).clone(), None, dependencies.clone(), metamodelica::nil(), metamodelica::nil())?;
                    Ok((cache.clone(), pathstr.clone(), fileName.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, path) => {
                    if !((Flags::isSet(Flags::GEN.clone())? && Flags::isSet(Flags::FAILTRACE.clone())?)) { bail!("guard") }
                    let mut pathstr: ArcStr = arcstr::literal!("");
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut cache = (*cache).clone();
                    let (__pa0, false) = (Static::isExternalObjectFunction(cache.clone(), env.clone(), path.clone())?) else { bail!("pattern mismatch") };
                    cache = __pa0.clone();
                    pathstr = (generateFunctionName(path.clone())?).clone();
                    fileName = (generateFunctionFileName(path.clone())?).clone();
                    Debug::trace(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.cevalGenerateFunction failed:\nfunction: ")); __mm_s.push_str(&*pathstr.clone()); __mm_s.push_str(&*literal!("\nfile: ")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, functionName, functionFileName))
}

fn matchQualifiedCalls(mut inExp: Arc<DAE::Exp>, mut inAcc: Arc<metamodelica::List<ArcStr>>) -> (Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>) {
    let mut outExp: Arc<DAE::Exp> = inExp.clone();
    let mut outAcc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outAcc = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name, .. } }, .. }, .. } => {
            List::consOnTrue(!(listMember((name.clone()).clone(), inAcc.clone())), (name.clone()).clone(), inAcc.clone())
        },
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name, .. } }, .. } => {
            List::consOnTrue(!(listMember((name.clone()).clone(), inAcc.clone())), (name.clone()).clone(), inAcc.clone())
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: false, .. }, componentRef: Deref @ DAE::ComponentRef::CREF_QUAL { ident: name, .. } } => {
            List::consOnTrue(!(listMember((name.clone()).clone(), inAcc.clone())), (name.clone()).clone(), inAcc.clone())
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name, .. } }, .. } => {
            List::consOnTrue(!(listMember((name.clone()).clone(), inAcc.clone())), (name.clone()).clone(), inAcc.clone())
        },
        _ => {
            inAcc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outAcc)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn instantiateDaeFunctions(mut icache: FCore::Cache, mut ienv: FCore::Graph, mut ipaths: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<FCore::Cache> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    outCache = (::match_deref::match_deref! { match &((icache.clone(), ienv.clone(), ipaths.clone())) {
        (cache, _, Deref @ metamodelica::List::Nil) => {
            cache.clone()
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: path, tail: paths }) => {
            let mut cache = (*cache).clone();
            let (__pa0, Util::SUCCESS { .. }) = (Static::instantiateDaeFunctionForceInst(cache.clone(), env.clone(), path.clone(), false, None, true)?) else { bail!("pattern mismatch") };
            cache = __pa0.clone();
            cache = instantiateDaeFunctions(cache.clone(), env.clone(), paths.clone())?;
            cache.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCache)
}

fn generateFunctions(mut icache: FCore::Cache, mut ienv: FCore::Graph, mut p: Absyn::Program, mut fullScodeProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut isp: Arc<metamodelica::List<Arc<SCode::Element>>>, mut cleanCache: bool) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env: FCore::Graph;
    (cache, env) = (::match_deref::match_deref! { match &((icache.clone(), ienv.clone(), isp.clone())) {
        (cache, env, Deref @ metamodelica::List::Nil) => {
            (cache.clone(), env.clone())
        },
        (cache, env, Deref @ metamodelica::List::Cons { head: cl @ Deref @ SCode::Element::CLASS { info, restriction: restr, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED, name, .. }, tail: sp }) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            let () = (match restr.clone() {
        SCode::Restriction::R_PACKAGE => (),
        SCode::Restriction::R_UNIONTYPE { .. } => (),
        _ => {
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Only package and uniontype is supported as top-level classes in OpenModelica.")).clone()], info.clone())?;
            bail!("fail")
        },
    });
            (cache, env) = generateFunctions2(cache.clone(), env.clone(), p.clone(), fullScodeProgram.clone(), cl.clone(), (name.clone()).clone(), info.clone(), cleanCache.clone())?;
            (cache, env) = generateFunctions(cache.clone(), env.clone(), p.clone(), fullScodeProgram.clone(), sp.clone(), cleanCache.clone())?;
            (cache.clone(), env.clone())
        },
        (_, _, Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::CLASS { info: info @ SourceInfo { fileName: file, .. }, name, encapsulatedPrefix: SCode::Encapsulated::NOT_ENCAPSULATED, .. }, tail: _ }) => {
            let mut n: i32 = 0;
            (n, _) = System::regex((file.clone()).clone(), (literal!("ModelicaBuiltin.mo$")).clone(), 1, false, false);
            Error::assertion(n.clone() > 0, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Not an encapsulated class (required for separate compilation): ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cache, env))
}

fn generateFunctions2(mut icache: FCore::Cache, mut ienv: FCore::Graph, mut p: Absyn::Program, mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>, mut cl: Arc<SCode::Element>, mut name: ArcStr, mut info: SourceInfo, mut cleanCache: bool) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env: FCore::Graph;
    (cache, env) = 'mc: {
        let __mc_input = (icache.clone(), ienv.clone(), info.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, SourceInfo { fileName: mut file, .. }) = __mc_input.clone() else { bail!("nomatch") };
            ::match_deref::match_deref! { match &(System::regex((file.clone()).clone(), (literal!("ModelicaBuiltin.mo$")).clone(), 1, false, false)) {
                (1, _) => (),
                _ => bail!("pattern mismatch"),
            } };
            Ok((cache.clone(), env.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (mut cache, mut env, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut dependencies: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut pathsMetarecord: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
            let mut funcs: Arc<AvlTreePathFunction::Tree> = Arc::new(AvlTreePathFunction::Tree::EMPTY);
            let mut d: Arc<metamodelica::List<DAE::Function>> = metamodelica::nil();
            let mut nameHeader: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut path: Arc<Absyn::Path>;
            let mut elements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let mut metarecords: Arc<metamodelica::List<Arc<DAE::Type>>> = metamodelica::nil();
            let mut t: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
            cache = if (cleanCache.clone()) {FCore::emptyCache()} else {cache.clone()};
            if SCodeUtil::isPartial(cl.clone()) {
                paths = metamodelica::nil();
                pathsMetarecord = metamodelica::nil();
            } else {
                path = Arc::new(Absyn::Path::FULLYQUALIFIED { path: Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }) });
                elements = getNonPartialElementsForInstantiatedClass(sp.clone(), cl.clone(), path.clone());
                (paths, pathsMetarecord) = List::fold22(elements.clone(), (std::sync::Arc::new(findFunctionsToCompile) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<Absyn::Path>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> + 'static>), path.clone(), sp.clone(), metamodelica::nil(), metamodelica::nil());
            }
            metarecords = metamodelica::nil();
            for mut mr in &*pathsMetarecord.clone() {
                let mut mr = mr.clone();
                (cache, t, _) = Lookup::lookupType(cache.clone(), env.clone(), mr.clone(), Some(info.clone()))?;
                metarecords = cons(t.clone(), metarecords.clone());
            }
            cache = instantiateDaeFunctions(cache.clone(), env.clone(), paths.clone())?;
            InstHashTable::release()?;
            funcs = FCore::getFunctionTree(cache.clone());
            d = List::map2(paths.clone(), (std::sync::Arc::new(DAEUtil::getNamedFunctionWithError) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<AvlTreePathFunction::Tree>, SourceInfo) -> Result<DAE::Function> + 'static>), funcs.clone(), info.clone());
            let (_, (_, __pa0)) = DAEUtil::traverseDAEFunctions(d.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(matchQualifiedCalls, Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<DAE::Exp>, Arc<metamodelica::List<ArcStr>>)> + 'static>), metamodelica::nil()));
            dependencies = __pa0.clone();
            dependencies = List::sort(dependencies.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
            dependencies = List::map1(dependencies.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!(".h")).clone());
            nameHeader = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".h")); ArcStr::from(__mm_s) }).clone();
            strs = List::map1r(cons((nameHeader.clone()).clone(), dependencies.clone()), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("$(GEN_DIR)")).clone());
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".deps")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("$(GEN_DIR)")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".o: $(GEN_DIR)")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".c")); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*stringDelimitList(strs.clone(), (literal!(" ")).clone())); ArcStr::from(__mm_s) }).clone())?;
            dependencies = List::map1(dependencies.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("\"")).clone());
            dependencies = List::map1r(dependencies.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("#include \"")).clone());
            translateFunctions(p.clone(), (name.clone()).clone(), None, d.clone(), metamodelica::nil(), dependencies.clone())?;
            r#str = (Tpl::tplString((std::sync::Arc::new(Unparsing::programExternalHeaderFromTypes) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, Arc<metamodelica::List<Arc<DAE::Type>>>) -> Result<Tpl::Text> + 'static>), metarecords.clone())?).clone();
            System::writeFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_records.c")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#include <meta/meta_modelica.h>\n")); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone())?;
            cache = if (cleanCache.clone()) {icache.clone()} else {cache.clone()};
            Ok((cache.clone(), env.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Error::addSourceMessage(Error::SEPARATE_COMPILATION_PACKAGE_FAILED.clone(), list![(name.clone()).clone()], info.clone())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((cache, env))
}

fn findFunctionsToCompile(mut elt: Arc<SCode::Element>, mut pathPrefix: Arc<Absyn::Path>, mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>, mut acc: Arc<metamodelica::List<Arc<Absyn::Path>>>, mut accMetarecord: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut pathsMetarecord: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut path: Arc<Absyn::Path>;
    let mut elements: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    path = AbsynUtil::joinPaths(pathPrefix.clone(), Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }))?;
    paths = if (SCodeUtil::isFunction(elt.clone())) {cons(path.clone(), acc.clone())} else {acc.clone()};
    pathsMetarecord = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { .. }, .. } => cons(path.clone(), accMetarecord.clone()),
        _ => accMetarecord.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elements = getNonPartialElementsForInstantiatedClass(sp.clone(), elt.clone(), path.clone());
    (paths, pathsMetarecord) = List::fold22(elements.clone(), (std::sync::Arc::new(findFunctionsToCompile) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, Arc<Absyn::Path>, Arc<metamodelica::List<Arc<SCode::Element>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::Path>>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> + 'static>), path.clone(), sp.clone(), paths.clone(), pathsMetarecord.clone());
    Ok((paths, pathsMetarecord))
}

fn getNonPartialElementsForInstantiatedClass(mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>, mut cl: Arc<SCode::Element>, mut p: Arc<Absyn::Path>) -> Arc<metamodelica::List<Arc<SCode::Element>>> {
    let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut env: FCore::Graph;
    let mut skip: bool = false;
    let mut eltsTmp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    skip = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::CLASS_EXTENDS { .. }, .. } => false,
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: eltsTmp, .. }, .. } => !(List::any(eltsTmp.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isElementExtendsOrClassExtends, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>))),
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if !(skip.clone()) {
        if '__try0: {
            ErrorExt::setCheckpoint((literal!("getNonPartialElementsForInstantiatedClass")).clone());
            (_, env, _, _) = unwrap_break_err!(Inst::instantiateClass(FCore::emptyCache(), InnerOuter::emptyInstHierarchy().clone(), sp.clone(), AbsynUtil::makeNotFullyQualified(p.clone()), false, true, false), '__try0);
            elts = FCore::RefTree::fold(FNode::children(FNode::fromRef(FGraph::lastScopeRef(env.clone()).unwrap()).unwrap()).unwrap(), (std::sync::Arc::new(addNonPartialClassRef) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, metamodelica::Array<FCore::Node>, Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> + 'static>), metamodelica::nil());
            ErrorExt::rollBack((literal!("getNonPartialElementsForInstantiatedClass")).clone());
            return elts.clone();
            Ok::<(), anyhow::Error>(())
        }.is_err() {
        }
        ErrorExt::rollBack((literal!("getNonPartialElementsForInstantiatedClass")).clone());
    }
    elts = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, .. } => ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (elts.clone()).into_iter().cloned() {
            if !(!(SCodeUtil::isPartial(e.clone())) && SCodeUtil::isClass(e.clone())) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    elts
}

fn addNonPartialClassRef(mut name: ArcStr, mut r#ref: metamodelica::Array<FCore::Node>, mut accum: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<Arc<SCode::Element>>>> {
    let mut classes: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut e: Arc<SCode::Element>;
    classes = (::match_deref::match_deref! { match &(FNode::fromRef(r#ref.clone())?) {
        FCore::Node { data: FCore::Data::CL { e: e @ Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::NOT_PARTIAL, .. }, .. }, .. } => cons(e.clone(), accum.clone()),
        _ => accum.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(classes)
}

pub fn cevalCallFunction(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut r#impl: bool, mut inMsg: Absyn::Msg, mut numIter: i32) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inValuesValueLst.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, .. }, vallst, msg) => {
                    let mut newval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut cache = (*cache).clone();
                    (cache, newval) = Ceval::cevalKnownExternalFuncs(cache.clone(), env.clone(), funcpath.clone(), vallst.clone(), msg.clone())?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { path: funcpath, .. }, _, msg) => {
                    let true = (FGraph::isNotEmpty(env.clone())) else { bail!("pattern mismatch") };
                    cevalIsExternalObjectConstructor(cache.clone(), funcpath.clone(), env.clone(), msg.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: complexName }, .. }, .. }, path: funcpath, .. }, pubVallst, msg) => {
                    let mut expl: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
                    let mut vallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut proVallst: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
                    let mut pubVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut proVarLst: Arc<metamodelica::List<Arc<DAE::Var>>> = metamodelica::nil();
                    let mut pubVarNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut proVarNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut varNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut cache = (*cache).clone();
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: record constructor: func: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" type path: ")); __mm_s.push_str(&*AbsynUtil::pathString(complexName.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    let true = (AbsynUtil::pathEqual(funcpath.clone(), complexName.clone())) else { bail!("pattern mismatch") };
                    (pubVarLst, proVarLst) = List::splitOnTrue(varLst.clone(), (std::sync::Arc::new(Types::isPublicVar) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<bool> + 'static>));
                    expl = List::map1(proVarLst.clone(), (std::sync::Arc::new(Types::getBindingExp) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>, Arc<Absyn::Path>) -> Result<Arc<DAE::Exp>> + 'static>), funcpath.clone());
                    (cache, proVallst) = Ceval::cevalList(cache.clone(), env.clone(), expl.clone(), r#impl.clone(), msg.clone(), numIter.clone())?;
                    pubVarNames = List::map(pubVarLst.clone(), (std::sync::Arc::new(Expression::varName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    proVarNames = List::map(proVarLst.clone(), (std::sync::Arc::new(Expression::varName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::Var>) -> Result<ArcStr> + 'static>));
                    varNames = listAppend(pubVarNames.clone(), proVarNames.clone());
                    vallst = listAppend(pubVallst.clone(), proVallst.clone());
                    Ok((cache.clone(), Arc::new(Values::Value::RECORD { record_: funcpath.clone(), orderd: vallst.clone(), comp: varNames.clone(), index: -1 })))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, ty, .. }, path: funcpath, .. }, _, msg) => {
                    let mut newval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut bIsCompleteFunction: bool = false;
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), funcpath.clone(), env.clone(), msg.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: try to evaluate or generate function: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    bIsCompleteFunction = isCompleteFunction(cache.clone(), env.clone(), funcpath.clone())?;
                    let false = (Types::hasMetaArray(ty.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: is complete function: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*if (bIsCompleteFunction.clone()) {literal!("[true]")} else {literal!("[false]")}); ArcStr::from(__mm_s) }).clone())?;
                    }
                    (cache, newval) = cevalCallFunctionEvaluateOrGenerate(inCache.clone(), inEnv.clone(), inExp.clone(), inValuesValueLst.clone(), r#impl.clone(), inMsg.clone(), bIsCompleteFunction.clone())?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, path: funcpath, .. }, _, msg) => {
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), funcpath.clone(), env.clone(), msg.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    let false = (isCompleteFunction(cache.clone(), env.clone(), funcpath.clone())?) else { bail!("pattern mismatch") };
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        Debug::traceln(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CALL: constant evaluation failed (not complete function): ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); ArcStr::from(__mm_s) }).clone())?;
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalCallFunctionEvaluateOrGenerate(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut r#impl: bool, mut inMsg: Absyn::Msg, mut bIsCompleteFunction: bool) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut numCheckpoints: i32 = 0;
    if isNone(openmodelica_util::Globals::stackoverFlowIndex.with(|__root| __root.borrow().clone())) {
        { let __v = Some(1); openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v) };
        numCheckpoints = ErrorExt::getNumCheckpoints();
        match '__try0: {
            StackOverflow::clearStacktraceMessages();
            (outCache, outValue) = unwrap_break_err!(cevalCallFunctionEvaluateOrGenerate2(inCache.clone(), inEnv.clone(), inExp.clone(), inValuesValueLst.clone(), r#impl.clone(), inMsg.clone(), bIsCompleteFunction.clone()), '__try0);
            Ok::<_, anyhow::Error>((outCache.clone(), outValue.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                outCache = __try0_o0;
                outValue = __try0_o1;
            }
            Err(_) => {
                { let __v = None; openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v) };
                ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints.clone());
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Stack overflow when evaluating function call: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("...\n")); __mm_s.push_str(&*stringDelimitList(StackOverflow::readableStacktraceMessages()?, (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), (match inMsg.clone() {
        Absyn::Msg::MSG { info: mut info } => {
            info.clone()
        },
        _ => {
            metamodelica::sourceInfo!()
        },
    }))?;
                StackOverflow::clearStacktraceMessages();
                outCache = inCache.clone();
                outValue = Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL);
            }
        }
        { let __v = None; openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v) };
    } else {
        (outCache, outValue) = cevalCallFunctionEvaluateOrGenerate2(inCache.clone(), inEnv.clone(), inExp.clone(), inValuesValueLst.clone(), r#impl.clone(), inMsg.clone(), bIsCompleteFunction.clone())?;
    }
    Ok((outCache, outValue))
}

fn cevalCallFunctionEvaluateOrGenerate2(mut inCache: FCore::Cache, mut inEnv: FCore::Graph, mut inExp: Arc<DAE::Exp>, mut inValuesValueLst: Arc<metamodelica::List<Arc<Values::Value>>>, mut r#impl: bool, mut inMsg: Absyn::Msg, mut bIsCompleteFunction: bool) -> Result<(FCore::Cache, Arc<Values::Value>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outValue: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    (outCache, outValue) = 'mc: {
        let __mc_input = (inCache.clone(), inEnv.clone(), inExp.clone(), inValuesValueLst.clone(), inMsg.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, path: funcpath, .. }, vallst, msg) => {
                    let mut newval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut sc: Arc<SCode::Element>;
                    let mut func: DAE::Function;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let true = (Flags::isSet(Flags::EVAL_FUNC.clone())?) else { bail!("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), funcpath.clone(), env.clone(), msg.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    match '__try1: {
                        func = unwrap_break_err!(FCore::getCachedInstFunc(cache.clone(), funcpath.clone()), '__try1);
                        Ok::<_, anyhow::Error>((func.clone(),))
                    } {
                        Ok((__try1_o0,)) => {
                            func = __try1_o0;
                        }
                        Err(_) => {
                            let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), funcpath.clone(), None)?) {
                                        (__pa2, __pa3 @ Deref @ SCode::Element::CLASS { partialPrefix: SCode::Partial::NOT_PARTIAL, .. }, __pa4) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                                        _ => bail!("pattern mismatch"),
                            } };
                            cache = __pa2.clone();
                            sc = __pa3.clone();
                            env = __pa4.clone();
                            isCevaluableFunction(sc.clone())?;
                            (cache, env, _) = InstFunction::implicitFunctionInstantiation(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, sc.clone(), metamodelica::nil())?;
                            func = FCore::getCachedInstFunc(cache.clone(), funcpath.clone())?;
                        }
                    }
                    (cache, newval) = CevalFunction::evaluate(cache.clone(), env.clone(), func.clone(), vallst.clone())?;
                    Ok((cache.clone(), newval.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: false, .. }, path: funcpath, .. }, vallst, msg) => {
                    if !((bIsCompleteFunction.clone() && Flags::isSet(Flags::GEN.clone())?)) { bail!("guard") }
                    let mut newval: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
                    let mut print_debug: bool = false;
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    let mut libHandle: i32 = 0;
                    let mut funcHandle: i32 = 0;
                    let mut funcstr: ArcStr = arcstr::literal!("");
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let mut w: Absyn::Within = Absyn::Within::TOP;
                    let mut cache = (*cache).clone();
                    if '__try0: {
                        unwrap_break_err!(cevalIsExternalObjectConstructor(cache.clone(), funcpath.clone(), env.clone(), msg.clone()), '__try0);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: [SOME SYMTAB] not in in CF list: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    p = SymbolTable::getAbsyn();
                    (cache, funcstr, fileName) = cevalGenerateFunction(cache.clone(), env.clone(), p.clone(), funcpath.clone())?;
                    print_debug = Flags::isSet(Flags::DYN_LOAD.clone())?;
                    libHandle = System::loadLibrary(({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }).clone(), true, print_debug.clone())?;
                    funcHandle = System::lookupFunction(libHandle.clone(), (stringAppend((literal!("in_")).clone(), (funcstr.clone()).clone())).clone())?;
                    execStatReset()?;
                    newval = DynLoad::executeFunction(funcHandle.clone(), vallst.clone(), print_debug.clone())?;
                    execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("executeFunction(")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
                    System::freeLibrary(libHandle.clone(), print_debug.clone())?;
                    let __pa1 = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(funcpath.clone(), p.clone(), false, false)?) {
                        Deref @ Absyn::Class { info: __pa1, restriction: Absyn::Restriction::R_FUNCTION { functionRestriction: _ }, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa1.clone();
                    w = ProgramUtil::buildWithin(funcpath.clone())?;
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: Updating build time for function path: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(" within: ")); __mm_s.push_str(&*Dump::unparseWithin(w.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    AbsynUtil::getFileNameFromInfo(info.clone())?;
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: [SOME SYMTAB] not in in CF list [finished]: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    Ok((cache.clone(), newval.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Exp::CALL { path: funcpath, .. }, _, _) => {
                    if Flags::isSet(Flags::DYN_LOAD.clone())? {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[dynload]: FAILED to constant evaluate function: ")); __mm_s.push_str(&*AbsynUtil::pathString(funcpath.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    }
                    let false = (Flags::isSet(Flags::GEN.clone())?) else { bail!("pattern mismatch") };
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("- codegeneration is turned off. switch \"nogen\" flag off\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outCache, outValue))
}

fn cevalIsExternalObjectConstructor(mut cache: FCore::Cache, mut funcpath: Arc<Absyn::Path>, mut env: FCore::Graph, mut msg: Absyn::Msg) -> Result<()> {
    let mut funcpath2: Arc<Absyn::Path>;
    let mut tp: Arc<DAE::Type> = Arc::new(DAE::Type::T_NORETCALL);
    let mut info: Option<SourceInfo> = None;
    let () = (match (env.clone(), msg.clone()) {
        (FCore::Graph::EG { name: _ }, Absyn::Msg::NO_MSG) => bail!("fail"),
        (_, Absyn::Msg::NO_MSG) => {
            let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::splitQualAndIdentPath(funcpath.clone())?) {
                (__pa0, Deref @ Absyn::Path::IDENT { name: Deref @ "constructor" }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            funcpath2 = __pa0.clone();
            info = if (msg.clone() == openmodelica_ast::Absyn::Msg::NO_MSG) {None} else {Some(Absyn::dummyInfo.clone())};
            (_, tp, _) = Lookup::lookupType(cache.clone(), env.clone(), funcpath2.clone(), info.clone())?;
            Types::externalObjectConstructorType(tp.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn checkLibraryUsage(mut inLibrary: ArcStr, mut inExp: Arc<Absyn::Exp>) -> Result<bool> {
    let mut isUsed: bool = false;
    isUsed = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::STRING { value: s } => {
            stringEq((s.clone()).clone(), (inLibrary.clone()).clone())?
        },
        Deref @ Absyn::Exp::ARRAY { arrayExp: exps } => {
            List::isMemberOnTrue((inLibrary.clone()).clone(), exps.clone(), (std::sync::Arc::new(checkLibraryUsage) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::Exp>) -> Result<bool> + 'static>))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(isUsed)
}

fn isCevaluableFunction(mut inElement: Arc<SCode::Element>) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { externalDecl: Some(Deref @ SCode::ExternalDecl { annotation_: Some(Deref @ SCode::Annotation { modification: r#mod }), funcName: Some(fid), .. }), .. }, restriction: SCode::Restriction::R_FUNCTION { functionRestriction: SCode::FunctionRestriction::FR_EXTERNAL_FUNCTION { purity: _ } }, .. } => {
            let mut lib: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(r#mod.clone(), (literal!("Library")).clone())?) {
                Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            lib = __pa0.clone();
            let true = (checkLibraryUsage((literal!("Lapack")).clone(), lib.clone())? || checkLibraryUsage((literal!("lapack")).clone(), lib.clone())?) else { bail!("pattern mismatch") };
            isCevaluableFunction2((fid.clone()).clone())?;
            ()
        },
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. } => {
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn isCevaluableFunction2(mut inFuncName: ArcStr) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inFuncName.clone()) {
        Deref @ "dgbsv" => (),
        Deref @ "dgeev" => (),
        Deref @ "dgegv" => (),
        Deref @ "dgels" => (),
        Deref @ "dgelsx" => (),
        Deref @ "dgelsy" => (),
        Deref @ "dgeqpf" => (),
        Deref @ "dgesv" => (),
        Deref @ "dgesvd" => (),
        Deref @ "dgetrf" => (),
        Deref @ "dgetri" => (),
        Deref @ "dgetrs" => (),
        Deref @ "dgglse" => (),
        Deref @ "dgtsv" => (),
        Deref @ "dorgqr" => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn isSimpleAPIFunction(mut ty: Arc<DAE::Type>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. }, .. }, .. } => isSimpleAPIFunctionArg(var_field!((*ty).funcResultType, DAE::Type::T_FUNCTION).clone()) && ({
        let mut __acc: Option<bool> = None;
        for mut fa in (var_field!((*ty).funcArg, DAE::Type::T_FUNCTION).clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(fa.clone()) {
        Deref @ DAE::FuncArg { .. } => isSimpleAPIFunctionArg(fa.ty.clone()),
        _ => bail!("match: no arm matched"),
    } });
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    }),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isSimpleAPIFunctionArg(mut ty: Arc<DAE::Type>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(ty.clone()) {
        Deref @ DAE::Type::T_INTEGER { .. } => true,
        Deref @ DAE::Type::T_REAL { .. } => true,
        Deref @ DAE::Type::T_BOOL { .. } => true,
        Deref @ DAE::Type::T_STRING { .. } => true,
        Deref @ DAE::Type::T_NORETCALL => true,
        Deref @ DAE::Type::T_ARRAY { .. } => isSimpleAPIFunctionArg(var_field!((*ty).ty, DAE::Type::T_ARRAY).clone()),
        Deref @ DAE::Type::T_CODE { ty: DAE::CodeType::C_TYPENAME } => true,
        Deref @ DAE::Type::T_TUPLE { .. } => ({
        let mut __acc: Option<bool> = None;
        for mut t in (var_field!((*ty).types, DAE::Type::T_TUPLE).clone()).into_iter().cloned() {
            let __x = isSimpleAPIFunctionArg(t.clone());
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction")).unwrap()
    }),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn verifyInterfaceType(mut elt: Arc<SCode::Element>, mut expected: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (elt.clone(), expected.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_METARECORD { moved: true, .. }, .. }, _) => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. }, Deref @ metamodelica::List::Cons { head: name, tail: _ }) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotation(ann.clone(), (literal!("__OpenModelica_Interface")).clone())?) {
                        Deref @ SCode::Mod::MOD { info: __pa0, binding: Some(Deref @ Absyn::Exp::STRING { value: __pa1 }), .. } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    info = __pa0.clone();
                    r#str = __pa1.clone();
                    Error::assertionOrAddSourceMessage(listMember((r#str.clone()).clone(), expected.clone()), Error::MISMATCHING_INTERFACE_TYPE.clone(), list![(r#str.clone()).clone(), (name.clone()).clone()], info.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::unparseElementStr(elt.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Error::addSourceMessage(Error::MISSING_INTERFACE_TYPE.clone(), metamodelica::nil(), SCodeUtil::elementInfo(elt.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn getInterfaceType(mut elt: Arc<SCode::Element>, mut assoc: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut it: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    it = 'mc: {
        let __mc_input = elt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { cmt: Deref @ SCode::Comment { annotation_: Some(ann), .. }, .. } => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut it: Arc<metamodelica::List<ArcStr>> = it.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::lookupAnnotationBinding(ann.clone(), (literal!("__OpenModelica_Interface")).clone())?) {
                        Some(Deref @ Absyn::Exp::STRING { value: __pa0 }) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    it = Util::assoc((r#str.clone()).clone(), assoc.clone())?;
                    Ok(it.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(Error::MISSING_INTERFACE_TYPE.clone(), metamodelica::nil(), SCodeUtil::elementInfo(elt.clone()))?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(it)
}

fn getInterfaceTypeAssocElt(mut val: Arc<Values::Value>, mut info: SourceInfo) -> Result<(ArcStr, Arc<metamodelica::List<ArcStr>>)> {
    let mut assoc: (ArcStr, Arc<metamodelica::List<ArcStr>>);
    assoc = (::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: Deref @ "" }, tail: _ }, .. } => {
            Error::addSourceMessage(Error::MISSING_INTERFACE_TYPE.clone(), metamodelica::nil(), info.clone())?;
            bail!("fail")
        },
        Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: r#str }, tail: vals }, .. } => {
            let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strs = List::select(List::map(vals.clone(), (std::sync::Arc::new(ValuesUtil::extractValueString) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Values::Value>) -> Result<ArcStr> + 'static>)), (std::sync::Arc::new(fnptr!(Util::isNotEmptyString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>));
            (r#str.clone(), cons((r#str.clone()).clone(), strs.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(assoc)
}

fn buildDependencyGraph(mut name: ArcStr, mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut edges: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    edges = (::match_deref::match_deref! { match &(sp.clone()) {
        _ => {
            let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue((name.clone()).clone(), sp.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isClassNamed, ArcStr, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SCode::Element>) -> Result<bool> + 'static>))?) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa0, .. }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elts = __pa0.clone();
            elts = ({
        let mut __acc: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
        for mut e in (elts.clone()).into_iter().cloned() {
            if !(SCodeUtil::isImport(e.clone())) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            List::map(elts.clone(), (std::sync::Arc::new(importDependency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(edges)
}

fn buildDependencyGraphPublicImports(mut name: ArcStr, mut sp: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut edges: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    edges = (::match_deref::match_deref! { match &(sp.clone()) {
        _ => {
            let mut elts: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            let __pa0 = ::match_deref::match_deref! { match &(List::getMemberOnTrue((name.clone()).clone(), sp.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isClassNamed, ArcStr, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SCode::Element>) -> Result<bool> + 'static>))?) {
                Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa0, .. }, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elts = __pa0.clone();
            elts = List::select(elts.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::elementIsPublicImport, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>));
            List::map(elts.clone(), (std::sync::Arc::new(importDependency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(edges)
}

fn buildTransitiveDependencyGraph(mut name: ArcStr, mut oldgraph: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut edges: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    edges = 'mc: {
        let __mc_input = oldgraph.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(List::setDifference(Graph::allReachableNodes((list![(name.clone()).clone()], metamodelica::nil()), oldgraph.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, list![(name.clone()).clone()])?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.buildTransitiveDependencyGraph failed: ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(edges)
}

fn importDependency(mut simp: Arc<SCode::Element>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((::match_deref::match_deref! { match &(simp.clone()) {
        Deref @ SCode::Element::IMPORT { imp: Absyn::Import::NAMED_IMPORT { path, .. }, .. } => {
            AbsynUtil::pathFirstIdent(path.clone())?
        },
        Deref @ SCode::Element::IMPORT { imp: Absyn::Import::NAMED_IMPORT { path, .. }, .. } => {
            AbsynUtil::pathFirstIdent(path.clone())?
        },
        Deref @ SCode::Element::IMPORT { imp: Absyn::Import::QUAL_IMPORT { path }, .. } => {
            AbsynUtil::pathFirstIdent(path.clone())?
        },
        Deref @ SCode::Element::IMPORT { imp: Absyn::Import::UNQUAL_IMPORT { path }, .. } => {
            AbsynUtil::pathFirstIdent(path.clone())?
        },
        Deref @ SCode::Element::IMPORT { imp: Absyn::Import::GROUP_IMPORT { prefix: path, .. }, .. } => {
            AbsynUtil::pathFirstIdent(path.clone())?
        },
        Deref @ SCode::Element::IMPORT { info, imp, .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.importDependency could not handle:")); __mm_s.push_str(&*Dump::unparseImportStr(imp.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()], info.clone())?;
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(name)
}

fn compareNumberOfDependencies(mut node1: (ArcStr, Arc<metamodelica::List<ArcStr>>), mut node2: (ArcStr, Arc<metamodelica::List<ArcStr>>)) -> bool {
    let mut cmp: bool = false;
    let mut deps1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut deps2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (_, deps1) = node1.clone();
    (_, deps2) = node2.clone();
    cmp = (deps1.clone().len() as i32) >= (deps2.clone().len() as i32);
    cmp
}

fn compareDependencyNode(mut node1: (ArcStr, Arc<metamodelica::List<ArcStr>>), mut node2: (ArcStr, Arc<metamodelica::List<ArcStr>>)) -> bool {
    let mut cmp: bool = false;
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    (s1, _) = node1.clone();
    (s2, _) = node2.clone();
    cmp = Util::strcmpBool((s1.clone()).clone(), (s2.clone()).clone());
    cmp
}

fn dependencyString(mut deps: (ArcStr, Arc<metamodelica::List<ArcStr>>)) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (r#str, strs) = deps.clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString((strs.clone().len() as i32))); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*stringDelimitList(strs.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn transitiveDependencyString(mut deps: (ArcStr, Arc<metamodelica::List<ArcStr>>)) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (r#str, strs) = deps.clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString((strs.clone().len() as i32))); __mm_s.push_str(&*literal!(": (")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*stringDelimitList(strs.clone(), (literal!(",")).clone())); ArcStr::from(__mm_s) }).clone();
    r#str
}

fn containsPublicInterface(mut elt: Arc<SCode::Element>) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED, restriction: SCode::Restriction::R_PACKAGE, .. } => {
            List::any(elts.clone(), (std::sync::Arc::new(fnptr!(containsPublicInterface2, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>))
        },
        _ => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (SCodeUtil::elementName(elt.clone())?).clone();
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.containsPublicInterface failed: ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(name.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn containsPublicInterface2(mut elt: Arc<SCode::Element>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::IMPORT { .. } => false,
        Deref @ SCode::Element::EXTENDS { .. } => false,
        Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_FUNCTION { functionRestriction: _ }, .. } => false,
        Deref @ SCode::Element::COMPONENT { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC, .. }, .. } => true,
        Deref @ SCode::Element::CLASS { prefixes: Deref @ SCode::Prefixes { visibility: SCode::Visibility::PUBLIC, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn containsImport(mut elt: Arc<SCode::Element>, mut visibility: SCode::Visibility) -> Result<bool> {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, encapsulatedPrefix: SCode::Encapsulated::ENCAPSULATED, restriction: SCode::Restriction::R_PACKAGE, .. } => {
            List::exist1(elts.clone(), (std::sync::Arc::new(fnptr!(containsImport2, Arc<SCode::Element>, SCode::Visibility)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, SCode::Visibility) -> Result<bool> + 'static>), visibility.clone())
        },
        _ => {
            let mut name: ArcStr = arcstr::literal!("");
            name = (SCodeUtil::elementName(elt.clone())?).clone();
            name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CevalScript.containsPublicInterface failed: ")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(name.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn containsImport2(mut elt: Arc<SCode::Element>, mut visibility: SCode::Visibility) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((elt.clone(), visibility.clone())) {
        (Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PUBLIC, .. }, SCode::Visibility::PUBLIC) => true,
        (Deref @ SCode::Element::IMPORT { visibility: SCode::Visibility::PROTECTED, .. }, SCode::Visibility::PROTECTED) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn printInterfaceString(mut elt: Arc<SCode::Element>) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ SCode::Element::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*boolString(containsPublicInterface(elt.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    Ok(())
}

fn writeModuleDepends(mut cl: Arc<SCode::Element>, mut prefix: ArcStr, mut suffix: ArcStr, mut deps: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ('mc: {
        let __mc_input = cl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info: SourceInfo { .. }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, name, .. } => {
                    let mut allDepends: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut protectedDepends: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut r#str: ArcStr = r#str.clone();
                    protectedDepends = List::map(List::select(elts.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::elementIsProtectedImport, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>)), (std::sync::Arc::new(importDependency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
                    protectedDepends = List::select(protectedDepends.clone(), (std::sync::Arc::new(fnptr!(isNotBuiltinImport, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>));
                    let __pa0 = ::match_deref::match_deref! { match &(Graph::allReachableNodes((cons((name.clone()).clone(), protectedDepends.clone()), metamodelica::nil()), deps.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?) {
                        Deref @ metamodelica::List::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    allDepends = __pa0.clone();
                    allDepends = List::map1r(allDepends.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (prefix.clone()).clone());
                    allDepends = List::map1(allDepends.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!(".interface.mo")).clone());
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefix.clone()); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*suffix.clone()); __mm_s.push_str(&*literal!(": $(RELPATH_")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(") ")); __mm_s.push_str(&*stringDelimitList(allDepends.clone(), (literal!(" ")).clone())); ArcStr::from(__mm_s) }).clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elts, .. }, name, .. } => {
                    let mut tmp1: ArcStr = arcstr::literal!("");
                    let mut allDepends: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut protectedDepends: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut tmp2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    protectedDepends = List::map(List::select(elts.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::elementIsProtectedImport, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>)), (std::sync::Arc::new(importDependency) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
                    protectedDepends = List::select(protectedDepends.clone(), (std::sync::Arc::new(fnptr!(isNotBuiltinImport, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<bool> + 'static>));
                    allDepends = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (deps.clone()).into_iter().cloned() {
                    let __x = Util::tuple21(e.clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                    for mut d in &*protectedDepends.clone() {
                        let mut d = d.clone();
                        if !(listMember((d.clone()).clone(), allDepends.clone())) {
                            Error::addSourceMessage(Error::GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED_UNKNOWN_PACKAGE.clone(), list![(name.clone()).clone(), (name.clone()).clone(), (d.clone()).clone()], info.clone())?;
                            bail!("fail");
                        }
                    }
                    for mut dep in &*deps.clone() {
                        let mut dep = dep.clone();
                        (tmp1, tmp2) = dep.clone();
                        for mut d in &*tmp2.clone() {
                            let mut d = d.clone();
                            if !(listMember((d.clone()).clone(), allDepends.clone())) {
                                        Error::addSourceMessage(Error::GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED_UNKNOWN_PACKAGE.clone(), list![(name.clone()).clone(), (tmp1.clone()).clone(), (d.clone()).clone()], info.clone())?;
                                        bail!("fail");
                            }
                        }
                    }
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info, name, .. } => {
                    Error::addSourceMessage(Error::GENERATE_SEPARATE_CODE_DEPENDENCIES_FAILED.clone(), list![(name.clone()).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(r#str)
}

fn isNotBuiltinImport(mut module: ArcStr) -> bool {
    let mut b: bool = module.clone() != literal!("MetaModelica");
    b
}

fn getTypeNameIdent(mut val: Arc<Values::Value>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(val.clone()) {
        Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: __pa0 } } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    Ok(r#str)
}

fn getChangedClass(mut elt: Arc<SCode::Element>, mut suffix: ArcStr) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    name = ('mc: {
        let __mc_input = elt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info: SourceInfo { .. }, name, .. } => {
                    let false = (System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*suffix.clone()); ArcStr::from(__mm_s) }).clone())) else { bail!("pattern mismatch") };
                    Ok(name.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { info: SourceInfo { fileName, .. }, name, .. } => {
                    let true = (System::fileIsNewerThan((fileName.clone()).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*suffix.clone()); ArcStr::from(__mm_s) }).clone())?) else { bail!("pattern mismatch") };
                    Ok(name.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(name)
}

fn isChanged(mut node: (ArcStr, Arc<metamodelica::List<ArcStr>>), mut hs: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<ArcStr>>), i32, i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))) -> bool {
    let mut b: bool = false;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (r#str, strs) = node.clone();
    b = List::exist1(cons((r#str.clone()).clone(), strs.clone()), (std::sync::Arc::new(BaseHashSet::has) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static>), hs.clone());
    b
}

fn reloadClass(mut filename: ArcStr, mut encoding: ArcStr) -> Result<()> {
    let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    newp = Parser::parse((filename.clone()).clone(), (encoding.clone()).clone(), (literal!("")).clone(), None, Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
    newp = ProgramUtil::updateProgram(newp.clone(), SymbolTable::getAbsyn(), false)?;
    SymbolTable::setAbsyn(newp.clone())?;
    Ok(())
}

pub fn translateFunctions(mut program: Absyn::Program, mut name: ArcStr, mut optMainFunction: Option<DAE::Function>, mut idaeElements: Arc<metamodelica::List<DAE::Function>>, mut metarecordTypes: Arc<metamodelica::List<Arc<DAE::Type>>>, mut inIncludes: Arc<metamodelica::List<ArcStr>>) -> Result<()> {
    { let __v = None; openmodelica_backend::Globals::optionSimCode.with(|__root| *__root.borrow_mut() = __v) };
    let () = (::match_deref::match_deref! { match &((optMainFunction.clone(), idaeElements.clone(), inIncludes.clone())) {
        (Some(daeMainFunction), daeElements, includes) => {
            let mut mainFunction: Arc<SimCodeFunction::Function::Function>;
            let mut fns: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut makefileParams: SimCodeFunction::MakefileParams = <SimCodeFunction::MakefileParams as ::std::default::Default>::default();
            let mut fnCode: SimCodeFunction::FunctionCode;
            let mut extraRecordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
            let mut literals: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut midCode: Tpl::Text;
            let mut midfuncs: Arc<metamodelica::List<MidCode::Function>> = metamodelica::nil();
            let mut daeElements = (*daeElements).clone();
            let mut includes = (*includes).clone();
            (daeElements, literals) = SimCodeFunctionUtil::findLiterals(cons(daeMainFunction.clone(), daeElements.clone()));
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(SimCodeFunctionUtil::elaborateFunctions(program.clone(), daeElements.clone(), metarecordTypes.clone(), literals.clone(), includes.clone())?) {
                (Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 }, __pa2, __pa3, __pa4, __pa5, __pa6) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
                _ => bail!("pattern mismatch"),
            } };
            mainFunction = __pa0.clone();
            fns = __pa1.clone();
            extraRecordDecls = __pa2.clone();
            includes = __pa3.clone();
            includeDirs = __pa4.clone();
            libs = __pa5.clone();
            libPaths = __pa6.clone();
            SimCodeFunctionUtil::checkValidMainFunction((name.clone()).clone(), mainFunction.clone())?;
            makefileParams = SimCodeFunctionUtil::createMakefileParams(includeDirs.clone(), libs.clone(), libPaths.clone(), true, false)?;
            fnCode = SimCodeFunction::FunctionCode { name: (name.clone()).clone(), mainFunction: Some(mainFunction.clone()), functions: fns.clone(), literals: literals.clone(), externalFunctionIncludes: includes.clone(), makefileParams: makefileParams.clone(), extraRecordDecls: extraRecordDecls.clone() };
            if Config::simCodeTarget()? == literal!("MidC") {
                Tpl::tplString((std::sync::Arc::new(CodegenCFunctions::translateFunctionHeaderFiles) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static>), fnCode.clone())?;
                midfuncs = DAEToMid::DAEFunctionsToMid(cons(mainFunction.clone(), fns.clone()))?;
                midCode = Tpl::tplCallWithFailError((std::sync::Arc::new(CodegenMidToC::genProgram) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, MidCode::Program) -> Result<Tpl::Text> + 'static>), MidCode::Program { name: (name.clone()).clone(), functions: midfuncs.clone() }, Tpl::emptyTxt.clone())?;
                Tpl::textFileConvertLines(midCode.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".c")); ArcStr::from(__mm_s) }).clone())?;
            } else {
                Tpl::tplString((std::sync::Arc::new(CodegenCFunctions::translateFunctions) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static>), fnCode.clone())?;
            }
            ()
        },
        (None, daeElements, includes) => {
            let mut fns: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            let mut libs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut libPaths: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut includeDirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut makefileParams: SimCodeFunction::MakefileParams = <SimCodeFunction::MakefileParams as ::std::default::Default>::default();
            let mut fnCode: SimCodeFunction::FunctionCode;
            let mut extraRecordDecls: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
            let mut literals: Arc<metamodelica::List<Arc<DAE::Exp>>> = metamodelica::nil();
            let mut midCode: Tpl::Text;
            let mut midfuncs: Arc<metamodelica::List<MidCode::Function>> = metamodelica::nil();
            let mut daeElements = (*daeElements).clone();
            let mut includes = (*includes).clone();
            (daeElements, literals) = SimCodeFunctionUtil::findLiterals(daeElements.clone());
            (fns, extraRecordDecls, includes, includeDirs, libs, libPaths) = SimCodeFunctionUtil::elaborateFunctions(program.clone(), daeElements.clone(), metarecordTypes.clone(), literals.clone(), includes.clone())?;
            makefileParams = SimCodeFunctionUtil::createMakefileParams(includeDirs.clone(), libs.clone(), libPaths.clone(), true, false)?;
            fns = removeThreadDataFunction(fns.clone(), metamodelica::nil());
            extraRecordDecls = removeThreadDataRecord(extraRecordDecls.clone(), metamodelica::nil());
            fnCode = SimCodeFunction::FunctionCode { name: (name.clone()).clone(), mainFunction: None, functions: fns.clone(), literals: literals.clone(), externalFunctionIncludes: includes.clone(), makefileParams: makefileParams.clone(), extraRecordDecls: extraRecordDecls.clone() };
            if Config::simCodeTarget()? == literal!("MidC") {
                Tpl::tplString((std::sync::Arc::new(CodegenCFunctions::translateFunctionHeaderFiles) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static>), fnCode.clone())?;
                midfuncs = DAEToMid::DAEFunctionsToMid(fns.clone())?;
                midCode = Tpl::tplCallWithFailError((std::sync::Arc::new(CodegenMidToC::genProgram) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, MidCode::Program) -> Result<Tpl::Text> + 'static>), MidCode::Program { name: (name.clone()).clone(), functions: midfuncs.clone() }, Tpl::emptyTxt.clone())?;
                Tpl::textFileConvertLines(midCode.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".c")); ArcStr::from(__mm_s) }).clone())?;
            } else {
                Tpl::tplString((std::sync::Arc::new(CodegenCFunctions::translateFunctions) as std::sync::Arc<dyn ::std::ops::Fn(Tpl::Text, SimCodeFunction::FunctionCode) -> Result<Tpl::Text> + 'static>), fnCode.clone())?;
            }
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeThreadDataRecord(mut inRecs: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>, mut inAcc: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>>) -> Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> {
    let mut outRecs: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
    outRecs = (::match_deref::match_deref! { match &(inRecs.clone()) {
        Deref @ metamodelica::List::Nil => {
            inAcc.clone().reverse()
        },
        Deref @ metamodelica::List::Cons { head: SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: Deref @ "OpenModelica_threadData_ThreadData", .. }, tail: rest } => {
            let mut acc: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
            acc = removeThreadDataRecord(rest.clone(), inAcc.clone());
            acc.clone()
        },
        Deref @ metamodelica::List::Cons { head: SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "threadData", path: Deref @ Absyn::Path::IDENT { name: Deref @ "ThreadData" } } }, .. }, tail: rest } => {
            let mut acc: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
            acc = removeThreadDataRecord(rest.clone(), inAcc.clone());
            acc.clone()
        },
        Deref @ metamodelica::List::Cons { head: r, tail: rest } => {
            let mut acc: Arc<metamodelica::List<SimCodeFunction::RecordDeclaration>> = metamodelica::nil();
            acc = removeThreadDataRecord(rest.clone(), cons(r.clone(), inAcc.clone()));
            acc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outRecs
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeThreadDataFunction(mut inFuncs: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>, mut inAcc: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>>) -> Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> {
    let mut outFuncs: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
    outFuncs = (::match_deref::match_deref! { match &(inFuncs.clone()) {
        Deref @ metamodelica::List::Nil => {
            inAcc.clone().reverse()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ SimCodeFunction::Function::RECORD_CONSTRUCTOR { name: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "threadData", path: Deref @ Absyn::Path::IDENT { name: Deref @ "ThreadData" } } } }, .. }, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            acc = removeThreadDataFunction(rest.clone(), inAcc.clone());
            acc.clone()
        },
        Deref @ metamodelica::List::Cons { head: f, tail: rest } => {
            let mut acc: Arc<metamodelica::List<Arc<SimCodeFunction::Function::Function>>> = metamodelica::nil();
            acc = removeThreadDataFunction(rest.clone(), cons(f.clone(), inAcc.clone()));
            acc.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outFuncs
}

fn unZipEncryptedPackageAndCheckFile(mut inWorkdir: ArcStr, mut filename: ArcStr, mut skipUnzip: bool) -> Result<(bool, ArcStr)> {
    let mut success: bool = false;
    let mut outFilename: ArcStr = arcstr::literal!("");
    let mut workdir: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s3: ArcStr = arcstr::literal!("");
    let mut filename_1: ArcStr = arcstr::literal!("");
    let mut filename1: ArcStr = arcstr::literal!("");
    let mut filename2: ArcStr = arcstr::literal!("");
    let mut filename3: ArcStr = arcstr::literal!("");
    let mut filename4: ArcStr = arcstr::literal!("");
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut str1: ArcStr = arcstr::literal!("");
    let mut str2: ArcStr = arcstr::literal!("");
    let mut str3: ArcStr = arcstr::literal!("");
    let mut str4: ArcStr = arcstr::literal!("");
    let mut cmd: ArcStr = arcstr::literal!("");
    let mut cmdPrefix: ArcStr = arcstr::literal!("");
    let mut isWindows: bool = arcstr::literal!(Autoconf::os) == literal!("Windows_NT");
    success = false;
    outFilename = (literal!("")).clone();
    if System::regularFileExists((filename.clone()).clone()) {
        if StringUtil::endsWith((filename.clone()).clone(), (literal!(".mol")).clone()) {
            workdir = (if (System::directoryExists((inWorkdir.clone()).clone())) {inWorkdir.clone()} else {System::pwd()}).clone();
            cmdPrefix = (if (isWindows.clone()) {literal!("ripunzip.exe -q unzip-file -d ")} else {literal!("unzip -q -o -d ")}).clone();
            cmd = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cmdPrefix.clone()); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("\" \"")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone();
            if skipUnzip.clone() || 0 == System::systemCall((cmd.clone()).clone(), (literal!("")).clone()) {
                s1 = (System::basename((filename.clone()).clone())).clone();
                s2 = (Util::removeLast4Char((s1.clone()).clone())?).clone();
                s3 = ((Util::stringSplitAtChar((s2.clone()).clone(), (literal!(" ")).clone())?).get(1)?).clone();
                filename1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("/package.moc")); ArcStr::from(__mm_s) }).clone();
                filename2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(".moc")); ArcStr::from(__mm_s) }).clone();
                filename3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("/package.moc")); ArcStr::from(__mm_s) }).clone();
                filename4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!(".moc")); ArcStr::from(__mm_s) }).clone();
                if System::regularFileExists((filename1.clone()).clone()) {
                    filename_1 = (filename1.clone()).clone();
                } else if System::regularFileExists((filename2.clone()).clone()) {
                    filename_1 = (filename2.clone()).clone();
                } else if System::regularFileExists((filename3.clone()).clone()) {
                    filename_1 = (filename3.clone()).clone();
                } else {
                    filename_1 = (filename4.clone()).clone();
                }
                str1 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("/package.mo")); ArcStr::from(__mm_s) }).clone();
                str2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone();
                str3 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("/package.mo")); ArcStr::from(__mm_s) }).clone();
                str4 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*workdir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*s3.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone();
                if System::regularFileExists((str1.clone()).clone()) {
                    r#str = (str1.clone()).clone();
                } else if System::regularFileExists((str2.clone()).clone()) {
                    r#str = (str2.clone()).clone();
                } else if System::regularFileExists((str3.clone()).clone()) {
                    r#str = (str3.clone()).clone();
                } else {
                    r#str = (str4.clone()).clone();
                }
                filename_1 = (if (System::regularFileExists((filename_1.clone()).clone())) {filename_1.clone()} else {r#str.clone()}).clone();
                if System::regularFileExists((filename_1.clone()).clone()) {
                    success = true;
                    outFilename = (filename_1.clone()).clone();
                } else {
                    Error::addMessage(Error::PACKAGE_FILE_NOT_FOUND_ERROR.clone(), list![(filename1.clone()).clone(), (filename2.clone()).clone(), (filename3.clone()).clone(), (filename4.clone()).clone(), (str1.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone(), (str4.clone()).clone()])?;
                }
            } else {
                Error::addMessage(Error::UNABLE_TO_UNZIP_FILE.clone(), list![(filename.clone()).clone()])?;
            }
        } else {
            Error::addMessage(Error::EXPECTED_ENCRYPTED_PACKAGE.clone(), list![(filename.clone()).clone()])?;
        }
    } else {
        Error::addMessage(Error::FILE_NOT_FOUND_ERROR.clone(), list![(filename.clone()).clone()])?;
    }
    Ok((success, outFilename))
}

fn listClass(mut args: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut scodeP: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut cl: Arc<SCode::Element>;
    let mut path: Arc<Absyn::Path>;
    let mut className: Arc<Absyn::Path>;
    let mut interface_only: bool = false;
    let mut short_only: bool = false;
    let dumpOpt: SCodeDump::SCodeDumpOptions = SCodeDump::SCodeDumpOptions { stripAlgorithmSections: true, stripProtectedImports: false, stripProtectedClasses: true, stripProtectedComponents: true, stripMetaRecords: true, stripGraphicalAnnotations: true, stripStringComments: true, stripExternalDecl: true, stripOutputBindings: true };
    r#str = ('mc: {
        let __mc_input = args.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut p: Absyn::Program = p.clone();
                    p = SymbolTable::getAbsyn();
                    let true = (Interactive::astContainsEncryptedClass(p.clone())?) else { bail!("pattern mismatch") };
                    Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AllLoadedClasses" } } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: false }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { name: path, .. }, tail: Deref @ metamodelica::List::Nil } } } } => {
                    Ok((::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(path.clone())?) {
        Deref @ "Absyn" => Dump::unparseStr(SymbolTable::getAbsyn(), false, Dump::defaultDumpOptions.clone())?,
        Deref @ "SCode" => SCodeDump::programStr(SymbolTable::getSCode()?, SCodeDump::defaultOptions.clone())?,
        Deref @ "MetaModelicaInterface" => SCodeDump::programStr(SymbolTable::getSCode()?, dumpOpt.clone())?,
        Deref @ "Internal" => System::anyStringCode(SymbolTable::getAbsyn()),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: interface_only }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: short_only }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::ENUM_LITERAL { name: path, .. }, tail: Deref @ metamodelica::List::Nil } } } } => {
                    let mut scodeP: Arc<metamodelica::List<Arc<SCode::Element>>> = scodeP.clone();
                    let mut p: Absyn::Program = p.clone();
                    let mut cl: Arc<SCode::Element>;
                    let mut absynClass: Arc<Absyn::Class> = absynClass.clone();
                    let false = (Arc::new(Absyn::Path::IDENT { name: (literal!("AllLoadedClasses")).clone() }) == className.clone()) else { bail!("pattern mismatch") };
                    p = SymbolTable::getAbsyn();
                    scodeP = SymbolTable::getSCode()?;
                    absynClass = ProgramUtil::getPathedClassInProgram(className.clone(), p.clone(), false, false)?;
                    absynClass = if (interface_only.clone()) {AbsynUtil::getFunctionInterface(absynClass.clone())?} else {absynClass.clone()};
                    absynClass = if (short_only.clone()) {AbsynUtil::getShortClass(absynClass.clone())?} else {absynClass.clone()};
                    p = Absyn::Program { classes: list![absynClass.clone()], within_: openmodelica_ast::Absyn::Within::TOP };
                    cl = FBuiltin::getElementWithPathCheckBuiltin(scodeP.clone(), className.clone())?;
                    Ok((::match_deref::match_deref! { match &(AbsynUtil::pathLastIdent(path.clone())?) {
        Deref @ "Absyn" => Dump::unparseStr(p.clone(), false, Dump::defaultDumpOptions.clone())?,
        Deref @ "SCode" => SCodeDump::unparseElementStr(cl.clone(), SCodeDump::defaultOptions.clone())?,
        Deref @ "MetaModelicaInterface" => SCodeDump::unparseElementStr(cl.clone(), dumpOpt.clone())?,
        Deref @ "Internal" => System::anyStringCode(p.clone()),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    res = Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() });
    Ok(res)
}

fn listFile(mut args: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut path: Arc<Absyn::Path>;
    let mut className: Arc<Absyn::Path>;
    let mut nested: bool = false;
    let mut access: Access = Access::hide;
    let mut absynClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut restriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    r#str = ('mc: {
        let __mc_input = args.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: className } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: nested }, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut access: Access = access.clone();
                    let mut r#str: ArcStr = r#str.clone();
                    let mut absynClass: Arc<Absyn::Class> = absynClass.clone();
                    let mut path: Arc<Absyn::Path>;
                    let mut restriction: Absyn::Restriction = restriction.clone();
                    path = (::match_deref::match_deref! { match &(className.clone()) {
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => var_field!((**className).path, Absyn::Path::FULLYQUALIFIED).clone(),
        _ => className.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    access = Interactive::checkAccessAnnotationAndEncryption(path.clone(), SymbolTable::getAbsyn());
                    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(ProgramUtil::getPathedClassInProgram(className.clone(), SymbolTable::getAbsyn(), false, false)?) {
                        __pa2 @ Deref @ Absyn::Class { info: SourceInfo { fileName: __pa0, .. }, restriction: __pa1, .. } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    restriction = __pa1.clone();
                    absynClass = __pa2.clone();
                    absynClass = if (nested.clone()) {absynClass.clone()} else {AbsynUtil::filterNestedClasses(absynClass.clone())};
                    if access.clone() >= Access::packageText.clone() || access.clone() >= Access::nonPackageText.clone() && !(AbsynUtil::isPackageRestriction(restriction.clone())) {
                        r#str = (Dump::unparseStr(Absyn::Program { classes: list![absynClass.clone()], within_: (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => openmodelica_ast::Absyn::Within::TOP,
        _ => Absyn::Within::WITHIN { path: AbsynUtil::stripLast(path.clone())? },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }) }, false, Dump::DumpOptions { fileName: (r#str.clone()).clone() })?).clone();
                    } else {
                        Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil())?;
                        r#str = (literal!("")).clone();
                    }
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    res = Arc::new(Values::Value::STRING { string: (r#str.clone()).clone() });
    Ok(res)
}

fn getClassNames(mut args: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut path: Arc<Absyn::Path>;
    let mut recursive: bool = false;
    let mut qualified: bool = false;
    let mut sort: bool = false;
    let mut builtin: bool = false;
    let mut protects: bool = false;
    let mut constants: bool = false;
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(args.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa3 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa4 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa5 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa6 }, tail: Deref @ metamodelica::List::Nil } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => bail!("pattern mismatch"),
    } };
    path = __pa0.clone();
    recursive = __pa1.clone();
    qualified = __pa2.clone();
    sort = __pa3.clone();
    builtin = __pa4.clone();
    protects = __pa5.clone();
    constants = __pa6.clone();
    p = SymbolTable::getAbsyn();
    if builtin.clone() {
        p = ProgramUtil::updateProgram(p.clone(), (FBuiltin::getInitialFunctions()?).0, false)?;
    }
    if AbsynUtil::pathEqual(path.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("AllLoadedClasses")).clone() })) {
        if recursive.clone() {
            (_, paths) = ProgramUtil::getClassNamesRecursive(None, p.clone(), protects.clone(), constants.clone(), metamodelica::nil())?;
            paths = paths.clone().reverse();
        } else {
            paths = Interactive::getTopClassnames(p.clone())?;
        }
    } else {
        if recursive.clone() {
            (_, paths) = ProgramUtil::getClassNamesRecursive(Some(path.clone()), p.clone(), protects.clone(), constants.clone(), metamodelica::nil())?;
            paths = paths.clone().reverse();
        } else {
            paths = Interactive::getClassnamesInPath(path.clone(), p.clone(), protects.clone(), constants.clone())?;
            if qualified.clone() {
                paths = ({
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
            let __x = AbsynUtil::joinPaths(path.clone(), p.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            }
        }
    }
    if sort.clone() {
        paths = List::sort(paths.clone(), (std::sync::Arc::new(AbsynUtil::pathGe) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Arc<Absyn::Path>) -> Result<bool> + 'static>))?;
    }
    res = ValuesMake::makeCodeTypeNameArray(paths.clone())?;
    Ok(res)
}

fn checkSettings() -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut vars: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut omhome: ArcStr = arcstr::literal!("");
    let mut omlib: ArcStr = arcstr::literal!("");
    let mut omcpath: ArcStr = arcstr::literal!("");
    let mut systemPath: ArcStr = arcstr::literal!("");
    let mut omdev: ArcStr = arcstr::literal!("");
    let mut os: ArcStr = arcstr::literal!("");
    let mut touch_file: ArcStr = arcstr::literal!("");
    let mut usercflags: ArcStr = arcstr::literal!("");
    let mut workdir: ArcStr = arcstr::literal!("");
    let mut uname: ArcStr = arcstr::literal!("");
    let mut senddata: ArcStr = arcstr::literal!("");
    let mut gcc: ArcStr = arcstr::literal!("");
    let mut gccVersion: ArcStr = arcstr::literal!("");
    let mut confcmd: ArcStr = arcstr::literal!("");
    let mut omcfound: bool = false;
    let mut touch_res: bool = false;
    let mut rm_res: bool = false;
    let mut have_corba: bool = false;
    let mut gcc_res: bool = false;
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    vars = list![(literal!("OPENMODELICAHOME")).clone(), (literal!("OPENMODELICALIBRARY")).clone(), (literal!("OMC_PATH")).clone(), (literal!("SYSTEM_PATH")).clone(), (literal!("OMDEV_PATH")).clone(), (literal!("OMC_FOUND")).clone(), (literal!("MODELICAUSERCFLAGS")).clone(), (literal!("WORKING_DIRECTORY")).clone(), (literal!("CREATE_FILE_WORKS")).clone(), (literal!("REMOVE_FILE_WORKS")).clone(), (literal!("OS")).clone(), (literal!("SYSTEM_INFO")).clone(), (literal!("RTLIBS")).clone(), (literal!("C_COMPILER")).clone(), (literal!("C_COMPILER_VERSION")).clone(), (literal!("C_COMPILER_RESPONDING")).clone(), (literal!("HAVE_CORBA")).clone(), (literal!("CONFIGURE_CMDLINE")).clone()];
    omhome = (Settings::getInstallationDirectoryPath()?).clone();
    omlib = (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone();
    omcpath = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*omhome.clone()); __mm_s.push_str(&*literal!("/bin/omc")); __mm_s.push_str(&*arcstr::literal!(Autoconf::exeExt)); ArcStr::from(__mm_s) }).clone();
    systemPath = (Util::makeValueOrDefault((std::sync::Arc::new(System::readEnv) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>), (literal!("PATH")).clone(), (literal!("")).clone())).clone();
    omdev = (Util::makeValueOrDefault((std::sync::Arc::new(System::readEnv) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>), (literal!("OMDEV")).clone(), (literal!("")).clone())).clone();
    omcfound = System::regularFileExists((omcpath.clone()).clone());
    os = (arcstr::literal!(Autoconf::os)).clone();
    touch_file = (literal!("omc.checksettings.create_file_test")).clone();
    usercflags = (Util::makeValueOrDefault((std::sync::Arc::new(System::readEnv) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>), (literal!("MODELICAUSERCFLAGS")).clone(), (literal!("")).clone())).clone();
    workdir = (System::pwd()).clone();
    touch_res = 0 == System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("touch ")); __mm_s.push_str(&*touch_file.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
    System::systemCall((literal!("uname -a")).clone(), (touch_file.clone()).clone());
    uname = (System::readFile((touch_file.clone()).clone())?).clone();
    rm_res = 0 == System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rm ")); __mm_s.push_str(&*touch_file.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
    senddata = (arcstr::literal!(Autoconf::ldflags_runtime)).clone();
    gcc = (System::getCCompiler()).clone();
    have_corba = Corba::haveCorba();
    System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rm -f ")); __mm_s.push_str(&*touch_file.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
    gcc_res = 0 == System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*gcc.clone()); __mm_s.push_str(&*literal!(" --version")); ArcStr::from(__mm_s) }).clone(), (touch_file.clone()).clone());
    gccVersion = (System::readFile((touch_file.clone()).clone())?).clone();
    System::systemCall(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("rm -f ")); __mm_s.push_str(&*touch_file.clone()); ArcStr::from(__mm_s) }).clone(), (literal!("")).clone());
    confcmd = (arcstr::literal!(Autoconf::configureCommandLine)).clone();
    vals = list![Arc::new(Values::Value::STRING { string: (omhome.clone()).clone() }), Arc::new(Values::Value::STRING { string: (omlib.clone()).clone() }), Arc::new(Values::Value::STRING { string: (omcpath.clone()).clone() }), Arc::new(Values::Value::STRING { string: (systemPath.clone()).clone() }), Arc::new(Values::Value::STRING { string: (omdev.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: omcfound.clone() }), Arc::new(Values::Value::STRING { string: (usercflags.clone()).clone() }), Arc::new(Values::Value::STRING { string: (workdir.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: touch_res.clone() }), Arc::new(Values::Value::BOOL { boolean: rm_res.clone() }), Arc::new(Values::Value::STRING { string: (os.clone()).clone() }), Arc::new(Values::Value::STRING { string: (uname.clone()).clone() }), Arc::new(Values::Value::STRING { string: (senddata.clone()).clone() }), Arc::new(Values::Value::STRING { string: (gcc.clone()).clone() }), Arc::new(Values::Value::STRING { string: (gccVersion.clone()).clone() }), Arc::new(Values::Value::BOOL { boolean: gcc_res.clone() }), Arc::new(Values::Value::BOOL { boolean: have_corba.clone() }), Arc::new(Values::Value::STRING { string: (confcmd.clone()).clone() })];
    res = Arc::new(Values::Value::RECORD { record_: Arc::new(Absyn::Path::IDENT { name: (literal!("OpenModelica.Scripting.CheckSettingsResult")).clone() }), orderd: vals.clone(), comp: vars.clone(), index: -1 });
    Ok(res)
}

fn generateSeparateCodeDependenciesMakefile(mut args: Arc<metamodelica::List<Arc<Values::Value>>>) -> Arc<Values::Value> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut strs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut deps: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
    let mut filename: ArcStr = arcstr::literal!("");
    let mut prefix: ArcStr = arcstr::literal!("");
    let mut suffix: ArcStr = arcstr::literal!("");
    match '__try0: {
        let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa2 }, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa3 }, tail: Deref @ metamodelica::List::Nil } } } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        filename = __pa1.clone();
        prefix = __pa2.clone();
        suffix = __pa3.clone();
        sp = unwrap_break_err!(SymbolTable::getSCode(), '__try0);
        names = List::filterMap(sp.clone(), (std::sync::Arc::new(SCodeUtil::getElementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
        deps = Graph::buildGraph(names.clone(), (std::sync::Arc::new(buildDependencyGraphPublicImports) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), sp.clone());
        strs = List::map3(sp.clone(), (std::sync::Arc::new(writeModuleDepends) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, ArcStr, ArcStr, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>>) -> Result<ArcStr> + 'static>), (prefix.clone()).clone(), (suffix.clone()).clone(), deps.clone());
        unwrap_break_err!(System::writeFile((filename.clone()).clone(), stringDelimitList(strs.clone(), (literal!("\n")).clone())), '__try0);
        res = Arc::new(Values::Value::BOOL { boolean: true });
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = Arc::new(Values::Value::BOOL { boolean: false });
        }
    }
    res
}

fn generateSeparateCodeDependencies(mut args: Arc<metamodelica::List<Arc<Values::Value>>>) -> Arc<Values::Value> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut suffix: ArcStr = arcstr::literal!("");
    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut namesPublic: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut namesChanged: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut fileNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut deps: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
    let mut depstransitive: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
    let mut depstransposed: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
    let mut depstransposedtransitive: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
    let mut depsmerged: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
    let mut depschanged: Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>> = metamodelica::nil();
    let mut hashSetString: (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<ArcStr>>), i32, i32, (HashSetString::FuncHashCref, HashSetString::FuncCrefEqual, HashSetString::FuncCrefStr));
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(args.clone()) {
            Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::List::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        suffix = __pa1.clone();
        sp = unwrap_break_err!(SymbolTable::getSCode(), '__try0);
        names = List::filterMap(sp.clone(), (std::sync::Arc::new(SCodeUtil::getElementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
        deps = Graph::buildGraph(names.clone(), (std::sync::Arc::new(buildDependencyGraph) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), sp.clone());
        namesPublic = List::map(List::select(sp.clone(), (std::sync::Arc::new(containsPublicInterface) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<bool> + 'static>)), (std::sync::Arc::new(SCodeUtil::getElementName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>) -> Result<ArcStr> + 'static>));
        namesChanged = List::filterMap1(sp.clone(), (std::sync::Arc::new(getChangedClass) as std::sync::Arc<dyn ::std::ops::Fn(Arc<SCode::Element>, ArcStr) -> Result<ArcStr> + 'static>), (suffix.clone()).clone());
        hashSetString = HashSetString::emptyHashSet();
        hashSetString = List::fold(namesChanged.clone(), (std::sync::Arc::new(BaseHashSet::add) as std::sync::Arc<dyn ::std::ops::Fn(_, _) -> Result<_> + 'static>), hashSetString.clone());
        depstransposed = unwrap_break_err!(Graph::transposeGraph(Graph::emptyGraph(names.clone()), deps.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)), '__try0);
        depstransposedtransitive = Graph::buildGraph(namesPublic.clone(), (std::sync::Arc::new(buildTransitiveDependencyGraph) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<metamodelica::List<(ArcStr, Arc<metamodelica::List<ArcStr>>)>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), depstransposed.clone());
        depstransitive = unwrap_break_err!(Graph::transposeGraph(Graph::emptyGraph(names.clone()), depstransposedtransitive.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)), '__try0);
        depstransitive = unwrap_break_err!(List::sort(depstransitive.clone(), (std::sync::Arc::new(fnptr!(compareNumberOfDependencies, (ArcStr, Arc<metamodelica::List<ArcStr>>), (ArcStr, Arc<metamodelica::List<ArcStr>>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<metamodelica::List<ArcStr>>), (ArcStr, Arc<metamodelica::List<ArcStr>>)) -> Result<bool> + 'static>)), '__try0);
        depsmerged = unwrap_break_err!(Graph::merge(deps.clone(), depstransitive.clone(), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), (std::sync::Arc::new(fnptr!(compareDependencyNode, (ArcStr, Arc<metamodelica::List<ArcStr>>), (ArcStr, Arc<metamodelica::List<ArcStr>>))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<metamodelica::List<ArcStr>>), (ArcStr, Arc<metamodelica::List<ArcStr>>)) -> Result<bool> + 'static>)), '__try0);
        depschanged = List::select1(depsmerged.clone(), (std::sync::Arc::new(fnptr!(isChanged, (ArcStr, Arc<metamodelica::List<ArcStr>>), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<ArcStr>>), i32, i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>)))) as std::sync::Arc<dyn ::std::ops::Fn((ArcStr, Arc<metamodelica::List<ArcStr>>), (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<ArcStr>>), i32, i32, (Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>))) -> Result<bool> + 'static>), hashSetString.clone());
        names = List::map(depschanged.clone(), std::sync::Arc::new(fnptr!(Util::tuple21, _)));
        fileNames = List::map1(names.clone(), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (suffix.clone()).clone());
        List::map(fileNames.clone(), (std::sync::Arc::new(fnptr!(System::removeFile, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>));
        res = unwrap_break_err!(ValuesMake::makeArray(List::map(names.clone(), (std::sync::Arc::new(fnptr!(ValuesMake::makeString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Values::Value>> + 'static>))), '__try0);
        Ok::<_, anyhow::Error>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = Arc::new(openmodelica_frontend_types::Values::Value::META_FAIL);
        }
    }
    res
}

fn generateSeparateCode(mut args: Arc<metamodelica::List<Arc<Values::Value>>>, mut cache: FCore::Cache, mut env: FCore::Graph) -> Result<(Arc<Values::Value>, FCore::Cache)> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut v: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut b: bool = false;
    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut cl: Arc<SCode::Element>;
    res = 'mc: {
        let __mc_input = args.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut p: Absyn::Program = p.clone();
                    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = sp.clone();
                    let mut name: ArcStr = name.clone();
                    let mut cl: Arc<SCode::Element>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    p = SymbolTable::getAbsyn();
                    sp = SymbolTable::getSCode()?;
                    name = (getTypeNameIdent(v.clone())?).clone();
                    { let __v = Some(true); openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = __v) };
                    cl = List::getMemberOnTrue((name.clone()).clone(), sp.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isClassNamed, ArcStr, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SCode::Element>) -> Result<bool> + 'static>))?;
                    (outCache, _) = generateFunctions(cache.clone(), env.clone(), p.clone(), sp.clone(), list![cl.clone()], b.clone())?;
                    { let __v = None; openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(Arc::new(Values::Value::BOOL { boolean: true }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: v, tail: Deref @ metamodelica::List::Cons { head: Deref @ Values::Value::BOOL { boolean: _ }, tail: Deref @ metamodelica::List::Nil } } => {
                    let mut sp: Arc<metamodelica::List<Arc<SCode::Element>>> = sp.clone();
                    let mut name: ArcStr = name.clone();
                    sp = SymbolTable::getSCode()?;
                    name = (getTypeNameIdent(v.clone())?).clone();
                    let false = (List::isMemberOnTrue((name.clone()).clone(), sp.clone(), (std::sync::Arc::new(fnptr!(SCodeUtil::isClassNamed, ArcStr, Arc<SCode::Element>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<SCode::Element>) -> Result<bool> + 'static>))) else { bail!("pattern mismatch") };
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(name.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    { let __v = None; openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| *__root.borrow_mut() = __v) };
                    Ok(Arc::new(Values::Value::BOOL { boolean: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((res, outCache))
}

pub fn getImportedNames(mut inClass: Arc<Absyn::Class>) -> Result<(Arc<metamodelica::List<Arc<Values::Value>>>, Arc<metamodelica::List<Arc<Values::Value>>>)> {
    let mut outPublicImports: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut outProtectedImports: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut ident: ArcStr = arcstr::literal!("");
    let mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    (pub_imports_list, pro_imports_list) = getImportList(inClass.clone(), metamodelica::nil(), metamodelica::nil());
    outPublicImports = metamodelica::nil();
    for mut imp in &*pub_imports_list.clone() {
        let mut imp = imp.clone();
        ident = (AbsynUtil::pathFirstIdent(AbsynUtil::importPath(imp.clone())?)?).clone();
        if ident.clone() != literal!("MetaModelica") {
            outPublicImports = cons(Arc::new(Values::Value::STRING { string: (ident.clone()).clone() }), outPublicImports.clone());
        }
    }
    outProtectedImports = metamodelica::nil();
    for mut imp in &*pro_imports_list.clone() {
        let mut imp = imp.clone();
        ident = (AbsynUtil::pathFirstIdent(AbsynUtil::importPath(imp.clone())?)?).clone();
        if ident.clone() != literal!("MetaModelica") {
            outProtectedImports = cons(Arc::new(Values::Value::STRING { string: (ident.clone()).clone() }), outProtectedImports.clone());
        }
    }
    Ok((outPublicImports, outProtectedImports))
}

pub fn getImportList(mut inClass: Arc<Absyn::Class>, mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>>, mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>>) -> (Arc<metamodelica::List<Absyn::Import>>, Arc<metamodelica::List<Absyn::Import>>) {
    let mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>> = pub_imports_list;
    let mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>> = pro_imports_list;
    for mut part in &*AbsynUtil::getClassPartsInClass(inClass.clone()) {
        let mut part = part.clone();
        (pub_imports_list, pro_imports_list) = getImportsInClassPart(part.clone(), pub_imports_list.clone(), pro_imports_list.clone());
    }
    pub_imports_list = pub_imports_list.clone().reverse();
    pro_imports_list = pro_imports_list.clone().reverse();
    (pub_imports_list, pro_imports_list)
}

fn getImportsInClassPart(mut part: Arc<Absyn::ClassPart>, mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>>, mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>>) -> (Arc<metamodelica::List<Absyn::Import>>, Arc<metamodelica::List<Absyn::Import>>) {
    let mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>> = pub_imports_list;
    let mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>> = pro_imports_list;
    let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            for mut elem in &*var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone() {
                let mut elem = elem.clone();
                pub_imports_list = getImportsInElementItem(elem.clone(), pub_imports_list.clone());
            }
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            for mut elem in &*var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone() {
                let mut elem = elem.clone();
                pro_imports_list = getImportsInElementItem(elem.clone(), pro_imports_list.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (pub_imports_list, pro_imports_list)
}

fn getImportsInElementItem(mut item: Arc<Absyn::ElementItem>, mut imports_list: Arc<metamodelica::List<Absyn::Import>>) -> Arc<metamodelica::List<Absyn::Import>> {
    let mut imports_list: Arc<metamodelica::List<Absyn::Import>> = imports_list;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::IMPORT { import_, .. }, .. } } => {
            imports_list = cons(import_.clone(), imports_list.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    imports_list
}

fn getMMfileTotalDependencies(mut in_package_name: ArcStr, mut public_imports_dir: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut total_pub_imports: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut package_class: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut pub_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut pro_imports_list: Arc<metamodelica::List<Absyn::Import>> = metamodelica::nil();
    let mut imp_ident: ArcStr = arcstr::literal!("");
    package_class = ProgramUtil::getPathedClassInProgram(Arc::new(Absyn::Path::IDENT { name: (in_package_name.clone()).clone() }), SymbolTable::getAbsyn(), false, false)?;
    (pub_imports_list, pro_imports_list) = getImportList(package_class.clone(), metamodelica::nil(), metamodelica::nil());
    for mut imp in &*pub_imports_list.clone() {
        let mut imp = imp.clone();
        imp_ident = (AbsynUtil::pathFirstIdent(AbsynUtil::importPath(imp.clone())?)?).clone();
        if imp_ident.clone() != literal!("MetaModelica") {
            total_pub_imports = getMMfilePublicDependencies((imp_ident.clone()).clone(), (public_imports_dir.clone()).clone(), total_pub_imports.clone())?;
        }
    }
    for mut imp in &*pro_imports_list.clone() {
        let mut imp = imp.clone();
        imp_ident = (AbsynUtil::pathFirstIdent(AbsynUtil::importPath(imp.clone())?)?).clone();
        if imp_ident.clone() != literal!("MetaModelica") {
            total_pub_imports = getMMfilePublicDependencies((imp_ident.clone()).clone(), (public_imports_dir.clone()).clone(), total_pub_imports.clone())?;
        }
    }
    Ok(total_pub_imports)
}

fn getMMfilePublicDependencies(mut in_package_name: ArcStr, mut public_imports_dir: ArcStr, mut packages: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut packages: Arc<metamodelica::List<ArcStr>> = packages;
    let mut dep_public_imports_file: ArcStr = arcstr::literal!("");
    let mut pub_imports_total: ArcStr = arcstr::literal!("");
    if listMember((in_package_name.clone()).clone(), packages.clone()) {
        return Ok(packages.clone());
    }
    packages = cons((in_package_name.clone()).clone(), packages.clone());
    dep_public_imports_file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*public_imports_dir.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*in_package_name.clone()); __mm_s.push_str(&*literal!(".public.imports")); ArcStr::from(__mm_s) }).clone();
    if !(System::regularFileExists((dep_public_imports_file.clone()).clone())) {
        Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("getMMfileTotalDependencies: missing dependency file ")); __mm_s.push_str(&*dep_public_imports_file.clone()); __mm_s.push_str(&*literal!(" — the module ")); __mm_s.push_str(&*in_package_name.clone()); __mm_s.push_str(&*literal!(" is imported (transitively) but is not part of the build. ")); __mm_s.push_str(&*literal!("Add its source file to the build configuration.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
        bail!("fail");
    }
    pub_imports_total = (System::readFile((dep_public_imports_file.clone()).clone())?).clone();
    for mut pub_imp in &*System::strtok((pub_imports_total.clone()).clone(), (literal!(";")).clone()) {
        let mut pub_imp = pub_imp.clone();
        packages = getMMfilePublicDependencies((pub_imp.clone()).clone(), (public_imports_dir.clone()).clone(), packages.clone())?;
    }
    Ok(packages)
}

