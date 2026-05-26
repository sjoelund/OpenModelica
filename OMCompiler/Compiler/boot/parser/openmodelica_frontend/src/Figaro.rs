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

use crate::FBuiltin;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Autoconf;
use openmodelica_util::Error;
use openmodelica_util::System;

// Imports
// Aliases
pub type Ident = ArcStr;

pub type Path = Arc<Absyn::Path>;

pub type TypeSpec = Arc<Absyn::TypeSpec>;

pub fn run(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inPath: Path, mut workingDir: ArcStr, mut inDatabaseFile: ArcStr, mut inMode: ArcStr, mut inOptions: ArcStr, mut inFigaroProcessorFile: ArcStr) -> Result<()> {
    let mut bdfFile: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*workingDir.clone()); __mm_s.push_str(&*literal!("/FigaroObjects.fi")); ArcStr::from(__mm_s) };
    let mut figaroFile: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*workingDir.clone()); __mm_s.push_str(&*literal!("/Figaro0.fi")); ArcStr::from(__mm_s) };
    let mut argumentFile: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*workingDir.clone()); __mm_s.push_str(&*literal!("/figp_commands.xml")); ArcStr::from(__mm_s) };
    let mut resultFile: ArcStr = { let mut __mm_s = String::new(); __mm_s.push_str(&*System::pwd()); __mm_s.push_str(&*literal!("/result.xml")); ArcStr::from(__mm_s) };
    let mut program: Arc<SCode::Element>;
    let mut figaro: ArcStr = arcstr::literal!("");
    let mut database: ArcStr = arcstr::literal!("");
    let mut xml: ArcStr = arcstr::literal!("");
    let mut xml2: ArcStr = arcstr::literal!("");
    let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    program = FBuiltin::getElementWithPathCheckBuiltin(inProgram.clone(), inPath.clone())?;
    figaro = (makeFigaro(inProgram.clone(), program.clone(), inProgram.clone())?).clone();
    if figaro.clone() == literal!("") {
        bail!("fail");
    }
    System::writeFile((bdfFile.clone()).clone(), (figaro.clone()).clone())?;
    database = (inDatabaseFile.clone()).clone();
    xml = (makeXml((workingDir.clone()).clone(), (database.clone()).clone(), (bdfFile.clone()).clone(), (inMode.clone()).clone(), (inOptions.clone()).clone(), (figaroFile.clone()).clone())?).clone();
    System::writeFile((argumentFile.clone()).clone(), (xml.clone()).clone())?;
    callFigaroProcessor((inFigaroProcessorFile.clone()).clone(), (argumentFile.clone()).clone());
    if arcstr::literal!(Autoconf::os) == literal!("Windows_NT") {
        System::systemCall((literal!("timeout 5")).clone(), (literal!("")).clone());
    } else {
        System::systemCall((literal!("sleep 5")).clone(), (literal!("")).clone());
    }
    xml2 = (System::readFile((resultFile.clone()).clone())?).clone();
    sl = interpret((xml2.clone()).clone())?;
    if reportErrors(sl.clone())? {
        bail!("fail");
    }
    Ok(())
}

/// A class that has a corresponding class in Figaro.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FigaroClass {
    pub className: Ident,
    /// Figaro type name
    pub typeName: ArcStr,
}

impl Default for FigaroClass {
    fn default() -> Self {
        Self {
            className: Default::default(),
            typeName: Default::default(),
        }
    }
}

pub type FIGAROCLASS = FigaroClass;


/// A component that will be an object in Figaro.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FigaroObject {
    pub objectName: ArcStr,
    /// Figaro type name
    pub typeName: ArcStr,
    /// a piece of Figaro code that belongs to the object
    pub figaroCode: ArcStr,
}

impl Default for FigaroObject {
    fn default() -> Self {
        Self {
            objectName: Default::default(),
            typeName: Default::default(),
            figaroCode: Default::default(),
        }
    }
}

pub type FIGAROOBJECT = FigaroObject;


