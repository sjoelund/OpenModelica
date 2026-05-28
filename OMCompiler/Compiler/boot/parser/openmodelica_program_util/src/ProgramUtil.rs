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

use openmodelica_ast::Absyn;
use openmodelica_frontend::FBuiltin;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_util::Autoconf;
use openmodelica_util::Error;
use openmodelica_util::JSON;
use openmodelica_util::Print;
use openmodelica_util::Settings;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

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

pub fn updateProgram(mut inNewProgram: Absyn::Program, mut inOldProgram: Absyn::Program, mut mergeAST: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
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
pub fn updateProgram2(mut inNewClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>, mut w: Absyn::Within, mut inOldProgram: Absyn::Program, mut mergeAST: bool) -> Result<Absyn::Program> {
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    outProgram = (::match_deref::match_deref! { match &((inNewClasses.clone(), w.clone(), inOldProgram.clone())) {
        (Deref @ metamodelica::List::Nil, _, prg) => {
            prg.clone()
        },
        (Deref @ metamodelica::List::Cons { head: c1 @ Deref @ Absyn::Class { name, .. }, tail: c2 }, Absyn::Within::TOP, p2 @ Absyn::Program { within_: w2, classes: c3 }) => {
            let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            if classInProgram((name.clone()).clone(), p2.clone())? {
                newp = replaceClassInProgram(c1.clone(), p2.clone(), mergeAST.clone())?;
            } else {
                newp = Absyn::Program { classes: cons(c1.clone(), c3.clone()), within_: w2.clone() };
            }
            updateProgram2(c2.clone(), w.clone(), newp.clone(), mergeAST.clone())?
        },
        (Deref @ metamodelica::List::Cons { head: c1, tail: c2 }, Absyn::Within::WITHIN { .. }, p2) => {
            let mut newp: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            let mut newp_1: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
            newp = insertClassInProgram(c1.clone(), w.clone(), p2.clone(), mergeAST.clone())?;
            newp_1 = updateProgram2(c2.clone(), w.clone(), newp.clone(), mergeAST.clone())?;
            newp_1.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outProgram)
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
        let () = (::match_deref::match_deref! { match &(elt.clone()) {
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

pub fn getComponentItemsName(mut inComponents: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, mut inQuoteNames: bool) -> Arc<metamodelica::List<ArcStr>> {
    let mut outStrings: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut name: ArcStr = arcstr::literal!("");
    for mut comp in &*inComponents.clone().reverse() {
        let mut comp = comp.clone();
        let () = (::match_deref::match_deref! { match &(comp.clone()) {
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

pub fn replaceClassInProgram2(mut inClass: Arc<Absyn::Class>, mut inClassName: ArcStr) -> Result<bool> {
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
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    let mut cls_name1: ArcStr = arcstr::literal!("");
    let mut clst: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut clsFilter: Arc<metamodelica::List<Arc<Absyn::Class>>> = metamodelica::nil();
    let mut w: Absyn::Within = Absyn::Within::TOP;
    let mut replaced: bool = false;
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
    let mut outProgram: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
    outProgram = 'mc: {
        let __mc_input = (inClass.clone(), inWithin.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (c1, w @ Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { name: n1, .. } }, p @ Absyn::Program { .. }) => {
                    let mut c2: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut c3: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
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
                    let mut c2: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut c3: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut pnew: Absyn::Program = <Absyn::Program as ::std::default::Default>::default();
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
                    s2 = stringAppendList(List::map1r(({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut p in (paths.clone()).into_iter().cloned() {
                    let __x = AbsynUtil::pathString(p.clone(), (literal!(".")).clone(), true, false)?;
                    __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), (std::sync::Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<ArcStr> + 'static>), (literal!("\n  ")).clone()));
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

pub fn insertClassInClass(mut inClass1: Arc<Absyn::Class>, mut inWithin2: Absyn::Within, mut inClass3: Arc<Absyn::Class>, mut mergeAST: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &((inClass1.clone(), inWithin2.clone(), inClass3.clone())) {
        (c1, Absyn::Within::WITHIN { path: Deref @ Absyn::Path::IDENT { .. } }, c2) => {
            replaceInnerClass(c1.clone(), c2.clone(), mergeAST.clone())?
        },
        (c1, Absyn::Within::WITHIN { path: Deref @ Absyn::Path::QUALIFIED { path, .. } }, c2) => {
            let mut cnew: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut cinner: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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

pub fn replaceInnerClass(mut inClass1: Arc<Absyn::Class>, mut inClass2: Arc<Absyn::Class>, mut mergeAST: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
                    assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
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
                    assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
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
                    assign_field!(outClass.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars.clone(), classAttrs: classAttrs.clone(), classParts: parts2.clone(), ann: ann.clone(), comment: cmt.clone() }));
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
                    assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
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
                    assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
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
                    assign_field!(outClass.body = Arc::new(Absyn::ClassDef::CLASS_EXTENDS { baseClassName: (bcname.clone()).clone(), modifications: modif.clone(), comment: cmt.clone(), parts: parts2.clone(), ann: ann.clone() }));
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

pub fn replaceClassInElementitemlist(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inClass: Arc<Absyn::Class>, mut mergeAST: bool) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, bool)> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut replaced: bool = false;
    (outAbsynElementItemLst, replaced) = (::match_deref::match_deref! { match &((inAbsynElementItemLst.clone(), inClass.clone())) {
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { constrainClass: h, specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: c1 @ Deref @ Absyn::Class { name: name1, .. }, replaceable_: e }, innerOuter: io, redeclareKeywords: b, finalPrefix: a, .. } }, tail: xs }, c2 @ Deref @ Absyn::Class { name, .. }) if (stringEq((name1.clone()).clone(), (name.clone()).clone())) => {
            let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
            let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
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

pub fn addClassInElementitemlist(mut inAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inClass: Arc<Absyn::Class>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outAbsynElementItemLst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::Class { info: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    outAbsynElementItemLst = listAppend(inAbsynElementItemLst.clone(), list![Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: false, redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: false, class_: inClass.clone() }), info: info.clone(), constrainClass: None }) })]);
    Ok(outAbsynElementItemLst)
}

pub fn getInnerClass(mut inClass: Arc<Absyn::Class>, mut inIdent: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = 'mc: {
        let __mc_input = (inClass.clone(), inIdent.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, name) => {
                    let mut publst: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
                    let mut c1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
                    let mut c1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
                    let mut c1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
                    let mut c1: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn deletePublicList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
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
pub fn deleteProtectedList(mut inAbsynClassPartLst: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
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

pub fn getClassFromElementitemlist(mut inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inIdent: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut elem: Arc<Absyn::ElementItem>;
    elem = List::getMemberOnTrue((inIdent.clone()).clone(), inElements.clone(), (std::sync::Arc::new(fnptr!(classElementItemIsNamed, ArcStr, Arc<Absyn::ElementItem>)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::ElementItem>) -> Result<bool> + 'static>))?;
    let __pa0 = ::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outClass = __pa0.clone();
    Ok(outClass)
}

pub fn classInProgram(mut name: ArcStr, mut p: Absyn::Program) -> Result<bool> {
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
                    return Ok(b.clone());
                }
            }
            false
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(b)
}

pub fn getPathedClassInProgram(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut enclOnErr: bool, mut showError: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
pub fn getPathedClassInProgramWork(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut enclOnErr: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    outClass = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => {
            getClassInProgram((var_field!((*inPath).name, Absyn::Path::IDENT).clone()).clone(), inProgram.clone())?
        },
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
pub fn getPathedClassInClass(mut inPath: Arc<Absyn::Path>, mut inClass: Arc<Absyn::Class>, mut enclOnError: bool) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
                    let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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

pub fn getClassInClass(mut name: ArcStr, mut inClass: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    for mut part in &*AbsynUtil::getClassPartsInClass(inClass.clone()) {
        let mut part = part.clone();
        for mut item in &*AbsynUtil::getElementItemsInClassPart(part.clone()) {
            let mut item = item.clone();
            if AbsynUtil::isElementItemClassNamed((name.clone()).clone(), item.clone()) {
                outClass = AbsynUtil::elementItemClass(item.clone())?;
                return Ok(outClass.clone());
            }
        }
    }
    bail!("fail");
    Ok(outClass)
}

pub fn getClassInProgram(mut name: ArcStr, mut program: Absyn::Program) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    cls = List::find(program.classes.clone(), Arc::new({ let __pe_b0 = (name.clone()).clone(); move |__pe_a1| Ok(AbsynUtil::isClassNamed(__pe_b0.clone(), __pe_a1)) }))?;
    Ok(cls)
}

pub fn getClassnamesInClassList(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClass: Arc<Absyn::Class>, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outString: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    outString = (::match_deref::match_deref! { match &((inClass.clone(), inShowProtected.clone(), includeConstants.clone())) {
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            strlist.clone()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            strlist.clone()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { .. }, .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::OVERLOAD { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::ENUMERATION { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PDER { .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outString)
}

pub fn getClassNamesRecursive(mut inPath: Option<Arc<Absyn::Path>>, mut inProgram: Absyn::Program, mut inShowProtected: bool, mut includeConstants: bool, mut inAcc: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Option<Arc<Absyn::Path>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> {
    let mut opath: Option<Arc<Absyn::Path>> = None;
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    (opath, paths) = 'mc: {
        let __mc_input = (inPath.clone(), inProgram.clone(), inShowProtected.clone(), includeConstants.clone(), inAcc.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(pp), p, b, c, acc) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut strlst: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut result_path_lst: Arc<metamodelica::List<Option<Arc<Absyn::Path>>>> = metamodelica::nil();
                    let mut acc = (*acc).clone();
                    acc = cons(pp.clone(), acc.clone());
                    cdef = getPathedClassInProgram(pp.clone(), p.clone(), false, false)?;
                    strlst = getClassnamesInClassList(pp.clone(), p.clone(), cdef.clone(), b.clone(), c.clone())?;
                    result_path_lst = List::map(List::map1(strlst.clone(), (std::sync::Arc::new(joinPaths) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> + 'static>), pp.clone()), std::sync::Arc::new(fnptr!(Util::makeOption, _)));
                    (_, acc) = List::map3Fold(result_path_lst.clone(), (std::sync::Arc::new(getClassNamesRecursive) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Path>>, Absyn::Program, bool, bool, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Option<Arc<Absyn::Path>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> + 'static>), p.clone(), b.clone(), c.clone(), acc.clone());
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
                    strlst = List::map(classes.clone(), (std::sync::Arc::new(AbsynUtil::getClassName) as std::sync::Arc<dyn ::std::ops::Fn(Arc<Absyn::Class>) -> Result<ArcStr> + 'static>));
                    result_path_lst = List::mapMap(strlst.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>), std::sync::Arc::new(fnptr!(Util::makeOption, _)));
                    (_, acc) = List::map3Fold(result_path_lst.clone(), (std::sync::Arc::new(getClassNamesRecursive) as std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Path>>, Absyn::Program, bool, bool, Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<(Option<Arc<Absyn::Path>>, Arc<metamodelica::List<Arc<Absyn::Path>>>)> + 'static>), p.clone(), b.clone(), c.clone(), acc.clone());
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

pub fn mergeClasses(mut cNew: Arc<Absyn::Class>, mut cOld: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut c: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
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
                    assign_field!(c.body = Arc::new(Absyn::ClassDef::PARTS { typeVars: typeVars1.clone(), classAttrs: classAttrs1.clone(), classParts: partsC1.clone(), ann: ann1.clone(), comment: cmt1.clone() }));
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

pub fn mergeElement(mut inEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inEl: Arc<Absyn::ElementItem>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
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
pub fn mergeElements(mut inEls1: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, mut inEls2: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
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
pub fn excludeElementsFromFile(mut inFile: ArcStr, mut inEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementItem>>>> {
    let mut outEls: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    outEls = ({
        let mut b: bool = false;
        (::match_deref::match_deref! { match &((inFile.clone(), inEls.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            metamodelica::nil()
        },
        (file, Deref @ metamodelica::List::Cons { head: e @ Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { info: SourceInfo { fileName: f, .. }, .. } }, tail: rest }) => {
            let mut filtered: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            b = stringEqual((file.clone()).clone(), (f.clone()).clone());
            filtered = excludeElementsFromFile((file.clone()).clone(), rest.clone())?;
            if (!(b.clone())) {cons(e.clone(), filtered.clone())} else {filtered.clone()}
        },
        (file, Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ElementItem::LEXER_COMMENT { comment: _ }, tail: rest }) => {
            let mut filtered: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
            filtered = excludeElementsFromFile((file.clone()).clone(), rest.clone())?;
            filtered.clone()
        },
        _ => bail!("match: no arm matched"),
    } })
    });
    Ok(outEls)
}

pub fn getClassnamesInClass(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut inClass: Arc<Absyn::Class>, mut inShowProtected: bool, mut includeConstants: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::Path>>>> {
    let mut paths: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    paths = (::match_deref::match_deref! { match &((inClass.clone(), inShowProtected.clone(), includeConstants.clone())) {
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::PARTS { classParts: parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            List::map(strlist.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>))
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::CLASS_EXTENDS { parts, .. }, .. }, b, c) => {
            let mut strlist: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            strlist = getClassnamesInParts(parts.clone(), b.clone(), c.clone())?;
            List::map(strlist.clone(), (std::sync::Arc::new(fnptr!(AbsynUtil::makeIdentPathFromString, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<Arc<Absyn::Path>> + 'static>))
        },
        (Deref @ Absyn::Class { body: Deref @ Absyn::ClassDef::DERIVED { typeSpec: Deref @ Absyn::TypeSpec::TPATH { path: _, arrayDim: _ }, .. }, .. }, _, _) => {
            metamodelica::nil()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(paths)
}

pub fn classElementItemIsNamed(mut inClassName: ArcStr, mut inElement: Arc<Absyn::ElementItem>) -> bool {
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

pub fn joinPaths(mut child: ArcStr, mut parent: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
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

pub fn getDefaultComponentPrefixesModStr(mut r#mod: Option<Arc<Absyn::Modification>>) -> Result<ArcStr> {
    let mut docStr: ArcStr = arcstr::literal!("");
    docStr = ('mc: {
        let __mc_input = r#mod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ Absyn::Modification { eqMod: Deref @ Absyn::EqMod::EQMOD { exp: e, .. }, .. }) => {
                    let mut docStr: ArcStr = docStr.clone();
                    docStr = (Dump::printExpStr(e.clone())?).clone();
                    Ok(docStr.clone())
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
    Ok(docStr)
}

pub fn getNamedAnnotationExp<T: Clone + 'static>(mut inPath: Arc<Absyn::Path>, mut inProgram: Absyn::Program, mut id: Arc<Absyn::Path>, mut default: Option<T>, mut f: Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<T> + 'static>) -> Result<T> {
    pub type ModFunc<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<T> + 'static>;

    let mut outString: T;
    outString = 'mc: {
        let __mc_input = (inPath.clone(), inProgram.clone(), default.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (modelpath, p, _) => {
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut r#str: T;
                    cdef = getPathedClassInProgram(modelpath.clone(), p.clone(), false, false)?;
                    let __pa0 = ::match_deref::match_deref! { match &(AbsynUtil::getNamedAnnotationInClass(cdef.clone(), id.clone(), f.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    r#str = __pa0.clone();
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Some(r#str)) => {
                    Ok(r#str.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outString)
}

pub fn getFileDir(mut inComponentRef: Arc<Absyn::ComponentRef>, mut inProgram: Absyn::Program) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ('mc: {
        let __mc_input = (inComponentRef.clone(), inProgram.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (class_, p) => {
                    let mut p_class: Arc<Absyn::Path>;
                    let mut cdef: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
                    let mut filename: ArcStr = arcstr::literal!("");
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut dir_1: ArcStr = arcstr::literal!("");
                    let mut pd_1: ArcStr = arcstr::literal!("");
                    let mut filename_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut dir: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    p_class = AbsynUtil::crefToPath(class_.clone())?;
                    cdef = getPathedClassInProgram(p_class.clone(), p.clone(), false, false)?;
                    filename = (AbsynUtil::classFilename(cdef.clone())?).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(stringListStringChar((pd.clone()).clone())) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    pd_1 = __pa0.clone();
                    filename_1 = Util::stringSplitAtChar((filename.clone()).clone(), (pd_1.clone()).clone())?;
                    dir = List::stripLast(filename_1.clone())?;
                    dir_1 = stringDelimitList(dir.clone(), (pd.clone()).clone());
                    Ok(dir_1.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut pd: ArcStr = arcstr::literal!("");
                    let mut dir_1: ArcStr = arcstr::literal!("");
                    let mut omhome: ArcStr = arcstr::literal!("");
                    let mut omhome_1: ArcStr = arcstr::literal!("");
                    omhome = (Settings::getInstallationDirectoryPath()?).clone();
                    omhome_1 = (System::trim((omhome.clone()).clone(), (literal!("\"")).clone())).clone();
                    pd = (arcstr::literal!(Autoconf::pathDelimiter)).clone();
                    dir_1 = stringAppendList(list![(literal!("\"")).clone(), (omhome_1.clone()).clone(), (pd.clone()).clone(), (literal!("work")).clone(), (literal!("\"")).clone()]);
                    Ok(dir_1.clone())
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
    Ok(outString)
}

pub fn getFullPathFromUri(mut program: Absyn::Program, mut uri: ArcStr, mut printError: bool) -> Result<ArcStr> {
    let mut path: ArcStr = arcstr::literal!("");
    let mut str1: ArcStr = arcstr::literal!("");
    let mut str2: ArcStr = arcstr::literal!("");
    let mut str3: ArcStr = arcstr::literal!("");
    (str1, str2, str3) = System::uriToClassAndPath((uri.clone()).clone())?;
    path = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*getBasePathFromUri((str1.clone()).clone(), (str2.clone()).clone(), program.clone(), (Settings::getModelicaPath(Testsuite::isRunning()?)?).clone(), printError.clone())?); __mm_s.push_str(&*str3.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(path)
}

pub fn getBasePathFromUri(mut scheme: ArcStr, mut iname: ArcStr, mut program: Absyn::Program, mut modelicaPath: ArcStr, mut printError: bool) -> Result<ArcStr> {
    let mut basePath: ArcStr = arcstr::literal!("");
    basePath = ('mc: {
        let __mc_input = (scheme.clone(), iname.clone(), modelicaPath.clone(), printError.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelica://", name, _, _) => {
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut mp: ArcStr = arcstr::literal!("");
                    let mut bp: ArcStr = arcstr::literal!("");
                    let mut fileName: ArcStr = arcstr::literal!("");
                    let mut name = (*name).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok((name.clone()).clone(), (literal!(".")).clone())) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    names = __pa1.clone();
                    let __pa2 = ::match_deref::match_deref! { match &(getPathedClassInProgram(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), program.clone(), false, false)?) {
                        Deref @ Absyn::Class { info: SourceInfo { fileName: __pa2, .. }, .. } => __pa2.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    fileName = __pa2.clone();
                    mp = (System::dirname((fileName.clone()).clone())).clone();
                    bp = (findModelicaPath2((mp.clone()).clone(), names.clone(), (literal!("")).clone(), true)?).clone();
                    Ok(bp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelica://", name, mp, _) => {
                    let mut isDir: bool = false;
                    let mut mps: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
                    let mut gd: ArcStr = arcstr::literal!("");
                    let mut bp: ArcStr = arcstr::literal!("");
                    let mut name = (*name).clone();
                    let mut mp = (*mp).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(System::strtok((name.clone()).clone(), (literal!(".")).clone())) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    names = __pa1.clone();
                    if '__try2: {
                        unwrap_break_err!(getPathedClassInProgram(Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), program.clone(), false, false), '__try2);
                        Ok::<(), anyhow::Error>(())
                    }.is_ok() { bail!("failure(): body succeeded") }
                    gd = (arcstr::literal!(Autoconf::groupDelimiter)).clone();
                    mps = System::strtok((mp.clone()).clone(), (gd.clone()).clone());
                    (mp, name, isDir) = System::getLoadModelPath((name.clone()).clone(), list![(literal!("default")).clone()], mps.clone(), false)?;
                    mp = (if (isDir.clone()) {{ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }} else {mp.clone()}).clone();
                    bp = (findModelicaPath2((mp.clone()).clone(), names.clone(), (literal!("")).clone(), true)?).clone();
                    Ok(bp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "file://", _, _, _) => {
                    Ok(literal!(""))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "modelica://", name, mp, true) => {
                    let mut r#str: ArcStr = arcstr::literal!("");
                    let mut name = (*name).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(System::strtok((name.clone()).clone(), (literal!(".")).clone())) {
                        Deref @ metamodelica::List::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    name = __pa0.clone();
                    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Could not resolve modelica://")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" with MODELICAPATH: ")); __mm_s.push_str(&*mp.clone()); ArcStr::from(__mm_s) }).clone();
                    Error::addMessage(Error::COMPILER_ERROR.clone(), list![(r#str.clone()).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(basePath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn findModelicaPath(mut imps: Arc<metamodelica::List<ArcStr>>, mut names: Arc<metamodelica::List<ArcStr>>, mut version: ArcStr) -> Result<ArcStr> {
    let mut basePath: ArcStr = arcstr::literal!("");
    basePath = ('mc: {
        let __mc_input = imps.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: mp, tail: _ } => {
                    Ok(findModelicaPath2((mp.clone()).clone(), names.clone(), (version.clone()).clone(), false)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: mps } => {
                    Ok(findModelicaPath(mps.clone(), names.clone(), (version.clone()).clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(basePath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn findModelicaPath2(mut mp: ArcStr, mut inames: Arc<metamodelica::List<ArcStr>>, mut version: ArcStr, mut b: bool) -> Result<ArcStr> {
    let mut basePath: ArcStr = arcstr::literal!("");
    basePath = ('mc: {
        let __mc_input = (inames.clone(), b.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: name, tail: names }, _) => {
                    let mut file: ArcStr = arcstr::literal!("");
                    let false = (stringEq((version.clone()).clone(), (literal!("")).clone())) else { bail!("pattern mismatch") };
                    file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*version.clone()); ArcStr::from(__mm_s) }).clone();
                    let true = (System::directoryExists((file.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(findModelicaPath2((file.clone()).clone(), names.clone(), (literal!("")).clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: name, tail: _ }, _) => {
                    let mut file: ArcStr = arcstr::literal!("");
                    let false = (stringEq((version.clone()).clone(), (literal!("")).clone())) else { bail!("pattern mismatch") };
                    file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*version.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone();
                    let true = (System::regularFileExists((file.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(mp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: name, tail: names }, _) => {
                    let mut file: ArcStr = arcstr::literal!("");
                    file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name.clone()); ArcStr::from(__mm_s) }).clone();
                    let true = (System::directoryExists((file.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(findModelicaPath2((file.clone()).clone(), names.clone(), (literal!("")).clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: name, tail: _ }, _) => {
                    let mut file: ArcStr = arcstr::literal!("");
                    file = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*mp.clone()); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(".mo")); ArcStr::from(__mm_s) }).clone();
                    let true = (System::regularFileExists((file.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(mp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, true) => {
                    Ok(mp.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    }).clone();
    Ok(basePath)
}

pub fn dumpJSONSourceInfo(mut info: SourceInfo, mut dumpFilename: bool) -> Result<Arc<JSON::JSON>> {
    let mut json: Arc<JSON::JSON> = JSON::makeNull();
    if dumpFilename.clone() {
        json = JSON::addPair((literal!("filename")).clone(), JSON::makeString((Testsuite::friendly(info.fileName.clone())?).clone()), json.clone())?;
    }
    json = JSON::addPair((literal!("lineStart")).clone(), JSON::makeInteger(info.lineNumberStart.clone()), json.clone())?;
    json = JSON::addPair((literal!("columnStart")).clone(), JSON::makeInteger(info.columnNumberStart.clone()), json.clone())?;
    json = JSON::addPair((literal!("lineEnd")).clone(), JSON::makeInteger(info.lineNumberEnd.clone()), json.clone())?;
    json = JSON::addPair((literal!("columnEnd")).clone(), JSON::makeInteger(info.columnNumberEnd.clone()), json.clone())?;
    if info.isReadOnly.clone() {
        json = JSON::addPair((literal!("readonly")).clone(), JSON::makeBoolean(true), json.clone())?;
    }
    Ok(json)
}

