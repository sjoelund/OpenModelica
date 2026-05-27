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
use openmodelica_ast::Absyn;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

pub fn createMetaClassesInProgram(mut inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = inProgram.clone();
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut meta_classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    if !(Config::acceptMetaModelicaGrammar()?) {
        return Ok(outProgram);
    }
    let _ = (match outProgram.clone() {
        Absyn::Program { .. } => {
            for mut c in &*outProgram.classes.clone() {
                let mut c = c.clone();
                (c, meta_classes) = createMetaClasses(c.clone())?;
                classes = cons(c.clone(), listAppend(meta_classes.clone(), classes.clone()));
            }
            outProgram.classes = Dangerous::listReverseInPlace(classes.clone());
            ()
        },
        _ => (),
    });
    Ok(outProgram)
}

fn createMetaClasses(mut inClass: Arc<Absyn::Class>) -> Result<(Arc<Absyn::Class>, Arc<metamodelica::List<Arc<Absyn::Class>>>)> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    let mut outMetaClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut body: Arc<Absyn::ClassDef>;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let _ = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::Class { body: body @ Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, restriction: Absyn::Restriction::R_UNIONTYPE, .. } => {
            let mut body = (*body).clone();
            let mut parts = (*parts).clone();
            (parts, outMetaClasses) = fixClassParts(parts.clone(), (outClass.name.clone()).clone(), var_field!((*body).typeVars, Absyn::ClassDef::PARTS).clone())?;
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = parts.clone());
            assign_field!(outClass.body = body.clone());
            ()
        },
        Deref @ Absyn::Class { body: body @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, restriction: Absyn::Restriction::R_UNIONTYPE, .. } => {
            let mut body = (*body).clone();
            let mut parts = (*parts).clone();
            (parts, outMetaClasses) = fixClassParts(parts.clone(), (outClass.name.clone()).clone(), metamodelica::nil())?;
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
            assign_field!(outClass.body = body.clone());
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    let _ = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::Class { body: body @ Deref @ Absyn::ClassDef::PARTS { .. }, .. } => {
            let mut body = (*body).clone();
            assign_variant_field!(body => Absyn::ClassDef::PARTS; classParts = createMetaClassesFromClassParts(var_field!((*body).classParts, Absyn::ClassDef::PARTS).clone())?);
            assign_field!(outClass.body = body.clone());
            ()
        },
        Deref @ Absyn::Class { body: body @ Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. }, .. } => {
            let mut body = (*body).clone();
            assign_variant_field!(body => Absyn::ClassDef::CLASS_EXTENDS; parts = createMetaClassesFromClassParts(var_field!((*body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?);
            assign_field!(outClass.body = body.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClass, outMetaClasses))
}

fn createMetaClassesFromClassParts(mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    outClassParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (inClassParts.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            assign_variant_field!(p => Absyn::ClassPart::PUBLIC; contents = createMetaClassesFromElementItems(var_field!((*p).contents, Absyn::ClassPart::PUBLIC).clone())?);
            p.clone()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            assign_variant_field!(p => Absyn::ClassPart::PROTECTED; contents = createMetaClassesFromElementItems(var_field!((*p).contents, Absyn::ClassPart::PROTECTED).clone())?);
            p.clone()
        },
        _ => p.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok(outClassParts)
}

fn createMetaClassesFromElementItems(mut inElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut meta_classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut els: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    for mut e in &*inElementItems.clone().reverse() {
        let mut e = e.clone();
        e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: cls, .. }, .. } } => {
            let mut cls = (*cls).clone();
            (cls, meta_classes) = createMetaClasses(cls.clone())?;
            els = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut c in (meta_classes.clone()).into_iter().cloned() {
            let __x = setElementItemClass(e.clone(), c.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            outElementItems = listAppend(els.clone(), outElementItems.clone());
            setElementItemClass(e.clone(), cls.clone())
        },
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outElementItems = cons(e.clone(), outElementItems.clone());
    }
    Ok(outElementItems)
}

fn setElementItemClass(mut inElementItem: Arc<Absyn::ElementItem>, mut inClass: Arc<Absyn::Class>) -> Arc<Absyn::ElementItem> {
    let mut outElementItem: Arc<Absyn::ElementItem> = inElementItem.clone();
    outElementItem = (::match_deref::match_deref! { match &(outElementItem.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: e @ Deref @ Absyn::Element::ELEMENT { specification: es @ Deref @ Absyn::ElementSpec::CLASSDEF { .. }, .. } } => {
            let mut e = (*e).clone();
            let mut es = (*es).clone();
            assign_variant_field!(es => Absyn::ElementSpec::CLASSDEF; class_ = inClass.clone());
            assign_variant_field!(e => Absyn::Element::ELEMENT; specification = es.clone());
            assign_variant_field!(outElementItem => Absyn::ElementItem::ELEMENTITEM; element = e.clone());
            outElementItem.clone()
        },
        _ => {
            outElementItem.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElementItem
}

fn convertElementToClass(mut inElementItem: Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let __pa0 = ::match_deref::match_deref! { match &(inElementItem.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outClass = __pa0.clone();
    Ok(outClass)
}

fn fixClassParts(mut inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, mut inClassName: ArcStr, mut typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, Arc<metamodelica::List<Arc<Absyn::Class>>>)> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut outMetaClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut meta_classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut els: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outClassParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for mut p in (inClassParts.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            (els, meta_classes) = fixElementItems(var_field!((*p).contents, Absyn::ClassPart::PUBLIC).clone(), (inClassName.clone()).clone(), typeVars.clone())?;
            assign_variant_field!(p => Absyn::ClassPart::PUBLIC; contents = els.clone());
            outMetaClasses = listAppend(meta_classes.clone(), outMetaClasses.clone());
            p.clone()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            (els, meta_classes) = fixElementItems(var_field!((*p).contents, Absyn::ClassPart::PROTECTED).clone(), (inClassName.clone()).clone(), typeVars.clone())?;
            assign_variant_field!(p => Absyn::ClassPart::PROTECTED; contents = els.clone());
            outMetaClasses = listAppend(meta_classes.clone(), outMetaClasses.clone());
            p.clone()
        },
        _ => p.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outClassParts, outMetaClasses))
}

fn fixElementItems(mut inElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inName: ArcStr, mut typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, Arc<metamodelica::List<Arc<Absyn::Class>>>)> {
    let mut outElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut outMetaClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut singleton: bool = {
        let mut __acc: i32 = 0;
        for mut e in (inElementItems.clone()).into_iter().cloned() {
            let __x = if (AbsynUtil::isElementItem(e.clone())) {1} else {0};
            __acc += __x;
        }
        __acc
    } == 1;
    let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut r: Absyn::Restriction = Absyn::Restriction::R_BLOCK;
    outElementItems = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for mut e in (inElementItems.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: c @ Deref @ Absyn::Class { restriction: Absyn::Restriction::R_RECORD, .. }, .. }, .. } } => {
            let mut body: Arc<Absyn::ClassDef>;
            let mut c = (*c).clone();
            body = c.body.clone();
            let _ = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::ClassDef::PARTS { typeVars: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
            Error::addSourceMessage(Error::METARECORD_WITH_TYPEVARS.clone(), list![stringDelimitList(var_field!((*body).typeVars, Absyn::ClassDef::PARTS).clone(), (literal!(",")).clone())], c.info.clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            r = Absyn::Restriction::R_METARECORD { name: Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), index: index.clone(), singleton: singleton.clone(), moved: true, typeVars: typeVars.clone() };
            assign_field!(c.restriction = r.clone());
            outMetaClasses = cons(c.clone(), outMetaClasses.clone());
            r = Absyn::Restriction::R_METARECORD { name: Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), index: index.clone(), singleton: singleton.clone(), moved: false, typeVars: typeVars.clone() };
            assign_field!(c.restriction = r.clone());
            index = index.clone() + 1;
            setElementItemClass(e.clone(), c.clone())
        },
        _ => {
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outElementItems, outMetaClasses))
}

pub fn transformArrayNodesToListNodes(mut inList: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Arc<metamodelica::List<Arc<Absyn::Exp>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    outList = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for mut e in (inList.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Nil } => Arc::new(Absyn::Exp::LIST { exps: metamodelica::nil() }),
        Deref @ Absyn::Exp::ARRAY { .. } => Arc::new(Absyn::Exp::LIST { exps: transformArrayNodesToListNodes(var_field!((*e).arrayExp, Absyn::Exp::ARRAY).clone()) }),
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

