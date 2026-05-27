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

use crate::Interactive;
use crate::InteractiveUtil;
use openmodelica_ast::Absyn;
use openmodelica_ast::GlobalScript;
use openmodelica_frontend::Parser;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_types::SCode;
use openmodelica_util::System;

// Imports
// Aliases
pub type Ident = ArcStr;

pub type Path = Arc<Absyn::Path>;

pub type TypeSpec = Arc<Absyn::TypeSpec>;

// Types
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mediator {
    pub mType: ArcStr,
    pub template: ArcStr,
    pub clients: Arc<metamodelica::List<Client>>,
    pub providers: Arc<metamodelica::List<Provider>>,
    pub preferred: Arc<metamodelica::List<Preferred>>,
}

pub type MEDIATOR = Mediator;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Client {
    pub modelID: ArcStr,
    pub component: ArcStr,
    pub template: ArcStr,
    pub isMandatory: bool,
}

pub type CLIENT = Client;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Provider {
    pub modelID: ArcStr,
    pub component: ArcStr,
    pub template: ArcStr,
}

pub type PROVIDER = Provider;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Preferred {
    pub clientInstancePath: ArcStr,
    pub providerInstancePath: ArcStr,
}

pub type PREFERRED = Preferred;


/// internal client list representation
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Client_e {
    CLIENT_E {
        components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>,
        typeSpec: TypeSpec,
        rootType: TypeSpec,
        def: Arc<Absyn::Class>,
        instance: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>,
        predecessors: Arc<Client_e>,
        mediator: Arc<metamodelica::List<Mediator>>,
    },
    NO_PRED,
}
pub use self::Client_e::{CLIENT_E,NO_PRED};

