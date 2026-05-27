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

use crate::HashTableStringToProgram;
use crate::PackageManagement;
use crate::Parser;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::Autoconf;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type HashTable = (metamodelica::Array<Arc<metamodelica::List<(ArcStr, i32)>>>, (i32, i32, metamodelica::Array<Option<(ArcStr, Absyn::Program)>>), i32, (HashTableStringToProgram::FuncHashCref, HashTableStringToProgram::FuncCrefEqual, HashTableStringToProgram::FuncCrefStr, HashTableStringToProgram::FuncExpStr));

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageOrder {
    CLASSPART {
        cp: Arc<Absyn::ClassPart>,
    },
    ELEMENT {
        element: Arc<Absyn::ElementItem>,
        /// public
        r#pub: bool,
    },
    CLASSLOAD {
        cl: ArcStr,
    },
}
pub use self::PackageOrder::{CLASSPART,ELEMENT,CLASSLOAD};

#[derive(Clone)]
pub enum LoadFileStrategy {
    STRATEGY_HASHTABLE {
        ht: HashTable,
    },
    STRATEGY_ON_DEMAND {
        encoding: ArcStr,
    },
}
impl PartialEq for LoadFileStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::STRATEGY_HASHTABLE { ht: __l_ht }, Self::STRATEGY_HASHTABLE { ht: __r_ht }) => std::sync::Arc::ptr_eq(__l_ht, __r_ht),
            (Self::STRATEGY_ON_DEMAND { encoding: __l_encoding }, Self::STRATEGY_ON_DEMAND { encoding: __r_encoding }) => __l_encoding == __r_encoding,
            _ => false,
        }
    }
}
impl Eq for LoadFileStrategy {}
impl PartialOrd for LoadFileStrategy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for LoadFileStrategy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn __variant_idx(__v: &LoadFileStrategy) -> u32 {
            match __v {
                LoadFileStrategy::STRATEGY_HASHTABLE { .. } => 0,
                LoadFileStrategy::STRATEGY_ON_DEMAND { .. } => 1,
            }
        }
        match __variant_idx(self).cmp(&__variant_idx(other)) {
            std::cmp::Ordering::Equal => {}
            non_eq => return non_eq,
        }
        match (self, other) {
            (Self::STRATEGY_HASHTABLE { ht: __l_ht }, Self::STRATEGY_HASHTABLE { ht: __r_ht }) => (std::sync::Arc::as_ptr(__l_ht) as *const ()).cmp(&(std::sync::Arc::as_ptr(__r_ht) as *const ())),
            (Self::STRATEGY_ON_DEMAND { encoding: __l_encoding }, Self::STRATEGY_ON_DEMAND { encoding: __r_encoding }) => __l_encoding.cmp(__r_encoding),
            _ => unreachable!("variant-index equality already implies same variant"),
        }
    }
}
impl std::fmt::Debug for LoadFileStrategy {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STRATEGY_HASHTABLE { ht: __d_ht } => {
                let mut __ds = __f.debug_struct("STRATEGY_HASHTABLE");
                __ds.field("ht", &format_args!("<fn@{:p}>", std::sync::Arc::as_ptr(__d_ht)));
                __ds.finish()
            }
            Self::STRATEGY_ON_DEMAND { encoding: __d_encoding } => {
                let mut __ds = __f.debug_struct("STRATEGY_ON_DEMAND");
                __ds.field("encoding", __d_encoding);
                __ds.finish()
            }
        }
    }
}

pub use self::LoadFileStrategy::{STRATEGY_HASHTABLE,STRATEGY_ON_DEMAND};

