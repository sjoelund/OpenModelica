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

use crate::Interactive::Access;
use crate::Interactive;
use crate::NFApi;
use crate::StaticScript;
use crate::SymbolTable;
use openmodelica_ast::Absyn;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::ConnectionGraph;
use openmodelica_frontend::DAEUtil;
use openmodelica_frontend::ExpressionSimplify;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend::FCore;
use openmodelica_frontend::FGraph;
use openmodelica_frontend::InnerOuter;
use openmodelica_frontend::Inst;
use openmodelica_frontend::InstTypes;
use openmodelica_frontend::Lookup;
use openmodelica_frontend::Mod;
use openmodelica_frontend::Parser;
use openmodelica_frontend::UnitAbsyn;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::ErrorExt;
use openmodelica_util::Flags;
use openmodelica_util::FlagsUtil;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

//public imports
// protected imports
pub type GraphicEnvCache = Interactive::GraphicEnvCache;

pub type AnnotationType = Interactive::AnnotationType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Visibility {
    PUBLIC = 1,
    PROTECTED = 2,
    ANY = 3,
}
impl PartialOrd for Visibility {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Visibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn getExtendsElementspecInClass(mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementSpec>>>> {
    let mut outAbsynElementSpecLst: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
    outAbsynElementSpecLst = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. } => {
                    let mut ext: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    ext = getExtendsElementspecInClassparts(parts.clone())?;
                    Ok(ext.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. } => {
                    let mut ext: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    ext = getExtendsElementspecInClassparts(parts.clone())?;
                    Ok(ext.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { arguments: eltArg, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: tp, arrayDim: _ }, .. }, .. } => {
                    Ok(list![Arc::new(Absyn::ElementSpec::EXTENDS { path: tp.clone(), elementArg: eltArg.clone(), annotationOpt: None })])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynElementSpecLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getExtendsElementspecInClassparts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementSpec>>>> {
    let mut outAbsynElementSpecLst: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
    outAbsynElementSpecLst = 'mc: {
        let __mc_input = inAbsynClassPartLst.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest } => {
                    let mut lst1: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    lst1 = getExtendsElementspecInClassparts(rest.clone())?;
                    lst2 = getExtendsElementspecInElementitems(elts.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest } => {
                    let mut lst1: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    let mut lst2: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    lst1 = getExtendsElementspecInClassparts(rest.clone())?;
                    lst2 = getExtendsElementspecInElementitems(elts.clone())?;
                    res = listAppend(lst1.clone(), lst2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    res = getExtendsElementspecInClassparts(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynElementSpecLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getExtendsElementspecInElementitems(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementSpec>>>> {
    let mut outAbsynElementSpecLst: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
    outAbsynElementSpecLst = 'mc: {
        let __mc_input = inAbsynElementItemLst.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: el }, tail: rest } => {
                    let mut elt: Arc<Absyn::ElementSpec>;
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    elt = getExtendsElementspecInElement(el.clone())?;
                    res = getExtendsElementspecInElementitems(rest.clone())?;
                    Ok(cons(elt.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut res: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    res = getExtendsElementspecInElementitems(rest.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynElementSpecLst)
}

fn getExtendsElementspecInElement(mut inElement: Arc<Absyn::Element>) -> Result<Arc<Absyn::ElementSpec>> {
    let mut outElementSpec: Arc<Absyn::ElementSpec>;
    outElementSpec = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: ext @ Deref @ Absyn::ElementSpec::EXTENDS { .. }, .. } => {
            ext.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outElementSpec)
}

pub fn removeElementModifiers(mut path: Arc<Absyn::Path>, mut inComponentName: ArcStr, mut inProgram: Absyn::Program, mut keepRedeclares: bool) -> (Absyn::Program, bool) {
    let mut outProgram: Absyn::Program;
    let mut outResult: bool = false;
    let mut within_: Absyn::Within = Absyn::Within::TOP;
    let mut cls: Arc<Absyn::Class>;
    match '__try0: {
        within_ = unwrap_break_err!(buildWithin(path.clone()), '__try0);
        cls = unwrap_break_err!(getPathedClassInProgram(path.clone(), inProgram.clone(), false, false), '__try0);
        cls = unwrap_break_err!(clearComponentModifiersInClass(cls.clone(), (inComponentName.clone()).clone(), keepRedeclares.clone()), '__try0);
        outProgram = unwrap_break_err!(updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, inProgram.clone(), false), '__try0);
        outResult = true;
        Ok::<_, anyhow::Error>((outProgram.clone(), outResult.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outProgram = __try0_o0;
            outResult = __try0_o1;
        }
        Err(_) => {
            outProgram = inProgram.clone();
            outResult = false;
        }
    }
    (outProgram, outResult)
}

pub fn clearComponentModifiersInClass(mut inClass: Arc<Absyn::Class>, mut inComponentName: ArcStr, mut keepRedeclares: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::traverseClassComponents(inClass.clone(), Arc::new({ let __pe_b2 = (inComponentName.clone()).clone(); let __pe_b3 = keepRedeclares.clone(); move |__pe_a0, __pe_a1| clearComponentModifiersInCompitems(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), false)?) {
        (__pa0, true) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outClass = __pa0.clone();
    Ok(outClass)
}

fn clearComponentModifiersInCompitems(mut inComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inFound: bool, mut inComponentName: ArcStr, mut keepRedeclares: bool) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, bool, bool)> {
    let mut outComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut outFound: bool = false;
    let mut outContinue: bool = false;
    let mut item: Arc<Absyn::ComponentItem>;
    let mut rest_items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = inComponents.clone();
    let mut comp: Absyn::Component;
    let mut args_old: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut args_new: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut eqmod_old: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut eqmod_new: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    while !(rest_items.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_items.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        item = __pa0.clone();
        rest_items = __pa1.clone();
        if AbsynUtil::componentName(item.clone())? == inComponentName.clone() {
            let _ = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ComponentItem { component: comp @ Absyn::Component { .. }, .. } => {
            let mut comp = (*comp).clone();
            comp.modification = if (!(keepRedeclares.clone())) {None} else {stripModifiersKeepRedeclares(comp.modification.clone())?};
            assign_field!(item.component = comp.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            outComponents = List::append_reverse(outComponents.clone(), cons(item.clone(), rest_items.clone()));
            outFound = true;
            outContinue = false;
            return Ok((outComponents, outFound, outContinue));
        }
        outComponents = cons(item.clone(), outComponents.clone());
    }
    outComponents = inComponents.clone();
    outFound = false;
    outContinue = true;
    Ok((outComponents, outFound, outContinue))
}

fn stripModifiersKeepRedeclares(mut inMod: Option<Arc<Absyn::Modification>>) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>> = None;
    outMod = (::match_deref::match_deref! { match &(inMod.clone()) {
        None => {
            None
        },
        Some(Deref @ Absyn::Modification { elementArgLst: ea, eqMod: _ }) => {
            let mut m: Arc<Absyn::Modification>;
            let mut ea = (*ea).clone();
            ea = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut e in (ea.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            m = Arc::new(Absyn::Modification { elementArgLst: ea.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) });
            Some(m.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outMod)
}

pub fn setElementModifier(mut inClass: Arc<Absyn::Path>, mut inElementName: Arc<Absyn::Path>, mut inMod: Arc<Absyn::Modification>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut outResult: bool = false;
    let mut within_: Absyn::Within = Absyn::Within::TOP;
    let mut cls: Arc<Absyn::Class>;
    match '__try0: {
        cls = unwrap_break_err!(getPathedClassInProgram(inClass.clone(), program.clone(), false, false), '__try0);
        (cls, outResult) = unwrap_break_err!(setElementSubmodifierInClass(cls.clone(), inElementName.clone(), inMod.clone()), '__try0);
        within_ = unwrap_break_err!(buildWithin(inClass.clone()), '__try0);
        program = unwrap_break_err!(updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, program.clone(), false), '__try0);
        Ok::<_, anyhow::Error>((cls.clone(), outResult.clone(), program.clone(), within_.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            cls = __try0_o0;
            outResult = __try0_o1;
            program = __try0_o2;
            within_ = __try0_o3;
        }
        Err(_) => {
            outResult = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, outResult)
}

pub fn setExtendsModifier(mut className: Arc<Absyn::Path>, mut extendsName: Arc<Absyn::Path>, mut elementName: Arc<Absyn::Path>, mut r#mod: Arc<Absyn::Modification>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut result: bool = false;
    let mut within_: Absyn::Within = Absyn::Within::TOP;
    let mut cls: Arc<Absyn::Class>;
    let mut env: GraphicEnvCache;
    match '__try0: {
        cls = unwrap_break_err!(getPathedClassInProgram(className.clone(), program.clone(), false, false), '__try0);
        env = unwrap_break_err!(Interactive::getClassEnv(program.clone(), className.clone()), '__try0);
        (cls, result) = unwrap_break_err!(setExtendsSubmodifierInClass(cls.clone(), extendsName.clone(), elementName.clone(), r#mod.clone(), env.clone()), '__try0);
        within_ = unwrap_break_err!(buildWithin(className.clone()), '__try0);
        program = unwrap_break_err!(updateProgram(Absyn::Program { classes: list![cls.clone()], within_: within_.clone() }, program.clone(), false), '__try0);
        Ok::<_, anyhow::Error>((cls.clone(), env.clone(), program.clone(), result.clone(), within_.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            cls = __try0_o0;
            env = __try0_o1;
            program = __try0_o2;
            result = __try0_o3;
            within_ = __try0_o4;
        }
        Err(_) => {
            result = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, result)
}

fn setElementSubmodifierInClass(mut inClass: Arc<Absyn::Class>, mut inElementName: Arc<Absyn::Path>, mut inMod: Arc<Absyn::Modification>) -> Result<(Arc<Absyn::Class>, bool)> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    let mut found: bool = false;
    (outClass, found) = AbsynUtil::traverseClassElements(inClass.clone(), Arc::new({ let __pe_b2 = inElementName.clone(); let __pe_b3 = inMod.clone(); move |__pe_a0, __pe_a1| setSubmodifierInElement(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }), false)?;
    Ok((outClass, found))
}

fn setExtendsSubmodifierInClass(mut cls: Arc<Absyn::Class>, mut extendsPath: Arc<Absyn::Path>, mut elementName: Arc<Absyn::Path>, mut r#mod: Arc<Absyn::Modification>, mut env: GraphicEnvCache) -> Result<(Arc<Absyn::Class>, bool)> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut found: bool = false;
    (cls, found) = AbsynUtil::traverseClassElements(cls.clone(), Arc::new({ let __pe_b1 = extendsPath.clone(); let __pe_b2 = elementName.clone(); let __pe_b3 = r#mod.clone(); let __pe_b4 = env.clone(); move |__pe_a0, __pe_a5| setExtendsSubmodifierInElement(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_b4.clone(), __pe_a5) }), false)?;
    Ok((cls, found))
}

fn setSubmodifierInElement(mut element: Arc<Absyn::Element>, mut found: bool, mut elementName: Arc<Absyn::Path>, mut r#mod: Arc<Absyn::Modification>) -> Result<(Arc<Absyn::Element>, bool, bool)> {
    let mut element: Arc<Absyn::Element> = element;
    let mut found: bool = found;
    let mut outContinue: bool = true;
    if AbsynUtil::isElementNamed((AbsynUtil::pathFirstIdent(elementName.clone())?).clone(), element.clone()) {
        match '__try0: {
            let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = unwrap_break_err!(setSubmodifierInElementSpec(elementName.clone(), r#mod.clone(), var_field!((*element).specification, Absyn::Element::ELEMENT).clone()), '__try0));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            found = true;
            outContinue = false;
            Ok::<_, anyhow::Error>((found.clone(), outContinue.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                found = __try0_o0;
                outContinue = __try0_o1;
            }
            Err(_) => {
                bail!("try/else: outputs not set in else branch");
            }
        }
    }
    Ok((element, found, outContinue))
}

fn setExtendsSubmodifierInElement(mut element: Arc<Absyn::Element>, mut extendsPath: Arc<Absyn::Path>, mut elementName: Arc<Absyn::Path>, mut r#mod: Arc<Absyn::Modification>, mut env: GraphicEnvCache, mut found: bool) -> Result<(Arc<Absyn::Element>, bool, bool)> {
    let mut element: Arc<Absyn::Element> = element;
    let mut found: bool = found;
    let mut outContinue: bool = true;
    let mut ext_spec: Arc<Absyn::ElementSpec>;
    let mut full_path: Arc<Absyn::Path>;
    let mut eargs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut opt_mod: Option<Arc<Absyn::Modification>> = None;
    found = 'mc: {
        let __mc_input = element.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Element::ELEMENT { specification: ext_spec @ Deref @ Absyn::ElementSpec::EXTENDS { elementArg: eargs, .. }, .. } => {
                    let mut ext_spec = (*ext_spec).clone();
                    let mut opt_mod: Option<Arc<Absyn::Modification>> = opt_mod.clone();
                    let mut element: Arc<Absyn::Element> = element.clone();
                    let mut full_path: Arc<Absyn::Path>;
                    (_, full_path) = Interactive::mkFullyQual(env.clone(), var_field!((**ext_spec).path, Absyn::ElementSpec::EXTENDS).clone(), false)?;
                    let true = (AbsynUtil::pathEqual(extendsPath.clone(), full_path.clone())) else { bail!("pattern mismatch") };
                    if AbsynUtil::pathFirstIdent(elementName.clone())? == literal!("_") {
                        opt_mod = propagateMod(elementName.clone(), r#mod.clone(), Some(Arc::new(Absyn::Modification { elementArgLst: eargs.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })))?;
                    } else {
                        opt_mod = propagateMod(AbsynUtil::prefixPath((literal!("dummy")).clone(), elementName.clone()), r#mod.clone(), Some(Arc::new(Absyn::Modification { elementArgLst: eargs.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })))?;
                    }
                    let __owned_variant_elementArg_0 = (::match_deref::match_deref! { match &(opt_mod.clone()) {
        Some(Deref @ Absyn::Modification { elementArgLst: eargs, .. }) => eargs.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    if let Absyn::ElementSpec::EXTENDS { elementArg, .. } = &mut ext_spec {
                        *elementArg = __owned_variant_elementArg_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ElementSpec::EXTENDS"); }
                    assign_variant_field!(element => Absyn::Element::ELEMENT; specification = ext_spec.clone());
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    outContinue = !(found.clone());
    Ok((element, found, outContinue))
}

fn setSubmodifierInElementSpec(mut elementName: Arc<Absyn::Path>, mut r#mod: Arc<Absyn::Modification>, mut elSpec: Arc<Absyn::ElementSpec>) -> Result<Arc<Absyn::ElementSpec>> {
    let mut elSpec: Arc<Absyn::ElementSpec> = elSpec;
    let () = (::match_deref::match_deref! { match &(elSpec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            assign_variant_field!(elSpec => Absyn::ElementSpec::CLASSDEF; class_ = setSubmodifierInClass(elementName.clone(), var_field!((*elSpec).class_, Absyn::ElementSpec::CLASSDEF).clone(), r#mod.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            assign_variant_field!(elSpec => Absyn::ElementSpec::COMPONENTS; components = setComponentSubmodifierInCompitems(var_field!((*elSpec).components, Absyn::ElementSpec::COMPONENTS).clone(), false, elementName.clone(), r#mod.clone())?.0);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(elSpec)
}

fn setSubmodifierInClass(mut inElementName: Arc<Absyn::Path>, mut inClass: Arc<Absyn::Class>, mut inMod: Arc<Absyn::Modification>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    let mut cls: Arc<Absyn::Class> = inClass.clone();
    let mut optMod: Option<Arc<Absyn::Modification>> = None;
    let mut r#mod: Arc<Absyn::Modification>;
    let mut body: Arc<Absyn::ClassDef>;
    outClass = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { .. } => {
            body = cls.body.clone();
            body = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(propagateMod(inElementName.clone(), inMod.clone(), Some(Arc::new(Absyn::Modification { elementArgLst: var_field!((*body).arguments, Absyn::ClassDef::DERIVED).clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })))?) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            r#mod = __pa0.clone();
            assign_variant_field!(body => Absyn::ClassDef::DERIVED; arguments = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::Modification { .. } => r#mod.elementArgLst.clone(),
        _ => bail!("match: no arm matched"),
    } }));
            body.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
            assign_field!(cls.body = body.clone());
            cls.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

pub fn setComponentSubmodifierInCompitems(mut inComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inFound: bool, mut inComponentName: Arc<Absyn::Path>, mut inMod: Arc<Absyn::Modification>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, bool, bool)> {
    let mut outComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut outFound: bool = false;
    let mut outContinue: bool = false;
    let mut item: Arc<Absyn::ComponentItem>;
    let mut rest_items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = inComponents.clone();
    let mut comp: Absyn::Component;
    let mut args_old: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut args_new: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut eqmod_old: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut eqmod_new: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut comp_id: ArcStr = arcstr::literal!("");
    comp_id = (AbsynUtil::pathFirstIdent(inComponentName.clone())?).clone();
    while !(rest_items.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_items.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        item = __pa0.clone();
        rest_items = __pa1.clone();
        if AbsynUtil::componentName(item.clone())? == comp_id.clone() {
            let _ = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ComponentItem { component: comp @ Absyn::Component { .. }, .. } => {
            let mut comp = (*comp).clone();
            comp.modification = propagateMod(inComponentName.clone(), inMod.clone(), comp.modification.clone())?;
            assign_field!(item.component = comp.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            outComponents = List::append_reverse(outComponents.clone(), cons(item.clone(), rest_items.clone()));
            outFound = true;
            outContinue = false;
            return Ok((outComponents, outFound, outContinue));
        }
        outComponents = cons(item.clone(), outComponents.clone());
    }
    outComponents = inComponents.clone();
    outFound = false;
    outContinue = true;
    Ok((outComponents, outFound, outContinue))
}

pub fn propagateMod(mut inComponentName: Arc<Absyn::Path>, mut inNewMod: Arc<Absyn::Modification>, mut inOldMod: Option<Arc<Absyn::Modification>>) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>> = None;
    let mut new_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut old_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut new_eqmod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut old_eqmod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let mut r#mod: Arc<Absyn::Modification>;
    if isSome(inOldMod.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inOldMod.clone()) {
            Some(Deref @ Absyn::Modification { eqMod: __pa0, elementArgLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        old_eqmod = __pa0.clone();
        old_args = __pa1.clone();
    } else {
        old_args = metamodelica::nil();
        old_eqmod = Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD);
    }
    if AbsynUtil::pathIsIdent(inComponentName.clone()) {
        let (__pa2, __pa3) = ::match_deref::match_deref! { match &(inNewMod.clone()) {
            Deref @ Absyn::Modification { eqMod: __pa2, elementArgLst: __pa3 } => (__pa2.clone(), __pa3.clone()),
            _ => bail!("pattern mismatch"),
        } };
        new_eqmod = __pa2.clone();
        new_args = __pa3.clone();
        if new_eqmod.clone() == Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) && !(new_args.clone().is_empty()) {
            new_eqmod = old_eqmod.clone();
        }
        if !(AbsynUtil::isEmptyMod(inNewMod.clone())) {
            new_args = mergeElementArgs(old_args.clone(), new_args.clone())?;
        }
        r#mod = Arc::new(Absyn::Modification { elementArgLst: new_args.clone(), eqMod: new_eqmod.clone() });
    } else {
        new_args = propagateMod2(inComponentName.clone(), old_args.clone(), inNewMod.clone())?;
        r#mod = Arc::new(Absyn::Modification { elementArgLst: new_args.clone(), eqMod: old_eqmod.clone() });
    }
    outMod = if (AbsynUtil::isEmptyMod(r#mod.clone())) {None} else {Some(r#mod.clone())};
    Ok(outMod)
}

fn mergeElementArgs(mut inOldArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inNewArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = inOldArgs.clone();
    let mut found: bool = false;
    if inOldArgs.clone().is_empty() {
        outArgs = removeEmptySubMods(inNewArgs.clone());
    } else if inNewArgs.clone().is_empty() {
        outArgs = inOldArgs.clone();
    } else {
        for mut narg in &*inNewArgs.clone() {
            let mut narg = narg.clone();
            (outArgs, found) = List::replaceOnTrue(narg.clone(), outArgs.clone(), Arc::new({ let __pe_b1 = narg.clone(); move |__pe_a0| Ok(AbsynUtil::elementArgEqualName(__pe_a0, __pe_b1.clone())) }))?;
            if !(found.clone()) {
                outArgs = List::appendElt(narg.clone(), outArgs.clone());
            }
        }
        outArgs = removeEmptySubMods(outArgs.clone());
    }
    Ok(outArgs)
}

fn removeEmptySubMods(mut subMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementArg>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut r#mod: Arc<Absyn::Modification>;
    for mut m in &*subMods.clone() {
        let mut m = m.clone();
        let () = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(r#mod), .. } => {
            let mut r#mod = (*r#mod).clone();
            assign_field!(r#mod.elementArgLst = removeEmptySubMods(r#mod.elementArgLst.clone()));
            assign_variant_field!(m => Absyn::ElementArg::MODIFICATION; modification = if (AbsynUtil::isEmptyMod(r#mod.clone())) {None} else {Some(r#mod.clone())});
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if !(AbsynUtil::isEmptySubMod(m.clone())) {
            outSubMods = cons(m.clone(), outSubMods.clone());
        }
    }
    outSubMods = Dangerous::listReverseInPlace(outSubMods.clone());
    outSubMods
}

fn propagateMod2(mut inComponentName: Arc<Absyn::Path>, mut inSubMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inNewMod: Arc<Absyn::Modification>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outSubMods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut submod: Arc<Absyn::ElementArg>;
    let mut rest_submods: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = inSubMods.clone();
    let mut new_mod: Arc<Absyn::Modification>;
    let mut comp_name: Arc<Absyn::Path>;
    let mut comp_rest: Arc<Absyn::Path>;
    while !(rest_submods.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_submods.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        submod = __pa0.clone();
        rest_submods = __pa1.clone();
        comp_name = AbsynUtil::pathRest(inComponentName.clone())?;
        comp_rest = comp_name.clone();
        while true {
            if AbsynUtil::pathEqual(comp_name.clone(), AbsynUtil::elementArgName(submod.clone())?) {
                let _ = (::match_deref::match_deref! { match &(submod.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            if !(AbsynUtil::pathIsIdent(comp_name.clone())) {
                comp_name = AbsynUtil::pathPrefix(comp_name.clone())?;
                comp_rest = AbsynUtil::removePrefix(comp_name.clone(), comp_rest.clone())?;
            }
            assign_variant_field!(submod => Absyn::ElementArg::MODIFICATION; modification = propagateMod(comp_rest.clone(), inNewMod.clone(), var_field!((*submod).modification, Absyn::ElementArg::MODIFICATION).clone())?);
            if isSome(var_field!((*submod).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                rest_submods = cons(submod.clone(), rest_submods.clone());
            }
            ()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            assign_variant_field!(submod => Absyn::ElementArg::REDECLARATION; elementSpec = setSubmodifierInElementSpec(comp_rest.clone(), inNewMod.clone(), var_field!((*submod).elementSpec, Absyn::ElementArg::REDECLARATION).clone())?);
            rest_submods = cons(submod.clone(), rest_submods.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                outSubMods = List::append_reverse(outSubMods.clone(), rest_submods.clone());
                return Ok(outSubMods);
            }
            if AbsynUtil::pathIsIdent(comp_name.clone()) {
                break;
            } else {
                comp_name = AbsynUtil::pathPrefix(comp_name.clone())?;
            }
        }
        outSubMods = cons(submod.clone(), outSubMods.clone());
    }
    if !(AbsynUtil::isEmptyMod(inNewMod.clone())) {
        submod = createNestedSubMod(AbsynUtil::pathRest(inComponentName.clone())?, inNewMod.clone())?;
        outSubMods = cons(submod.clone(), outSubMods.clone()).reverse();
    } else {
        outSubMods = inSubMods.clone();
    }
    Ok(outSubMods)
}

fn createNestedSubMod(mut inComponentName: Arc<Absyn::Path>, mut inMod: Arc<Absyn::Modification>) -> Result<Arc<Absyn::ElementArg>> {
    let mut outSubMod: Arc<Absyn::ElementArg>;
    let mut e: Arc<Absyn::ElementArg>;
    if AbsynUtil::pathIsIdent(inComponentName.clone()) {
        outSubMod = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: inComponentName.clone(), modification: Some(inMod.clone()), comment: None, info: Absyn::dummyInfo.clone() });
    } else {
        outSubMod = createNestedSubMod(AbsynUtil::pathRest(inComponentName.clone())?, inMod.clone())?;
        outSubMod = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: AbsynUtil::pathFirstPath(inComponentName.clone())?, modification: Some(Arc::new(Absyn::Modification { elementArgLst: list![outSubMod.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() });
    }
    Ok(outSubMod)
}

pub fn getElementModifierValue(mut classRef: Arc<Absyn::ComponentRef>, mut varRef: Arc<Absyn::ComponentRef>, mut subModRef: Arc<Absyn::ComponentRef>, mut program: Absyn::Program) -> ArcStr {
    let mut valueStr: ArcStr = arcstr::literal!("");
    let mut cls_path: Arc<Absyn::Path>;
    let mut name: ArcStr = arcstr::literal!("");
    let mut elName: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Absyn::Class>;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut elems: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    let mut found: bool = false;
    let mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut optMod: Option<Arc<Absyn::Modification>> = None;
    match '__try0: {
        cls_path = unwrap_break_err!(AbsynUtil::crefToPath(classRef.clone()), '__try0);
        elName = (unwrap_break_err!(AbsynUtil::crefIdent(varRef.clone()), '__try0)).clone();
        cls = unwrap_break_err!(getPathedClassInProgram(cls_path.clone(), program.clone(), false, false), '__try0);
        elems = getElementsInClass(cls.clone());
        for mut e in &*elems.clone() {
            let mut e = e.clone();
            args = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { arguments: args, .. }, name, .. }, .. }, .. } if (unwrap_break_err!(stringEq((name.clone()).clone(), (elName.clone()).clone()), '__try0)) => {
            found = true;
            args.clone()
        },
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components, .. }, .. } => {
            for mut c in &*components.clone() {
                let mut c = c.clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(c.clone()) {
                    Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: __pa0, name: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                optMod = __pa0.clone();
                name = __pa1.clone();
                if unwrap_break_err!(stringEq((name.clone()).clone(), (elName.clone()).clone()), '__try0) {
                    let __pa2 = ::match_deref::match_deref! { match &(Util::getOptionOrDefault(optMod.clone(), Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) }))) {
                        Deref @ Absyn::Modification { elementArgLst: __pa2, .. } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    args = __pa2.clone();
                    found = true;
                }
            }
            args.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            if found.clone() {
                break;
            }
        }
        if found.clone() {
            valueStr = (unwrap_break_err!(getModificationValueStr(args.clone(), AbsynUtil::crefToPath(subModRef.clone()).unwrap()), '__try0)).clone();
        } else {
            valueStr = (literal!("")).clone();
        }
        Ok::<_, anyhow::Error>((valueStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            valueStr = __try0_o0;
        }
        Err(_) => {
            valueStr = (literal!("")).clone();
        }
    }
    valueStr
}

pub fn getModificationValueStr(mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut path: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut value: ArcStr = literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut rest_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = args.clone();
    let mut arg: Arc<Absyn::ElementArg>;
    let mut found: bool = false;
    let mut elSpec: Arc<Absyn::ElementSpec>;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    while !(found.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_args.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        arg = __pa0.clone();
        rest_args = __pa1.clone();
        found = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } if (AbsynUtil::pathEqual(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), path.clone())) => {
            let __pa0 = ::match_deref::match_deref! { match &(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa0, .. }, .. }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            exp = __pa0.clone();
            value = (Dump::printExpStr(exp.clone())?).clone();
            true
        },
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name }, .. } if (name.clone() == AbsynUtil::pathFirstIdent(path.clone())?) => {
            let __pa0 = ::match_deref::match_deref! { match &(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                Some(Deref @ Absyn::Modification { elementArgLst: __pa0, .. }) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            rest_args = __pa0.clone();
            value = (getModificationValueStr(rest_args.clone(), AbsynUtil::pathRest(path.clone())?)?).clone();
            true
        },
        Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elSpec, .. } if (AbsynUtil::pathFirstIdent(path.clone())? == AbsynUtil::elementSpecName(elSpec.clone())?) => {
            value = (System::escapedString((Dump::unparseElementArgStr(arg.clone())?).clone(), false)).clone();
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    Ok(value)
}

pub fn getElementModifierValues(mut inComponentRef1: Arc<Absyn::ComponentRef>, mut inComponentRef2: Arc<Absyn::ComponentRef>, mut inComponentRef3: Arc<Absyn::ComponentRef>, mut inProgram4: Absyn::Program) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inComponentRef1.clone(), inComponentRef2.clone(), inComponentRef3.clone(), inProgram4.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, ident, subident, p) => {
                    let mut p_class: Arc<Absyn::Path>;
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut res: ArcStr = arcstr::literal!("");
                    let mut cdef: Arc<Absyn::Class>;
                    let mut elems: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
                    let mut compelts: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>>> = metamodelica::nil();
                    let mut compelts_1: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut r#mod: Arc<Absyn::Modification>;
                    let mut elementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    p_class = AbsynUtil::crefToPath(class_.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::crefToPath(ident.clone())?) {
                        Deref @ Absyn::Path::IDENT { name: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    cdef = getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    elems = getElementsInClass(cdef.clone());
                    compelts = List::map(elems.clone(), Arc::new(fnptr!(getComponentitemsInElement, Arc<Absyn::Element>)));
                    compelts_1 = List::flatten(compelts.clone());
                    let metamodelica::List::Cons { head: Absyn::COMPONENTITEM { component: Absyn::COMPONENT { modification: Some(Absyn::CLASSMOD { elementArgLst: __pa1, .. }), .. }, .. }, tail: __t2 } = (List::select1(compelts_1.clone(), Arc::new(componentitemNamed), (name.clone()).clone())) else { bail!("pattern mismatch") };
                    let metamodelica::List::Nil = (__t2.clone()) else { bail!("pattern mismatch") };
                    elementArgLst = __pa1.clone();
                    r#mod = getModificationValues(elementArgLst.clone(), AbsynUtil::crefToPath(subident.clone())?)?;
                    res = (unparseMods(r#mod.clone())?).clone();
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("Error"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(outString)
}

pub fn unparseMods(mut r#mod: Arc<Absyn::Modification>) -> Result<ArcStr> {
    let mut s: ArcStr = arcstr::literal!("");
    let mut arg: Arc<Absyn::ElementArg>;
    s = ((::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::List::Cons { head: arg @ Deref @ Absyn::ElementArg::REDECLARATION { .. }, tail: _ }, .. } => System::escapedString((Dump::unparseElementArgStr(arg.clone())?).clone(), false),
        _ => Dump::unparseModificationStr(r#mod.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(s)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getModificationValues(mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Modification>> {
    let mut outModification: Arc<Absyn::Modification>;
    outModification = (::match_deref::match_deref! { match &((inAbsynElementArgLst.clone(), inPath.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(r#mod), path: p1, .. }, tail: _ }, p2) if (AbsynUtil::pathEqual(p1.clone(), p2.clone())) => {
            r#mod.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, .. }), path: Deref @ Absyn::Path::IDENT { name: name1 }, .. }, tail: _ }, Deref @ Absyn::Path::QUALIFIED { path: p2, name: name2 }) if (stringEq((name1.clone()).clone(), (name2.clone()).clone())?) => {
            let mut res: Arc<Absyn::Modification>;
            res = getModificationValues(args.clone(), p2.clone())?;
            res.clone()
        },
        (Deref @ metamodelica::List::Cons { head: elArg @ Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elSpec, .. }, tail: _ }, p1) if (AbsynUtil::pathFirstIdent(p1.clone())? == AbsynUtil::elementSpecName(elSpec.clone())?) => {
            Arc::new(Absyn::Modification { elementArgLst: list![elArg.clone()], eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })
        },
        (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
            let mut r#mod: Arc<Absyn::Modification>;
            r#mod = getModificationValues(rest.clone(), inPath.clone())?;
            r#mod.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outModification)
}

pub fn getElementModifierNames(mut path: Arc<Absyn::Path>, mut inElementName: ArcStr, mut inProgram3: Absyn::Program) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outList: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outList = 'mc: {
        let __mc_input = (path.clone(), inElementName.clone(), inProgram3.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, p) => {
                    let mut cdef: Arc<Absyn::Class>;
                    let mut elems: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    let mut r#mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut found: bool = false;
                    let mut optMod: Option<Arc<Absyn::Modification>> = None;
                    cdef = getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    elems = getElementsInClass(cdef.clone());
                    for mut e in &*elems.clone() {
                        let mut e = e.clone();
                        r#mod = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { arguments: args, .. }, name, .. }, .. }, .. } if (stringEq((name.clone()).clone(), (inElementName.clone()).clone())?) => {
                    found = true;
                    args.clone()
        },
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components, .. }, .. } => {
                    for mut c in &*components.clone() {
                        let mut c = c.clone();
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(c.clone()) {
                            Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: __pa0, name: __pa1, .. }, .. } => (__pa0.clone(), __pa1.clone()),
                            _ => bail!("pattern mismatch"),
                        } };
                        optMod = __pa0.clone();
                        name = __pa1.clone();
                        if stringEq((name.clone()).clone(), (inElementName.clone()).clone())? {
                            let __pa2 = ::match_deref::match_deref! { match &(Util::getOptionOrDefault(optMod.clone(), Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) }))) {
                                        Deref @ Absyn::Modification { elementArgLst: __pa2, .. } => __pa2.clone(),
                                        _ => bail!("pattern mismatch"),
                            } };
                            r#mod = __pa2.clone();
                            found = true;
                        }
                    }
                    r#mod.clone()
        },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                        if found.clone() {
                            break;
                        }
                    }
                    res = getModificationNames(r#mod.clone(), true)?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut found: bool = false;
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outList)
}

pub fn getExtendsModifierNames(mut classPath: Arc<Absyn::Path>, mut extendsPath: Arc<Absyn::Path>, mut useQuotes: bool, mut program: Absyn::Program) -> Arc<Values::Value> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cdef: Arc<Absyn::Class>;
    let mut exts: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
    let mut env: GraphicEnvCache;
    let mut extmod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut res_str: ArcStr = arcstr::literal!("");
    let mut silent: bool = !(Flags::isSet(Flags::NF_API_NOISE.clone()).unwrap());
    if silent.clone() {
        ErrorExt::setCheckpoint((literal!("InteractiveUtil.getExtendsModifierNames")).clone());
    }
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(getPathedExtendsInProgram(classPath.clone(), extendsPath.clone(), program.clone())) {
            Some(Deref @ Absyn::ElementSpec::EXTENDS { elementArg: __pa1, .. }) => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        extmod = __pa1.clone();
        res = unwrap_break_err!(getModificationNames(extmod.clone(), true), '__try0);
        if useQuotes.clone() {
            res = Interactive::insertQuotesToList(res.clone());
        }
        result = unwrap_break_err!(ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut s in (res.clone()).into_iter().cloned() {
            let __x = ValuesMake::makeCodeTypeName(AbsynUtil::makeIdentPathFromString((s.clone()).clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), '__try0);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeString((literal!("Error")).clone());
        }
    }
    if silent.clone() {
        ErrorExt::rollBack((literal!("InteractiveUtil.getExtendsModifierNames")).clone());
    }
    result
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getModificationNames(mut inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut includeRedeclares: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = inAbsynElementArgLst.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: None, path: Deref @ Absyn::Path::IDENT { name }, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    names = getModificationNames(rest.clone(), includeRedeclares.clone())?;
                    Ok(cons(name.clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::List::Nil, eqMod: _ }), path: p, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    names = getModificationNames(rest.clone(), includeRedeclares.clone())?;
                    Ok(cons(name.clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: Deref @ Absyn::EqMod::EQMOD { .. } }), path: p, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    names2 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (getModificationNames(args.clone(), includeRedeclares.clone())?).into_iter().cloned() {
                    let __x = stringAppend((stringAppend((name.clone()).clone(), (literal!(".")).clone())).clone(), (n.clone()).clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    names = getModificationNames(rest.clone(), includeRedeclares.clone())?;
                    res = listAppend(names2.clone(), names.clone());
                    Ok(cons(name.clone(), res.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { elementArgLst: args, eqMod: _ }), path: p, .. }, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?).clone();
                    names2 = {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut n in (getModificationNames(args.clone(), includeRedeclares.clone())?).into_iter().cloned() {
                    let __x = stringAppend((stringAppend((name.clone()).clone(), (literal!(".")).clone())).clone(), (n.clone()).clone());
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
                    names = getModificationNames(rest.clone(), includeRedeclares.clone())?;
                    res = listAppend(names2.clone(), names.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::REDECLARATION { elementSpec: elSpec, .. }, tail: rest } => {
                    if !((includeRedeclares.clone())) { bail!("guard") }
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut name: ArcStr = arcstr::literal!("");
                    name = (AbsynUtil::elementSpecName(elSpec.clone())?).clone();
                    names = getModificationNames(rest.clone(), includeRedeclares.clone())?;
                    Ok(cons(name.clone(), names.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    names = getModificationNames(rest.clone(), includeRedeclares.clone())?;
                    Ok(names.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

pub fn getElementBinding(mut path: Arc<Absyn::Path>, mut parameterName: ArcStr, mut program: Absyn::Program) -> ArcStr {
    let mut bindingStr: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Absyn::Class>;
    let mut component: Arc<Absyn::ComponentItem>;
    match '__try0: {
        cls = unwrap_break_err!(getPathedClassInProgram(path.clone(), program.clone(), false, false), '__try0);
        component = unwrap_break_err!(getComponentInClass(cls.clone(), (parameterName.clone()).clone()), '__try0);
        bindingStr = (unwrap_break_err!(Dump::printExpStr(getVariableBindingInComponentitem(component.clone()).unwrap()), '__try0)).clone();
        Ok::<_, anyhow::Error>((bindingStr.clone(),))
    } {
        Ok((__try0_o0,)) => {
            bindingStr = __try0_o0;
        }
        Err(_) => {
            bindingStr = (literal!("")).clone();
        }
    }
    bindingStr
}

pub fn getComponentInClass(mut cls: Arc<Absyn::Class>, mut componentName: ArcStr) -> Result<Arc<Absyn::ComponentItem>> {
    let mut component: Arc<Absyn::ComponentItem>;
    let mut body: Arc<Absyn::ClassDef>;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut elements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut found: bool = false;
    let __pa0 = ::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class { body: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    body = __pa0.clone();
    parts = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone(),
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(),
        _ => bail!("match: no arm matched"),
    } });
    for mut part in &*parts.clone() {
        let mut part = part.clone();
        elements = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone(),
        Deref @ Absyn::ClassPart::PROTECTED { .. } => var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        for mut e in &*elements.clone() {
            let mut e = e.clone();
            components = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components, .. }, .. } } => components.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            for mut c in &*components.clone() {
                let mut c = c.clone();
                if AbsynUtil::componentName(c.clone())? == componentName.clone() {
                    component = c.clone();
                    return Ok(component);
                }
            }
        }
    }
    bail!("fail");
    Ok(component)
}

pub fn getNthComponentInClass(mut inClass: Arc<Absyn::Class>, mut nth: i32) -> Arc<Absyn::Element> {
    let mut outElement: Arc<Absyn::Element>;
    let mut r#pub: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    let mut pro: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    let mut n: i32 = 0;
    r#pub = getPublicComponentsInClass(inClass.clone());
    n = (r#pub.clone().len() as i32);
    if nth.clone() <= n.clone() {
        outElement = (r#pub.clone()).get(nth.clone()).unwrap();
    } else {
        pro = getProtectedComponentsInClass(inClass.clone());
        outElement = (pro.clone()).get(nth.clone() - n.clone()).unwrap();
    }
    outElement
}

pub fn getComponentsInClass(mut inClass: Arc<Absyn::Class>, mut visibility: Visibility) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut outAbsynElementLst: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    outAbsynElementLst = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                res = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } if (visibility.clone() != Visibility::PROTECTED.clone()) => {
            lst1 = getComponentsInElementitems(var_field!((*elt).contents, Absyn::ClassPart::PUBLIC).clone());
            List::append_reverse(lst1.clone(), res.clone())
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } if (visibility.clone() != Visibility::PUBLIC.clone()) => {
            lst1 = getComponentsInElementitems(var_field!((*elt).contents, Absyn::ClassPart::PROTECTED).clone());
            List::append_reverse(lst1.clone(), res.clone())
        },
        _ => res.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            Dangerous::listReverseInPlace(res.clone())
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                res = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } if (visibility.clone() != Visibility::PROTECTED.clone()) => {
            lst1 = getComponentsInElementitems(var_field!((*elt).contents, Absyn::ClassPart::PUBLIC).clone());
            List::append_reverse(lst1.clone(), res.clone())
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } if (visibility.clone() != Visibility::PUBLIC.clone()) => {
            lst1 = getComponentsInElementitems(var_field!((*elt).contents, Absyn::ClassPart::PROTECTED).clone());
            List::append_reverse(lst1.clone(), res.clone())
        },
        _ => res.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            Dangerous::listReverseInPlace(res.clone())
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

pub fn getPublicComponentsInClass(mut inClass: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut components: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    components = getComponentsInClass(inClass.clone(), Visibility::PUBLIC.clone());
    components
}

pub fn getProtectedComponentsInClass(mut inClass: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut components: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    components = getComponentsInClass(inClass.clone(), Visibility::PROTECTED.clone());
    components
}

pub fn getComponentsInElementitems(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut outAbsynElementLst: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    for mut el in &*inAbsynElementItemLst.clone() {
        let mut el = el.clone();
        let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt @ Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } } => {
            outAbsynElementLst = cons(elt.clone(), outAbsynElementLst.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outAbsynElementLst = Dangerous::listReverseInPlace(outAbsynElementLst.clone());
    outAbsynElementLst
}

pub fn getVariableBindingInComponentitem(mut inComponentItem: Arc<Absyn::ComponentItem>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    outExp = (::match_deref::match_deref! { match &(inComponentItem.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }), .. }, .. } => {
            e.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn buildWithin(mut inPath: Arc<Absyn::Path>) -> Result<Absyn::Within> {
    let mut outWithin: Absyn::Within = Absyn::Within::TOP;
    outWithin = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            openmodelica_ast::Absyn::Within::TOP
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
            buildWithin(path.clone())?
        },
        path => {
            let mut w_path: Arc<Absyn::Path>;
            w_path = AbsynUtil::stripLast(path.clone())?;
            Absyn::Within::WITHIN { path: w_path.clone() }
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outWithin)
}

pub fn componentitemNamed(mut inComponentItem: Arc<Absyn::ComponentItem>, mut inIdent: ArcStr) -> Result<bool> {
    let mut outBoolean: bool = false;
    outBoolean = (::match_deref::match_deref! { match &((inComponentItem.clone(), inIdent.clone())) {
        (Deref @ Absyn::ComponentItem { component: Absyn::Component { name: id1, .. }, .. }, id2) if (stringEq((id1.clone()).clone(), (id2.clone()).clone())?) => {
            true
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBoolean)
}

pub fn getComponentitemsInElement(mut inElement: Arc<Absyn::Element>) -> Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> {
    let mut outAbsynComponentItemLst: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    outAbsynComponentItemLst = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: l, .. }, .. } => {
            l.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynComponentItemLst
}

pub fn updateProgram(mut inNewProgram: Absyn::Program, mut inOldProgram: Absyn::Program, mut mergeAST: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut cs: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut w: Absyn::Within = Absyn::Within::TOP;
    let Absyn::PROGRAM { within_: __pa0, classes: __pa1 } = (inNewProgram.clone()) else { bail!("pattern mismatch") };
    w = __pa0.clone();
    cs = __pa1.clone();
    outProgram = updateProgram2(cs.clone().reverse(), w.clone(), inOldProgram.clone(), mergeAST.clone())?;
    Ok(outProgram)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn updateProgram2(mut inNewClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>, mut w: Absyn::Within, mut inOldProgram: Absyn::Program, mut mergeAST: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    outProgram = (::match_deref::match_deref! { match &((inNewClasses.clone(), w.clone(), inOldProgram.clone())) {
        (Deref @ metamodelica::List::Nil, _, prg) => {
            prg.clone()
        },
        (Deref @ metamodelica::List::Cons { head: c1 @ Deref @ Absyn::Class { name, .. }, tail: c2 }, Absyn::Within::TOP, p2 @ Absyn::Program { within_: w2, classes: c3 }) => {
            let mut newp: Absyn::Program;
            if classInProgram((name.clone()).clone(), p2.clone())? {
                newp = replaceClassInProgram(c1.clone(), p2.clone(), mergeAST.clone())?;
            } else {
                newp = Absyn::Program { classes: cons(c1.clone(), c3.clone()), within_: w2.clone() };
            }
            updateProgram2(c2.clone(), w.clone(), newp.clone(), mergeAST.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: c1, tail: c2 }, Absyn::Within::WITHIN { .. }, p2) => {
            let mut newp: Absyn::Program;
            let mut newp_1: Absyn::Program;
            newp = insertClassInProgram(c1.clone(), w.clone(), p2.clone(), mergeAST.clone())?;
            newp_1 = updateProgram2(c2.clone(), w.clone(), newp.clone(), mergeAST.clone())?;
            newp_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outProgram)
}

pub fn createEnvironment(mut p: Absyn::Program, mut os: Option<Arc<metamodelica::List<Arc<SCode::Element>>>>, mut modelPath: Arc<Absyn::Path>) -> Result<Interactive::GraphicEnvCache> {
    let mut genv: Interactive::GraphicEnvCache;
    let mut env: FCore::Graph;
    let mut env_1: FCore::Graph;
    let mut env2: FCore::Graph;
    let mut s: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut c: Arc<SCode::Element>;
    let mut id: ArcStr = arcstr::literal!("");
    let mut encflag: SCode::Encapsulated = SCode::Encapsulated::ENCAPSULATED;
    let mut restr: SCode::Restriction = SCode::Restriction::R_BLOCK;
    let mut ci_state: ClassInf::State;
    let mut model_: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut b: bool = false;
    let mut permissive: bool = false;
    if Flags::isSet(Flags::NF_API.clone())? {
        genv = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { program: p.clone(), modelPath: modelPath.clone(), cache: FCore::emptyCache(), env: FGraph::emptyGraph().clone() };
    } else {
        s = Util::getOptionOrDefault(os.clone(), AbsynToSCode::translateAbsyn2SCode(p.clone())?);
        (cache, env) = Inst::makeEnvFromProgram(s.clone())?;
        let (__pa0, __pa4, __pa1, __pa2, __pa3, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(cache.clone(), env.clone(), modelPath.clone(), None)?) {
            (__pa0, __pa4 @ Deref @ SCode::Element::CLASS { restriction: __pa1, encapsulatedPrefix: __pa2, name: __pa3, .. }, __pa5) => (__pa0.clone(), __pa4.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa5.clone()),
            _ => bail!("pattern mismatch"),
        } };
        cache = __pa0.clone();
        restr = __pa1.clone();
        encflag = __pa2.clone();
        id = __pa3.clone();
        c = __pa4.clone();
        env_1 = __pa5.clone();
        env2 = FGraph::openScope(env_1.clone(), encflag.clone(), (id.clone()).clone(), FGraph::restrictionToScopeType(restr.clone()))?;
        ci_state = ClassInfUtil::start(restr.clone(), FGraph::getGraphName(env2.clone())?)?;
        permissive = Flags::getConfigBool(Flags::PERMISSIVE.clone())?;
        FlagsUtil::setConfigBool(Flags::PERMISSIVE.clone(), true)?;
        match '__try6: {
            (_, env2, _, _, _) = unwrap_break_err!(Inst::partialInstClassIn(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, ci_state.clone(), c.clone(), openmodelica_frontend_types::SCode::Visibility::PUBLIC, metamodelica::nil(), 0), '__try6);
            unwrap_break_err!(FlagsUtil::setConfigBool(Flags::PERMISSIVE.clone(), permissive.clone()), '__try6);
            Ok::<_, anyhow::Error>((env2.clone(),))
        } {
            Ok((__try6_o0,)) => {
                env2 = __try6_o0;
            }
            Err(_) => {
                FlagsUtil::setConfigBool(Flags::PERMISSIVE.clone(), permissive.clone())?;
                bail!("fail");
            }
        }
        genv = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { program: SymbolTable::getAbsyn(), modelPath: modelPath.clone(), cache: cache.clone(), env: env2.clone() };
    }
    Ok(genv)
}

pub fn getClassCommentInCommentOpt(mut inComment: Option<Arc<Absyn::Comment>>) -> ArcStr {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inComment.clone()) {
        Some(Deref @ Absyn::Comment { comment: Some(outString), .. }) => outString.clone(),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    outString
}

pub fn getClassnamesInClass(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClass: Arc<Absyn::Class>, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = (::match_deref::match_deref! { match &((inPath.clone(), inProgram.clone(), inClass.clone(), inShowProtected.clone(), includeConstants.clone())) {
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            List::map(strlist.clone(), Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)))
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            List::map(strlist.clone(), Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)))
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(paths)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getClassnamesInParts(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = (inAbsynClassPartLst.clone(), inShowProtected.clone(), includeConstants.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest }, b, c) => {
                    let mut l1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    l1 = getClassnamesInElts(elts.clone(), c.clone())?;
                    l2 = getClassnamesInParts(rest.clone(), b.clone(), c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest }, true, c) => {
                    let mut l1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    l1 = getClassnamesInElts(elts.clone(), c.clone())?;
                    l2 = getClassnamesInParts(rest.clone(), true, c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, b, c) => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getClassnamesInParts(rest.clone(), b.clone(), c.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

pub fn getClassnamesInElts(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut delst: DoubleEnded::MutableList<ArcStr>;
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    for mut elt in &*inAbsynElementItemLst.clone() {
        let mut elt = elt.clone();
        let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: id, .. }, .. }, .. }, .. } } => {
            DoubleEnded::push_back(delst.clone(), (id.clone()).clone());
            ()
        },
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: id, .. }, .. }, .. } } => {
            DoubleEnded::push_back(delst.clone(), (id.clone()).clone());
            ()
        },
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: lst, attributes: Absyn::ElementAttributes { variability: Absyn::Variability::CONST, .. }, .. }, .. } } if (includeConstants.clone()) => {
            DoubleEnded::push_list_back(delst.clone(), getComponentItemsName(lst.clone(), false));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outStringLst = DoubleEnded::toListAndClear(delst.clone(), metamodelica::nil());
    Ok(outStringLst)
}

pub fn getElementAnnotationsFromElts(mut els: Arc<metamodelica::List<Arc<Absyn::Element>>>, mut inClass: Arc<Absyn::Class>, mut inFullProgram: Absyn::Program, mut inModelPath: Arc<Absyn::Path>) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut graphicProgramSCode: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut env: FCore::Graph;
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut placementProgram: Absyn::Program;
    let mut cache: GraphicEnvCache;
    if !(Flags::isSet(Flags::NF_API.clone())?) {
        placementProgram = modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?;
        graphicProgramSCode = AbsynToSCode::translateAbsyn2SCode(placementProgram.clone())?;
        (_, env) = Inst::makeEnvFromProgram(graphicProgramSCode.clone())?;
    } else {
        env = FGraph::emptyGraph().clone();
    }
    cache = Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { program: inFullProgram.clone(), modelPath: inModelPath.clone() };
    result = getElementitemsAnnotations(els.clone(), env.clone(), inClass.clone(), cache.clone())?;
    Ok(result)
}

fn getElementitemsAnnotations(mut inElements: Arc<metamodelica::List<Arc<Absyn::Element>>>, mut inEnv: FCore::Graph, mut inClass: Arc<Absyn::Class>, mut inCache: GraphicEnvCache) -> Result<Arc<Values::Value>> {
    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut res: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut accum: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut cache: GraphicEnvCache = inCache.clone();
    let mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut cc: Option<Arc<Absyn::ConstrainClass>> = None;
    let mut annotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut cmt: Option<Arc<Absyn::Comment>> = None;
    let mut fullProgram: Absyn::Program;
    let mut modelPath: Arc<Absyn::Path>;
    if Flags::isSet(Flags::NF_API.clone())? {
        (fullProgram, modelPath) = Interactive::cacheProgramAndPath(inCache.clone())?;
        result = makeAnnotationArrayValue(NFApi::evaluateAnnotations(fullProgram.clone(), modelPath.clone(), inElements.clone())?)?;
        return Ok(result);
    }
    for mut e in &*inElements.clone().reverse() {
        let mut e = e.clone();
        accum = 'mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Element::ELEMENT { constrainClass: cc, specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: items, .. }, .. } => {
                    let mut cache: Interactive::GraphicEnvCache = cache.clone();
                    let mut res: Arc<metamodelica::List<Arc<Values::Value>>> = res.clone();
                    (res, cache) = getElementitemsAnnotationsFromItems(items.clone(), getAnnotationsFromConstraintClass(cc.clone()), inEnv.clone(), inClass.clone(), cache.clone())?;
                    Ok(listAppend(res.clone(), accum.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Element::ELEMENT { constrainClass: cc, specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: cmt, .. }, .. }, .. }, .. } => {
                    let mut res: Arc<metamodelica::List<Arc<Values::Value>>> = res.clone();
                    let mut cache: Interactive::GraphicEnvCache = cache.clone();
                    let mut annotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = annotations.clone();
                    annotations = (::match_deref::match_deref! { match &(cmt.clone()) {
        Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annotations }), .. }) => annotations.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                    (res, cache) = getElementitemsAnnotationsFromElArgs(annotations.clone(), getAnnotationsFromConstraintClass(cc.clone()), inEnv.clone(), inClass.clone(), cache.clone())?;
                    Ok(cons(ValuesMake::makeArray(res.clone())?, accum.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { .. }, .. } => {
                    Ok(cons(Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), accum.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { .. }, .. }, .. }, .. } => {
                    Ok(cons(Arc::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] }), accum.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(accum.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    }
    result = ValuesMake::makeArray(accum.clone())?;
    Ok(result)
}

fn getElementitemsAnnotationsFromElArgs(mut inAnnotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut ccAnnotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inEnv: FCore::Graph, mut inClass: Arc<Absyn::Class>, mut inCache: GraphicEnvCache) -> Result<(Arc<metamodelica::List<Arc<Values::Value>>>, GraphicEnvCache)> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut outCache: GraphicEnvCache = inCache.clone();
    let mut annotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    annotations = listAppend(inAnnotations.clone(), ccAnnotations.clone());
    (strl, outCache) = getElementitemsAnnotationsElArgs(annotations.clone(), inEnv.clone(), inClass.clone(), outCache.clone(), true)?;
    result = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut s in (strl.clone()).into_iter().cloned() {
            let __x = ValuesMake::makeCodeTypeNameStr((s.clone()).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((result, outCache))
}

fn getAnnotationsFromConstraintClass(mut inCC: Option<Arc<Absyn::ConstrainClass>>) -> Arc<metamodelica::List<Arc<Absyn::ElementArg>>> {
    let mut outElArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    outElArgLst = (::match_deref::match_deref! { match &(inCC.clone()) {
        Some(Deref @ Absyn::ConstrainClass { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs }), .. }), .. }) => {
            elementArgs.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElArgLst
}

pub fn getElementitemsAnnotationsElArgs(mut inElementArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inEnv: FCore::Graph, mut inClass: Arc<Absyn::Class>, mut inCache: GraphicEnvCache, mut addAnnotationName: bool) -> Result<(Arc<metamodelica::List<ArcStr>>, GraphicEnvCache)> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut outCache: GraphicEnvCache = inCache.clone();
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut ann_name: ArcStr = arcstr::literal!("");
    let mut eq_aexp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut graphic_exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut eq_dexp: Arc<DAE::Exp>;
    let mut graphic_dexp: Arc<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut info: SourceInfo;
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut env: FCore::Graph;
    let mut env2: FCore::Graph;
    let mut r#mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut stripped_mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut graphic_mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut c: Arc<SCode::Element>;
    let mut smod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
    let mut dmod: Arc<DAE::Mod> = Arc::new(DAE::Mod::NOMOD);
    let mut dae: DAE::DAElist;
    let mut is_icon: bool = false;
    let mut is_diagram: bool = false;
    let mut graphic_prog: Absyn::Program;
    let mut placement_cls: Arc<SCode::Element>;
    for mut e in &*inElementArgs.clone().reverse() {
        let mut e = e.clone();
        e = AbsynUtil::createChoiceArray(e.clone())?;
        r#str = ('mc: {
        let __mc_input = e.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementArg::MODIFICATION { info, modification: Some(Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::List::Nil, eqMod: Deref @ Absyn::EqMod::EQMOD { exp: eq_aexp, .. } }), path: Deref @ Absyn::Path::IDENT { name: ann_name }, .. } => {
                    let mut env: FCore::Graph;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut outCache: Interactive::GraphicEnvCache = outCache.clone();
                    let mut prop: DAE::Properties;
                    let mut eq_dexp: Arc<DAE::Exp>;
                    let mut r#str: ArcStr = r#str.clone();
                    (cache, env, _, outCache) = buildEnvForGraphicProgram(outCache.clone(), metamodelica::nil())?;
                    (_, eq_dexp, prop) = StaticScript::elabGraphicsExp(cache.clone(), env.clone(), eq_aexp.clone(), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    (cache, eq_dexp, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), eq_dexp.clone(), prop.clone(), false, info.clone())?;
                    (eq_dexp, _) = ExpressionSimplify::simplify1(eq_dexp.clone())?;
                    Print::clearErrorBuf();
                    r#str = (ExpressionBasics::printExpStr(eq_dexp.clone())?).clone();
                    Ok(stringAppendList(list![(ann_name.clone()).clone(), (literal!("=")).clone(), (r#str.clone()).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementArg::MODIFICATION { info, modification: Some(Deref @ Absyn::Modification { elementArgLst: r#mod, eqMod: Deref @ Absyn::EqMod::NOMOD }), path: Deref @ Absyn::Path::IDENT { name: ann_name }, .. } => {
                    let mut prop: DAE::Properties;
                    let mut is_icon: bool = is_icon.clone();
                    let mut graphic_prog: Absyn::Program;
                    let mut placement_cls: Arc<SCode::Element>;
                    let mut graphic_exp: Arc<Absyn::Exp> = graphic_exp.clone();
                    let mut is_diagram: bool = is_diagram.clone();
                    let mut r#str: ArcStr = r#str.clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut c: Arc<SCode::Element>;
                    let mut env2: FCore::Graph;
                    let mut stripped_mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = stripped_mod.clone();
                    let mut dmod: Arc<DAE::Mod> = dmod.clone();
                    let mut graphic_dexp: Arc<DAE::Exp>;
                    let mut smod: Arc<SCode::Mod> = smod.clone();
                    let mut outCache: Interactive::GraphicEnvCache = outCache.clone();
                    let mut env: FCore::Graph;
                    let mut dae: DAE::DAElist;
                    let mut graphic_mod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = graphic_mod.clone();
                    if !(listMember((ann_name.clone()).clone(), list![(literal!("Icon")).clone(), (literal!("Diagram")).clone(), (literal!("choices")).clone()])) {
                        (cache, env, _, outCache) = buildEnvForGraphicProgram(outCache.clone(), r#mod.clone())?;
                        (cache, c, env2) = Lookup::lookupClassIdent(cache.clone(), inEnv.clone(), (ann_name.clone()).clone(), None)?;
                        smod = AbsynToSCode::translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: r#mod.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                        (cache, dmod) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, smod.clone(), false, Mod::ModScope::COMPONENT { name: (ann_name.clone()).clone() }, Absyn::dummyInfo.clone())?;
                        c = SCodeUtil::classSetPartial(c.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                        (_, _, _, _, dae, _, _, _, _, _) = Inst::instClass(cache.clone(), env2.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), dmod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, openmodelica_frontend::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?;
                        r#str = (DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(dae.clone())?)?).clone();
                    } else {
                        is_icon = ann_name.clone() == literal!("Icon");
                        is_diagram = ann_name.clone() == literal!("Diagram") || ann_name.clone() == literal!("choices");
                        (stripped_mod, graphic_mod) = AbsynUtil::stripGraphicsAndInteractionModification(r#mod.clone())?;
                        ErrorExt::setCheckpoint((literal!("buildEnvForGraphicProgram")).clone());
                        match '__try0: {
                            (cache, env, graphic_prog, _) = unwrap_break_err!(buildEnvForGraphicProgram(inCache.clone(), r#mod.clone()), '__try0);
                            ErrorExt::rollBack((literal!("buildEnvForGraphicProgram")).clone());
                            Ok::<_, anyhow::Error>((cache.clone(), env.clone(), graphic_prog.clone()))
                        } {
                            Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                                        cache = __try0_o0;
                                        env = __try0_o1;
                                        graphic_prog = __try0_o2;
                            }
                            Err(_) => {
                                        ErrorExt::delCheckpoint((literal!("buildEnvForGraphicProgram")).clone());
                                        (cache, env, graphic_prog, _) = buildEnvForGraphicProgram(inCache.clone(), metamodelica::nil())?;
                            }
                        }
                        smod = AbsynToSCode::translateMod(Some(Arc::new(Absyn::Modification { elementArgLst: stripped_mod.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), openmodelica_frontend_types::SCode::Final::NOT_FINAL, openmodelica_frontend_types::SCode::Each::NOT_EACH, None, info.clone(), false)?;
                        (cache, dmod) = Mod::elabMod(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, smod.clone(), false, Mod::ModScope::COMPONENT { name: (ann_name.clone()).clone() }, info.clone())?;
                        placement_cls = AbsynToSCode::translateClass(getClassInProgram((ann_name.clone()).clone(), graphic_prog.clone())?)?;
                        (cache, _, _, _, dae, _, _, _, _, _) = Inst::instClass(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), dmod.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, placement_cls.clone(), metamodelica::nil(), false, openmodelica_frontend::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?;
                        r#str = (DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(dae.clone())?)?).clone();
                        if is_icon.clone() || is_diagram.clone() {
                            if '__try1: {
                                        let __pa2 = ::match_deref::match_deref! { match &(graphic_mod.clone()) {
                                            Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: __pa2, .. }, .. }), .. }, tail: Deref @ metamodelica::List::Nil } => __pa2.clone(),
                                            _ => break '__try1 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                                        } };
                                        graphic_exp = __pa2.clone();
                                        (_, graphic_dexp, prop) = unwrap_break_err!(StaticScript::elabGraphicsExp(cache.clone(), env.clone(), graphic_exp.clone(), false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone()), '__try1);
                                        if is_icon.clone() {
                                            ErrorExt::setCheckpoint((literal!("getAnnotationString: Icon")).clone());
                                            (cache, graphic_dexp, _) = unwrap_break_err!(Ceval::cevalIfConstant(cache.clone(), env.clone(), graphic_dexp.clone(), prop.clone(), false, info.clone()), '__try1);
                                            (graphic_dexp, _) = unwrap_break_err!(ExpressionSimplify::simplify1(graphic_dexp.clone()), '__try1);
                                            ErrorExt::rollBack((literal!("getAnnotationString: Icon")).clone());
                                        }
                                        r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!(",")); __mm_s.push_str(&*unwrap_break_err!(ExpressionBasics::printExpStr(graphic_dexp.clone()), '__try1)); ArcStr::from(__mm_s) }).clone();
                                        Ok::<(), anyhow::Error>(())
                            }.is_err() {
                            }
                        }
                        Print::clearErrorBuf();
                    }
                    Ok(if (addAnnotationName.clone()) {stringAppendList(list![(ann_name.clone()).clone(), (literal!("(")).clone(), (r#str.clone()).clone(), (literal!(")")).clone()])} else {r#str.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementArg::MODIFICATION { modification: None, path: Deref @ Absyn::Path::IDENT { name: ann_name }, .. } => {
                    let mut c: Arc<SCode::Element>;
                    let mut cache: FCore::Cache = cache.clone();
                    let mut outCache: Interactive::GraphicEnvCache = outCache.clone();
                    let mut env: FCore::Graph;
                    let mut r#str: ArcStr = r#str.clone();
                    let mut dae: DAE::DAElist;
                    (cache, _, _, outCache) = buildEnvForGraphicProgram(outCache.clone(), metamodelica::nil())?;
                    (cache, c, env) = Lookup::lookupClassIdent(cache.clone(), inEnv.clone(), (ann_name.clone()).clone(), None)?;
                    c = SCodeUtil::classSetPartial(c.clone(), openmodelica_frontend_types::SCode::Partial::NOT_PARTIAL)?;
                    (_, _, _, _, dae, _, _, _, _, _) = Inst::instClass(cache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), UnitAbsyn::noStore().clone(), Arc::new(openmodelica_frontend_types::DAE::Mod::NOMOD), openmodelica_frontend_types::DAE::Prefix::NOPRE, c.clone(), metamodelica::nil(), false, openmodelica_frontend::InstTypes::CallingScope::TOP_CALL, ConnectionGraph::EMPTY.clone(), Connect::emptySet.clone())?;
                    r#str = (DAEUtil::getVariableBindingsStr(DAEUtil::daeElements(dae.clone())?)?).clone();
                    Ok(if (addAnnotationName.clone()) {stringAppendList(list![(ann_name.clone()).clone(), (literal!("(")).clone(), (r#str.clone()).clone(), (literal!(")")).clone()])} else {r#str.clone()})
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ElementArg::MODIFICATION { info, path: Deref @ Absyn::Path::IDENT { name: ann_name }, .. } => {
                    let mut r#str: ArcStr = r#str.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("error evaluating: annotation(")); __mm_s.push_str(&*Dump::unparseElementArgStr(e.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
                    r#str = (Util::escapeQuotes((r#str.clone()).clone())?).clone();
                    Ok(stringAppendList(list![(ann_name.clone()).clone(), (literal!("(\"")).clone(), (r#str.clone()).clone(), (literal!("\")")).clone()]))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
        outStringLst = cons(r#str.clone(), outStringLst.clone());
    }
    Ok((outStringLst, outCache))
}

fn getElementitemsAnnotationsFromItems(mut inComponentItems: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut ccAnnotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mut inEnv: FCore::Graph, mut inClass: Arc<Absyn::Class>, mut inCache: GraphicEnvCache) -> Result<(Arc<metamodelica::List<Arc<Values::Value>>>, GraphicEnvCache)> {
    let mut result: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut outCache: GraphicEnvCache = inCache.clone();
    let mut annotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut strl: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    for mut comp in &*inComponentItems.clone().reverse() {
        let mut comp = comp.clone();
        annotations = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Absyn::ComponentItem { comment: Some(Deref @ Absyn::Comment { annotation_: Some(Deref @ Absyn::Annotation { elementArgs: annotations }), .. }), .. } => listAppend(annotations.clone(), ccAnnotations.clone()),
        _ => ccAnnotations.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (strl, outCache) = getElementitemsAnnotationsElArgs(annotations.clone(), inEnv.clone(), inClass.clone(), outCache.clone(), true)?;
        result = cons(makeAnnotationArrayValue(strl.clone())?, result.clone());
    }
    Ok((result, outCache))
}

pub fn modelicaAnnotationProgram(mut annotationVersion: ArcStr) -> Result<Absyn::Program> {
    let mut annotationProgram: Absyn::Program;
    let mut filename: ArcStr = arcstr::literal!("");
    filename = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?); __mm_s.push_str(&*literal!("/lib/omc/AnnotationsBuiltin_")); __mm_s.push_str(&*Util::stringReplaceChar((annotationVersion.clone()).clone(), (literal!(".")).clone(), (literal!("_")).clone())?); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone();
    annotationProgram = Parser::parse((filename.clone()).clone(), (literal!("UTF-8")).clone(), (literal!("")).clone(), None, Config::acceptedGrammar()?, Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone())?, Flags::getConfigBool(Flags::STRICT.clone())?)?;
    Ok(annotationProgram)
}

pub fn buildEnvForGraphicProgram(mut inCache: GraphicEnvCache, mut inAnnotationMod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(FCore::Cache, FCore::Graph, Absyn::Program, GraphicEnvCache)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outGraphicProgram: Absyn::Program;
    let mut outGraphicEnvCache: GraphicEnvCache;
    (outCache, outEnv, outGraphicProgram, outGraphicEnvCache) = (match inCache.clone() {
        Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { .. } => {
            (var_field!(inCache.cache, Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone(), var_field!(inCache.env, Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE).clone(), Absyn::dummyProgram.clone(), inCache.clone())
        },
        Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE { .. } => {
            if AbsynUtil::onlyLiteralsInAnnotationMod(inAnnotationMod.clone())? {
                outCache = var_field!(inCache.cache, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone();
                outEnv = var_field!(inCache.env, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone();
                outGraphicEnvCache = inCache.clone();
                outGraphicProgram = var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone();
            } else {
                (outCache, outEnv, outGraphicProgram) = buildEnvForGraphicProgramFull(var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone(), var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone())?;
                outGraphicEnvCache = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { program: var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone(), modelPath: var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE).clone(), cache: outCache.clone(), env: outEnv.clone() };
            }
            (outCache.clone(), outEnv.clone(), outGraphicProgram.clone(), outGraphicEnvCache.clone())
        },
        Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE { .. } => {
            let mut scode_program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
            if AbsynUtil::onlyLiteralsInAnnotationMod(inAnnotationMod.clone())? {
                outGraphicProgram = modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?;
                scode_program = AbsynToSCode::translateAbsyn2SCode(outGraphicProgram.clone())?;
                (outCache, outEnv) = Inst::makeEnvFromProgram(scode_program.clone())?;
                outGraphicEnvCache = Interactive::GraphicEnvCache::GRAPHIC_ENV_PARTIAL_CACHE { program: var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(), modelPath: var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(), cache: outCache.clone(), env: outEnv.clone() };
            } else {
                (outCache, outEnv, outGraphicProgram) = buildEnvForGraphicProgramFull(var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(), var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone())?;
                outGraphicEnvCache = Interactive::GraphicEnvCache::GRAPHIC_ENV_FULL_CACHE { program: var_field!(inCache.program, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(), modelPath: var_field!(inCache.modelPath, Interactive::GraphicEnvCache::GRAPHIC_ENV_NO_CACHE).clone(), cache: outCache.clone(), env: outEnv.clone() };
            }
            (outCache.clone(), outEnv.clone(), outGraphicProgram.clone(), outGraphicEnvCache.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok((outCache, outEnv, outGraphicProgram, outGraphicEnvCache))
}

fn buildEnvForGraphicProgramFull(mut inProgram: Absyn::Program, mut inModelPath: Arc<Absyn::Path>) -> Result<(FCore::Cache, FCore::Graph, Absyn::Program)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outEnv: FCore::Graph;
    let mut outProgram: Absyn::Program;
    let mut check_model: bool = false;
    let mut eval_param: bool = false;
    let mut failed: bool = false;
    let mut graphics_mode: bool = false;
    let mut graphic_program: Absyn::Program;
    let mut scode_program: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    graphic_program = modelicaAnnotationProgram((Config::getAnnotationVersion()?).clone())?;
    outProgram = updateProgram(graphic_program.clone(), inProgram.clone(), false)?;
    scode_program = AbsynToSCode::translateAbsyn2SCode(outProgram.clone())?;
    check_model = Flags::getConfigBool(Flags::CHECK_MODEL.clone())?;
    eval_param = Config::getEvaluateParametersInAnnotations()?;
    graphics_mode = Config::getGraphicsExpMode()?;
    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), true)?;
    Config::setEvaluateParametersInAnnotations(true)?;
    Config::setGraphicsExpMode(true)?;
    match '__try0: {
        (outCache, outEnv, _, _) = unwrap_break_err!(Inst::instantiateClass(FCore::emptyCache(), InnerOuter::emptyInstHierarchy().clone(), scode_program.clone(), inModelPath.clone(), true, true, true), '__try0);
        Ok::<_, anyhow::Error>((outCache.clone(), outEnv.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outCache = __try0_o0;
            outEnv = __try0_o1;
        }
        Err(_) => {
            failed = true;
            bail!("try/else: outputs not set in else branch");
        }
    }
    Config::setEvaluateParametersInAnnotations(eval_param.clone())?;
    FlagsUtil::setConfigBool(Flags::CHECK_MODEL.clone(), check_model.clone())?;
    Config::setGraphicsExpMode(graphics_mode.clone())?;
    if failed.clone() {
        bail!("fail");
    }
    Ok((outCache, outEnv, outProgram))
}

pub fn getElementsInClass(mut inClass: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut outAbsynElementLst: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    outAbsynElementLst = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res = Dangerous::listReverseInPlace(res.clone());
            res.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res = Dangerous::listReverseInPlace(res.clone());
            res.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

pub fn getPublicElementsInClass(mut inClass: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut outAbsynElementLst: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    outAbsynElementLst = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res = Dangerous::listReverseInPlace(res.clone());
            res.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res = Dangerous::listReverseInPlace(res.clone());
            res.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

pub fn getProtectedElementsInClass(mut inClass: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut outAbsynElementLst: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    outAbsynElementLst = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PROTECTED { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res = Dangerous::listReverseInPlace(res.clone());
            res.clone()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::List::Nil, .. }, .. } => {
            metamodelica::nil()
        },
        Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: lst, .. }, .. } => {
            let mut lst1: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
            let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res = metamodelica::nil();
            for mut elt in &*lst.clone() {
                let mut elt = elt.clone();
                let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ClassPart::PROTECTED { contents: elts } => {
            lst1 = getElementsInElementitems(elts.clone());
            res = List::append_reverse(lst1.clone(), res.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            res = Dangerous::listReverseInPlace(res.clone());
            res.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementLst
}

fn getElementsInElementitems(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut outAbsynElementLst: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    for mut el in &*inAbsynElementItemLst.clone() {
        let mut el = el.clone();
        let _ = (::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: elt } => {
            outAbsynElementLst = cons(elt.clone(), outAbsynElementLst.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outAbsynElementLst = Dangerous::listReverseInPlace(outAbsynElementLst.clone());
    outAbsynElementLst
}

pub fn dimensionListValues(mut dims: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    vals = {
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut d in (dims.clone()).into_iter().cloned() {
            let __x = ValuesMake::makeCodeTypeNameStr((Dump::printSubscriptStr(d.clone())?).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(vals)
}

fn getElementInfo(mut element: Arc<Absyn::Element>, mut isPublic: bool, mut quoteNames: bool, mut onlyComponents: bool, mut env: GraphicEnvCache, mut infos: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut infos: Arc<metamodelica::List<Arc<Values::Value>>> = infos;
    let mut attr: Absyn::ElementAttributes;
    let mut ty: Arc<Absyn::Path>;
    let mut cc_path: Arc<Absyn::Path>;
    let mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut cmt: ArcStr = arcstr::literal!("");
    let mut common_info: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut info: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut cls: Arc<Absyn::Class>;
    let mut restriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    let mut opt_cmt: Option<Arc<Absyn::Comment>> = None;
    let mut opt_cc: Option<Arc<Absyn::ConstrainClass>> = None;
    let mut opt_adim: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> = None;
    let mut common_dims: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut dims: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut dims_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    infos = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { constrainClass: opt_cc, specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: comps, typeSpec: Deref @ Absyn::TypeSpec::TPATH { arrayDim: opt_adim, path: ty }, attributes: attr }, .. } => {
            let mut ty = (*ty).clone();
            common_info = metamodelica::nil();
            ty = qualifyPath(env.clone(), ty.clone(), false)?;
            common_dims = dimensionListValues(attr.arrayDim.clone())?;
            if !(onlyComponents.clone()) {
                cc_path = getConstrainClassPath(env.clone(), opt_cc.clone())?;
                if quoteNames.clone() {
                    common_info = cons(ValuesMake::makeString((AbsynUtil::pathString(cc_path.clone(), (literal!(".")).clone(), true, false)?).clone()), common_info.clone());
                } else {
                    common_info = cons(ValuesMake::makeCodeTypeName(getConstrainClassPath(env.clone(), opt_cc.clone())?), common_info.clone());
                }
            }
            common_info = getElementAttributeValues(element.clone(), isPublic.clone(), quoteNames.clone(), common_info.clone())?;
            for mut comp in &*comps.clone().reverse() {
                let mut comp = comp.clone();
                name = (comp.component.name.clone()).clone();
                cmt = (getComponentComment(comp.clone(), element.clone())).clone();
                dims = dimensionListValues(comp.component.arrayDim.clone())?;
                dims_val = ValuesMake::makeArray(listAppend(dims.clone(), common_dims.clone()))?;
                if quoteNames.clone() {
                    dims_val = ValuesMake::makeString((ValuesDump::printValStr(dims_val.clone())?).clone());
                }
                info = List::appendElt(dims_val.clone(), common_info.clone());
                info = cons(ValuesMake::makeString((cmt.clone()).clone()), info.clone());
                if quoteNames.clone() {
                    info = cons(ValuesMake::makeString((name.clone()).clone()), info.clone());
                    info = cons(ValuesMake::makeString((AbsynUtil::pathString(ty.clone(), (literal!(".")).clone(), true, false)?).clone()), info.clone());
                } else {
                    info = cons(ValuesMake::makeCodeTypeNameStr((name.clone()).clone()), info.clone());
                    info = cons(ValuesMake::makeCodeTypeName(ty.clone()), info.clone());
                }
                if !(onlyComponents.clone()) {
                    info = cons(ValuesMake::makeString((literal!("co")).clone()), cons(ValuesMake::makeString((literal!("-")).clone()), info.clone()));
                }
                infos = cons(ValuesMake::makeArray(info.clone())?, infos.clone());
            }
            infos.clone()
        },
        Deref @ Absyn::Element::ELEMENT { constrainClass: opt_cc, specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: cls @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { comment: opt_cmt, attributes: attr, typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: ty, arrayDim: opt_adim }, .. }, restriction, name, .. }, .. }, .. } if (!(onlyComponents.clone())) => {
            let mut ty = (*ty).clone();
            ty = qualifyPath(env.clone(), ty.clone(), false)?;
            cmt = (getConstrainingClassComment(opt_cc.clone())).clone();
            if stringEmpty((cmt.clone()).clone()) {
                cmt = (getClassCommentInCommentOpt(opt_cmt.clone())).clone();
            }
            dims = if (isSome(opt_adim.clone())) {dimensionListValues(Util::getOption(opt_adim.clone())?)?} else {metamodelica::nil()};
            dims = listAppend(dimensionListValues(attr.arrayDim.clone())?, dims.clone());
            dims_val = ValuesMake::makeArray(dims.clone())?;
            if quoteNames.clone() {
                dims_val = ValuesMake::makeString((ValuesDump::printValStr(dims_val.clone())?).clone());
            }
            info = list![dims_val.clone()];
            info = cons(ValuesMake::makeCodeTypeName(getConstrainClassPath(env.clone(), opt_cc.clone())?), info.clone());
            info = getElementAttributeValues(element.clone(), isPublic.clone(), quoteNames.clone(), info.clone())?;
            info = cons(ValuesMake::makeString((cmt.clone()).clone()), info.clone());
            info = cons(ValuesMake::makeCodeTypeNameStr((name.clone()).clone()), info.clone());
            info = cons(ValuesMake::makeCodeTypeName(ty.clone()), info.clone());
            info = cons(ValuesMake::makeString((Dump::unparseRestrictionStr(restriction.clone())?).clone()), info.clone());
            info = cons(ValuesMake::makeString((literal!("cl")).clone()), info.clone());
            infos = cons(ValuesMake::makeArray(info.clone())?, infos.clone());
            infos.clone()
        },
        _ => infos.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(infos)
}

pub fn getElementAttributeValues(mut element: Arc<Absyn::Element>, mut isPublic: bool, mut quoteNames: bool, mut attrValues: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut attrValues: Arc<metamodelica::List<Arc<Values::Value>>> = attrValues;
    let mut attr: Absyn::ElementAttributes;
    let mut vis_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut final_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut flow_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut stream_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut repl_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut var_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut io_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut dir_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    attrValues = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            attr = (::match_deref::match_deref! { match &(var_field!((*element).specification, Absyn::Element::ELEMENT).clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { attributes: attr, .. }, .. }, .. } => attr.clone(),
        Deref @ Absyn::ElementSpec::COMPONENTS { attributes: attr, .. } => attr.clone(),
        _ => bail!("match: no arm matched"),
    } });
            attrValues = cons(ValuesMake::makeString((attrVariabilityStr(attr.clone())?).clone()), cons(ValuesMake::makeString((innerOuterStr(var_field!((*element).innerOuter, Absyn::Element::ELEMENT).clone())?).clone()), cons(ValuesMake::makeString((attrDirectionStr(attr.clone())?).clone()), attrValues.clone())));
            if quoteNames.clone() {
                attrValues = cons(ValuesMake::makeString(ArcStr::from(::std::format!("{}", var_field!((*element).finalPrefix, Absyn::Element::ELEMENT).clone()))), cons(ValuesMake::makeString(ArcStr::from(::std::format!("{}", attr.flowPrefix.clone()))), cons(ValuesMake::makeString(ArcStr::from(::std::format!("{}", attr.streamPrefix.clone()))), cons(ValuesMake::makeString(ArcStr::from(::std::format!("{}", AbsynUtil::isElementReplaceable(element.clone())))), attrValues.clone()))));
            } else {
                attrValues = cons(ValuesMake::makeBoolean(var_field!((*element).finalPrefix, Absyn::Element::ELEMENT).clone()), cons(ValuesMake::makeBoolean(attr.flowPrefix.clone()), cons(ValuesMake::makeBoolean(attr.streamPrefix.clone()), cons(ValuesMake::makeBoolean(AbsynUtil::isElementReplaceable(element.clone())), attrValues.clone()))));
            }
            attrValues = cons(ValuesMake::makeString((if (isPublic.clone()) {literal!("public")} else {literal!("protected")}).clone()), attrValues.clone());
            attrValues.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(attrValues)
}

pub fn qualifyPath(mut inEnv: GraphicEnvCache, mut inPath: Arc<Absyn::Path>, mut failOnError: bool) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut n: ArcStr = arcstr::literal!("");
    outPath = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::FULLYQUALIFIED { path: _ } => inPath.clone(),
        Deref @ Absyn::Path::IDENT { name: Deref @ "Real" } => inPath.clone(),
        Deref @ Absyn::Path::IDENT { name: Deref @ "Integer" } => inPath.clone(),
        Deref @ Absyn::Path::IDENT { name: Deref @ "Boolean" } => inPath.clone(),
        Deref @ Absyn::Path::IDENT { name: Deref @ "String" } => inPath.clone(),
        _ => {
            match '__try0: {
                if unwrap_break_err!(Flags::isSet(Flags::NF_API.clone()), '__try0) {
                    (_, outPath) = unwrap_break_err!(Interactive::mkFullyQual(inEnv.clone(), inPath.clone(), failOnError.clone()), '__try0);
                } else {
                    outPath = unwrap_break_err!(qualifyType(Interactive::envFromGraphicEnvCache(inEnv.clone())?, inPath.clone()), '__try0);
                }
                Ok::<_, anyhow::Error>((outPath.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    outPath = __try0_o0;
                }
                Err(_) => {
                    if failOnError.clone() {
                        bail!("fail");
                    } else {
                        outPath = inPath.clone();
                    }
                }
            }
            outPath.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

pub fn getConstrainClassPath(mut inEnv: GraphicEnvCache, mut occ: Option<Arc<Absyn::ConstrainClass>>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = 'mc: {
        let __mc_input = occ.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::ConstrainClass { elementSpec: Deref @ Absyn::ElementSpec::EXTENDS { path, .. }, .. }) => {
                    Ok(qualifyPath(inEnv.clone(), path.clone(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(Arc::new(Absyn::Path::IDENT { name: (literal!("$Any")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(path)
}

pub fn qualifyType(mut inEnv: FCore::Graph, mut p: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut fqp: Arc<Absyn::Path> = p.clone();
    let mut oenv_path: Option<Arc<Absyn::Path>> = None;
    let mut env_path: Arc<Absyn::Path>;
    let mut tp_path: Arc<Absyn::Path>;
    let mut pkg_path: Arc<Absyn::Path>;
    let mut tp_name: ArcStr = arcstr::literal!("");
    let mut env: FCore::Graph;
    if AbsynUtil::pathIsFullyQualified(p.clone()) {
        return Ok(fqp);
    }
    fqp = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut oenv_path: Option<Arc<Absyn::Path>> = oenv_path.clone();
            let mut env: FCore::Graph;
            let mut env_path: Arc<Absyn::Path>;
            let mut tp_name: ArcStr = tp_name.clone();
            let mut tp_path: Arc<Absyn::Path>;
            (_, _, env) = Lookup::lookupClass(FCore::emptyCache(), inEnv.clone(), p.clone(), None)?;
            oenv_path = FGraph::getScopePath(env.clone())?;
            if isSome(oenv_path.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oenv_path.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                env_path = __pa0.clone();
                tp_name = (AbsynUtil::pathLastIdent(p.clone())?).clone();
                tp_path = AbsynUtil::suffixPath(env_path.clone(), (tp_name.clone()).clone())?;
            } else {
                tp_path = p.clone();
            }
            Ok(tp_path.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            let mut env: FCore::Graph;
            let mut oenv_path: Option<Arc<Absyn::Path>> = oenv_path.clone();
            let mut pkg_path: Arc<Absyn::Path>;
            let mut env_path: Arc<Absyn::Path>;
            let mut tp_path: Arc<Absyn::Path>;
            pkg_path = AbsynUtil::pathFirstPath(p.clone())?;
            (_, _, env) = Lookup::lookupClass(FCore::emptyCache(), inEnv.clone(), pkg_path.clone(), None)?;
            oenv_path = FGraph::getScopePath(env.clone())?;
            if isSome(oenv_path.clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(oenv_path.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                env_path = __pa0.clone();
                tp_path = AbsynUtil::joinPaths(env_path.clone(), p.clone())?;
            } else {
                tp_path = p.clone();
            }
            Ok(tp_path.clone())
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(p.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(fqp)
}

pub fn getElementsInfo(mut elements: Arc<metamodelica::List<Arc<Absyn::Element>>>, mut isPublic: bool, mut useQuotes: bool, mut onlyComponents: bool, mut env: Interactive::GraphicEnvCache, mut infos: Arc<metamodelica::List<Arc<Values::Value>>>) -> Result<Arc<metamodelica::List<Arc<Values::Value>>>> {
    let mut infos: Arc<metamodelica::List<Arc<Values::Value>>> = infos;
    for mut elem in &*elements.clone().reverse() {
        let mut elem = elem.clone();
        infos = getElementInfo(elem.clone(), isPublic.clone(), useQuotes.clone(), onlyComponents.clone(), env.clone(), infos.clone())?;
    }
    Ok(infos)
}

pub fn keywordReplaceable(mut inAbsynRedeclareKeywordsOption: Option<Absyn::RedeclareKeywords>) -> bool {
    let mut outBoolean: bool = false;
    outBoolean = (match inAbsynRedeclareKeywordsOption.clone() {
        Some(Absyn::RedeclareKeywords::REPLACEABLE) => true,
        Some(Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE) => true,
        _ => false,
    });
    outBoolean
}

pub fn innerOuterStr(mut inInnerOuter: Absyn::InnerOuter) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inInnerOuter.clone() {
        Absyn::InnerOuter::INNER => literal!("inner"),
        Absyn::InnerOuter::OUTER => literal!("outer"),
        Absyn::InnerOuter::NOT_INNER_OUTER => literal!("none"),
        Absyn::InnerOuter::INNER_OUTER => literal!("innerouter"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn attrFlowStr(mut inElementAttributes: Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inElementAttributes.clone() {
        Absyn::ElementAttributes { flowPrefix: mut f, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (boolString(f.clone())).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn attrStreamStr(mut inElementAttributes: Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inElementAttributes.clone() {
        Absyn::ElementAttributes { streamPrefix: mut s, .. } => {
            let mut res: ArcStr = arcstr::literal!("");
            res = (boolString(s.clone())).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn attrParallelismStr(mut inElementAttributes: Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inElementAttributes.clone() {
        Absyn::ElementAttributes { parallelism: Absyn::Parallelism::PARGLOBAL, .. } => literal!("parglobal"),
        Absyn::ElementAttributes { parallelism: Absyn::Parallelism::PARLOCAL, .. } => literal!("parlocal"),
        Absyn::ElementAttributes { parallelism: Absyn::Parallelism::NON_PARALLEL, .. } => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn attrVariabilityStr(mut inElementAttributes: Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inElementAttributes.clone() {
        Absyn::ElementAttributes { variability: Absyn::Variability::VAR, .. } => literal!("unspecified"),
        Absyn::ElementAttributes { variability: Absyn::Variability::DISCRETE, .. } => literal!("discrete"),
        Absyn::ElementAttributes { variability: Absyn::Variability::PARAM, .. } => literal!("parameter"),
        Absyn::ElementAttributes { variability: Absyn::Variability::CONST, .. } => literal!("constant"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn attrDirectionStr(mut inElementAttributes: Absyn::ElementAttributes) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inElementAttributes.clone() {
        Absyn::ElementAttributes { direction: Absyn::Direction::INPUT, .. } => literal!("input"),
        Absyn::ElementAttributes { direction: Absyn::Direction::OUTPUT, .. } => literal!("output"),
        Absyn::ElementAttributes { direction: Absyn::Direction::BIDIR, .. } => literal!("unspecified"),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn getConstrainingClassComment(mut constrainingClass: Option<Arc<Absyn::ConstrainClass>>) -> ArcStr {
    let mut comment: ArcStr = arcstr::literal!("");
    comment = ((::match_deref::match_deref! { match &(constrainingClass.clone()) {
        Some(Deref @ Absyn::ConstrainClass { comment: Some(Deref @ Absyn::Comment { comment: Some(comment), .. }), .. }) => comment.clone(),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    comment
}

pub fn getComponentComment(mut component: Arc<Absyn::ComponentItem>, mut element: Arc<Absyn::Element>) -> ArcStr {
    let mut comment: ArcStr = arcstr::literal!("");
    comment = (getConstrainingClassComment(AbsynUtil::getElementConstrainingClass(element.clone()))).clone();
    if stringEmpty((comment.clone()).clone()) {
        comment = (getClassCommentInCommentOpt(component.comment.clone())).clone();
    }
    comment
}

pub fn getComponentItemsNameAndComment(mut inComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inElement: Arc<Absyn::Element>) -> Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> {
    let mut outStrings: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let mut cmt_str: ArcStr = arcstr::literal!("");
    for mut comp in &*inComponents.clone().reverse() {
        let mut comp = comp.clone();
        let _ = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name, .. }, .. } => {
            cmt_str = (getComponentComment(comp.clone(), inElement.clone())).clone();
            cmt_str = (StringUtil::quote((cmt_str.clone()).clone())).clone();
            outStrings = cons(list![(name.clone()).clone(), (cmt_str.clone()).clone()], outStrings.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outStrings
}

pub fn getComponentItemsName(mut inComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inQuoteNames: bool) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    for mut comp in &*inComponents.clone().reverse() {
        let mut comp = comp.clone();
        let _ = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Absyn::ComponentItem { component: Absyn::Component { name, .. }, .. } => {
            outStrings = cons(if (inQuoteNames.clone()) {stringAppendList(list![(literal!("\"")).clone(), (name.clone()).clone(), (literal!("\"")).clone()])} else {stringAppendList(list![(name.clone()).clone()])}, outStrings.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outStrings
}

fn replaceClassInProgram2(mut inClass: Arc<Absyn::Class>, mut inClassName: ArcStr) -> Result<bool> {
    let mut outReplace: bool = false;
    let mut cls_name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls_name = __pa0.clone();
    outReplace = cls_name.clone() == inClassName.clone();
    Ok(outReplace)
}

pub fn replaceClassInProgram(mut inClass: Arc<Absyn::Class>, mut inProgram: Absyn::Program, mut mergeAST: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut cls_name1: ArcStr = arcstr::literal!("");
    let mut cls_name2: ArcStr = arcstr::literal!("");
    let mut clst: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut clsFilter: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut w: Absyn::Within = Absyn::Within::TOP;
    let mut replaced: bool = false;
    let mut cls: Arc<Absyn::Class>;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls_name1 = __pa0.clone();
    let Absyn::PROGRAM { within_: __pa1, classes: __pa2 } = (inProgram.clone()) else { bail!("pattern mismatch") };
    w = __pa1.clone();
    clst = __pa2.clone();
    if mergeAST.clone() {
        clsFilter = List::filterOnTrue(clst.clone(), Arc::new({ let __pe_b1 = (cls_name1.clone()).clone(); move |__pe_a0| replaceClassInProgram2(__pe_a0, __pe_b1.clone()) }));
        if clsFilter.clone().is_empty() {
            cls = inClass.clone();
        } else {
            let __pa3 = ::match_deref::match_deref! { match &(clsFilter.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa3, tail: _ } => __pa3.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cls = __pa3.clone();
            cls = mergeClasses(inClass.clone(), cls.clone())?;
        }
    } else {
        cls = inClass.clone();
    }
    (clst, replaced) = List::replaceOnTrue(cls.clone(), clst.clone(), Arc::new({ let __pe_b1 = (cls_name1.clone()).clone(); move |__pe_a0| replaceClassInProgram2(__pe_a0, __pe_b1.clone()) }))?;
    if !(replaced.clone()) {
        clst = List::appendElt(inClass.clone(), clst.clone());
    }
    outProgram = Absyn::Program { classes: clst.clone(), within_: w.clone() };
    Ok(outProgram)
}

pub fn insertClassInProgram(mut inClass: Arc<Absyn::Class>, mut inWithin: Absyn::Within, mut inProgram: Absyn::Program, mut mergeAST: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    outProgram = 'mc: {
        let __mc_input = (inClass.clone(), inWithin.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, w @ Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { name: n1, .. } }, p @ Absyn::Program { .. }) => {
                    let mut c2: Arc<Absyn::Class>;
                    let mut c3: Arc<Absyn::Class>;
                    let mut pnew: Absyn::Program;
                    c2 = getClassInProgram((n1.clone()).clone(), p.clone())?;
                    c3 = insertClassInClass(c1.clone(), w.clone(), c2.clone(), mergeAST.clone())?;
                    pnew = updateProgram(Absyn::Program { classes: list![c3.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), mergeAST.clone())?;
                    Ok(pnew.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, w @ Absyn::Within::WITHIN { path: Deref @ Absyn::Path::IDENT { name: n1 } }, p @ Absyn::Program { .. }) => {
                    let mut c2: Arc<Absyn::Class>;
                    let mut c3: Arc<Absyn::Class>;
                    let mut pnew: Absyn::Program;
                    c2 = getClassInProgram((n1.clone()).clone(), p.clone())?;
                    c3 = insertClassInClass(c1.clone(), w.clone(), c2.clone(), mergeAST.clone())?;
                    pnew = updateProgram(Absyn::Program { classes: list![c3.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), mergeAST.clone())?;
                    Ok(pnew.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "OpenModelica", .. } }, p) => {
                    Ok(p.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name, .. }, w, p) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    let mut s2: ArcStr = arcstr::literal!("");
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    s1 = (Dump::unparseWithin(w.clone())?).clone();
                    (_, paths) = getClassNamesRecursive(None, p.clone(), false, false, metamodelica::nil())?;
                    s2 = stringAppendList(List::map1r({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
                    let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }, Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)), (literal!("\n  ")).clone()));
                    Error::addMessage(Error::INSERT_CLASS.clone(), list![(name.clone()).clone(), (s1.clone()).clone(), (s2.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outProgram)
}

fn insertClassInClass(mut inClass1: Arc<Absyn::Class>, mut inWithin2: Absyn::Within, mut inClass3: Arc<Absyn::Class>, mut mergeAST: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &((inClass1.clone(), inWithin2.clone(), inClass3.clone())) {
        (c1, Absyn::Within::WITHIN { path: Deref @ Absyn::Path::IDENT { .. } }, c2) => {
            replaceInnerClass(c1.clone(), c2.clone(), mergeAST.clone())?
        },
        (c1, Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { path, .. } }, c2) => {
            let mut cnew: Arc<Absyn::Class>;
            let mut cinner: Arc<Absyn::Class>;
            let mut name2: ArcStr = arcstr::literal!("");
            name2 = (AbsynUtil::pathFirstIdent(path.clone())?).clone();
            cinner = getInnerClass(c2.clone(), (name2.clone()).clone())?;
            cnew = insertClassInClass(c1.clone(), Absyn::Within::WITHIN { path: path.clone() }, cinner.clone(), mergeAST.clone())?;
            replaceInnerClass(cnew.clone(), c2.clone(), mergeAST.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

fn classElementItemIsNamed(mut inClassName: ArcStr, mut inElement: Arc<Absyn::ElementItem>) -> bool {
    let mut outIsNamed: bool = false;
    outIsNamed = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name, .. }, .. }, .. } } => {
            inClassName.clone() == name.clone()
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

fn replaceInnerClass(mut inClass1: Arc<Absyn::Class>, mut inClass2: Arc<Absyn::Class>, mut mergeAST: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = (inClass1.clone(), inClass2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(parts.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(publst.clone(), c1.clone(), mergeAST.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    publst2 = __pa0.clone();
                    parts2 = replacePublicList(parts.clone(), publst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }) => {
                    let mut prolst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut prolst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    prolst = getProtectedList(parts.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(prolst.clone(), c1.clone(), mergeAST.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    prolst2 = __pa0.clone();
                    parts2 = replaceProtectedList(parts.clone(), prolst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(parts.clone());
                    publst = addClassInElementitemlist(publst.clone(), c1.clone())?;
                    parts2 = replacePublicList(parts.clone(), publst.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: modif, baseClassName: bcname }, .. }) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(parts.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(publst.clone(), c1.clone(), mergeAST.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    publst2 = __pa0.clone();
                    parts2 = replacePublicList(parts.clone(), publst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: modif, baseClassName: bcname }, .. }) => {
                    let mut prolst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut prolst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    prolst = getProtectedList(parts.clone());
                    let __pa0 = ::match_deref::match_deref! { match &(replaceClassInElementitemlist(prolst.clone(), c1.clone(), mergeAST.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    prolst2 = __pa0.clone();
                    parts2 = replaceProtectedList(parts.clone(), prolst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: modif, baseClassName: bcname }, .. }) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(parts.clone());
                    publst = addClassInElementitemlist(publst.clone(), c1.clone())?;
                    parts2 = replacePublicList(parts.clone(), publst.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf((literal!("Failed in replaceInnerClass\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

fn replaceClassInElementitemlist(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inClass: Arc<Absyn::Class>, mut mergeAST: bool) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, bool)> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut replaced: bool = false;
    (outAbsynElementItemLst, replaced) = (::match_deref::match_deref! { match &((inAbsynElementItemLst.clone(), inClass.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { constrainClass: h, specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: c1 @ Deref @ Absyn::Class { name: name1, .. }, replaceable_: e }, innerOuter: io, redeclareKeywords: b, finalPrefix: a, .. } }, tail: xs }, c2 @ Deref @ Absyn::Class { name, .. }) if (stringEq((name1.clone()).clone(), (name.clone()).clone())?) => {
            let mut c: Arc<Absyn::Class>;
            let mut info: SourceInfo;
            c = if (mergeAST.clone()) {mergeClasses(c2.clone(), c1.clone())?} else {c2.clone()};
            let __pa0 = ::match_deref::match_deref! { match &(c.clone()) {
                Deref @ Absyn::Class { info: __pa0, .. } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            info = __pa0.clone();
            (cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: a.clone(), redeclareKeywords: b.clone(), innerOuter: io.clone(), specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: e.clone(), class_: c.clone() }), info: info.clone(), constrainClass: h.clone() }) }), xs.clone()), true)
        },
        (Deref @ metamodelica::List::Cons { head: e1, tail: xs }, c) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            (res, replaced) = replaceClassInElementitemlist(xs.clone(), c.clone(), mergeAST.clone())?;
            (cons(e1.clone(), res.clone()), replaced.clone())
        },
        _ => {
            (metamodelica::nil(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outAbsynElementItemLst, replaced))
}

fn addClassInElementitemlist(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut info: SourceInfo;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { info: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    outAbsynElementItemLst = listAppend(inAbsynElementItemLst.clone(), list![Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: false, class_: inClass.clone() }), info: info.clone(), constrainClass: None }) })]);
    Ok(outAbsynElementItemLst)
}

fn getInnerClass(mut inClass: Arc<Absyn::Class>, mut inIdent: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = (inClass.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, name) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut c1: Arc<Absyn::Class>;
                    publst = getPublicList(parts.clone());
                    c1 = getClassFromElementitemlist(publst.clone(), (name.clone()).clone())?;
                    Ok(c1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, name) => {
                    let mut prolst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut c1: Arc<Absyn::Class>;
                    prolst = getProtectedList(parts.clone());
                    c1 = getClassFromElementitemlist(prolst.clone(), (name.clone()).clone())?;
                    Ok(c1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, name) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut c1: Arc<Absyn::Class>;
                    publst = getPublicList(parts.clone());
                    c1 = getClassFromElementitemlist(publst.clone(), (name.clone()).clone())?;
                    Ok(c1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, name) => {
                    let mut prolst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut c1: Arc<Absyn::Class>;
                    prolst = getProtectedList(parts.clone());
                    c1 = getClassFromElementitemlist(prolst.clone(), (name.clone()).clone())?;
                    Ok(c1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

pub fn replacePublicList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outAbsynClassPartLst = (::match_deref::match_deref! { match &((inAbsynClassPartLst.clone(), inAbsynElementItemLst.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { .. }, tail: rest }, newpublst) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            rest_1 = deletePublicList(rest.clone());
            cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: newpublst.clone() }), rest_1.clone())
        },
        (Deref @ metamodelica::List::Cons { head: x, tail: xs }, new) => {
            let mut ys: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            ys = replacePublicList(xs.clone(), new.clone())?;
            cons(x.clone(), ys.clone())
        },
        (Deref @ metamodelica::List::Nil, newpublist) => {
            list![Arc::new(Absyn::ClassPart::PUBLIC { contents: newpublist.clone() })]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAbsynClassPartLst)
}

pub fn replaceProtectedList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outAbsynClassPartLst = (::match_deref::match_deref! { match &((inAbsynClassPartLst.clone(), inAbsynElementItemLst.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { .. }, tail: rest }, newprotlist) => {
            let mut rest_1: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            rest_1 = deleteProtectedList(rest.clone());
            cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: newprotlist.clone() }), rest_1.clone())
        },
        (Deref @ metamodelica::List::Cons { head: x, tail: xs }, new) => {
            let mut ys: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            ys = replaceProtectedList(xs.clone(), new.clone())?;
            cons(x.clone(), ys.clone())
        },
        (Deref @ metamodelica::List::Nil, newprotlist) => {
            list![Arc::new(Absyn::ClassPart::PROTECTED { contents: newprotlist.clone() })]
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAbsynClassPartLst)
}

pub fn replaceEquationList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outAbsynClassPartLst = (::match_deref::match_deref! { match &((inAbsynClassPartLst.clone(), inAbsynEquationItemLst.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { .. }, tail: rest }, newequationlst) => {
            cons(Arc::new(Absyn::ClassPart::EQUATIONS { contents: newequationlst.clone() }), rest.clone())
        },
        (Deref @ metamodelica::List::Cons { head: x, tail: xs }, new) => {
            let mut ys: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            ys = replaceEquationList(xs.clone(), new.clone())?;
            cons(x.clone(), ys.clone())
        },
        (Deref @ metamodelica::List::Nil, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAbsynClassPartLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deletePublicList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
    let mut outAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outAbsynClassPartLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { .. }, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            res = deletePublicList(xs.clone());
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            res = deletePublicList(xs.clone());
            cons(x.clone(), res.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynClassPartLst
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn deleteProtectedList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
    let mut outAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outAbsynClassPartLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { .. }, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            res = deleteProtectedList(xs.clone());
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: x, tail: xs } => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            res = deleteProtectedList(xs.clone());
            cons(x.clone(), res.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynClassPartLst
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getPublicList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outAbsynElementItemLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: res1 }, tail: rest } => {
            let mut res2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res2 = getPublicList(rest.clone());
            res = listAppend(res1.clone(), res2.clone());
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut ys: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            ys = getPublicList(xs.clone());
            ys.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementItemLst
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getProtectedList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outAbsynElementItemLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: res1 }, tail: rest } => {
            let mut res2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut res: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            res2 = getProtectedList(rest.clone());
            res = listAppend(res1.clone(), res2.clone());
            res.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut ys: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            ys = getProtectedList(xs.clone());
            ys.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outAbsynElementItemLst
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getEquationList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outAbsynEquationItemLst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    outAbsynEquationItemLst = (::match_deref::match_deref! { match &(inAbsynClassPartLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::EQUATIONS { contents: lst }, tail: _ } => {
            lst.clone()
        },
        Deref @ metamodelica::List::Cons { head: _, tail: xs } => {
            let mut ys: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            ys = getEquationList(xs.clone())?;
            ys.clone()
        },
        _ => {
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAbsynEquationItemLst)
}

fn getClassFromElementitemlist(mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inIdent: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    let mut elem: Arc<Absyn::ElementItem>;
    elem = List::getMemberOnTrue((inIdent.clone()).clone(), inElements.clone(), Arc::new(fnptr!(classElementItemIsNamed, ArcStr, Arc<Absyn::ElementItem>)))?;
    let __pa0 = ::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outClass = __pa0.clone();
    Ok(outClass)
}

fn classInProgram(mut name: ArcStr, mut p: Absyn::Program) -> Result<bool> {
    let mut b: bool = false;
    b = (match p.clone() {
        Absyn::Program { .. } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            for mut cl in &*p.classes.clone() {
                let mut cl = cl.clone();
                let __pa0 = ::match_deref::match_deref! { match &(cl.clone()) {
                    Deref @ Absyn::Class { name: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                r#str = __pa0.clone();
                if r#str.clone() == name.clone() {
                    b = true;
                    return Ok(b);
                }
            }
            false
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(b)
}

pub fn getPathedClassInProgram(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut enclOnErr: bool, mut showError: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok(getPathedClassInProgramWork(inPath.clone(), inProgram.clone(), enclOnErr.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok(getPathedClassInProgramWork(inPath.clone(), (FBuiltin::getInitialFunctions()?).0, enclOnErr.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            if showError.clone() {
                Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(AbsynUtil::pathString(inPath.clone(), (literal!(".")).clone(), true, false)?).clone(), (literal!("<TOP>")).clone()])?;
            }
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPathedClassInProgramWork(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut enclOnErr: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            getClassInProgram((var_field!((*inPath).name, Absyn::Path::IDENT).clone()).clone(), inProgram.clone())?
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            let mut c: Arc<Absyn::Class>;
            c = getClassInProgram((var_field!((*inPath).name, Absyn::Path::QUALIFIED).clone()).clone(), inProgram.clone())?;
            getPathedClassInClass(var_field!((*inPath).path, Absyn::Path::QUALIFIED).clone(), c.clone(), enclOnErr.clone())?
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
            getPathedClassInProgramWork(var_field!((*inPath).path, Absyn::Path::FULLYQUALIFIED).clone(), inProgram.clone(), enclOnErr.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPathedClassInClass(mut inPath: Arc<Absyn::Path>, mut inClass: Arc<Absyn::Class>, mut enclOnError: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = inPath.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { name: r#str } => {
                    Ok(getClassInClass((r#str.clone()).clone(), inClass.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::FULLYQUALIFIED { path } => {
                    Ok(getPathedClassInClass(path.clone(), inClass.clone(), enclOnError.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { path, name: r#str } => {
                    let mut c: Arc<Absyn::Class>;
                    c = getClassInClass((r#str.clone()).clone(), inClass.clone())?;
                    Ok(getPathedClassInClass(path.clone(), c.clone(), enclOnError.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if !((enclOnError.clone())) { bail!("guard") }
                    Ok(inClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

fn getClassInClass(mut name: ArcStr, mut inClass: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    for mut part in &*AbsynUtil::getClassPartsInClass(inClass.clone()) {
        let mut part = part.clone();
        for mut item in &*AbsynUtil::getElementItemsInClassPart(part.clone()) {
            let mut item = item.clone();
            if AbsynUtil::isElementItemClassNamed((name.clone()).clone(), item.clone()) {
                outClass = AbsynUtil::elementItemClass(item.clone())?;
                return Ok(outClass);
            }
        }
    }
    bail!("fail");
    Ok(outClass)
}

pub fn getClassInProgram(mut name: ArcStr, mut program: Absyn::Program) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class>;
    cls = List::find(program.classes.clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); move |__pe_a1| Ok(AbsynUtil::isClassNamed(__pe_b0.clone(), __pe_a1)) }))?;
    Ok(cls)
}

pub fn annotationListToAbsyn(mut inAbsynNamedArgLst: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Result<Arc<Absyn::Annotation>> {
    let mut outAnnotation: Arc<Absyn::Annotation>;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    for mut arg in &*inAbsynNamedArgLst.clone() {
        let mut arg = arg.clone();
        args = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::NamedArg { argValue: e, argName: Deref @ "annotate" } => {
            let mut eltarg: Arc<Absyn::ElementArg>;
            eltarg = recordConstructorToModification(e.clone())?;
            cons(eltarg.clone(), args.clone())
        },
        Deref @ Absyn::NamedArg { argName: Deref @ "comment", .. } => {
            args.clone()
        },
        _ => {
            args.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outAnnotation = Arc::new(Absyn::Annotation { elementArgs: Dangerous::listReverseInPlace(args.clone()) });
    Ok(outAnnotation)
}

fn recordConstructorToModification(mut inExp: Arc<Absyn::Exp>) -> Result<Arc<Absyn::ElementArg>> {
    let mut outElementArg: Arc<Absyn::ElementArg>;
    outElementArg = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs, args: Deref @ metamodelica::List::Cons { head: e @ Deref @ Absyn::Exp::CALL { .. }, tail: Deref @ metamodelica::List::Nil } }, function_: cr, .. } => {
                    let mut eltarglst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut res: Arc<Absyn::ElementArg>;
                    let mut emod: Arc<Absyn::ElementArg>;
                    let mut p: Arc<Absyn::Path>;
                    eltarglst = List::map(nargs.clone(), Arc::new(namedargToModification));
                    emod = recordConstructorToModification(e.clone())?;
                    p = AbsynUtil::crefToPath(cr.clone())?;
                    res = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: p.clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: cons(emod.clone(), eltarglst.clone()), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: nargs, args: Deref @ metamodelica::List::Nil }, function_: cr, .. } => {
                    let mut eltarglst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut res: Arc<Absyn::ElementArg>;
                    let mut p: Arc<Absyn::Path>;
                    eltarglst = List::map(nargs.clone(), Arc::new(namedargToModification));
                    p = AbsynUtil::crefToPath(cr.clone())?;
                    res = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: p.clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: eltarglst.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { argNames: Deref @ metamodelica::List::Nil, args: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } }, function_: cr, .. } => {
                    let mut res: Arc<Absyn::ElementArg>;
                    let mut p: Arc<Absyn::Path>;
                    p = AbsynUtil::crefToPath(cr.clone())?;
                    res = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: p.clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: e.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf((literal!("InteractiveUtil.recordConstructorToModification failed, exp=")).clone())?;
                    Print::printBuf((Dump::printExpStr(inExp.clone())?).clone())?;
                    Print::printBuf((literal!("\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElementArg)
}

fn namedargToModification(mut inNamedArg: Arc<Absyn::NamedArg>) -> Result<Arc<Absyn::ElementArg>> {
    let mut outElementArg: Arc<Absyn::ElementArg>;
    outElementArg = 'mc: {
        let __mc_input = inNamedArg.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::NamedArg { argValue: c @ Deref @ Absyn::Exp::CALL { functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::List::Nil, .. }, .. }, argName: id } => {
                    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
                    let mut res: Arc<Absyn::ElementArg>;
                    let __pa0 = ::match_deref::match_deref! { match &(recordConstructorToModification(c.clone())?) {
                        Deref @ Absyn::ElementArg::MODIFICATION { comment: None, modification: Some(Deref @ Absyn::Modification { elementArgLst: __pa0, eqMod: _ }), .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    elts = __pa0.clone();
                    res = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: elts.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::NamedArg { argValue: e, argName: id } => {
                    let mut res: Arc<Absyn::ElementArg>;
                    res = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: e.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() });
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Print::printBuf((literal!("- InteractiveUtil.namedargToModification failed\n")).clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outElementArg)
}

fn getClassnamesInClassList(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClass: Arc<Absyn::Class>, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outString = (::match_deref::match_deref! { match &((inPath.clone(), inProgram.clone(), inClass.clone(), inShowProtected.clone(), includeConstants.clone())) {
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            strlist.clone()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            strlist.clone()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PDER { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outString)
}

fn joinPaths(mut child: ArcStr, mut parent: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((child.clone(), parent.clone())) {
        (c, r) => {
            let mut res: Arc<Absyn::Path>;
            res = AbsynUtil::joinPaths(r.clone(), Arc::new(Absyn::Path::IDENT { name: (c.clone()).clone() }))?;
            res.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPath)
}

pub fn getClassNamesRecursive(mut inPath: Option<Arc<Absyn::Path>>, mut inProgram: Absyn::Program, mut inShowProtected: bool, mut includeConstants: bool, mut inAcc: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Option<Arc<Absyn::Path>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> {
    let mut opath: Option<Arc<Absyn::Path>> = None;
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    (opath, paths) = 'mc: {
        let __mc_input = (inPath.clone(), inProgram.clone(), inShowProtected.clone(), includeConstants.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(pp), p, b, c, acc) => {
                    let mut cdef: Arc<Absyn::Class>;
                    let mut strlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut result_path_lst: Arc<metamodelica::List<Option<Arc<Absyn::Path>>>> = metamodelica::nil();
                    let mut acc = (*acc).clone();
                    acc = cons(pp.clone(), acc.clone());
                    cdef = getPathedClassInProgram(pp.clone(), p.clone(), false, false)?;
                    strlst = getClassnamesInClassList(pp.clone(), p.clone(), cdef.clone(), b.clone(), c.clone())?;
                    result_path_lst = List::map(List::map1(strlst.clone(), Arc::new(joinPaths), pp.clone()), Arc::new(fnptr!(Util::makeOption, _)));
                    (_, acc) = List::map3Fold(result_path_lst.clone(), Arc::new(getClassNamesRecursive), p.clone(), b.clone(), c.clone(), acc.clone());
                    Ok((inPath.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, p @ Absyn::Program { classes, .. }, b, c, acc) => {
                    let mut strlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut result_path_lst: Arc<metamodelica::List<Option<Arc<Absyn::Path>>>> = metamodelica::nil();
                    let mut acc = (*acc).clone();
                    strlst = List::map(classes.clone(), Arc::new(AbsynUtil::getClassName));
                    result_path_lst = List::mapMap(strlst.clone(), Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)), Arc::new(fnptr!(Util::makeOption, _)));
                    (_, acc) = List::map3Fold(result_path_lst.clone(), Arc::new(getClassNamesRecursive), p.clone(), b.clone(), c.clone(), acc.clone());
                    Ok((inPath.clone(), acc.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(pp), _, _, _, _) => {
                    let mut s1: ArcStr = arcstr::literal!("");
                    s1 = (AbsynUtil::pathString(pp.clone(), (literal!(".")).clone(), true, false)?).clone();
                    Error::addMessage(Error::LOOKUP_ERROR.clone(), list![(s1.clone()).clone(), (literal!("<TOP>")).clone()])?;
                    Ok((inPath.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((opath, paths))
}

pub fn getAllInheritedClasses(mut inClassName: Arc<Absyn::Path>, mut inProgram: Absyn::Program) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut outBaseClassNames: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut genv: GraphicEnvCache;
    outBaseClassNames = 'mc: {
        let __mc_input = (inClassName.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (p_class, p) => {
                    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut fqpaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut allPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    let mut cdef: Arc<Absyn::Class>;
                    let mut exts: Arc<metamodelica::List<Arc<Absyn::ElementSpec>>> = metamodelica::nil();
                    let mut genv: Interactive::GraphicEnvCache;
                    cdef = getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    exts = getExtendsElementspecInClass(cdef.clone())?;
                    paths = List::map(exts.clone(), Arc::new(getBaseClassNameFromExtends));
                    fqpaths = metamodelica::nil();
                    match '__try0: {
                        genv = unwrap_break_err!(createEnvironment(p.clone(), None, p_class.clone()), '__try0);
                        for mut pt in &*paths.clone() {
                            let mut pt = pt.clone();
                            fqpaths = cons(unwrap_break_err!(qualifyPath(genv.clone(), pt.clone(), false), '__try0), fqpaths.clone());
                        }
                        fqpaths = fqpaths.clone().reverse();
                        Ok::<_, anyhow::Error>((fqpaths.clone(),))
                    } {
                        Ok((__try0_o0,)) => {
                            fqpaths = __try0_o0;
                        }
                        Err(_) => {
                            fqpaths = paths.clone();
                        }
                    }
                    allPaths = metamodelica::nil();
                    for mut pt in &*fqpaths.clone() {
                        let mut pt = pt.clone();
                        allPaths = List::append_reverse(getAllInheritedClasses(pt.clone(), p.clone())?, allPaths.clone());
                    }
                    allPaths = Dangerous::listReverseInPlace(List::unique(allPaths.clone()));
                    Ok(listAppend(fqpaths.clone(), allPaths.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut allPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBaseClassNames)
}

pub fn getBaseClassNameFromExtends(mut inElementSpec: Arc<Absyn::ElementSpec>) -> Result<Arc<Absyn::Path>> {
    let mut outBaseClassPath: Arc<Absyn::Path>;
    outBaseClassPath = (::match_deref::match_deref! { match &(inElementSpec.clone()) {
        Deref @ Absyn::ElementSpec::EXTENDS { path, .. } => {
            path.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBaseClassPath)
}

fn mergeClasses(mut cNew: Arc<Absyn::Class>, mut cOld: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut c: Arc<Absyn::Class>;
    c = 'mc: {
        let __mc_input = (cNew.clone(), cOld.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: Deref @ metamodelica::List::Nil, .. }, .. }) => {
                    Ok(cNew.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts: Deref @ metamodelica::List::Nil, .. }, .. }) => {
                    Ok(cNew.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars: typeVars1, classAttrs: classAttrs1, classParts: partsC1, ann: ann1, comment: cmt1 }, .. }, Deref @ Absyn::Class { info: SourceInfo { fileName: file, .. }, body: Deref @ Absyn::ClassDef::PARTS { classParts: partsC2, .. }, .. }) => {
                    let mut pubElementsC1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut pubElementsC2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut c = (*c).clone();
                    let mut partsC1 = (*partsC1).clone();
                    pubElementsC2 = getPublicList(partsC2.clone());
                    pubElementsC2 = excludeElementsFromFile((file.clone()).clone(), pubElementsC2.clone())?;
                    pubElementsC1 = getPublicList(partsC1.clone());
                    pubElementsC1 = mergeElements(pubElementsC1.clone(), pubElementsC2.clone())?;
                    partsC1 = replacePublicList(partsC1.clone(), pubElementsC1.clone())?;
                    todo!("unhandled field-assign shape: c.body");
                    Ok(c.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok(cNew.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(c)
}

fn mergeElement(mut inEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inEl: Arc<Absyn::ElementItem>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outEls = 'mc: {
        let __mc_input = (inEls.clone(), inEl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(cons(inEl.clone(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: f, redeclareKeywords: redecl, innerOuter: innout, specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: r, class_: c1 @ Deref @ Absyn::Class { name: n1, .. } }, info: i, constrainClass: cc } }, tail: rest }, Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { replaceable_: _, class_: c2 @ Deref @ Absyn::Class { name: n2, .. } }, .. } }) => {
                    let mut c1 = (*c1).clone();
                    let true = (stringEqual((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    c1 = mergeClasses(c1.clone(), c2.clone())?;
                    Ok(cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: f.clone(), redeclareKeywords: redecl.clone(), innerOuter: innout.clone(), specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: r.clone(), class_: c1.clone() }), info: i.clone(), constrainClass: cc.clone() }) }), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: e1, tail: rest }, e2) => {
                    let mut filtered: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    filtered = mergeElement(rest.clone(), e2.clone())?;
                    Ok(cons(e1.clone(), filtered.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEls)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn mergeElements(mut inEls1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inEls2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outEls = 'mc: {
        let __mc_input = (inEls1.clone(), inEls2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    Ok(inEls2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Nil) => {
                    Ok(inEls1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::List::Cons { head: e2, tail: rest }) => {
                    let mut merged: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    merged = mergeElement(inEls1.clone(), e2.clone())?;
                    merged = mergeElements(merged.clone(), rest.clone())?;
                    Ok(merged.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outEls)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn excludeElementsFromFile(mut inFile: ArcStr, mut inEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outEls = (::match_deref::match_deref! { match &((inFile.clone(), inEls.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            let mut b: bool = false;
            metamodelica::nil()
        },
        (file, Deref @ metamodelica::List::Cons { head: e @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { info: SourceInfo { fileName: f, .. }, .. } }, tail: rest }) => {
            let mut filtered: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut b: bool = false;
            b = stringEqual((file.clone()).clone(), (f.clone()).clone());
            filtered = excludeElementsFromFile((file.clone()).clone(), rest.clone())?;
            if (!(b.clone())) {cons(e.clone(), filtered.clone())} else {filtered.clone()}
        },
        (file, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::LEXER_COMMENT { comment: _ }, tail: rest }) => {
            let mut filtered: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut b: bool = false;
            filtered = excludeElementsFromFile((file.clone()).clone(), rest.clone())?;
            filtered.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outEls)
}

pub mod ClassEntry {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClassEntry {
        pub path: Arc<Absyn::Path>,
        pub cls: Arc<Absyn::Class>,
    }

    pub type CLASS_ENTRY = ClassEntry;

    pub fn getPath(mut entry: Arc<ClassEntry>) -> Arc<Absyn::Path> {
        let mut path: Arc<Absyn::Path> = entry.path.clone();
        path
    }

    pub fn greaterEq(mut entry1: Arc<ClassEntry>, mut entry2: Arc<ClassEntry>) -> bool {
        let mut res: bool = AbsynUtil::pathGe(entry1.path.clone(), entry2.path.clone()).unwrap();
        res
    }

    pub fn equal(mut entry1: Arc<ClassEntry>, mut entry2: Arc<ClassEntry>) -> bool {
        let mut res: bool = referenceEq(&entry1.cls.clone(),&entry2.cls.clone());
        res
    }

}

pub fn getAllSubtypeOf(mut baseClass: Arc<Absyn::Path>, mut parentClass: Arc<Absyn::Path>, mut program: Absyn::Program, mut includePartial: bool, mut sort: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut classes: Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>> = metamodelica::nil();
    classes = getAllSubtypeOf2(baseClass.clone(), parentClass.clone(), program.clone(), includePartial.clone(), sort.clone())?;
    paths = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut c in (classes.clone()).into_iter().cloned() {
            let __x = ClassEntry::getPath(c.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(paths)
}

pub fn getReplaceableChoices(mut baseClass: Arc<Absyn::Path>, mut parentClass: Arc<Absyn::Path>, mut program: Absyn::Program, mut includePartial: bool, mut sort: bool) -> Result<Arc<Values::Value>> {
    let mut res: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut classes: Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>> = metamodelica::nil();
    let mut vals: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
    let mut name_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut cmt_val: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    classes = getAllSubtypeOf2(baseClass.clone(), parentClass.clone(), program.clone(), includePartial.clone(), sort.clone())?;
    for mut entry in &*classes.clone() {
        let mut entry = entry.clone();
        name_val = ValuesMake::makeString((AbsynUtil::pathString(entry.path.clone(), (literal!(".")).clone(), true, false)?).clone());
        cmt_val = ValuesMake::makeString((AbsynUtil::classDefStringComment(entry.cls.body.clone())).clone());
        vals = cons(ValuesMake::makeArray(list![name_val.clone(), cmt_val.clone()])?, vals.clone());
    }
    res = ValuesMake::makeArray(Dangerous::listReverseInPlace(vals.clone()))?;
    Ok(res)
}

fn getAllSubtypeOfCandidates(mut path: Arc<Absyn::Path>, mut parentClass: Arc<Absyn::Path>, mut program: Absyn::Program, mut includePartial: bool, mut candidates: Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>>) -> Result<Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>>> {
    let mut candidates: Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>> = candidates;
    let mut cdef: Arc<Absyn::Class>;
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut is_parent: bool = false;
    if let Ok(__iflet0) = getPathedClassInProgram(path.clone(), program.clone(), false, false) {
        cdef = __iflet0;
    } else {
        return Ok(candidates);
    }
    if includePartial.clone() || AbsynUtil::isNotPartial(cdef.clone())? {
        candidates = cons(Arc::new(ClassEntry::ClassEntry { path: path.clone(), cls: cdef.clone() }), candidates.clone());
        is_parent = AbsynUtil::pathEqual(path.clone(), parentClass.clone());
        if AbsynUtil::isPackageRestriction(cdef.restriction.clone()) || is_parent.clone() {
            names = getClassnamesInClassListNoPartial(path.clone(), program.clone(), cdef.clone(), is_parent.clone(), false)?;
            paths = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut n in (names.clone()).into_iter().cloned() {
            let __x = AbsynUtil::suffixPath(path.clone(), (n.clone()).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            candidates = List::fold(paths.clone(), Arc::new({ let __pe_b1 = parentClass.clone(); let __pe_b2 = program.clone(); let __pe_b3 = includePartial.clone(); move |__pe_a0, __pe_a4| getAllSubtypeOfCandidates(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone(), __pe_a4) }), candidates.clone());
        }
    }
    Ok(candidates)
}

pub fn getAllSubtypeOf2(mut baseClass: Arc<Absyn::Path>, mut parentClass: Arc<Absyn::Path>, mut program: Absyn::Program, mut includePartial: bool, mut sort: bool) -> Result<Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>>> {
    let mut entries: Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>> = metamodelica::nil();
    let mut strlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut p: Arc<Absyn::Path>;
    let mut parent: Arc<Absyn::Path>;
    let mut base_class: Arc<Absyn::Path>;
    let mut cls: Arc<Absyn::Class>;
    let mut base_entry: Arc<ClassEntry::ClassEntry>;
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut result_path_lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>> = metamodelica::nil();
    let mut locals: Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>> = metamodelica::nil();
    let mut extendPaths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut candidates: Arc<metamodelica::List<(Arc<Absyn::Path>, Arc<metamodelica::List<Arc<ClassEntry::ClassEntry>>>)>> = metamodelica::nil();
    let mut genv: GraphicEnvCache;
    let mut opt_path: Option<Arc<Absyn::Path>> = None;
    for mut ext in &*getAllInheritedClasses(parentClass.clone(), program.clone())? {
        let mut ext = ext.clone();
        acc = getAllSubtypeOfCandidates(ext.clone(), ext.clone(), program.clone(), includePartial.clone(), metamodelica::nil())?;
        candidates = cons((ext.clone(), acc.clone()), candidates.clone());
    }
    let Absyn::PROGRAM { classes: __pa0, .. } = (program.clone()) else { bail!("pattern mismatch") };
    classes = __pa0.clone();
    if !(includePartial.clone()) {
        classes = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
        for mut c in (classes.clone()).into_iter().cloned() {
            if !(AbsynUtil::isNotPartial(c.clone())?) { continue; }
            let __x = c.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    }
    strlst = List::map(classes.clone(), Arc::new(AbsynUtil::getClassName));
    result_path_lst = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
        for mut r#str in (strlst.clone()).into_iter().cloned() {
            let __x = AbsynUtil::makeIdentPathFromString((r#str.clone()).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    acc = metamodelica::nil();
    for mut p in &*result_path_lst.clone() {
        let mut p = p.clone();
        acc = getAllSubtypeOfCandidates(p.clone(), parentClass.clone(), program.clone(), includePartial.clone(), acc.clone())?;
    }
    candidates = cons((parentClass.clone(), acc.clone()), candidates.clone());
    match '__try1: {
        genv = unwrap_break_err!(createEnvironment(program.clone(), None, parentClass.clone()), '__try1);
        base_class = unwrap_break_err!(qualifyPath(genv.clone(), baseClass.clone(), true), '__try1);
        Ok::<_, anyhow::Error>((base_class.clone(), genv.clone()))
    } {
        Ok((__try1_o0, __try1_o1)) => {
            base_class = __try1_o0;
            genv = __try1_o1;
        }
        Err(_) => {
            entries = metamodelica::nil();
            return Ok(entries);
        }
    }
    entries = metamodelica::nil();
    locals = metamodelica::nil();
    for mut tup in &*candidates.clone() {
        let mut tup = tup.clone();
        (parent, acc) = tup.clone();
        for mut entry in &*acc.clone() {
            let mut entry = entry.clone();
            if isSubtypeOf(entry.path.clone(), base_class.clone(), program.clone())? {
                opt_path = AbsynUtil::removePrefixOpt(parent.clone(), entry.path.clone());
                if isSome(opt_path.clone()) {
                    assign_field!(entry.path = Util::getOption(opt_path.clone())?);
                    locals = cons(entry.clone(), locals.clone());
                } else {
                    entries = cons(entry.clone(), entries.clone());
                }
            }
        }
    }
    cls = getPathedClassInProgram(base_class.clone(), program.clone(), false, false)?;
    base_entry = Arc::new(ClassEntry::ClassEntry { path: base_class.clone(), cls: cls.clone() });
    for mut tup in &*candidates.clone() {
        let mut tup = tup.clone();
        (_, acc) = tup.clone();
        if List::contains(acc.clone(), base_entry.clone(), Arc::new(fnptr!(ClassEntry::equal, Arc<ClassEntry::ClassEntry>, Arc<ClassEntry::ClassEntry>))) {
            entries = cons(base_entry.clone(), entries.clone());
            break;
        }
    }
    entries = listAppend(locals.clone(), entries.clone());
    entries = List::uniqueOnTrue(entries.clone(), Arc::new(fnptr!(ClassEntry::equal, Arc<ClassEntry::ClassEntry>, Arc<ClassEntry::ClassEntry>)));
    if sort.clone() {
        entries = List::sort(entries.clone(), Arc::new(fnptr!(ClassEntry::greaterEq, Arc<ClassEntry::ClassEntry>, Arc<ClassEntry::ClassEntry>)))?;
    }
    Ok(entries)
}

fn isSubtypeOf(mut classPath: Arc<Absyn::Path>, mut baseClassPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<bool> {
    let mut res: bool = false;
    let mut base_classes: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    base_classes = getAllInheritedClasses(classPath.clone(), program.clone())?;
    res = List::contains(base_classes.clone(), baseClassPath.clone(), Arc::new(AbsynUtil::pathSuffixOfr));
    Ok(res)
}

pub fn updateConnectionAnnotation(mut inClass: Arc<Absyn::ComponentRef>, mut inFrom: ArcStr, mut inTo: ArcStr, mut inAnnotation: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program;
    let mut class_path: Arc<Absyn::Path>;
    let mut cls: Arc<Absyn::Class>;
    let mut class_within: Absyn::Within = Absyn::Within::TOP;
    class_path = AbsynUtil::crefToPath(inClass.clone())?;
    cls = getPathedClassInProgram(class_path.clone(), inProgram.clone(), false, false)?;
    cls = updateConnectionAnnotationInClass(cls.clone(), (inFrom.clone()).clone(), (inTo.clone()).clone(), annotationListToAbsyn(inAnnotation.clone())?)?;
    class_within = if (AbsynUtil::pathIsIdent(class_path.clone())) {openmodelica_ast::Absyn::Within::TOP} else {Absyn::Within::WITHIN { path: AbsynUtil::stripLast(class_path.clone())? }};
    outProgram = updateProgram(Absyn::Program { classes: list![cls.clone()], within_: class_within.clone() }, inProgram.clone(), false)?;
    Ok(outProgram)
}

pub fn updateConnectionAnnotationInClass(mut inClass1: Arc<Absyn::Class>, mut inFrom: ArcStr, mut inTo: ArcStr, mut inAnnotation: Arc<Absyn::Annotation>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &((inClass1.clone(), inFrom.clone(), inTo.clone(), inAnnotation.clone())) {
        (outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }, from, to, annotation_) => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = getEquationList(parts.clone())?;
            eqlst_1 = updateConnectionAnnotationInEqList(eqlst.clone(), (from.clone()).clone(), (to.clone()).clone(), annotation_.clone())?;
            parts2 = replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        (outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: modif, baseClassName: bcname }, .. }, from, to, annotation_) => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = getEquationList(parts.clone())?;
            eqlst_1 = updateConnectionAnnotationInEqList(eqlst.clone(), (from.clone()).clone(), (to.clone()).clone(), annotation_.clone())?;
            parts2 = replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

fn updateConnectionAnnotationInEqList(mut equations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut from: ArcStr, mut to: ArcStr, mut ann: Arc<Absyn::Annotation>) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut c1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut c2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut c1_str: ArcStr = arcstr::literal!("");
    let mut c2_str: ArcStr = arcstr::literal!("");
    let mut found: bool = false;
    for mut eq in &*equations.clone() {
        let mut eq = eq.clone();
        if !(found.clone()) {
            eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector2: c2, connector1: c1 }, .. } => {
            c1_str = (AbsynUtil::crefString(c1.clone())?).clone();
            c2_str = (AbsynUtil::crefString(c2.clone())?).clone();
            if c1_str.clone() == from.clone() && c2_str.clone() == to.clone() {
                found = true;
            }
            if !(found.clone()) {
                found = c1_str.clone() == to.clone() && c2_str.clone() == from.clone();
            }
            if found.clone() {
                assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; comment = Some(Arc::new(Absyn::Comment { annotation_: Some(ann.clone()), comment: None })));
            }
            eq.clone()
        },
        _ => eq.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        outEquations = cons(eq.clone(), outEquations.clone());
    }
    outEquations = Dangerous::listReverseInPlace(outEquations.clone());
    Ok(outEquations)
}

pub fn updateConnectionNames(mut inPath: Arc<Absyn::Path>, mut inFrom: ArcStr, mut inTo: ArcStr, mut inFromNew: ArcStr, mut inToNew: ArcStr, mut inProgram: Absyn::Program) -> Result<(bool, Absyn::Program)> {
    let mut outResult: bool = false;
    let mut outProgram: Absyn::Program;
    (outResult, outProgram) = 'mc: {
        let __mc_input = (inPath.clone(), inFrom.clone(), inTo.clone(), inFromNew.clone(), inToNew.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, from, to, fromNew, toNew, p @ Absyn::Program { .. }) => {
                    let mut modelwithin: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class>;
                    let mut newcdef: Arc<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    modelwithin = AbsynUtil::stripLast(path.clone())?;
                    cdef = getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    newcdef = updateConnectionNamesInClass(cdef.clone(), (from.clone()).clone(), (to.clone()).clone(), (fromNew.clone()).clone(), (toNew.clone()).clone())?;
                    newp = updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: Absyn::Within::WITHIN { path: modelwithin.clone() } }, p.clone(), false)?;
                    Ok((true, newp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, from, to, fromNew, toNew, p @ Absyn::Program { .. }) => {
                    let mut cdef: Arc<Absyn::Class>;
                    let mut newcdef: Arc<Absyn::Class>;
                    let mut newp: Absyn::Program;
                    cdef = getPathedClassInProgram(path.clone(), p.clone(), false, false)?;
                    newcdef = updateConnectionNamesInClass(cdef.clone(), (from.clone()).clone(), (to.clone()).clone(), (fromNew.clone()).clone(), (toNew.clone()).clone())?;
                    newp = updateProgram(Absyn::Program { classes: list![newcdef.clone()], within_: openmodelica_ast::Absyn::Within::TOP }, p.clone(), false)?;
                    Ok((true, newp.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, p @ Absyn::Program { .. }) => {
                    Ok((false, p.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outResult, outProgram))
}

fn updateConnectionNamesInClass(mut inClass1: Arc<Absyn::Class>, mut inFrom: ArcStr, mut inTo: ArcStr, mut inFromNew: ArcStr, mut inToNew: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &((inClass1.clone(), inFrom.clone(), inTo.clone(), inFromNew.clone(), inToNew.clone())) {
        (outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }, from, to, fromNew, toNew) => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = getEquationList(parts.clone())?;
            eqlst_1 = updateConnectionNamesInEqList(eqlst.clone(), (from.clone()).clone(), (to.clone()).clone(), (fromNew.clone()).clone(), (toNew.clone()).clone())?;
            parts2 = replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
            outClass.clone()
        },
        (outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: cmt, ann, parts, modifications: modif, baseClassName: bcname }, .. }, from, to, fromNew, toNew) => {
            let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut eqlst_1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
            let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut outClass = (*outClass).clone();
            eqlst = getEquationList(parts.clone())?;
            eqlst_1 = updateConnectionNamesInEqList(eqlst.clone(), (from.clone()).clone(), (to.clone()).clone(), (fromNew.clone()).clone(), (toNew.clone()).clone())?;
            parts2 = replaceEquationList(parts.clone(), eqlst_1.clone())?;
            assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

fn updateConnectionNamesInEqList(mut equations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut from: ArcStr, mut to: ArcStr, mut fromNew: ArcStr, mut toNew: ArcStr) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut outEquations: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut c1: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut c2: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    let mut c1_str: ArcStr = arcstr::literal!("");
    let mut c2_str: ArcStr = arcstr::literal!("");
    let mut found: bool = false;
    for mut eq in &*equations.clone() {
        let mut eq = eq.clone();
        if !(found.clone()) {
            eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: Deref @ Absyn::Equation::EQ_CONNECT { connector2: c2, connector1: c1 }, .. } => {
            c1_str = (AbsynUtil::crefString(c1.clone())?).clone();
            c2_str = (AbsynUtil::crefString(c2.clone())?).clone();
            found = if (c1_str.clone() == from.clone() && c2_str.clone() == to.clone()) {true} else {c1_str.clone() == to.clone() && c2_str.clone() == from.clone()};
            if found.clone() {
                assign_variant_field!(eq => Absyn::EquationItem::EQUATIONITEM; equation_ = Arc::new(Absyn::Equation::EQ_CONNECT { connector1: Parser::stringCref((fromNew.clone()).clone())?, connector2: Parser::stringCref((toNew.clone()).clone())? }));
            }
            eq.clone()
        },
        _ => eq.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        }
        outEquations = cons(eq.clone(), outEquations.clone());
    }
    outEquations = Dangerous::listReverseInPlace(outEquations.clone());
    Ok(outEquations)
}

fn getClassnamesInClassListNoPartial(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClass: Arc<Absyn::Class>, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    if AbsynUtil::isPartial(inClass.clone())? {
        outString = metamodelica::nil();
        return Ok(outString);
    }
    outString = (::match_deref::match_deref! { match &((inPath.clone(), inProgram.clone(), inClass.clone(), inShowProtected.clone(), includeConstants.clone())) {
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInPartsNoPartial(parts.clone(), b.clone(), c.clone())?;
            strlist.clone()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInPartsNoPartial(parts.clone(), b.clone(), c.clone())?;
            strlist.clone()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (_, _, Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PDER { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getClassnamesInPartsNoPartial(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outStringLst = 'mc: {
        let __mc_input = (inAbsynClassPartLst.clone(), inShowProtected.clone(), includeConstants.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _, _) => {
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PUBLIC { contents: elts }, tail: rest }, b, c) => {
                    let mut l1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    l1 = getClassnamesInEltsNoPartial(elts.clone(), c.clone())?;
                    l2 = getClassnamesInPartsNoPartial(rest.clone(), b.clone(), c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ClassPart::PROTECTED { contents: elts }, tail: rest }, true, c) => {
                    let mut l1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    l1 = getClassnamesInEltsNoPartial(elts.clone(), c.clone())?;
                    l2 = getClassnamesInPartsNoPartial(rest.clone(), true, c.clone())?;
                    res = listAppend(l1.clone(), l2.clone());
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, b, c) => {
                    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    res = getClassnamesInPartsNoPartial(rest.clone(), b.clone(), c.clone())?;
                    Ok(res.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outStringLst)
}

pub fn getClassnamesInEltsNoPartial(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outStringLst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut delst: DoubleEnded::MutableList<ArcStr>;
    delst = DoubleEnded::fromList(metamodelica::nil())?;
    for mut elt in &*inAbsynElementItemLst.clone() {
        let mut elt = elt.clone();
        let _ = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { baseClassName: id, .. }, partialPrefix: false, .. }, .. }, .. } } => {
            DoubleEnded::push_back(delst.clone(), (id.clone()).clone());
            ()
        },
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: Deref @ Absyn::Class { name: id, partialPrefix: false, .. }, .. }, .. } } => {
            DoubleEnded::push_back(delst.clone(), (id.clone()).clone());
            ()
        },
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: lst, attributes: Absyn::ElementAttributes { variability: Absyn::Variability::CONST, .. }, .. }, .. } } if (includeConstants.clone()) => {
            DoubleEnded::push_list_back(delst.clone(), getComponentItemsName(lst.clone(), false));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outStringLst = DoubleEnded::toListAndClear(delst.clone(), metamodelica::nil());
    Ok(outStringLst)
}

pub fn removeInnerClass(mut inClass1: Arc<Absyn::Class>, mut inClass2: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = 'mc: {
        let __mc_input = (inClass1.clone(), inClass2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(parts.clone());
                    publst2 = removeClassInElementitemlist(publst.clone(), c1.clone())?;
                    parts2 = replacePublicList(parts.clone(), publst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { comment: cmt, ann, classParts: parts, classAttrs, typeVars }, .. }) => {
                    let mut prolst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut prolst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    prolst = getProtectedList(parts.clone());
                    prolst2 = removeClassInElementitemlist(prolst.clone(), c1.clone())?;
                    parts2 = replaceProtectedList(parts.clone(), prolst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { ann, comment: cmt, parts, modifications: modif, baseClassName: bcname }, .. }) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut publst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    publst = getPublicList(parts.clone());
                    publst2 = removeClassInElementitemlist(publst.clone(), c1.clone())?;
                    parts2 = replacePublicList(parts.clone(), publst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, outClass @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { ann, comment: cmt, parts, modifications: modif, baseClassName: bcname }, .. }) => {
                    let mut prolst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut prolst2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut parts2: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
                    let mut outClass = (*outClass).clone();
                    prolst = getProtectedList(parts.clone());
                    prolst2 = removeClassInElementitemlist(prolst.clone(), c1.clone())?;
                    parts2 = replaceProtectedList(parts.clone(), prolst2.clone())?;
                    todo!("unhandled field-assign shape: outClass.body");
                    Ok(outClass.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { name: n, .. }, Deref @ Absyn::Class { info: file_info, name: a, .. }) => {
                    Error::addSourceMessage(Error::CLASS_NOT_FOUND.clone(), list![(n.clone()).clone(), (a.clone()).clone()], file_info.clone())?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outClass)
}

fn removeClassInElementitemlist(mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    (outElements, _) = List::deleteMemberOnTrue((name.clone()).clone(), inElements.clone(), Arc::new(fnptr!(classElementItemIsNamed, ArcStr, Arc<Absyn::ElementItem>)))?;
    Ok(outElements)
}

pub fn getPathedElementInProgram(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element>;
    let mut cls: Arc<Absyn::Class>;
    match '__try0: {
        cls = unwrap_break_err!(getClassInProgram((AbsynUtil::pathFirstIdent(path.clone())?).clone(), program.clone()), '__try0);
        Ok::<_, anyhow::Error>((cls.clone(),))
    } {
        Ok((__try0_o0,)) => {
            cls = __try0_o0;
        }
        Err(_) => {
            cls = getClassInProgram((AbsynUtil::pathFirstIdent(path.clone())?).clone(), (FBuiltin::getInitialFunctions()?).0)?;
        }
    }
    if AbsynUtil::pathIsIdent(path.clone()) {
        element = Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: false, class_: cls.clone() }), info: cls.info.clone(), constrainClass: None });
    } else {
        let __pa1 = ::match_deref::match_deref! { match &(getPathedElementInClass(AbsynUtil::pathRest(path.clone())?, cls.clone())?) {
            Some(__pa1) => __pa1.clone(),
            _ => bail!("pattern mismatch"),
        } };
        element = __pa1.clone();
    }
    Ok(element)
}

fn getPathedElementInClass(mut path: Arc<Absyn::Path>, mut cls: Arc<Absyn::Class>) -> Result<Option<Arc<Absyn::Element>>> {
    let mut element: Option<Arc<Absyn::Element>> = None;
    for mut part in &*AbsynUtil::getClassPartsInClass(cls.clone()) {
        let mut part = part.clone();
        element = getPathedElementInClassPart(path.clone(), part.clone())?;
        if isSome(element.clone()) {
            break;
        }
    }
    Ok(element)
}

fn getPathedElementInClassPart(mut path: Arc<Absyn::Path>, mut part: Arc<Absyn::ClassPart>) -> Result<Option<Arc<Absyn::Element>>> {
    let mut element: Option<Arc<Absyn::Element>> = None;
    let mut e: Arc<Absyn::Element>;
    for mut item in &*AbsynUtil::getElementItemsInClassPart(part.clone()) {
        let mut item = item.clone();
        if AbsynUtil::isElementItemNamed((AbsynUtil::pathFirstIdent(path.clone())?).clone(), item.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(item.clone()) {
                Deref @ Absyn::ElementItem::ELEMENTITEM { element: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            e = __pa0.clone();
            if AbsynUtil::pathIsIdent(path.clone()) {
                element = Some(e.clone());
            } else {
                element = getPathedElementInElement(AbsynUtil::pathRest(path.clone())?, e.clone())?;
            }
            break;
        }
    }
    Ok(element)
}

fn getPathedElementInElement(mut path: Arc<Absyn::Path>, mut element: Arc<Absyn::Element>) -> Result<Option<Arc<Absyn::Element>>> {
    let mut outElement: Option<Arc<Absyn::Element>> = None;
    let mut cls: Arc<Absyn::Class>;
    outElement = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: cls, .. }, .. } => getPathedElementInClass(path.clone(), cls.clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outElement)
}

pub fn getPathedExtendsInProgram(mut classPath: Arc<Absyn::Path>, mut extendsPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> Option<Arc<Absyn::ElementSpec>> {
    let mut extendsSpec: Option<Arc<Absyn::ElementSpec>> = None;
    let mut cls: Arc<Absyn::Class>;
    let mut env: GraphicEnvCache;
    if '__try0: {
        cls = unwrap_break_err!(getPathedClassInProgram(classPath.clone(), program.clone(), false, false), '__try0);
        env = unwrap_break_err!(Interactive::getClassEnv(program.clone(), classPath.clone()), '__try0);
        for mut ext in &*unwrap_break_err!(getExtendsElementspecInClass(cls.clone()), '__try0) {
            let mut ext = ext.clone();
            ext = unwrap_break_err!(Interactive::makeExtendsFullyQualified(ext.clone(), env.clone()), '__try0);
            if AbsynUtil::pathEqual(extendsPath.clone(), AbsynUtil::elementSpecToPath(ext.clone()).unwrap()) {
                extendsSpec = Some(ext.clone());
                return extendsSpec;
            }
        }
        Ok::<(), anyhow::Error>(())
    }.is_err() {
    }
    extendsSpec = None;
    extendsSpec
}

pub fn transformPathedElementInList<T: Clone + 'static>(mut inList: Arc<metamodelica::List<T>>, mut inFunc: Arc<dyn ::std::ops::Fn(T) -> Result<(T, Option<Arc<Absyn::Element>>, bool)> + 'static>) -> Result<(Arc<metamodelica::List<T>>, Option<Arc<Absyn::Element>>, bool)> {
    pub type FuncType<T: Clone> = fn(T) -> Result<(T, Option<Arc<Absyn::Element>>, bool)>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outElement: Option<Arc<Absyn::Element>> = None;
    let mut outFound: bool = false;
    let mut e: T;
    let mut rest: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest.clone().is_empty()) && !(outFound.clone()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest = __pa1.clone();
        (e, outElement, outFound) = inFunc(e.clone())?;
        outList = cons(e.clone(), outList.clone());
    }
    outList = List::append_reverse(outList.clone(), rest.clone());
    Ok((outList, outElement, outFound))
}

pub fn transformPathedElementInProgram(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut program: Absyn::Program) -> Result<(Absyn::Program, Option<Arc<Absyn::Element>>, bool)> {
    pub type Func = fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>>;

    fn transform_class(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut cls: Arc<Absyn::Class>) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Element>>, bool)> {
        let mut cls: Arc<Absyn::Class> = cls;
        let mut outElement: Option<Arc<Absyn::Element>> = None;
        let mut found: bool = false;
        let mut elem: Arc<Absyn::Element>;
        found = AbsynUtil::pathFirstIdent(path.clone())? == cls.name.clone();
        if found.clone() {
            if AbsynUtil::pathIsIdent(path.clone()) {
                elem = Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: false, class_: cls.clone() }), info: cls.info.clone(), constrainClass: None });
                elem = func(elem.clone())?;
                outElement = Some(elem.clone());
                let __pa0 = ::match_deref::match_deref! { match &(elem.clone()) {
                    Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                cls = __pa0.clone();
            } else {
                (cls, outElement, found) = transformPathedElementInClass(AbsynUtil::pathRest(path.clone())?, func.clone(), cls.clone())?;
            }
        } else {
            outElement = None;
        }
        Ok((cls, outElement, found))
    }

    let mut program: Absyn::Program = program;
    let mut element: Option<Arc<Absyn::Element>> = None;
    let mut success: bool = false;
    let mut clss: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    (clss, element, success) = transformPathedElementInList(program.classes.clone(), Arc::new({ let __pe_b0 = path.clone(); let __pe_b1 = func.clone(); move |__pe_a2| transform_class(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?;
    if success.clone() {
        program.classes = clss.clone();
    }
    Ok((program, element, success))
}

fn transformPathedElementInClass(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut cls: Arc<Absyn::Class>) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Element>>, bool)> {
    pub type Func = fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>>;

    let mut cls: Arc<Absyn::Class> = cls;
    let mut element: Option<Arc<Absyn::Element>> = None;
    let mut success: bool = false;
    let mut def: Arc<Absyn::ClassDef>;
    (def, element, success) = transformPathedElementInClassDef(path.clone(), func.clone(), cls.body.clone())?;
    if success.clone() {
        assign_field!(cls.body = def.clone());
    }
    Ok((cls, element, success))
}

fn transformPathedElementInClassDef(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut def: Arc<Absyn::ClassDef>) -> Result<(Arc<Absyn::ClassDef>, Option<Arc<Absyn::Element>>, bool)> {
    pub type Func = fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>>;

    let mut def: Arc<Absyn::ClassDef> = def;
    let mut element: Option<Arc<Absyn::Element>> = None;
    let mut success: bool = false;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    success = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            (parts, element, success) = transformPathedElementInList(var_field!((*def).classParts, Absyn::ClassDef::PARTS).clone(), Arc::new({ let __pe_b0 = path.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static> = func.clone(); move |__pe_a2| transformPathedElementInClassPart(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?;
            if success.clone() {
                assign_variant_field!(def => Absyn::ClassDef::PARTS; classParts = parts.clone());
            }
            success.clone()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            (parts, element, success) = transformPathedElementInList(var_field!((*def).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), Arc::new({ let __pe_b0 = path.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static> = func.clone(); move |__pe_a2| transformPathedElementInClassPart(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?;
            if success.clone() {
                assign_variant_field!(def => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
            }
            success.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((def, element, success))
}

fn transformPathedElementInClassPart(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut part: Arc<Absyn::ClassPart>) -> Result<(Arc<Absyn::ClassPart>, Option<Arc<Absyn::Element>>, bool)> {
    pub type Func = fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>>;

    let mut part: Arc<Absyn::ClassPart> = part;
    let mut element: Option<Arc<Absyn::Element>> = None;
    let mut success: bool = false;
    let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    success = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            (items, element, success) = transformPathedElementInList(var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone(), Arc::new({ let __pe_b0 = path.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static> = func.clone(); move |__pe_a2| transformPathedElementInElementItem(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?;
            if success.clone() {
                assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = items.clone());
            }
            success.clone()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            (items, element, success) = transformPathedElementInList(var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone(), Arc::new({ let __pe_b0 = path.clone(); let __pe_b1: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static> = func.clone(); move |__pe_a2| transformPathedElementInElementItem(__pe_b0.clone(), __pe_b1.clone(), __pe_a2) }))?;
            if success.clone() {
                assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = items.clone());
            }
            success.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((part, element, success))
}

fn transformPathedElementInElementItem(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut item: Arc<Absyn::ElementItem>) -> Result<(Arc<Absyn::ElementItem>, Option<Arc<Absyn::Element>>, bool)> {
    pub type Func = fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>>;

    let mut item: Arc<Absyn::ElementItem> = item;
    let mut outElement: Option<Arc<Absyn::Element>> = None;
    let mut success: bool = false;
    let mut element: Arc<Absyn::Element>;
    success = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } if (AbsynUtil::isElementItemNamed((AbsynUtil::pathFirstIdent(path.clone())?).clone(), item.clone())) => {
            if AbsynUtil::pathIsIdent(path.clone()) {
                assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = func(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone())?);
                outElement = Some(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone());
                success = true;
            } else {
                (element, outElement, success) = transformPathedElementInElement(AbsynUtil::pathRest(path.clone())?, func.clone(), var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone())?;
                if success.clone() {
                    assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = element.clone());
                }
            }
            success.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((item, outElement, success))
}

fn transformPathedElementInElement(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut element: Arc<Absyn::Element>) -> Result<(Arc<Absyn::Element>, Option<Arc<Absyn::Element>>, bool)> {
    pub type Func = fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>>;

    let mut element: Arc<Absyn::Element> = element;
    let mut outElement: Option<Arc<Absyn::Element>> = None;
    let mut success: bool = false;
    let mut spec: Arc<Absyn::ElementSpec>;
    success = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            (spec, outElement, success) = transformPathedElementInElementSpec(path.clone(), func.clone(), var_field!((*element).specification, Absyn::Element::ELEMENT).clone())?;
            if success.clone() {
                assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            }
            success.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((element, outElement, success))
}

fn transformPathedElementInElementSpec(mut path: Arc<Absyn::Path>, mut func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>> + 'static>, mut spec: Arc<Absyn::ElementSpec>) -> Result<(Arc<Absyn::ElementSpec>, Option<Arc<Absyn::Element>>, bool)> {
    pub type Func = fn(Arc<Absyn::Element>) -> Result<Arc<Absyn::Element>>;

    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let mut element: Option<Arc<Absyn::Element>> = None;
    let mut success: bool = false;
    let mut cls: Arc<Absyn::Class>;
    success = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            (cls, element, success) = transformPathedElementInClass(path.clone(), func.clone(), var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone())?;
            if success.clone() {
                assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = cls.clone());
            }
            success.clone()
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((spec, element, success))
}

pub fn getPathedClassRestriction(mut path: Arc<Absyn::Path>, mut program: Absyn::Program) -> Absyn::Restriction {
    let mut restriction: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(getPathedClassInProgram(path.clone(), program.clone(), false, false), '__try0)) {
            Deref @ Absyn::Class { restriction: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        restriction = __pa1.clone();
        Ok::<_, anyhow::Error>((restriction.clone(),))
    } {
        Ok((__try0_o0,)) => {
            restriction = __try0_o0;
        }
        Err(_) => {
            restriction = openmodelica_ast::Absyn::Restriction::R_UNKNOWN;
        }
    }
    restriction
}

pub fn getPathedSCodeElementInProgram(mut path: Arc<Absyn::Path>, mut program: Arc<metamodelica::List<Arc<SCode::Element>>>) -> Result<Arc<SCode::Element>> {
    let mut element: Arc<SCode::Element>;
    let mut name: ArcStr = arcstr::literal!("");
    name = (AbsynUtil::pathFirstIdent(path.clone())?).clone();
    element = List::find(program.clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); move |__pe_a1| Ok(SCodeUtil::isElementNamed(__pe_b0.clone(), __pe_a1)) }))?;
    if !(AbsynUtil::pathIsIdent(path.clone())) {
        element = getPathedSCodeElementInProgram(AbsynUtil::pathRest(path.clone())?, SCodeUtil::getClassElements(element.clone()))?;
    }
    Ok(element)
}

pub fn getElementAnnotation(mut elementPath: Arc<Absyn::Path>, mut program: Absyn::Program) -> ArcStr {
    let mut annotationString: ArcStr = arcstr::literal!("");
    let mut cls: Arc<Absyn::Class>;
    let mut elem: Arc<Absyn::Element>;
    let mut ann: Option<Arc<Absyn::Annotation>> = None;
    let mut eargs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    match '__try0: {
        elem = unwrap_break_err!(getPathedElementInProgram(elementPath.clone(), program.clone()), '__try0);
        ann = unwrap_break_err!(AbsynUtil::getElementAnnotation(elem.clone(), (AbsynUtil::pathLastIdent(elementPath.clone()).unwrap()).clone()), '__try0);
        if isSome(ann.clone()) {
            let __pa1 = ::match_deref::match_deref! { match &(ann.clone()) {
                Some(Deref @ Absyn::Annotation { elementArgs: __pa1 }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
            } };
            eargs = __pa1.clone();
            annotationString = (unwrap_break_err!(List::toString(eargs.clone(), Arc::new(Dump::unparseElementArgStr), (literal!("")).clone(), (literal!("(")).clone(), (literal!(", ")).clone(), (literal!(")")).clone(), true, 0), '__try0)).clone();
        } else {
            annotationString = (literal!("()")).clone();
        }
        Ok::<_, anyhow::Error>((annotationString.clone(),))
    } {
        Ok((__try0_o0,)) => {
            annotationString = __try0_o0;
        }
        Err(_) => {
            annotationString = (literal!("")).clone();
        }
    }
    annotationString
}

pub fn setElementAnnotation(mut elementPath: Arc<Absyn::Path>, mut annotationMod: Arc<Absyn::Modification>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut ann: Option<Arc<Absyn::Annotation>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut elem_opt: Option<Arc<Absyn::Element>> = None;
    match '__try0: {
        if annotationMod.elementArgLst.clone().is_empty() {
            ann = None;
        } else {
            ann = Some(Arc::new(Absyn::Annotation { elementArgs: annotationMod.elementArgLst.clone() }));
        }
        name = (unwrap_break_err!(AbsynUtil::pathLastIdent(elementPath.clone()), '__try0)).clone();
        (program, elem_opt, success) = unwrap_break_err!(transformPathedElementInProgram(elementPath.clone(), Arc::new({ let __pe_b1 = (name.clone()).clone(); let __pe_b2 = ann.clone(); move |__pe_a0| AbsynUtil::setElementAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), program.clone()), '__try0);
        if success.clone() {
            unwrap_break_err!(SymbolTable::setAbsynElement(program.clone(), Util::getOption(elem_opt.clone()).unwrap(), elementPath.clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((ann.clone(), elem_opt.clone(), name.clone(), program.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
            ann = __try0_o0;
            elem_opt = __try0_o1;
            name = __try0_o2;
            program = __try0_o3;
            success = __try0_o4;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

pub fn loadClassContentString(mut content: ArcStr, mut classPath: Arc<Absyn::Path>, mut offsetX: i32, mut offsetY: i32, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut parsed_body: Arc<Absyn::ClassDef>;
    match '__try0: {
        let Absyn::Program { classes: metamodelica::List::Cons { head: Absyn::Class { body: __pa1, .. }, tail: __t2 }, .. } = (unwrap_break_err!(Parser::parsestring(stringAppendList(list![(literal!("model dummy\n")).clone(), (content.clone()).clone(), (literal!("end dummy;\n")).clone()]), (literal!("<interactive>")).clone(), Config::acceptedGrammar().unwrap(), Flags::getConfigEnum(Flags::LANGUAGE_STANDARD.clone()).unwrap(), Flags::getConfigBool(Flags::STRICT.clone()).unwrap()), '__try0)) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        ::match_deref::match_deref! { match &(__t2.clone()) {
            Deref @ metamodelica::List::Nil => (),
            _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
        } };
        parsed_body = __pa1.clone();
        parsed_body = unwrap_break_err!(offsetAnnotationsInClassDef(parsed_body.clone(), offsetX.clone(), offsetY.clone()), '__try0);
        (program, _, success) = unwrap_break_err!(transformPathedElementInProgram(classPath.clone(), Arc::new({ let __pe_b1 = parsed_body.clone(); move |__pe_a0| mergeClassContents(__pe_a0, __pe_b1.clone()) }), program.clone()), '__try0);
        Ok::<_, anyhow::Error>((parsed_body.clone(), program.clone(), success.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            parsed_body = __try0_o0;
            program = __try0_o1;
            success = __try0_o2;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

pub fn mergeClassContents(mut element: Arc<Absyn::Element>, mut newContent: Arc<Absyn::ClassDef>) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let mut spec: Arc<Absyn::ElementSpec>;
    let mut cls: Arc<Absyn::Class>;
    let mut old_content: Arc<Absyn::ClassDef>;
    let mut new_content: Arc<Absyn::ClassDef>;
    new_content = resolveMergeContentsConflicts(element.clone(), newContent.clone())?;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: spec @ Deref @ Absyn::ElementSpec::CLASSDEF { class_: cls @ Deref @ Absyn::Class { body: old_content, .. }, .. }, .. } => {
            let mut spec = (*spec).clone();
            let mut cls = (*cls).clone();
            let () = (::match_deref::match_deref! { match &((old_content.clone(), new_content.clone())) {
        (Deref @ Absyn::ClassDef::PARTS { .. }, Deref @ Absyn::ClassDef::PARTS { .. }) => {
            assign_variant_field!(old_content => Absyn::ClassDef::PARTS;
                classParts = mergeClassParts(var_field!((*new_content).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**old_content).classParts, Absyn::ClassDef::PARTS).clone())?,
                ann = mergeAnnotationLists(var_field!((*new_content).ann, Absyn::ClassDef::PARTS).clone(), var_field!((**old_content).ann, Absyn::ClassDef::PARTS).clone())?
            );
            ()
        },
        (Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, Deref @ Absyn::ClassDef::PARTS { .. }) => {
            assign_variant_field!(old_content => Absyn::ClassDef::CLASS_EXTENDS;
                parts = mergeClassParts(var_field!((*new_content).classParts, Absyn::ClassDef::PARTS).clone(), var_field!((**old_content).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?,
                ann = mergeAnnotationLists(var_field!((*new_content).ann, Absyn::ClassDef::PARTS).clone(), var_field!((**old_content).ann, Absyn::ClassDef::CLASS_EXTENDS).clone())?
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
            assign_field!(cls.body = old_content.clone());
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = cls.clone());
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = spec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(element)
}

fn mergeClassParts(mut newParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut oldParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut parts: Arc<Vector::Vector<Arc<Absyn::ClassPart>>>;
    let mut op: Option<Arc<Absyn::ClassPart>> = None;
    let mut p: Arc<Absyn::ClassPart>;
    let mut index: i32 = 0;
    parts = Vector::fromList(oldParts.clone());
    for mut part in &*newParts.clone() {
        let mut part = part.clone();
        let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            (op, index) = Vector::findLast(parts.clone(), Arc::new(fnptr!(AbsynUtil::isElementSection, Arc<Absyn::ClassPart>)));
            let () = (::match_deref::match_deref! { match &(op.clone()) {
        Some(p @ Deref @ Absyn::ClassPart::PUBLIC { .. }) => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::PUBLIC).clone(), var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone()));
            Vector::updateNoBounds(parts.clone(), index.clone(), part.clone());
            ()
        },
        _ => {
            Vector::insert(parts.clone(), part.clone(), std::cmp::max(index.clone() + 1, 1))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            (op, index) = Vector::findLast(parts.clone(), Arc::new(fnptr!(AbsynUtil::isElementSection, Arc<Absyn::ClassPart>)));
            let () = (::match_deref::match_deref! { match &(op.clone()) {
        Some(p @ Deref @ Absyn::ClassPart::PROTECTED { .. }) => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::PROTECTED).clone(), var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone()));
            Vector::updateNoBounds(parts.clone(), index.clone(), part.clone());
            ()
        },
        _ => {
            Vector::insert(parts.clone(), part.clone(), std::cmp::max(index.clone() + 1, 1))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            (op, index) = Vector::findLast(parts.clone(), Arc::new(fnptr!(AbsynUtil::isEquationSection, Arc<Absyn::ClassPart>)));
            let () = (::match_deref::match_deref! { match &(op.clone()) {
        Some(p @ Deref @ Absyn::ClassPart::EQUATIONS { .. }) => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::EQUATIONS).clone(), var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone()));
            Vector::updateNoBounds(parts.clone(), index.clone(), part.clone());
            ()
        },
        _ => {
            if index.clone() == -1 {
                (_, index) = Vector::findLast(parts.clone(), Arc::new(fnptr!(AbsynUtil::isElementSection, Arc<Absyn::ClassPart>)));
            }
            Vector::insert(parts.clone(), part.clone(), std::cmp::max(index.clone() + 1, 1))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            (op, index) = Vector::findLast(parts.clone(), Arc::new(fnptr!(AbsynUtil::isEquationSection, Arc<Absyn::ClassPart>)));
            let () = (::match_deref::match_deref! { match &(op.clone()) {
        Some(p @ Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. }) => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = listAppend(var_field!((*p).contents, Absyn::ClassPart::INITIALEQUATIONS).clone(), var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone()));
            Vector::updateNoBounds(parts.clone(), index.clone(), part.clone());
            ()
        },
        _ => {
            if index.clone() == -1 {
                (_, index) = Vector::findLast(parts.clone(), Arc::new(fnptr!(AbsynUtil::isElementSection, Arc<Absyn::ClassPart>)));
            }
            Vector::insert(parts.clone(), part.clone(), std::cmp::max(index.clone() + 1, 1))?;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            ()
        },
        Deref @ Absyn::ClassPart::EXTERNAL { .. } => {
            (_, index) = Vector::findLast(parts.clone(), Arc::new(fnptr!(AbsynUtil::isExternalPart, Arc<Absyn::ClassPart>)));
            if index.clone() != -1 {
                Vector::updateNoBounds(parts.clone(), index.clone(), part.clone());
            } else {
                Vector::push(parts.clone(), part.clone());
            }
            ()
        },
        _ => {
            Vector::push(parts.clone(), part.clone());
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outParts = Vector::toList(parts.clone());
    Ok(outParts)
}

fn mergeAnnotationLists(mut newAnnotations: Arc<metamodelica::List<Arc<Absyn::Annotation>>>, mut oldAnnotations: Arc<metamodelica::List<Arc<Absyn::Annotation>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Annotation>>>> {
    let mut outAnnotations: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
    let mut old_ann: Arc<Absyn::Annotation>;
    if oldAnnotations.clone().is_empty() {
        outAnnotations = newAnnotations.clone();
    } else {
        old_ann = listHead(oldAnnotations.clone())?;
        for mut new_ann in &*newAnnotations.clone() {
            let mut new_ann = new_ann.clone();
            old_ann = AbsynUtil::mergeAnnotations(old_ann.clone(), new_ann.clone(), true, true)?;
        }
        outAnnotations = cons(old_ann.clone(), listRest(oldAnnotations.clone())?);
    }
    Ok(outAnnotations)
}

fn resolveMergeContentsConflicts(mut oldElement: Arc<Absyn::Element>, mut newContent: Arc<Absyn::ClassDef>) -> Result<Arc<Absyn::ClassDef>> {
    let mut newContent: Arc<Absyn::ClassDef> = newContent;
    let mut old_names: Arc<UnorderedSet::UnorderedSet<ArcStr>>;
    let mut rename_map: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>;
    let mut new_name: ArcStr = arcstr::literal!("");
    let mut index: i32 = 0;
    let mut conflicting_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    old_names = UnorderedSet::new(fnptr!(stringHashDjb2, ArcStr), stringEq, 13);
    for mut e in &*AbsynUtil::getElementItemsInElement(oldElement.clone()) {
        let mut e = e.clone();
        for mut name in &*AbsynUtil::elementItemNames(e.clone())? {
            let mut name = name.clone();
            UnorderedSet::add((name.clone()).clone(), old_names.clone())?;
        }
    }
    rename_map = UnorderedMap::new(fnptr!(stringHashDjb2, ArcStr), stringEq, 1);
    for mut e in &*AbsynUtil::getElementItemsInClassDef(newContent.clone()) {
        let mut e = e.clone();
        for mut name in &*AbsynUtil::elementItemNames(e.clone())? {
            let mut name = name.clone();
            if UnorderedSet::contains((name.clone()).clone(), old_names.clone())? {
                conflicting_names = cons(name.clone(), conflicting_names.clone());
            } else {
                UnorderedSet::add((name.clone()).clone(), old_names.clone())?;
            }
        }
    }
    if conflicting_names.clone().is_empty() {
        return Ok(newContent);
    }
    for mut name in &*conflicting_names.clone().reverse() {
        let mut name = name.clone();
        index = 1;
        new_name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); ArcStr::from(__mm_s) }).clone();
        while UnorderedSet::contains((new_name.clone()).clone(), old_names.clone())? {
            index = index.clone() + 1;
            new_name = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); ArcStr::from(__mm_s) }).clone();
        }
        UnorderedMap::add((name.clone()).clone(), (new_name.clone()).clone(), rename_map.clone())?;
        UnorderedSet::add((new_name.clone()).clone(), old_names.clone())?;
    }
    if !(UnorderedMap::isEmpty(rename_map.clone())) {
        newContent = renameElementsInClassDef(newContent.clone(), rename_map.clone())?;
    }
    Ok(newContent)
}

fn renameElementsInElement(mut element: Arc<Absyn::Element>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT;
                specification = renameElementsInElementSpec(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), nameMap.clone(), true)?,
                constrainClass = renameElementsInConstrainClassOpt(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone(), nameMap.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

fn renameElementsInElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>, mut renameElement: bool) -> Result<Arc<Absyn::ElementSpec>> {
    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = renameElementsInClass(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), nameMap.clone(), renameElement.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS;
                elementArg = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone()).into_iter().cloned() {
            let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                annotationOpt = renameElementsInAnnotationOpt(var_field!((*spec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone(), nameMap.clone())
            );
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS;
                attributes = renameElementsInAttributes(var_field!((*spec).attributes, Absyn::ElementSpec::COMPONENTS).clone(), nameMap.clone())?,
                typeSpec = renameElementsInTypeSpec(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS).clone(), nameMap.clone())?,
                components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
            let __x = renameElementsInComponentItem(c.clone(), nameMap.clone(), renameElement.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(spec)
}

fn renameElementsInClass(mut cls: Arc<Absyn::Class>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>, mut renameElement: bool) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    if renameElement.clone() {
        assign_field!(cls.name = renameElementsInIdent((cls.name.clone()).clone(), nameMap.clone()));
    }
    assign_field!(cls.body = renameElementsInClassDef(cls.body.clone(), nameMap.clone())?);
    Ok(cls)
}

fn renameElementsInClassDef(mut classDef: Arc<Absyn::ClassDef>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::ClassDef>> {
    let mut classDef: Arc<Absyn::ClassDef> = classDef;
    let () = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::PARTS;
                classParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (var_field!((*classDef).classParts, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = renameElementsInClassPart(p.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*classDef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = renameElementsInAnnotation(a.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::DERIVED;
                typeSpec = renameElementsInTypeSpec(var_field!((*classDef).typeSpec, Absyn::ClassDef::DERIVED).clone(), nameMap.clone())?,
                attributes = renameElementsInAttributes(var_field!((*classDef).attributes, Absyn::ClassDef::DERIVED).clone(), nameMap.clone())?,
                arguments = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (var_field!((*classDef).arguments, Absyn::ClassDef::DERIVED).clone()).into_iter().cloned() {
            let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                comment = renameElementsInCommentOpt(var_field!((*classDef).comment, Absyn::ClassDef::DERIVED).clone(), nameMap.clone())
            );
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(classDef => Absyn::ClassDef::CLASS_EXTENDS;
                modifications = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (var_field!((*classDef).modifications, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                parts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (var_field!((*classDef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = renameElementsInClassPart(p.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*classDef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = renameElementsInAnnotation(a.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(classDef)
}

fn renameElementsInClassPart(mut part: Arc<Absyn::ClassPart>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::ClassPart>> {
    let mut part: Arc<Absyn::ClassPart> = part;
    let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut i in (var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone()).into_iter().cloned() {
            let __x = renameElementsInElementItem(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut i in (var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone()).into_iter().cloned() {
            let __x = renameElementsInElementItem(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ClassPart::CONSTRAINTS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::CONSTRAINTS; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (var_field!((*part).contents, Absyn::ClassPart::CONSTRAINTS).clone()).into_iter().cloned() {
            let __x = (renameElementsInExp(e.clone(), nameMap.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = renameElementsInEquationItems(var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone(), nameMap.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = renameElementsInEquationItems(var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone(), nameMap.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::ALGORITHMS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::ALGORITHMS; contents = renameElementsInAlgorithmItems(var_field!((*part).contents, Absyn::ClassPart::ALGORITHMS).clone(), nameMap.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALALGORITHMS; contents = renameElementsInAlgorithmItems(var_field!((*part).contents, Absyn::ClassPart::INITIALALGORITHMS).clone(), nameMap.clone())?);
            ()
        },
        Deref @ Absyn::ClassPart::EXTERNAL { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EXTERNAL;
                externalDecl = renameElementsInExternalDecl(var_field!((*part).externalDecl, Absyn::ClassPart::EXTERNAL).clone(), nameMap.clone())?,
                annotation_ = renameElementsInAnnotationOpt(var_field!((*part).annotation_, Absyn::ClassPart::EXTERNAL).clone(), nameMap.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(part)
}

fn renameElementsInElementItem(mut item: Arc<Absyn::ElementItem>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::ElementItem>> {
    let mut item: Arc<Absyn::ElementItem> = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = renameElementsInElement(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone(), nameMap.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

fn renameElementsInEquationItems(mut items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<metamodelica::List<Arc<Absyn::EquationItem>>>> {
    let mut items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = items;
    items = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
        for mut i in (items.clone()).into_iter().cloned() {
            let __x = renameElementsInEquationItem(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(items)
}

fn renameElementsInEquationItem(mut item: Arc<Absyn::EquationItem>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::EquationItem>> {
    let mut item: Arc<Absyn::EquationItem> = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { .. } => {
            assign_variant_field!(item => Absyn::EquationItem::EQUATIONITEM;
                equation_ = renameElementsInEquation(var_field!((*item).equation_, Absyn::EquationItem::EQUATIONITEM).clone(), nameMap.clone())?,
                comment = renameElementsInCommentOpt(var_field!((*item).comment, Absyn::EquationItem::EQUATIONITEM).clone(), nameMap.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

fn renameElementsInEquation(mut eq: Arc<Absyn::Equation>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::Equation>> {
    let mut eq: Arc<Absyn::Equation> = eq;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::Equation::EQ_IF { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_IF;
                ifExp = AbsynUtil::traverseExp(var_field!((*eq).ifExp, Absyn::Equation::EQ_IF).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                equationTrueItems = renameElementsInEquationItems(var_field!((*eq).equationTrueItems, Absyn::Equation::EQ_IF).clone(), nameMap.clone())?,
                elseIfBranches = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*eq).elseIfBranches, Absyn::Equation::EQ_IF).clone()).into_iter().cloned() {
            let __x = renameElementsInEquationBranch(b.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                equationElseItems = renameElementsInEquationItems(var_field!((*eq).equationElseItems, Absyn::Equation::EQ_IF).clone(), nameMap.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_EQUALS { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_EQUALS;
                leftSide = AbsynUtil::traverseExp(var_field!((*eq).leftSide, Absyn::Equation::EQ_EQUALS).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                rightSide = AbsynUtil::traverseExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_EQUALS).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_PDE { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_PDE;
                leftSide = AbsynUtil::traverseExp(var_field!((*eq).leftSide, Absyn::Equation::EQ_PDE).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                rightSide = AbsynUtil::traverseExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_PDE).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_CONNECT { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_CONNECT;
                connector1 = renameElementsInCref(var_field!((*eq).connector1, Absyn::Equation::EQ_CONNECT).clone(), nameMap.clone(), false)?,
                connector2 = renameElementsInCref(var_field!((*eq).connector2, Absyn::Equation::EQ_CONNECT).clone(), nameMap.clone(), false)?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_FOR { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FOR;
                iterators = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for mut i in (var_field!((*eq).iterators, Absyn::Equation::EQ_FOR).clone()).into_iter().cloned() {
            let __x = renameElementsInIterator(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                forEquations = renameElementsInEquationItems(var_field!((*eq).forEquations, Absyn::Equation::EQ_FOR).clone(), nameMap.clone())?
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_WHEN_E;
                whenExp = AbsynUtil::traverseExp(var_field!((*eq).whenExp, Absyn::Equation::EQ_WHEN_E).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                whenEquations = renameElementsInEquationItems(var_field!((*eq).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), nameMap.clone())?,
                elseWhenEquations = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*eq).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone()).into_iter().cloned() {
            let __x = renameElementsInEquationBranch(b.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_NORETCALL;
                functionName = renameElementsInCref(var_field!((*eq).functionName, Absyn::Equation::EQ_NORETCALL).clone(), nameMap.clone(), false)?,
                functionArgs = AbsynUtil::traverseExpBidirFunctionArgs(var_field!((*eq).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), Arc::new(renameElementsInExp), Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, Arc<Absyn::Exp>, _)), nameMap.clone())?.0
            );
            ()
        },
        Deref @ Absyn::Equation::EQ_FAILURE { .. } => {
            assign_variant_field!(eq => Absyn::Equation::EQ_FAILURE; equ = renameElementsInEquationItem(var_field!((*eq).equ, Absyn::Equation::EQ_FAILURE).clone(), nameMap.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eq)
}

fn renameElementsInEquationBranch(mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)> {
    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>) = branch;
    let mut cond: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut body: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    (cond, body) = branch.clone();
    (cond, _) = AbsynUtil::traverseExp(cond.clone(), Arc::new(renameElementsInExp), nameMap.clone())?;
    body = renameElementsInEquationItems(body.clone(), nameMap.clone())?;
    branch = (cond.clone(), body.clone());
    Ok(branch)
}

fn renameElementsInIterator(mut iter: Arc<Absyn::ForIterator>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::ForIterator>> {
    let mut iter: Arc<Absyn::ForIterator> = iter;
    if isSome(iter.range.clone()) {
        assign_field!(iter.range = Some(AbsynUtil::traverseExp(Util::getOption(iter.range.clone())?, Arc::new(renameElementsInExp), nameMap.clone())?));
    }
    Ok(iter)
}

fn renameElementsInAlgorithmItems(mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>> {
    let mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = items;
    items = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
        for mut i in (items.clone()).into_iter().cloned() {
            let __x = renameElementsInAlgorithmItem(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(items)
}

fn renameElementsInAlgorithmItem(mut item: Arc<Absyn::AlgorithmItem>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::AlgorithmItem>> {
    let mut item: Arc<Absyn::AlgorithmItem> = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. } => {
            assign_variant_field!(item => Absyn::AlgorithmItem::ALGORITHMITEM;
                algorithm_ = renameElementsInAlgorithm(var_field!((*item).algorithm_, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), nameMap.clone())?,
                comment = renameElementsInCommentOpt(var_field!((*item).comment, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), nameMap.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

fn renameElementsInAlgorithm(mut alg: Arc<Absyn::Algorithm>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::Algorithm>> {
    let mut alg: Arc<Absyn::Algorithm> = alg;
    let () = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_ASSIGN;
                assignComponent = AbsynUtil::traverseExp(var_field!((*alg).assignComponent, Absyn::Algorithm::ALG_ASSIGN).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                value = AbsynUtil::traverseExp(var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_IF { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_IF;
                ifExp = AbsynUtil::traverseExp(var_field!((*alg).ifExp, Absyn::Algorithm::ALG_IF).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                trueBranch = renameElementsInAlgorithmItems(var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone(), nameMap.clone())?,
                elseIfAlgorithmBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone()).into_iter().cloned() {
            let __x = renameElementsInAlgorithmBranch(b.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                elseBranch = renameElementsInAlgorithmItems(var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone(), nameMap.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_FOR { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_FOR;
                iterators = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for mut i in (var_field!((*alg).iterators, Absyn::Algorithm::ALG_FOR).clone()).into_iter().cloned() {
            let __x = renameElementsInIterator(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                forBody = renameElementsInAlgorithmItems(var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone(), nameMap.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_PARFOR { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_PARFOR;
                iterators = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for mut i in (var_field!((*alg).iterators, Absyn::Algorithm::ALG_PARFOR).clone()).into_iter().cloned() {
            let __x = renameElementsInIterator(i.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                parforBody = renameElementsInAlgorithmItems(var_field!((*alg).parforBody, Absyn::Algorithm::ALG_PARFOR).clone(), nameMap.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHILE;
                boolExpr = AbsynUtil::traverseExp(var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                whileBody = renameElementsInAlgorithmItems(var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone(), nameMap.clone())?
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_WHEN_A;
                boolExpr = AbsynUtil::traverseExp(var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHEN_A).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0,
                whenBody = renameElementsInAlgorithmItems(var_field!((*alg).whenBody, Absyn::Algorithm::ALG_WHEN_A).clone(), nameMap.clone())?,
                elseWhenAlgorithmBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>> = metamodelica::nil();
        for mut b in (var_field!((*alg).elseWhenAlgorithmBranch, Absyn::Algorithm::ALG_WHEN_A).clone()).into_iter().cloned() {
            let __x = renameElementsInAlgorithmBranch(b.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { .. } => {
            assign_variant_field!(alg => Absyn::Algorithm::ALG_NORETCALL;
                functionCall = renameElementsInCref(var_field!((*alg).functionCall, Absyn::Algorithm::ALG_NORETCALL).clone(), nameMap.clone(), false)?,
                functionArgs = AbsynUtil::traverseExpBidirFunctionArgs(var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), Arc::new(renameElementsInExp), Arc::new(fnptr!(AbsynUtil::dummyTraverseExp, Arc<Absyn::Exp>, _)), nameMap.clone())?.0
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(alg)
}

fn renameElementsInAlgorithmBranch(mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>), mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)> {
    let mut branch: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>) = branch;
    let mut cond: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut body: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>> = metamodelica::nil();
    (cond, body) = branch.clone();
    (cond, _) = AbsynUtil::traverseExp(cond.clone(), Arc::new(renameElementsInExp), nameMap.clone())?;
    body = renameElementsInAlgorithmItems(body.clone(), nameMap.clone())?;
    branch = (cond.clone(), body.clone());
    Ok(branch)
}

fn renameElementsInElementArg(mut arg: Arc<Absyn::ElementArg>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::ElementArg>> {
    let mut arg: Arc<Absyn::ElementArg> = arg;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = renameElementsInModificationOpt(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone(), nameMap.clone()));
            ()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            assign_variant_field!(arg => Absyn::ElementArg::REDECLARATION;
                elementSpec = renameElementsInElementSpec(var_field!((*arg).elementSpec, Absyn::ElementArg::REDECLARATION).clone(), nameMap.clone(), false)?,
                constrainClass = renameElementsInConstrainClassOpt(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone(), nameMap.clone())
            );
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

fn renameElementsInConstrainClassOpt(mut cc: Option<Arc<Absyn::ConstrainClass>>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Option<Arc<Absyn::ConstrainClass>> {
    let mut cc: Option<Arc<Absyn::ConstrainClass>> = cc;
    cc = Util::applyOption(cc.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| renameElementsInConstrainClass(__pe_a0, __pe_b1.clone()) }));
    cc
}

fn renameElementsInConstrainClass(mut cc: Arc<Absyn::ConstrainClass>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::ConstrainClass>> {
    let mut cc: Arc<Absyn::ConstrainClass> = cc;
    assign_field!(
        cc.elementSpec = renameElementsInElementSpec(cc.elementSpec.clone(), nameMap.clone(), true)?,
        cc.comment = renameElementsInCommentOpt(cc.comment.clone(), nameMap.clone())
    );
    Ok(cc)
}

fn renameElementsInCommentOpt(mut comment: Option<Arc<Absyn::Comment>>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Option<Arc<Absyn::Comment>> {
    let mut comment: Option<Arc<Absyn::Comment>> = comment;
    comment = Util::applyOption(comment.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| Ok(renameElementsInComment(__pe_a0, __pe_b1.clone())) }));
    comment
}

fn renameElementsInComment(mut comment: Arc<Absyn::Comment>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Arc<Absyn::Comment> {
    let mut comment: Arc<Absyn::Comment> = comment;
    assign_field!(comment.annotation_ = Util::applyOption(comment.annotation_.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| renameElementsInAnnotation(__pe_a0, __pe_b1.clone()) })));
    comment
}

fn renameElementsInAnnotationOpt(mut ann: Option<Arc<Absyn::Annotation>>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Option<Arc<Absyn::Annotation>> {
    let mut ann: Option<Arc<Absyn::Annotation>> = ann;
    ann = Util::applyOption(ann.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| renameElementsInAnnotation(__pe_a0, __pe_b1.clone()) }));
    ann
}

fn renameElementsInAnnotation(mut ann: Arc<Absyn::Annotation>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::Annotation>> {
    let mut ann: Arc<Absyn::Annotation> = ann;
    assign_field!(ann.elementArgs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (ann.elementArgs.clone()).into_iter().cloned() {
            let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(ann)
}

fn renameElementsInModificationOpt(mut r#mod: Option<Arc<Absyn::Modification>>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Option<Arc<Absyn::Modification>> {
    let mut r#mod: Option<Arc<Absyn::Modification>> = r#mod;
    r#mod = Util::applyOption(r#mod.clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| renameElementsInModification(__pe_a0, __pe_b1.clone()) }));
    r#mod
}

fn renameElementsInModification(mut r#mod: Arc<Absyn::Modification>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::Modification>> {
    let mut r#mod: Arc<Absyn::Modification> = r#mod;
    assign_field!(
        r#mod.elementArgLst = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
        for mut a in (r#mod.elementArgLst.clone()).into_iter().cloned() {
            let __x = renameElementsInElementArg(a.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        r#mod.eqMod = renameElementsInEqMod(r#mod.eqMod.clone(), nameMap.clone())?
    );
    Ok(r#mod)
}

fn renameElementsInEqMod(mut eqMod: Arc<Absyn::EqMod>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::EqMod>> {
    let mut eqMod: Arc<Absyn::EqMod> = eqMod;
    let () = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => {
            assign_variant_field!(eqMod => Absyn::EqMod::EQMOD; exp = AbsynUtil::traverseExp(var_field!((*eqMod).exp, Absyn::EqMod::EQMOD).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(eqMod)
}

fn renameElementsInExp(mut exp: Arc<Absyn::Exp>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<(Arc<Absyn::Exp>, Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>)> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>> = nameMap;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CREF; componentRef = renameElementsInCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), nameMap.clone(), false)?);
            ()
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            assign_variant_field!(exp => Absyn::Exp::CALL; function_ = renameElementsInCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone(), nameMap.clone(), false)?);
            ()
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { .. } => {
            assign_variant_field!(exp => Absyn::Exp::PARTEVALFUNCTION; function_ = renameElementsInCref(var_field!((*exp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), nameMap.clone(), false)?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, nameMap))
}

fn renameElementsInCref(mut cref: Arc<Absyn::ComponentRef>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>, mut onlySubs: bool) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            if !(onlySubs.clone()) {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; name = renameElementsInIdent((var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), nameMap.clone()));
            }
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL;
                subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone()).into_iter().cloned() {
            let __x = renameElementsInSubscript(s.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                componentRef = renameElementsInCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), nameMap.clone(), true)?
            );
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            if !(onlySubs.clone()) {
                assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = renameElementsInIdent((var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), nameMap.clone()));
            }
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone()).into_iter().cloned() {
            let __x = renameElementsInSubscript(s.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

fn renameElementsInPath(mut path: Arc<Absyn::Path>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Arc<Absyn::Path> {
    let mut path: Arc<Absyn::Path> = path;
    let () = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            assign_variant_field!(path => Absyn::Path::QUALIFIED; name = renameElementsInIdent((var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone(), nameMap.clone()));
            ()
        },
        Deref @ Absyn::Path::IDENT { .. } => {
            assign_variant_field!(path => Absyn::Path::IDENT; name = renameElementsInIdent((var_field!((*path).name, Absyn::Path::IDENT).clone()).clone(), nameMap.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    path
}

fn renameElementsInIdent(mut ident: ArcStr, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> ArcStr {
    let mut ident: ArcStr = ident;
    ident = (UnorderedMap::getOrDefault((ident.clone()).clone(), nameMap.clone(), (ident.clone()).clone())).clone();
    ident
}

fn renameElementsInSubscripts(mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = subs;
    subs = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for mut s in (subs.clone()).into_iter().cloned() {
            let __x = renameElementsInSubscript(s.clone(), nameMap.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(subs)
}

fn renameElementsInSubscript(mut sub: Arc<Absyn::Subscript>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::Subscript>> {
    let mut sub: Arc<Absyn::Subscript> = sub;
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = AbsynUtil::traverseExp(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), Arc::new(renameElementsInExp), nameMap.clone())?.0);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(sub)
}

fn renameElementsInExternalDecl(mut extDecl: Arc<Absyn::ExternalDecl>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::ExternalDecl>> {
    let mut extDecl: Arc<Absyn::ExternalDecl> = extDecl;
    assign_field!(
        extDecl.args = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut a in (extDecl.args.clone()).into_iter().cloned() {
            let __x = (renameElementsInExp(a.clone(), nameMap.clone())?).0;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        extDecl.annotation_ = renameElementsInAnnotationOpt(extDecl.annotation_.clone(), nameMap.clone())
    );
    Ok(extDecl)
}

fn renameElementsInTypeSpec(mut spec: Arc<Absyn::TypeSpec>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Arc<Absyn::TypeSpec>> {
    let mut spec: Arc<Absyn::TypeSpec> = spec;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            assign_variant_field!(spec => Absyn::TypeSpec::TPATH;
                path = renameElementsInPath(var_field!((*spec).path, Absyn::TypeSpec::TPATH).clone(), nameMap.clone()),
                arrayDim = Util::applyOption(var_field!((*spec).arrayDim, Absyn::TypeSpec::TPATH).clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| renameElementsInSubscripts(__pe_a0, __pe_b1.clone()) }))
            );
            ()
        },
        Deref @ Absyn::TypeSpec::TCOMPLEX { .. } => {
            assign_variant_field!(spec => Absyn::TypeSpec::TCOMPLEX;
                path = renameElementsInPath(var_field!((*spec).path, Absyn::TypeSpec::TCOMPLEX).clone(), nameMap.clone()),
                arrayDim = Util::applyOption(var_field!((*spec).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), Arc::new({ let __pe_b1 = nameMap.clone(); move |__pe_a0| renameElementsInSubscripts(__pe_a0, __pe_b1.clone()) }))
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(spec)
}

fn renameElementsInAttributes(mut attrs: Absyn::ElementAttributes, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>) -> Result<Absyn::ElementAttributes> {
    let mut attrs: Absyn::ElementAttributes = attrs;
    attrs.arrayDim = renameElementsInSubscripts(attrs.arrayDim.clone(), nameMap.clone())?;
    Ok(attrs)
}

fn renameElementsInComponentItem(mut component: Arc<Absyn::ComponentItem>, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>, mut renameElement: bool) -> Result<Arc<Absyn::ComponentItem>> {
    let mut component: Arc<Absyn::ComponentItem> = component;
    let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    assign_field!(component.component = renameElementsInComponent(component.component.clone(), nameMap.clone(), renameElement.clone())?);
    if isSome(component.condition.clone()) {
        let __pa0 = ::match_deref::match_deref! { match &(component.condition.clone()) {
            Some(__pa0) => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        exp = __pa0.clone();
        (exp, _) = AbsynUtil::traverseExp(exp.clone(), Arc::new(renameElementsInExp), nameMap.clone())?;
        assign_field!(component.condition = Some(exp.clone()));
    }
    assign_field!(component.comment = renameElementsInCommentOpt(component.comment.clone(), nameMap.clone()));
    Ok(component)
}

fn renameElementsInComponent(mut component: Absyn::Component, mut nameMap: Arc<UnorderedMap::UnorderedMap<ArcStr, ArcStr>>, mut renameElement: bool) -> Result<Absyn::Component> {
    let mut component: Absyn::Component = component;
    if renameElement.clone() {
        component.name = renameElementsInIdent((component.name.clone()).clone(), nameMap.clone());
    }
    component.arrayDim = renameElementsInSubscripts(component.arrayDim.clone(), nameMap.clone())?;
    component.modification = renameElementsInModificationOpt(component.modification.clone(), nameMap.clone());
    Ok(component)
}

pub fn getInheritedAnnotation(mut modelPath: Arc<Absyn::Path>, mut annotationName: ArcStr, mut program: Absyn::Program, mut printConflictWarning: bool) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outAnnotation: Option<Arc<Absyn::Modification>> = None;
    let mut cls: Arc<Absyn::Class>;
    let mut extends_paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut extends_oannl: Arc<metamodelica::List<Option<Arc<Absyn::Modification>>>> = metamodelica::nil();
    let mut cls_ann: Arc<Absyn::Modification>;
    let mut extends_ann: Arc<Absyn::Modification>;
    let mut extends_ann2: Arc<Absyn::Modification>;
    let mut extends_path: Arc<Absyn::Path>;
    cls = getPathedClassInProgram(modelPath.clone(), program.clone(), false, false)?;
    outAnnotation = AbsynUtil::lookupClassAnnotation(cls.clone(), (annotationName.clone()).clone())?;
    ErrorExt::setCheckpoint((literal!("InteractiveUtil.getInheritedAnnotation")).clone());
    match '__try0: {
        extends_paths = unwrap_break_err!(NFApi::getInheritedClasses(modelPath.clone(), program.clone()), '__try0);
        Ok::<_, anyhow::Error>((extends_paths.clone(),))
    } {
        Ok((__try0_o0,)) => {
            extends_paths = __try0_o0;
        }
        Err(_) => {
            extends_paths = metamodelica::nil();
        }
    }
    ErrorExt::rollBack((literal!("InteractiveUtil.getInheritedAnnotation")).clone());
    if extends_paths.clone().is_empty() {
        return Ok(outAnnotation);
    }
    extends_oannl = {
        let mut __acc: Arc<metamodelica::List<Option<Arc<Absyn::Modification>>>> = metamodelica::nil();
        for mut ep in (extends_paths.clone()).into_iter().cloned() {
            let __x = getInheritedAnnotation(ep.clone(), (annotationName.clone()).clone(), program.clone(), true)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    while !(extends_oannl.clone().is_empty()) {
        if isSome(listHead(extends_oannl.clone())?) {
            extends_ann = Util::getOption(listHead(extends_oannl.clone())?)?;
            if isSome(outAnnotation.clone()) {
                outAnnotation = Some(AbsynUtil::mergeModifiers(Util::getOption(outAnnotation.clone())?, extends_ann.clone())?);
            } else {
                outAnnotation = Some(extends_ann.clone());
            }
            if printConflictWarning.clone() {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(extends_paths.clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                extends_path = __pa1.clone();
                extends_paths = __pa2.clone();
                for mut a in &*listRest(extends_oannl.clone())? {
                    let mut a = a.clone();
                    if isSome(a.clone()) {
                        let __pa3 = ::match_deref::match_deref! { match &(a.clone()) {
                            Some(__pa3) => __pa3.clone(),
                            _ => bail!("pattern mismatch"),
                        } };
                        extends_ann2 = __pa3.clone();
                        if !(extends_ann.clone() == extends_ann2.clone()) {
                            Error::addMessage(Error::CONFLICTING_INHERITED_ANNOTATIONS.clone(), list![(annotationName.clone()).clone(), (AbsynUtil::pathString(modelPath.clone(), (literal!(".")).clone(), true, false)?).clone(), (Dump::unparseModificationStr(extends_ann.clone())?).clone(), (AbsynUtil::pathString(extends_path.clone(), (literal!(".")).clone(), true, false)?).clone(), (Dump::unparseModificationStr(extends_ann2.clone())?).clone(), (AbsynUtil::pathString(listHead(extends_paths.clone())?, (literal!(".")).clone(), true, false)?).clone()])?;
                            break;
                        }
                        extends_paths = listRest(extends_paths.clone())?;
                    }
                }
            }
            return Ok(outAnnotation);
        }
        extends_oannl = listRest(extends_oannl.clone())?;
        extends_paths = listRest(extends_paths.clone())?;
    }
    Ok(outAnnotation)
}

pub fn setElementType(mut elementPath: Arc<Absyn::Path>, mut className: Arc<Absyn::ComponentRef>, mut program: Absyn::Program) -> (Absyn::Program, bool) {
    let mut program: Absyn::Program = program;
    let mut success: bool = true;
    let mut ann: Option<Arc<Absyn::Annotation>> = None;
    let mut name: ArcStr = arcstr::literal!("");
    let mut elem_opt: Option<Arc<Absyn::Element>> = None;
    let mut ty: Arc<Absyn::TypeSpec>;
    match '__try0: {
        ty = unwrap_break_err!(AbsynUtil::crefToTypeSpec(className.clone()), '__try0);
        (program, elem_opt, success) = unwrap_break_err!(transformPathedElementInProgram(elementPath.clone(), Arc::new({ let __pe_b1 = ty.clone(); let __pe_b2 = false; move |__pe_a0| AbsynUtil::setElementType(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), program.clone()), '__try0);
        if success.clone() {
            unwrap_break_err!(SymbolTable::setAbsynElement(program.clone(), Util::getOption(elem_opt.clone()).unwrap(), elementPath.clone()), '__try0);
        }
        Ok::<_, anyhow::Error>((elem_opt.clone(), program.clone(), success.clone(), ty.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            elem_opt = __try0_o0;
            program = __try0_o1;
            success = __try0_o2;
            ty = __try0_o3;
        }
        Err(_) => {
            success = false;
            panic!("try/else: outputs not set in else branch");
        }
    }
    (program, success)
}

pub fn makeCommentFromArgs(mut commentExp: Arc<Absyn::Exp>, mut annotationExp: Arc<Absyn::Exp>, mut oldComment: Option<Arc<Absyn::Comment>>) -> Result<Option<Arc<Absyn::Comment>>> {
    let mut comment: Option<Arc<Absyn::Comment>> = None;
    let mut ann: Option<Arc<Absyn::Annotation>> = None;
    let mut old_ann: Option<Arc<Absyn::Annotation>> = None;
    let mut cmt: Option<ArcStr> = None;
    let mut old_cmt: Option<ArcStr> = None;
    cmt = (::match_deref::match_deref! { match &(commentExp.clone()) {
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Nil } => None,
        Deref @ Absyn::Exp::STRING { .. } => Some((var_field!((*commentExp).value, Absyn::Exp::STRING).clone()).clone()),
        _ => bail!("match: no arm matched"),
    } });
    ann = (::match_deref::match_deref! { match &(annotationExp.clone()) {
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Nil } => None,
        _ => Some(Arc::new(Absyn::Annotation { elementArgs: list![recordConstructorToModification(annotationExp.clone())?] })),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    if isSome(cmt.clone()) || isSome(ann.clone()) {
        cmt = if (isSome(cmt.clone())) {cmt.clone()} else {AbsynUtil::getCommentOptComment(oldComment.clone())?};
        ann = if (isSome(ann.clone())) {ann.clone()} else {AbsynUtil::getCommentOptAnnotation(oldComment.clone())?};
        comment = Some(Arc::new(Absyn::Comment { annotation_: ann.clone(), comment: cmt.clone() }));
    } else {
        comment = oldComment.clone();
    }
    Ok(comment)
}

pub fn makeModifierFromArgs(mut bindingExp: Arc<Absyn::Exp>, mut modifier: Arc<Absyn::Modification>, mut info: SourceInfo, mut oldModifier: Option<Arc<Absyn::Modification>>) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outModifier: Option<Arc<Absyn::Modification>> = None;
    outModifier = (::match_deref::match_deref! { match &((bindingExp.clone(), modifier.clone())) {
        (Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Nil }, Deref @ Absyn::Modification { elementArgLst: Deref @ metamodelica::List::Nil, .. }) => oldModifier.clone(),
        (Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::List::Nil }, _) => Some(modifier.clone()),
        (_, Deref @ Absyn::Modification { .. }) => Some(Arc::new(Absyn::Modification { elementArgLst: modifier.elementArgLst.clone(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: bindingExp.clone(), info: info.clone() }) })),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outModifier)
}

pub fn accessClass(mut classPath: Arc<Absyn::Path>, mut program: Absyn::Program, mut r#fn: Arc<dyn ::std::ops::Fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>> + 'static>, mut evaluateParams: bool, mut graphicsExpMode: bool, mut accessLevel: Access) -> Result<Arc<Values::Value>> {
    pub type Fn = fn(Arc<Absyn::Path>, Absyn::Program, Access) -> Result<Arc<Values::Value>>;

    let mut result: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    let mut access: Access = Access::hide;
    let mut silent: bool = false;
    let mut eval_params: bool = false;
    let mut graphics_exp_mode: bool = false;
    eval_params = Config::getEvaluateParametersInAnnotations()?;
    graphics_exp_mode = Config::getGraphicsExpMode()?;
    match '__try0: {
        access = Interactive::checkAccessAnnotationAndEncryption(classPath.clone(), program.clone());
        if access.clone() < accessLevel.clone() {
            unwrap_break_err!(Error::addMessage(Error::ACCESS_ENCRYPTED_PROTECTED_CONTENTS.clone(), metamodelica::nil()), '__try0);
            result = ValuesMake::makeBoolean(false);
            return Ok(result);
        }
        silent = !(unwrap_break_err!(Flags::isSet(Flags::NF_API_NOISE.clone()), '__try0));
        if silent.clone() {
            ErrorExt::setCheckpoint((literal!("InteractiveUtil.accessClass")).clone());
        }
        unwrap_break_err!(Config::setEvaluateParametersInAnnotations(evaluateParams.clone()), '__try0);
        unwrap_break_err!(Config::setGraphicsExpMode(graphicsExpMode.clone()), '__try0);
        result = unwrap_break_err!(r#fn(classPath.clone(), program.clone(), access.clone()), '__try0);
        Ok::<_, anyhow::Error>((result.clone(),))
    } {
        Ok((__try0_o0,)) => {
            result = __try0_o0;
        }
        Err(_) => {
            result = ValuesMake::makeBoolean(false);
        }
    }
    if silent.clone() {
        ErrorExt::rollBack((literal!("InteractiveUtil.accessClass")).clone());
    }
    Config::setGraphicsExpMode(graphics_exp_mode.clone())?;
    Config::setEvaluateParametersInAnnotations(eval_params.clone())?;
    Ok(result)
}

pub fn makeAnnotationArrayValue(mut annotations: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Values::Value>> {
    let mut arr: Arc<Values::Value> = Arc::new(Values::Value::META_FAIL);
    arr = ValuesMake::makeArray({
        let mut __acc: Arc<metamodelica::List<Arc<Values::Value>>> = metamodelica::nil();
        for mut s in (annotations.clone()).into_iter().cloned() {
            let __x = ValuesMake::makeCodeTypeNameStr((s.clone()).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })?;
    Ok(arr)
}

pub fn parseWithinPath(mut path: Arc<Absyn::Path>) -> Absyn::Within {
    let mut outWithin: Absyn::Within = Absyn::Within::TOP;
    outWithin = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "__OpenModelica_TopLevel" } => openmodelica_ast::Absyn::Within::TOP,
        _ => Absyn::Within::WITHIN { path: path.clone() },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outWithin
}

pub fn offsetAnnotationsInClassDef(mut cdef: Arc<Absyn::ClassDef>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ClassDef>> {
    let mut cdef: Arc<Absyn::ClassDef> = cdef;
    if x.clone() == 0 && y.clone() == 0 {
        return Ok(cdef);
    }
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS;
                classParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = offsetAnnotationsInClassPart(p.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone()).into_iter().cloned() {
            let __x = offsetDiagramAnnotation(a.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED; comment = offsetDiagramAnnotationInOptComment(var_field!((*cdef).comment, Absyn::ClassDef::DERIVED).clone(), x.clone(), y.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::ENUMERATION; comment = offsetDiagramAnnotationInOptComment(var_field!((*cdef).comment, Absyn::ClassDef::ENUMERATION).clone(), x.clone(), y.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::OVERLOAD { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::OVERLOAD; comment = offsetDiagramAnnotationInOptComment(var_field!((*cdef).comment, Absyn::ClassDef::OVERLOAD).clone(), x.clone(), y.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS;
                parts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = offsetAnnotationsInClassPart(p.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
                ann = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Annotation>>> = metamodelica::nil();
        for mut a in (var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()).into_iter().cloned() {
            let __x = offsetDiagramAnnotation(a.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::ClassDef::PDER { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PDER; comment = offsetDiagramAnnotationInOptComment(var_field!((*cdef).comment, Absyn::ClassDef::PDER).clone(), x.clone(), y.clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cdef)
}

pub fn offsetAnnotationsInClassPart(mut part: Arc<Absyn::ClassPart>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ClassPart>> {
    let mut part: Arc<Absyn::ClassPart> = part;
    let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PUBLIC; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut i in (var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone()).into_iter().cloned() {
            let __x = offsetAnnotationsInElementItem(i.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::PROTECTED; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut i in (var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone()).into_iter().cloned() {
            let __x = offsetAnnotationsInElementItem(i.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
        for mut e in (var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone()).into_iter().cloned() {
            let __x = offsetAnnotationsInEquationItem(e.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(part)
}

pub fn offsetAnnotationsInElementItem(mut item: Arc<Absyn::ElementItem>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ElementItem>> {
    let mut item: Arc<Absyn::ElementItem> = item;
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => {
            assign_variant_field!(item => Absyn::ElementItem::ELEMENTITEM; element = offsetAnnotationsInElement(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone(), x.clone(), y.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

pub fn offsetAnnotationsInElement(mut element: Arc<Absyn::Element>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = offsetAnnotationsInElementSpec(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), x.clone(), y.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub fn offsetAnnotationsInElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ElementSpec>> {
    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
        for mut c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
            let __x = offsetAnnotationsInComponentItem(c.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(spec)
}

pub static PLACEMENT_ORIGIN_PATH: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Placement")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("transformation")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("origin")).clone() }) }) }) });

pub static PLACEMENT_ICON_TRANSFORMATION_PATH: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Placement")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("iconTransformation")).clone() }) }) });

pub static LINE_POINTS_PATH: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Line")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("points")).clone() }) }) });

pub static DIAGRAM_GRAPHICS_PATH: std::sync::LazyLock<Arc<Absyn::Path>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Diagram")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("graphics")).clone() }) }) });

pub fn offsetAnnotationsInComponentItem(mut item: Arc<Absyn::ComponentItem>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ComponentItem>> {
    let mut item: Arc<Absyn::ComponentItem> = item;
    let mut oann: Option<Arc<Absyn::Annotation>> = None;
    let mut ann: Arc<Absyn::Annotation>;
    oann = AbsynUtil::getCommentOptAnnotation(item.comment.clone())?;
    ann = if (isSome(oann.clone())) {Util::getOption(oann.clone())?} else {Arc::new(Absyn::Annotation { elementArgs: metamodelica::nil() })};
    ann = AbsynUtil::transformAnnotationArg(ann.clone(), PLACEMENT_ORIGIN_PATH.clone(), Arc::new({ let __pe_b1 = x.clone(); let __pe_b2 = y.clone(); move |__pe_a0| offsetOriginAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), true)?;
    ann = offsetIconTransformationAnnotation(ann.clone(), x.clone(), y.clone());
    item = AbsynUtil::setComponentItemAnnotation(item.clone(), Some(ann.clone()))?;
    Ok(item)
}

pub fn offsetIconTransformationAnnotation(mut ann: Arc<Absyn::Annotation>, mut x: i32, mut y: i32) -> Arc<Absyn::Annotation> {
    fn r#impl(mut arg: Arc<Absyn::ElementArg>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ElementArg>> {
        let mut arg: Arc<Absyn::ElementArg> = arg;
        let mut r#mod: Arc<Absyn::Modification>;
        let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(r#mod), .. } => {
            let mut r#mod = (*r#mod).clone();
            assign_field!(r#mod.elementArgLst = AbsynUtil::transformAnnotationInArgs(r#mod.elementArgLst.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("origin")).clone() }), Arc::new({ let __pe_b1 = x.clone(); let __pe_b2 = y.clone(); move |__pe_a0| offsetOriginAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), true)?);
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(arg)
    }

    let mut ann: Arc<Absyn::Annotation> = ann;
    match '__try0: {
        ann = unwrap_break_err!(AbsynUtil::transformAnnotationArg(ann.clone(), PLACEMENT_ICON_TRANSFORMATION_PATH.clone(), Arc::new({ let __pe_b1 = x.clone(); let __pe_b2 = y.clone(); move |__pe_a0| r#impl(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), false), '__try0);
        Ok::<_, anyhow::Error>((ann.clone(),))
    } {
        Ok((__try0_o0,)) => {
            ann = __try0_o0;
        }
        Err(_) => {
            panic!("try/else: outputs not set in else branch");
        }
    }
    ann
}

pub fn offsetOriginAnnotation(mut arg: Arc<Absyn::ElementArg>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ElementArg>> {
    let mut arg: Arc<Absyn::ElementArg> = arg;
    let mut r#mod: Arc<Absyn::Modification>;
    let mut eq_mod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            if isSome(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                let __pa0 = ::match_deref::match_deref! { match &(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                r#mod = __pa0.clone();
            } else {
                r#mod = Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) });
            }
            eq_mod = r#mod.eqMod.clone();
            assign_field!(r#mod.eqMod = (::match_deref::match_deref! { match &(eq_mod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => {
            assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = offsetPointExpression(var_field!((*eq_mod).exp, Absyn::EqMod::EQMOD).clone(), x.clone(), y.clone()));
            eq_mod.clone()
        },
        _ => Arc::new(Absyn::EqMod::EQMOD { exp: makeOrigin(x.clone(), y.clone()), info: Absyn::dummyInfo.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } }));
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(arg)
}

pub fn makeOrigin(mut x: i32, mut y: i32) -> Arc<Absyn::Exp> {
    let mut origin: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::ARRAY { arrayExp: list![Arc::new(Absyn::Exp::INTEGER { value: x.clone() }), Arc::new(Absyn::Exp::INTEGER { value: y.clone() })] });
    origin
}

pub fn offsetPointExpression(mut point: Arc<Absyn::Exp>, mut x: i32, mut y: i32) -> Arc<Absyn::Exp> {
    let mut point: Arc<Absyn::Exp> = point;
    let mut e1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut e2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    point = (::match_deref::match_deref! { match &(point.clone()) {
        Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Cons { head: e1, tail: Deref @ metamodelica::List::Cons { head: e2, tail: Deref @ metamodelica::List::Nil } } } => Arc::new(Absyn::Exp::ARRAY { arrayExp: list![offsetIntegerExpression(e1.clone(), x.clone()), offsetIntegerExpression(e2.clone(), y.clone())] }),
        _ => Arc::new(Absyn::Exp::BINARY { exp1: point.clone(), op: openmodelica_ast::Absyn::Operator::ADD, exp2: makeOrigin(x.clone(), y.clone()) }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    point
}

pub fn offsetIntegerExpression(mut exp: Arc<Absyn::Exp>, mut offset: i32) -> Arc<Absyn::Exp> {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut v: i32 = 0;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::INTEGER { .. } => Arc::new(Absyn::Exp::INTEGER { value: var_field!((*exp).value, Absyn::Exp::INTEGER).clone() + offset.clone() }),
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::INTEGER { value: v }, op: Absyn::Operator::UPLUS { .. } } => Arc::new(Absyn::Exp::INTEGER { value: v.clone() + offset.clone() }),
        Deref @ Absyn::Exp::UNARY { exp: Deref @ Absyn::Exp::INTEGER { value: v }, op: Absyn::Operator::UMINUS { .. } } => Arc::new(Absyn::Exp::INTEGER { value: -(v.clone()) + offset.clone() }),
        _ => if (offset.clone() > 0) {Arc::new(Absyn::Exp::BINARY { exp1: exp.clone(), op: openmodelica_ast::Absyn::Operator::ADD, exp2: Arc::new(Absyn::Exp::INTEGER { value: offset.clone() }) })} else if (offset.clone() < 0) {Arc::new(Absyn::Exp::BINARY { exp1: exp.clone(), op: openmodelica_ast::Absyn::Operator::SUB, exp2: Arc::new(Absyn::Exp::INTEGER { value: -(offset.clone()) }) })} else {exp.clone()},
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp
}

pub fn offsetLineExpression(mut line: Arc<Absyn::Exp>, mut x: i32, mut y: i32) -> Arc<Absyn::Exp> {
    let mut line: Arc<Absyn::Exp> = line;
    let () = (::match_deref::match_deref! { match &(line.clone()) {
        Deref @ Absyn::Exp::ARRAY { .. } => {
            assign_variant_field!(line => Absyn::Exp::ARRAY; arrayExp = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut p in (var_field!((*line).arrayExp, Absyn::Exp::ARRAY).clone()).into_iter().cloned() {
            let __x = offsetPointExpression(p.clone(), x.clone(), y.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    line
}

pub fn offsetAnnotationsInEquationItem(mut item: Arc<Absyn::EquationItem>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::EquationItem>> {
    let mut item: Arc<Absyn::EquationItem> = item;
    let mut cmt: Arc<Absyn::Comment>;
    let mut ann: Arc<Absyn::Annotation>;
    let () = 'mc: {
        let __mc_input = item.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::EquationItem::EQUATIONITEM { comment: Some(cmt @ Deref @ Absyn::Comment { annotation_: Some(ann), .. }), .. } => {
                    let mut cmt = (*cmt).clone();
                    let mut ann = (*ann).clone();
                    let mut item: Arc<Absyn::EquationItem> = item.clone();
                    ann = AbsynUtil::transformAnnotationArg(ann.clone(), LINE_POINTS_PATH.clone(), Arc::new({ let __pe_b1 = x.clone(); let __pe_b2 = y.clone(); move |__pe_a0| Ok(offsetConnectionLineAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }), false)?;
                    todo!("unhandled field-assign shape: cmt.annotation_");
                    assign_variant_field!(item => Absyn::EquationItem::EQUATIONITEM; comment = Some(cmt.clone()));
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
    Ok(item)
}

pub fn offsetConnectionLineAnnotation(mut arg: Arc<Absyn::ElementArg>, mut x: i32, mut y: i32) -> Arc<Absyn::ElementArg> {
    let mut arg: Arc<Absyn::ElementArg> = arg;
    let mut r#mod: Arc<Absyn::Modification>;
    let mut eq_mod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: eq_mod @ Deref @ Absyn::EqMod::EQMOD { .. }, .. }), .. } => {
            let mut eq_mod = (*eq_mod).clone();
            assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = offsetLineExpression(var_field!((*eq_mod).exp, Absyn::EqMod::EQMOD).clone(), x.clone(), y.clone()));
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: eq_mod.clone() })));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    arg
}

pub fn offsetDiagramAnnotationInOptComment(mut cmt: Option<Arc<Absyn::Comment>>, mut x: i32, mut y: i32) -> Result<Option<Arc<Absyn::Comment>>> {
    let mut cmt: Option<Arc<Absyn::Comment>> = cmt;
    let mut cmt_str: Option<ArcStr> = None;
    let mut ann: Arc<Absyn::Annotation>;
    let () = (::match_deref::match_deref! { match &(cmt.clone()) {
        Some(Deref @ Absyn::Comment { annotation_: Some(ann), comment: cmt_str }) => {
            let mut ann = (*ann).clone();
            ann = offsetDiagramAnnotation(ann.clone(), x.clone(), y.clone())?;
            cmt = Some(Arc::new(Absyn::Comment { annotation_: Some(ann.clone()), comment: cmt_str.clone() }));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cmt)
}

pub fn offsetDiagramAnnotation(mut ann: Arc<Absyn::Annotation>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::Annotation>> {
    let mut ann: Arc<Absyn::Annotation> = ann;
    ann = AbsynUtil::transformAnnotationArg(ann.clone(), DIAGRAM_GRAPHICS_PATH.clone(), Arc::new({ let __pe_b1 = x.clone(); let __pe_b2 = y.clone(); move |__pe_a0| offsetGraphicsAnnotation(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }), true)?;
    Ok(ann)
}

pub fn offsetGraphicsAnnotation(mut arg: Arc<Absyn::ElementArg>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::ElementArg>> {
    let mut arg: Arc<Absyn::ElementArg> = arg;
    let mut r#mod: Arc<Absyn::Modification>;
    let mut eq_mod: Arc<Absyn::EqMod> = Arc::new(Absyn::EqMod::NOMOD);
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Deref @ Absyn::Modification { eqMod: eq_mod @ Deref @ Absyn::EqMod::EQMOD { .. }, .. }), .. } => {
            let mut eq_mod = (*eq_mod).clone();
            assign_variant_field!(eq_mod => Absyn::EqMod::EQMOD; exp = offsetGraphicsExpression(var_field!((*eq_mod).exp, Absyn::EqMod::EQMOD).clone(), x.clone(), y.clone())?);
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: eq_mod.clone() })));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(arg)
}

pub fn offsetGraphicsExpression(mut graphics: Arc<Absyn::Exp>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::Exp>> {
    let mut graphics: Arc<Absyn::Exp> = graphics;
    let () = (::match_deref::match_deref! { match &(graphics.clone()) {
        Deref @ Absyn::Exp::ARRAY { .. } => {
            assign_variant_field!(graphics => Absyn::Exp::ARRAY; arrayExp = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut p in (var_field!((*graphics).arrayExp, Absyn::Exp::ARRAY).clone()).into_iter().cloned() {
            let __x = offsetGraphicsItemExpression(p.clone(), x.clone(), y.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(graphics)
}

pub fn offsetGraphicsItemExpression(mut item: Arc<Absyn::Exp>, mut x: i32, mut y: i32) -> Result<Arc<Absyn::Exp>> {
    fn offset_named_origin(mut arg: Arc<Absyn::NamedArg>, mut x: i32, mut y: i32) -> (Arc<Absyn::NamedArg>, bool) {
        let mut arg: Arc<Absyn::NamedArg> = arg;
        let mut found: bool = false;
        found = arg.argName.clone() == literal!("origin");
        if found.clone() {
            assign_field!(arg.argValue = offsetPointExpression(arg.argValue.clone(), x.clone(), y.clone()));
        }
        (arg, found)
    }

    let mut item: Arc<Absyn::Exp> = item;
    let mut args: Arc<Absyn::FunctionArgs>;
    let mut named_args: Arc<metamodelica::List<Arc<Absyn::NamedArg>>> = metamodelica::nil();
    let mut found: bool = false;
    let mut visible: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut origin: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    let mut rest: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    let () = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::Exp::CALL { functionArgs: args @ Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. }, .. } => {
            let mut args = (*args).clone();
            if (var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone().len() as i32) >= 2 {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone()) {
                    Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Cons { head: __pa1, tail: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => bail!("pattern mismatch"),
                } };
                visible = __pa0.clone();
                origin = __pa1.clone();
                rest = __pa2.clone();
                origin = offsetPointExpression(origin.clone(), x.clone(), y.clone());
                assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; args = cons(visible.clone(), cons(origin.clone(), rest.clone())));
            } else {
                (named_args, found) = List::findMap(var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone(), Arc::new({ let __pe_b1 = x.clone(); let __pe_b2 = y.clone(); move |__pe_a0| Ok(offset_named_origin(__pe_a0, __pe_b1.clone(), __pe_b2.clone())) }))?;
                if found.clone() {
                    assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; argNames = named_args.clone());
                } else {
                    assign_variant_field!(args => Absyn::FunctionArgs::FUNCTIONARGS; argNames = cons(Arc::new(Absyn::NamedArg { argName: (literal!("origin")).clone(), argValue: makeOrigin(x.clone(), y.clone()) }), var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone()));
                }
            }
            assign_variant_field!(item => Absyn::Exp::CALL; functionArgs = args.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(item)
}

pub fn addToPublic(mut cls: Arc<Absyn::Class>, mut element: Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut cdef: Arc<Absyn::ClassDef>;
    let () = 'mc: {
        let __mc_input = cls.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = elems.clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    elems = getPublicList(var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone());
                    elems = List::appendElt(element.clone(), elems.clone());
                    let __owned_variant_classParts_0 = replacePublicList(var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone(), elems.clone())?;
                    if let Absyn::ClassDef::PARTS { classParts, .. } = &mut cdef {
                        *classParts = __owned_variant_classParts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let __owned_variant_classParts_0 = cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![element.clone()] }), var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone());
                    if let Absyn::ClassDef::PARTS { classParts, .. } = &mut cdef {
                        *classParts = __owned_variant_classParts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = elems.clone();
                    elems = getPublicList(var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone());
                    elems = List::appendElt(element.clone(), elems.clone());
                    let __owned_variant_parts_0 = replacePublicList(var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), elems.clone())?;
                    if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut cdef {
                        *parts = __owned_variant_parts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let __owned_variant_parts_0 = cons(Arc::new(Absyn::ClassPart::PUBLIC { contents: list![element.clone()] }), var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone());
                    if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut cdef {
                        *parts = __owned_variant_parts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cls)
}

pub fn addToProtected(mut cls: Arc<Absyn::Class>, mut element: Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut cdef: Arc<Absyn::ClassDef>;
    let () = 'mc: {
        let __mc_input = cls.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = elems.clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    elems = getProtectedList(var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone());
                    elems = List::appendElt(element.clone(), elems.clone());
                    let __owned_variant_classParts_0 = replaceProtectedList(var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone(), elems.clone())?;
                    if let Absyn::ClassDef::PARTS { classParts, .. } = &mut cdef {
                        *classParts = __owned_variant_classParts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let __owned_variant_classParts_0 = cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: list![element.clone()] }), var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone());
                    if let Absyn::ClassDef::PARTS { classParts, .. } = &mut cdef {
                        *classParts = __owned_variant_classParts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let mut elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = elems.clone();
                    elems = getProtectedList(var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone());
                    elems = List::appendElt(element.clone(), elems.clone());
                    let __owned_variant_parts_0 = replaceProtectedList(var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), elems.clone())?;
                    if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut cdef {
                        *parts = __owned_variant_parts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let __owned_variant_parts_0 = cons(Arc::new(Absyn::ClassPart::PROTECTED { contents: list![element.clone()] }), var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone());
                    if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut cdef {
                        *parts = __owned_variant_parts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cls)
}

pub fn addToEquation(mut cls: Arc<Absyn::Class>, mut eq: Arc<Absyn::EquationItem>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = metamodelica::nil();
    let mut cdef: Arc<Absyn::ClassDef>;
    let () = 'mc: {
        let __mc_input = cls.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = eqlst.clone();
                    eqlst = getEquationList(var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone())?;
                    eqlst = List::appendElt(eq.clone(), eqlst.clone());
                    let __owned_variant_classParts_0 = replaceEquationList(var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone(), eqlst.clone())?;
                    if let Absyn::ClassDef::PARTS { classParts, .. } = &mut cdef {
                        *classParts = __owned_variant_classParts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let __owned_variant_classParts_0 = List::appendElt(Arc::new(Absyn::ClassPart::EQUATIONS { contents: list![eq.clone()] }), var_field!((**cdef).classParts, Absyn::ClassDef::PARTS).clone());
                    if let Absyn::ClassDef::PARTS { classParts, .. } = &mut cdef {
                        *classParts = __owned_variant_classParts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let mut eqlst: Arc<metamodelica::List<Arc<Absyn::EquationItem>>> = eqlst.clone();
                    eqlst = getEquationList(var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?;
                    eqlst = List::appendElt(eq.clone(), eqlst.clone());
                    let __owned_variant_parts_0 = replaceEquationList(var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), eqlst.clone())?;
                    if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut cdef {
                        *parts = __owned_variant_parts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Class { body: cdef @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
                    let mut cdef = (*cdef).clone();
                    let mut cls: Arc<Absyn::Class> = cls.clone();
                    let __owned_variant_parts_0 = List::appendElt(Arc::new(Absyn::ClassPart::EQUATIONS { contents: list![eq.clone()] }), var_field!((**cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone());
                    if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut cdef {
                        *parts = __owned_variant_parts_0;
                    } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
                    assign_field!(cls.body = cdef.clone());
                    Ok(())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(cls)
}