pub fn inferBindings(mut model_path: Path, mut env: Absyn::Program) -> Result<Absyn::Program> {
    let mut out_model_def: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut ms: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
    let mut model_def: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut out_vmodel: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut scode_def: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut client_list: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
    model_def = InteractiveUtil::getPathedClassInProgram(model_path.clone(), env.clone(), false, false)?;
    scode_def = AbsynToSCode::translateAbsyn2SCode(env.clone())?;
    ms = getMediatorDefsElements(scode_def.clone(), metamodelica::nil())?;
    client_list = buildInstList(model_def.clone(), env.clone(), Arc::new(crate::Binding::Client_e::NO_PRED), ms.clone(), metamodelica::nil(), metamodelica::nil())?;
    out_vmodel = inferBindingClientList(client_list.clone(), model_def.clone(), env.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::unparseClassStr(out_vmodel.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    out_model_def = InteractiveUtil::updateProgram(Absyn::Program { classes: list![out_vmodel.clone()], within_: InteractiveUtil::buildWithin(model_path.clone())? }, env.clone(), false)?;
    Ok(out_model_def)
}

pub fn generateVerificationScenarios(mut package_path: Path, mut in_env: Absyn::Program) -> Result<Absyn::Program> {
    let mut out_env: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut ms: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
    let mut package_def: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut out_vmodel: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut autogen_class: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut autogen_class2: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut scode_def: Arc<metamodelica::List<Arc<SCode::Element>>> = metamodelica::nil();
    let mut design_alts: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>> = metamodelica::nil();
    let mut reqs: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>> = metamodelica::nil();
    let mut scenarios: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>> = metamodelica::nil();
    let mut client_list: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
    let mut ag_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut autogen_model_list: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut autogen_model: Arc<Absyn::ElementItem>;
    let mut i: i32 = 0;
    scode_def = AbsynToSCode::translateAbsyn2SCode(in_env.clone())?;
    design_alts = getAllElementsOfType(scode_def.clone(), (literal!("VVDRlib.Verification.Design")).clone(), (literal!("")).clone(), metamodelica::nil())?;
    reqs = getAllElementsOfType(scode_def.clone(), (literal!("VVDRlib.Verification.Requirement")).clone(), (literal!("")).clone(), metamodelica::nil())?;
    scenarios = getAllElementsOfType(scode_def.clone(), (literal!("VVDRlib.Verification.Scenario")).clone(), (literal!("")).clone(), metamodelica::nil())?;
    ms = getMediatorDefsElements(scode_def.clone(), metamodelica::nil())?;
    package_def = InteractiveUtil::getPathedClassInProgram(package_path.clone(), in_env.clone(), false, false)?;
    autogen_model_list = metamodelica::nil();
    i = 0;
    for mut s in &*scenarios.clone() {
        let mut s = s.clone();
        for mut d in &*design_alts.clone() {
            let mut d = d.clone();
            ag_elems = populateModel(cons(s.clone(), cons(d.clone(), reqs.clone())), 0, metamodelica::nil())?;
            ag_elems = cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::EXTENDS { path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("VVDRlib")).clone(), path: Arc::new(Absyn::Path::QUALIFIED { name: (literal!("Verification")).clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("VerificationModel")).clone() }) }) }), elementArg: metamodelica::nil(), annotationOpt: None }), info: Absyn::dummyInfo.clone(), constrainClass: None }) }), ag_elems.clone());
            autogen_class = Arc::new(Absyn::Class { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("verif_model_autogen_")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), partialPrefix: false, finalPrefix: false, encapsulatedPrefix: false, restriction: openmodelica_ast::Absyn::Restriction::R_MODEL, body: Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: list![Arc::new(Absyn::ClassPart::PUBLIC { contents: ag_elems.clone() })], ann: metamodelica::nil(), comment: Some((literal!("Autogenerated verification model")).clone()) }), commentsBeforeClass: metamodelica::nil(), commentsBeforeEnd: metamodelica::nil(), commentsAfterEnd: metamodelica::nil(), info: Absyn::dummyInfo.clone() });
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** Autogenerated class: ")); __mm_s.push_str(&*literal!("verif_model_autogen_")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            client_list = buildInstList(autogen_class.clone(), in_env.clone(), Arc::new(crate::Binding::Client_e::NO_PRED), ms.clone(), metamodelica::nil(), metamodelica::nil())?;
            autogen_class2 = inferBindingClientList(client_list.clone(), autogen_class.clone(), in_env.clone())?;
            autogen_model = Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: false, class_: autogen_class2.clone() }), info: Absyn::dummyInfo.clone(), constrainClass: None }) });
            autogen_model_list = cons(autogen_model.clone(), autogen_model_list.clone());
            i = i.clone() + 1;
        }
    }
    out_vmodel = updatePackage(package_def.clone(), autogen_model_list.clone())?;
    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("******** Autogenerated classes:\n")); __mm_s.push_str(&*Dump::unparseClassStr(out_vmodel.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    out_env = InteractiveUtil::updateProgram(Absyn::Program { classes: list![out_vmodel.clone()], within_: InteractiveUtil::buildWithin(package_path.clone())? }, in_env.clone(), false)?;
    Ok(out_env)
}

pub fn updatePackage(mut in_class: Arc<Absyn::Class>, mut ag_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<Absyn::Class>> {
    let mut out_class: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    out_class = (::match_deref::match_deref! { match &(in_class.clone()) {
        out_class @ Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts: _, ann, comment }, .. } => {
            let mut out_class = (*out_class).clone();
            assign_field!(out_class.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: list![Arc::new(Absyn::ClassPart::PUBLIC { contents: ag_elems.clone() })], ann: ann.clone(), comment: comment.clone() }));
            out_class.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_class)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn populateModel(mut element_defs: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>>, mut autoVal: i32, mut elements_in: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut elements_out: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    elements_out = (::match_deref::match_deref! { match &(element_defs.clone()) {
        Deref @ metamodelica::List::Nil => {
            elements_in.clone()
        },
        Deref @ metamodelica::List::Cons { head: (Deref @ SCode::Element::CLASS { name: cname, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: _, classDef: _, cmt: _, info: _ }, p_path), tail: rest } => {
            let mut el: Arc<Absyn::Element>;
            let mut nName: ArcStr = arcstr::literal!("");
            nName = (if (p_path.clone() == literal!("")) {cname.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*p_path.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*cname.clone()); ArcStr::from(__mm_s) }}).clone();
            el = Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: Absyn::ElementAttributes { flowPrefix: false, streamPrefix: false, parallelism: openmodelica_ast::Absyn::Parallelism::NON_PARALLEL, variability: openmodelica_ast::Absyn::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD, arrayDim: metamodelica::nil() }, typeSpec: Arc::new(Absyn::TypeSpec::TPATH { path: AbsynUtil::stringPath((nName.clone()).clone())?, arrayDim: None }), components: list![Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("_agen_")); __mm_s.push_str(&*cname.clone()); __mm_s.push_str(&*intString(autoVal.clone())); ArcStr::from(__mm_s) }).clone(), arrayDim: metamodelica::nil(), modification: None }, condition: None, comment: None })] }), info: Absyn::dummyInfo.clone(), constrainClass: None });
            populateModel(rest.clone(), autoVal.clone() + 1, cons(Arc::new(Absyn::ElementItem::ELEMENTITEM { element: el.clone() }), elements_in.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(elements_out)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAllElementsOfType(mut element_defs: Arc<metamodelica::List<Arc<SCode::Element>>>, mut typeName: Ident, mut pathInProg: ArcStr, mut elements_in: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>>) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>>> {
    let mut elements_out: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>> = metamodelica::nil();
    elements_out = (::match_deref::match_deref! { match &(element_defs.clone()) {
        Deref @ metamodelica::List::Nil => {
            elements_in.clone()
        },
        Deref @ metamodelica::List::Cons { head: el, tail: rest } => {
            let mut m: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>> = metamodelica::nil();
            m = listAppend(getAllElementsOfType2(el.clone(), (typeName.clone()).clone(), (pathInProg.clone()).clone())?, elements_in.clone());
            getAllElementsOfType(rest.clone(), (typeName.clone()).clone(), (pathInProg.clone()).clone(), m.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elements_out)
}

fn getAllElementsOfType2(mut el: Arc<SCode::Element>, mut typeName: Ident, mut pathInProg: ArcStr) -> Result<Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>>> {
    let mut res_elem: Arc<metamodelica::List<(Arc<SCode::Element>, ArcStr)>> = metamodelica::nil();
    res_elem = 'mc: {
        let __mc_input = el.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Modelica", prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: _, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    println!("{}", (literal!("**** Ignoring Standard Modelica library\n")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "OpenModelica", prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: _, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    println!("{}", (literal!("**** Ignoring Open Modelica library\n")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: Deref @ "Complex", prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: _, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    println!("{}", (literal!("**** Ignoring Complex library\n")).clone());
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    let mut nName: ArcStr = arcstr::literal!("");
                    nName = (if (pathInProg.clone() == literal!("")) {name.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*pathInProg.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }}).clone();
                    Ok(getAllElementsOfType(elist.clone(), (typeName.clone()).clone(), (nName.clone()).clone(), metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: _, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: _, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    let true = (isOfType(elist.clone(), (typeName.clone()).clone())?) else { bail!("pattern mismatch") };
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("*** Found a ")); __mm_s.push_str(&*typeName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok(list![(el.clone(), pathInProg.clone())])
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
    Ok(res_elem)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isOfType(mut elems: Arc<metamodelica::List<Arc<SCode::Element>>>, mut typeName: ArcStr) -> Result<bool> {
    let mut result: bool = false;
    result = 'mc: {
        let __mc_input = elems.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: p, visibility: _, modifications: _, ann: _, info: _ }, tail: _ } => {
                    let true = (AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)? == typeName.clone()) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(isOfType(rest.clone(), (typeName.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(result)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn inferBindingClientList(mut client_list: Arc<metamodelica::List<Arc<Client_e>>>, mut vmodel: Arc<Absyn::Class>, mut env: Absyn::Program) -> Result<Arc<Absyn::Class>> {
    let mut out_vmodel: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    out_vmodel = (::match_deref::match_deref! { match &(client_list.clone()) {
        Deref @ metamodelica::List::Nil => {
            vmodel.clone()
        },
        Deref @ metamodelica::List::Cons { head: ce, tail: rest } => {
            let mut upd_vmodel: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            upd_vmodel = inferBindingClient(ce.clone(), vmodel.clone(), env.clone())?;
            inferBindingClientList(rest.clone(), upd_vmodel.clone(), env.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_vmodel)
}

fn inferBindingClient(mut client_e: Arc<Client_e>, mut vmodel: Arc<Absyn::Class>, mut env: Absyn::Program) -> Result<Arc<Absyn::Class>> {
    let mut out_vmodel: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    out_vmodel = (::match_deref::match_deref! { match &(client_e.clone()) {
        Deref @ Client_e::CLIENT_E { components: _, typeSpec, rootType, def: _, instance: iname, predecessors: _, mediator: Deref @ metamodelica::List::Cons { head: Mediator { mType: _, template, clients: _, providers, preferred: Deref @ metamodelica::List::Nil }, tail: _ } } => {
            let mut out_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut new_exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut out_class: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... infer binding ")); __mm_s.push_str(&*Dump::unparseTypeSpec(typeSpec.clone())?); __mm_s.push_str(&*literal!("     ")); __mm_s.push_str(&*Dump::unparseTypeSpec(rootType.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            out_es = getProviders(providers.clone(), vmodel.clone(), env.clone(), metamodelica::nil())?;
            if template.clone() == literal!("") {
                out_class = updateClass(vmodel.clone(), typeSpec.clone(), rootType.clone(), out_es.clone(), iname.clone(), env.clone(), false, metamodelica::nil(), (literal!("")).clone())?;
            } else {
                let GlobalScript::ISTMTS { interactiveStmtLst: metamodelica::List::Cons { head: GlobalScript::IEXP { exp: __pa0, info: _ }, tail: __t1 }, semicolon: _ } = (Parser::parsestringexp((template.clone()).clone(), (literal!("<interactive>")).clone())?) else { bail!("pattern mismatch") };
                ::match_deref::match_deref! { match &(__t1.clone()) {
                    Deref @ metamodelica::List::Nil => (),
                    _ => bail!("pattern mismatch"),
                } };
                exp = __pa0.clone();
                new_exp = parseAggregator(exp.clone(), Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![Arc::new(Absyn::Exp::LIST { exps: toExpList(out_es.clone(), metamodelica::nil()) })], argNames: metamodelica::nil() }));
                out_class = updateClass(vmodel.clone(), typeSpec.clone(), rootType.clone(), list![(new_exp.clone(), literal!(""))], iname.clone(), env.clone(), false, metamodelica::nil(), (literal!("")).clone())?;
            }
            out_class.clone()
        },
        Deref @ Client_e::CLIENT_E { components: _, typeSpec, rootType, def: _, instance: iname, predecessors: _, mediator: Deref @ metamodelica::List::Cons { head: Mediator { mType: _, template: _, clients: _, providers, preferred }, tail: _ } } => {
            let mut out_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
            let mut out_class: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            out_es = getProviders(providers.clone(), vmodel.clone(), env.clone(), metamodelica::nil())?;
            out_class = updateClass(vmodel.clone(), typeSpec.clone(), rootType.clone(), out_es.clone(), iname.clone(), env.clone(), true, preferred.clone(), (literal!("")).clone())?;
            out_class.clone()
        },
        Deref @ Client_e::NO_PRED { .. } => {
            vmodel.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_vmodel)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn toExpList(mut e_list: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut in_es: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Arc<metamodelica::List<Arc<Absyn::Exp>>> {
    let mut out_es: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    out_es = (::match_deref::match_deref! { match &(e_list.clone()) {
        Deref @ metamodelica::List::Nil => {
            in_es.clone()
        },
        Deref @ metamodelica::List::Cons { head: (exp, _), tail: rest } => {
            toExpList(rest.clone(), cons(exp.clone(), in_es.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_es
}

pub fn updateClass(mut in_class: Arc<Absyn::Class>, mut typeSpec: TypeSpec, mut rootType: TypeSpec, mut exp: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut defs: Absyn::Program, mut hasPreferred: bool, mut preferred: Arc<metamodelica::List<Preferred>>, mut path: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut out_class: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let __pa0 = ::match_deref::match_deref! { match &(in_class.clone()) {
        __pa0 @ Deref @ Absyn::Class { .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    out_class = __pa0.clone();
    assign_field!(out_class.body = parseClassDef(out_class.body.clone(), defs.clone(), typeSpec.clone(), rootType.clone(), exp.clone(), instance_name.clone(), hasPreferred.clone(), preferred.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*path.clone()); __mm_s.push_str(&*out_class.name.clone()); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }).clone())?);
    Ok(out_class)
}

fn parseClassDef(mut in_def: Arc<Absyn::ClassDef>, mut defs: Absyn::Program, mut typeSpec: TypeSpec, mut rootType: TypeSpec, mut exp: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut hasPreferred: bool, mut preferred: Arc<metamodelica::List<Preferred>>, mut path: ArcStr) -> Result<Arc<Absyn::ClassDef>> {
    let mut out_def: Arc<Absyn::ClassDef>;
    out_def = (::match_deref::match_deref! { match &(in_def.clone()) {
        Deref @ Absyn::ClassDef::PARTS { typeVars, classAttrs, classParts, ann, comment } => {
            let mut nclsp: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            nclsp = parseClassParts(classParts.clone(), defs.clone(), typeSpec.clone(), rootType.clone(), exp.clone(), instance_name.clone(), hasPreferred.clone(), preferred.clone(), (path.clone()).clone())?;
            Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: nclsp.clone(), ann: ann.clone(), comment: comment.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_def)
}

fn parseClassParts(mut classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut defs: Absyn::Program, mut typeSpec: TypeSpec, mut rootType: TypeSpec, mut exp: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut hasPreferred: bool, mut preferred: Arc<metamodelica::List<Preferred>>, mut path: ArcStr) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut out_classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    out_classes = (::match_deref::match_deref! { match &(classes.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: cls, tail: r_classes } => {
            let mut nr_classes: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
            let mut n_cls: Arc<Absyn::ClassPart>;
            n_cls = parseClassPart(cls.clone(), defs.clone(), typeSpec.clone(), rootType.clone(), exp.clone(), instance_name.clone(), hasPreferred.clone(), preferred.clone(), (path.clone()).clone())?;
            nr_classes = parseClassParts(r_classes.clone(), defs.clone(), typeSpec.clone(), rootType.clone(), exp.clone(), instance_name.clone(), hasPreferred.clone(), preferred.clone(), (path.clone()).clone())?;
            cons(n_cls.clone(), nr_classes.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_classes)
}

fn parseClassPart(mut in_def: Arc<Absyn::ClassPart>, mut defs: Absyn::Program, mut typeSpec: TypeSpec, mut rootType: TypeSpec, mut exp: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut hasPreferred: bool, mut preferred: Arc<metamodelica::List<Preferred>>, mut path: ArcStr) -> Result<Arc<Absyn::ClassPart>> {
    let mut out_def: Arc<Absyn::ClassPart>;
    out_def = (::match_deref::match_deref! { match &(in_def.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { contents: elems } => {
            let mut elems1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            elems1 = parseElems(elems.clone(), defs.clone(), typeSpec.clone(), rootType.clone(), exp.clone(), instance_name.clone(), hasPreferred.clone(), preferred.clone(), (path.clone()).clone())?;
            Arc::new(Absyn::ClassPart::PUBLIC { contents: elems1.clone() })
        },
        _ => {
            in_def.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_def)
}

fn parseElems(mut in_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut defs: Absyn::Program, mut typeSpec: TypeSpec, mut rootType: TypeSpec, mut exp2: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut hasPreferred: bool, mut preferred: Arc<metamodelica::List<Preferred>>, mut pathInClass: ArcStr) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut out_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    out_elems = 'mc: {
        let __mc_input = in_elems.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(in_elems.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix, redeclareKeywords, innerOuter, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes, typeSpec: tSpec, components }, info, constrainClass } }, tail: rest } => {
                    let mut e_list: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut newName: bool = false;
                    let _ = AbsynUtil::typeSpecPath(typeSpec.clone())?;
                    if AbsynUtil::typeSpecPathString(rootType.clone()) == AbsynUtil::typeSpecPathString(tSpec.clone()) && !(exp2.clone().is_empty()) {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... found instance provider ")); __mm_s.push_str(&*Dump::unparseTypeSpec(tSpec.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        if hasPreferred.clone() {
                            e_list = applyModifiersPreferred(components.clone(), exp2.clone(), instance_name.clone(), (pathInClass.clone()).clone(), finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), info.clone(), constrainClass.clone(), attributes.clone(), tSpec.clone(), preferred.clone())?;
                        } else {
                            newName = (exp2.clone().len() as i32) != 1;
                            e_list = applyModifiers(components.clone(), exp2.clone(), instance_name.clone(), 0, finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), info.clone(), constrainClass.clone(), attributes.clone(), tSpec.clone(), newName.clone())?;
                        }
                    } else {
                        e_list = list![Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclareKeywords.clone(), innerOuter: innerOuter.clone(), specification: Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: attributes.clone(), typeSpec: tSpec.clone(), components: components.clone() }), info: info.clone(), constrainClass: constrainClass.clone() }) })];
                    }
                    Ok(listAppend(e_list.clone(), parseElems(rest.clone(), defs.clone(), typeSpec.clone(), rootType.clone(), exp2.clone(), instance_name.clone(), hasPreferred.clone(), preferred.clone(), (pathInClass.clone()).clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: e_item, tail: rest } => {
                    Ok(cons(e_item.clone(), parseElems(rest.clone(), defs.clone(), typeSpec.clone(), rootType.clone(), exp2.clone(), instance_name.clone(), hasPreferred.clone(), preferred.clone(), (pathInClass.clone()).clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_elems)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn applyModifiersPreferred(mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut exp: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut typeSp: ArcStr, mut finalPrefix: bool, mut redeclareKeywords: Option<Absyn::RedeclareKeywords>, mut innerOuter: Absyn::InnerOuter, mut info: SourceInfo, mut constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut attributes: Absyn::ElementAttributes, mut tSpec: TypeSpec, mut preferred: Arc<metamodelica::List<Preferred>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut out_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    out_elems = 'mc: {
        let __mc_input = exp.clone();
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
                Deref @ metamodelica::List::Cons { head: (e, ename), tail: rest } => {
                    let mut cnew: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut client_pref: ArcStr = arcstr::literal!("");
                    let mut enew: Arc<Absyn::ElementItem>;
                    client_pref = (getPreferredBinding((ename.clone()).clone(), preferred.clone())?).clone();
                    cnew = applyModifierPreferred(comps.clone(), e.clone(), (client_pref.clone()).clone(), instance_name.clone(), (ename.clone()).clone())?;
                    enew = Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclareKeywords.clone(), innerOuter: innerOuter.clone(), specification: Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: attributes.clone(), typeSpec: tSpec.clone(), components: cnew.clone() }), info: info.clone(), constrainClass: constrainClass.clone() }) });
                    Ok(cons(enew.clone(), applyModifiersPreferred(comps.clone(), rest.clone(), instance_name.clone(), (typeSp.clone()).clone(), finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), info.clone(), constrainClass.clone(), attributes.clone(), tSpec.clone(), preferred.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(applyModifiersPreferred(comps.clone(), rest.clone(), instance_name.clone(), (typeSp.clone()).clone(), finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), info.clone(), constrainClass.clone(), attributes.clone(), tSpec.clone(), preferred.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_elems)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPreferredBinding(mut ename: ArcStr, mut elems: Arc<metamodelica::List<Preferred>>) -> Result<ArcStr> {
    let mut cl_name: ArcStr = arcstr::literal!("");
    cl_name = ('mc: {
        let __mc_input = elems.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Preferred { clientInstancePath: c_id, providerInstancePath: p_id }, tail: _ } => {
                    let true = (p_id.clone() == ename.clone()) else { bail!("pattern mismatch") };
                    Ok(c_id.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getPreferredBinding((ename.clone()).clone(), rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(cl_name)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn applyModifierPreferred(mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut exp: Arc<Absyn::Exp>, mut typeSp: ArcStr, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut ename: ArcStr) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>> {
    let mut out_comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    out_comps = 'mc: {
        let __mc_input = comps.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim, modification: _ }, condition, comment }, tail: _ } => {
                    let mut cnew: Arc<Absyn::ComponentItem>;
                    let true = (typeSp.clone() == name.clone()) else { bail!("pattern mismatch") };
                    cnew = Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: (name.clone()).clone(), arrayDim: arrayDim.clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: buildComponentModifiers(instance_name.clone(), exp.clone())?, eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })) }, condition: condition.clone(), comment: comment.clone() });
                    Ok(list![cnew.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(applyModifierPreferred(rest.clone(), exp.clone(), (typeSp.clone()).clone(), instance_name.clone(), (ename.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_comps)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn applyModifiers(mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut exp: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut counter: i32, mut finalPrefix: bool, mut redeclareKeywords: Option<Absyn::RedeclareKeywords>, mut innerOuter: Absyn::InnerOuter, mut info: SourceInfo, mut constrainClass: Option<Arc<Absyn::ConstrainClass>>, mut attributes: Absyn::ElementAttributes, mut tSpec: TypeSpec, mut newName: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut out_elems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    out_elems = 'mc: {
        let __mc_input = exp.clone();
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
                Deref @ metamodelica::List::Cons { head: (e, _), tail: rest } => {
                    let mut cnew: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
                    let mut enew: Arc<Absyn::ElementItem>;
                    cnew = applyModifier(comps.clone(), e.clone(), instance_name.clone(), counter.clone(), newName.clone())?;
                    enew = Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: finalPrefix.clone(), redeclareKeywords: redeclareKeywords.clone(), innerOuter: innerOuter.clone(), specification: Arc::new(Absyn::ElementSpec::COMPONENTS { attributes: attributes.clone(), typeSpec: tSpec.clone(), components: cnew.clone() }), info: info.clone(), constrainClass: constrainClass.clone() }) });
                    Ok(cons(enew.clone(), applyModifiers(comps.clone(), rest.clone(), instance_name.clone(), counter.clone() + 1, finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), info.clone(), constrainClass.clone(), attributes.clone(), tSpec.clone(), newName.clone())?))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(applyModifiers(comps.clone(), rest.clone(), instance_name.clone(), counter.clone(), finalPrefix.clone(), redeclareKeywords.clone(), innerOuter.clone(), info.clone(), constrainClass.clone(), attributes.clone(), tSpec.clone(), newName.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_elems)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn applyModifier(mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut exp: Arc<Absyn::Exp>, mut instance_name: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut counter: i32, mut newName: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>> {
    let mut out_comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> = metamodelica::nil();
    out_comps = 'mc: {
        let __mc_input = comps.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim, modification: _ }, condition, comment }, tail: _ } => {
                    let mut cnew: Arc<Absyn::ComponentItem>;
                    let mut new_name: ArcStr = arcstr::literal!("");
                    new_name = (if (newName.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!("_autogen_bind_")); __mm_s.push_str(&*intString(counter.clone())); ArcStr::from(__mm_s) }} else {name.clone()}).clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("**** Applying modifier ")); __mm_s.push_str(&*new_name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    cnew = Arc::new(Absyn::ComponentItem { component: Absyn::Component { name: (new_name.clone()).clone(), arrayDim: arrayDim.clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: buildComponentModifiers(instance_name.clone(), exp.clone())?, eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })) }, condition: condition.clone(), comment: comment.clone() });
                    Ok(list![cnew.clone()])
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(applyModifier(rest.clone(), exp.clone(), instance_name.clone(), counter.clone(), newName.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_comps)
}

fn buildComponentModifiers(mut name_list: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut exp: Arc<Absyn::Exp>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut out_modifiers: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut parsed_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    parsed_names = buildAllComponentModifiers(name_list.clone());
    out_modifiers = buildComponentModifiers2(parsed_names.clone(), exp.clone())?;
    Ok(out_modifiers)
}

fn buildComponentModifiers2(mut name_list: Arc<metamodelica::List<ArcStr>>, mut exp: Arc<Absyn::Exp>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut out_modifiers: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    out_modifiers = (::match_deref::match_deref! { match &(name_list.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: name, tail: rest } => {
            cons(Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: AbsynUtil::stringPath((name.clone()).clone())?, modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: exp.clone(), info: Absyn::dummyInfo.clone() }) })), comment: None, info: Absyn::dummyInfo.clone() }), buildComponentModifiers2(rest.clone(), exp.clone())?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_modifiers)
}

fn buildAllComponentModifiers(mut name_list: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut out_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    out_names = (::match_deref::match_deref! { match &(name_list.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: l, tail: rest } => {
            let mut tmp_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            tmp_names = buildAllComponentModifiers(rest.clone());
            buildAllComponentModifiers2(l.clone(), tmp_names.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_names
}

fn buildAllComponentModifiers2(mut name_list: Arc<metamodelica::List<ArcStr>>, mut name_list2: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut out_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    out_names = (::match_deref::match_deref! { match &(name_list.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: s, tail: rest } => {
            let mut tmp_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            tmp_names = buildAllComponentModifiers2(rest.clone(), name_list2.clone());
            listAppend(buildAllComponentModifiers3((s.clone()).clone(), name_list2.clone()), tmp_names.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_names
}

fn buildAllComponentModifiers3(mut prefix: ArcStr, mut name_list2: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
    let mut out_names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    out_names = (::match_deref::match_deref! { match &(name_list2.clone()) {
        Deref @ metamodelica::List::Nil => {
            list![(prefix.clone()).clone()]
        },
        Deref @ metamodelica::List::Cons { head: s, tail: rest } => {
            cons(({ let mut __mm_s = String::new(); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*prefix.clone()); ArcStr::from(__mm_s) }).clone(), buildAllComponentModifiers2(rest.clone(), name_list2.clone()))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_names
}

fn parseAggregator(mut in_eq: Arc<Absyn::Exp>, mut fargs: Arc<Absyn::FunctionArgs>) -> Arc<Absyn::Exp> {
    let mut out_eq: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    out_eq = (::match_deref::match_deref! { match &(in_eq.clone()) {
        Deref @ Absyn::Exp::BINARY { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseAggregator(exp1.clone(), fargs.clone());
            nexp2 = parseAggregator(exp2.clone(), fargs.clone());
            Arc::new(Absyn::Exp::BINARY { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() })
        },
        Deref @ Absyn::Exp::LBINARY { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseAggregator(exp1.clone(), fargs.clone());
            nexp2 = parseAggregator(exp2.clone(), fargs.clone());
            Arc::new(Absyn::Exp::LBINARY { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() })
        },
        Deref @ Absyn::Exp::RELATION { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseAggregator(exp1.clone(), fargs.clone());
            nexp2 = parseAggregator(exp2.clone(), fargs.clone());
            Arc::new(Absyn::Exp::RELATION { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() })
        },
        Deref @ Absyn::Exp::UNARY { op, exp: exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseAggregator(exp2.clone(), fargs.clone());
            Arc::new(Absyn::Exp::UNARY { op: op.clone(), exp: nexp1.clone() })
        },
        Deref @ Absyn::Exp::LUNARY { op, exp: exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseAggregator(exp2.clone(), fargs.clone());
            Arc::new(Absyn::Exp::LUNARY { op: op.clone(), exp: nexp1.clone() })
        },
        Deref @ Absyn::Exp::IFEXP { ifExp: ife, trueBranch: exp1, elseBranch: exp2, elseIfBranch: elif } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nife: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nife = parseAggregator(ife.clone(), fargs.clone());
            nexp1 = parseAggregator(exp1.clone(), fargs.clone());
            nexp2 = parseAggregator(exp2.clone(), fargs.clone());
            Arc::new(Absyn::Exp::IFEXP { ifExp: nife.clone(), trueBranch: nexp1.clone(), elseBranch: nexp2.clone(), elseIfBranch: elif.clone() })
        },
        Deref @ Absyn::Exp::CALL { function_: crf, functionArgs: _, .. } => {
            Arc::new(Absyn::Exp::CALL { function_: crf.clone(), functionArgs: fargs.clone(), typeVars: var_field!((*in_eq).typeVars, Absyn::Exp::CALL).clone() })
        },
        _ => {
            in_eq.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    out_eq
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getProviders(mut providers: Arc<metamodelica::List<Provider>>, mut vmodel: Arc<Absyn::Class>, mut env: Absyn::Program, mut in_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>> {
    let mut out_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
    out_es = (::match_deref::match_deref! { match &(providers.clone()) {
        Deref @ metamodelica::List::Nil => {
            in_es.clone()
        },
        Deref @ metamodelica::List::Cons { head: Provider { modelID: className, component: instance, template }, tail: rest } => {
            let mut comps: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>> = metamodelica::nil();
            let mut exps: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
            let mut new_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
            let mut mlist: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            let mut exp: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            mlist = AbsynUtil::getElementItemsInClass(vmodel.clone());
            comps = getAllProviderInstances((className.clone()).clone(), (template.clone()).clone(), mlist.clone(), env.clone(), metamodelica::nil(), (literal!("")).clone())?;
            let GlobalScript::ISTMTS { interactiveStmtLst: metamodelica::List::Cons { head: GlobalScript::IEXP { exp: __pa0, info: _ }, tail: __t1 }, semicolon: _ } = (Parser::parsestringexp((template.clone()).clone(), (literal!("<interactive>")).clone())?) else { bail!("pattern mismatch") };
            ::match_deref::match_deref! { match &(__t1.clone()) {
                Deref @ metamodelica::List::Nil => (),
                _ => bail!("pattern mismatch"),
            } };
            exp = __pa0.clone();
            exps = applyTemplate(exp.clone(), comps.clone(), metamodelica::nil())?;
            new_es = listAppend(exps.clone(), in_es.clone());
            getProviders(rest.clone(), vmodel.clone(), env.clone(), new_es.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(out_es)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn applyTemplate(mut exp: Arc<Absyn::Exp>, mut comps: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>>, mut in_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>> {
    let mut out_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
    out_es = 'mc: {
        let __mc_input = comps.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(in_es.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: (clist, pathInClass), tail: rest } => {
                    let mut new_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
                    new_es = applyTemplate2(exp.clone(), clist.clone(), in_es.clone(), (pathInClass.clone()).clone())?;
                    Ok(applyTemplate(exp.clone(), rest.clone(), new_es.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(applyTemplate(exp.clone(), rest.clone(), in_es.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_es)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn applyTemplate2(mut exp: Arc<Absyn::Exp>, mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut in_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>, mut pathInClass: ArcStr) -> Result<Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>>> {
    let mut out_es: Arc<metamodelica::List<(Arc<Absyn::Exp>, ArcStr)>> = metamodelica::nil();
    out_es = 'mc: {
        let __mc_input = comps.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(in_es.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: rest } => {
                    let mut newName: ArcStr = arcstr::literal!("");
                    newName = (if (pathInClass.clone() == literal!("")) {name.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*pathInClass.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }}).clone();
                    Ok(applyTemplate2(exp.clone(), rest.clone(), cons((parseExpression(exp.clone(), (newName.clone()).clone())?, newName.clone()), in_es.clone()), (pathInClass.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(applyTemplate2(exp.clone(), rest.clone(), in_es.clone(), (pathInClass.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_es)
}

fn parseExpression(mut in_eq: Arc<Absyn::Exp>, mut fargs: ArcStr) -> Result<Arc<Absyn::Exp>> {
    let mut out_eq: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    out_eq = (::match_deref::match_deref! { match &(in_eq.clone()) {
        Deref @ Absyn::Exp::BINARY { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseExpression(exp1.clone(), (fargs.clone()).clone())?;
            nexp2 = parseExpression(exp2.clone(), (fargs.clone()).clone())?;
            Arc::new(Absyn::Exp::BINARY { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() })
        },
        Deref @ Absyn::Exp::LBINARY { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseExpression(exp1.clone(), (fargs.clone()).clone())?;
            nexp2 = parseExpression(exp2.clone(), (fargs.clone()).clone())?;
            Arc::new(Absyn::Exp::LBINARY { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() })
        },
        Deref @ Absyn::Exp::RELATION { exp1, op, exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseExpression(exp1.clone(), (fargs.clone()).clone())?;
            nexp2 = parseExpression(exp2.clone(), (fargs.clone()).clone())?;
            Arc::new(Absyn::Exp::RELATION { exp1: nexp1.clone(), op: op.clone(), exp2: nexp2.clone() })
        },
        Deref @ Absyn::Exp::UNARY { op, exp: exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseExpression(exp2.clone(), (fargs.clone()).clone())?;
            Arc::new(Absyn::Exp::UNARY { op: op.clone(), exp: nexp1.clone() })
        },
        Deref @ Absyn::Exp::LUNARY { op, exp: exp2 } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nexp1 = parseExpression(exp2.clone(), (fargs.clone()).clone())?;
            Arc::new(Absyn::Exp::LUNARY { op: op.clone(), exp: nexp1.clone() })
        },
        Deref @ Absyn::Exp::IFEXP { ifExp: ife, trueBranch: exp1, elseBranch: exp2, elseIfBranch: elif } => {
            let mut nexp1: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nexp2: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            let mut nife: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
            nife = parseExpression(ife.clone(), (fargs.clone()).clone())?;
            nexp1 = parseExpression(exp1.clone(), (fargs.clone()).clone())?;
            nexp2 = parseExpression(exp2.clone(), (fargs.clone()).clone())?;
            Arc::new(Absyn::Exp::IFEXP { ifExp: nife.clone(), trueBranch: nexp1.clone(), elseBranch: nexp2.clone(), elseIfBranch: elif.clone() })
        },
        Deref @ Absyn::Exp::CREF { componentRef: crf } => {
            let mut new_crf: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
            new_crf = updateCRF(crf.clone(), (fargs.clone()).clone())?;
            Arc::new(Absyn::Exp::CREF { componentRef: new_crf.clone() })
        },
        _ => {
            in_eq.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(out_eq)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn updateCRF(mut componentRef: Arc<Absyn::ComponentRef>, mut name: ArcStr) -> Result<Arc<Absyn::ComponentRef>> {
    let mut out_componentRef: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
    out_componentRef = 'mc: {
        let __mc_input = componentRef.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cRef } => {
                    Ok(updateCRF(cRef.clone(), (name.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "getPath", subscripts, componentRef: cRef } => {
                    Ok(Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (name.clone()).clone(), subscripts: subscripts.clone(), componentRef: cRef.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: id, subscripts, componentRef: cRef } => {
                    let mut new_cRef: Arc<Absyn::ComponentRef> = Arc::new(Absyn::ComponentRef::ALLWILD);
                    new_cRef = updateCRF(cRef.clone(), (name.clone()).clone())?;
                    Ok(Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id.clone()).clone(), subscripts: subscripts.clone(), componentRef: new_cRef.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "getPath", subscripts } => {
                    Ok(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subscripts.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(componentRef.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_componentRef)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getAllProviderInstances(mut className: ArcStr, mut template: ArcStr, mut e_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut env: Absyn::Program, mut in_components: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>>, mut pathInClass: ArcStr) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>>> {
    let mut out_components: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>> = metamodelica::nil();
    out_components = 'mc: {
        let __mc_input = e_items.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(in_components.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec, components }, info: _, constrainClass: _ } }, tail: rest } => {
                    let mut re_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut cnew: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>> = metamodelica::nil();
                    let mut cnew2: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>> = metamodelica::nil();
                    let mut path: Path;
                    let mut def: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    path = AbsynUtil::typeSpecPath(typeSpec.clone())?;
                    def = InteractiveUtil::getPathedClassInProgram(path.clone(), env.clone(), false, false)?;
                    if AbsynUtil::typeSpecPathString(typeSpec.clone()) == className.clone() {
                        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... found provider ")); __mm_s.push_str(&*className.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                        cnew = cons((components.clone(), pathInClass.clone()), in_components.clone());
                    } else {
                        cnew = in_components.clone();
                    }
                    re_items = AbsynUtil::getElementItemsInClass(def.clone());
                    cnew2 = parseComponents((className.clone()).clone(), (template.clone()).clone(), re_items.clone(), env.clone(), components.clone(), cnew.clone(), (pathInClass.clone()).clone())?;
                    Ok(getAllProviderInstances((className.clone()).clone(), (template.clone()).clone(), rest.clone(), env.clone(), cnew2.clone(), (pathInClass.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getAllProviderInstances((className.clone()).clone(), (template.clone()).clone(), rest.clone(), env.clone(), in_components.clone(), (pathInClass.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_components)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn parseComponents(mut className: ArcStr, mut template: ArcStr, mut e_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut env: Absyn::Program, mut components: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut in_components: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>>, mut pathInClass: ArcStr) -> Result<Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>>> {
    let mut out_components: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>> = metamodelica::nil();
    out_components = 'mc: {
        let __mc_input = components.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(in_components.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ComponentItem { component: Absyn::Component { name, arrayDim: _, modification: _ }, condition: _, comment: _ }, tail: rest } => {
                    let mut tmp: Arc<metamodelica::List<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArcStr)>> = metamodelica::nil();
                    let mut newName: ArcStr = arcstr::literal!("");
                    newName = (if (pathInClass.clone() == literal!("")) {name.clone()} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*pathInClass.clone()); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }}).clone();
                    tmp = getAllProviderInstances((className.clone()).clone(), (template.clone()).clone(), e_items.clone(), env.clone(), in_components.clone(), (newName.clone()).clone())?;
                    Ok(parseComponents((className.clone()).clone(), (template.clone()).clone(), e_items.clone(), env.clone(), rest.clone(), tmp.clone(), (pathInClass.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(parseComponents((className.clone()).clone(), (template.clone()).clone(), e_items.clone(), env.clone(), rest.clone(), in_components.clone(), (pathInClass.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(out_components)
}

fn buildInstList(mut clazz: Arc<Absyn::Class>, mut env: Absyn::Program, mut predecessors: Arc<Client_e>, mut mediators: Arc<metamodelica::List<Mediator>>, mut client_list_in: Arc<metamodelica::List<Arc<Client_e>>>, mut instance_list: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Result<Arc<metamodelica::List<Arc<Client_e>>>> {
    let mut client_list: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
    let mut e_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    e_items = AbsynUtil::getElementItemsInClass(clazz.clone());
    client_list = parseElementInstList(e_items.clone(), env.clone(), Arc::new(crate::Binding::Client_e::NO_PRED), mediators.clone(), client_list_in.clone(), instance_list.clone())?;
    Ok(client_list)
}

fn buildInstList2(mut clazz: Arc<Absyn::Class>, mut env: Absyn::Program, mut predecessors: Arc<Client_e>, mut mediators: Arc<metamodelica::List<Mediator>>, mut client_list_in: Arc<metamodelica::List<Arc<Client_e>>>, mut instance_list: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut rootType: TypeSpec) -> Result<Arc<metamodelica::List<Arc<Client_e>>>> {
    let mut client_list: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
    let mut e_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    e_items = AbsynUtil::getElementItemsInClass(clazz.clone());
    client_list = parseElementInstList2(e_items.clone(), env.clone(), Arc::new(crate::Binding::Client_e::NO_PRED), mediators.clone(), client_list_in.clone(), instance_list.clone(), rootType.clone())?;
    Ok(client_list)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn isAlreadyInList(mut ts: Arc<Absyn::TypeSpec>, mut predecessors: Arc<metamodelica::List<Arc<Client_e>>>) -> Result<bool> {
    let mut val: bool = false;
    val = (::match_deref::match_deref! { match &(predecessors.clone()) {
        Deref @ metamodelica::List::Nil => {
            false
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Client_e::CLIENT_E { components: _, typeSpec: _, rootType: ots, def: _, instance: _, predecessors: _, mediator: _ }, tail: rest } => {
            if (AbsynUtil::typeSpecEqual(ts.clone(), ots.clone())) {true} else {isAlreadyInList(ts.clone(), rest.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(val)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn parseElementInstList(mut e_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut env: Absyn::Program, mut predecessors: Arc<Client_e>, mut mediators: Arc<metamodelica::List<Mediator>>, mut in_client_list: Arc<metamodelica::List<Arc<Client_e>>>, mut instance_list: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>) -> Result<Arc<metamodelica::List<Arc<Client_e>>>> {
    let mut client_list: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
    client_list = 'mc: {
        let __mc_input = e_items.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(in_client_list.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec, components }, info: _, constrainClass: _ } }, tail: rest } => {
                    let mut path: Path;
                    let mut iname: ArcStr = arcstr::literal!("");
                    let mut new_predecessors: Arc<Client_e> = Arc::new(Client_e::NO_PRED);
                    let mut def: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut l1: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
                    let mut isCl: bool = false;
                    let mut m: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
                    path = AbsynUtil::typeSpecPath(typeSpec.clone())?;
                    def = InteractiveUtil::getPathedClassInProgram(path.clone(), env.clone(), false, false)?;
                    (isCl, iname, m) = isClient((AbsynUtil::typeSpecPathString(typeSpec.clone())).clone(), mediators.clone(), metamodelica::nil())?;
                    if isCl.clone() && !(isAlreadyInList(typeSpec.clone(), in_client_list.clone())?) {
                        new_predecessors = Arc::new(Client_e::CLIENT_E { components: components.clone(), typeSpec: typeSpec.clone(), rootType: typeSpec.clone(), def: def.clone(), instance: cons(list![(iname.clone()).clone()], instance_list.clone()), predecessors: predecessors.clone(), mediator: m.clone() });
                        l2 = cons(new_predecessors.clone(), in_client_list.clone());
                    } else {
                        new_predecessors = predecessors.clone();
                        l2 = in_client_list.clone();
                    }
                    l1 = buildInstList2(def.clone(), env.clone(), new_predecessors.clone(), mediators.clone(), l2.clone(), instance_list.clone(), typeSpec.clone())?;
                    Ok(parseElementInstList(rest.clone(), env.clone(), predecessors.clone(), mediators.clone(), l1.clone(), instance_list.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(parseElementInstList(rest.clone(), env.clone(), predecessors.clone(), mediators.clone(), in_client_list.clone(), instance_list.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(client_list)
}

fn getComponentNames(mut l: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut res: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    res = (::match_deref::match_deref! { match &(l.clone()) {
        Deref @ metamodelica::List::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::List::Cons { head: ci, tail: r } => {
            cons((AbsynUtil::componentName(ci.clone())?).clone(), getComponentNames(r.clone())?)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn parseElementInstList2(mut e_items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut env: Absyn::Program, mut predecessors: Arc<Client_e>, mut mediators: Arc<metamodelica::List<Mediator>>, mut in_client_list: Arc<metamodelica::List<Arc<Client_e>>>, mut instance_list: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>>, mut rootType: TypeSpec) -> Result<Arc<metamodelica::List<Arc<Client_e>>>> {
    let mut client_list: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
    client_list = 'mc: {
        let __mc_input = e_items.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok(in_client_list.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { finalPrefix: _, redeclareKeywords: _, innerOuter: _, specification: Deref @ Absyn::ElementSpec::COMPONENTS { attributes: _, typeSpec, components }, info: _, constrainClass: _ } }, tail: rest } => {
                    let mut path: Path;
                    let mut iname: ArcStr = arcstr::literal!("");
                    let mut new_predecessors: Arc<Client_e> = Arc::new(Client_e::NO_PRED);
                    let mut def: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut l1: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
                    let mut l2: Arc<metamodelica::List<Arc<Client_e>>> = metamodelica::nil();
                    let mut isCl: bool = false;
                    let mut m: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
                    path = AbsynUtil::typeSpecPath(typeSpec.clone())?;
                    def = InteractiveUtil::getPathedClassInProgram(path.clone(), env.clone(), false, false)?;
                    (isCl, iname, m) = isClient((AbsynUtil::typeSpecPathString(typeSpec.clone())).clone(), mediators.clone(), metamodelica::nil())?;
                    if isCl.clone() {
                        new_predecessors = Arc::new(Client_e::CLIENT_E { components: components.clone(), typeSpec: typeSpec.clone(), rootType: rootType.clone(), def: def.clone(), instance: cons(list![(iname.clone()).clone()], cons(getComponentNames(components.clone())?, instance_list.clone())), predecessors: predecessors.clone(), mediator: m.clone() });
                        l2 = cons(new_predecessors.clone(), in_client_list.clone());
                    } else {
                        new_predecessors = predecessors.clone();
                        l2 = in_client_list.clone();
                    }
                    l1 = buildInstList2(def.clone(), env.clone(), new_predecessors.clone(), mediators.clone(), l2.clone(), cons(getComponentNames(components.clone())?, instance_list.clone()), rootType.clone())?;
                    Ok(parseElementInstList2(rest.clone(), env.clone(), predecessors.clone(), mediators.clone(), l1.clone(), instance_list.clone(), rootType.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(parseElementInstList2(rest.clone(), env.clone(), predecessors.clone(), mediators.clone(), in_client_list.clone(), instance_list.clone(), rootType.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(client_list)
}

fn isClient(mut ci_name: ArcStr, mut mediators: Arc<metamodelica::List<Mediator>>, mut in_m: Arc<metamodelica::List<Mediator>>) -> Result<(bool, ArcStr, Arc<metamodelica::List<Mediator>>)> {
    let mut isClient: bool = false;
    let mut iname: ArcStr = arcstr::literal!("");
    let mut m: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
    (isClient, iname, m) = 'mc: {
        let __mc_input = mediators.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((false, literal!(""), in_m.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Mediator { mType, template, clients, providers, preferred }, tail: _ } => {
                    let mut nm: ArcStr = arcstr::literal!("");
                    let (true, __pa0) = (isClientInMediator((ci_name.clone()).clone(), clients.clone())?) else { bail!("pattern mismatch") };
                    nm = __pa0.clone();
                    println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("... found client : ")); __mm_s.push_str(&*ci_name.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
                    Ok((true, nm.clone(), cons(Mediator { mType: (mType.clone()).clone(), template: (template.clone()).clone(), clients: clients.clone(), providers: providers.clone(), preferred: preferred.clone() }, in_m.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(self::isClient((ci_name.clone()).clone(), rest.clone(), in_m.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((isClient, iname, m))
}

fn isClientInMediator(mut ci_name: ArcStr, mut clients: Arc<metamodelica::List<Client>>) -> Result<(bool, ArcStr)> {
    let mut isClient: bool = false;
    let mut iname: ArcStr = arcstr::literal!("");
    (isClient, iname) = 'mc: {
        let __mc_input = clients.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((false, literal!("")))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Client { modelID: name, component: inst, template: _, isMandatory: _ }, tail: _ } => {
                    let true = (name.clone() == ci_name.clone()) else { bail!("pattern mismatch") };
                    Ok((true, inst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(isClientInMediator((ci_name.clone()).clone(), rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((isClient, iname))
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getMediatorDefsElements(mut mediator_defs: Arc<metamodelica::List<Arc<SCode::Element>>>, mut mediators_in: Arc<metamodelica::List<Mediator>>) -> Result<Arc<metamodelica::List<Mediator>>> {
    let mut mediators_out: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
    mediators_out = (::match_deref::match_deref! { match &(mediator_defs.clone()) {
        Deref @ metamodelica::List::Nil => {
            mediators_in.clone()
        },
        Deref @ metamodelica::List::Cons { head: el, tail: rest } => {
            let mut m: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
            m = listAppend(getMediatorDefsElement(el.clone())?, mediators_in.clone());
            getMediatorDefsElements(rest.clone(), m.clone())?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(mediators_out)
}

fn getMediatorDefsElement(mut el: Arc<SCode::Element>) -> Result<Arc<metamodelica::List<Mediator>>> {
    let mut mediator: Arc<metamodelica::List<Mediator>> = metamodelica::nil();
    mediator = 'mc: {
        let __mc_input = el.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: _, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_PACKAGE, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    Ok(getMediatorDefsElements(elist.clone(), metamodelica::nil())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Element::CLASS { name: _, prefixes: _, encapsulatedPrefix: _, partialPrefix: _, restriction: SCode::Restriction::R_RECORD { isOperator: _ }, classDef: Deref @ SCode::ClassDef::PARTS { elementLst: elist, normalEquationLst: _, initialEquationLst: _, normalAlgorithmLst: _, initialAlgorithmLst: _, constraintLst: _, clsattrs: _, externalDecl: _ }, cmt: _, info: _ } => {
                    let mut r#mod: Arc<SCode::Mod> = Arc::new(SCode::Mod::NOMOD);
                    let mut cMod: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut pMod: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut prMod: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
                    let mut template: ArcStr = arcstr::literal!("");
                    let mut mType: ArcStr = arcstr::literal!("");
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    let mut cls: Arc<metamodelica::List<Client>> = metamodelica::nil();
                    let mut prvs: Arc<metamodelica::List<Provider>> = metamodelica::nil();
                    let mut pref: Arc<metamodelica::List<Preferred>> = metamodelica::nil();
                    let __pa0 = ::match_deref::match_deref! { match &(extendsType(elist.clone(), (literal!("Mediator")).clone())?) {
                        (true, Some(__pa0)) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#mod = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(getValue(r#mod.clone(), (literal!("template")).clone(), (literal!("string")).clone())?) {
                        Deref @ Absyn::Exp::STRING { value: __pa1 } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    template = __pa1.clone();
                    str1 = (System::stringReplace((template.clone()).clone(), (literal!("%")).clone(), (literal!("")).clone())?).clone();
                    str2 = (System::stringReplace((str1.clone()).clone(), (literal!(":")).clone(), (literal!("all")).clone())?).clone();
                    let __pa2 = ::match_deref::match_deref! { match &(getValue(r#mod.clone(), (literal!("mType")).clone(), (literal!("string")).clone())?) {
                        Deref @ Absyn::Exp::STRING { value: __pa2 } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    mType = __pa2.clone();
                    let __pa3 = ::match_deref::match_deref! { match &(getValue(r#mod.clone(), (literal!("clients")).clone(), (literal!("array")).clone())?) {
                        Deref @ Absyn::Exp::ARRAY { arrayExp: __pa3 } => __pa3.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    cMod = __pa3.clone();
                    cls = getClientList(cMod.clone(), metamodelica::nil())?;
                    let __pa4 = ::match_deref::match_deref! { match &(getValue(r#mod.clone(), (literal!("providers")).clone(), (literal!("array")).clone())?) {
                        Deref @ Absyn::Exp::ARRAY { arrayExp: __pa4 } => __pa4.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    pMod = __pa4.clone();
                    prvs = getProviderList(pMod.clone(), metamodelica::nil())?;
                    let __pa5 = ::match_deref::match_deref! { match &(getValue(r#mod.clone(), (literal!("preferred")).clone(), (literal!("array")).clone())?) {
                        Deref @ Absyn::Exp::ARRAY { arrayExp: __pa5 } => __pa5.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    prMod = __pa5.clone();
                    pref = getPreferredList(prMod.clone(), metamodelica::nil())?;
                    Ok(list![Mediator { mType: (mType.clone()).clone(), template: (str2.clone()).clone(), clients: cls.clone(), providers: prvs.clone(), preferred: pref.clone() }])
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
    Ok(mediator)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getPreferredList(mut e: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut val: Arc<metamodelica::List<Preferred>>) -> Result<Arc<metamodelica::List<Preferred>>> {
    let mut n_val: Arc<metamodelica::List<Preferred>> = metamodelica::nil();
    n_val = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ metamodelica::List::Nil => {
            val.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CALL { function_: _, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames }, .. }, tail: rest } => {
            let mut clientInstancePath: ArcStr = arcstr::literal!("");
            let mut providerInstancePath: ArcStr = arcstr::literal!("");
            clientInstancePath = (getArg(argNames.clone(), (literal!("clientInstancePath")).clone())?).clone();
            providerInstancePath = (getArg(argNames.clone(), (literal!("providerInstancePath")).clone())?).clone();
            getPreferredList(rest.clone(), cons(Preferred { clientInstancePath: (clientInstancePath.clone()).clone(), providerInstancePath: (providerInstancePath.clone()).clone() }, val.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(n_val)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getClientList(mut e: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut val: Arc<metamodelica::List<Client>>) -> Result<Arc<metamodelica::List<Client>>> {
    let mut n_val: Arc<metamodelica::List<Client>> = metamodelica::nil();
    n_val = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ metamodelica::List::Nil => {
            val.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CALL { function_: _, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames }, .. }, tail: rest } => {
            let mut className: ArcStr = arcstr::literal!("");
            let mut instance: ArcStr = arcstr::literal!("");
            let mut template: ArcStr = arcstr::literal!("");
            let mut isM: ArcStr = arcstr::literal!("");
            let mut isMandatory: bool = false;
            className = (getArg(argNames.clone(), (literal!("modelID")).clone())?).clone();
            instance = (getArg(argNames.clone(), (literal!("component")).clone())?).clone();
            template = (getArg(argNames.clone(), (literal!("template")).clone())?).clone();
            isM = (getArg(argNames.clone(), (literal!("isMandatory")).clone())?).clone();
            if isM.clone() == literal!("true") {
                isMandatory = true;
            } else {
                isMandatory = false;
            }
            getClientList(rest.clone(), cons(Client { modelID: (className.clone()).clone(), component: (instance.clone()).clone(), template: (template.clone()).clone(), isMandatory: isMandatory.clone() }, val.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(n_val)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getProviderList(mut e: Arc<metamodelica::List<Arc<Absyn::Exp>>>, mut val: Arc<metamodelica::List<Provider>>) -> Result<Arc<metamodelica::List<Provider>>> {
    let mut n_val: Arc<metamodelica::List<Provider>> = metamodelica::nil();
    n_val = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ metamodelica::List::Nil => {
            val.clone()
        },
        Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CALL { function_: _, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames }, .. }, tail: rest } => {
            let mut className: ArcStr = arcstr::literal!("");
            let mut providerTemplate: ArcStr = arcstr::literal!("");
            let mut instance: ArcStr = arcstr::literal!("");
            className = (getArg(argNames.clone(), (literal!("modelID")).clone())?).clone();
            instance = (getArg(argNames.clone(), (literal!("component")).clone())?).clone();
            providerTemplate = (getArg(argNames.clone(), (literal!("template")).clone())?).clone();
            getProviderList(rest.clone(), cons(Provider { modelID: (className.clone()).clone(), component: (instance.clone()).clone(), template: (providerTemplate.clone()).clone() }, val.clone()))?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(n_val)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getArg(mut argNames: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>, mut name: ArcStr) -> Result<ArcStr> {
    let mut val: ArcStr = arcstr::literal!("");
    val = ('mc: {
        let __mc_input = argNames.clone();
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
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NamedArg { argName: nname, argValue: Deref @ Absyn::Exp::STRING { value: r#str } }, tail: _ } => {
                    let mut str1: ArcStr = arcstr::literal!("");
                    let mut str2: ArcStr = arcstr::literal!("");
                    str1 = (System::stringReplace((r#str.clone()).clone(), (literal!("%")).clone(), (literal!("")).clone())?).clone();
                    str2 = (System::stringReplace((str1.clone()).clone(), (literal!(":")).clone(), (literal!("all")).clone())?).clone();
                    let true = (nname.clone() == name.clone()) else { bail!("pattern mismatch") };
                    Ok(str2.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(getArg(rest.clone(), (name.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(val)
}

fn extendsType(mut elems: Arc<metamodelica::List<Arc<SCode::Element>>>, mut typeName: ArcStr) -> Result<(bool, Option<Arc<SCode::Mod>>)> {
    let mut result: bool = false;
    let mut mods: Option<Arc<SCode::Mod>> = None;
    (result, mods) = 'mc: {
        let __mc_input = elems.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    Ok((false, None))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ SCode::Element::EXTENDS { baseClassPath: Deref @ Absyn::Path::IDENT { name: tName }, visibility: _, modifications: r#mod, ann: _, info: _ }, tail: _ } => {
                    let true = (tName.clone() == typeName.clone()) else { bail!("pattern mismatch") };
                    Ok((true, Some(r#mod.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    Ok(extendsType(rest.clone(), (typeName.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((result, mods))
}

fn getValue(mut r#mod: Arc<SCode::Mod>, mut name: Ident, mut retype: ArcStr) -> Result<Arc<Absyn::Exp>> {
    let mut val: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    val = (::match_deref::match_deref! { match &(r#mod.clone()) {
        Deref @ SCode::Mod::MOD { finalPrefix: _, eachPrefix: _, subModLst: smod, binding: _, comment: _, .. } => {
            getValueR(smod.clone(), (name.clone()).clone(), (retype.clone()).clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(val)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getValueR(mut smod: Arc<metamodelica::List<Arc<SCode::SubMod>>>, mut name: Ident, mut retype: ArcStr) -> Result<Arc<Absyn::Exp>> {
    let mut val: Arc<Absyn::Exp> = Arc::new(Absyn::Exp::BREAK);
    val = 'mc: {
        let __mc_input = (smod.clone(), retype.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ "bool") => {
                    Ok(Arc::new(Absyn::Exp::BOOL { value: false }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ "array") => {
                    Ok(Arc::new(Absyn::Exp::ARRAY { arrayExp: metamodelica::nil() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, Deref @ "string") => {
                    Ok(Arc::new(Absyn::Exp::STRING { value: (literal!("")).clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ SCode::SubMod { ident: n, r#mod: Deref @ SCode::Mod::MOD { finalPrefix: _, eachPrefix: _, subModLst: _, binding: Some(eval), comment: _, .. } }, tail: _ }, _) => {
                    if n.clone() != name.clone() {
                        bail!("fail");
                    }
                    Ok(eval.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: rest }, _) => {
                    Ok(getValueR(rest.clone(), (name.clone()).clone(), (retype.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(val)
}