pub fn loadClass(mut inPath: Arc<Absyn::Path>, mut priorityList: Arc<metamodelica::List<ArcStr>>, mut modelicaPath: ArcStr, mut encoding: Option<ArcStr>, mut requireExactVersion: bool, mut encrypted: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    outProgram = 'mc: {
        let __mc_input = (inPath.clone(), priorityList.clone(), modelicaPath.clone(), encoding.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: classname }, _, mp, _) => {
                    let mut gd: ArcStr = arcstr::literal!("");
                    let mut mps: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    gd = (arcstr::literal!(Autoconf::groupDelimiter)).clone();
                    mps = System::strtok((mp.clone()).clone(), (gd.clone()).clone());
                    p = loadClassFromMps((classname.clone()).clone(), priorityList.clone(), mps.clone(), encoding.clone(), requireExactVersion.clone(), encrypted.clone())?;
                    checkOnLoadMessage(p.clone())?;
                    Ok(p.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { name: pack, .. }, _, mp, _) => {
                    let mut gd: ArcStr = arcstr::literal!("");
                    let mut mps: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut p: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
                    gd = (arcstr::literal!(Autoconf::groupDelimiter)).clone();
                    mps = System::strtok((mp.clone()).clone(), (gd.clone()).clone());
                    p = loadClassFromMps((pack.clone()).clone(), priorityList.clone(), mps.clone(), encoding.clone(), requireExactVersion.clone(), encrypted.clone())?;
                    checkOnLoadMessage(p.clone())?;
                    Ok(p.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { bail!("pattern mismatch") };
                    Debug::trace((literal!("ClassLoader.loadClass failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProgram)
}

fn loadClassFromMps(mut id: ArcStr, mut prios: Arc<metamodelica::List<ArcStr>>, mut mps: Arc<metamodelica::List<ArcStr>>, mut encoding: Option<ArcStr>, mut requireExactVersion: bool, mut encrypted: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut mp: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut pwd: ArcStr = arcstr::literal!("");
    let mut cmd: ArcStr = arcstr::literal!("");
    let mut version: ArcStr = arcstr::literal!("");
    let mut userLibraries: ArcStr = arcstr::literal!("");
    let mut isDir: bool = false;
    let mut impactOK: bool = false;
    let mut cl: Option<Arc<Absyn::Class>> = None;
    let mut versionsThatProvideTheWanted: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut commands: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut versions: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if !(requireExactVersion.clone()) {
        if prios.clone().is_empty() {
            versions = PackageManagement::versionsThatProvideTheWanted((id.clone()).clone(), (literal!("default")).clone(), false);
        } else {
            versions = metamodelica::nil();
            for mut v in &*prios.clone().reverse() {
                let mut v = v.clone();
                versionsThatProvideTheWanted = PackageManagement::versionsThatProvideTheWanted((id.clone()).clone(), (v.clone()).clone(), false);
                if versionsThatProvideTheWanted.clone().is_empty() {
                    versions = cons((v.clone()).clone(), versions.clone());
                } else {
                    versions = listAppend(versionsThatProvideTheWanted.clone(), versions.clone());
                }
            }
        }
    } else {
        versions = prios.clone();
    }
    if let Ok((__pa0, __pa1, __pa2)) = System::getLoadModelPath((id.clone()).clone(), versions.clone(), mps.clone(), requireExactVersion.clone()) {
        mp = __pa0.clone();
        name = __pa1.clone();
        isDir = __pa2.clone();
    } else {
        version = ((::match_deref::match_deref! { match &(prios.clone()) {
        Deref @ metamodelica::List::Cons { head: version, tail: _ } => version.clone(),
        _ => literal!("default"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        versionsThatProvideTheWanted = PackageManagement::versionsThatProvideTheWanted((id.clone()).clone(), (version.clone()).clone(), false);
        if !(versionsThatProvideTheWanted.clone().is_empty()) {
            if version.clone() == literal!("default") || version.clone() == literal!("") {
                commands = list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  installPackage(")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone()];
            } else {
                commands = metamodelica::nil();
                if listMember((version.clone()).clone(), versionsThatProvideTheWanted.clone()) {
                    commands = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  installPackage(")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(", \"")); __mm_s.push_str(&*version.clone()); __mm_s.push_str(&*literal!("\", exactMatch=true)")); ArcStr::from(__mm_s) }).clone(), commands.clone());
                }
                commands = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  installPackage(")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(", \"")); __mm_s.push_str(&*version.clone()); __mm_s.push_str(&*literal!("\", exactMatch=false)")); ArcStr::from(__mm_s) }).clone(), commands.clone());
            }
            if listHead(versionsThatProvideTheWanted.clone())? != version.clone() {
                commands = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("  installPackage(")); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*literal!(", \"")); __mm_s.push_str(&*listHead(versionsThatProvideTheWanted.clone())?); __mm_s.push_str(&*literal!("\", exactMatch=true)")); ArcStr::from(__mm_s) }).clone(), commands.clone());
            }
            Error::addMessage(Error::NOTIFY_PKG_FOUND.clone(), list![stringDelimitList(commands.clone(), (literal!("\n")).clone())])?;
        }
        bail!("fail");
    }
    Config::setLanguageStandardFromMSL((name.clone()).clone(), false)?;
    cl = loadClassFromMp((id.clone()).clone(), (mp.clone()).clone(), (name.clone()).clone(), isDir.clone(), encoding.clone(), encrypted.clone())?;
    if isSome(cl.clone()) {
        outProgram = Absyn::Program { classes: list![Util::getOption(cl.clone())?], within_: openmodelica_ast::Absyn::Within::TOP };
    } else {
        outProgram = Absyn::Program { classes: metamodelica::nil(), within_: openmodelica_ast::Absyn::Within::TOP };
    }
    Ok(outProgram)
}

