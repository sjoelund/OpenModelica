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
use openmodelica_ast::Absyn;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

fn convertElementToClass(inElementItem: Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    let __pa0 = ::match_deref::match_deref! { match &(inElementItem.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outClass = __pa0.clone();
    Ok(outClass)
}

fn createMetaClasses(inClass: Arc<Absyn::Class>) -> Result<(Arc<Absyn::Class>, Arc<metamodelica::List<Arc<Absyn::Class>>>)> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    let mut outMetaClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut body: Arc<Absyn::ClassDef>;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    let _ = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::CLASS { body: body @ Deref @ Absyn::PARTS { classParts: parts, .. }, restriction: Absyn::R_UNIONTYPE, .. } => {
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut body = (*body).clone();
            let mut parts = (*parts).clone();
            (parts, outMetaClasses) = fixClassParts(parts.clone(), (outClass.name.clone()).clone(), var_field!((**body).typeVars, Absyn::ClassDef::PARTS).clone())?;
            let __owned_variant_classParts_0 = parts.clone();
            if let Absyn::ClassDef::PARTS { classParts, .. } = &mut body {
                *classParts = __owned_variant_classParts_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
            assign_variant_field!(outClass => Absyn::Class::CLASS; body = Arc::new(body.clone()));
            ()
        },
        Deref @ Absyn::CLASS { body: body @ Deref @ Absyn::CLASS_EXTENDS { parts, .. }, restriction: Absyn::R_UNIONTYPE, .. } => {
            let mut typeVars: Arc<metamodelica::List<ArcStr>>;
            let mut body = (*body).clone();
            let mut parts = (*parts).clone();
            (parts, outMetaClasses) = fixClassParts(parts.clone(), (outClass.name.clone()).clone(), metamodelica::nil())?;
            let __owned_variant_parts_0 = parts.clone();
            if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut body {
                *parts = __owned_variant_parts_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
            assign_variant_field!(outClass => Absyn::Class::CLASS; body = Arc::new(body.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    let _ = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::CLASS { body: body @ Deref @ Absyn::PARTS { .. }, .. } => {
            let mut body = (*body).clone();
            let __owned_variant_classParts_0 = createMetaClassesFromClassParts(var_field!((**body).classParts, Absyn::ClassDef::PARTS).clone())?;
            if let Absyn::ClassDef::PARTS { classParts, .. } = &mut body {
                *classParts = __owned_variant_classParts_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
            assign_variant_field!(outClass => Absyn::Class::CLASS; body = Arc::new(body.clone()));
            ()
        },
        Deref @ Absyn::CLASS { body: body @ Deref @ Absyn::CLASS_EXTENDS { .. }, .. } => {
            let mut body = (*body).clone();
            let __owned_variant_parts_0 = createMetaClassesFromClassParts(var_field!((**body).parts, Absyn::ClassDef::CLASS_EXTENDS).clone())?;
            if let Absyn::ClassDef::CLASS_EXTENDS { parts, .. } = &mut body {
                *parts = __owned_variant_parts_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::CLASS_EXTENDS"); }
            assign_variant_field!(outClass => Absyn::Class::CLASS; body = Arc::new(body.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClass, outMetaClasses))
}

fn createMetaClassesFromClassParts(inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ClassPart>>>> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    outClassParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for p in (inClassParts.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::PUBLIC { .. } => {
            p.contents = createMetaClassesFromElementItems(var_field!(p.contents, Absyn::ClassPart::PUBLIC).clone())?; // TODO: unhandled field-assign shape
            p.clone()
        },
        Deref @ Absyn::PROTECTED { .. } => {
            p.contents = createMetaClassesFromElementItems(var_field!(p.contents, Absyn::ClassPart::PROTECTED).clone())?; // TODO: unhandled field-assign shape
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

fn createMetaClassesFromElementItems(inElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut cls: Arc<Absyn::Class>;
    let mut meta_classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
    let mut els: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    for e in &*inElementItems.clone().reverse() {
        e = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { class_: cls, .. }, .. } } => {
            let mut cls = (*cls).clone();
            (cls, meta_classes) = createMetaClasses(cls.clone())?;
            els = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for c in (meta_classes.clone()).into_iter().cloned() {
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

pub fn createMetaClassesInProgram(inProgram: Absyn::Program) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = inProgram.clone();
    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut meta_classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
    if !(Config::acceptMetaModelicaGrammar()?) {
        return Ok(outProgram);
    }
    let _ = (match outProgram.clone() {
        Absyn::PROGRAM { .. } => {
            for c in &*outProgram.classes.clone() {
                (c, meta_classes) = createMetaClasses(c.clone())?;
                classes = cons(c.clone(), listAppend(meta_classes.clone(), classes.clone()));
            }
            let __owned_variant_classes_0 = Dangerous::listReverseInPlace(classes.clone());
            if let Absyn::Program::PROGRAM { classes, .. } = &mut outProgram {
                *classes = __owned_variant_classes_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Program::PROGRAM"); }
            ()
        },
        _ => (),
    });
    Ok(outProgram)
}

fn fixClassParts(inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, inClassName: ArcStr, typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, Arc<metamodelica::List<Arc<Absyn::Class>>>)> {
    let mut outClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    let mut outMetaClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut meta_classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
    let mut els: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    outClassParts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for p in (inClassParts.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::PUBLIC { .. } => {
            (els, meta_classes) = fixElementItems(var_field!(p.contents, Absyn::ClassPart::PUBLIC).clone(), (inClassName.clone()).clone(), typeVars.clone())?;
            p.contents = els.clone(); // TODO: unhandled field-assign shape
            outMetaClasses = listAppend(meta_classes.clone(), outMetaClasses.clone());
            p.clone()
        },
        Deref @ Absyn::PROTECTED { .. } => {
            (els, meta_classes) = fixElementItems(var_field!(p.contents, Absyn::ClassPart::PROTECTED).clone(), (inClassName.clone()).clone(), typeVars.clone())?;
            p.contents = els.clone(); // TODO: unhandled field-assign shape
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

fn fixElementItems(inElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, inName: ArcStr, typeVars: Arc<metamodelica::List<ArcStr>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, Arc<metamodelica::List<Arc<Absyn::Class>>>)> {
    let mut outElementItems: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut outMetaClasses: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut index: i32 = 0;
    let mut singleton: bool = {
        let mut __acc: i32 = 0;
        for e in (inElementItems.clone()).into_iter().cloned() {
            let __x = if (AbsynUtil::isElementItem(e.clone())) {1} else {0};
            __acc += __x;
        }
        __acc
    } == 1;
    let mut c: Arc<Absyn::Class>;
    let mut r: Absyn::Restriction;
    outElementItems = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
        for e in (inElementItems.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { class_: c @ Deref @ Absyn::CLASS { restriction: Absyn::R_RECORD, .. }, .. }, .. } } => {
            let mut body: Arc<Absyn::ClassDef>;
            let mut c = (*c).clone();
            body = c.body.clone();
            let _ = (::match_deref::match_deref! { match &(body.clone()) {
        Deref @ Absyn::PARTS { typeVars: Deref @ metamodelica::List::Cons { head: _, tail: _ }, .. } => {
            Error::addSourceMessage(Error::METARECORD_WITH_TYPEVARS.clone(), list![stringDelimitList(var_field!((*body).typeVars, Absyn::ClassDef::PARTS).clone(), (literal!(",")).clone())], c.info.clone())?;
            bail!("fail")
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            r = Absyn::Restriction::R_METARECORD { name: Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), index: index.clone(), singleton: singleton.clone(), moved: true, typeVars: typeVars.clone() };
            c.restriction = r.clone(); // TODO: unhandled field-assign shape
            outMetaClasses = cons(c.clone(), outMetaClasses.clone());
            r = Absyn::Restriction::R_METARECORD { name: Arc::new(Absyn::Path::IDENT { name: (inName.clone()).clone() }), index: index.clone(), singleton: singleton.clone(), moved: false, typeVars: typeVars.clone() };
            c.restriction = r.clone(); // TODO: unhandled field-assign shape
            index = index.clone() + 1;
            setElementItemClass(e.clone(), c.clone())
        },
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    Ok((outElementItems, outMetaClasses))
}

fn setElementItemClass(inElementItem: Arc<Absyn::ElementItem>, inClass: Arc<Absyn::Class>) -> Arc<Absyn::ElementItem> {
    let mut outElementItem: Arc<Absyn::ElementItem> = inElementItem.clone();
    outElementItem = (::match_deref::match_deref! { match &(outElementItem.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: e @ Deref @ Absyn::ELEMENT { specification: es @ Deref @ Absyn::CLASSDEF { .. }, .. } } => {
            let mut e = (*e).clone();
            let mut es = (*es).clone();
            es.class_ = inClass.clone(); // TODO: unhandled field-assign shape
            let __owned_variant_specification_0 = es.clone();
            if let Absyn::Element::ELEMENT { specification, .. } = &mut e {
                *specification = __owned_variant_specification_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Element::ELEMENT"); }
            assign_variant_field!(outElementItem => Absyn::ElementItem::ELEMENTITEM; element = e.clone());
            outElementItem.clone()
        },
        _ => outElementItem.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElementItem
}

pub fn transformArrayNodesToListNodes(inList: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Arc<metamodelica::List<Arc<Absyn::Exp>>> {
    let mut outList: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    outList = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for e in (inList.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ARRAY { arrayExp: Deref @ metamodelica::List::Nil } => Arc::new(Absyn::Exp::LIST { exps: metamodelica::nil() }),
        Deref @ Absyn::ARRAY { .. } => Arc::new(Absyn::Exp::LIST { exps: transformArrayNodesToListNodes(var_field!(e.arrayExp, Absyn::Exp::ARRAY).clone()) }),
        _ => e.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outList
}