pub fn makeFigaro(mut inProgram: Arc<metamodelica::List<Arc<SCode::Element>>>, mut inModel: Arc<SCode::Element>, mut env: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<ArcStr> {
    let mut outCode: ArcStr = arcstr::literal!("");
    let mut fcl: Arc<metamodelica::List<FigaroClass>> = metamodelica::nil();
    let mut fol: Arc<metamodelica::List<FigaroObject>> = metamodelica::nil();
    fcl = listAppend(fcElementList((literal!("Figaro_Object")).clone(), (literal!("")).clone(), inModel.clone(), None, inProgram.clone(), env.clone())?, fcElementList((literal!("Figaro_Object_connector")).clone(), (literal!("")).clone(), inModel.clone(), None, inProgram.clone(), env.clone())?);
    printFigaroClassList(fcl.clone())?;
    println!("{}", (literal!("\n\n")).clone());
    fol = foElement(fcl.clone(), inModel.clone())?;
    printFigaroObjectList(fol.clone())?;
    outCode = (figaroObjectListToString(fol.clone())?).clone();
    Ok(outCode)
}

/* Finds all classes derived from the specified base class and also
carries along the Figaro type name in order to assign the correct Figaro type to a class if it
does not have an explicit fullClassName modifier. */
fn fcElement(mut inFigaroBase: Ident, mut inFigaroType: ArcStr, mut inProgram: Arc<SCode::Element>, mut inClassName: Option<ArcStr>, mut inElement: Arc<SCode::Element>, mut env: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<FigaroClass>>> {
    let mut outFigaroClassList: Arc<metamodelica::List<FigaroClass>> = metamodelica::nil();
    outFigaroClassList = 'mc: {
        let __mc_input = (inFigaroBase.clone(), inFigaroType.clone(), inProgram.clone(), inClassName.clone(), inElement.clone(), env.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, Some(cn), Deref @ SCode::Element::EXTENDS { modifications: m, baseClassPath: bcp, .. }, e) => {
                    let mut tn: ArcStr = arcstr::literal!("");
                    let true = (fb.clone() == getLastIdent(bcp.clone())?) else { bail!("pattern mismatch") };
                    tn = (fcMod1(m.clone())?).clone();
                    Ok(fcAddFigaroClass((ft.clone()).clone(), program.clone(), (cn.clone()).clone(), (tn.clone()).clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, Some(cn), Deref @ SCode::Element::EXTENDS { modifications: m, baseClassPath: bcp, .. }, e) => {
                    let mut cdef: Arc<SCode::Element>;
                    let mut tn: ArcStr = arcstr::literal!("");
                    cdef = FBuiltin::getElementWithPathCheckBuiltin(e.clone(), bcp.clone())?;
                    let true = (fcExtends((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), Some((cn.clone()).clone()), cdef.clone(), e.clone())?) else { bail!("pattern mismatch") };
                    tn = (fcMod1(m.clone())?).clone();
                    Ok(fcAddFigaroClass((ft.clone()).clone(), program.clone(), (cn.clone()).clone(), (tn.clone()).clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, _, Deref @ SCode::Element::CLASS { classDef: cd, name: n, .. }, e) => {
                    Ok(fcClassDef((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), (n.clone()).clone(), cd.clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFigaroClassList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fcExtends(mut inFigaroBase: Ident, mut inFigaroType: ArcStr, mut inProgram: Arc<SCode::Element>, mut inClassName: Option<ArcStr>, mut inElement: Arc<SCode::Element>, mut env: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<bool> {
    let mut doExtend: bool = false;
    doExtend = 'mc: {
        let __mc_input = (inFigaroBase.clone(), inFigaroType.clone(), inProgram.clone(), inClassName.clone(), inElement.clone(), env.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, _, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: el, .. }, name: n, .. }, e) => {
                    Ok(fcElementListExt((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), Some((n.clone()).clone()), el.clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, _, _, Some(_), Deref @ SCode::Element::EXTENDS { baseClassPath: bcp, .. }, _) => {
                    let true = (fb.clone() == getLastIdent(bcp.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, Some(cn), Deref @ SCode::Element::EXTENDS { baseClassPath: bcp, .. }, e) => {
                    let mut cdef: Arc<SCode::Element>;
                    cdef = FBuiltin::getElementWithPathCheckBuiltin(e.clone(), bcp.clone())?;
                    Ok(fcExtends((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), Some((cn.clone()).clone()), cdef.clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(doExtend)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fcElementListExt(mut inFigaroBase: Ident, mut inFigaroType: ArcStr, mut inProgram: Arc<SCode::Element>, mut inClassName: Option<ArcStr>, mut inElementList: Arc<metamodelica::List<Arc<SCode::Element>>>, mut env: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<bool> {
    let mut res: bool = false;
    res = 'mc: {
        let __mc_input = (inFigaroBase.clone(), inFigaroType.clone(), inProgram.clone(), inClassName.clone(), inElementList.clone(), env.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Nil, _) => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::List::Cons { head: first, tail: _ }, e) => {
                    let true = (fcExtends((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), cn.clone(), first.clone(), e.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::List::Cons { head: _, tail: rest }, e) => {
                    Ok(fcElementListExt((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), cn.clone(), rest.clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

fn fcAddFigaroClass(mut inFigaroType: ArcStr, mut inProgram: Arc<SCode::Element>, mut inClassName: Ident, mut inTypeName: ArcStr, mut env: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<FigaroClass>>> {
    let mut outFigaroClassList: Arc<metamodelica::List<FigaroClass>> = metamodelica::nil();
    let mut tn: ArcStr = arcstr::literal!("");
    let mut fc: FigaroClass;
    tn = (if (inTypeName.clone() == literal!("")) {inFigaroType.clone()} else {inTypeName.clone()}).clone();
    fc = FigaroClass { className: (inClassName.clone()).clone(), typeName: (tn.clone()).clone() };
    outFigaroClassList = cons(fc.clone(), fcElement((inClassName.clone()).clone(), (tn.clone()).clone(), inProgram.clone(), None, inProgram.clone(), env.clone())?);
    Ok(outFigaroClassList)
}

fn fcClassDef(mut inFigaroBase: Ident, mut inFigaroType: ArcStr, mut inProgram: Arc<SCode::Element>, mut inClassName: Ident, mut inClassDef: Arc<SCode::ClassDef>, mut env: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<FigaroClass>>> {
    let mut outFigaroClassList: Arc<metamodelica::List<FigaroClass>> = metamodelica::nil();
    outFigaroClassList = (::match_deref::match_deref! { match &((inFigaroBase.clone(), inFigaroType.clone(), inProgram.clone(), inClassName.clone(), inClassDef.clone(), env.clone())) {
        (fb, ft, program, cn, Deref @ SCode::ClassDef::PARTS { elementLst: el, .. }, e) => {
            fcElementList((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), Some((cn.clone()).clone()), el.clone(), e.clone())?
        },
        (fb, ft, program, cn, Deref @ SCode::ClassDef::DERIVED { modifications: m, typeSpec: ts, .. }, e) => {
            let mut p: Path;
            let mut tn: ArcStr = arcstr::literal!("");
            p = AbsynUtil::typeSpecPath(ts.clone())?;
            let true = (fb.clone() == getLastIdent(p.clone())?) else { bail!("pattern mismatch") };
            tn = (fcMod1(m.clone())?).clone();
            fcAddFigaroClass((ft.clone()).clone(), program.clone(), (cn.clone()).clone(), (tn.clone()).clone(), e.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFigaroClassList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fcElementList(mut inFigaroBase: Ident, mut inFigaroType: ArcStr, mut inProgram: Arc<SCode::Element>, mut inClassName: Option<ArcStr>, mut inElementList: Arc<metamodelica::List<Arc<SCode::Element>>>, mut env: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<FigaroClass>>> {
    let mut outFigaroClassList: Arc<metamodelica::List<FigaroClass>> = metamodelica::nil();
    outFigaroClassList = 'mc: {
        let __mc_input = (inFigaroBase.clone(), inFigaroType.clone(), inProgram.clone(), inClassName.clone(), inElementList.clone(), env.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, Deref @ metamodelica::List::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::List::Cons { head: first, tail: rest }, e) => {
                    let mut rf: Arc<metamodelica::List<FigaroClass>> = metamodelica::nil();
                    let mut rr: Arc<metamodelica::List<FigaroClass>> = metamodelica::nil();
                    rf = fcElement((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), cn.clone(), first.clone(), e.clone())?;
                    rr = fcElementList((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), cn.clone(), rest.clone(), e.clone())?;
                    Ok(listAppend(rf.clone(), rr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fb, ft, program, cn, Deref @ metamodelica::List::Cons { head: _, tail: rest }, e) => {
                    Ok(fcElementList((fb.clone()).clone(), (ft.clone()).clone(), program.clone(), cn.clone(), rest.clone(), e.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFigaroClassList)
}

fn fcMod1(mut inMod: Arc<SCode::Mod>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr = arcstr::literal!("");
    outTypeName = ((::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { subModLst: sml, .. } => {
            fcSubModList(sml.clone())?
        },
        Deref @ SCode::Mod::NOMOD => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outTypeName)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn fcSubModList(mut inSubModList: Arc<metamodelica::List<Arc<SCode::SubMod>>>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr = arcstr::literal!("");
    outTypeName = ('mc: {
        let __mc_input = inSubModList.clone();
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
                Deref @ metamodelica::List::Cons { head: first, tail: _ } => {
                    Ok(fcSubMod(first.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(fcSubModList(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outTypeName)
}

fn fcSubMod(mut inSubMod: Arc<SCode::SubMod>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr = arcstr::literal!("");
    outTypeName = ((::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { r#mod: m, ident: n } => {
            let true = (n.clone() == literal!("fullClassName")) else { bail!("pattern mismatch") };
            fcMod2(m.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outTypeName)
}

fn fcMod2(mut inMod: Arc<SCode::Mod>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr = arcstr::literal!("");
    outTypeName = ((::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { binding: None, .. } => {
            literal!("")
        },
        Deref @ SCode::Mod::MOD { binding: Some(e), .. } => {
            fcExp(e.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outTypeName)
}

fn fcExp(mut inExp: Arc<Absyn::Exp>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr = arcstr::literal!("");
    outTypeName = ((::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::STRING { value: tn } => {
            tn.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outTypeName)
}

/* Finds declarations and checks whether the type matches any of the Figaro classes.
If that is the case, then those objects are collected. */
fn foElement(mut inFigaroClassList: Arc<metamodelica::List<FigaroClass>>, mut inElement: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<FigaroObject>>> {
    let mut outFigaroObjectList: Arc<metamodelica::List<FigaroObject>> = metamodelica::nil();
    outFigaroObjectList = (::match_deref::match_deref! { match &((inFigaroClassList.clone(), inElement.clone())) {
        (fcl, Deref @ SCode::Element::CLASS { classDef: cd, .. }) => {
            foClassDef(fcl.clone(), cd.clone())?
        },
        (fcl, Deref @ SCode::Element::COMPONENT { modifications: m, typeSpec: ts, name: n, .. }) => {
            let mut p: Path;
            let mut tn: ArcStr = arcstr::literal!("");
            let mut c: ArcStr = arcstr::literal!("");
            let mut tmp: ArcStr = arcstr::literal!("");
            let mut fo: FigaroObject;
            p = AbsynUtil::typeSpecPath(ts.clone())?;
            tmp = (foMod1(m.clone(), (literal!("fullClassName")).clone())?).clone();
            tn = (if (tmp.clone() == literal!("")) {findFigaroTypeName(p.clone(), fcl.clone())?} else {tmp.clone()}).clone();
            c = (foMod1(m.clone(), (literal!("codeInstanceFigaro")).clone())?).clone();
            fo = FigaroObject { objectName: (n.clone()).clone(), typeName: (tn.clone()).clone(), figaroCode: (c.clone()).clone() };
            list![fo.clone()]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFigaroObjectList)
}

fn foClassDef(mut inFigaroClassList: Arc<metamodelica::List<FigaroClass>>, mut inClassDef: Arc<SCode::ClassDef>) -> Result<Arc<metamodelica::List<FigaroObject>>> {
    let mut outFigaroObjectList: Arc<metamodelica::List<FigaroObject>> = metamodelica::nil();
    outFigaroObjectList = (::match_deref::match_deref! { match &((inFigaroClassList.clone(), inClassDef.clone())) {
        (fcl, Deref @ SCode::ClassDef::PARTS { elementLst: el, .. }) => {
            foElementList(fcl.clone(), el.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outFigaroObjectList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn foElementList(mut inFigaroClassList: Arc<metamodelica::List<FigaroClass>>, mut inElementList: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<metamodelica::List<FigaroObject>>> {
    let mut outFigaroObjectList: Arc<metamodelica::List<FigaroObject>> = metamodelica::nil();
    outFigaroObjectList = 'mc: {
        let __mc_input = (inFigaroClassList.clone(), inElementList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fcl, Deref @ metamodelica::List::Cons { head: first, tail: rest }) => {
                    let mut rf: Arc<metamodelica::List<FigaroObject>> = metamodelica::nil();
                    let mut rr: Arc<metamodelica::List<FigaroObject>> = metamodelica::nil();
                    rf = foElement(fcl.clone(), first.clone())?;
                    rr = foElementList(fcl.clone(), rest.clone())?;
                    Ok(listAppend(rf.clone(), rr.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (fcl, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    Ok(foElementList(fcl.clone(), rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outFigaroObjectList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn findFigaroTypeName(mut inClassPath: Path, mut inFigaroClassList: Arc<metamodelica::List<FigaroClass>>) -> Result<ArcStr> {
    let mut outTypeName: ArcStr = arcstr::literal!("");
    outTypeName = ('mc: {
        let __mc_input = (inClassPath.clone(), inFigaroClassList.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, Deref @ metamodelica::List::Cons { head: first, tail: _ }) => {
                    let mut tn: ArcStr = arcstr::literal!("");
                    tn = (getFigaroTypeName(p.clone(), first.clone())?).clone();
                    Ok(tn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p, Deref @ metamodelica::List::Cons { head: _, tail: rest }) => {
                    let mut tn: ArcStr = arcstr::literal!("");
                    tn = (findFigaroTypeName(p.clone(), rest.clone())?).clone();
                    Ok(tn.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outTypeName)
}

fn getFigaroTypeName(mut inClassPath: Path, mut inFigaroClass: FigaroClass) -> Result<ArcStr> {
    let mut outTypeName: ArcStr = arcstr::literal!("");
    outTypeName = ((::match_deref::match_deref! { match &((inClassPath.clone(), inFigaroClass.clone())) {
        (p, FigaroClass { typeName: tn, className: cn }) => {
            let true = (getLastIdent(p.clone())? == cn.clone()) else { bail!("pattern mismatch") };
            tn.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outTypeName)
}

fn foMod1(mut inMod: Arc<SCode::Mod>, mut name: ArcStr) -> Result<ArcStr> {
    let mut outCode: ArcStr = arcstr::literal!("");
    outCode = ((::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { subModLst: sml, .. } => {
            foSubModList(sml.clone(), (name.clone()).clone())?
        },
        Deref @ SCode::Mod::NOMOD => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outCode)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn foSubModList(mut inSubModList: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut name: ArcStr) -> Result<ArcStr> {
    let mut outCode: ArcStr = arcstr::literal!("");
    outCode = ('mc: {
        let __mc_input = inSubModList.clone();
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
                Deref @ metamodelica::List::Cons { head: first, tail: _ } => {
                    Ok(foSubMod(first.clone(), (name.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(foSubModList(rest.clone(), (name.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outCode)
}

fn foSubMod(mut inSubMod: Arc<SCode::SubMod>, mut name: ArcStr) -> Result<ArcStr> {
    let mut outCode: ArcStr = arcstr::literal!("");
    outCode = ((::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ SCode::SubMod { r#mod: m, ident: n } => {
            let true = (n.clone() == name.clone()) else { bail!("pattern mismatch") };
            foMod2(m.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outCode)
}

fn foMod2(mut inMod: Arc<SCode::Mod>) -> Result<ArcStr> {
    let mut outCode: ArcStr = arcstr::literal!("");
    outCode = ((::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ SCode::Mod::MOD { binding: None, .. } => {
            literal!("")
        },
        Deref @ SCode::Mod::MOD { binding: Some(e), .. } => {
            foExp(e.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outCode)
}

fn foExp(mut inExp: Arc<Absyn::Exp>) -> Result<ArcStr> {
    let mut outCode: ArcStr = arcstr::literal!("");
    outCode = ((::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::STRING { value: c } => {
            c.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outCode)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getLastIdent(mut inPath: Path) -> Result<Ident> {
    let mut outIdent: Ident = arcstr::literal!("");
    outIdent = ((::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::QUALIFIED { path: p, .. } => {
            getLastIdent(p.clone())?
        },
        Deref @ Absyn::Path::IDENT { name: n } => {
            n.clone()
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path: p } => {
            getLastIdent(p.clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

fn figaroObjectListToString(mut inFigaroObjectList: Arc<metamodelica::List<FigaroObject>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inFigaroObjectList.clone()) {
        Deref @ metamodelica::List::Nil => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
            let mut rf: ArcStr = arcstr::literal!("");
            let mut rr: ArcStr = arcstr::literal!("");
            rf = (figaroObjectToString(first.clone())?).clone();
            rr = (figaroObjectListToString(rest.clone())?).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*rf.clone()); __mm_s.push_str(&*rr.clone()); ArcStr::from(__mm_s) }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn figaroObjectToString(mut inFigaroObject: FigaroObject) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inFigaroObject.clone() {
        FigaroObject { figaroCode: mut fc, typeName: mut tn, objectName: mut on } => {
            let mut middle: ArcStr = arcstr::literal!("");
            middle = (if (fc.clone() == literal!("")) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*fc.clone()); ArcStr::from(__mm_s) }}).clone();
            { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OBJECT ")); __mm_s.push_str(&*on.clone()); __mm_s.push_str(&*literal!(" IS_A ")); __mm_s.push_str(&*tn.clone()); __mm_s.push_str(&*literal!(";")); __mm_s.push_str(&*middle.clone()); __mm_s.push_str(&*literal!("\n\n")); ArcStr::from(__mm_s) }
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn makeXml(mut workingDir: ArcStr, mut inDatabase: ArcStr, mut inBdfFile: ArcStr, mut inMode: ArcStr, mut inOptions: ArcStr, mut inFigaroFile: ArcStr) -> Result<ArcStr> {
    let mut outXml: ArcStr = arcstr::literal!("");
    let mut xml: ArcStr = arcstr::literal!("");
    let mut newName: ArcStr = arcstr::literal!("");
    let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    xml = (literal!("<REQUESTS>\n  ")).clone();
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("\n\n<LOAD_BDC_FI>\n    <FILE_FI>")); ArcStr::from(__mm_s) }).clone();
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*inDatabase.clone()); __mm_s.push_str(&*literal!("</FILE_FI>\n")); ArcStr::from(__mm_s) }).clone();
    sl = stringListStringChar((inDatabase.clone()).clone());
    newName = (truncateExtension(sl.clone())?).clone();
    if System::regularFileExists(({ let mut __mm_s = String::new(); __mm_s.push_str(&*newName.clone()); __mm_s.push_str(&*literal!(".bdc")); ArcStr::from(__mm_s) }).clone()) {
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("<FILE> ")); __mm_s.push_str(&*newName.clone()); __mm_s.push_str(&*literal!(".bdc</FILE>\n")); ArcStr::from(__mm_s) }).clone();
    }
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("</LOAD_BDC_FI>\n")); ArcStr::from(__mm_s) }).clone();
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("\n\n<LOAD_BDF_FI>\n    <FILE>")); ArcStr::from(__mm_s) }).clone();
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*inBdfFile.clone()); ArcStr::from(__mm_s) }).clone();
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("</FILE>\n</LOAD_BDF_FI>\n")); ArcStr::from(__mm_s) }).clone();
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("<RUN_TREATMENT>\n")); ArcStr::from(__mm_s) }).clone();
    if inMode.clone() == literal!("figaro0") {
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("    <TREATMENT>GENERATE_FIG0</TREATMENT>\n    <FILE>")); ArcStr::from(__mm_s) }).clone();
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*inFigaroFile.clone()); ArcStr::from(__mm_s) }).clone();
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("</FILE>")); ArcStr::from(__mm_s) }).clone();
    } else if inMode.clone() == literal!("fault-tree") {
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("    <TREATMENT>GENERATE_TREE</TREATMENT>\n    <FILE>")); ArcStr::from(__mm_s) }).clone();
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*workingDir.clone()); __mm_s.push_str(&*literal!("/FaultTree.xml")); ArcStr::from(__mm_s) }).clone();
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("</FILE>\n")); ArcStr::from(__mm_s) }).clone();
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("    <FILE_MACRO>fiab_ADD.h</FILE_MACRO>")); ArcStr::from(__mm_s) }).clone();
        xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("\n    <FILE_TREE_OPTIONS>")); __mm_s.push_str(&*inOptions.clone()); __mm_s.push_str(&*literal!("</FILE_TREE_OPTIONS>")); ArcStr::from(__mm_s) }).clone();
    }
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("\n    <RESOLVE_CONST>VRAI</RESOLVE_CONST>\n    <RESOLVE_ATTR>FAUX</RESOLVE_ATTR>\n    <INST_RULE>VRAI</INST_RULE>\n")); ArcStr::from(__mm_s) }).clone();
    xml = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*xml.clone()); __mm_s.push_str(&*literal!("</RUN_TREATMENT>\n</REQUESTS>")); ArcStr::from(__mm_s) }).clone();
    outXml = (xml.clone()).clone();
    Ok(outXml)
}

fn truncateExtension(mut name: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut newName: ArcStr = arcstr::literal!("");
    newName = ((::match_deref::match_deref! { match &(name.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ ".", tail: _ } => {
            literal!("")
        },
        Deref @ metamodelica::List::Cons { head: c, tail: rest } => {
            stringAppend((c.clone()).clone(), (truncateExtension(rest.clone())?).clone())
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(newName)
}

fn callFigaroProcessor(mut inFigaroProcessorFile: ArcStr, mut inArgumentFile: ArcStr) -> () {
    let mut command: ArcStr = arcstr::literal!("");
    command = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("start ")); __mm_s.push_str(&*inFigaroProcessorFile.clone()); __mm_s.push_str(&*literal!(" -testxml ")); __mm_s.push_str(&*inArgumentFile.clone()); ArcStr::from(__mm_s) }).clone();
    System::systemCall((command.clone()).clone(), (literal!("")).clone());
    ()
}

/// An XML token.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    OPENTAG {
        tagName: ArcStr,
    },
    CLOSETAG {
        tagName: ArcStr,
    },
    TEXT {
        text: ArcStr,
    },
}
impl Default for Token {
    fn default() -> Self {
        Self::OPENTAG {
            tagName: Default::default(),
        }
    }
}
pub use self::Token::{OPENTAG,CLOSETAG,TEXT};

fn interpret(mut inString: ArcStr) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringList = (match inString.clone() {
        mut s => {
            let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut sl2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut tl: Arc<metamodelica::List<Token>> = metamodelica::nil();
            let mut tl2: Arc<metamodelica::List<Token>> = metamodelica::nil();
            let mut tl3: Arc<metamodelica::List<Token>> = metamodelica::nil();
            sl = stringListStringChar((s.clone()).clone());
            tl = scan(sl.clone())?;
            tl2 = removeFirstIfText(tl.clone());
            tl3 = removeTokens(tl2.clone())?;
            sl2 = parse(tl3.clone())?;
            sl2.clone()
        },
        _ => {
            bail!("fail")
        },
    });
    Ok(outStringList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn scan(mut inStringList: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<Token>>> {
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    outTokenList = 'mc: {
        let __mc_input = inStringList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: Deref @ metamodelica::List::Cons { head: Deref @ "?", tail: rest } } => {
                    let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    r = scanDeclaration(rest.clone())?;
                    Ok(scan(r.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: Deref @ metamodelica::List::Cons { head: Deref @ "/", tail: rest } } => {
                    let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut t: Token;
                    let mut s: ArcStr = arcstr::literal!("");
                    (r, s) = scanTagName(rest.clone(), (literal!("")).clone())?;
                    t = Token::CLOSETAG { tagName: (s.clone()).clone() };
                    Ok(cons(t.clone(), scan(r.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: rest } => {
                    let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut t: Token;
                    let mut s: ArcStr = arcstr::literal!("");
                    (r, s) = scanTagName(rest.clone(), (literal!("")).clone())?;
                    t = Token::OPENTAG { tagName: (s.clone()).clone() };
                    Ok(cons(t.clone(), scan(r.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                rest => {
                    let mut r: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut t: Token;
                    let mut s: ArcStr = arcstr::literal!("");
                    (r, s) = scanText(rest.clone(), (literal!("")).clone())?;
                    t = Token::TEXT { text: (s.clone()).clone() };
                    Ok(cons(t.clone(), scan(r.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTokenList)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn scanDeclaration(mut inStringList: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringList = 'mc: {
        let __mc_input = inStringList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "?", tail: Deref @ metamodelica::List::Cons { head: Deref @ ">", tail: rest } } => {
                    Ok(rest.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(scanDeclaration(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringList)
}

fn scanTagName(mut inStringList: Arc<metamodelica::List<ArcStr>>, mut inTagName: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr)> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outTagName: ArcStr = arcstr::literal!("");
    (outStringList, outTagName) = 'mc: {
        let __mc_input = inStringList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ ">", tail: rest } => {
                    Ok((rest.clone(), inTagName.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
                    Ok(scanTagName(rest.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inTagName.clone()); __mm_s.push_str(&*first.clone()); ArcStr::from(__mm_s) }).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStringList, outTagName))
}

fn scanText(mut inStringList: Arc<metamodelica::List<ArcStr>>, mut inText: ArcStr) -> Result<(Arc<metamodelica::List<ArcStr>>, ArcStr)> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outText: ArcStr = arcstr::literal!("");
    (outStringList, outText) = 'mc: {
        let __mc_input = inStringList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((metamodelica::nil(), literal!("")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ "<", tail: _ } => {
                    Ok((inStringList.clone(), inText.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
                    Ok(scanText(rest.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inText.clone()); __mm_s.push_str(&*first.clone()); ArcStr::from(__mm_s) }).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outStringList, outText))
}

/* These functions walk over the token sequence from the lexer and throw away tokens that will not
be usable. E. g., if a tag is not known, the tokens associated with it will be thrown away.
The purpose of this step is to return a very simple sequence for the parser to work on. */
// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeTokens(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<Arc<metamodelica::List<Token>>> {
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    outTokenList = 'mc: {
        let __mc_input = inTokenList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
                    let mut r: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let true = (isKnownTag((tn.clone()).clone())) else { bail!("pattern mismatch") };
                    let false = (isInfoTag((tn.clone()).clone())) else { bail!("pattern mismatch") };
                    r = removeFirstIfText(rest.clone());
                    Ok(cons(Token::OPENTAG { tagName: (tn.clone()).clone() }, removeTokens(r.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
                    let mut r: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    let false = (isKnownTag((tn.clone()).clone())) else { bail!("pattern mismatch") };
                    r = removeUnknown(rest.clone(), (tn.clone()).clone())?;
                    Ok(removeTokens(r.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
                    let mut r: Arc<metamodelica::List<Token>> = metamodelica::nil();
                    r = removeFirstIfText(rest.clone());
                    Ok(cons(Token::CLOSETAG { tagName: (tn.clone()).clone() }, removeTokens(r.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
                    Ok(cons(first.clone(), removeTokens(rest.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTokenList)
}

fn removeFirstIfText(mut inTokenList: Arc<metamodelica::List<Token>>) -> Arc<metamodelica::List<Token>> {
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    outTokenList = (::match_deref::match_deref! { match &(inTokenList.clone()) {
        Deref @ metamodelica::List::Cons { head: Token::TEXT { .. }, tail: rest } => {
            rest.clone()
        },
        _ => {
            inTokenList.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTokenList
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn removeUnknown(mut inTokenList: Arc<metamodelica::List<Token>>, mut inTagName: ArcStr) -> Result<Arc<metamodelica::List<Token>>> {
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    outTokenList = 'mc: {
        let __mc_input = inTokenList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
                    let true = (tn.clone() == inTagName.clone()) else { bail!("pattern mismatch") };
                    Ok(removeFirstIfText(rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(removeUnknown(rest.clone(), (inTagName.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTokenList)
}

fn isKnownTag(mut inTagName: ArcStr) -> bool {
    let mut outBoolean: bool = false;
    let mut ktl: Arc<metamodelica::List<ArcStr>> = list![(literal!("ANSWERS")).clone(), (literal!("ANSWER")).clone(), (literal!("ERROR")).clone(), (literal!("LABEL")).clone(), (literal!("CRITICITY")).clone()];
    outBoolean = listMember((inTagName.clone()).clone(), ktl.clone());
    outBoolean
}

fn isInfoTag(mut inTagName: ArcStr) -> bool {
    let mut outBoolean: bool = false;
    let mut itl: Arc<metamodelica::List<ArcStr>> = list![(literal!("LABEL")).clone(), (literal!("CRITICITY")).clone()];
    outBoolean = listMember((inTagName.clone()).clone(), itl.clone());
    outBoolean
}

fn parse(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringList = (::match_deref::match_deref! { match &(inTokenList.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let true = (tn.clone() == literal!("ANSWERS")) else { bail!("pattern mismatch") };
            parseAnswers(rest.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outStringList)
}

fn parseAnswers(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (sl, _) = parseAnswerList(inTokenList.clone())?;
    outStringList = sl.clone();
    Ok(outStringList)
}

fn parseAnswerList(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Token>>)> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (outStringList, outTokenList) = (::match_deref::match_deref! { match &(inTokenList.clone()) {
        Deref @ metamodelica::List::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut sl2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut tl: Arc<metamodelica::List<Token>> = metamodelica::nil();
            let mut tl2: Arc<metamodelica::List<Token>> = metamodelica::nil();
            let true = (tn.clone() == literal!("ANSWER")) else { bail!("pattern mismatch") };
            (sl, tl) = parseAnswer(rest.clone())?;
            (sl2, tl2) = parseAnswerList(tl.clone())?;
            (listAppend(sl.clone(), sl2.clone()), tl2.clone())
        },
        Deref @ metamodelica::List::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
            let true = (tn.clone() == literal!("ANSWERS")) else { bail!("pattern mismatch") };
            (metamodelica::nil(), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStringList, outTokenList))
}

fn parseAnswer(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Token>>)> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (outStringList, outTokenList) = parseErrorList(inTokenList.clone())?;
    Ok((outStringList, outTokenList))
}

fn parseErrorList(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Token>>)> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (outStringList, outTokenList) = (::match_deref::match_deref! { match &(inTokenList.clone()) {
        Deref @ metamodelica::List::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut sl2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            let mut tl: Arc<metamodelica::List<Token>> = metamodelica::nil();
            let mut tl2: Arc<metamodelica::List<Token>> = metamodelica::nil();
            let true = (tn.clone() == literal!("ERROR")) else { bail!("pattern mismatch") };
            (sl, tl) = parseError(rest.clone())?;
            (sl2, tl2) = parseErrorList(tl.clone())?;
            (listAppend(sl.clone(), sl2.clone()), tl2.clone())
        },
        Deref @ metamodelica::List::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
            let true = (tn.clone() == literal!("ANSWER")) else { bail!("pattern mismatch") };
            (metamodelica::nil(), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStringList, outTokenList))
}

fn parseError(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<(Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Token>>)> {
    let mut outStringList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut stl: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut tl: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut sl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    (stl, tl) = parseInfoList(inTokenList.clone())?;
    sl = if (isToBeReported(stl.clone())?) {list![(getMessage(stl.clone())?).clone()]} else {metamodelica::nil()};
    (outStringList, outTokenList) = (sl.clone(), tl.clone());
    Ok((outStringList, outTokenList))
}

fn parseInfoList(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<(Arc<metamodelica::List<(ArcStr, ArcStr)>>, Arc<metamodelica::List<Token>>)> {
    let mut outStringTupleList: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (outStringTupleList, outTokenList) = (::match_deref::match_deref! { match &(inTokenList.clone()) {
        Deref @ metamodelica::List::Cons { head: Token::OPENTAG { tagName: tn }, tail: rest } => {
            let mut s: ArcStr = arcstr::literal!("");
            let mut stl: Arc<metamodelica::List<(ArcStr, ArcStr)>> = metamodelica::nil();
            let mut tl: Arc<metamodelica::List<Token>> = metamodelica::nil();
            let mut tl2: Arc<metamodelica::List<Token>> = metamodelica::nil();
            (s, tl) = parseInfo(rest.clone())?;
            (stl, tl2) = parseInfoList(tl.clone())?;
            (cons((tn.clone(), s.clone()), stl.clone()), tl2.clone())
        },
        Deref @ metamodelica::List::Cons { head: Token::CLOSETAG { tagName: tn }, tail: rest } => {
            let true = (tn.clone() == literal!("ERROR")) else { bail!("pattern mismatch") };
            (metamodelica::nil(), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outStringTupleList, outTokenList))
}

fn parseInfo(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<(ArcStr, Arc<metamodelica::List<Token>>)> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut outTokenList: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (outString, outTokenList) = (::match_deref::match_deref! { match &(inTokenList.clone()) {
        Deref @ metamodelica::List::Cons { head: Token::TEXT { text: s }, tail: Deref @ metamodelica::List::Cons { head: _, tail: rest } } => {
            (s.clone(), rest.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outString, outTokenList))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isToBeReported(mut inStringTupleList: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    let mut errorsToReport: Arc<metamodelica::List<ArcStr>> = list![(literal!("FATAL")).clone()];
    outBoolean = 'mc: {
        let __mc_input = inStringTupleList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (k, v), tail: _ } => {
                    let true = (k.clone() == literal!("CRITICITY")) else { bail!("pattern mismatch") };
                    Ok(listMember((v.clone()).clone(), errorsToReport.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(isToBeReported(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getMessage(mut inStringTupleList: Arc<metamodelica::List<(ArcStr, ArcStr)>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = inStringTupleList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (k, v), tail: _ } => {
                    let true = (k.clone() == literal!("LABEL")) else { bail!("pattern mismatch") };
                    Ok(v.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getMessage(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

fn reportErrors(mut inStringList: Arc<metamodelica::List<ArcStr>>) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &(inStringList.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
            Error::addMessage(Error::FIGARO_ERROR.clone(), list![(first.clone()).clone()])?;
            reportErrors(rest.clone())?;
            true
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

/* Debug */
fn printFigaroClassList(mut inFigaroClassList: Arc<metamodelica::List<FigaroClass>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inFigaroClassList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
                    printFigaroClass(first.clone())?;
                    printFigaroClassList(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    printFigaroClassList(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printFigaroClass(mut inFigaroClass: FigaroClass) -> Result<()> {
    let _ = (match inFigaroClass.clone() {
        FigaroClass { typeName: mut tn, className: mut cn } => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*cn.clone()); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*tn.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

fn printFigaroObjectList(mut inFigaroObjectList: Arc<metamodelica::List<FigaroObject>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inFigaroObjectList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
                    println!("{}", (figaroObjectToString(first.clone())?).clone());
                    printFigaroObjectList(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    printFigaroObjectList(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printTokenList(mut inTokenList: Arc<metamodelica::List<Token>>) -> Result<()> {
    let _ = 'mc: {
        let __mc_input = inTokenList.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: first, tail: rest } => {
                    printToken(first.clone())?;
                    println!("{}", (literal!("\n")).clone());
                    printTokenList(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    printTokenList(rest.clone())?;
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(())
}

fn printToken(mut inToken: Token) -> Result<()> {
    let _ = (match inToken.clone() {
        Token::OPENTAG { tagName: mut s } => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("OPEN: ")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone());
            ()
        },
        Token::CLOSETAG { tagName: mut s } => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("CLOSE: ")); __mm_s.push_str(&*s.clone()); ArcStr::from(__mm_s) }).clone());
            ()
        },
        Token::TEXT { text: mut s } => {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(())
}