pub fn loadClassFromMp(mut id: ArcStr, mut path: ArcStr, mut name: ArcStr, mut isDir: bool, mut optEncoding: Option<ArcStr>, mut encrypted: bool) -> Result<Option<Arc<Absyn::Class>>> {
    let mut outClass: Option<Arc<Absyn::Class>> = None;
    outClass = (match (id.clone(), path.clone(), name.clone(), isDir.clone(), optEncoding.clone()) {
        (_, _, _, false, _) => {
            let mut pd: ArcStr = arcstr::literal!("");
            let mut encoding: ArcStr = arcstr::literal!("");
            let mut encodingfile: ArcStr = arcstr::literal!("");
            let mut cl: Option<Arc<Absyn::Class>> = None;
            let mut strategy: LoadFileStrategy;
            pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
            encodingfile = stringAppendList(list![(path.clone()).clone(), (pd.clone()).clone(), (literal!("package.encoding")).clone()]);
            encoding = (System::trimChar((System::trimChar((if (System::regularFileExists((encodingfile.clone()).clone())) {System::readFile((encodingfile.clone()).clone())?} else {Util::getOptionOrDefault(optEncoding.clone(), (literal!("UTF-8")).clone())}).clone(), (literal!("\n")).clone())?).clone(), (literal!(" ")).clone())?).clone();
            strategy = LoadFileStrategy::STRATEGY_ON_DEMAND { encoding: (encoding.clone()).clone() };
            cl = parsePackageFile(({ let mut __mm_s = String::new(); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), strategy.clone(), false, openmodelica_ast::Absyn::Within::TOP, (id.clone()).clone(), encrypted.clone())?;
            cl.clone()
        },
        (_, _, _, true, _) => {
            let mut pd: ArcStr = arcstr::literal!("");
            let mut encoding: ArcStr = arcstr::literal!("");
            let mut encodingfile: ArcStr = arcstr::literal!("");
            let mut cl: Option<Arc<Absyn::Class>> = None;
            let mut filenames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut strategy: LoadFileStrategy;
            let mut lveStarted: bool = false;
            let mut lveInstance: Option<i32> = None;
            pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
            encodingfile = stringAppendList(list![(path.clone()).clone(), (pd.clone()).clone(), (name.clone()).clone(), (pd.clone()).clone(), (literal!("package.encoding")).clone()]);
            encoding = (System::trimChar((System::trimChar((if (System::regularFileExists((encodingfile.clone()).clone())) {System::readFile((encodingfile.clone()).clone())?} else {Util::getOptionOrDefault(optEncoding.clone(), (literal!("UTF-8")).clone())}).clone(), (literal!("\n")).clone())?).clone(), (literal!(" ")).clone())?).clone();
            lveInstance = None;
            if encrypted.clone() {
                (lveStarted, lveInstance) = Parser::startLibraryVendorExecutable(({ let mut __mm_s = String::new(); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone());
                if !(lveStarted.clone()) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Unable to start library vendor executable.")).clone()])?;
                    bail!("fail");
                }
            }
            if (Testsuite::isRunning()? || Config::noProc()? == 1) && !(encrypted.clone()) {
                strategy = LoadFileStrategy::STRATEGY_ON_DEMAND { encoding: (encoding.clone()).clone() };
            } else {
                filenames = getAllFilesFromDirectory(({ let mut __mm_s = String::new(); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), encrypted.clone(), metamodelica::nil());
                strategy = LoadFileStrategy::STRATEGY_HASHTABLE { ht: Parser::parallelParseFiles(filenames.clone(), (encoding.clone()).clone(), Config::noProc()?, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone(), lveInstance.clone())? };
            }
            cl = loadCompletePackageFromMp((id.clone()).clone(), (name.clone()).clone(), (path.clone()).clone(), strategy.clone(), openmodelica_ast::Absyn::Within::TOP, Error::getNumErrorMessages(), encrypted.clone())?;
            if encrypted.clone() && lveStarted.clone() {
                Parser::stopLibraryVendorExecutable(lveInstance.clone());
            }
            cl.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outClass)
}

fn getAllFilesFromDirectory(mut dir: ArcStr, mut encrypted: bool, mut acc: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut files: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut subdirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut pd: ArcStr = arcstr::literal!(Autoconf::pathDelimiter);
    if encrypted.clone() {
        files = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*literal!("package.moc")); ArcStr::from(__mm_s) }).clone(), listAppend({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (System::mocFiles((dir.clone()).clone())).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, acc.clone()));
    } else {
        files = cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*literal!("package.mo")); ArcStr::from(__mm_s) }).clone(), listAppend({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut f in (System::moFiles((dir.clone()).clone())).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*f.clone()); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, acc.clone()));
    }
    subdirs = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut d in (List::filter2OnTrue(System::subDirectories((dir.clone()).clone()), (std::sync::Arc::new(fnptr!(existPackage, ArcStr, ArcStr, bool)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, bool) -> Result<bool> + 'static>), (dir.clone()).clone(), encrypted.clone())).into_iter().cloned() {
            let __x = { let mut __mm_s = String::new(); __mm_s.push_str(&*dir.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*d.clone()); ArcStr::from(__mm_s) };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    files = List::fold1(subdirs.clone(), (std::sync::Arc::new(fnptr!(getAllFilesFromDirectory, ArcStr, bool, Arc<metamodelica::List<ArcStr>>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, bool, Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> + 'static>), encrypted.clone(), files.clone());
    files
}

fn loadCompletePackageFromMp(mut id: ArcStr, mut inIdent: ArcStr, mut inString: ArcStr, mut strategy: LoadFileStrategy, mut inWithin: Absyn::Within, mut numError: i32, mut encrypted: bool) -> Result<Option<Arc<Absyn::Class>>> {
    let mut cl: Option<Arc<Absyn::Class>> = None;
    cl = 'mc: {
        let __mc_input = (id.clone(), inIdent.clone(), inString.clone(), inWithin.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut pack, mut mp, mut within_) = __mc_input.clone() else { bail!("nomatch") };
            let mut pd: ArcStr = arcstr::literal!("");
            let mut mp_1: ArcStr = arcstr::literal!("");
            let mut packagefile: ArcStr = arcstr::literal!("");
            let mut orderfile: ArcStr = arcstr::literal!("");
            let mut tv: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut ca: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
            let mut cp: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut cmt: Option<ArcStr> = None;
            let mut opt_cl: Option<Arc<Absyn::Class>> = None;
            let mut class_: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut path: Arc<Absyn::Path>;
            let mut w2: Absyn::Within = Absyn::Within::TOP;
            let mut reverseOrder: Arc<metamodelica::List<PackageOrder>> = metamodelica::nil();
            let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
            pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
            mp_1 = stringAppendList(list![(mp.clone()).clone(), (pd.clone()).clone(), (pack.clone()).clone()]);
            packagefile = stringAppendList(list![(mp_1.clone()).clone(), (pd.clone()).clone(), (if (encrypted.clone()) {literal!("package.moc")} else {literal!("package.mo")}).clone()]);
            orderfile = stringAppendList(list![(mp_1.clone()).clone(), (pd.clone()).clone(), (literal!("package.order")).clone()]);
            if !(System::regularFileExists((packagefile.clone()).clone())) {
                Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected file ")); __mm_s.push_str(&*packagefile.clone()); __mm_s.push_str(&*literal!(" to exist")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                bail!("fail");
            }
            opt_cl = parsePackageFile((packagefile.clone()).clone(), strategy.clone(), true, within_.clone(), (id.clone()).clone(), encrypted.clone())?;
            if isSome(opt_cl.clone()) {
                let (__pa5, __pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Util::getOption(opt_cl.clone())?) {
                    __pa5 @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars: __pa0, classAttrs: __pa1, classParts: __pa2, ann: __pa3, comment: __pa4 }, .. } => (__pa5.clone(), __pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                tv = __pa0.clone();
                ca = __pa1.clone();
                cp = __pa2.clone();
                ann = __pa3.clone();
                cmt = __pa4.clone();
                class_ = __pa5.clone();
                reverseOrder = getPackageContentNames(class_.clone(), (orderfile.clone()).clone(), (mp_1.clone()).clone(), Error::getNumErrorMessages(), encrypted.clone())?;
                path = AbsynUtil::joinWithinPath(within_.clone(), Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }))?;
                w2 = Absyn::Within::WITHIN { path: path.clone() };
                cp = List::fold4(reverseOrder.clone(), (std::sync::Arc::new(loadCompletePackageFromMp2) as std::sync::Arc<dyn ::std::ops::Fn(PackageOrder, ArcStr, LoadFileStrategy, Absyn::Within, bool, Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> + 'static>), (mp_1.clone()).clone(), strategy.clone(), w2.clone(), encrypted.clone(), metamodelica::nil());
                assign_field!(class_.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: tv.clone(), classAttrs: ca.clone(), classParts: cp.clone(), ann: ann.clone(), comment: cmt.clone() }));
                opt_cl = Some(class_.clone());
            }
            Ok(opt_cl.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (_, mut pack, mut mp, _) = __mc_input.clone() else { bail!("nomatch") };
            let true = (numError.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("loadCompletePackageFromMp failed for unknown reason: mp=")); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!(" pack=")); __mm_s.push_str(&*pack.clone()); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cl)
}

fn mergeBefore(mut cp: Arc<Absyn::ClassPart>, mut cps: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
    let mut ocp: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    ocp = (::match_deref::match_deref! { match &((cp.clone(), cps.clone())) {
        (Deref @ Absyn::ClassPart::PUBLIC { contents: ei1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: ei2 }, tail: rest }) => {
            let mut ei: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            ei = listAppend(ei1.clone(), ei2.clone());
            cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: ei.clone() }), rest.clone())
        },
        (Deref @ Absyn::ClassPart::PROTECTED { contents: ei1 }, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: ei2 }, tail: rest }) => {
            let mut ei: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            ei = listAppend(ei1.clone(), ei2.clone());
            cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: ei.clone() }), rest.clone())
        },
        _ => {
            cons(cp.clone(), cps.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ocp
}

fn loadCompletePackageFromMp2(mut po: PackageOrder, mut mp: ArcStr, mut strategy: LoadFileStrategy, mut w1: Absyn::Within, mut encrypted: bool, mut acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut cps: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    cps = (match po.clone() {
        PackageOrder::CLASSPART { cp: mut cp } => {
            cps = mergeBefore(cp.clone(), acc.clone());
            cps.clone()
        },
        PackageOrder::ELEMENT { element: ref ei, r#pub: true } => {
            cps = mergeBefore(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![ei.clone()] }), acc.clone());
            cps.clone()
        },
        PackageOrder::ELEMENT { element: ref ei, r#pub: false } => {
            cps = mergeBefore(Arc::new(Absyn::ClassPart::PROTECTED { contents: list![ei.clone()] }), acc.clone());
            cps.clone()
        },
        PackageOrder::CLASSLOAD { cl: mut id } => {
            let mut ei: Arc<Absyn::ElementItem>;
            let mut pd: ArcStr = arcstr::literal!("");
            let mut file: ArcStr = arcstr::literal!("");
            let mut cl: Option<Arc<Absyn::Class>> = None;
            let mut bDirectoryAndFileExists: bool = false;
            pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
            file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*if (encrypted.clone()) {literal!("/package.moc")} else {literal!("/package.mo")}); ArcStr::from(__mm_s) }).clone();
            bDirectoryAndFileExists = System::directoryExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*id.clone()); ArcStr::from(__mm_s) }).clone()) && System::regularFileExists((file.clone()).clone());
            if bDirectoryAndFileExists.clone() {
                cl = loadCompletePackageFromMp((id.clone()).clone(), (id.clone()).clone(), (mp.clone()).clone(), strategy.clone(), w1.clone(), Error::getNumErrorMessages(), encrypted.clone())?;
                if isSome(cl.clone()) {
                    ei = AbsynUtil::makeClassElement(Util::getOption(cl.clone())?)?;
                    cps = mergeBefore(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![ei.clone()] }), acc.clone());
                } else {
                    cps = acc.clone();
                }
            } else {
                file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*id.clone()); __mm_s.push_str(&*if (encrypted.clone()) {literal!(".moc")} else {literal!(".mo")}); ArcStr::from(__mm_s) }).clone();
                if !(System::regularFileExists((file.clone()).clone())) {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expected file ")); __mm_s.push_str(&*file.clone()); __mm_s.push_str(&*literal!(" to exist")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    bail!("fail");
                }
                cl = parsePackageFile((file.clone()).clone(), strategy.clone(), false, w1.clone(), (id.clone()).clone(), encrypted.clone())?;
                if isSome(cl.clone()) {
                    ei = AbsynUtil::makeClassElement(Util::getOption(cl.clone())?)?;
                    cps = mergeBefore(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![ei.clone()] }), acc.clone());
                } else {
                    cps = acc.clone();
                }
            }
            cps.clone()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(cps)
}

pub fn parsePackageFile(mut name: ArcStr, mut strategy: LoadFileStrategy, mut expectPackage: bool, mut w1: Absyn::Within, mut pack: ArcStr, mut encrypted: bool) -> Result<Option<Arc<Absyn::Class>>> {
    let mut cl: Option<Arc<Absyn::Class>> = None;
    let mut class_: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut cs: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut w2: Absyn::Within = Absyn::Within::TOP;
    let mut classNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s1: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    let mut cname: ArcStr = arcstr::literal!("");
    let mut body: Arc<Absyn::ClassDef>;
    let Absyn::PROGRAM { classes: __pa0, within_: __pa1 } = (getProgramFromStrategy((name.clone()).clone(), strategy.clone())?) else { bail!("pattern mismatch") };
    cs = __pa0.clone();
    w2 = __pa1.clone();
    classNames = List::map(cs.clone(), (std::sync::Arc::new(AbsynUtil::getClassName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<ArcStr> + 'static>));
    r#str = stringDelimitList(classNames.clone(), (literal!(", ")).clone());
    if !((cs.clone().len() as i32) == 1) {
        if encrypted.clone() {
            cl = None;
            return Ok(cl);
        } else {
            Error::addSourceMessage(Error::LIBRARY_ONE_PACKAGE_PER_FILE.clone(), list![(r#str.clone()).clone()], SourceInfo { fileName: (name.clone()).clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) })?;
            bail!("fail");
        }
    }
    let (__pa5, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(cs.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa5 @ Deref @ Absyn::Class { info: __pa2, body: __pa3, name: __pa4, .. }, tail: Deref @ metamodelica::List::Nil } => (__pa5.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa2.clone();
    body = __pa3.clone();
    cname = __pa4.clone();
    class_ = __pa5.clone();
    cl = Some(class_.clone());
    if !(stringEqual((cname.clone()).clone(), (pack.clone()).clone())) {
        if stringEqual((System::tolower((cname.clone()).clone())).clone(), (System::tolower((pack.clone()).clone())).clone()) {
            Error::addSourceMessage(Error::LIBRARY_UNEXPECTED_NAME_CASE_SENSITIVE.clone(), list![(pack.clone()).clone(), (cname.clone()).clone()], info.clone())?;
        } else {
            Error::addSourceMessage(Error::LIBRARY_UNEXPECTED_NAME.clone(), list![(pack.clone()).clone(), (cname.clone()).clone()], info.clone())?;
            bail!("fail");
        }
    }
    if expectPackage.clone() && !(AbsynUtil::isParts(body.clone())) {
        Error::addSourceMessage(Error::LIBRARY_EXPECTED_PARTS.clone(), list![(pack.clone()).clone()], info.clone())?;
        bail!("fail");
    } else if !(AbsynUtil::withinEqual(w1.clone(), w2.clone()) || Config::languageStandardAtMost(Config::LanguageStandard::_2_x.clone())?) {
        s1 = (AbsynUtil::withinString(w1.clone())?).clone();
        s2 = (AbsynUtil::withinString(w2.clone())?).clone();
        if AbsynUtil::withinEqualCaseInsensitive(w1.clone(), w2.clone()) {
            Error::addSourceMessage(Error::LIBRARY_WITHIN_WRONG_CASE.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
        } else {
            Error::addSourceMessage(Error::LIBRARY_UNEXPECTED_WITHIN.clone(), list![(s1.clone()).clone(), (s2.clone()).clone()], info.clone())?;
            bail!("fail");
        }
    }
    Ok(cl)
}

fn getBothPackageAndFilename(mut r#str: ArcStr, mut mp: ArcStr) -> Result<ArcStr> {
    let mut out: ArcStr = arcstr::literal!("");
    out = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Testsuite::friendly((System::realpath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone())?).clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Testsuite::friendly((System::realpath(({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("/package.mo")); ArcStr::from(__mm_s) }).clone())?).clone())?); ArcStr::from(__mm_s) }).clone();
    Ok(out)
}

fn getPackageContentNames(mut cl: Arc<Absyn::Class>, mut filename: ArcStr, mut mp: ArcStr, mut numError: i32, mut encrypted: bool) -> Result<Arc<metamodelica::List<PackageOrder>>> {
    let mut po: Arc<metamodelica::List<PackageOrder>> = metamodelica::nil();
    po = 'mc: {
        let __mc_input = (cl.clone(), filename.clone(), mp.clone(), numError.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { info, body: Deref @ Absyn::ClassDef::PARTS { classParts: cp, .. }, .. }, _, _, _) => {
                    let mut contents: ArcStr = arcstr::literal!("");
                    let mut duplicatesStr: ArcStr = arcstr::literal!("");
                    let mut differencesStr: ArcStr = arcstr::literal!("");
                    let mut duplicates: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut namesToFind: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut mofiles: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut subdirs: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut differences: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut intersection: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut po1: Arc<metamodelica::List<PackageOrder>> = metamodelica::nil();
                    let mut po2: Arc<metamodelica::List<PackageOrder>> = metamodelica::nil();
                    let mut po: Arc<metamodelica::List<PackageOrder>> = po.clone();
                    match '__try0: {
                        let true = (System::regularFileExists((filename.clone()).clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
                        contents = (unwrap_break_err!(System::readFile((filename.clone()).clone()), '__try0)).clone();
                        namesToFind = System::strtok((contents.clone()).clone(), (literal!("\n")).clone());
                        namesToFind = List::removeOnTrue((literal!("")).clone(), (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), List::map(namesToFind.clone(), (std::sync::Arc::new(fnptr!(System::trimWhitespace, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>)));
                        duplicates = unwrap_break_err!(List::sortedDuplicates(List::sort(namesToFind.clone(), (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?, (std::sync::Arc::new(stringEq) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)), '__try0);
                        duplicatesStr = stringDelimitList(duplicates.clone(), (literal!(", ")).clone());
                        unwrap_break_err!(Error::assertionOrAddSourceMessage(duplicates.clone().is_empty(), Error::PACKAGE_ORDER_DUPLICATES.clone(), list![(duplicatesStr.clone()).clone()], SourceInfo { fileName: (filename.clone()).clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) }), '__try0);
                        if encrypted.clone() {
                            mofiles = List::map(System::mocFiles((mp.clone()).clone()), (std::sync::Arc::new(Util::removeLast4Char) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>));
                        } else {
                            mofiles = List::map(System::moFiles((mp.clone()).clone()), (std::sync::Arc::new(Util::removeLast3Char) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>));
                        }
                        subdirs = System::subDirectories((mp.clone()).clone());
                        subdirs = List::filter2OnTrue(subdirs.clone(), (std::sync::Arc::new(fnptr!(existPackage, ArcStr, ArcStr, bool)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, bool) -> Result<bool> + 'static>), (mp.clone()).clone(), encrypted.clone());
                        intersection = List::intersectionOnTrue(subdirs.clone(), mofiles.clone(), (std::sync::Arc::new(stringEq) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>));
                        differencesStr = stringDelimitList(List::map1(intersection.clone(), (std::sync::Arc::new(getBothPackageAndFilename) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (mp.clone()).clone()), (literal!(", ")).clone());
                        unwrap_break_err!(Error::assertionOrAddSourceMessage(intersection.clone().is_empty(), Error::PACKAGE_DUPLICATE_CHILDREN.clone(), list![(differencesStr.clone()).clone()], SourceInfo { fileName: (filename.clone()).clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) }), '__try0);
                        mofiles = listAppend(subdirs.clone(), mofiles.clone());
                        differences = unwrap_break_err!(List::setDifference(mofiles.clone(), namesToFind.clone()), '__try0);
                        po1 = unwrap_break_err!(getPackageContentNamesinParts(namesToFind.clone(), cp.clone(), metamodelica::nil()), '__try0);
                        (po1, differences) = List::map3Fold(po1.clone(), (std::sync::Arc::new(checkPackageOrderFilesExist) as std::sync::Arc<dyn ::std::ops::Fn(PackageOrder, ArcStr, SourceInfo, bool, Arc<metamodelica::List<ArcStr>>) -> Result<(PackageOrder, Arc<metamodelica::List<ArcStr>>)> + 'static>), (mp.clone()).clone(), info.clone(), encrypted.clone(), differences.clone());
                        differencesStr = stringDelimitList(differences.clone(), (literal!("\n\t")).clone());
                        unwrap_break_err!(Error::assertionOrAddSourceMessage(differences.clone().is_empty(), Error::PACKAGE_ORDER_FILE_NOT_COMPLETE.clone(), list![(differencesStr.clone()).clone()], SourceInfo { fileName: (filename.clone()).clone(), isReadOnly: true, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: metamodelica::OrderedFloat(0.0_f64) }), '__try0);
                        po2 = List::map(differences.clone(), (std::sync::Arc::new(fnptr!(makeClassLoad, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<PackageOrder> + 'static>));
                        po = listAppend(po2.clone(), po1.clone());
                        Ok::<_, anyhow::Error>((differencesStr.clone(), intersection.clone(), mofiles.clone(), po.clone(), subdirs.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
                            differencesStr = __try0_o0;
                            intersection = __try0_o1;
                            mofiles = __try0_o2;
                            po = __try0_o3;
                            subdirs = __try0_o4;
                        }
                        Err(_) => {
                            mofiles = List::map(System::moFiles((mp.clone()).clone()), (std::sync::Arc::new(Util::removeLast3Char) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>));
                            subdirs = System::subDirectories((mp.clone()).clone());
                            subdirs = List::filter2OnTrue(subdirs.clone(), (std::sync::Arc::new(fnptr!(existPackage, ArcStr, ArcStr, bool)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr, bool) -> Result<bool> + 'static>), (mp.clone()).clone(), encrypted.clone());
                            mofiles = List::sort(listAppend(subdirs.clone(), mofiles.clone()), (std::sync::Arc::new(fnptr!(Util::strcmpBool, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
                            intersection = List::sortedDuplicates(mofiles.clone(), (std::sync::Arc::new(stringEq) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
                            differencesStr = stringDelimitList(List::map1(intersection.clone(), (std::sync::Arc::new(getBothPackageAndFilename) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (mp.clone()).clone()), (literal!(", ")).clone());
                            Error::assertionOrAddSourceMessage(intersection.clone().is_empty(), Error::PACKAGE_DUPLICATE_CHILDREN.clone(), list![(differencesStr.clone()).clone()], info.clone())?;
                            po = listAppend(List::map(cp.clone(), (std::sync::Arc::new(fnptr!(makeClassPart, Arc<Absyn::ClassPart>)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ClassPart>) -> Result<PackageOrder> + 'static>)), List::map(mofiles.clone(), (std::sync::Arc::new(fnptr!(makeClassLoad, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<PackageOrder> + 'static>)));
                        }
                    }
                    Ok(po.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { info, .. }, _, _, _) => {
                    let true = (numError.clone() == Error::getNumErrorMessages()) else { bail!("pattern mismatch") };
                    Error::addSourceMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("getPackageContentNames failed for unknown reason")).clone()], info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(po)
}

fn makeClassPart(mut part: Arc<Absyn::ClassPart>) -> PackageOrder {
    let mut po: PackageOrder;
    po = PackageOrder::CLASSPART { cp: part.clone() };
    po
}

fn makeElement(mut el: Arc<Absyn::ElementItem>, mut r#pub: bool) -> PackageOrder {
    let mut po: PackageOrder;
    po = PackageOrder::ELEMENT { element: el.clone(), r#pub: r#pub.clone() };
    po
}

fn makeClassLoad(mut r#str: ArcStr) -> PackageOrder {
    let mut po: PackageOrder;
    po = PackageOrder::CLASSLOAD { cl: (r#str.clone()).clone() };
    po
}

fn checkPackageOrderFilesExist(mut po: PackageOrder, mut mp: ArcStr, mut info: SourceInfo, mut encrypted: bool, mut differences: Arc<metamodelica::List<ArcStr>>) -> Result<(PackageOrder, Arc<metamodelica::List<ArcStr>>)> {
    let mut po: PackageOrder = po;
    let mut differences: Arc<metamodelica::List<ArcStr>> = differences;
    let _ = (match (po.clone(), mp.clone(), info.clone()) {
        (PackageOrder::CLASSLOAD { cl: mut r#str }, _, _) => {
            let mut pd: ArcStr = arcstr::literal!("");
            let mut str2: ArcStr = arcstr::literal!("");
            let mut str3: ArcStr = arcstr::literal!("");
            let mut str4: ArcStr = arcstr::literal!("");
            pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
            str2 = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*if (encrypted.clone()) {literal!(".moc")} else {literal!(".mo")}); ArcStr::from(__mm_s) }).clone();
            if !(System::directoryExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*r#str.clone()); ArcStr::from(__mm_s) }).clone()) || System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*str2.clone()); ArcStr::from(__mm_s) }).clone())) {
                if let Ok(__iflet0) = List::find(System::moFiles((mp.clone()).clone()), Arc::new({ let __pe_b1 = (System::tolower((str2.clone()).clone())).clone(); move |__pe_a0| Ok(Util::stringEqCaseInsensitive(__pe_a0, __pe_b1.clone())) })) {
                    str3 = __iflet0;
                } else {
                    Error::addSourceMessage(Error::PACKAGE_ORDER_FILE_NOT_FOUND.clone(), list![(r#str.clone()).clone()], info.clone())?;
                    bail!("fail");
                }
                Error::addSourceMessage(Error::PACKAGE_ORDER_CASE_SENSITIVE.clone(), list![(r#str.clone()).clone(), (str2.clone()).clone(), (str3.clone()).clone()], info.clone())?;
                str4 = (Util::removeLastNChar((str3.clone()).clone(), if (encrypted.clone()) {4} else {3})?).clone();
                differences = List::removeOnTrue((str4.clone()).clone(), (std::sync::Arc::new(stringEq) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), differences.clone());
                po = PackageOrder::CLASSLOAD { cl: (str4.clone()).clone() };
            }
            ()
        },
        _ => {
            ()
        },
    });
    Ok((po, differences))
}

fn existPackage(mut name: ArcStr, mut mp: ArcStr, mut encrypted: bool) -> bool {
    let mut b: bool = false;
    let mut pd: ArcStr = arcstr::literal!("");
    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
    b = System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*pd.clone()); __mm_s.push_str(&*if (encrypted.clone()) {literal!("package.moc")} else {literal!("package.mo")}); ArcStr::from(__mm_s) }).clone());
    b
}

fn getPackageContentNamesinParts(mut inNamesToSort: Arc<metamodelica::List<ArcStr>>, mut cps: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut acc: Arc<metamodelica::List<PackageOrder>>) -> Result<Arc<metamodelica::List<PackageOrder>>> {
    let mut outOrder: Arc<metamodelica::List<PackageOrder>> = metamodelica::nil();
    outOrder = (::match_deref::match_deref! { match &((inNamesToSort.clone(), cps.clone(), acc.clone())) {
        (namesToSort, Deref @ metamodelica::List::Nil, _) => {
            outOrder = listAppend(List::mapReverse(namesToSort.clone(), (std::sync::Arc::new(fnptr!(makeClassLoad, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<PackageOrder> + 'static>)), acc.clone());
            outOrder.clone()
        },
        (namesToSort, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rcp }, _) => {
            let mut namesToSort = (*namesToSort).clone();
            (outOrder, namesToSort) = getPackageContentNamesinElts(namesToSort.clone(), elts.clone(), acc.clone(), true)?;
            outOrder = getPackageContentNamesinParts(namesToSort.clone(), rcp.clone(), outOrder.clone())?;
            outOrder.clone()
        },
        (namesToSort, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rcp }, _) => {
            let mut namesToSort = (*namesToSort).clone();
            (outOrder, namesToSort) = getPackageContentNamesinElts(namesToSort.clone(), elts.clone(), acc.clone(), false)?;
            outOrder = getPackageContentNamesinParts(namesToSort.clone(), rcp.clone(), outOrder.clone())?;
            outOrder.clone()
        },
        (namesToSort, Deref @ metamodelica::List::Cons { head: cp, tail: rcp }, _) => {
            outOrder = getPackageContentNamesinParts(namesToSort.clone(), rcp.clone(), cons(PackageOrder::CLASSPART { cp: cp.clone() }, acc.clone()))?;
            outOrder.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outOrder)
}

fn getPackageContentNamesinElts(mut inNamesToSort: Arc<metamodelica::List<ArcStr>>, mut inElts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut po: Arc<metamodelica::List<PackageOrder>>, mut r#pub: bool) -> Result<(Arc<metamodelica::List<PackageOrder>>, Arc<metamodelica::List<ArcStr>>)> {
    let mut outOrder: Arc<metamodelica::List<PackageOrder>> = metamodelica::nil();
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (outOrder, outNames) = (::match_deref::match_deref! { match &((inNamesToSort.clone(), inElts.clone(), po.clone(), r#pub.clone())) {
        (namesToSort, Deref @ metamodelica::List::Nil, _, _) => {
            (po.clone(), namesToSort.clone())
        },
        (Deref @ metamodelica::List::Cons { head: name1, tail: _ }, Deref @ metamodelica::List::Cons { head: ei @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { info, specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, .. }, .. } }, tail: elts }, _, _) => {
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut compNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut b: bool = false;
            let mut orderElt: PackageOrder;
            compNames = List::map(comps.clone(), (std::sync::Arc::new(AbsynUtil::componentName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentItem>) -> Result<ArcStr> + 'static>));
            (names, b) = matchCompNames(inNamesToSort.clone(), compNames.clone(), info.clone())?;
            orderElt = if (b.clone()) {makeElement(ei.clone(), r#pub.clone())} else {makeClassLoad((name1.clone()).clone())};
            (outOrder, names) = getPackageContentNamesinElts(names.clone(), if (b.clone()) {elts.clone()} else {inElts.clone()}, cons(orderElt.clone(), po.clone()), r#pub.clone())?;
            (outOrder.clone(), names.clone())
        },
        (Deref @ metamodelica::List::Cons { head: name1, tail: namesToSort }, Deref @ metamodelica::List::Cons { head: ei @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { info, name: name2, .. }, .. }, .. } }, tail: elts }, _, _) => {
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut b: bool = false;
            let mut orderElt: PackageOrder;
            let mut load: PackageOrder;
            load = makeClassLoad((name1.clone()).clone());
            b = name1.clone() == name2.clone();
            Error::assertionOrAddSourceMessage(if (b.clone()) {!(listMember(load.clone(), po.clone()))} else {true}, Error::PACKAGE_MO_NOT_IN_ORDER.clone(), list![(name2.clone()).clone()], info.clone())?;
            orderElt = if (b.clone()) {makeElement(ei.clone(), r#pub.clone())} else {load.clone()};
            (outOrder, names) = getPackageContentNamesinElts(namesToSort.clone(), if (b.clone()) {elts.clone()} else {inElts.clone()}, cons(orderElt.clone(), po.clone()), r#pub.clone())?;
            (outOrder.clone(), names.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { info, name: name2, .. }, .. }, .. } }, tail: _ }, _, _) => {
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut load: PackageOrder;
            load = makeClassLoad((name2.clone()).clone());
            Error::assertionOrAddSourceMessage(!(listMember(load.clone(), po.clone())), Error::PACKAGE_MO_NOT_IN_ORDER.clone(), list![(name2.clone()).clone()], info.clone())?;
            Error::addSourceMessage(Error::FOUND_ELEMENT_NOT_IN_ORDER_FILE.clone(), list![(name2.clone()).clone()], info.clone())?;
            (outOrder, names) = getPackageContentNamesinElts(cons((name2.clone()).clone(), inNamesToSort.clone()), inElts.clone(), po.clone(), r#pub.clone())?;
            (outOrder.clone(), names.clone())
        },
        (Deref @ metamodelica::List::Nil, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { info, specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name: name2, .. }, .. }, tail: _ }, .. }, .. } }, tail: _ }, _, _) => {
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut load: PackageOrder;
            load = makeClassLoad((name2.clone()).clone());
            Error::assertionOrAddSourceMessage(!(listMember(load.clone(), po.clone())), Error::PACKAGE_MO_NOT_IN_ORDER.clone(), list![(name2.clone()).clone()], info.clone())?;
            Error::addSourceMessage(Error::FOUND_ELEMENT_NOT_IN_ORDER_FILE.clone(), list![(name2.clone()).clone()], info.clone())?;
            (outOrder, names) = getPackageContentNamesinElts(cons((name2.clone()).clone(), inNamesToSort.clone()), inElts.clone(), po.clone(), r#pub.clone())?;
            (outOrder.clone(), names.clone())
        },
        (namesToSort, Deref @ metamodelica::List::Cons { head: ei, tail: elts }, _, _) => {
            let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            (outOrder, names) = getPackageContentNamesinElts(namesToSort.clone(), elts.clone(), cons(PackageOrder::ELEMENT { element: ei.clone(), r#pub: r#pub.clone() }, po.clone()), r#pub.clone())?;
            (outOrder.clone(), names.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outOrder, outNames))
}

fn matchCompNames(mut names: Arc<metamodelica::List<ArcStr>>, mut comps: Arc<metamodelica::List<ArcStr>>, mut info: SourceInfo) -> Result<(Arc<metamodelica::List<ArcStr>>, bool)> {
    let mut outNames: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut matchedNames: bool = false;
    (outNames, matchedNames) = (::match_deref::match_deref! { match &((names.clone(), comps.clone(), info.clone())) {
        (_, Deref @ metamodelica::List::Nil, _) => {
            (names.clone(), true)
        },
        (Deref @ metamodelica::List::Cons { head: n1, tail: rest1 }, Deref @ metamodelica::List::Cons { head: n2, tail: rest2 }, _) => {
            let mut b: bool = false;
            let mut b1: bool = false;
            let mut rest1 = (*rest1).clone();
            if n1.clone() == n2.clone() {
                (rest1, b) = matchCompNames(rest1.clone(), rest2.clone(), info.clone())?;
                Error::assertionOrAddSourceMessage(b.clone(), Error::ORDER_FILE_COMPONENTS.clone(), metamodelica::nil(), info.clone())?;
                b1 = true;
            } else {
                b1 = false;
            }
            (rest1.clone(), b1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNames, matchedNames))
}

fn packageOrderName(mut ord: PackageOrder) -> ArcStr {
    let mut name: ArcStr = arcstr::literal!("");
    name = ((match ord.clone() {
        PackageOrder::CLASSLOAD { cl: mut name } => name.clone(),
        _ => literal!("#"),
    })).clone();
    name
}

pub fn checkOnLoadMessage(mut p1: Absyn::Program) -> Result<()> {
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let Absyn::PROGRAM { classes: __pa0, .. } = (p1.clone()) else { bail!("pattern mismatch") };
    classes = __pa0.clone();
    let _ = List::map2(classes.clone(), (std::sync::Arc::new(AbsynUtil::getNamedAnnotationInClass) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>, Arc<Absyn::Path>, _) -> Result<_> + 'static>), Arc::new(Absyn::Path::IDENT { name: (literal!("__OpenModelica_messageOnLoad")).clone() }), (std::sync::Arc::new(checkOnLoadMessageWork) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<i32> + 'static>));
    Ok(())
}

fn checkOnLoadMessageWork(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<i32> {
    let mut dummy: i32 = 0;
    dummy = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: Deref @ Absyn::Exp::STRING { value: r#str }, info }, .. }) => {
            Error::addSourceMessage(Error::COMPILER_NOTIFICATION_SCRIPTING.clone(), list![(r#str.clone()).clone()], info.clone())?;
            1
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(dummy)
}

fn getProgramFromStrategy(mut filename: ArcStr, mut strategy: LoadFileStrategy) -> Result<Absyn::Program> {
    let mut program: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut f: ArcStr = filename.clone();
    program = (match strategy.clone() {
        LoadFileStrategy::STRATEGY_HASHTABLE { .. } => {
            if !(BaseHashTable::hasKey((filename.clone()).clone(), var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone())) {
                if let Ok(__iflet0) = List::getMemberOnTrue((filename.clone()).clone(), BaseHashTable::hashTableKeyList(var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone()), (std::sync::Arc::new(fnptr!(Util::stringEqCaseInsensitive, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>)) {
                    f = __iflet0;
                } else {
                    Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("HashTable missing file: ")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(" - all entries include:\n")); __mm_s.push_str(&*stringDelimitList(BaseHashTable::hashTableKeyList(var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone()), (literal!("\n")).clone())); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
                    bail!("fail");
                }
            }
            BaseHashTable::get((f.clone()).clone(), var_field!(strategy.ht, LoadFileStrategy::STRATEGY_HASHTABLE).clone())?
        },
        LoadFileStrategy::STRATEGY_ON_DEMAND { .. } => Parser::parse((filename.clone()).clone(), (var_field!(strategy.encoding, LoadFileStrategy::STRATEGY_ON_DEMAND).clone()).clone(), (literal!("")).clone(), None, Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?,
        _ => bail!("match: no arm matched"),
    });
    Ok(program)
}

