// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use crate::Dump;
use openmodelica_ast::Absyn;
use openmodelica_util::Error;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type IteratorIndexedCref = (Arc<Absyn::ComponentRef>, i32);

pub fn addSubscriptsLast(icr: Arc<Absyn::ComponentRef>, i: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut ocr: Arc<Absyn::ComponentRef>;
    ocr = (::match_deref::match_deref! { match &((icr.clone(), i.clone())) {
        (Deref @ Absyn::CREF_IDENT { name: id, subscripts: subs }, _) => Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: listAppend(subs.clone(), i.clone()) }),
        (Deref @ Absyn::CREF_QUAL { name: id, subscripts: subs, componentRef: cr }, _) => {
            let mut cr = (*cr).clone();
            cr = addSubscriptsLast(cr.clone(), i.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id.clone()).clone(), subscripts: subs.clone(), componentRef: cr.clone() })
        },
        (Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cr }, _) => {
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut id: ArcStr;
            let mut cr = (*cr).clone();
            cr = addSubscriptsLast(cr.clone(), i.clone())?;
            crefMakeFullyQualified(cr.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ocr)
}

pub fn allFieldsAreCrefs(expLst: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> bool {
    let mut b: bool;
    b = List::all(expLst.clone(), Arc::new(fnptr!(complexIsCref, Arc<Absyn::Exp>)));
    b
}

pub fn annotationEqual(ann1: Arc<Absyn::Annotation>, ann2: Arc<Absyn::Annotation>) -> bool {
    let mut equal: bool = List::isEqualOnTrue(ann1.elementArgs.clone(), ann2.elementArgs.clone(), Arc::new(elementArgEqual));
    equal
}

pub fn annotationToElementArgs(ann: Arc<Absyn::Annotation>) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let __pa0 = ::match_deref::match_deref! { match &(ann.clone()) {
        Deref @ Absyn::ANNOTATION { elementArgs: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    args = __pa0.clone();
    Ok(args)
}

pub fn appendEquation(eq: Arc<Absyn::EquationItem>, isInitial: bool, cls: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    fn append_eq(eq: Arc<Absyn::EquationItem>, isInitial: bool, part: Arc<Absyn::ClassPart>) -> (Arc<Absyn::ClassPart>, bool) {
        let mut part: Arc<Absyn::ClassPart> = part;
        let mut found: bool;
        found = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::EQUATIONS { .. } if (!(isInitial.clone())) => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = List::appendElt(eq.clone(), var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone()));
            true
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } if (isInitial.clone()) => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = List::appendElt(eq.clone(), var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone()));
            true
        },
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (part, found)
    }

    let mut cls: Arc<Absyn::Class> = cls;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    let mut found: bool;
    parts = getClassPartsInClass(cls.clone()).reverse();
    (parts, found) = List::findMap(parts.clone(), Arc::new({ let __pe_b0 = eq.clone(); let __pe_b1 = isInitial.clone(); move |__pe_a2| Ok(append_eq(__pe_b0.clone(), __pe_b1.clone(), __pe_a2)) }))?;
    if !(found.clone()) {
        parts = if (isInitial.clone()) {cons(Arc::new(Absyn::ClassPart::INITIALEQUATIONS { contents: list![eq.clone()] }), parts.clone())} else {cons(Arc::new(Absyn::ClassPart::EQUATIONS { contents: list![eq.clone()] }), parts.clone())};
    }
    cls = setClassPartsInClass(parts.clone().reverse(), cls.clone())?;
    Ok(cls)
}

pub fn canonIfExp(inExp: Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> {
    let mut outExp: Arc<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::IFEXP { elseIfBranch: Deref @ metamodelica::List::Nil, .. } => inExp.clone(),
        Deref @ Absyn::IFEXP { elseIfBranch: Deref @ metamodelica::List::Cons { head: (ei_cond, ei_tb), tail: eib }, elseBranch: eb, trueBranch: tb, ifExp: cond } => {
            let mut e: Arc<Absyn::Exp>;
            e = canonIfExp(Arc::new(Absyn::Exp::IFEXP { ifExp: ei_cond.clone(), trueBranch: ei_tb.clone(), elseBranch: eb.clone(), elseIfBranch: eib.clone() }))?;
            Arc::new(Absyn::Exp::IFEXP { ifExp: cond.clone(), trueBranch: tb.clone(), elseBranch: e.clone(), elseIfBranch: metamodelica::nil() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn classDefStringComment(def: Arc<Absyn::ClassDef>) -> ArcStr {
    let mut comment: ArcStr;
    comment = ((::match_deref::match_deref! { match &(def.clone()) {
        Deref @ Absyn::ClassDef::PARTS { comment: Some(comment), .. } => comment.clone(),
        Deref @ Absyn::ClassDef::DERIVED { comment: Some(Absyn::Comment::COMMENT { comment: Some(comment), .. }), .. } => comment.clone(),
        Deref @ Absyn::ClassDef::ENUMERATION { comment: Some(Absyn::Comment::COMMENT { comment: Some(comment), .. }), .. } => comment.clone(),
        Deref @ Absyn::ClassDef::OVERLOAD { comment: Some(Absyn::Comment::COMMENT { comment: Some(comment), .. }), .. } => comment.clone(),
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { comment: Some(comment), .. } => comment.clone(),
        Deref @ Absyn::ClassDef::PDER { comment: Some(Absyn::Comment::COMMENT { comment: Some(comment), .. }), .. } => comment.clone(),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    comment
}

pub fn classFilename(inClass: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut outFilename: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::CLASS { info: SourceInfo { fileName: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outFilename = __pa0.clone();
    Ok(outFilename)
}

fn classHasLocalClasses(cl: Arc<Absyn::Class>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::CLASS { body: Deref @ Absyn::PARTS { classParts: parts, .. }, .. } => partsHasLocalClass(parts.clone())?,
        Deref @ Absyn::CLASS { body: Deref @ Absyn::CLASS_EXTENDS { parts, .. }, .. } => partsHasLocalClass(parts.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

pub fn className(cl: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn classNameCompare(c1: Arc<Absyn::Class>, c2: Arc<Absyn::Class>) -> i32 {
    let mut o: i32;
    o = stringCompare((c1.name.clone()).clone(), (c2.name.clone()).clone());
    o
}

pub fn classNameGreater(c1: Arc<Absyn::Class>, c2: Arc<Absyn::Class>) -> bool {
    let mut b: bool;
    b = stringCompare((c1.name.clone()).clone(), (c2.name.clone()).clone()) > 0;
    b
}

pub fn commentEqual(cmt1: Arc<Absyn::Comment>, cmt2: Arc<Absyn::Comment>) -> bool {
    let mut equal: bool = Util::optionEqual(cmt1.comment.clone(), cmt2.comment.clone(), Arc::new(fnptr!(stringEq, ArcStr, ArcStr))) && Util::optionEqual(cmt1.annotation_.clone(), cmt2.annotation_.clone(), Arc::new(fnptr!(annotationEqual, Arc<Absyn::Annotation>, Arc<Absyn::Annotation>)));
    equal
}

pub fn complexIsCref(inExp: Arc<Absyn::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::TUPLE { expressions: _ } => allFieldsAreCrefs(var_field!((*inExp).expressions, Absyn::Exp::TUPLE).clone()),
        Deref @ Absyn::CONS { head: _, .. } => complexIsCref(var_field!((*inExp).head, Absyn::Exp::CONS).clone()) && complexIsCref(var_field!((*inExp).rest, Absyn::Exp::CONS).clone()),
        _ => isCref(inExp.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn componentName(c: Arc<Absyn::ComponentItem>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(c.clone()) {
        Deref @ Absyn::COMPONENTITEM { component: Absyn::COMPONENT { name: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn createChoiceArray(inChoices: Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> {
    let mut outChoices: Arc<Absyn::ElementArg> = inChoices.clone();
    let mut choices: Arc<Absyn::ElementArg>;
    let mut choice: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut acc_choice: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut acc: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut c: Arc<Absyn::ElementArg>;
    let mut el: Arc<Absyn::ElementArg>;
    let mut info1: SourceInfo;
    let mut info2: SourceInfo;
    let mut cmt1: Option<ArcStr>;
    let mut cmt2: Option<ArcStr>;
    let mut fp1: bool;
    let mut fp2: bool;
    let mut ep1: Absyn::Each;
    let mut ep2: Absyn::Each;
    let mut choiceArray: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut s: ArcStr;
    let mut e: Arc<Absyn::Exp>;
    outChoices = (::match_deref::match_deref! { match &(inChoices.clone()) {
        Deref @ Absyn::MODIFICATION { info: info1, comment: cmt1, modification: Some(Absyn::CLASSMOD { elementArgLst: choice, eqMod: Deref @ Absyn::NOMOD }), path: Deref @ Absyn::IDENT { name: Deref @ "choices" }, eachPrefix: ep1, finalPrefix: fp1 } => {
            for m in choice.clone() /* Unknown type for iterator Unknown */ {
                (choiceArray, acc) = (::match_deref::match_deref! { match &(m.clone()) {
        Absyn::MODIFICATION { finalPrefix: fp2, eachPrefix: ep2, path: Deref @ Absyn::IDENT { name: Deref @ "choice" }, modification: Some(Absyn::CLASSMOD { elementArgLst: Deref @ metamodelica::List::Cons { head: el, tail: Deref @ metamodelica::List::Nil }, eqMod: Deref @ Absyn::NOMOD }), comment: cmt2, info: info2 } => {
            s = (Dump::unparseElementArgStr(el.clone())?).clone();
            (cons(s.clone(), choiceArray.clone()), acc.clone())
        },
        Absyn::MODIFICATION { finalPrefix: fp2, eachPrefix: ep2, path: Deref @ Absyn::IDENT { name: Deref @ "choice" }, modification: Some(Absyn::CLASSMOD { elementArgLst: Deref @ metamodelica::List::Nil, eqMod: Deref @ Absyn::EQMOD { exp: e, .. } }), comment: cmt2, info: info2 } => {
            s = (Dump::printExpStr(e.clone())?).clone();
            (cons(s.clone(), choiceArray.clone()), acc.clone())
        },
        _ => (choiceArray.clone(), cons(m.clone(), acc.clone())),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            if !(choiceArray.clone().is_empty()) {
                e = Arc::new(Absyn::Exp::ARRAY { arrayExp: {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for s in (choiceArray.clone().reverse()).into_iter().cloned() {
            let __x = Arc::new(Absyn::Exp::STRING { value: (s.clone()).clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    } });
                c = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fp2.clone(), eachPrefix: ep2.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("choice")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(Absyn::EqMod::EQMOD { exp: e.clone(), info: info2.clone() }) })), comment: cmt2.clone(), info: info2.clone() });
                args = cons(c.clone(), acc.clone()).reverse();
            } else {
                args = acc.clone().reverse();
            }
            choices = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: fp1.clone(), eachPrefix: ep1.clone(), path: Arc::new(Absyn::Path::IDENT { name: (literal!("choices")).clone() }), modification: Some(Arc::new(Absyn::Modification { elementArgLst: args.clone(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) })), comment: cmt1.clone(), info: info1.clone() });
            choices.clone()
        },
        _ => inChoices.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outChoices)
}

pub fn crefCompare(cr1: Arc<Absyn::ComponentRef>, cr2: Arc<Absyn::ComponentRef>) -> Result<i32> {
    let mut comp: i32;
    let mut name: ArcStr;
    let mut cr: Arc<Absyn::ComponentRef>;
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    if referenceEq(&cr1.clone(),&cr2.clone()) {
        comp = 0;
        return Ok(comp);
    }
    comp = Util::intCompare(valueConstructor(cr1.clone()), valueConstructor(cr2.clone()));
    if comp.clone() != 0 {
        return Ok(comp);
    }
    comp = (::match_deref::match_deref! { match &(cr1.clone()) {
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(cr2.clone()) {
                Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            cr = __pa0.clone();
            crefCompare(var_field!((*cr1).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), cr.clone())?
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(cr2.clone()) {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: __pa0, subscripts: __pa1, componentRef: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            subs = __pa1.clone();
            cr = __pa2.clone();
            comp = stringCompare((var_field!((*cr1).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), (name.clone()).clone());
            if comp.clone() == 0 {
                comp = List::compare(var_field!((*cr1).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), subs.clone(), Arc::new(subscriptCompare))?;
            }
            if (comp.clone() == 0) {crefCompare(var_field!((*cr1).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), cr.clone())?} else {comp.clone()}
        },
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cr2.clone()) {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name: __pa0, subscripts: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            subs = __pa1.clone();
            comp = stringCompare((var_field!((*cr1).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), (name.clone()).clone());
            if (comp.clone() == 0) {List::compare(var_field!((*cr1).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), subs.clone(), Arc::new(subscriptCompare))?} else {comp.clone()}
        },
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefEqual(cref1: Arc<Absyn::ComponentRef>, cref2: Arc<Absyn::ComponentRef>) -> bool {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((cref1.clone(), cref2.clone())) {
        (Deref @ Absyn::CREF_IDENT { .. }, Deref @ Absyn::CREF_IDENT { .. }) => stringEq((var_field!((*cref1).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cref2).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone()) && subscriptsEqual(var_field!((*cref1).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), var_field!((*cref2).subscripts, Absyn::ComponentRef::CREF_IDENT).clone()),
        (Deref @ Absyn::CREF_QUAL { .. }, Deref @ Absyn::CREF_QUAL { .. }) => stringEq((var_field!((*cref1).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cref2).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone()) && subscriptsEqual(var_field!((*cref1).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), var_field!((*cref2).subscripts, Absyn::ComponentRef::CREF_QUAL).clone()) && crefEqual(var_field!((*cref1).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), var_field!((*cref2).componentRef, Absyn::ComponentRef::CREF_QUAL).clone()),
        (Deref @ Absyn::CREF_FULLYQUALIFIED { .. }, Deref @ Absyn::CREF_FULLYQUALIFIED { .. }) => crefEqual(var_field!((*cref1).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), var_field!((*cref2).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefEqualNoSubs(cr1: Arc<Absyn::ComponentRef>, cr2: Arc<Absyn::ComponentRef>) -> bool {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((cr1.clone(), cr2.clone())) {
        (Deref @ Absyn::CREF_IDENT { .. }, Deref @ Absyn::CREF_IDENT { .. }) => stringEq((var_field!((*cr1).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone(), (var_field!((*cr2).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone()),
        (Deref @ Absyn::CREF_QUAL { .. }, Deref @ Absyn::CREF_QUAL { .. }) => stringEq((var_field!((*cr1).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), (var_field!((*cr2).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone()) && crefEqualNoSubs(var_field!((*cr1).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), var_field!((*cr2).componentRef, Absyn::ComponentRef::CREF_QUAL).clone()),
        (Deref @ Absyn::CREF_FULLYQUALIFIED { .. }, Deref @ Absyn::CREF_FULLYQUALIFIED { .. }) => crefEqualNoSubs(var_field!((*cr1).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), var_field!((*cr2).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

pub fn crefExp(cr: Arc<Absyn::ComponentRef>) -> Arc<Absyn::Exp> {
    let mut exp: Arc<Absyn::Exp>;
    exp = Arc::new(Absyn::Exp::CREF { componentRef: cr.clone() });
    exp
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefExplode(inCref: Arc<Absyn::ComponentRef>, inAccum: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>) -> Arc<metamodelica::List<Arc<Absyn::ComponentRef>>> {
    let mut outCrefParts: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outCrefParts = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::CREF_QUAL { .. } => crefExplode(var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), cons(crefFirstCref(inCref.clone()), inAccum.clone())),
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefExplode(var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), inAccum.clone()),
        _ => cons(inCref.clone(), inAccum.clone()).reverse(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCrefParts
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefFirstCref(inCref: Arc<Absyn::ComponentRef>) -> Arc<Absyn::ComponentRef> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::CREF_QUAL { .. } => Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (var_field!((*inCref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone(), subscripts: var_field!((*inCref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone() }),
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefFirstCref(var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone()),
        _ => inCref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub fn crefFirstEqual(iCr1: Arc<Absyn::ComponentRef>, iCr2: Arc<Absyn::ComponentRef>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = stringEq((crefFirstIdent(iCr1.clone())?).clone(), (crefFirstIdent(iCr2.clone())?).clone());
    Ok(outBoolean)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefFirstIdent(inCref: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => var_field!((*inCref).name, Absyn::ComponentRef::CREF_IDENT).clone(),
        Deref @ Absyn::CREF_QUAL { .. } => var_field!((*inCref).name, Absyn::ComponentRef::CREF_QUAL).clone(),
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefFirstIdent(var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefFirstIdentNoSubs(cref: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, .. } => var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone(),
        Deref @ Absyn::CREF_QUAL { subscripts: Deref @ metamodelica::List::Nil, .. } => var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone(),
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefFirstIdentNoSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefFirstSubs(cref: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    subscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(),
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(),
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => crefFirstSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(subscripts)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefGetLastIdent(cref: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut ident: ArcStr;
    ident = ((::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone(),
        Deref @ Absyn::CREF_QUAL { .. } => crefGetLastIdent(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone())?,
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefGetLastIdent(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(ident)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefGetLastSubs(cref: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    subscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(),
        Deref @ Absyn::CREF_QUAL { .. } => crefGetLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone())?,
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefGetLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(subscripts)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefHasSubscripts(cref: Arc<Absyn::ComponentRef>) -> bool {
    let mut hasSubscripts: bool;
    hasSubscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => !(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone().is_empty()),
        Deref @ Absyn::CREF_QUAL { subscripts: Deref @ metamodelica::List::Nil, .. } => crefHasSubscripts(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone()),
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefHasSubscripts(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone()),
        Deref @ Absyn::WILD => false,
        Deref @ Absyn::ALLWILD => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasSubscripts
}

pub fn crefIdent(cr: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(cr.clone()) {
        Deref @ Absyn::CREF_IDENT { name: __pa0, subscripts: Deref @ metamodelica::List::Nil } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    Ok(r#str)
}

pub fn crefInsertSubscriptLstLst(inExp: Arc<Absyn::Exp>, inLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>) -> Result<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>)> {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>;
    (outExp, outLst) = 'mc: {
        let __mc_input = (inExp.clone(), inLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CREF { componentRef: cref }, subs) => {
                    let mut cref2: Arc<Absyn::ComponentRef>;
                    let mut e: Arc<Absyn::Exp>;
                    cref2 = crefInsertSubscriptLstLst2(cref.clone(), subs.clone())?;
                    Ok((Arc::new(Absyn::Exp::CREF { componentRef: cref2.clone() }), subs.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut cref: Arc<Absyn::ComponentRef>;
                    let mut cref2: Arc<Absyn::ComponentRef>;
                    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>;
                    let mut e: Arc<Absyn::Exp>;
                    Ok((inExp.clone(), inLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outExp, outLst))
}

pub fn crefInsertSubscriptLstLst2(inCref: Arc<Absyn::ComponentRef>, inSubs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = 'mc: {
        let __mc_input = (inCref.clone(), inSubs.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref, Deref @ metamodelica::List::Nil) => {
                    let mut cref2: Arc<Absyn::ComponentRef>;
                    let mut n: ArcStr;
                    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>;
                    let mut s: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    Ok(cref.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CREF_IDENT { name: n, .. }, Deref @ metamodelica::List::Cons { head: s, tail: Deref @ metamodelica::List::Nil }) => {
                    let mut cref: Arc<Absyn::ComponentRef>;
                    let mut cref2: Arc<Absyn::ComponentRef>;
                    let mut subs: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>>;
                    Ok(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (n.clone()).clone(), subscripts: s.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CREF_QUAL { componentRef: cref, name: n, .. }, Deref @ metamodelica::List::Cons { head: s, tail: subs }) => {
                    let mut cref2: Arc<Absyn::ComponentRef>;
                    cref2 = crefInsertSubscriptLstLst2(cref.clone(), subs.clone())?;
                    Ok(Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (n.clone()).clone(), subscripts: s.clone(), componentRef: cref2.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cref }, subs) => {
                    let mut cref2: Arc<Absyn::ComponentRef>;
                    let mut n: ArcStr;
                    let mut s: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
                    cref2 = crefInsertSubscriptLstLst2(cref.clone(), subs.clone())?;
                    Ok(crefMakeFullyQualified(cref2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outCref)
}

pub fn crefIsFullyQualified(inCref: Arc<Absyn::ComponentRef>) -> bool {
    let mut outIsFullyQualified: bool;
    outIsFullyQualified = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsFullyQualified
}

pub fn crefIsIdent(inComponentRef: Arc<Absyn::ComponentRef>) -> bool {
    let mut outIsIdent: bool;
    outIsIdent = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsIdent
}

pub fn crefIsQual(inComponentRef: Arc<Absyn::ComponentRef>) -> bool {
    let mut outIsQual: bool;
    outIsQual = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::CREF_QUAL { .. } => true,
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsQual
}

pub fn crefIsWild(cref: Arc<Absyn::ComponentRef>) -> bool {
    let mut wild: bool;
    wild = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::WILD => true,
        Deref @ Absyn::ALLWILD => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    wild
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefLastIdent(cref: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone(),
        Deref @ Absyn::CREF_QUAL { .. } => crefLastIdent(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone())?,
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefLastIdent(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefLastSubs(cref: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    subscripts = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(),
        Deref @ Absyn::CREF_QUAL { .. } => crefLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone())?,
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(subscripts)
}

pub fn crefMakeFullyQualified(inComponentRef: Arc<Absyn::ComponentRef>) -> Arc<Absyn::ComponentRef> {
    let mut outComponentRef: Arc<Absyn::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => inComponentRef.clone(),
        _ => Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: inComponentRef.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outComponentRef
}

pub fn crefReplaceFirst(cref: Arc<Absyn::ComponentRef>, replacement: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => replacement.clone(),
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => joinCrefs(replacement.clone(), crefStripFirst(cref.clone())?)?,
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: crefReplaceFirst(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), replacement.clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn crefReplaceFirstIdent(icref: Arc<Absyn::ComponentRef>, replPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &((icref.clone(), replPath.clone())) {
        (Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cr }, _) => {
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut cr = (*cr).clone();
            cr = crefReplaceFirstIdent(cr.clone(), replPath.clone())?;
            crefMakeFullyQualified(cr.clone())
        },
        (Deref @ Absyn::CREF_QUAL { subscripts: subs, componentRef: cr, .. }, _) => {
            let mut cref: Arc<Absyn::ComponentRef>;
            cref = pathToCref(replPath.clone())?;
            cref = addSubscriptsLast(cref.clone(), subs.clone())?;
            joinCrefs(cref.clone(), cr.clone())?
        },
        (Deref @ Absyn::CREF_IDENT { subscripts: subs, .. }, _) => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut cref: Arc<Absyn::ComponentRef>;
            cref = pathToCref(replPath.clone())?;
            cref = addSubscriptsLast(cref.clone(), subs.clone())?;
            cref.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefSecondIdent(cref: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut ident: ArcStr;
    ident = ((::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_QUAL { .. } => crefFirstIdent(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone())?,
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => crefSecondIdent(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(ident)
}

pub fn crefSetFirstIdent(cref: Arc<Absyn::ComponentRef>, ident: ArcStr) -> Arc<Absyn::ComponentRef> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; name = ident.clone());
            ()
        },
        Deref @ Absyn::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; name = ident.clone());
            ()
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefSetFirstIdent(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), (ident.clone()).clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn crefSetFirstSubs(cref: Arc<Absyn::ComponentRef>, subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = subscripts.clone());
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; subscripts = subscripts.clone());
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefSetFirstSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), subscripts.clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn crefSetLastSubs(cref: Arc<Absyn::ComponentRef>, inSubscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = inSubscripts.clone());
            ()
        },
        Deref @ Absyn::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; componentRef = crefSetLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), inSubscripts.clone())?);
            ()
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefSetLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), inSubscripts.clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn crefString(inCr: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (Dump::printComponentRefStr(inCr.clone())?).clone();
    Ok(outStr)
}

pub fn crefStringIgnoreSubs(inCr: Arc<Absyn::ComponentRef>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    let mut p: Arc<Absyn::Path>;
    p = crefToPathIgnoreSubs(inCr.clone())?;
    outStr = (pathString(makeNotFullyQualified(p.clone()), (literal!(".")).clone(), true, false)?).clone();
    Ok(outStr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn crefStripFirst(inComponentRef: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outComponentRef: Arc<Absyn::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::CREF_QUAL { componentRef: cr, .. } => cr.clone(),
        Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cr } => crefStripFirst(cr.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn crefStripLast(inCref: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => bail!("fail"),
        Deref @ Absyn::CREF_QUAL { componentRef: Deref @ Absyn::CREF_IDENT { .. }, subscripts: subs, name: r#str } => Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (r#str.clone()).clone(), subscripts: subs.clone() }),
        Deref @ Absyn::CREF_QUAL { componentRef: c, subscripts: subs, name: r#str } => {
            let mut c_1: Arc<Absyn::ComponentRef>;
            c_1 = crefStripLast(c.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (r#str.clone()).clone(), subscripts: subs.clone(), componentRef: c_1.clone() })
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: c } => {
            let mut r#str: ArcStr;
            let mut c_1: Arc<Absyn::ComponentRef>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            c_1 = crefStripLast(c.clone())?;
            crefMakeFullyQualified(c_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outCref)
}

pub fn crefStripLastSubs(cref: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = metamodelica::nil());
            ()
        },
        Deref @ Absyn::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; componentRef = crefStripLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone())?);
            ()
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = crefStripLastSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone())?);
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cref)
}

pub fn crefToPath(inComponentRef: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: i } => Arc::new(Absyn::Path::IDENT { name: (i.clone()).clone() }),
        Deref @ Absyn::CREF_QUAL { componentRef: c, subscripts: Deref @ metamodelica::List::Nil, name: i } => {
            let mut p: Arc<Absyn::Path>;
            p = crefToPath(c.clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (i.clone()).clone(), path: p.clone() })
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: c } => {
            let mut i: ArcStr;
            let mut p: Arc<Absyn::Path>;
            p = crefToPath(c.clone())?;
            Arc::new(Absyn::Path::FULLYQUALIFIED { path: p.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn crefToPathIgnoreSubs(inComponentRef: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inComponentRef.clone()) {
        Deref @ Absyn::CREF_IDENT { name: i, .. } => Arc::new(Absyn::Path::IDENT { name: (i.clone()).clone() }),
        Deref @ Absyn::CREF_QUAL { componentRef: c, name: i, .. } => {
            let mut p: Arc<Absyn::Path>;
            p = crefToPathIgnoreSubs(c.clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (i.clone()).clone(), path: p.clone() })
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: c } => {
            let mut i: ArcStr;
            let mut p: Arc<Absyn::Path>;
            p = crefToPathIgnoreSubs(c.clone())?;
            Arc::new(Absyn::Path::FULLYQUALIFIED { path: p.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn crefToTypeSpec(cref: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::TypeSpec>> {
    let mut ty: Arc<Absyn::TypeSpec>;
    let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    let mut path: Arc<Absyn::Path>;
    subs = crefGetLastSubs(cref.clone())?;
    path = crefToPath(crefStripLastSubs(cref.clone())?)?;
    ty = Arc::new(Absyn::TypeSpec::TPATH { path: path.clone(), arrayDim: if (subs.clone().is_empty()) {None} else {Some(subs.clone())} });
    Ok(ty)
}

pub fn directionEqual(inDirection1: Absyn::Direction, inDirection2: Absyn::Direction) -> bool {
    let mut outEqual: bool;
    outEqual = (match (inDirection1.clone(), inDirection2.clone()) {
        (Absyn::BIDIR, Absyn::BIDIR) => true,
        (Absyn::INPUT, Absyn::INPUT) => true,
        (Absyn::OUTPUT, Absyn::OUTPUT) => true,
        (Absyn::INPUT_OUTPUT, Absyn::INPUT_OUTPUT) => true,
        _ => false,
    });
    outEqual
}

pub static dummyInfo: SourceInfo = SourceInfo { fileName: literal!(""), isReadOnly: false, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0, lastModification: 0.0 };

pub static dummyParts: std::sync::LazyLock<Arc<Absyn::ClassDef>> = std::sync::LazyLock::new(|| { Arc::new(Absyn::ClassDef::PARTS { typeVars: metamodelica::nil(), classAttrs: metamodelica::nil(), classParts: metamodelica::nil(), ann: metamodelica::nil(), comment: None }) });

pub static dummyProgram: std::sync::LazyLock<Absyn::Program> = std::sync::LazyLock::new(|| { Absyn::Program { classes: metamodelica::nil(), within_: openmodelica_ast::Absyn::Within::TOP } });

pub fn dummyTraverseExp<Arg: Clone + 'static>(inExp: Arc<Absyn::Exp>, inArg: Arg) -> (Arc<Absyn::Exp>, Arg) {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outArg: Arg;
    outExp = inExp.clone();
    outArg = inArg.clone();
    (outExp, outArg)
}

pub fn eachBool(eachPrefix: Absyn::Each) -> bool {
    let mut res: bool;
    res = (match eachPrefix.clone() {
        Absyn::Each::EACH { .. } => true,
        _ => false,
    });
    res
}

pub fn elementArgEqual(arg1: Arc<Absyn::ElementArg>, arg2: Arc<Absyn::ElementArg>) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((arg1.clone(), arg2.clone())) {
        (Deref @ Absyn::ElementArg::MODIFICATION { .. }, Deref @ Absyn::ElementArg::MODIFICATION { .. }) => var_field!((*arg1).finalPrefix, Absyn::ElementArg::MODIFICATION).clone() == var_field!((*arg2).finalPrefix, Absyn::ElementArg::MODIFICATION).clone() && var_field!((*arg1).eachPrefix, Absyn::ElementArg::MODIFICATION).clone() == var_field!((*arg2).eachPrefix, Absyn::ElementArg::MODIFICATION).clone() && pathEqual(var_field!((*arg1).path, Absyn::ElementArg::MODIFICATION).clone(), var_field!((*arg2).path, Absyn::ElementArg::MODIFICATION).clone()) && Util::optionEqual(var_field!((*arg1).modification, Absyn::ElementArg::MODIFICATION).clone(), var_field!((*arg2).modification, Absyn::ElementArg::MODIFICATION).clone(), Arc::new(fnptr!(modEqual, Arc<Absyn::Modification>, Arc<Absyn::Modification>))) && Util::optionEqual((var_field!((*arg1).comment, Absyn::ElementArg::MODIFICATION).clone()).clone(), (var_field!((*arg2).comment, Absyn::ElementArg::MODIFICATION).clone()).clone(), Arc::new(fnptr!(stringEq, ArcStr, ArcStr))),
        (Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. }, Deref @ Absyn::ElementArg::ELEMENTARGCOMMENT { .. }) => var_field!((*arg1).comment, Absyn::ElementArg::ELEMENTARGCOMMENT).clone() == var_field!((*arg2).comment, Absyn::ElementArg::ELEMENTARGCOMMENT).clone(),
        (Deref @ Absyn::ElementArg::INHERITANCEBREAK { .. }, Deref @ Absyn::ElementArg::INHERITANCEBREAK { .. }) => equationEqual(var_field!((*arg1).cnct, Absyn::ElementArg::INHERITANCEBREAK).clone(), var_field!((*arg2).cnct, Absyn::ElementArg::INHERITANCEBREAK).clone(), false, true)?,
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynUtil.elementArgEqual")); __mm_s.push_str(&*literal!(" got unknown element.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn elementArgEqualName(inArg1: Arc<Absyn::ElementArg>, inArg2: Arc<Absyn::ElementArg>) -> bool {
    let mut outEqual: bool = pathEqual(elementArgName(inArg1.clone()).unwrap(), elementArgName(inArg2.clone()).unwrap());
    outEqual
}

pub fn elementArgName(inArg: Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::Path>> {
    let mut outName: Arc<Absyn::Path>;
    outName = (::match_deref::match_deref! { match &(inArg.clone()) {
        Deref @ Absyn::MODIFICATION { path: outName, .. } => outName.clone(),
        Deref @ Absyn::REDECLARATION { elementSpec: e, .. } => makeIdentPathFromString((elementSpecName(e.clone())?).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outName)
}

pub fn elementItemClass(item: Arc<Absyn::ElementItem>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class>;
    let __pa0 = ::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: __pa0, .. }, .. } } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cls = __pa0.clone();
    Ok(cls)
}

pub fn elementItemNames(item: Arc<Absyn::ElementItem>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut names: Arc<metamodelica::List<ArcStr>>;
    names = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => elementNames(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone())?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(names)
}

pub fn elementNames(element: Arc<Absyn::Element>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut names: Arc<metamodelica::List<ArcStr>>;
    names = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => elementSpecNames(var_field!((*element).specification, Absyn::Element::ELEMENT).clone())?,
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(names)
}

pub fn elementSpec(el: Arc<Absyn::Element>) -> Result<Arc<Absyn::ElementSpec>> {
    let mut elSpec: Arc<Absyn::ElementSpec>;
    let __pa0 = ::match_deref::match_deref! { match &(el.clone()) {
        Deref @ Absyn::ELEMENT { specification: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    elSpec = __pa0.clone();
    Ok(elSpec)
}

pub fn elementSpecName(inElementSpec: Arc<Absyn::ElementSpec>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &(inElementSpec.clone()) {
        Deref @ Absyn::CLASSDEF { class_: Deref @ Absyn::CLASS { name: n, .. }, .. } => n.clone(),
        Deref @ Absyn::COMPONENTS { components: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::COMPONENTITEM { component: Absyn::COMPONENT { name: n, .. }, .. }, tail: Deref @ metamodelica::List::Nil }, .. } => n.clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

pub fn elementSpecNames(spec: Arc<Absyn::ElementSpec>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut names: Arc<metamodelica::List<ArcStr>>;
    names = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => list![(className(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone())?).clone()],
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for c in (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone()).into_iter().cloned() {
            let __x = componentName(c.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    },
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(names)
}

pub fn elementSpecToPath(inElementSpec: Arc<Absyn::ElementSpec>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(inElementSpec.clone()) {
        Deref @ Absyn::EXTENDS { path: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outPath = __pa0.clone();
    Ok(outPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn eltsHasLocalClass(inElts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Result<bool> {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = inElts.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { .. }, .. } }, tail: _ } => {
                    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: elts } => {
                    Ok(eltsHasLocalClass(elts.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn enumLiteralName(literal: Arc<Absyn::EnumLiteral>) -> ArcStr {
    let mut name: ArcStr = literal.literal.clone();
    name
}

pub fn eqModEqual(eqMod1: Arc<Absyn::EqMod>, eqMod2: Arc<Absyn::EqMod>) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((eqMod1.clone(), eqMod2.clone())) {
        (Deref @ Absyn::EqMod::NOMOD { .. }, Deref @ Absyn::EqMod::NOMOD { .. }) => true,
        (Deref @ Absyn::EqMod::EQMOD { .. }, Deref @ Absyn::EqMod::EQMOD { .. }) => expEqual(var_field!((*eqMod1).exp, Absyn::EqMod::EQMOD).clone(), var_field!((*eqMod2).exp, Absyn::EqMod::EQMOD).clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn equationEqual(eq1: Arc<Absyn::Equation>, eq2: Arc<Absyn::Equation>, shallow: bool, ignoreComment: bool) -> Result<bool> {
    fn branch_eq(branch1: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), branch2: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), shallow: bool, ignoreComment: bool) -> bool {
        let mut equal: bool = expEqual(Util::tuple21(branch1.clone()), Util::tuple21(branch2.clone())).unwrap() && shallow.clone() || equationItemsEqual(Util::tuple22(branch1.clone()), Util::tuple22(branch2.clone()), false, ignoreComment.clone());
        equal
    }

    let mut equal: bool;
    let mut e1: Arc<Absyn::Exp>;
    let mut e2: Arc<Absyn::Exp>;
    let mut eql1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut eql2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut branches: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
    let mut cr1: Arc<Absyn::ComponentRef>;
    let mut cr2: Arc<Absyn::ComponentRef>;
    let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
    let mut args: Arc<Absyn::FunctionArgs>;
    let mut eq: Arc<Absyn::EquationItem>;
    if valueConstructor(eq1.clone()) != valueConstructor(eq2.clone()) {
        equal = false;
        return Ok(equal);
    }
    equal = (::match_deref::match_deref! { match &(eq1.clone()) {
        Deref @ Absyn::Equation::EQ_IF { .. } => {
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_IF { ifExp: __pa0, equationTrueItems: __pa1, elseIfBranches: __pa2, equationElseItems: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            eql1 = __pa1.clone();
            branches = __pa2.clone();
            eql2 = __pa3.clone();
            expEqual(var_field!((*eq1).ifExp, Absyn::Equation::EQ_IF).clone(), e1.clone())? && shallow.clone() || equationItemsEqual(var_field!((*eq1).equationTrueItems, Absyn::Equation::EQ_IF).clone(), eql1.clone(), false, true) && List::isEqualOnTrue(var_field!((*eq1).elseIfBranches, Absyn::Equation::EQ_IF).clone(), branches.clone(), Arc::new({ let __pe_b2 = shallow.clone(); let __pe_b3 = ignoreComment.clone(); move |__pe_a0, __pe_a1| Ok(branch_eq(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())) })) && shallow.clone() || equationItemsEqual(var_field!((*eq1).equationElseItems, Absyn::Equation::EQ_IF).clone(), eql2.clone(), false, ignoreComment.clone())
        },
        Deref @ Absyn::Equation::EQ_EQUALS { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: __pa0, rightSide: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            e2 = __pa1.clone();
            expEqual(var_field!((*eq1).leftSide, Absyn::Equation::EQ_EQUALS).clone(), e1.clone())? && expEqual(var_field!((*eq1).rightSide, Absyn::Equation::EQ_EQUALS).clone(), e2.clone())?
        },
        Deref @ Absyn::Equation::EQ_PDE { .. } => {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_PDE { leftSide: __pa0, rightSide: __pa1, domain: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            e2 = __pa1.clone();
            cr1 = __pa2.clone();
            expEqual(var_field!((*eq1).leftSide, Absyn::Equation::EQ_PDE).clone(), e1.clone())? && expEqual(var_field!((*eq1).rightSide, Absyn::Equation::EQ_PDE).clone(), e2.clone())? && crefEqual(var_field!((*eq1).domain, Absyn::Equation::EQ_PDE).clone(), cr1.clone())
        },
        Deref @ Absyn::Equation::EQ_CONNECT { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_CONNECT { connector1: __pa0, connector2: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr1 = __pa0.clone();
            cr2 = __pa1.clone();
            crefEqual(var_field!((*eq1).connector1, Absyn::Equation::EQ_CONNECT).clone(), cr1.clone()) && crefEqual(var_field!((*eq1).connector2, Absyn::Equation::EQ_CONNECT).clone(), cr2.clone())
        },
        Deref @ Absyn::Equation::EQ_FOR { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_FOR { iterators: __pa0, forEquations: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            iters = __pa0.clone();
            eql1 = __pa1.clone();
            List::isEqualOnTrue(var_field!((*eq1).iterators, Absyn::Equation::EQ_FOR).clone(), iters.clone(), Arc::new(fnptr!(forIteratorEqual, Arc<Absyn::ForIterator>, Arc<Absyn::ForIterator>))) && shallow.clone() || equationItemsEqual(var_field!((*eq1).forEquations, Absyn::Equation::EQ_FOR).clone(), eql1.clone(), false, ignoreComment.clone())
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { .. } => {
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_WHEN_E { whenExp: __pa0, whenEquations: __pa1, elseWhenEquations: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => bail!("pattern mismatch"),
            } };
            e1 = __pa0.clone();
            eql1 = __pa1.clone();
            branches = __pa2.clone();
            expEqual(var_field!((*eq1).whenExp, Absyn::Equation::EQ_WHEN_E).clone(), e1.clone())? && equationItemsEqual(var_field!((*eq1).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), eql1.clone(), false, true) && List::isEqualOnTrue(var_field!((*eq1).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone(), branches.clone(), Arc::new({ let __pe_b2 = shallow.clone(); let __pe_b3 = ignoreComment.clone(); move |__pe_a0, __pe_a1| Ok(branch_eq(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone())) }))
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { .. } => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_NORETCALL { functionName: __pa0, functionArgs: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            cr1 = __pa0.clone();
            args = __pa1.clone();
            crefEqual(var_field!((*eq1).functionName, Absyn::Equation::EQ_NORETCALL).clone(), cr1.clone()) && functionArgsEqual(var_field!((*eq1).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), args.clone())?
        },
        Deref @ Absyn::Equation::EQ_FAILURE { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(eq2.clone()) {
                Deref @ Absyn::Equation::EQ_FAILURE { equ: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            eq = __pa0.clone();
            shallow.clone() || equationItemEqual(var_field!((*eq1).equ, Absyn::Equation::EQ_FAILURE).clone(), eq.clone(), false, ignoreComment.clone())?
        },
        _ => {
            Error::assertion(false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynUtil.equationEqual")); __mm_s.push_str(&*literal!(" got unknown equation.")); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn equationItemEqual(eq1: Arc<Absyn::EquationItem>, eq2: Arc<Absyn::EquationItem>, shallow: bool, ignoreComment: bool) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((eq1.clone(), eq2.clone())) {
        (Deref @ Absyn::EquationItem::EQUATIONITEM { .. }, Deref @ Absyn::EquationItem::EQUATIONITEM { .. }) => equationEqual(var_field!((*eq1).equation_, Absyn::EquationItem::EQUATIONITEM).clone(), var_field!((*eq2).equation_, Absyn::EquationItem::EQUATIONITEM).clone(), shallow.clone(), true)? && ignoreComment.clone() || Util::optionEqual(var_field!((*eq1).comment, Absyn::EquationItem::EQUATIONITEM).clone(), var_field!((*eq2).comment, Absyn::EquationItem::EQUATIONITEM).clone(), Arc::new(fnptr!(commentEqual, Arc<Absyn::Comment>, Arc<Absyn::Comment>))),
        (Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { .. }, Deref @ Absyn::EquationItem::EQUATIONITEMCOMMENT { .. }) => var_field!((*eq1).comment, Absyn::EquationItem::EQUATIONITEMCOMMENT).clone() == var_field!((*eq2).comment, Absyn::EquationItem::EQUATIONITEMCOMMENT).clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn equationItemsEqual(eql1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, eql2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, shallow: bool, ignoreComment: bool) -> bool {
    let mut equal: bool = List::isEqualOnTrue(eql1.clone(), eql2.clone(), Arc::new({ let __pe_b2 = shallow.clone(); let __pe_b3 = ignoreComment.clone(); move |__pe_a0, __pe_a1| equationItemEqual(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }));
    equal
}

pub fn expContainsInitial(inExp: Arc<Absyn::Exp>) -> Result<bool> {
    let mut hasInitial: bool;
    hasInitial = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool;
                    (_, b) = traverseExp(inExp.clone(), Arc::new(fnptr!(isInitialTraverseHelper, Arc<Absyn::Exp>, bool)), false)?;
                    Ok(b.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(hasInitial)
}

pub fn expCref(exp: Arc<Absyn::Exp>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut cr: Arc<Absyn::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::CREF { componentRef: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    cr = __pa0.clone();
    Ok(cr)
}

pub fn expEqual(exp1: Arc<Absyn::Exp>, exp2: Arc<Absyn::Exp>) -> Result<bool> {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((exp1.clone(), exp2.clone())) {
        (Deref @ Absyn::INTEGER { .. }, Deref @ Absyn::REAL { .. }) => realEq(intReal(var_field!((*exp1).value, Absyn::Exp::INTEGER).clone()), stringReal(var_field!((*exp2).value, Absyn::Exp::REAL).clone())?),
        (Deref @ Absyn::REAL { .. }, Deref @ Absyn::INTEGER { .. }) => realEq(intReal(var_field!((*exp2).value, Absyn::Exp::INTEGER).clone()), stringReal(var_field!((*exp1).value, Absyn::Exp::REAL).clone())?),
        _ => exp1.clone() == exp2.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn expString(exp: Arc<Absyn::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::STRING { value: __pa0 } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    r#str = __pa0.clone();
    Ok(r#str)
}

fn filterAnnotationItem(elt: Arc<Absyn::ElementItem>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match &(elt.clone()) {
        Deref @ Absyn::ELEMENTITEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn filterNestedClasses(cl: Arc<Absyn::Class>) -> Arc<Absyn::Class> {
    let mut cl: Arc<Absyn::Class> = cl;
    let mut def: Arc<Absyn::ClassDef>;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::CLASS { body: def @ Deref @ Absyn::PARTS { .. }, .. } => {
            let mut def = (*def).clone();
            let __owned_variant_classParts_0 = List::fold(var_field!((**def).classParts, Absyn::ClassDef::PARTS).clone().reverse(), Arc::new(fnptr!(filterNestedClassesParts, Arc<Absyn::ClassPart>, Arc<metamodelica::List<Arc<Absyn::ClassPart>>>)), metamodelica::nil());
            if let Absyn::ClassDef::PARTS { classParts, .. } = &mut def {
                *classParts = __owned_variant_classParts_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::ClassDef::PARTS"); }
            assign_variant_field!(cl => Absyn::Class::CLASS; body = Arc::new(def.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cl
}

fn filterNestedClassesParts(classPart: Arc<Absyn::ClassPart>, inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
    let mut outClassPart: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    outClassPart = (::match_deref::match_deref! { match &((classPart.clone(), inClassParts.clone())) {
        (Deref @ Absyn::PUBLIC { contents: elts }, classParts) => {
            assign_variant_field!(classPart => Absyn::ClassPart::PUBLIC; contents = List::filterOnFalse(elts.clone(), Arc::new(fnptr!(isElementItemClass, Arc<Absyn::ElementItem>))));
            cons(classPart.clone(), classParts.clone())
        },
        (Deref @ Absyn::PROTECTED { contents: elts }, classParts) => {
            assign_variant_field!(classPart => Absyn::ClassPart::PROTECTED; contents = List::filterOnFalse(elts.clone(), Arc::new(fnptr!(isElementItemClass, Arc<Absyn::ElementItem>))));
            cons(classPart.clone(), classParts.clone())
        },
        _ => cons(classPart.clone(), inClassParts.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outClassPart
}

pub fn findIteratorIndexedCrefs(inExp: Arc<Absyn::Exp>, inIterator: ArcStr, inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Result<Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>;
    (_, outCrefs) = traverseExp(inExp.clone(), Arc::new({ let __pe_b2 = inIterator.clone(); move |__pe_a0, __pe_a1| Ok(findIteratorIndexedCrefs_traverser(__pe_a0, __pe_a1, __pe_b2.clone())) }), metamodelica::nil())?;
    outCrefs = List::fold(outCrefs.clone(), Arc::new({ let __pe_b2 = fnptr!(iteratorIndexedCrefsEqual, (Arc<Absyn::ComponentRef>, i32), (Arc<Absyn::ComponentRef>, i32)); move |__pe_a0, __pe_a1| Ok(List::unionEltOnTrue(__pe_a0, __pe_a1, __pe_b2.clone())) }), inCrefs.clone());
    Ok(outCrefs)
}

fn findIteratorIndexedCrefs_traverser(inExp: Arc<Absyn::Exp>, inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>, inIterator: ArcStr) -> (Arc<Absyn::Exp>, Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) {
    let mut outExp: Arc<Absyn::Exp> = inExp.clone();
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>;
    outCrefs = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::CREF { .. } => getIteratorIndexedCrefs(var_field!((*inExp).componentRef, Absyn::Exp::CREF).clone(), (inIterator.clone()).clone(), inCrefs.clone()),
        _ => inCrefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outCrefs)
}

pub fn forIteratorEqual(iter1: Arc<Absyn::ForIterator>, iter2: Arc<Absyn::ForIterator>) -> bool {
    let mut equal: bool = iter1.name.clone() == iter2.name.clone() && Util::optionEqual(iter1.guardExp.clone(), iter2.guardExp.clone(), Arc::new(expEqual)) && Util::optionEqual(iter1.range.clone(), iter2.range.clone(), Arc::new(expEqual));
    equal
}

pub fn functionArgsEqual(args1: Arc<Absyn::FunctionArgs>, args2: Arc<Absyn::FunctionArgs>) -> Result<bool> {
    fn named_arg_equal(arg1: Arc<Absyn::NamedArg>, arg2: Arc<Absyn::NamedArg>) -> bool {
        let mut equal: bool = arg1.argName.clone() == arg2.argName.clone() && expEqual(arg1.argValue.clone(), arg2.argValue.clone()).unwrap();
        equal
    }

    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((args1.clone(), args2.clone())) {
        (Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. }, Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. }) => List::isEqualOnTrue(var_field!((*args1).args, Absyn::FunctionArgs::FUNCTIONARGS).clone(), var_field!((*args2).args, Absyn::FunctionArgs::FUNCTIONARGS).clone(), Arc::new(expEqual)) && List::isEqualOnTrue(var_field!((*args1).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone(), var_field!((*args2).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone(), Arc::new(fnptr!(named_arg_equal, Arc<Absyn::NamedArg>, Arc<Absyn::NamedArg>))),
        (Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. }, Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. }) => expEqual(var_field!((*args1).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), var_field!((*args2).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone())? && var_field!((*args1).iterType, Absyn::FunctionArgs::FOR_ITER_FARG).clone() == var_field!((*args2).iterType, Absyn::FunctionArgs::FOR_ITER_FARG).clone() && List::isEqualOnTrue(var_field!((*args1).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), var_field!((*args2).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), Arc::new(fnptr!(forIteratorEqual, Arc<Absyn::ForIterator>, Arc<Absyn::ForIterator>))),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equal)
}

pub fn getAnnotationsFromConstraintClass(inCC: Option<Arc<Absyn::ConstrainClass>>) -> Arc<metamodelica::List<Arc<Absyn::ElementArg>>> {
    let mut elementArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    elementArgs = (match inCC.clone() {
        Some(Absyn::CONSTRAINCLASS { comment: Some(Absyn::COMMENT { annotation_: Some(Absyn::ANNOTATION { elementArgs: mut elementArgs }), .. }), .. }) => elementArgs.clone(),
        _ => metamodelica::nil(),
    });
    elementArgs
}

pub fn getAnnotationsFromItems(inComponentItems: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ccAnnotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>>> {
    let mut outLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>>> = metamodelica::nil();
    let mut annotations: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut res: Arc<metamodelica::List<ArcStr>>;
    let mut r#str: ArcStr;
    for comp in &*inComponentItems.clone().reverse() {
        annotations = (::match_deref::match_deref! { match &(comp.clone()) {
        Deref @ Absyn::COMPONENTITEM { comment: Some(Absyn::COMMENT { annotation_: Some(Absyn::ANNOTATION { elementArgs: annotations }), .. }), .. } => listAppend(annotations.clone(), ccAnnotations.clone()),
        _ => ccAnnotations.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outLst = cons(annotations.clone(), outLst.clone());
    }
    outLst
}

pub fn getArrayDimOptAsList(inArrayDim: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outArrayDim: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    outArrayDim = (match inArrayDim.clone() {
        Some(mut ad) => ad.clone(),
        _ => metamodelica::nil(),
    });
    outArrayDim
}

pub fn getClassAnnotation(cls: Arc<Absyn::Class>) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut outAnnotation: Option<Arc<Absyn::Annotation>>;
    outAnnotation = getClassDefAnnotation(cls.body.clone())?;
    Ok(outAnnotation)
}

pub fn getClassDefAnnotation(def: Arc<Absyn::ClassDef>) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut outAnnotation: Option<Arc<Absyn::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(def.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } if (!(var_field!((*def).ann, Absyn::ClassDef::PARTS).clone().is_empty())) => Some(listHead(var_field!((*def).ann, Absyn::ClassDef::PARTS).clone())?),
        Deref @ Absyn::ClassDef::DERIVED { .. } => getCommentOptAnnotation(var_field!((*def).comment, Absyn::ClassDef::DERIVED).clone())?,
        Deref @ Absyn::ClassDef::ENUMERATION { .. } => getCommentOptAnnotation(var_field!((*def).comment, Absyn::ClassDef::ENUMERATION).clone())?,
        Deref @ Absyn::ClassDef::OVERLOAD { .. } => getCommentOptAnnotation(var_field!((*def).comment, Absyn::ClassDef::OVERLOAD).clone())?,
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } if (!(var_field!((*def).ann, Absyn::ClassDef::CLASS_EXTENDS).clone().is_empty())) => Some(listHead(var_field!((*def).ann, Absyn::ClassDef::CLASS_EXTENDS).clone())?),
        Deref @ Absyn::ClassDef::PDER { .. } => getCommentOptAnnotation(var_field!((*def).comment, Absyn::ClassDef::PDER).clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

pub fn getClassName(inClass: Arc<Absyn::Class>) -> Result<ArcStr> {
    let mut outName: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::CLASS { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outName = __pa0.clone();
    Ok(outName)
}

pub fn getClassPartsInClass(cls: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::ClassPart>>> {
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    let mut cdef: Arc<Absyn::ClassDef> = cls.body.clone();
    parts = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(),
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    parts
}

pub fn getCommentOptAnnotation(commentOpt: Option<Arc<Absyn::Comment>>) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut outAnnotation: Option<Arc<Absyn::Annotation>>;
    if isSome(commentOpt.clone()) {
        let Some(Absyn::Comment::COMMENT { annotation_: __pa0, .. }) = (commentOpt.clone()) else { bail!("pattern mismatch") };
        outAnnotation = __pa0.clone();
    } else {
        outAnnotation = None;
    }
    Ok(outAnnotation)
}

pub fn getCommentOptComment(commentOpt: Option<Arc<Absyn::Comment>>) -> Result<Option<ArcStr>> {
    let mut outComment: Option<ArcStr>;
    if isSome(commentOpt.clone()) {
        let Some(Absyn::Comment::COMMENT { comment: __pa0, .. }) = (commentOpt.clone()) else { bail!("pattern mismatch") };
        outComment = __pa0.clone();
    } else {
        outComment = None;
    }
    Ok(outComment)
}

pub fn getComponentItemsAnnotation(items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, name: ArcStr) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut outAnnotation: Option<Arc<Absyn::Annotation>>;
    let mut oi: Option<Arc<Absyn::ComponentItem>>;
    let mut i: Arc<Absyn::ComponentItem>;
    oi = List::findOption(items.clone(), Arc::new({ let __pe_b0 = name.clone(); move |__pe_a1| Ok(isComponentItemNamed(__pe_b0.clone(), __pe_a1)) }));
    if isSome(oi.clone()) {
        let Some(__pa0) = (oi.clone()) else { bail!("pattern mismatch") };
        i = __pa0.clone();
        outAnnotation = getCommentOptAnnotation(i.comment.clone())?;
    } else {
        outAnnotation = None;
    }
    Ok(outAnnotation)
}

pub fn getComponentItemsFromElement(element: Arc<Absyn::Element>) -> Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> {
    let mut items: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
    items = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::COMPONENTS { components: items, .. }, .. } => items.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    items
}

pub fn getComponentItemsFromElementItem(inElementItem: Arc<Absyn::ElementItem>) -> Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> {
    let mut componentItems: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
    componentItems = (match getElementSpecificationFromElementItemOpt(inElementItem.clone()) {
        Some(mut elementSpec) => getComponentItemsFromElementSpec(elementSpec.clone()),
        _ => metamodelica::nil(),
    });
    componentItems
}

pub fn getComponentItemsFromElementSpec(elemSpec: Arc<Absyn::ElementSpec>) -> Arc<metamodelica::List<Arc<Absyn::ComponentItem>>> {
    let mut componentItems: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
    componentItems = (::match_deref::match_deref! { match &(elemSpec.clone()) {
        Deref @ Absyn::COMPONENTS { .. } => var_field!((*elemSpec).components, Absyn::ElementSpec::COMPONENTS).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    componentItems
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getCrefFromExp(inExp: Arc<Absyn::Exp>, includeSubs: bool, includeFunctions: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outComponentRefLst = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::INTEGER { .. } => metamodelica::nil(),
        Deref @ Absyn::REAL { .. } => metamodelica::nil(),
        Deref @ Absyn::STRING { .. } => metamodelica::nil(),
        Deref @ Absyn::BOOL { .. } => metamodelica::nil(),
        Deref @ Absyn::CREF { componentRef: Deref @ Absyn::ALLWILD } => metamodelica::nil(),
        Deref @ Absyn::CREF { componentRef: Deref @ Absyn::WILD } => metamodelica::nil(),
        Deref @ Absyn::CREF { componentRef: cr } if (!(includeSubs.clone())) => list![cr.clone()],
        Deref @ Absyn::CREF { componentRef: cr } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            subs = getSubsFromCref(cr.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l1 = getCrefsFromSubs(subs.clone(), includeSubs.clone(), includeFunctions.clone())?;
            cons(cr.clone(), l1.clone())
        },
        Deref @ Absyn::BINARY { exp2: e2, exp1: e1, .. } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            l1 = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l2 = getCrefFromExp(e2.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = listAppend(l1.clone(), l2.clone());
            res.clone()
        },
        Deref @ Absyn::UNARY { exp: e1, .. } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            res = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res.clone()
        },
        Deref @ Absyn::LBINARY { exp2: e2, exp1: e1, .. } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            l1 = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l2 = getCrefFromExp(e2.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = listAppend(l1.clone(), l2.clone());
            res.clone()
        },
        Deref @ Absyn::LUNARY { exp: e1, .. } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            res = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res.clone()
        },
        Deref @ Absyn::RELATION { exp2: e2, exp1: e1, .. } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            l1 = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l2 = getCrefFromExp(e2.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = listAppend(l1.clone(), l2.clone());
            res.clone()
        },
        Deref @ Absyn::IFEXP { elseBranch: e3, trueBranch: e2, ifExp: e1, .. } => List::flatten(list![getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?, getCrefFromExp(e2.clone(), includeSubs.clone(), includeFunctions.clone())?, getCrefFromExp(e3.clone(), includeSubs.clone(), includeFunctions.clone())?]),
        Deref @ Absyn::CALL { functionArgs: farg, function_: cr, .. } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            res = getCrefFromFarg(farg.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = if (includeFunctions.clone()) {cons(cr.clone(), res.clone())} else {res.clone()};
            res.clone()
        },
        Deref @ Absyn::PARTEVALFUNCTION { functionArgs: farg, function_: cr } => {
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            res = getCrefFromFarg(farg.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = if (includeFunctions.clone()) {cons(cr.clone(), res.clone())} else {res.clone()};
            res.clone()
        },
        Deref @ Absyn::ARRAY { arrayExp: expl } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            lstres1 = List::map2(expl.clone(), Arc::new(getCrefFromExp), includeSubs.clone(), includeFunctions.clone());
            res = List::flatten(lstres1.clone());
            res.clone()
        },
        Deref @ Absyn::MATRIX { matrix: expll } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            res = List::flatten(List::flatten(List::map2List(expll.clone(), Arc::new(getCrefFromExp), includeSubs.clone(), includeFunctions.clone())));
            res.clone()
        },
        Deref @ Absyn::RANGE { stop: e2, step: Some(e3), start: e1 } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            l1 = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l2 = getCrefFromExp(e2.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l2 = listAppend(l1.clone(), l2.clone());
            l1 = getCrefFromExp(e3.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = listAppend(l1.clone(), l2.clone());
            res.clone()
        },
        Deref @ Absyn::RANGE { stop: e2, step: None, start: e1 } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            l1 = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l2 = getCrefFromExp(e2.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = listAppend(l1.clone(), l2.clone());
            res.clone()
        },
        Deref @ Absyn::END => metamodelica::nil(),
        Deref @ Absyn::TUPLE { expressions: expl } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            crefll = List::map2(expl.clone(), Arc::new(getCrefFromExp), includeSubs.clone(), includeFunctions.clone());
            res = List::flatten(crefll.clone());
            res.clone()
        },
        Deref @ Absyn::CODE { .. } => metamodelica::nil(),
        Deref @ Absyn::AS { exp: e1, .. } => getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?,
        Deref @ Absyn::CONS { head: e1, rest: e2 } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            l1 = getCrefFromExp(e1.clone(), includeSubs.clone(), includeFunctions.clone())?;
            l2 = getCrefFromExp(e2.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = listAppend(l1.clone(), l2.clone());
            res.clone()
        },
        Deref @ Absyn::LIST { exps: expl } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            crefll = List::map2(expl.clone(), Arc::new(getCrefFromExp), includeSubs.clone(), includeFunctions.clone());
            res = List::flatten(crefll.clone());
            res.clone()
        },
        Deref @ Absyn::MATCHEXP { .. } => bail!("fail"),
        Deref @ Absyn::DOT { .. } => getCrefFromExp(var_field!((*inExp).exp, Absyn::Exp::DOT).clone(), includeSubs.clone(), includeFunctions.clone())?,
        Deref @ Absyn::EXPRESSIONCOMMENT { .. } => getCrefFromExp(var_field!((*inExp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), includeSubs.clone(), includeFunctions.clone())?,
        Deref @ Absyn::SUBSCRIPTED_EXP { .. } => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            l1 = getCrefFromExp(var_field!((*inExp).exp, Absyn::Exp::SUBSCRIPTED_EXP).clone(), includeSubs.clone(), includeFunctions.clone())?;
            if includeSubs.clone() {
                l2 = getCrefsFromSubs(var_field!((*inExp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone(), includeSubs.clone(), includeFunctions.clone())?;
                l1 = listAppend(l2.clone(), l1.clone());
            }
            l1.clone()
        },
        Deref @ Absyn::BREAK => metamodelica::nil(),
        _ => {
            let mut cr: Arc<Absyn::ComponentRef>;
            let mut l1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut l2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut op: Absyn::Operator;
            let mut e4: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut farg: Arc<Absyn::FunctionArgs>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut lstres1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut crefll: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            Error::addInternalError(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AbsynUtil.getCrefFromExp")); __mm_s.push_str(&*literal!(" failed ")); __mm_s.push_str(&*Dump::printExpStr(inExp.clone())?); ArcStr::from(__mm_s) }).clone(), metamodelica::sourceInfo!())?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outComponentRefLst)
}

pub fn getCrefFromFarg(inFunctionArgs: Arc<Absyn::FunctionArgs>, includeSubs: bool, includeFunctions: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outComponentRefLst = (::match_deref::match_deref! { match &((inFunctionArgs.clone(), includeSubs.clone(), includeFunctions.clone())) {
        (Deref @ Absyn::FUNCTIONARGS { argNames: nargl, args: expl }, _, _) => {
            let mut l1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut l2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut fl1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut fl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut fl3: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut exp: Arc<Absyn::Exp>;
            l1 = List::map2(expl.clone(), Arc::new(getCrefFromExp), includeSubs.clone(), includeFunctions.clone());
            fl1 = List::flatten(l1.clone());
            l2 = List::map2(nargl.clone(), Arc::new(getCrefFromNarg), includeSubs.clone(), includeFunctions.clone());
            fl2 = List::flatten(l2.clone());
            res = listAppend(fl1.clone(), fl2.clone());
            res.clone()
        },
        (Deref @ Absyn::FOR_ITER_FARG { exp, iterType: _, iterators }, _, _) => {
            let mut l1: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut l2: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>>>;
            let mut fl1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut fl2: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut fl3: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut res: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            let mut expl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut nargl: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            l1 = List::map2Option(List::map(iterators.clone(), Arc::new(iteratorRange)), Arc::new(getCrefFromExp), includeSubs.clone(), includeFunctions.clone())?;
            l2 = List::map2Option(List::map(iterators.clone(), Arc::new(iteratorGuard)), Arc::new(getCrefFromExp), includeSubs.clone(), includeFunctions.clone())?;
            fl1 = List::flatten(l1.clone());
            fl2 = List::flatten(l2.clone());
            fl3 = getCrefFromExp(exp.clone(), includeSubs.clone(), includeFunctions.clone())?;
            res = listAppend(fl1.clone(), listAppend(fl2.clone(), fl3.clone()));
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRefLst)
}

fn getCrefFromNarg(inNamedArg: Arc<Absyn::NamedArg>, includeSubs: bool, includeFunctions: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outComponentRefLst = getCrefFromExp(inNamedArg.argValue.clone(), includeSubs.clone(), includeFunctions.clone())?;
    Ok(outComponentRefLst)
}

pub fn getCrefsFromSubs(isubs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, includeSubs: bool, includeFunctions: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut crefs: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    crefs = (::match_deref::match_deref! { match &((isubs.clone(), includeSubs.clone(), includeFunctions.clone())) {
        (Deref @ metamodelica::List::Nil, _, _) => metamodelica::nil(),
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NOSUB, tail: subs }, _, _) => getCrefsFromSubs(subs.clone(), includeSubs.clone(), includeFunctions.clone())?,
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::SUBSCRIPT { subscript: exp }, tail: subs }, _, _) => {
            let mut crefs1: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
            crefs1 = getCrefsFromSubs(subs.clone(), includeSubs.clone(), includeFunctions.clone())?;
            crefs = getCrefFromExp(exp.clone(), includeSubs.clone(), includeFunctions.clone())?;
            listAppend(crefs.clone(), crefs1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(crefs)
}

pub fn getDefineUnitsInElements(elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Arc<metamodelica::List<Arc<Absyn::Element>>> {
    let mut outElts: Arc<metamodelica::List<Arc<Absyn::Element>>> = metamodelica::nil();
    for i in &*elts.clone() {
        outElts = (::match_deref::match_deref! { match &(i.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::DEFINEUNIT { .. } } => cons(var_field!(i.element, Absyn::ElementItem::ELEMENTITEM).clone(), outElts.clone()),
        _ => outElts.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    outElts = outElts.clone().reverse();
    outElts
}

pub fn getDirection(elementItem: Arc<Absyn::ElementItem>) -> Absyn::Direction {
    let mut oDirection: Absyn::Direction;
    oDirection = (::match_deref::match_deref! { match &(elementItem.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::COMPONENTS { attributes: Absyn::ATTR { direction: oDirection, .. }, .. }, .. } } => oDirection.clone(),
        _ => openmodelica_ast::Absyn::Direction::BIDIR,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oDirection
}

pub fn getElementAnnotation(element: Arc<Absyn::Element>, name: ArcStr) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut outAnnotation: Option<Arc<Absyn::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => getElementSpecAnnotation(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), (name.clone()).clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

pub fn getElementConstrainingClass(element: Arc<Absyn::Element>) -> Option<Arc<Absyn::ConstrainClass>> {
    let mut cc: Option<Arc<Absyn::ConstrainClass>>;
    cc = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone(),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cc
}

pub fn getElementItemsInClass(inClass: Arc<Absyn::Class>) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = getElementItemsInClassDef(inClass.body.clone());
    outElements
}

pub fn getElementItemsInClassDef(classDef: Arc<Absyn::ClassDef>) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    outElements = (::match_deref::match_deref! { match &(classDef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => List::mapFlat(var_field!((*classDef).classParts, Absyn::ClassDef::PARTS).clone(), Arc::new(fnptr!(getElementItemsInClassPart, Arc<Absyn::ClassPart>))),
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => List::mapFlat(var_field!((*classDef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), Arc::new(fnptr!(getElementItemsInClassPart, Arc<Absyn::ClassPart>))),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

pub fn getElementItemsInClassPart(inClassPart: Arc<Absyn::ClassPart>) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    outElements = (::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::PUBLIC { .. } => var_field!((*inClassPart).contents, Absyn::ClassPart::PUBLIC).clone(),
        Deref @ Absyn::PROTECTED { .. } => var_field!((*inClassPart).contents, Absyn::ClassPart::PROTECTED).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

pub fn getElementItemsInElement(element: Arc<Absyn::Element>) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut outElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut cls: Arc<Absyn::Class>;
    outElements = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { specification: Deref @ Absyn::ElementSpec::CLASSDEF { class_: cls, .. }, .. } => getElementItemsInClass(cls.clone()),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElements
}

pub fn getElementSpecAnnotation(spec: Arc<Absyn::ElementSpec>, name: ArcStr) -> Result<Option<Arc<Absyn::Annotation>>> {
    let mut outAnnotation: Option<Arc<Absyn::Annotation>>;
    outAnnotation = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => getClassAnnotation(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone())?,
        Deref @ Absyn::ElementSpec::EXTENDS { .. } => var_field!((*spec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone(),
        Deref @ Absyn::ElementSpec::IMPORT { .. } => getCommentOptAnnotation(var_field!((*spec).comment, Absyn::ElementSpec::IMPORT).clone())?,
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => getComponentItemsAnnotation(var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone(), (name.clone()).clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

pub fn getElementSpecificationFromElementItemOpt(inElementItem: Arc<Absyn::ElementItem>) -> Option<Arc<Absyn::ElementSpec>> {
    let mut outSpec: Option<Arc<Absyn::ElementSpec>>;
    outSpec = (::match_deref::match_deref! { match &(inElementItem.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: spec, .. } } => Some(spec.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSpec
}

pub fn getEquationItemsInPart(part: Arc<Absyn::ClassPart>) -> Arc<metamodelica::List<Arc<Absyn::EquationItem>>> {
    let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    eqs = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone(),
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eqs
}

pub fn getExpsFromArrayDim(inAd: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut hasUnknownDimensions: bool;
    let mut outExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    (hasUnknownDimensions, outExps) = getExpsFromArrayDim_tail(inAd.clone(), metamodelica::nil())?;
    Ok((hasUnknownDimensions, outExps))
}

pub fn getExpsFromArrayDimOpt(inAdO: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut hasUnknownDimensions: bool;
    let mut outExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    (hasUnknownDimensions, outExps) = (match inAdO.clone() {
        None => (false, metamodelica::nil()),
        Some(mut ad) => getExpsFromArrayDim_tail(ad.clone(), metamodelica::nil())?,
    });
    Ok((hasUnknownDimensions, outExps))
}

pub fn getExpsFromArrayDim_tail(inAd: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, inAccumulator: Arc<metamodelica::List<Arc<Absyn::Exp>>>) -> Result<(bool, Arc<metamodelica::List<Arc<Absyn::Exp>>>)> {
    let mut hasUnknownDimensions: bool;
    let mut outExps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    (hasUnknownDimensions, outExps) = (::match_deref::match_deref! { match &((inAd.clone(), inAccumulator.clone())) {
        (Deref @ metamodelica::List::Nil, acc) => (false, acc.clone().reverse()),
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::SUBSCRIPT { subscript: e }, tail: rest }, acc) => {
            let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut b: bool;
            (b, exps) = getExpsFromArrayDim_tail(rest.clone(), cons(e.clone(), acc.clone()))?;
            (b.clone(), exps.clone())
        },
        (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::NOSUB, tail: rest }, acc) => {
            let mut e: Arc<Absyn::Exp>;
            let mut exps: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut b: bool;
            (_, exps) = getExpsFromArrayDim_tail(rest.clone(), acc.clone())?;
            (true, exps.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((hasUnknownDimensions, outExps))
}

pub fn getExternalDecl(inCls: Arc<Absyn::Class>) -> Result<Arc<Absyn::ClassPart>> {
    let mut outExternal: Arc<Absyn::ClassPart>;
    let mut cp: Arc<Absyn::ClassPart>;
    let mut class_parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    let __pa0 = ::match_deref::match_deref! { match &(inCls.clone()) {
        Deref @ Absyn::CLASS { body: Deref @ Absyn::PARTS { classParts: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    class_parts = __pa0.clone();
    outExternal = List::find(class_parts.clone(), Arc::new(fnptr!(isExternalPart, Arc<Absyn::ClassPart>)))?;
    Ok(outExternal)
}

pub fn getFileNameFromInfo(inInfo: SourceInfo) -> Result<ArcStr> {
    let mut inFileName: ArcStr;
    let SourceInfo { fileName: __pa0, .. } = (inInfo.clone()) else { bail!("pattern mismatch") };
    inFileName = __pa0.clone();
    Ok(inFileName)
}

pub fn getFunctionInterface(cl: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut cl: Arc<Absyn::Class> = cl;
    let mut def: Arc<Absyn::ClassDef>;
    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::CLASS { body: def @ Deref @ Absyn::PARTS { .. }, restriction: Absyn::R_FUNCTION { .. }, .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(List::fold(var_field!((**def).classParts, Absyn::ClassDef::PARTS).clone().reverse(), Arc::new(fnptr!(getFunctionInterfaceParts, Arc<Absyn::ClassPart>, Arc<metamodelica::List<Arc<Absyn::ElementItem>>>)), metamodelica::nil())) {
                __pa0 @ Deref @ metamodelica::List::Cons { head: _, tail: _ } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            elts = __pa0.clone();
            assign_variant_field!(cl => Absyn::Class::CLASS;
                body = Arc::new(Absyn::ClassDef::PARTS { typeVars: var_field!((**def).typeVars, Absyn::ClassDef::PARTS).clone(), classAttrs: var_field!((**def).classAttrs, Absyn::ClassDef::PARTS).clone(), classParts: list![Arc::new(Absyn::ClassPart::PUBLIC { contents: elts.clone() })], ann: metamodelica::nil(), comment: None }),
                commentsBeforeEnd = metamodelica::nil(),
                commentsAfterEnd = metamodelica::nil()
            );
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

fn getFunctionInterfaceParts(part: Arc<Absyn::ClassPart>, elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>) -> Arc<metamodelica::List<Arc<Absyn::ElementItem>>> {
    let mut oelts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    oelts = (::match_deref::match_deref! { match &((part.clone(), elts.clone())) {
        (Deref @ Absyn::PUBLIC { contents: elts1 }, elts2) => {
            let mut elts1 = (*elts1).clone();
            elts1 = List::filterOnTrue(elts1.clone(), Arc::new(fnptr!(filterAnnotationItem, Arc<Absyn::ElementItem>)));
            listAppend(elts1.clone(), elts2.clone())
        },
        _ => elts.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    oelts
}

fn getIteratorIndexedCrefs(inCref: Arc<Absyn::ComponentRef>, inIterator: ArcStr, inCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>) -> Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> {
    let mut outCrefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>> = inCrefs.clone();
    let mut crefs: Arc<metamodelica::List<(Arc<Absyn::ComponentRef>, i32)>>;
    outCrefs = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::CREF_IDENT { subscripts: subs, name: id } => {
            let mut idx: i32;
            let mut name: ArcStr;
            let mut cref: Arc<Absyn::ComponentRef>;
            idx = 1;
            for sub in &*subs.clone() {
                let _ = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::SUBSCRIPT { subscript: Deref @ Absyn::CREF { componentRef: Deref @ Absyn::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name } } } => {
            if name.clone() == inIterator.clone() {
                outCrefs = cons((Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: metamodelica::nil() }), idx.clone()), outCrefs.clone());
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                idx = idx.clone() + 1;
            }
            outCrefs.clone()
        },
        Deref @ Absyn::CREF_QUAL { componentRef: cref, subscripts: subs, name: id } => {
            let mut idx: i32;
            let mut name: ArcStr;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), (inIterator.clone()).clone(), metamodelica::nil());
            for cr in &*crefs.clone() {
                (cref, idx) = cr.clone();
                outCrefs = cons((Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id.clone()).clone(), subscripts: subs.clone(), componentRef: cref.clone() }), idx.clone()), outCrefs.clone());
            }
            getIteratorIndexedCrefs(Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (id.clone()).clone(), subscripts: subs.clone() }), (inIterator.clone()).clone(), outCrefs.clone())
        },
        Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut idx: i32;
            let mut name: ArcStr;
            let mut id: ArcStr;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), (inIterator.clone()).clone(), metamodelica::nil());
            for cr in &*crefs.clone() {
                (cref, idx) = cr.clone();
                outCrefs = cons((Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref.clone() }), idx.clone()), outCrefs.clone());
            }
            outCrefs.clone()
        },
        _ => inCrefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCrefs
}

pub fn getNamedAnnotationInClass<T: Clone + 'static>(inClass: Arc<Absyn::Class>, id: Arc<Absyn::Path>, f: Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<T> + 'static>) -> Result<Option<T>> {
    pub type ModFunc<T: Clone> = fn(Option<Arc<Absyn::Modification>>) -> Result<T>;

    let mut outString: Option<T>;
    outString = 'mc: {
        let __mc_input = inClass.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::CLASS { body: Deref @ Absyn::PARTS { ann, .. }, .. } => {
                    let mut r#str: T;
                    let mut res: T;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut annlst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    annlst = List::flatten(List::map(ann.clone(), Arc::new(annotationToElementArgs)));
                    Ok(getNamedAnnotationStr(annlst.clone(), id.clone(), f.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::CLASS { body: Deref @ Absyn::CLASS_EXTENDS { ann, .. }, .. } => {
                    let mut r#str: T;
                    let mut res: T;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut annlst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    annlst = List::flatten(List::map(ann.clone(), Arc::new(annotationToElementArgs)));
                    Ok(getNamedAnnotationStr(annlst.clone(), id.clone(), f.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::CLASS { body: Deref @ Absyn::DERIVED { comment: Some(Absyn::COMMENT { annotation_: Some(Absyn::ANNOTATION { elementArgs: annlst }), comment: _ }), .. }, .. } => {
                    let mut r#str: T;
                    let mut res: T;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
                    Ok(getNamedAnnotationStr(annlst.clone(), id.clone(), f.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::CLASS { body: Deref @ Absyn::ENUMERATION { comment: Some(Absyn::COMMENT { annotation_: Some(Absyn::ANNOTATION { elementArgs: annlst }), comment: _ }), .. }, .. } => {
                    let mut r#str: T;
                    let mut res: T;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
                    Ok(getNamedAnnotationStr(annlst.clone(), id.clone(), f.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::CLASS { body: Deref @ Absyn::OVERLOAD { comment: Some(Absyn::COMMENT { annotation_: Some(Absyn::ANNOTATION { elementArgs: annlst }), comment: _ }), .. }, .. } => {
                    let mut r#str: T;
                    let mut res: T;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
                    Ok(getNamedAnnotationStr(annlst.clone(), id.clone(), f.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: T;
                    let mut res: T;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut annlst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut ann: Arc<metamodelica::List<Arc<Absyn::Annotation>>>;
                    Ok(None)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getNamedAnnotationStr<T: Clone + 'static>(inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, id: Arc<Absyn::Path>, f: Arc<dyn ::std::ops::Fn(Option<Arc<Absyn::Modification>>) -> Result<T> + 'static>) -> Result<Option<T>> {
    pub type ModFunc<T: Clone> = fn(Option<Arc<Absyn::Modification>>) -> Result<T>;

    let mut outString: Option<T>;
    outString = 'mc: {
        let __mc_input = (inAbsynElementArgLst.clone(), id.clone(), f.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::MODIFICATION { modification: r#mod, path: Deref @ Absyn::IDENT { name: id1 }, .. }, tail: _ }, Deref @ Absyn::IDENT { name: id2 }, _) => {
                    let mut r#str: T;
                    let mut ann: Arc<Absyn::ElementArg>;
                    let mut xs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut rest: Arc<Absyn::Path>;
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    r#str = f(r#mod.clone())?;
                    Ok(Some(r#str.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: Deref @ Absyn::MODIFICATION { modification: Some(Absyn::CLASSMOD { elementArgLst: xs, .. }), path: Deref @ Absyn::IDENT { name: id1 }, .. }, tail: _ }, Deref @ Absyn::QUALIFIED { path: rest, name: id2 }, _) => {
                    let mut r#str: T;
                    let mut ann: Arc<Absyn::ElementArg>;
                    let mut r#mod: Option<Arc<Absyn::Modification>>;
                    let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(getNamedAnnotationStr(xs.clone(), rest.clone(), f.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: xs }, _, _) => {
                    let mut r#str: T;
                    let mut ann: Arc<Absyn::ElementArg>;
                    let mut r#mod: Option<Arc<Absyn::Modification>>;
                    let mut id1: ArcStr;
                    let mut id2: ArcStr;
                    let mut rest: Arc<Absyn::Path>;
                    Ok(getNamedAnnotationStr(xs.clone(), id.clone(), f.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outString)
}

pub fn getNamedFuncArgNamesAndValues(namedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> (Arc<metamodelica::List<ArcStr>>, Arc<metamodelica::List<Arc<Absyn::Exp>>>) {
    let mut names: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut values: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
    for arg in &*namedArgs.clone().reverse() {
        names = cons(arg.argName.clone(), names.clone());
        values = cons(arg.argValue.clone(), values.clone());
    }
    (names, values)
}

pub fn getShortClass(cl: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut cl: Arc<Absyn::Class> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::CLASS { body: Deref @ Absyn::PARTS { .. }, .. } => bail!("fail"),
        Deref @ Absyn::CLASS { body: Deref @ Absyn::CLASS_EXTENDS { .. }, .. } => bail!("fail"),
        Deref @ Absyn::CLASS { .. } => {
            assign_variant_field!(cl => Absyn::Class::CLASS; body = stripClassDefComment(cl.body.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cl)
}

pub fn getString(exp: Arc<Absyn::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::EXPRESSIONCOMMENT { .. } => getString(var_field!((*exp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone())?,
        Deref @ Absyn::STRING { value: r#str } => r#str.clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(r#str)
}

pub fn getSubsFromCref(cr: Arc<Absyn::ComponentRef>, includeSubs: bool, includeFunctions: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::Subscript>>>> {
    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    subscripts = (::match_deref::match_deref! { match &((cr.clone(), includeSubs.clone(), includeFunctions.clone())) {
        (Deref @ Absyn::CREF_IDENT { name: _, subscripts: subs2 }, _, _) => subs2.clone(),
        (Deref @ Absyn::CREF_QUAL { name: _, subscripts: subs2, componentRef: child }, _, _) => {
            subscripts = getSubsFromCref(child.clone(), includeSubs.clone(), includeFunctions.clone())?;
            subscripts = List::unionOnTrue(subscripts.clone(), subs2.clone(), Arc::new(subscriptEqual));
            subscripts.clone()
        },
        (Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: child }, _, _) => {
            let mut subs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            subscripts = getSubsFromCref(child.clone(), includeSubs.clone(), includeFunctions.clone())?;
            subscripts.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(subscripts)
}

pub fn getTypeSpecFromElementItemOpt(inElementItem: Arc<Absyn::ElementItem>) -> Option<Arc<Absyn::TypeSpec>> {
    let mut outTypeSpec: Option<Arc<Absyn::TypeSpec>>;
    outTypeSpec = (::match_deref::match_deref! { match &(inElementItem.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::COMPONENTS { typeSpec: ty_spec, .. }, .. } } => Some(ty_spec.clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outTypeSpec
}

pub fn importEqual(im1: Absyn::Import, im2: Absyn::Import) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match (im1.clone(), im2.clone()) {
        (Absyn::NAMED_IMPORT { .. }, Absyn::NAMED_IMPORT { .. }) => stringEq((var_field!(im1.name, Absyn::Import::NAMED_IMPORT).clone()).clone(), (var_field!(im2.name, Absyn::Import::NAMED_IMPORT).clone()).clone()) && pathEqual(var_field!(im1.path, Absyn::Import::NAMED_IMPORT).clone(), var_field!(im2.path, Absyn::Import::NAMED_IMPORT).clone()),
        (Absyn::QUAL_IMPORT { .. }, Absyn::QUAL_IMPORT { .. }) => pathEqual(var_field!(im1.path, Absyn::Import::QUAL_IMPORT).clone(), var_field!(im2.path, Absyn::Import::QUAL_IMPORT).clone()),
        (Absyn::UNQUAL_IMPORT { .. }, Absyn::UNQUAL_IMPORT { .. }) => pathEqual(var_field!(im1.path, Absyn::Import::UNQUAL_IMPORT).clone(), var_field!(im2.path, Absyn::Import::UNQUAL_IMPORT).clone()),
        _ => false,
    });
    outBoolean
}

pub fn importName(inImport: Absyn::Import) -> Result<ArcStr> {
    let mut outName: ArcStr;
    outName = ((match inImport.clone() {
        Absyn::NAMED_IMPORT { .. } => var_field!(inImport.name, Absyn::Import::NAMED_IMPORT).clone(),
        Absyn::QUAL_IMPORT { .. } => pathLastIdent(var_field!(inImport.path, Absyn::Import::QUAL_IMPORT).clone())?,
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outName)
}

pub fn importPath(inImport: Absyn::Import) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (match inImport.clone() {
        Absyn::NAMED_IMPORT { path: mut path, .. } => path.clone(),
        Absyn::QUAL_IMPORT { path: mut path } => path.clone(),
        Absyn::UNQUAL_IMPORT { path: mut path } => path.clone(),
        Absyn::GROUP_IMPORT { prefix: ref path, .. } => path.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(outPath)
}

pub fn importString(inImp: Absyn::Import) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (Dump::unparseImportStr(inImp.clone())?).clone();
    Ok(outStr)
}

pub fn innerOuterEqual(io1: Absyn::InnerOuter, io2: Absyn::InnerOuter) -> bool {
    let mut res: bool;
    res = (match (io1.clone(), io2.clone()) {
        (Absyn::INNER, Absyn::INNER) => true,
        (Absyn::OUTER, Absyn::OUTER) => true,
        (Absyn::INNER_OUTER, Absyn::INNER_OUTER) => true,
        (Absyn::NOT_INNER_OUTER, Absyn::NOT_INNER_OUTER) => true,
        _ => false,
    });
    res
}

pub fn innerOuterStr(io: Absyn::InnerOuter) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match io.clone() {
        Absyn::INNER_OUTER => literal!("inner outer"),
        Absyn::INNER => literal!("inner"),
        Absyn::OUTER => literal!("outer"),
        Absyn::NOT_INNER_OUTER => literal!(""),
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

pub fn isAlgorithmItem(inAlg: Arc<Absyn::AlgorithmItem>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (::match_deref::match_deref! { match &(inAlg.clone()) {
        Deref @ Absyn::ALGORITHMITEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

pub fn isAlgorithmSection(part: Arc<Absyn::ClassPart>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::ALGORITHMS { .. } => true,
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isBlock(cls: Arc<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class::CLASS { restriction: Absyn::Restriction::R_BLOCK { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isClassNamed(inName: ArcStr, inClass: Arc<Absyn::Class>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::CLASS { .. } => inName.clone() == inClass.name.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

pub fn isClassOrComponentElementSpec(inElementSpec: Arc<Absyn::ElementSpec>) -> bool {
    let mut yes: bool = false;
    yes = (::match_deref::match_deref! { match &(inElementSpec.clone()) {
        Deref @ Absyn::CLASSDEF { class_: Deref @ Absyn::CLASS { .. }, .. } => true,
        Deref @ Absyn::COMPONENTS { components: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::COMPONENTITEM { .. }, tail: Deref @ metamodelica::List::Nil }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    yes
}

pub fn isClassdef(inElement: Arc<Absyn::Element>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isComponentItemNamed(name: ArcStr, component: Arc<Absyn::ComponentItem>) -> bool {
    let mut res: bool = isComponentNamed((name.clone()).clone(), component.component.clone());
    res
}

pub fn isComponentNamed(name: ArcStr, component: Absyn::Component) -> bool {
    let mut res: bool = name.clone() == component.name.clone();
    res
}

pub fn isConnector(cls: Arc<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class::CLASS { restriction: Absyn::Restriction::R_CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isCref(exp: Arc<Absyn::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::CREF { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isDerCref(exp: Arc<Absyn::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::CALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil }, function_: Deref @ Absyn::CREF_IDENT { name: Deref @ "der", subscripts: Deref @ metamodelica::List::Nil }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isDerCrefFail(exp: Arc<Absyn::Exp>) -> Result<()> {
    ::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::CALL { functionArgs: Deref @ Absyn::FUNCTIONARGS { args: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::CREF { .. }, tail: Deref @ metamodelica::List::Nil }, argNames: Deref @ metamodelica::List::Nil }, function_: Deref @ Absyn::CREF_IDENT { name: Deref @ "der", subscripts: Deref @ metamodelica::List::Nil }, .. } => (),
        _ => bail!("pattern mismatch"),
    } };
    Ok(())
}

pub fn isElementItem(inElement: Arc<Absyn::ElementItem>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::ELEMENTITEM { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

pub fn isElementItemClass(inElement: Arc<Absyn::ElementItem>) -> bool {
    let mut outIsClass: bool;
    outIsClass = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { .. }, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsClass
}

pub fn isElementItemClassNamed(inName: ArcStr, inElement: Arc<Absyn::ElementItem>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::CLASSDEF { class_: Deref @ Absyn::CLASS { name, .. }, .. }, .. } } => name.clone() == inName.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsNamed
}

pub fn isElementItemExtends(item: Arc<Absyn::ElementItem>) -> bool {
    let mut isExtends: bool;
    isExtends = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { specification: Deref @ Absyn::EXTENDS { .. }, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isExtends
}

pub fn isElementItemNamed(name: ArcStr, element: Arc<Absyn::ElementItem>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::ELEMENTITEM { .. } => isElementNamed((name.clone()).clone(), var_field!((*element).element, Absyn::ElementItem::ELEMENTITEM).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isElementNamed(name: ArcStr, element: Arc<Absyn::Element>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => isElementSpecNamed((name.clone()).clone(), var_field!((*element).specification, Absyn::Element::ELEMENT).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isElementRedeclare(element: Arc<Absyn::Element>) -> bool {
    let mut res: bool;
    let mut redecl: Absyn::RedeclareKeywords;
    res = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { redeclareKeywords: Some(redecl), .. } => (match redecl.clone() {
        Absyn::RedeclareKeywords::REDECLARE { .. } => true,
        _ => false,
    }),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isElementReplaceable(element: Arc<Absyn::Element>) -> bool {
    let mut res: bool;
    let mut redecl: Absyn::RedeclareKeywords;
    res = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { redeclareKeywords: Some(redecl), .. } => (match redecl.clone() {
        Absyn::RedeclareKeywords::REPLACEABLE { .. } => true,
        Absyn::RedeclareKeywords::REDECLARE_REPLACEABLE { .. } => true,
        _ => false,
    }),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isElementSection(part: Arc<Absyn::ClassPart>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => true,
        Deref @ Absyn::ClassPart::PROTECTED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isElementSpecNamed(name: ArcStr, elementSpec: Arc<Absyn::ElementSpec>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(elementSpec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => isClassNamed((name.clone()).clone(), var_field!((*elementSpec).class_, Absyn::ElementSpec::CLASSDEF).clone()),
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => List::any(var_field!((*elementSpec).components, Absyn::ElementSpec::COMPONENTS).clone(), Arc::new({ let __pe_b0 = name.clone(); move |__pe_a1| Ok(isComponentItemNamed(__pe_b0.clone(), __pe_a1)) })),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isEmptyClassPart(inClassPart: Arc<Absyn::ClassPart>) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::PUBLIC { contents: Deref @ metamodelica::List::Nil } => true,
        Deref @ Absyn::PROTECTED { contents: Deref @ metamodelica::List::Nil } => true,
        Deref @ Absyn::CONSTRAINTS { contents: Deref @ metamodelica::List::Nil } => true,
        Deref @ Absyn::EQUATIONS { contents: Deref @ metamodelica::List::Nil } => true,
        Deref @ Absyn::INITIALEQUATIONS { contents: Deref @ metamodelica::List::Nil } => true,
        Deref @ Absyn::ALGORITHMS { contents: Deref @ metamodelica::List::Nil } => true,
        Deref @ Absyn::INITIALALGORITHMS { contents: Deref @ metamodelica::List::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn isEmptyEqMod(eqMod: Arc<Absyn::EqMod>) -> bool {
    let mut isEmpty: bool;
    isEmpty = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::EqMod::NOMOD { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn isEmptyMod(inMod: Arc<Absyn::Modification>) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (::match_deref::match_deref! { match &(inMod.clone()) {
        Deref @ Absyn::CLASSMOD { elementArgLst: Deref @ metamodelica::List::Nil, eqMod: Deref @ Absyn::NOMOD } => true,
        Deref @ Absyn::CLASSMOD { elementArgLst: Deref @ metamodelica::List::Nil, eqMod: Deref @ Absyn::EQMOD { exp: Deref @ Absyn::TUPLE { expressions: Deref @ metamodelica::List::Nil }, .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn isEmptySubMod(inSubMod: Arc<Absyn::ElementArg>) -> bool {
    let mut outIsEmpty: bool;
    outIsEmpty = (::match_deref::match_deref! { match &(inSubMod.clone()) {
        Deref @ Absyn::MODIFICATION { finalPrefix: true, .. } => false,
        Deref @ Absyn::MODIFICATION { modification: None, .. } => true,
        Deref @ Absyn::MODIFICATION { modification: Some(r#mod), .. } => isEmptyMod(r#mod.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsEmpty
}

pub fn isEquationSection(part: Arc<Absyn::ClassPart>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => true,
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isExpandableConnector(cls: Arc<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class::CLASS { restriction: Absyn::Restriction::R_EXP_CONNECTOR { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isExternalPart(inClassPart: Arc<Absyn::ClassPart>) -> bool {
    let mut outFound: bool;
    outFound = (::match_deref::match_deref! { match &(inClassPart.clone()) {
        Deref @ Absyn::EXTERNAL { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outFound
}

pub fn isFieldEqual(isField1: Absyn::IsField, isField2: Absyn::IsField) -> bool {
    let mut outEqual: bool;
    outEqual = (match (isField1.clone(), isField2.clone()) {
        (Absyn::NONFIELD, Absyn::NONFIELD) => true,
        (Absyn::FIELD, Absyn::FIELD) => true,
        _ => false,
    });
    outEqual
}

pub fn isFunctionRestriction(inRestriction: Absyn::Restriction) -> bool {
    let mut outIsFunction: bool;
    outIsFunction = (match inRestriction.clone() {
        Absyn::R_FUNCTION { .. } => true,
        _ => false,
    });
    outIsFunction
}

pub fn isImpure(purity: Absyn::FunctionPurity, defaultImpure: bool) -> bool {
    let mut isImpure: bool;
    isImpure = (match purity.clone() {
        Absyn::FunctionPurity::IMPURE { .. } => true,
        Absyn::FunctionPurity::NO_PURITY { .. } => defaultImpure.clone(),
        _ => false,
    });
    isImpure
}

pub fn isInitial(inExp: Arc<Absyn::Exp>) -> bool {
    let mut hasReinit: bool;
    hasReinit = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::CALL { function_: Deref @ Absyn::CREF_IDENT { name: Deref @ "initial", subscripts: _ }, .. } => true,
        Deref @ Absyn::CALL { function_: Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: Deref @ Absyn::CREF_IDENT { name: Deref @ "initial", subscripts: _ } }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    hasReinit
}

fn isInitialTraverseHelper(inExp: Arc<Absyn::Exp>, inBool: bool) -> (Arc<Absyn::Exp>, bool) {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outBool: bool;
    (outExp, outBool) = (::match_deref::match_deref! { match &((inExp.clone(), inBool.clone())) {
        (Deref @ Absyn::UNARY { op: Absyn::NOT, exp: _ }, _) => (inExp.clone(), inBool.clone()),
        (e, _) => {
            let mut b: bool;
            b = isInitial(e.clone());
            (e.clone(), b.clone())
        },
        _ => (inExp.clone(), inBool.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outBool)
}

pub fn isInner(io: Absyn::InnerOuter) -> bool {
    let mut isItAnInner: bool;
    isItAnInner = (match io.clone() {
        Absyn::INNER_OUTER => true,
        Absyn::INNER => true,
        _ => false,
    });
    isItAnInner
}

pub fn isInnerOuter(inIO: Absyn::InnerOuter) -> bool {
    let mut outIsInnerOuter: bool;
    outIsInnerOuter = (match inIO.clone() {
        Absyn::INNER_OUTER => true,
        _ => false,
    });
    outIsInnerOuter
}

pub fn isInput(inDirection: Absyn::Direction) -> bool {
    let mut outIsInput: bool;
    outIsInput = (match inDirection.clone() {
        Absyn::INPUT => true,
        Absyn::INPUT_OUTPUT => true,
        _ => false,
    });
    outIsInput
}

pub fn isInputOrOutput(direction: Absyn::Direction) -> Result<bool> {
    let mut isIorO: bool;
    isIorO = (match direction.clone() {
        Absyn::INPUT => true,
        Absyn::OUTPUT => true,
        Absyn::INPUT_OUTPUT => true,
        Absyn::BIDIR => false,
        _ => bail!("match: no arm matched"),
    });
    Ok(isIorO)
}

pub fn isInvariantExpNoTraverse(e: Arc<Absyn::Exp>, b: bool) -> (Arc<Absyn::Exp>, bool) {
    let mut e: Arc<Absyn::Exp> = e;
    let mut b: bool = b;
    if !(b.clone()) {
        return (e, b);
    }
    b = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::INTEGER { .. } => true,
        Deref @ Absyn::REAL { .. } => true,
        Deref @ Absyn::STRING { .. } => true,
        Deref @ Absyn::BOOL { .. } => true,
        Deref @ Absyn::BINARY { .. } => true,
        Deref @ Absyn::UNARY { .. } => true,
        Deref @ Absyn::LBINARY { .. } => true,
        Deref @ Absyn::LUNARY { .. } => true,
        Deref @ Absyn::RELATION { .. } => true,
        Deref @ Absyn::IFEXP { .. } => true,
        Deref @ Absyn::CALL { function_: Deref @ Absyn::CREF_FULLYQUALIFIED { .. }, .. } => true,
        Deref @ Absyn::PARTEVALFUNCTION { function_: Deref @ Absyn::CREF_FULLYQUALIFIED { .. }, .. } => true,
        Deref @ Absyn::ARRAY { .. } => true,
        Deref @ Absyn::MATRIX { .. } => true,
        Deref @ Absyn::RANGE { .. } => true,
        Deref @ Absyn::CONS { .. } => true,
        Deref @ Absyn::LIST { .. } => true,
        Deref @ Absyn::BREAK => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (e, b)
}

pub fn isLiteralExp(exp: Arc<Absyn::Exp>) -> bool {
    let mut literal: bool;
    literal = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::INTEGER { .. } => true,
        Deref @ Absyn::Exp::REAL { .. } => true,
        Deref @ Absyn::Exp::STRING { .. } => true,
        Deref @ Absyn::Exp::BOOL { .. } => true,
        Deref @ Absyn::Exp::ARRAY { .. } => List::all(var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone(), Arc::new(fnptr!(isLiteralExp, Arc<Absyn::Exp>))),
        Deref @ Absyn::Exp::MATRIX { .. } => {
            literal = true;
            for row in &*var_field!((*exp).matrix, Absyn::Exp::MATRIX).clone() {
                literal = literal.clone() && List::all(row.clone(), Arc::new(fnptr!(isLiteralExp, Arc<Absyn::Exp>)));
                if !(literal.clone()) {
                    break;
                }
            }
            literal.clone()
        },
        Deref @ Absyn::Exp::RANGE { .. } => isLiteralExp(var_field!((*exp).start, Absyn::Exp::RANGE).clone()) && Util::applyOptionOrDefault(var_field!((*exp).step, Absyn::Exp::RANGE).clone(), Arc::new(fnptr!(isLiteralExp, Arc<Absyn::Exp>)), true) && isLiteralExp(var_field!((*exp).stop, Absyn::Exp::RANGE).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    literal
}

pub fn isModel(cls: Arc<Absyn::Class>) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(cls.clone()) {
        Deref @ Absyn::Class::CLASS { restriction: Absyn::Restriction::R_MODEL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isModificationOfPath(r#mod: Arc<Absyn::ElementArg>, path: Arc<Absyn::Path>) -> bool {
    let mut yes: bool;
    yes = (::match_deref::match_deref! { match &((r#mod.clone(), path.clone())) {
        (Deref @ Absyn::MODIFICATION { path: Deref @ Absyn::IDENT { name: id1 }, .. }, Deref @ Absyn::IDENT { name: id2 }) => id1.clone() == id2.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    yes
}

pub fn isNamedPathIdent(path: Arc<Absyn::Path>, name: ArcStr) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::IDENT { .. } => var_field!((*path).name, Absyn::Path::IDENT).clone() == name.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn isNotInnerOuter(inIO: Absyn::InnerOuter) -> bool {
    let mut outIsNotInnerOuter: bool;
    outIsNotInnerOuter = (match inIO.clone() {
        Absyn::NOT_INNER_OUTER => true,
        _ => false,
    });
    outIsNotInnerOuter
}

pub fn isNotPartial(inClass: Arc<Absyn::Class>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = !(isPartial(inClass.clone())?);
    Ok(outBoolean)
}

pub fn isOnlyInner(inIO: Absyn::InnerOuter) -> bool {
    let mut outOnlyInner: bool;
    outOnlyInner = (match inIO.clone() {
        Absyn::INNER => true,
        _ => false,
    });
    outOnlyInner
}

pub fn isOnlyOuter(inIO: Absyn::InnerOuter) -> bool {
    let mut outOnlyOuter: bool;
    outOnlyOuter = (match inIO.clone() {
        Absyn::OUTER => true,
        _ => false,
    });
    outOnlyOuter
}

pub fn isOuter(io: Absyn::InnerOuter) -> bool {
    let mut isItAnOuter: bool;
    isItAnOuter = (match io.clone() {
        Absyn::INNER_OUTER => true,
        Absyn::OUTER => true,
        _ => false,
    });
    isItAnOuter
}

pub fn isOutput(inDirection: Absyn::Direction) -> bool {
    let mut outIsOutput: bool;
    outIsOutput = (match inDirection.clone() {
        Absyn::OUTPUT => true,
        Absyn::INPUT_OUTPUT => true,
        _ => false,
    });
    outIsOutput
}

pub fn isPackageRestriction(inRestriction: Absyn::Restriction) -> bool {
    let mut outIsPackage: bool;
    outIsPackage = (match inRestriction.clone() {
        Absyn::R_PACKAGE => true,
        _ => false,
    });
    outIsPackage
}

pub fn isPartial(inClass: Arc<Absyn::Class>) -> Result<bool> {
    let mut outBoolean: bool;
    let __pa0 = ::match_deref::match_deref! { match &(inClass.clone()) {
        Deref @ Absyn::CLASS { partialPrefix: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    outBoolean = __pa0.clone();
    Ok(outBoolean)
}

pub fn isParts(cl: Arc<Absyn::ClassDef>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::PARTS { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isTuple(exp: Arc<Absyn::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::TUPLE { expressions: _ } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isUniontype(cls: Arc<Absyn::Class>) -> bool {
    let mut b: bool;
    b = (match cls.restriction.clone() {
        Absyn::R_UNIONTYPE => true,
        _ => false,
    });
    b
}

pub fn iteratorGuard(iterator: Arc<Absyn::ForIterator>) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut guardExp: Option<Arc<Absyn::Exp>>;
    let __pa0 = ::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ Absyn::ITERATOR { guardExp: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    guardExp = __pa0.clone();
    Ok(guardExp)
}

fn iteratorIndexedCrefsEqual(inCref1: IteratorIndexedCref, inCref2: IteratorIndexedCref) -> bool {
    let mut outEqual: bool;
    let mut cr1: Arc<Absyn::ComponentRef>;
    let mut cr2: Arc<Absyn::ComponentRef>;
    let mut idx1: i32;
    let mut idx2: i32;
    (cr1, idx1) = inCref1.clone();
    (cr2, idx2) = inCref2.clone();
    outEqual = idx1.clone() == idx2.clone() && crefEqual(cr1.clone(), cr2.clone());
    outEqual
}

pub fn iteratorName(iterator: Arc<Absyn::ForIterator>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ Absyn::ITERATOR { name: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    Ok(name)
}

pub fn iteratorRange(iterator: Arc<Absyn::ForIterator>) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut range: Option<Arc<Absyn::Exp>>;
    let __pa0 = ::match_deref::match_deref! { match &(iterator.clone()) {
        Deref @ Absyn::ITERATOR { range: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    range = __pa0.clone();
    Ok(range)
}

pub fn joinCrefs(inComponentRef1: Arc<Absyn::ComponentRef>, inComponentRef2: Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outComponentRef: Arc<Absyn::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &((inComponentRef1.clone(), inComponentRef2.clone())) {
        (Deref @ Absyn::CREF_IDENT { subscripts: sub, name: id }, cr2) => {
            let mut cr_1: Arc<Absyn::ComponentRef>;
            let mut cr: Arc<Absyn::ComponentRef>;
            if '__try0: {
                ::match_deref::match_deref! { match &(cr2.clone()) {
                    Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => (),
                    _ => break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")),
                } };
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id.clone()).clone(), subscripts: sub.clone(), componentRef: cr2.clone() })
        },
        (Deref @ Absyn::CREF_QUAL { componentRef: cr, subscripts: sub, name: id }, cr2) => {
            let mut cr_1: Arc<Absyn::ComponentRef>;
            cr_1 = joinCrefs(cr.clone(), cr2.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (id.clone()).clone(), subscripts: sub.clone(), componentRef: cr_1.clone() })
        },
        (Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cr }, cr2) => {
            let mut id: ArcStr;
            let mut sub: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut cr_1: Arc<Absyn::ComponentRef>;
            cr_1 = joinCrefs(cr.clone(), cr2.clone())?;
            crefMakeFullyQualified(cr_1.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn joinPaths(inPath1: Arc<Absyn::Path>, inPath2: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath1.clone(), inPath2.clone())) {
        (Deref @ Absyn::IDENT { name: r#str }, p2) => Arc::new(Absyn::Path::QUALIFIED { name: (r#str.clone()).clone(), path: p2.clone() }),
        (Deref @ Absyn::QUALIFIED { path: p, name: r#str }, p2) => {
            let mut p_1: Arc<Absyn::Path>;
            p_1 = joinPaths(p.clone(), p2.clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (r#str.clone()).clone(), path: p_1.clone() })
        },
        (Deref @ Absyn::FULLYQUALIFIED { path: p }, p2) => joinPaths(p.clone(), p2.clone())?,
        (p, Deref @ Absyn::FULLYQUALIFIED { path: p2 }) => joinPaths(p.clone(), p2.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn joinPathsOpt(inPath1: Option<Arc<Absyn::Path>>, inPath2: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (match inPath1.clone() {
        None => inPath2.clone(),
        Some(mut p) => joinPaths(p.clone(), inPath2.clone())?,
    });
    Ok(outPath)
}

pub fn joinPathsOptSuffix(inPath1: Arc<Absyn::Path>, inPath2: Option<Arc<Absyn::Path>>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (match inPath2.clone() {
        Some(mut p) => joinPaths(inPath1.clone(), p.clone())?,
        _ => inPath1.clone(),
    });
    Ok(outPath)
}

pub fn joinWithinPath(within_: Absyn::Within, path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (match within_.clone() {
        Absyn::TOP => path.clone(),
        Absyn::WITHIN { .. } => joinPaths(var_field!(within_.path, Absyn::Within::WITHIN).clone(), path.clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(outPath)
}

pub fn lastClassname(inProgram: Absyn::Program) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Class>>>;
    let mut id: ArcStr;
    let Absyn::PROGRAM { classes: __pa0, .. } = (inProgram.clone()) else { bail!("pattern mismatch") };
    lst = __pa0.clone();
    let __pa1 = ::match_deref::match_deref! { match &(List::last(lst.clone())?) {
        Deref @ Absyn::CLASS { name: __pa1, .. } => __pa1.clone(),
        _ => bail!("pattern mismatch"),
    } };
    id = __pa1.clone();
    outPath = Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() });
    Ok(outPath)
}

pub fn lookupAnnotation(ann: Arc<Absyn::Annotation>, name: ArcStr) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>> = None;
    for m in &*ann.elementArgs.clone() {
        outMod = (::match_deref::match_deref! { match &(m.clone()) {
        Deref @ Absyn::MODIFICATION { .. } if (pathFirstIdent(var_field!(m.path, Absyn::ElementArg::MODIFICATION).clone())? == name.clone()) => var_field!(m.modification, Absyn::ElementArg::MODIFICATION).clone(),
        _ => outMod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        if isSome(outMod.clone()) {
            break;
        }
    }
    Ok(outMod)
}

pub fn lookupClassAnnotation(cls: Arc<Absyn::Class>, name: ArcStr) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>>;
    outMod = lookupClassDefAnnotation(cls.body.clone(), (name.clone()).clone())?;
    Ok(outMod)
}

pub fn lookupClassDefAnnotation(cdef: Arc<Absyn::ClassDef>, name: ArcStr) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>> = None;
    let mut ann: Arc<Absyn::Annotation>;
    outMod = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::PARTS { .. } => List::findSome(var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone(), Arc::new({ let __pe_b1 = name.clone(); move |__pe_a0| lookupAnnotation(__pe_a0, __pe_b1.clone()) })),
        Deref @ Absyn::CLASS_EXTENDS { .. } => List::findSome(var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone(), Arc::new({ let __pe_b1 = name.clone(); move |__pe_a0| lookupAnnotation(__pe_a0, __pe_b1.clone()) })),
        Deref @ Absyn::DERIVED { .. } => lookupCommentOptAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::DERIVED).clone(), (name.clone()).clone())?,
        Deref @ Absyn::ENUMERATION { .. } => lookupCommentOptAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::ENUMERATION).clone(), (name.clone()).clone())?,
        Deref @ Absyn::OVERLOAD { .. } => lookupCommentOptAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::OVERLOAD).clone(), (name.clone()).clone())?,
        Deref @ Absyn::PDER { .. } => lookupCommentOptAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::PDER).clone(), (name.clone()).clone())?,
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outMod)
}

pub fn lookupCommentOptAnnotation(cmt: Option<Arc<Absyn::Comment>>, name: ArcStr) -> Result<Option<Arc<Absyn::Modification>>> {
    let mut outMod: Option<Arc<Absyn::Modification>>;
    let mut ann: Arc<Absyn::Annotation>;
    outMod = (match cmt.clone() {
        Some(Absyn::COMMENT { annotation_: Some(mut ann), .. }) => lookupAnnotation(ann.clone(), (name.clone()).clone())?,
        _ => None,
    });
    Ok(outMod)
}

pub fn makeCall(name: Arc<Absyn::ComponentRef>, posArgs: Arc<metamodelica::List<Arc<Absyn::Exp>>>, namedArgs: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>) -> Arc<Absyn::Exp> {
    let mut callExp: Arc<Absyn::Exp>;
    callExp = Arc::new(Absyn::Exp::CALL { function_: name.clone(), functionArgs: Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: posArgs.clone(), argNames: namedArgs.clone() }), typeVars: metamodelica::nil() });
    callExp
}

pub fn makeClassElement(cl: Arc<Absyn::Class>) -> Result<Arc<Absyn::ElementItem>> {
    let mut el: Arc<Absyn::ElementItem>;
    let mut info: SourceInfo;
    let mut fp: bool;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::CLASS { info: __pa0, finalPrefix: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    info = __pa0.clone();
    fp = __pa1.clone();
    el = Arc::new(Absyn::ElementItem::ELEMENTITEM { element: Arc::new(Absyn::Element::ELEMENT { finalPrefix: fp.clone(), redeclareKeywords: None, innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER, specification: Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: false, class_: cl.clone() }), info: info.clone(), constrainClass: None }) });
    Ok(el)
}

pub fn makeCons(e1: Arc<Absyn::Exp>, e2: Arc<Absyn::Exp>) -> Arc<Absyn::Exp> {
    let mut e: Arc<Absyn::Exp>;
    e = Arc::new(Absyn::Exp::CONS { head: e1.clone(), rest: e2.clone() });
    e
}

pub fn makeFullyQualified(inPath: Arc<Absyn::Path>) -> Arc<Absyn::Path> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::FULLYQUALIFIED { .. } => inPath.clone(),
        _ => Arc::new(Absyn::Path::FULLYQUALIFIED { path: inPath.clone() }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPath
}

pub fn makeIdentPathFromString(s: ArcStr) -> Arc<Absyn::Path> {
    let mut p: Arc<Absyn::Path>;
    p = Arc::new(Absyn::Path::IDENT { name: (s.clone()).clone() });
    p
}

pub fn makeIntegerSubscript(n: i32) -> Arc<Absyn::Subscript> {
    let mut sub: Arc<Absyn::Subscript>;
    sub = Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: Arc::new(Absyn::Exp::INTEGER { value: n.clone() }) });
    sub
}

pub fn makeNotFullyQualified(inPath: Arc<Absyn::Path>) -> Arc<Absyn::Path> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::FULLYQUALIFIED { path } => path.clone(),
        _ => inPath.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPath
}

pub fn makeQualifiedPathFromStrings(s1: ArcStr, s2: ArcStr) -> Arc<Absyn::Path> {
    let mut p: Arc<Absyn::Path>;
    p = Arc::new(Absyn::Path::QUALIFIED { name: (s1.clone()).clone(), path: Arc::new(Absyn::Path::IDENT { name: (s2.clone()).clone() }) });
    p
}

pub fn makeSubscript(inExp: Arc<Absyn::Exp>) -> Arc<Absyn::Subscript> {
    let mut outSubscript: Arc<Absyn::Subscript>;
    outSubscript = Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: inExp.clone() });
    outSubscript
}

pub fn mapAnnotationBinding(ann: Arc<Absyn::Annotation>, path: Arc<Absyn::Path>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<(Arc<Absyn::Annotation>, bool)> {
    pub type MapFunc = fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>>;

    let mut ann: Arc<Absyn::Annotation> = ann;
    let mut found: bool;
    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = ann.elementArgs.clone();
    (args, found) = List::findMap(args.clone(), Arc::new({ let __pe_b1 = path.clone(); let __pe_b2 = func.clone(); move |__pe_a0| mapAnnotationBindingInArg(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
    assign_field!(ann.elementArgs = args.clone());
    Ok((ann, found))
}

pub fn mapAnnotationBindingInArg(arg: Arc<Absyn::ElementArg>, path: Arc<Absyn::Path>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<(Arc<Absyn::ElementArg>, bool)> {
    pub type MapFunc = fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>>;

    let mut arg: Arc<Absyn::ElementArg> = arg;
    let mut found: bool = false;
    let mut r#mod: Arc<Absyn::Modification>;
    let mut mod_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut mod_eq: Arc<Absyn::EqMod>;
    let mut rest_path: Arc<Absyn::Path>;
    let mut arg_path_len: i32;
    let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(r#mod), .. } => {
            let mut r#mod = (*r#mod).clone();
            if pathPrefixOf(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), path.clone()) {
                arg_path_len = pathPartCount(var_field!((*arg).path, Absyn::ElementArg::MODIFICATION).clone(), 0)?;
                if arg_path_len.clone() == pathPartCount(path.clone(), 0)? {
                    mod_eq = mapAnnotationBindingInEqMod(r#mod.eqMod.clone(), func.clone());
                    r#mod.eqMod = mod_eq.clone(); // TODO: unhandled field-assign shape
                    found = true;
                } else {
                    rest_path = Util::foldcallN(arg_path_len.clone(), Arc::new(pathRest), path.clone());
                    (mod_args, found) = List::findMap(r#mod.elementArgLst.clone(), Arc::new({ let __pe_b1 = rest_path.clone(); let __pe_b2 = func.clone(); move |__pe_a0| mapAnnotationBindingInArg(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }))?;
                    r#mod.elementArgLst = mod_args.clone(); // TODO: unhandled field-assign shape
                }
                if found.clone() {
                    assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod.clone()));
                }
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((arg, found))
}

pub fn mapAnnotationBindingInEqMod(eqMod: Arc<Absyn::EqMod>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Arc<Absyn::EqMod> {
    pub type MapFunc = fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>>;

    let mut eqMod: Arc<Absyn::EqMod> = eqMod;
    let () = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => {
            assign_variant_field!(eqMod => Absyn::EqMod::EQMOD; exp = func(var_field!((*eqMod).exp, Absyn::EqMod::EQMOD).clone()).unwrap());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    eqMod
}

pub fn mapCrefExps(cref: Arc<Absyn::ComponentRef>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Arc<Absyn::ComponentRef> {
    pub type Func = fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>>;

    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let () = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_IDENT; subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for s in (var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone()).into_iter().cloned() {
            let __x = mapSubscriptExp(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_QUAL; subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for s in (var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone()).into_iter().cloned() {
            let __x = mapSubscriptExp(s.clone(), func.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
            assign_variant_field!(cref => Absyn::ComponentRef::CREF_FULLYQUALIFIED; componentRef = mapCrefExps(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), func.clone()));
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cref
}

pub fn mapCrefParts(inCref: Arc<Absyn::ComponentRef>, inMapFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>> + 'static>) -> Result<Arc<Absyn::ComponentRef>> {
    pub type MapFunc = fn(Arc<Absyn::ComponentRef>) -> Result<Arc<Absyn::ComponentRef>>;

    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &((inCref.clone(), inMapFunc.clone())) {
        (Deref @ Absyn::CREF_QUAL { name, subscripts: subs, componentRef: rest_cref }, _) => {
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut name = (*name).clone();
            let mut subs = (*subs).clone();
            let mut rest_cref = (*rest_cref).clone();
            cref = Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subs.clone() });
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inMapFunc(cref.clone())?) {
                Deref @ Absyn::CREF_IDENT { name: __pa0, subscripts: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => bail!("pattern mismatch"),
            } };
            name = __pa0.clone();
            subs = __pa1.clone();
            rest_cref = mapCrefParts(rest_cref.clone(), inMapFunc.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (name.clone()).clone(), subscripts: subs.clone(), componentRef: rest_cref.clone() })
        },
        (Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cref }, _) => {
            let mut name: ArcStr;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut rest_cref: Arc<Absyn::ComponentRef>;
            let mut cref = (*cref).clone();
            cref = mapCrefParts(cref.clone(), inMapFunc.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref.clone() })
        },
        _ => inMapFunc(inCref.clone())?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCref)
}

pub fn mapSubscriptExp(sub: Arc<Absyn::Subscript>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>> + 'static>) -> Arc<Absyn::Subscript> {
    pub type Func = fn(Arc<Absyn::Exp>) -> Result<Arc<Absyn::Exp>>;

    let mut sub: Arc<Absyn::Subscript> = sub;
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = func(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone()).unwrap());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sub
}

fn mergeAnnotationEqMods(oldEq: Arc<Absyn::EqMod>, newEq: Arc<Absyn::EqMod>, mergeExpressions: bool) -> Arc<Absyn::EqMod> {
    let mut outEq: Arc<Absyn::EqMod>;
    let mut new_exp: Arc<Absyn::Exp>;
    let mut old_exp: Arc<Absyn::Exp>;
    outEq = (::match_deref::match_deref! { match &((oldEq.clone(), newEq.clone())) {
        (Deref @ Absyn::EqMod::NOMOD { .. }, _) => newEq.clone(),
        (_, Deref @ Absyn::EqMod::NOMOD { .. }) => oldEq.clone(),
        (Deref @ Absyn::EqMod::EQMOD { exp: old_exp, .. }, Deref @ Absyn::EqMod::EQMOD { exp: new_exp, .. }) if (mergeExpressions.clone()) => {
            let mut new_exp = (*new_exp).clone();
            new_exp = (::match_deref::match_deref! { match &((old_exp.clone(), new_exp.clone())) {
        (Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CALL { .. }, tail: _ } }, Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::List::Cons { head: Deref @ Absyn::Exp::CALL { .. }, tail: _ } }) => Arc::new(Absyn::Exp::ARRAY { arrayExp: listAppend(var_field!((*old_exp).arrayExp, Absyn::Exp::ARRAY).clone(), var_field!((*new_exp).arrayExp, Absyn::Exp::ARRAY).clone()) }),
        _ => new_exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            Arc::new(Absyn::EqMod::EQMOD { exp: Arc::new(new_exp.clone()), info: var_field!((*newEq).info, Absyn::EqMod::EQMOD).clone() })
        },
        _ => newEq.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEq
}

pub fn mergeAnnotations(oldAnnotation: Arc<Absyn::Annotation>, newAnnotation: Arc<Absyn::Annotation>, mergeSubMods: bool, mergeEqMods: bool) -> Result<Arc<Absyn::Annotation>> {
    let mut outAnnotation: Arc<Absyn::Annotation>;
    let mut args1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut args2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    outAnnotation = (::match_deref::match_deref! { match &((oldAnnotation.clone(), newAnnotation.clone())) {
        (Deref @ Absyn::ANNOTATION { elementArgs: Deref @ metamodelica::List::Nil }, _) => newAnnotation.clone(),
        (_, Deref @ Absyn::ANNOTATION { elementArgs: Deref @ metamodelica::List::Nil }) => oldAnnotation.clone(),
        _ => Arc::new(Absyn::Annotation { elementArgs: mergeAnnotations2(oldAnnotation.elementArgs.clone(), newAnnotation.elementArgs.clone(), mergeSubMods.clone(), mergeEqMods.clone())? }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAnnotation)
}

fn mergeAnnotations2(oldArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, newArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, mergeSubMods: bool, mergeEqMods: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    let mut outArgs: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = oldArgs.clone();
    let mut found: bool;
    let mut new_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = metamodelica::nil();
    for arg in &*newArgs.clone() {
        (outArgs, found) = List::findAndMap(outArgs.clone(), Arc::new({ let __pe_b1 = elementArgName(arg.clone())?; move |__pe_a0| Ok(isModificationOfPath(__pe_a0, __pe_b1.clone())) }), Arc::new(if (mergeSubMods.clone()) {{ let __pe_b1 = arg.clone(); let __pe_b2 = mergeEqMods.clone(); move |__pe_a0| mergeAnnotations3(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }} else {{ let __pe_b1 = arg.clone(); move |__pe_a0| subModsInSameOrder(__pe_a0, __pe_b1.clone()) }}))?;
        if !(found.clone()) {
            new_args = cons(arg.clone(), new_args.clone());
        }
    }
    outArgs = listAppend(outArgs.clone(), new_args.clone().reverse());
    Ok(outArgs)
}

fn mergeAnnotations3(oldArg: Arc<Absyn::ElementArg>, newArg: Arc<Absyn::ElementArg>, mergeEqMods: bool) -> Result<Arc<Absyn::ElementArg>> {
    let mut outArg: Arc<Absyn::ElementArg>;
    let mut old_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut new_args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut old_eq: Arc<Absyn::EqMod>;
    let mut new_eq: Arc<Absyn::EqMod>;
    let mut cmt: Option<ArcStr>;
    outArg = (::match_deref::match_deref! { match &((oldArg.clone(), newArg.clone())) {
        (Deref @ Absyn::ElementArg::MODIFICATION { modification: None, .. }, _) => newArg.clone(),
        (_, Deref @ Absyn::ElementArg::MODIFICATION { modification: None, .. }) => oldArg.clone(),
        (Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Absyn::Modification::CLASSMOD { elementArgLst: old_args, eqMod: old_eq }), .. }, Deref @ Absyn::ElementArg::MODIFICATION { modification: Some(Absyn::Modification::CLASSMOD { elementArgLst: new_args, eqMod: new_eq }), .. }) => {
            let mut new_args = (*new_args).clone();
            let mut new_eq = (*new_eq).clone();
            new_eq = mergeAnnotationEqMods(old_eq.clone(), new_eq.clone(), mergeEqMods.clone());
            new_args = mergeAnnotations2(old_args.clone(), new_args.clone(), true, mergeEqMods.clone())?;
            cmt = (if (isSome((var_field!((*newArg).comment, Absyn::ElementArg::MODIFICATION).clone()).clone())) {var_field!((*newArg).comment, Absyn::ElementArg::MODIFICATION).clone()} else {var_field!((*oldArg).comment, Absyn::ElementArg::MODIFICATION).clone()}).clone();
            Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: var_field!((*oldArg).path, Absyn::ElementArg::MODIFICATION).clone(), modification: Some(Arc::new(Absyn::Modification { elementArgLst: new_args.clone(), eqMod: new_eq.clone() })), comment: cmt.clone(), info: var_field!((*oldArg).info, Absyn::ElementArg::MODIFICATION).clone() })
        },
        _ => newArg.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

pub fn mergeAnnotationsList(oldAnnotation: Arc<Absyn::Annotation>, newAnnotations: Arc<metamodelica::List<Arc<Absyn::Annotation>>>) -> Result<Arc<Absyn::Annotation>> {
    let mut outAnnotation: Arc<Absyn::Annotation> = oldAnnotation.clone();
    for ann in &*newAnnotations.clone() {
        outAnnotation = mergeAnnotations(ann.clone(), outAnnotation.clone(), false, false)?;
    }
    Ok(outAnnotation)
}

pub fn mergeCommentAnnotation(inAnnotation: Arc<Absyn::Annotation>, inComment: Option<Arc<Absyn::Comment>>) -> Result<Option<Arc<Absyn::Comment>>> {
    let mut outComment: Option<Arc<Absyn::Comment>>;
    outComment = (match inComment.clone() {
        None => Some(Arc::new(Absyn::Comment { annotation_: Some(inAnnotation.clone()), comment: None })),
        Some(Absyn::COMMENT { comment: mut cmt, annotation_: None }) => Some(Arc::new(Absyn::Comment { annotation_: Some(inAnnotation.clone()), comment: cmt.clone() })),
        Some(Absyn::COMMENT { comment: mut cmt, annotation_: Some(mut ann) }) => Some(Arc::new(Absyn::Comment { annotation_: Some(mergeAnnotations(ann.clone(), inAnnotation.clone(), false, false)?), comment: cmt.clone() })),
        _ => bail!("match: no arm matched"),
    });
    Ok(outComment)
}

pub fn mergeEqMods(outerEqMod: Arc<Absyn::EqMod>, innerEqMod: Arc<Absyn::EqMod>) -> Arc<Absyn::EqMod> {
    let mut outEqMod: Arc<Absyn::EqMod>;
    outEqMod = (::match_deref::match_deref! { match &(outerEqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => outerEqMod.clone(),
        _ => innerEqMod.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outEqMod
}

pub fn mergeModifiers(outerMod: Arc<Absyn::Modification>, innerMod: Arc<Absyn::Modification>) -> Result<Arc<Absyn::Modification>> {
    let mut outMod: Arc<Absyn::Modification>;
    outMod = Arc::new(Absyn::Modification { elementArgLst: mergeAnnotations2(innerMod.elementArgLst.clone(), outerMod.elementArgLst.clone(), false, false)?, eqMod: mergeEqMods(outerMod.eqMod.clone(), innerMod.eqMod.clone()) });
    Ok(outMod)
}

pub fn modEqual(mod1: Arc<Absyn::Modification>, mod2: Arc<Absyn::Modification>) -> bool {
    let mut equal: bool = eqModEqual(mod1.eqMod.clone(), mod2.eqMod.clone()).unwrap() && List::isEqualOnTrue(mod1.elementArgLst.clone(), mod2.elementArgLst.clone(), Arc::new(elementArgEqual));
    equal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn onlyLiteralsInAnnotationMod(inMod: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<bool> {
    let mut onlyLiterals: bool;
    onlyLiterals = 'mc: {
        let __mc_input = inMod.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut dive: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut eqMod: Arc<Absyn::EqMod>;
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::MODIFICATION { path: Deref @ Absyn::IDENT { name: Deref @ "interaction" }, .. }, tail: rest } => {
                    let mut dive: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut eqMod: Arc<Absyn::EqMod>;
                    Ok(onlyLiteralsInAnnotationMod(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::MODIFICATION { modification: Some(Absyn::CLASSMOD { elementArgLst: dive, eqMod }), .. }, tail: rest } => {
                    Ok(onlyLiteralsInEqMod(eqMod.clone())? && onlyLiteralsInAnnotationMod(dive.clone())? && onlyLiteralsInAnnotationMod(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: rest } => {
                    let mut dive: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut eqMod: Arc<Absyn::EqMod>;
                    Ok(onlyLiteralsInAnnotationMod(rest.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dive: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut eqMod: Arc<Absyn::EqMod>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(onlyLiterals)
}

pub fn onlyLiteralsInEqMod(eqMod: Arc<Absyn::EqMod>) -> Result<bool> {
    let mut onlyLiterals: bool;
    onlyLiterals = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::NOMOD => true,
        Deref @ Absyn::EQMOD { .. } => onlyLiteralsInExp(var_field!((*eqMod).exp, Absyn::EqMod::EQMOD).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(onlyLiterals)
}

pub fn onlyLiteralsInExp(exp: Arc<Absyn::Exp>) -> Result<bool> {
    let mut onlyLiterals: bool;
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    let __pa0 = ::match_deref::match_deref! { match &(traverseExpBidir(exp.clone(), Arc::new(fnptr!(onlyLiteralsInExpEnter, Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>)), Arc::new(fnptr!(onlyLiteralsInExpExit, Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>)), cons(metamodelica::nil(), metamodelica::nil()))?) {
        (_, Deref @ metamodelica::List::Cons { head: __pa0, tail: Deref @ metamodelica::List::Nil }) => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    lst = __pa0.clone();
    onlyLiterals = lst.clone().is_empty();
    Ok(onlyLiterals)
}

fn onlyLiteralsInExpEnter(inExp: Arc<Absyn::Exp>, inLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
    (outExp, outLst) = (::match_deref::match_deref! { match &((inExp.clone(), inLst.clone())) {
        (e @ Deref @ Absyn::CREF { componentRef: Deref @ Absyn::CREF_QUAL { name, .. } }, Deref @ metamodelica::List::Cons { head: lst, tail: rest }) => {
            let mut b: bool;
            let mut lst = (*lst).clone();
            b = listMember((name.clone()).clone(), list![(literal!("LinePattern")).clone(), (literal!("Arrow")).clone(), (literal!("FillPattern")).clone(), (literal!("BorderPattern")).clone(), (literal!("TextStyle")).clone(), (literal!("Smooth")).clone(), (literal!("TextAlignment")).clone()]);
            lst = List::consOnTrue(!(b.clone()), e.clone(), lst.clone());
            (inExp.clone(), cons(lst.clone(), rest.clone()))
        },
        (Deref @ Absyn::CREF { .. }, Deref @ metamodelica::List::Cons { head: lst, tail: rest }) => (inExp.clone(), cons(cons(inExp.clone(), lst.clone()), rest.clone())),
        _ => (inExp.clone(), inLst.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outLst)
}

fn onlyLiteralsInExpExit(inExp: Arc<Absyn::Exp>, inLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) -> (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>) {
    let mut outExp: Arc<Absyn::Exp>;
    let mut outLst: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
    (outExp, outLst) = (::match_deref::match_deref! { match &((inExp.clone(), inLst.clone())) {
        (Deref @ Absyn::CALL { function_: Deref @ Absyn::CREF_IDENT { name: Deref @ "DynamicSelect", .. }, .. }, lst) => (inExp.clone(), lst.clone()),
        _ => (inExp.clone(), inLst.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outLst)
}

pub fn opEqual(op1: Absyn::Operator, op2: Absyn::Operator) -> bool {
    let mut isEqual: bool;
    isEqual = op1.clone() == op2.clone();
    isEqual
}

pub fn opIsElementWise(op: Absyn::Operator) -> bool {
    let mut isElementWise: bool;
    isElementWise = (match op.clone() {
        Absyn::ADD_EW => true,
        Absyn::SUB_EW => true,
        Absyn::MUL_EW => true,
        Absyn::DIV_EW => true,
        Absyn::POW_EW => true,
        Absyn::UPLUS_EW => true,
        Absyn::UMINUS_EW => true,
        _ => false,
    });
    isElementWise
}

pub fn optArrayDimEqual(oad1: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>, oad2: Option<Arc<metamodelica::List<Arc<Absyn::Subscript>>>>) -> bool {
    let mut b: bool;
    b = (match (oad1.clone(), oad2.clone()) {
        (Some(mut ad1), Some(mut ad2)) => List::isEqualOnTrue(ad1.clone(), ad2.clone(), Arc::new(subscriptEqual)),
        (None, None) => true,
        _ => false,
    });
    b
}

pub fn optMsg(inShowMessage: bool, inInfo: SourceInfo) -> Absyn::Msg {
    let mut outMsg: Absyn::Msg;
    outMsg = if (inShowMessage.clone()) {Absyn::Msg::MSG { info: inInfo.clone() }} else {openmodelica_ast::Absyn::Msg::NO_MSG};
    outMsg
}

pub fn optPathString(inPathOption: Option<Arc<Absyn::Path>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = ((match inPathOption.clone() {
        None => literal!(""),
        Some(mut p) => pathString(p.clone(), (literal!(".")).clone(), true, false)?,
    })).clone();
    Ok(outString)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn partsHasLocalClass(inParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>) -> Result<bool> {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = inParts.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::PUBLIC { contents: elts }, tail: _ } => {
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let true = (eltsHasLocalClass(elts.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::PROTECTED { contents: elts }, tail: _ } => {
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let true = (eltsHasLocalClass(elts.clone())?) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: _, tail: parts } => {
                    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
                    Ok(partsHasLocalClass(parts.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn pathCompare(ip1: Arc<Absyn::Path>, ip2: Arc<Absyn::Path>) -> Result<i32> {
    let mut o: i32;
    o = (::match_deref::match_deref! { match &((ip1.clone(), ip2.clone())) {
        (Deref @ Absyn::FULLYQUALIFIED { path: p1 }, Deref @ Absyn::FULLYQUALIFIED { path: p2 }) => pathCompare(p1.clone(), p2.clone())?,
        (Deref @ Absyn::FULLYQUALIFIED { .. }, _) => 1,
        (_, Deref @ Absyn::FULLYQUALIFIED { .. }) => -1,
        (Deref @ Absyn::QUALIFIED { name: i1, path: p1 }, Deref @ Absyn::QUALIFIED { name: i2, path: p2 }) => {
            o = stringCompare((i1.clone()).clone(), (i2.clone()).clone());
            o = if (o.clone() == 0) {pathCompare(p1.clone(), p2.clone())?} else {o.clone()};
            o.clone()
        },
        (Deref @ Absyn::QUALIFIED { .. }, _) => 1,
        (_, Deref @ Absyn::QUALIFIED { .. }) => -1,
        (Deref @ Absyn::IDENT { name: i1 }, Deref @ Absyn::IDENT { name: i2 }) => stringCompare((i1.clone()).clone(), (i2.clone()).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(o)
}

pub fn pathCompareNoQual(ip1: Arc<Absyn::Path>, ip2: Arc<Absyn::Path>) -> Result<i32> {
    let mut o: i32;
    o = (::match_deref::match_deref! { match &((ip1.clone(), ip2.clone())) {
        (Deref @ Absyn::FULLYQUALIFIED { path: p1 }, p2) => pathCompareNoQual(p1.clone(), p2.clone())?,
        (p1, Deref @ Absyn::FULLYQUALIFIED { path: p2 }) => pathCompareNoQual(p1.clone(), p2.clone())?,
        (Deref @ Absyn::QUALIFIED { name: i1, path: p1 }, Deref @ Absyn::QUALIFIED { name: i2, path: p2 }) => {
            o = stringCompare((i1.clone()).clone(), (i2.clone()).clone());
            o = if (o.clone() == 0) {pathCompare(p1.clone(), p2.clone())?} else {o.clone()};
            o.clone()
        },
        (Deref @ Absyn::QUALIFIED { .. }, _) => 1,
        (_, Deref @ Absyn::QUALIFIED { .. }) => -1,
        (Deref @ Absyn::IDENT { name: i1 }, Deref @ Absyn::IDENT { name: i2 }) => stringCompare((i1.clone()).clone(), (i2.clone()).clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(o)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathContains(path: Arc<Absyn::Path>, name: ArcStr) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => var_field!((*path).name, Absyn::Path::IDENT).clone() == name.clone(),
        Deref @ Absyn::Path::QUALIFIED { .. } => var_field!((*path).name, Absyn::Path::QUALIFIED).clone() == name.clone() || pathContains(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), (name.clone()).clone())?,
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => pathContains(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), (name.clone()).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathEqual(path1: Arc<Absyn::Path>, path2: Arc<Absyn::Path>) -> bool {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((path1.clone(), path2.clone())) {
        (Deref @ Absyn::FULLYQUALIFIED { .. }, _) => pathEqual(var_field!((*path1).path, Absyn::Path::FULLYQUALIFIED).clone(), path2.clone()),
        (_, Deref @ Absyn::FULLYQUALIFIED { .. }) => pathEqual(path1.clone(), var_field!((*path2).path, Absyn::Path::FULLYQUALIFIED).clone()),
        (Deref @ Absyn::IDENT { .. }, Deref @ Absyn::IDENT { .. }) => stringEq((var_field!((*path1).name, Absyn::Path::IDENT).clone()).clone(), (var_field!((*path2).name, Absyn::Path::IDENT).clone()).clone()),
        (Deref @ Absyn::QUALIFIED { .. }, Deref @ Absyn::QUALIFIED { .. }) => stringEq((var_field!((*path1).name, Absyn::Path::QUALIFIED).clone()).clone(), (var_field!((*path2).name, Absyn::Path::QUALIFIED).clone()).clone()) && pathEqual(var_field!((*path1).path, Absyn::Path::QUALIFIED).clone(), var_field!((*path2).path, Absyn::Path::QUALIFIED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathEqualCaseInsensitive(path1: Arc<Absyn::Path>, path2: Arc<Absyn::Path>) -> bool {
    let mut equal: bool;
    equal = (::match_deref::match_deref! { match &((path1.clone(), path2.clone())) {
        (Deref @ Absyn::FULLYQUALIFIED { .. }, _) => pathEqualCaseInsensitive(var_field!((*path1).path, Absyn::Path::FULLYQUALIFIED).clone(), path2.clone()),
        (_, Deref @ Absyn::FULLYQUALIFIED { .. }) => pathEqualCaseInsensitive(path1.clone(), var_field!((*path2).path, Absyn::Path::FULLYQUALIFIED).clone()),
        (Deref @ Absyn::IDENT { .. }, Deref @ Absyn::IDENT { .. }) => stringEq((System::tolower((var_field!((*path1).name, Absyn::Path::IDENT).clone()).clone())).clone(), (System::tolower((var_field!((*path2).name, Absyn::Path::IDENT).clone()).clone())).clone()),
        (Deref @ Absyn::QUALIFIED { .. }, Deref @ Absyn::QUALIFIED { .. }) => stringEq((System::tolower((var_field!((*path1).name, Absyn::Path::QUALIFIED).clone()).clone())).clone(), (System::tolower((var_field!((*path2).name, Absyn::Path::QUALIFIED).clone()).clone())).clone()) && pathEqualCaseInsensitive(var_field!((*path1).path, Absyn::Path::QUALIFIED).clone(), var_field!((*path2).path, Absyn::Path::QUALIFIED).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    equal
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathFirstIdent(path: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::FULLYQUALIFIED { .. } => pathFirstIdent(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone())?,
        Deref @ Absyn::QUALIFIED { .. } => var_field!((*path).name, Absyn::Path::QUALIFIED).clone(),
        Deref @ Absyn::IDENT { .. } => var_field!((*path).name, Absyn::Path::IDENT).clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathFirstPath(path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::IDENT { .. } => path.clone(),
        Deref @ Absyn::QUALIFIED { .. } => Arc::new(Absyn::Path::IDENT { name: (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone() }),
        Deref @ Absyn::FULLYQUALIFIED { .. } => pathFirstPath(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn pathGe(path1: Arc<Absyn::Path>, path2: Arc<Absyn::Path>) -> Result<bool> {
    let mut ge: bool;
    ge = !(pathLt(path1.clone(), path2.clone())?);
    Ok(ge)
}

pub fn pathHash(path: Arc<Absyn::Path>) -> Result<i32> {
    let mut hash: i32;
    hash = pathHashContinue(path.clone(), Util::HASH_SEED.clone())?;
    Ok(hash)
}

pub fn pathHashContinue(path: Arc<Absyn::Path>, hash: i32) -> Result<i32> {
    let mut hash: i32 = hash;
    hash = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::FULLYQUALIFIED { .. } => {
            hash = stringHashDjb2Continue((literal!(".")).clone(), hash.clone());
            pathHashContinue(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), hash.clone())?
        },
        Deref @ Absyn::QUALIFIED { .. } => {
            hash = stringHashDjb2Continue((literal!(".")).clone(), hash.clone());
            hash = stringHashDjb2Continue((var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone(), hash.clone());
            pathHashContinue(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), hash.clone())?
        },
        Deref @ Absyn::IDENT { .. } => {
            hash = stringHashDjb2Continue((literal!(".")).clone(), hash.clone());
            hash = stringHashDjb2Continue((var_field!((*path).name, Absyn::Path::IDENT).clone()).clone(), hash.clone());
            hash.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(hash)
}

pub fn pathIsFullyQualified(inPath: Arc<Absyn::Path>) -> bool {
    let mut outIsQualified: bool;
    outIsQualified = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::FULLYQUALIFIED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsQualified
}

pub fn pathIsIdent(inPath: Arc<Absyn::Path>) -> bool {
    let mut outIsIdent: bool;
    outIsIdent = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::IDENT { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsIdent
}

pub fn pathIsQual(inPath: Arc<Absyn::Path>) -> bool {
    let mut outIsQual: bool;
    outIsQual = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::QUALIFIED { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsQual
}

pub fn pathLast(path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path> = path;
    path = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::QUALIFIED { .. } => pathLast(var_field!((*path).path, Absyn::Path::QUALIFIED).clone())?,
        Deref @ Absyn::IDENT { .. } => path.clone(),
        Deref @ Absyn::FULLYQUALIFIED { .. } => pathLast(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(path)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathLastIdent(path: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::QUALIFIED { .. } => pathLastIdent(var_field!((*path).path, Absyn::Path::QUALIFIED).clone())?,
        Deref @ Absyn::IDENT { .. } => var_field!((*path).name, Absyn::Path::IDENT).clone(),
        Deref @ Absyn::FULLYQUALIFIED { .. } => pathLastIdent(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

pub fn pathLt(path1: Arc<Absyn::Path>, path2: Arc<Absyn::Path>) -> Result<bool> {
    let mut lt: bool;
    lt = stringCompare((pathString(path1.clone(), (literal!(".")).clone(), true, false)?).clone(), (pathString(path2.clone(), (literal!(".")).clone(), true, false)?).clone()) < 0;
    Ok(lt)
}

pub fn pathNthIdent(path: Arc<Absyn::Path>, n: i32) -> Result<ArcStr> {
    let mut ident: ArcStr;
    let mut p: Arc<Absyn::Path> = makeNotFullyQualified(path.clone());
    for i in 2..=n.clone() {
        let __pa0 = ::match_deref::match_deref! { match &(p.clone()) {
            Deref @ Absyn::QUALIFIED { path: __pa0, .. } => __pa0.clone(),
            _ => bail!("pattern mismatch"),
        } };
        p = __pa0.clone();
    }
    ident = (pathFirstIdent(p.clone())?).clone();
    Ok(ident)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathPartCount(path: Arc<Absyn::Path>, partsAccum: i32) -> Result<i32> {
    let mut parts: i32;
    parts = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::IDENT { .. } => partsAccum.clone() + 1,
        Deref @ Absyn::QUALIFIED { .. } => pathPartCount(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), partsAccum.clone() + 1)?,
        Deref @ Absyn::FULLYQUALIFIED { .. } => pathPartCount(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), partsAccum.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(parts)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathPrefix(path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut prefix: Arc<Absyn::Path>;
    prefix = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::FULLYQUALIFIED { .. } => pathPrefix(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone())?,
        Deref @ Absyn::QUALIFIED { path: Deref @ Absyn::IDENT { .. }, .. } => Arc::new(Absyn::Path::IDENT { name: (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone() }),
        Deref @ Absyn::QUALIFIED { .. } => Arc::new(Absyn::Path::QUALIFIED { name: (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone(), path: pathPrefix(var_field!((*path).path, Absyn::Path::QUALIFIED).clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(prefix)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathPrefixOf(prefixPath: Arc<Absyn::Path>, path: Arc<Absyn::Path>) -> bool {
    let mut isPrefix: bool;
    isPrefix = (::match_deref::match_deref! { match &((prefixPath.clone(), path.clone())) {
        (Deref @ Absyn::FULLYQUALIFIED { path: p }, p2) => pathPrefixOf(p.clone(), p2.clone()),
        (p, Deref @ Absyn::FULLYQUALIFIED { path: p2 }) => pathPrefixOf(p.clone(), p2.clone()),
        (Deref @ Absyn::IDENT { name: id }, Deref @ Absyn::IDENT { name: id2 }) => stringEq((id.clone()).clone(), (id2.clone()).clone()),
        (Deref @ Absyn::IDENT { name: id }, Deref @ Absyn::QUALIFIED { name: id2, .. }) => stringEq((id.clone()).clone(), (id2.clone()).clone()),
        (Deref @ Absyn::QUALIFIED { name: id, path: p }, Deref @ Absyn::QUALIFIED { name: id2, path: p2 }) => stringEq((id.clone()).clone(), (id2.clone()).clone()) && pathPrefixOf(p.clone(), p2.clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isPrefix
}

pub fn pathReplaceFirst(path: Arc<Absyn::Path>, prefix: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => prefix.clone(),
        Deref @ Absyn::Path::QUALIFIED { .. } => joinPaths(prefix.clone(), var_field!((*path).path, Absyn::Path::QUALIFIED).clone())?,
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => Arc::new(Absyn::Path::FULLYQUALIFIED { path: pathReplaceFirst(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), prefix.clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn pathRest(inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::QUALIFIED { path: outPath, .. } => outPath.clone(),
        Deref @ Absyn::FULLYQUALIFIED { path: outPath } => pathRest(outPath.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathSecondIdent(inPath: Arc<Absyn::Path>) -> Result<ArcStr> {
    let mut outIdent: ArcStr;
    outIdent = ((::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::QUALIFIED { path: Deref @ Absyn::QUALIFIED { name: n, .. }, .. } => n.clone(),
        Deref @ Absyn::QUALIFIED { path: Deref @ Absyn::IDENT { name: n }, .. } => n.clone(),
        Deref @ Absyn::FULLYQUALIFIED { path: p } => pathSecondIdent(p.clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

pub fn pathSetFirstIdent(path: Arc<Absyn::Path>, ident: ArcStr) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::IDENT { .. } => Arc::new(Absyn::Path::IDENT { name: (ident.clone()).clone() }),
        Deref @ Absyn::QUALIFIED { .. } => Arc::new(Absyn::Path::QUALIFIED { name: (ident.clone()).clone(), path: var_field!((*path).path, Absyn::Path::QUALIFIED).clone() }),
        Deref @ Absyn::FULLYQUALIFIED { .. } => Arc::new(Absyn::Path::FULLYQUALIFIED { path: pathSetFirstIdent(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), (ident.clone()).clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn pathSetLastIdent(path: Arc<Absyn::Path>, ident: ArcStr) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::IDENT { .. } => Arc::new(Absyn::Path::IDENT { name: (ident.clone()).clone() }),
        Deref @ Absyn::QUALIFIED { .. } => Arc::new(Absyn::Path::QUALIFIED { name: (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone(), path: pathSetLastIdent(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), (ident.clone()).clone())? }),
        Deref @ Absyn::FULLYQUALIFIED { .. } => Arc::new(Absyn::Path::FULLYQUALIFIED { path: pathSetLastIdent(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), (ident.clone()).clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn pathSetNthIdent(path: Arc<Absyn::Path>, ident: ArcStr, n: i32) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    if n.clone() == 1 {
        outPath = pathSetFirstIdent(path.clone(), (ident.clone()).clone())?;
    } else {
        outPath = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::QUALIFIED { .. } => Arc::new(Absyn::Path::QUALIFIED { name: (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone(), path: pathSetNthIdent(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), (ident.clone()).clone(), n.clone() - 1)? }),
        Deref @ Absyn::FULLYQUALIFIED { .. } => Arc::new(Absyn::Path::FULLYQUALIFIED { path: pathSetNthIdent(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), (ident.clone()).clone(), n.clone())? }),
        _ => bail!("match: no arm matched"),
    } });
    }
    Ok(outPath)
}

pub fn pathString(path: Arc<Absyn::Path>, delimiter: ArcStr, usefq: bool, reverse: bool) -> Result<ArcStr> {
    let mut s: ArcStr;
    let mut p1: Arc<Absyn::Path>;
    let mut p2: Arc<Absyn::Path>;
    let mut count: i32 = 0;
    let mut len: i32 = 0;
    let mut dlen: i32 = ((delimiter.clone()).clone().len() as i32);
    let mut b: bool;
    p1 = if (usefq.clone()) {path.clone()} else {makeNotFullyQualified(path.clone())};
    let _ = (::match_deref::match_deref! { match &(p1.clone()) {
        Deref @ Absyn::IDENT { .. } => {
            s = (var_field!((*p1).name, Absyn::Path::IDENT).clone()).clone();
            return Ok(s);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    p2 = p1.clone();
    b = true;
    while b.clone() {
        (p2, len, count, b) = (::match_deref::match_deref! { match &(p2.clone()) {
        Deref @ Absyn::IDENT { .. } => (p2.clone(), len.clone() + 1, count.clone() + ((var_field!((*p2).name, Absyn::Path::IDENT).clone()).clone().len() as i32), false),
        Deref @ Absyn::QUALIFIED { .. } => (var_field!((*p2).path, Absyn::Path::QUALIFIED).clone(), len.clone() + 1, count.clone() + ((var_field!((*p2).name, Absyn::Path::QUALIFIED).clone()).clone().len() as i32), true),
        Deref @ Absyn::FULLYQUALIFIED { .. } => (var_field!((*p2).path, Absyn::Path::FULLYQUALIFIED).clone(), len.clone() + 1, count.clone(), true),
        _ => bail!("match: no arm matched"),
    } });
    }
    s = (pathStringWork(p1.clone(), len.clone() - 1 * dlen.clone() + count.clone(), (delimiter.clone()).clone(), dlen.clone(), reverse.clone())?).clone();
    Ok(s)
}

pub fn pathStringDefault(path: Arc<Absyn::Path>) -> ArcStr {
    let mut s: ArcStr = pathString(path.clone(), (literal!(".")).clone(), true, false).unwrap();
    s
}

// pub fn pathStringNoQual = pathString(usefq=false) -- function alias with default-arg modifications not yet supported
pub fn pathStringNoQual() { todo!("function alias pathStringNoQual = pathString(usefq=false)") }

pub fn pathStringUnquoteReplaceDot(inPath: Arc<Absyn::Path>, repStr: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr;
    let mut strlst: Arc<metamodelica::List<ArcStr>>;
    let mut rep_rep: ArcStr;
    rep_rep = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*repStr.clone()); __mm_s.push_str(&*repStr.clone()); ArcStr::from(__mm_s) }).clone();
    strlst = pathToStringList(inPath.clone())?;
    strlst = List::map2(strlst.clone(), Arc::new(System::stringReplace), (repStr.clone()).clone(), (rep_rep.clone()).clone());
    strlst = List::map(strlst.clone(), Arc::new(fnptr!(System::unquoteIdentifier, ArcStr)));
    outString = stringDelimitList(strlst.clone(), (repStr.clone()).clone());
    Ok(outString)
}

fn pathStringWork(inPath: Arc<Absyn::Path>, len: i32, delimiter: ArcStr, dlen: i32, reverse: bool) -> Result<ArcStr> {
    let mut s: ArcStr = literal!("");
    let mut p: Arc<Absyn::Path> = inPath.clone();
    let mut b: bool = true;
    let mut count: i32 = 0;
    let mut sb: System::StringAllocator = System::StringAllocator(len.clone())?;
    while b.clone() {
        (p, count, b) = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::IDENT { .. } => {
            System::stringAllocatorStringCopy(sb.clone(), (var_field!((*p).name, Absyn::Path::IDENT).clone()).clone(), if (reverse.clone()) {len.clone() - count.clone() - ((var_field!((*p).name, Absyn::Path::IDENT).clone()).clone().len() as i32)} else {count.clone()});
            (p.clone(), count.clone() + ((var_field!((*p).name, Absyn::Path::IDENT).clone()).clone().len() as i32), false)
        },
        Deref @ Absyn::QUALIFIED { .. } => {
            System::stringAllocatorStringCopy(sb.clone(), (var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone(), if (reverse.clone()) {len.clone() - count.clone() - dlen.clone() - ((var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone().len() as i32)} else {count.clone()});
            System::stringAllocatorStringCopy(sb.clone(), (delimiter.clone()).clone(), if (reverse.clone()) {len.clone() - count.clone() - dlen.clone()} else {count.clone() + ((var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone().len() as i32)});
            (var_field!((*p).path, Absyn::Path::QUALIFIED).clone(), count.clone() + ((var_field!((*p).name, Absyn::Path::QUALIFIED).clone()).clone().len() as i32) + dlen.clone(), true)
        },
        Deref @ Absyn::FULLYQUALIFIED { .. } => {
            System::stringAllocatorStringCopy(sb.clone(), (delimiter.clone()).clone(), if (reverse.clone()) {len.clone() - count.clone() - dlen.clone()} else {count.clone()});
            (var_field!((*p).path, Absyn::Path::FULLYQUALIFIED).clone(), count.clone() + dlen.clone(), true)
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    s = (System::stringAllocatorResult(sb.clone(), (s.clone()).clone())).clone();
    Ok(s)
}

pub fn pathStripSamePrefix(inPath1: Arc<Absyn::Path>, inPath2: Arc<Absyn::Path>) -> Result<Option<Arc<Absyn::Path>>> {
    let mut outPath: Option<Arc<Absyn::Path>>;
    let mut path1: Arc<Absyn::Path> = inPath1.clone();
    let mut path2: Arc<Absyn::Path> = inPath2.clone();
    while pathFirstIdent(path1.clone())? == pathFirstIdent(path2.clone())? {
        if pathIsIdent(path1.clone()) {
            outPath = None;
            return Ok(outPath);
        }
        path1 = pathRest(path1.clone())?;
        if pathIsIdent(path2.clone()) {
            break;
        }
        path2 = pathRest(path2.clone())?;
    }
    outPath = Some(path1.clone());
    Ok(outPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathSuffixOf(suffix_path: Arc<Absyn::Path>, path: Arc<Absyn::Path>) -> Result<bool> {
    let mut res: bool;
    res = 'mc: {
        let __mc_input = (suffix_path.clone(), path.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut p: Arc<Absyn::Path>;
                    let true = (pathEqual(suffix_path.clone(), path.clone())) else { bail!("pattern mismatch") };
                    Ok(true)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::FULLYQUALIFIED { path: p }) => {
                    Ok(pathSuffixOf(suffix_path.clone(), p.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::QUALIFIED { path: p, .. }) => {
                    Ok(pathSuffixOf(suffix_path.clone(), p.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut p: Arc<Absyn::Path>;
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(res)
}

pub fn pathSuffixOfr(path: Arc<Absyn::Path>, suffix_path: Arc<Absyn::Path>) -> Result<bool> {
    let mut res: bool;
    res = pathSuffixOf(suffix_path.clone(), path.clone())?;
    Ok(res)
}

pub fn pathToCref(inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outComponentRef: Arc<Absyn::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::IDENT { name: i } => Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (i.clone()).clone(), subscripts: metamodelica::nil() }),
        Deref @ Absyn::QUALIFIED { path: p, name: i } => {
            let mut c: Arc<Absyn::ComponentRef>;
            c = pathToCref(p.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (i.clone()).clone(), subscripts: metamodelica::nil(), componentRef: c.clone() })
        },
        Deref @ Absyn::FULLYQUALIFIED { path: p } => {
            let mut i: ArcStr;
            let mut c: Arc<Absyn::ComponentRef>;
            c = pathToCref(p.clone())?;
            crefMakeFullyQualified(c.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn pathToCrefWithSubs(inPath: Arc<Absyn::Path>, inSubs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> Result<Arc<Absyn::ComponentRef>> {
    let mut outComponentRef: Arc<Absyn::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &((inPath.clone(), inSubs.clone())) {
        (Deref @ Absyn::IDENT { name: i }, _) => Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (i.clone()).clone(), subscripts: inSubs.clone() }),
        (Deref @ Absyn::QUALIFIED { path: p, name: i }, _) => {
            let mut c: Arc<Absyn::ComponentRef>;
            c = pathToCrefWithSubs(p.clone(), inSubs.clone())?;
            Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (i.clone()).clone(), subscripts: metamodelica::nil(), componentRef: c.clone() })
        },
        (Deref @ Absyn::FULLYQUALIFIED { path: p }, _) => {
            let mut i: ArcStr;
            let mut c: Arc<Absyn::ComponentRef>;
            c = pathToCrefWithSubs(p.clone(), inSubs.clone())?;
            crefMakeFullyQualified(c.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub fn pathToStringList(path: Arc<Absyn::Path>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outPaths: Arc<metamodelica::List<ArcStr>>;
    outPaths = pathToStringListReverse(path.clone(), metamodelica::nil())?.reverse();
    Ok(outPaths)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn pathToStringListReverse(path: Arc<Absyn::Path>, acc: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut outPaths: Arc<metamodelica::List<ArcStr>>;
    outPaths = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::IDENT { .. } => cons(var_field!((*path).name, Absyn::Path::IDENT).clone(), acc.clone()),
        Deref @ Absyn::QUALIFIED { .. } => pathToStringListReverse(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), cons(var_field!((*path).name, Absyn::Path::QUALIFIED).clone(), acc.clone()))?,
        Deref @ Absyn::FULLYQUALIFIED { .. } => pathToStringListReverse(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), acc.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPaths)
}

pub fn pathToTypeSpec(inPath: Arc<Absyn::Path>) -> Arc<Absyn::TypeSpec> {
    let mut outTypeSpec: Arc<Absyn::TypeSpec>;
    outTypeSpec = Arc::new(Absyn::TypeSpec::TPATH { path: inPath.clone(), arrayDim: None });
    outTypeSpec
}

pub fn prefixPath(prefix: ArcStr, path: Arc<Absyn::Path>) -> Arc<Absyn::Path> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = Arc::new(Absyn::Path::QUALIFIED { name: (prefix.clone()).clone(), path: path.clone() });
    outPath
}

pub fn printImportString(imp: Absyn::Import) -> Result<ArcStr> {
    let mut ostring: ArcStr;
    ostring = ((match imp.clone() {
        Absyn::NAMED_IMPORT { .. } => var_field!(imp.name, Absyn::Import::NAMED_IMPORT).clone(),
        Absyn::QUAL_IMPORT { .. } => pathString(var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(), (literal!(".")).clone(), true, false)?,
        Absyn::UNQUAL_IMPORT { .. } => pathString(var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT).clone(), (literal!(".")).clone(), true, false)?,
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(ostring)
}

pub fn purityEqual(purity1: Absyn::FunctionPurity, purity2: Absyn::FunctionPurity, defaultImpure: bool) -> bool {
    let mut isEqual: bool;
    if valueConstructor(purity1.clone()) == valueConstructor(purity2.clone()) {
        isEqual = true;
    } else if defaultImpure.clone() {
        isEqual = (match (purity1.clone(), purity2.clone()) {
        (Absyn::FunctionPurity::NO_PURITY { .. }, Absyn::FunctionPurity::IMPURE { .. }) => true,
        (Absyn::FunctionPurity::IMPURE { .. }, Absyn::FunctionPurity::NO_PURITY { .. }) => true,
        _ => false,
    });
    } else {
        isEqual = (match (purity1.clone(), purity2.clone()) {
        (Absyn::FunctionPurity::NO_PURITY { .. }, Absyn::FunctionPurity::PURE { .. }) => true,
        (Absyn::FunctionPurity::PURE { .. }, Absyn::FunctionPurity::NO_PURITY { .. }) => true,
        _ => false,
    });
    }
    isEqual
}

pub fn refString(inRef: Absyn::Ref) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((match inRef.clone() {
        Absyn::RCR { .. } => crefString(var_field!(inRef.cr, Absyn::Ref::RCR).clone())?,
        Absyn::RTS { .. } => typeSpecString(var_field!(inRef.ts, Absyn::Ref::RTS).clone())?,
        Absyn::RIM { .. } => importString(var_field!(inRef.im, Absyn::Ref::RIM).clone())?,
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outStr)
}

pub fn refStringBrief(inRef: Absyn::Ref) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((match inRef.clone() {
        Absyn::RCR { .. } => crefStringIgnoreSubs(var_field!(inRef.cr, Absyn::Ref::RCR).clone())?,
        Absyn::RTS { .. } => typeSpecStringNoQualNoDims(var_field!(inRef.ts, Absyn::Ref::RTS).clone())?,
        Absyn::RIM { .. } => importString(var_field!(inRef.im, Absyn::Ref::RIM).clone())?,
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outStr)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn removeCrefFromCrefs(inAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>, inComponentRef: Arc<Absyn::ComponentRef>) -> Result<Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>> {
    let mut outAbsynComponentRefLst: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
    outAbsynComponentRefLst = 'mc: {
        let __mc_input = (inAbsynComponentRefLst.clone(), inComponentRef.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, _) => {
                    let mut n1: ArcStr;
                    let mut n2: ArcStr;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ComponentRef>>>;
                    let mut cr1: Arc<Absyn::ComponentRef>;
                    let mut cr2: Arc<Absyn::ComponentRef>;
                    Ok(metamodelica::nil())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr1, tail: rest }, cr2) => {
                    let mut n1: ArcStr;
                    let mut n2: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(cr1.clone()) {
                        Deref @ Absyn::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: __pa0 } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    n1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(cr2.clone()) {
                        Deref @ Absyn::CREF_IDENT { subscripts: Deref @ metamodelica::List::Nil, name: __pa1 } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    n2 = __pa1.clone();
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(removeCrefFromCrefs(rest.clone(), cr2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr1, tail: rest }, cr2) => {
                    let mut n1: ArcStr;
                    let mut n2: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(cr1.clone()) {
                        Deref @ Absyn::CREF_QUAL { name: __pa0, .. } => __pa0.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    n1 = __pa0.clone();
                    let __pa1 = ::match_deref::match_deref! { match &(cr2.clone()) {
                        Deref @ Absyn::CREF_IDENT { name: __pa1, .. } => __pa1.clone(),
                        _ => bail!("pattern mismatch"),
                    } };
                    n2 = __pa1.clone();
                    let true = (stringEq((n1.clone()).clone(), (n2.clone()).clone())) else { bail!("pattern mismatch") };
                    Ok(removeCrefFromCrefs(rest.clone(), cr2.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: cr1, tail: rest }, cr2) => {
                    let mut n1: ArcStr;
                    let mut n2: ArcStr;
                    let mut rest = (*rest).clone();
                    rest = removeCrefFromCrefs(rest.clone(), cr2.clone())?;
                    Ok(cons(cr1.clone(), rest.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outAbsynComponentRefLst)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn removePartialPrefix(inPrefix: Arc<Absyn::Path>, inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = 'mc: {
        let __mc_input = inPrefix.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(removePrefix(inPrefix.clone(), inPath.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::QUALIFIED { .. } => {
                    Ok(removePrefix(var_field!((*inPrefix).path, Absyn::Path::QUALIFIED).clone(), inPath.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FULLYQUALIFIED { .. } => {
                    Ok(removePartialPrefix(var_field!((*inPrefix).path, Absyn::Path::FULLYQUALIFIED).clone(), inPath.clone())?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inPath.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn removePrefix(prefix_path: Arc<Absyn::Path>, path: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut newPath: Arc<Absyn::Path>;
    newPath = (::match_deref::match_deref! { match &((prefix_path.clone(), path.clone())) {
        (p, Deref @ Absyn::FULLYQUALIFIED { path: p2 }) => removePrefix(p.clone(), p2.clone())?,
        (Deref @ Absyn::QUALIFIED { path: p, name: id1 }, Deref @ Absyn::QUALIFIED { path: p2, name: id2 }) => {
            let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
            removePrefix(p.clone(), p2.clone())?
        },
        (Deref @ Absyn::IDENT { name: id1 }, Deref @ Absyn::QUALIFIED { path: p2, name: id2 }) => {
            let mut p: Arc<Absyn::Path>;
            let true = (stringEq((id1.clone()).clone(), (id2.clone()).clone())) else { bail!("pattern mismatch") };
            p2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(newPath)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn removePrefixOpt(prefixPath: Arc<Absyn::Path>, path: Arc<Absyn::Path>) -> Option<Arc<Absyn::Path>> {
    let mut outPath: Option<Arc<Absyn::Path>>;
    outPath = (::match_deref::match_deref! { match &((prefixPath.clone(), path.clone())) {
        (_, Deref @ Absyn::FULLYQUALIFIED { .. }) => removePrefixOpt(prefixPath.clone(), var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone()),
        (Deref @ Absyn::QUALIFIED { .. }, Deref @ Absyn::QUALIFIED { .. }) if (var_field!((*prefixPath).name, Absyn::Path::QUALIFIED).clone() == var_field!((*path).name, Absyn::Path::QUALIFIED).clone()) => removePrefixOpt(prefixPath::path.clone(), var_field!((*path).path, Absyn::Path::QUALIFIED).clone()),
        (Deref @ Absyn::IDENT { .. }, Deref @ Absyn::QUALIFIED { .. }) if (var_field!((*prefixPath).name, Absyn::Path::IDENT).clone() == var_field!((*path).name, Absyn::Path::QUALIFIED).clone()) => Some(var_field!((*path).path, Absyn::Path::QUALIFIED).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outPath
}

pub fn restrString(inRestriction: Absyn::Restriction) -> ArcStr {
    let mut outString: ArcStr;
    outString = ((match inRestriction.clone() {
        Absyn::R_CLASS => literal!("CLASS"),
        Absyn::R_OPTIMIZATION => literal!("OPTIMIZATION"),
        Absyn::R_MODEL => literal!("MODEL"),
        Absyn::R_RECORD => literal!("RECORD"),
        Absyn::R_BLOCK => literal!("BLOCK"),
        Absyn::R_CONNECTOR => literal!("CONNECTOR"),
        Absyn::R_EXP_CONNECTOR => literal!("EXPANDABLE CONNECTOR"),
        Absyn::R_TYPE => literal!("TYPE"),
        Absyn::R_PACKAGE => literal!("PACKAGE"),
        Absyn::R_FUNCTION { functionRestriction: Absyn::FR_NORMAL_FUNCTION { purity: Absyn::PURE } } => literal!("PURE FUNCTION"),
        Absyn::R_FUNCTION { functionRestriction: Absyn::FR_NORMAL_FUNCTION { purity: Absyn::IMPURE } } => literal!("IMPURE FUNCTION"),
        Absyn::R_FUNCTION { functionRestriction: Absyn::FR_NORMAL_FUNCTION { purity: Absyn::NO_PURITY } } => literal!("FUNCTION"),
        Absyn::R_FUNCTION { functionRestriction: Absyn::FR_OPERATOR_FUNCTION } => literal!("OPERATOR FUNCTION"),
        Absyn::R_PREDEFINED_INTEGER => literal!("PREDEFINED_INT"),
        Absyn::R_PREDEFINED_REAL => literal!("PREDEFINED_REAL"),
        Absyn::R_PREDEFINED_STRING => literal!("PREDEFINED_STRING"),
        Absyn::R_PREDEFINED_BOOLEAN => literal!("PREDEFINED_BOOL"),
        Absyn::R_PREDEFINED_CLOCK => literal!("PREDEFINED_CLOCK"),
        Absyn::R_UNIONTYPE => literal!("UNIONTYPE"),
        _ => literal!("* Unknown restriction *"),
    })).clone();
    outString
}

pub fn setClassAnnotation(cls: Arc<Absyn::Class>, ann: Option<Arc<Absyn::Annotation>>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    assign_field!(cls.body = setClassDefAnnotation(cls.body.clone(), ann.clone())?);
    Ok(cls)
}

pub fn setClassBody(inClass: Arc<Absyn::Class>, inBody: Arc<Absyn::ClassDef>) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    outClass = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::CLASS { .. } => {
            assign_variant_field!(outClass => Absyn::Class::CLASS; body = inBody.clone());
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

pub fn setClassCommentsAfterEnd(cl: Arc<Absyn::Class>, comments: Arc<metamodelica::List<ArcStr>>) -> Arc<Absyn::Class> {
    let mut cl: Arc<Absyn::Class> = cl;
    assign_field!(cl.commentsAfterEnd = comments.clone());
    cl
}

pub fn setClassDefAnnotation(cdef: Arc<Absyn::ClassDef>, ann: Option<Arc<Absyn::Annotation>>) -> Result<Arc<Absyn::ClassDef>> {
    let mut cdef: Arc<Absyn::ClassDef> = cdef;
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            if !(var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone().is_empty()) {
                assign_variant_field!(cdef => Absyn::ClassDef::PARTS; ann = listRest(var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone())?);
            }
            if isSome(ann.clone()) {
                assign_variant_field!(cdef => Absyn::ClassDef::PARTS; ann = cons(Util::getOption(ann.clone())?, var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone()));
            }
            ()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED; comment = setCommentAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::DERIVED).clone(), ann.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::ENUMERATION; comment = setCommentAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::ENUMERATION).clone(), ann.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::OVERLOAD { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::OVERLOAD; comment = setCommentAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::OVERLOAD).clone(), ann.clone())?);
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            if !(var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone().is_empty()) {
                assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; ann = listRest(var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone())?);
            }
            if isSome(ann.clone()) {
                assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; ann = cons(Util::getOption(ann.clone())?, var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone()));
            }
            ()
        },
        Deref @ Absyn::ClassDef::PDER { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PDER; comment = setCommentAnnotation(var_field!((*cdef).comment, Absyn::ClassDef::PDER).clone(), ann.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cdef)
}

pub fn setClassDefType(cdef: Arc<Absyn::ClassDef>, typeSpec: Arc<Absyn::TypeSpec>) -> Result<Arc<Absyn::ClassDef>> {
    let mut cdef: Arc<Absyn::ClassDef> = cdef;
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::DERIVED; typeSpec = typeSpec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(cdef)
}

pub fn setClassFilename(inClass: Arc<Absyn::Class>, fileName: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class>;
    outClass = (::match_deref::match_deref! { match &(inClass.clone()) {
        cl @ Deref @ Absyn::CLASS { info: info @ SourceInfo { .. }, .. } => {
            let mut cl = (*cl).clone();
            let mut info = (*info).clone();
            info.fileName = (fileName.clone()).clone(); // TODO: unhandled field-assign shape
            assign_variant_field!(cl => Absyn::Class::CLASS; info = info.clone());
            cl.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

pub fn setClassName(inClass: Arc<Absyn::Class>, newName: ArcStr) -> Result<Arc<Absyn::Class>> {
    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    outClass = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::CLASS { .. } => {
            assign_variant_field!(outClass => Absyn::Class::CLASS; name = newName.clone());
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outClass)
}

pub fn setClassPartsInClass(parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, cls: Arc<Absyn::Class>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    let mut cdef: Arc<Absyn::ClassDef> = cls.body.clone();
    let () = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
            ()
        },
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    assign_field!(cls.body = cdef.clone());
    Ok(cls)
}

pub fn setClassType(cls: Arc<Absyn::Class>, typeSpec: Arc<Absyn::TypeSpec>) -> Result<Arc<Absyn::Class>> {
    let mut cls: Arc<Absyn::Class> = cls;
    assign_field!(cls.body = setClassDefType(cls.body.clone(), typeSpec.clone())?);
    Ok(cls)
}

pub fn setCommentAnnotation(comment: Option<Arc<Absyn::Comment>>, ann: Option<Arc<Absyn::Annotation>>) -> Result<Option<Arc<Absyn::Comment>>> {
    let mut comment: Option<Arc<Absyn::Comment>> = comment;
    let mut old_ann: Option<Arc<Absyn::Annotation>>;
    let mut r#str: Option<ArcStr>;
    if isSome(comment.clone()) {
        let Some(Absyn::COMMENT { annotation_: __pa0, comment: __pa1 }) = (comment.clone()) else { bail!("pattern mismatch") };
        old_ann = __pa0.clone();
        r#str = __pa1.clone();
        comment = if (isSome(ann.clone()) || isSome(r#str.clone())) {Some(Arc::new(Absyn::Comment { annotation_: ann.clone(), comment: r#str.clone() }))} else {None};
    } else if isSome(ann.clone()) {
        comment = Some(Arc::new(Absyn::Comment { annotation_: ann.clone(), comment: None }));
    }
    Ok(comment)
}

pub fn setCommentString(comment: Option<Arc<Absyn::Comment>>, commentString: Option<ArcStr>) -> Result<Option<Arc<Absyn::Comment>>> {
    let mut comment: Option<Arc<Absyn::Comment>> = comment;
    let mut ann: Option<Arc<Absyn::Annotation>>;
    let mut r#str: Option<ArcStr>;
    let mut new_str: Option<ArcStr>;
    if isSome(comment.clone()) {
        let Some(Absyn::COMMENT { annotation_: __pa0, comment: __pa1 }) = (comment.clone()) else { bail!("pattern mismatch") };
        ann = __pa0.clone();
        r#str = __pa1.clone();
        comment = if (isSome(ann.clone()) || isSome(r#str.clone())) {Some(Arc::new(Absyn::Comment { annotation_: ann.clone(), comment: commentString.clone() }))} else {None};
    } else if isSome(commentString.clone()) {
        comment = Some(Arc::new(Absyn::Comment { annotation_: None, comment: commentString.clone() }));
    }
    Ok(comment)
}

pub fn setComponentItemAnnotation(item: Arc<Absyn::ComponentItem>, inAnnotation: Option<Arc<Absyn::Annotation>>) -> Result<Arc<Absyn::ComponentItem>> {
    let mut item: Arc<Absyn::ComponentItem> = item;
    assign_field!(item.comment = setCommentAnnotation(item.comment.clone(), inAnnotation.clone())?);
    Ok(item)
}

pub fn setElementAnnotation(element: Arc<Absyn::Element>, name: ArcStr, inAnnotation: Option<Arc<Absyn::Annotation>>) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = setElementSpecAnnotation(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), (name.clone()).clone(), inAnnotation.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub fn setElementSpecAnnotation(spec: Arc<Absyn::ElementSpec>, name: ArcStr, inAnnotation: Option<Arc<Absyn::Annotation>>) -> Result<Arc<Absyn::ElementSpec>> {
    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let mut cls: Arc<Absyn::Class>;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = setClassAnnotation(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), inAnnotation.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::EXTENDS; annotationOpt = inAnnotation.clone());
            ()
        },
        Deref @ Absyn::ElementSpec::IMPORT { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::IMPORT; comment = setCommentAnnotation(var_field!((*spec).comment, Absyn::ElementSpec::IMPORT).clone(), inAnnotation.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; components = List::findAndMap(var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone(), Arc::new({ let __pe_b0 = name.clone(); move |__pe_a1| Ok(isComponentItemNamed(__pe_b0.clone(), __pe_a1)) }), Arc::new({ let __pe_b1 = inAnnotation.clone(); move |__pe_a0| setComponentItemAnnotation(__pe_a0, __pe_b1.clone()) }))?.0);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(spec)
}

pub fn setElementSpecType(spec: Arc<Absyn::ElementSpec>, typeSpec: Arc<Absyn::TypeSpec>, allowMultipleComponents: bool) -> Result<Arc<Absyn::ElementSpec>> {
    let mut spec: Arc<Absyn::ElementSpec> = spec;
    let mut cls: Arc<Absyn::Class>;
    let () = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => {
            assign_variant_field!(spec => Absyn::ElementSpec::CLASSDEF; class_ = setClassType(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), typeSpec.clone())?);
            ()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } if (allowMultipleComponents.clone() || (var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone().len() as i32) == 1) => {
            assign_variant_field!(spec => Absyn::ElementSpec::COMPONENTS; typeSpec = typeSpec.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(spec)
}

pub fn setElementType(element: Arc<Absyn::Element>, typeSpec: Arc<Absyn::TypeSpec>, allowMultipleComponents: bool) -> Result<Arc<Absyn::Element>> {
    let mut element: Arc<Absyn::Element> = element;
    let () = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            assign_variant_field!(element => Absyn::Element::ELEMENT; specification = setElementSpecType(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), typeSpec.clone(), allowMultipleComponents.clone())?);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(element)
}

pub fn setEquationItemsInPart(eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, part: Arc<Absyn::ClassPart>) -> Result<Arc<Absyn::ClassPart>> {
    let mut part: Arc<Absyn::ClassPart> = part;
    let () = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::EQUATIONS; contents = eqs.clone());
            ()
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            assign_variant_field!(part => Absyn::ClassPart::INITIALEQUATIONS; contents = eqs.clone());
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(part)
}

pub fn setImportPath(imp: Absyn::Import, path: Arc<Absyn::Path>) -> Result<Absyn::Import> {
    let mut imp: Absyn::Import = imp;
    let () = (match imp.clone() {
        Absyn::NAMED_IMPORT { .. } => {
            let __owned_variant_path_0 = path.clone();
            if let Absyn::Import::NAMED_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::NAMED_IMPORT"); }
            ()
        },
        Absyn::QUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = path.clone();
            if let Absyn::Import::QUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::QUAL_IMPORT"); }
            ()
        },
        Absyn::UNQUAL_IMPORT { .. } => {
            let __owned_variant_path_0 = path.clone();
            if let Absyn::Import::UNQUAL_IMPORT { path, .. } = &mut imp {
                *path = __owned_variant_path_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::UNQUAL_IMPORT"); }
            ()
        },
        Absyn::GROUP_IMPORT { .. } => {
            let __owned_variant_prefix_0 = path.clone();
            if let Absyn::Import::GROUP_IMPORT { prefix, .. } = &mut imp {
                *prefix = __owned_variant_prefix_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Import::GROUP_IMPORT"); }
            ()
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(imp)
}

pub fn splitQualAndIdentPath(inPath: Arc<Absyn::Path>) -> Result<(Arc<Absyn::Path>, Arc<Absyn::Path>)> {
    let mut outPath1: Arc<Absyn::Path>;
    let mut outPath2: Arc<Absyn::Path>;
    (outPath1, outPath2) = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::QUALIFIED { path: Deref @ Absyn::IDENT { name: s2 }, name: s1 } => (Arc::new(Absyn::Path::IDENT { name: (s1.clone()).clone() }), Arc::new(Absyn::Path::IDENT { name: (s2.clone()).clone() })),
        Deref @ Absyn::QUALIFIED { path: qPath, name: s1 } => {
            let mut curPath: Arc<Absyn::Path>;
            let mut identPath: Arc<Absyn::Path>;
            let mut s2: ArcStr;
            (curPath, identPath) = splitQualAndIdentPath(qPath.clone())?;
            (Arc::new(Absyn::Path::QUALIFIED { name: (s1.clone()).clone(), path: curPath.clone() }), identPath.clone())
        },
        Deref @ Absyn::FULLYQUALIFIED { path: qPath } => {
            let mut curPath: Arc<Absyn::Path>;
            let mut identPath: Arc<Absyn::Path>;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            (curPath, identPath) = splitQualAndIdentPath(qPath.clone())?;
            (curPath.clone(), identPath.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outPath1, outPath2))
}

pub fn stringListPath(paths: Arc<metamodelica::List<ArcStr>>) -> Arc<Absyn::Path> {
    let mut qualifiedPath: Arc<Absyn::Path> = stringListPathReversed(paths.clone().reverse()).unwrap();
    qualifiedPath
}

pub fn stringListPathReversed(inStrings: Arc<metamodelica::List<ArcStr>>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    let mut id: ArcStr;
    let mut rest_str: Arc<metamodelica::List<ArcStr>>;
    let mut path: Arc<Absyn::Path>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inStrings.clone()) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    id = __pa0.clone();
    rest_str = __pa1.clone();
    outPath = Arc::new(Absyn::Path::IDENT { name: (id.clone()).clone() });
    for s in &*rest_str.clone() {
        outPath = Arc::new(Absyn::Path::QUALIFIED { name: (s.clone()).clone(), path: outPath.clone() });
    }
    Ok(outPath)
}

pub fn stringPath(r#str: ArcStr) -> Result<Arc<Absyn::Path>> {
    let mut qualifiedPath: Arc<Absyn::Path>;
    let mut paths: Arc<metamodelica::List<ArcStr>>;
    paths = Util::stringSplitAtChar((r#str.clone()).clone(), (literal!(".")).clone())?;
    qualifiedPath = stringListPath(paths.clone());
    Ok(qualifiedPath)
}

fn stripClassDefComment(cl: Arc<Absyn::ClassDef>) -> Arc<Absyn::ClassDef> {
    let mut cl: Arc<Absyn::ClassDef> = cl;
    let () = (::match_deref::match_deref! { match &(cl.clone()) {
        Deref @ Absyn::PARTS { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::PARTS; comment = None);
            ()
        },
        Deref @ Absyn::DERIVED { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::DERIVED; comment = None);
            ()
        },
        Deref @ Absyn::ENUMERATION { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::ENUMERATION; comment = None);
            ()
        },
        Deref @ Absyn::OVERLOAD { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::OVERLOAD; comment = None);
            ()
        },
        Deref @ Absyn::CLASS_EXTENDS { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::CLASS_EXTENDS; comment = None);
            ()
        },
        Deref @ Absyn::PDER { .. } => {
            assign_variant_field!(cl => Absyn::ClassDef::PDER; comment = None);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    cl
}

pub fn stripCommentExpressions(exp: Arc<Absyn::Exp>, onlyComments: bool) -> Result<Arc<Absyn::Exp>> {
    let mut exp: Arc<Absyn::Exp> = exp;
    (exp, _) = traverseExp(exp.clone(), Arc::new(fnptr!(stripCommentExpressionsHelper, Arc<Absyn::Exp>, bool)), onlyComments.clone())?;
    Ok(exp)
}

fn stripCommentExpressionsHelper(exp: Arc<Absyn::Exp>, onlyComments: bool) -> (Arc<Absyn::Exp>, bool) {
    let mut exp: Arc<Absyn::Exp> = exp;
    let mut onlyComments: bool = onlyComments;
    let mut e: Arc<Absyn::Exp>;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::TUPLE { expressions: Deref @ metamodelica::List::Cons { head: e, tail: Deref @ metamodelica::List::Nil } } if (!(onlyComments.clone())) => e.clone(),
        Deref @ Absyn::EXPRESSIONCOMMENT { .. } => var_field!((*exp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(),
        _ => exp.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, onlyComments)
}

pub fn stripGraphicsAndInteractionModification(inAbsynElementArgLst: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, Arc<metamodelica::List<Arc<Absyn::ElementArg>>>)> {
    let mut outAbsynElementArgLst1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    let mut outAbsynElementArgLst2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
    (outAbsynElementArgLst1, outAbsynElementArgLst2) = 'mc: {
        let __mc_input = inAbsynElementArgLst.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Nil => {
                    let mut r#mod: Arc<Absyn::ElementArg>;
                    let mut rest: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::MODIFICATION { path: Deref @ Absyn::IDENT { name: Deref @ "interaction" }, .. }, tail: rest } => {
                    let mut r#mod: Arc<Absyn::ElementArg>;
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(rest.clone())?;
                    Ok((l1.clone(), l2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: Deref @ Absyn::MODIFICATION { path: Deref @ Absyn::IDENT { name: Deref @ "graphics" }, modification: None, .. }, tail: rest } => {
                    let mut r#mod: Arc<Absyn::ElementArg>;
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(rest.clone())?;
                    Ok((l1.clone(), l2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#mod @ Deref @ Absyn::MODIFICATION { path: Deref @ Absyn::IDENT { name: Deref @ "graphics" }, modification: Some(_), .. }, tail: rest } => {
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(rest.clone())?;
                    Ok((l1.clone(), cons(r#mod.clone(), l2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#mod @ Deref @ Absyn::MODIFICATION { path: Deref @ Absyn::IDENT { name: Deref @ "choice" }, modification: Some(_), .. }, tail: rest } => {
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(rest.clone())?;
                    Ok((l1.clone(), cons(r#mod.clone(), l2.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::List::Cons { head: r#mod @ Deref @ Absyn::MODIFICATION { .. }, tail: rest } => {
                    let mut l1: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    let mut l2: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
                    (l1, l2) = stripGraphicsAndInteractionModification(rest.clone())?;
                    Ok((cons(r#mod.clone(), l1.clone()), l2.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outAbsynElementArgLst1, outAbsynElementArgLst2))
}

pub fn stripLast(inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &(inPath.clone()) {
        Deref @ Absyn::QUALIFIED { path: Deref @ Absyn::IDENT { .. }, name: r#str } => Arc::new(Absyn::Path::IDENT { name: (r#str.clone()).clone() }),
        Deref @ Absyn::QUALIFIED { path: p, name: r#str } => {
            let mut p = (*p).clone();
            p = stripLast(p.clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (r#str.clone()).clone(), path: p.clone() })
        },
        Deref @ Absyn::FULLYQUALIFIED { path: p } => {
            let mut r#str: ArcStr;
            let mut p = (*p).clone();
            p = stripLast(p.clone())?;
            Arc::new(Absyn::Path::FULLYQUALIFIED { path: p.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn subModsInSameOrder(oldmod: Arc<Absyn::ElementArg>, newmod: Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> {
    let mut r#mod: Arc<Absyn::ElementArg>;
    r#mod = (::match_deref::match_deref! { match &((oldmod.clone(), newmod.clone())) {
        (_, Deref @ Absyn::MODIFICATION { modification: None, .. }) => newmod.clone(),
        (Deref @ Absyn::MODIFICATION { modification: None, .. }, _) => newmod.clone(),
        (Deref @ Absyn::MODIFICATION { modification: Some(Absyn::CLASSMOD { elementArgLst: args1, eqMod: _ }), .. }, arg2 @ Deref @ Absyn::MODIFICATION { modification: Some(Absyn::CLASSMOD { elementArgLst: args2, eqMod: eq2 }), .. }) => {
            let mut res: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>;
            let mut eq1: Arc<Absyn::EqMod>;
            let mut p: Arc<Absyn::Path>;
            let mut arg2 = (*arg2).clone();
            res = metamodelica::nil();
            for arg1 in &*args1.clone() {
                let __pa0 = ::match_deref::match_deref! { match &(arg1.clone()) {
                    Deref @ Absyn::MODIFICATION { path: __pa0, .. } => __pa0.clone(),
                    _ => bail!("pattern mismatch"),
                } };
                p = __pa0.clone();
                if List::any(args2.clone(), Arc::new({ let __pe_b1 = p.clone(); move |__pe_a0| Ok(isModificationOfPath(__pe_a0, __pe_b1.clone())) })) {
                    res = cons(arg1.clone(), res.clone());
                }
            }
            res = res.clone().reverse();
            res = mergeAnnotations2(res.clone(), args2.clone(), false, false)?;
            assign_variant_field!(arg2 => Absyn::ElementArg::MODIFICATION; modification = Some(Arc::new(Absyn::Modification { elementArgLst: res.clone(), eqMod: eq2.clone() })));
            arg2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(r#mod)
}

pub fn subscriptCompare(sub1: Arc<Absyn::Subscript>, sub2: Arc<Absyn::Subscript>) -> Result<i32> {
    let mut comp: i32;
    let mut exp: Arc<Absyn::Exp>;
    if referenceEq(&sub1.clone(),&sub2.clone()) {
        comp = 0;
    }
    comp = Util::intCompare(valueConstructor(sub1.clone()), valueConstructor(sub2.clone()));
    if comp.clone() != 0 {
        return Ok(comp);
    }
    comp = (::match_deref::match_deref! { match &(sub1.clone()) {
        Deref @ Absyn::Subscript::NOSUB { .. } => 0,
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            let __pa0 = ::match_deref::match_deref! { match &(sub2.clone()) {
                Deref @ Absyn::Subscript::SUBSCRIPT { subscript: __pa0 } => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            exp = __pa0.clone();
            stringCompare((Dump::printExpStr(var_field!((*sub1).subscript, Absyn::Subscript::SUBSCRIPT).clone())?).clone(), (Dump::printExpStr(exp.clone())?).clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(comp)
}

pub fn subscriptEqual(inSubscript1: Arc<Absyn::Subscript>, inSubscript2: Arc<Absyn::Subscript>) -> Result<bool> {
    let mut outIsEqual: bool;
    outIsEqual = (::match_deref::match_deref! { match &((inSubscript1.clone(), inSubscript2.clone())) {
        (Deref @ Absyn::NOSUB, Deref @ Absyn::NOSUB) => true,
        (Deref @ Absyn::SUBSCRIPT { subscript: e1 }, Deref @ Absyn::SUBSCRIPT { subscript: e2 }) => expEqual(e1.clone(), e2.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outIsEqual)
}

pub fn subscriptExpOpt(inSub: Arc<Absyn::Subscript>) -> Result<Option<Arc<Absyn::Exp>>> {
    let mut outExpOpt: Option<Arc<Absyn::Exp>>;
    outExpOpt = (::match_deref::match_deref! { match &(inSub.clone()) {
        Deref @ Absyn::SUBSCRIPT { .. } => Some(var_field!((*inSub).subscript, Absyn::Subscript::SUBSCRIPT).clone()),
        Deref @ Absyn::NOSUB => None,
        _ => bail!("match: no arm matched"),
    } });
    Ok(outExpOpt)
}

pub fn subscriptsEqual(inSubList1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, inSubList2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>) -> bool {
    let mut outIsEqual: bool;
    outIsEqual = List::isEqualOnTrue(inSubList1.clone(), inSubList2.clone(), Arc::new(subscriptEqual));
    outIsEqual
}

pub fn suffixPath(inPath: Arc<Absyn::Path>, inSuffix: ArcStr) -> Result<Arc<Absyn::Path>> {
    let mut outPath: Arc<Absyn::Path>;
    outPath = (::match_deref::match_deref! { match &((inPath.clone(), inSuffix.clone())) {
        (Deref @ Absyn::IDENT { name }, _) => Arc::new(Absyn::Path::QUALIFIED { name: (name.clone()).clone(), path: Arc::new(Absyn::Path::IDENT { name: (inSuffix.clone()).clone() }) }),
        (Deref @ Absyn::QUALIFIED { name, path }, _) => {
            let mut path = (*path).clone();
            path = suffixPath(path.clone(), (inSuffix.clone()).clone())?;
            Arc::new(Absyn::Path::QUALIFIED { name: (name.clone()).clone(), path: path.clone() })
        },
        (Deref @ Absyn::FULLYQUALIFIED { path }, _) => {
            let mut name: ArcStr;
            let mut path = (*path).clone();
            path = suffixPath(path.clone(), (inSuffix.clone()).clone())?;
            Arc::new(Absyn::Path::FULLYQUALIFIED { path: path.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outPath)
}

pub fn transformAnnotationArg(ann: Arc<Absyn::Annotation>, path: Arc<Absyn::Path>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> + 'static>, insert: bool) -> Result<Arc<Absyn::Annotation>> {
    pub type Func = fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>>;

    let mut ann: Arc<Absyn::Annotation> = ann;
    assign_field!(ann.elementArgs = transformAnnotationInArgs(ann.elementArgs.clone(), path.clone(), func.clone(), insert.clone())?);
    Ok(ann)
}

pub fn transformAnnotationInArgs(args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>>, path: Arc<Absyn::Path>, r#fn: Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> + 'static>, insert: bool) -> Result<Arc<metamodelica::List<Arc<Absyn::ElementArg>>>> {
    pub type Fn = fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>>;

    fn is_named(arg: Arc<Absyn::ElementArg>, name: ArcStr) -> bool {
        let mut result: bool;
        let mut arg_name: ArcStr;
        result = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { path: Deref @ Absyn::Path::IDENT { name: arg_name }, .. } => name.clone() == arg_name.clone(),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        result
    }

    fn apply_fn(arg: Arc<Absyn::ElementArg>, path: Arc<Absyn::Path>, r#fn: Arc<dyn ::std::ops::Fn(Arc<Absyn::ElementArg>) -> Result<Arc<Absyn::ElementArg>> + 'static>, insert: bool) -> Result<Arc<Absyn::ElementArg>> {
        let mut arg: Arc<Absyn::ElementArg> = arg;
        let mut r#mod: Arc<Absyn::Modification>;
        if pathIsIdent(path.clone()) {
            arg = r#fn(arg.clone())?;
        } else {
            let () = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            if isSome(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                let Some(__pa0) = (var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) else { bail!("pattern mismatch") };
                r#mod = __pa0.clone();
            } else if insert.clone() {
                r#mod = Arc::new(Absyn::Modification { elementArgLst: metamodelica::nil(), eqMod: Arc::new(openmodelica_ast::Absyn::EqMod::NOMOD) });
            } else {
                bail!("fail");
            }
            assign_field!(r#mod.elementArgLst = transformAnnotationInArgs(r#mod.elementArgLst.clone(), pathRest(path.clone())?, r#fn.clone(), insert.clone())?);
            assign_variant_field!(arg => Absyn::ElementArg::MODIFICATION; modification = Some(r#mod.clone()));
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
        }
        Ok(arg)
    }

    let mut args: Arc<metamodelica::List<Arc<Absyn::ElementArg>>> = args;
    let mut name: ArcStr;
    let mut found: bool;
    let mut arg: Arc<Absyn::ElementArg>;
    name = (pathFirstIdent(path.clone())?).clone();
    (args, found) = List::findAndMap(args.clone(), Arc::new({ let __pe_b1 = name.clone(); move |__pe_a0| Ok(is_named(__pe_a0, __pe_b1.clone())) }), Arc::new({ let __pe_b1 = path.clone(); let __pe_b2 = r#fn.clone(); let __pe_b3 = insert.clone(); move |__pe_a0| apply_fn(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone()) }))?;
    if !(found.clone()) {
        if insert.clone() {
            arg = Arc::new(Absyn::ElementArg::MODIFICATION { finalPrefix: false, eachPrefix: openmodelica_ast::Absyn::Each::NON_EACH, path: Arc::new(Absyn::Path::IDENT { name: (name.clone()).clone() }), modification: None, comment: None, info: dummyInfo.clone() });
            arg = apply_fn(arg.clone(), path.clone(), r#fn.clone(), insert.clone())?;
            args = cons(arg.clone(), args.clone());
        } else {
            bail!("fail");
        }
    }
    Ok(args)
}

fn traverseAlgorithmBidir<Arg: Clone + 'static>(alg: Arc<Absyn::Algorithm>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::Algorithm>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut alg: Arc<Absyn::Algorithm> = alg;
    let mut arg: Arg = arg;
    alg = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::ALG_ASSIGN { assignComponent: e1, value: e2 } => {
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            Arc::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: e1.clone(), value: e2.clone() })
        },
        Deref @ Absyn::ALG_IF { ifExp: e1, trueBranch: algs1, elseIfAlgorithmBranch: else_branch, elseBranch: algs2 } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut e1 = (*e1).clone();
            let mut algs1 = (*algs1).clone();
            let mut else_branch = (*else_branch).clone();
            let mut algs2 = (*algs2).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (else_branch, arg) = List::map2FoldCheckReferenceEq(else_branch.clone(), Arc::new(traverseAlgorithmBidirElse), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (algs2, arg) = traverseAlgorithmItemListBidir(algs2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Algorithm::ALG_IF { ifExp: e1.clone(), trueBranch: algs1.clone(), elseIfAlgorithmBranch: else_branch.clone(), elseBranch: algs2.clone() })
        },
        Deref @ Absyn::ALG_FOR { iterators: iters, forBody: algs1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut iters = (*iters).clone();
            let mut algs1 = (*algs1).clone();
            (iters, arg) = List::map2FoldCheckReferenceEq(iters.clone(), Arc::new(traverseExpBidirIterator), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Algorithm::ALG_FOR { iterators: iters.clone(), forBody: algs1.clone() })
        },
        Deref @ Absyn::ALG_PARFOR { iterators: iters, parforBody: algs1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut iters = (*iters).clone();
            let mut algs1 = (*algs1).clone();
            (iters, arg) = List::map2FoldCheckReferenceEq(iters.clone(), Arc::new(traverseExpBidirIterator), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Algorithm::ALG_PARFOR { iterators: iters.clone(), parforBody: algs1.clone() })
        },
        Deref @ Absyn::ALG_WHILE { boolExpr: e1, whileBody: algs1 } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut e1 = (*e1).clone();
            let mut algs1 = (*algs1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Algorithm::ALG_WHILE { boolExpr: e1.clone(), whileBody: algs1.clone() })
        },
        Deref @ Absyn::ALG_WHEN_A { boolExpr: e1, whenBody: algs1, elseWhenAlgorithmBranch: else_branch } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut e1 = (*e1).clone();
            let mut algs1 = (*algs1).clone();
            let mut else_branch = (*else_branch).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (else_branch, arg) = List::map2FoldCheckReferenceEq(else_branch.clone(), Arc::new(traverseAlgorithmBidirElse), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Algorithm::ALG_WHEN_A { boolExpr: e1.clone(), whenBody: algs1.clone(), elseWhenAlgorithmBranch: else_branch.clone() })
        },
        Deref @ Absyn::ALG_NORETCALL { functionCall: cref1, functionArgs: func_args } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut algs1: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut cref1 = (*cref1).clone();
            let mut func_args = (*func_args).clone();
            (cref1, arg) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (func_args, arg) = traverseExpBidirFunctionArgs(func_args.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            Arc::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: cref1.clone(), functionArgs: func_args.clone() })
        },
        Deref @ Absyn::ALG_RETURN => alg.clone(),
        Deref @ Absyn::ALG_BREAK => alg.clone(),
        Deref @ Absyn::ALG_CONTINUE => alg.clone(),
        Deref @ Absyn::ALG_FAILURE { equ: algs1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut algs2: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut algs1 = (*algs1).clone();
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Algorithm::ALG_FAILURE { equ: algs1.clone() })
        },
        Deref @ Absyn::ALG_TRY { body: algs1, elseBody: algs2 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut algs1 = (*algs1).clone();
            let mut algs2 = (*algs2).clone();
            (algs1, arg) = traverseAlgorithmItemListBidir(algs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (algs2, arg) = traverseAlgorithmItemListBidir(algs2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Algorithm::ALG_TRY { body: algs1.clone(), elseBody: algs2.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((alg, arg))
}

fn traverseAlgorithmBidirElse<Arg: Clone + 'static>(inElse: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>), enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>), Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outElse: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>);
    let mut arg: Arg;
    let mut e: Arc<Absyn::Exp>;
    let mut algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
    (e, algs) = inElse.clone();
    (e, arg) = traverseExpBidir(e.clone(), enterFunc.clone(), exitFunc.clone(), inArg.clone())?;
    (algs, arg) = traverseAlgorithmItemListBidir(algs.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
    outElse = (e.clone(), algs.clone());
    Ok((outElse, arg))
}

fn traverseAlgorithmItemBidir<Arg: Clone + 'static>(algorithmItem: Arc<Absyn::AlgorithmItem>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::AlgorithmItem>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut algorithmItem: Arc<Absyn::AlgorithmItem> = algorithmItem;
    let mut arg: Arg = arg;
    let () = (::match_deref::match_deref! { match &(algorithmItem.clone()) {
        Deref @ Absyn::ALGORITHMITEM { algorithm_: alg, .. } => {
            let mut alg = (*alg).clone();
            (alg, arg) = traverseAlgorithmBidir(alg.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            assign_variant_field!(algorithmItem => Absyn::AlgorithmItem::ALGORITHMITEM; algorithm_ = alg.clone());
            ()
        },
        Deref @ Absyn::ALGORITHMITEMCOMMENT { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok((algorithmItem, arg))
}

pub fn traverseAlgorithmItemListBidir<Arg: Clone + 'static>(inAlgs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> (Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, Arg) {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outAlgs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
    let mut outArg: Arg;
    (outAlgs, outArg) = List::map2FoldCheckReferenceEq(inAlgs.clone(), Arc::new(traverseAlgorithmItemBidir), enterFunc.clone(), exitFunc.clone(), inArg.clone());
    (outAlgs, outArg)
}

pub fn traverseClassComponents<ArgT: Clone + 'static>(inClass: Arc<Absyn::Class>, inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)> + 'static>, inArg: ArgT) -> Result<(Arc<Absyn::Class>, ArgT)> {
    pub type FuncType<ArgT: Clone> = fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)>;

    let mut outClass: Arc<Absyn::Class> = inClass.clone();
    let mut outArg: ArgT;
    outClass = (::match_deref::match_deref! { match &(outClass.clone()) {
        Deref @ Absyn::CLASS { .. } => {
            let mut body: Arc<Absyn::ClassDef>;
            (body, outArg, _) = traverseClassDef(outClass.body.clone(), Arc::new({ let __pe_b1 = inFunc.clone(); move |__pe_a0, __pe_a2| traverseClassPartComponents(__pe_a0, __pe_b1.clone(), __pe_a2) }), inArg.clone())?;
            if !(referenceEq(&body.clone(),&outClass.body.clone())) {
                assign_variant_field!(outClass => Absyn::Class::CLASS; body = body.clone());
            }
            outClass.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outClass, outArg))
}

fn traverseClassDef<ArgT: Clone + 'static>(inClassDef: Arc<Absyn::ClassDef>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::ClassPart>, ArgT) -> Result<(Arc<Absyn::ClassPart>, ArgT, bool)> + 'static>, inArg: ArgT) -> Result<(Arc<Absyn::ClassDef>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone> = fn(Arc<Absyn::ClassPart>, ArgT) -> Result<(Arc<Absyn::ClassPart>, ArgT, bool)>;

    let mut outClassDef: Arc<Absyn::ClassDef> = inClassDef.clone();
    let mut outArg: ArgT = inArg.clone();
    let mut outContinue: bool = true;
    let _ = (::match_deref::match_deref! { match &(outClassDef.clone()) {
        Deref @ Absyn::PARTS { .. } => {
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            (parts, outArg, outContinue) = traverseListGeneric(var_field!((*outClassDef).classParts, Absyn::ClassDef::PARTS).clone(), inFunc.clone(), inArg.clone())?;
            assign_variant_field!(outClassDef => Absyn::ClassDef::PARTS; classParts = parts.clone());
            ()
        },
        Deref @ Absyn::CLASS_EXTENDS { .. } => {
            let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
            (parts, outArg, outContinue) = traverseListGeneric(var_field!((*outClassDef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), inFunc.clone(), inArg.clone())?;
            assign_variant_field!(outClassDef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassDef, outArg, outContinue))
}

pub fn traverseClassDefElements<ArgT: Clone + 'static>(classDef: Arc<Absyn::ClassDef>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)> + 'static>, arg: ArgT) -> Result<(Arc<Absyn::ClassDef>, ArgT)> {
    pub type FuncType<ArgT: Clone> = fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)>;

    let mut classDef: Arc<Absyn::ClassDef> = classDef;
    let mut arg: ArgT = arg;
    (classDef, arg, _) = traverseClassDef(classDef.clone(), Arc::new({ let __pe_b1 = func.clone(); move |__pe_a0, __pe_a2| traverseClassPartElements(__pe_a0, __pe_b1.clone(), __pe_a2) }), arg.clone())?;
    Ok((classDef, arg))
}

pub fn traverseClassElements<ArgT: Clone + 'static>(cls: Arc<Absyn::Class>, func: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)> + 'static>, arg: ArgT) -> Result<(Arc<Absyn::Class>, ArgT)> {
    pub type FuncType<ArgT: Clone> = fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)>;

    let mut cls: Arc<Absyn::Class> = cls;
    let mut arg: ArgT = arg;
    let mut body: Arc<Absyn::ClassDef>;
    (body, arg) = traverseClassDefElements(cls.body.clone(), func.clone(), arg.clone())?;
    if !(referenceEq(&body.clone(),&cls.body.clone())) {
        assign_field!(cls.body = body.clone());
    }
    Ok((cls, arg))
}

fn traverseClassPartBidir<Arg: Clone + 'static>(cp: Arc<Absyn::ClassPart>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::ClassPart>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut cp: Arc<Absyn::ClassPart> = cp;
    let mut arg: Arg = arg;
    (cp, arg) = (::match_deref::match_deref! { match &(cp.clone()) {
        Deref @ Absyn::ALGORITHMS { contents: algs } => {
            let mut eqs: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut algs = (*algs).clone();
            (algs, arg) = List::map2FoldCheckReferenceEq(algs.clone(), Arc::new(traverseAlgorithmItemBidir), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (Arc::new(Absyn::ClassPart::ALGORITHMS { contents: algs.clone() }), arg.clone())
        },
        Deref @ Absyn::EQUATIONS { contents: eqs } => {
            let mut algs: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>;
            let mut eqs = (*eqs).clone();
            (eqs, arg) = List::map2FoldCheckReferenceEq(eqs.clone(), Arc::new(traverseEquationItemBidir), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (Arc::new(Absyn::ClassPart::EQUATIONS { contents: eqs.clone() }), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((cp, arg))
}

fn traverseClassPartComponents<ArgT: Clone + 'static>(inClassPart: Arc<Absyn::ClassPart>, inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)> + 'static>, inArg: ArgT) -> Result<(Arc<Absyn::ClassPart>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone> = fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)>;

    let mut outClassPart: Arc<Absyn::ClassPart> = inClassPart.clone();
    let mut outArg: ArgT = inArg.clone();
    let mut outContinue: bool = true;
    let _ = (::match_deref::match_deref! { match &(outClassPart.clone()) {
        Deref @ Absyn::PUBLIC { .. } => {
            let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            (items, outArg, outContinue) = traverseListGeneric(var_field!((*outClassPart).contents, Absyn::ClassPart::PUBLIC).clone(), Arc::new({ let __pe_b1 = inFunc.clone(); move |__pe_a0, __pe_a2| Ok(traverseElementItemComponents(__pe_a0, __pe_b1.clone(), __pe_a2)) }), inArg.clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = items.clone());
            ()
        },
        Deref @ Absyn::PROTECTED { .. } => {
            let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            (items, outArg, outContinue) = traverseListGeneric(var_field!((*outClassPart).contents, Absyn::ClassPart::PROTECTED).clone(), Arc::new({ let __pe_b1 = inFunc.clone(); move |__pe_a0, __pe_a2| Ok(traverseElementItemComponents(__pe_a0, __pe_b1.clone(), __pe_a2)) }), inArg.clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = items.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassPart, outArg, outContinue))
}

fn traverseClassPartElements<ArgT: Clone + 'static>(inClassPart: Arc<Absyn::ClassPart>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)> + 'static>, inArg: ArgT) -> Result<(Arc<Absyn::ClassPart>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone> = fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)>;

    let mut outClassPart: Arc<Absyn::ClassPart> = inClassPart.clone();
    let mut outArg: ArgT = inArg.clone();
    let mut outContinue: bool = true;
    let _ = (::match_deref::match_deref! { match &(outClassPart.clone()) {
        Deref @ Absyn::PUBLIC { .. } => {
            let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            (items, outArg, outContinue) = traverseListGeneric(var_field!((*outClassPart).contents, Absyn::ClassPart::PUBLIC).clone(), Arc::new({ let __pe_b1 = inFunc.clone(); move |__pe_a0, __pe_a2| Ok(traverseElementItem(__pe_a0, __pe_b1.clone(), __pe_a2)) }), inArg.clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PUBLIC; contents = items.clone());
            ()
        },
        Deref @ Absyn::PROTECTED { .. } => {
            let mut items: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
            (items, outArg, outContinue) = traverseListGeneric(var_field!((*outClassPart).contents, Absyn::ClassPart::PROTECTED).clone(), Arc::new({ let __pe_b1 = inFunc.clone(); move |__pe_a0, __pe_a2| Ok(traverseElementItem(__pe_a0, __pe_b1.clone(), __pe_a2)) }), inArg.clone())?;
            assign_variant_field!(outClassPart => Absyn::ClassPart::PROTECTED; contents = items.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outClassPart, outArg, outContinue))
}

pub fn traverseClasses<Arg: Clone + 'static>(inProgram: Absyn::Program, inPath: Option<Arc<Absyn::Path>>, inFunc: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>, inArg: Arg, inVisitProtected: bool) -> Result<(Absyn::Program, Option<Arc<Absyn::Path>>, Arg)> {
    pub type FuncType<Arg: Clone> = fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)>;

    let mut outTpl: (Absyn::Program, Option<Arc<Absyn::Path>>, Arg);
    outTpl = (match inProgram.clone() {
        mut p @ Absyn::PROGRAM { .. } => {
            let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
            let mut pa: Option<Arc<Absyn::Path>>;
            let mut arg: Arg;
            (classes, pa, arg) = traverseClasses2(p.classes.clone(), inPath.clone(), inFunc.clone(), inArg.clone(), inVisitProtected.clone())?;
            let __owned_variant_classes_0 = classes.clone();
            if let Absyn::Program::PROGRAM { classes, .. } = &mut p {
                *classes = __owned_variant_classes_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Program::PROGRAM"); }
            (p.clone(), pa.clone(), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    });
    Ok(outTpl)
}

fn traverseClasses2<Arg: Clone + 'static>(inClasses: Arc<metamodelica::List<Arc<Absyn::Class>>>, inPath: Option<Arc<Absyn::Path>>, inFunc: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>, inArg: Arg, inVisitProtected: bool) -> Result<(Arc<metamodelica::List<Arc<Absyn::Class>>>, Option<Arc<Absyn::Path>>, Arg)> {
    pub type FuncType<Arg: Clone> = fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)>;

    let mut outTpl: (Arc<metamodelica::List<Arc<Absyn::Class>>>, Option<Arc<Absyn::Path>>, Arg);
    outTpl = 'mc: {
        let __mc_input = (inClasses.clone(), inPath.clone(), inFunc.clone(), inArg.clone(), inVisitProtected.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Nil, pa, _, args, _) => {
                    let mut pa_1: Option<Arc<Absyn::Path>>;
                    let mut pa_2: Option<Arc<Absyn::Path>>;
                    let mut pa_3: Option<Arc<Absyn::Path>>;
                    let mut visitor: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>;
                    let mut args_1: Arg;
                    let mut args_2: Arg;
                    let mut args_3: Arg;
                    let mut class_1: Arc<Absyn::Class>;
                    let mut class_2: Arc<Absyn::Class>;
                    let mut class_: Arc<Absyn::Class>;
                    let mut classes_1: Arc<metamodelica::List<Arc<Absyn::Class>>>;
                    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
                    let mut traverse_prot: bool;
                    Ok((metamodelica::nil(), pa.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: class_, tail: classes }, pa, visitor, args, traverse_prot) => {
                    let mut pa_1: Option<Arc<Absyn::Path>>;
                    let mut pa_2: Option<Arc<Absyn::Path>>;
                    let mut pa_3: Option<Arc<Absyn::Path>>;
                    let mut args_1: Arg;
                    let mut args_2: Arg;
                    let mut args_3: Arg;
                    let mut class_1: Arc<Absyn::Class>;
                    let mut class_2: Arc<Absyn::Class>;
                    let mut classes_1: Arc<metamodelica::List<Arc<Absyn::Class>>>;
                    (class_1, _, args_1) = visitor((class_.clone(), pa.clone(), args.clone()))?;
                    (class_2, _, args_2) = traverseInnerClass(class_1.clone(), pa.clone(), visitor.clone(), args_1.clone(), traverse_prot.clone())?;
                    (classes_1, pa_3, args_3) = traverseClasses2(classes.clone(), pa.clone(), visitor.clone(), args_2.clone(), traverse_prot.clone())?;
                    Ok((cons(class_2.clone(), classes_1.clone()), pa_3.clone(), args_3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: class_, tail: classes }, pa, visitor, args, traverse_prot) => {
                    let mut pa_1: Option<Arc<Absyn::Path>>;
                    let mut pa_2: Option<Arc<Absyn::Path>>;
                    let mut pa_3: Option<Arc<Absyn::Path>>;
                    let mut args_1: Arg;
                    let mut args_2: Arg;
                    let mut args_3: Arg;
                    let mut class_1: Arc<Absyn::Class>;
                    let mut class_2: Arc<Absyn::Class>;
                    let mut classes_1: Arc<metamodelica::List<Arc<Absyn::Class>>>;
                    (class_2, _, args_2) = traverseInnerClass(class_.clone(), pa.clone(), visitor.clone(), args.clone(), traverse_prot.clone())?;
                    let true = (classHasLocalClasses(class_2.clone())?) else { bail!("pattern mismatch") };
                    (classes_1, pa_3, args_3) = traverseClasses2(classes.clone(), pa.clone(), visitor.clone(), args_2.clone(), traverse_prot.clone())?;
                    Ok((cons(class_2.clone(), classes_1.clone()), pa_3.clone(), args_3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: _, tail: classes }, pa, visitor, args, traverse_prot) => {
                    let mut pa_1: Option<Arc<Absyn::Path>>;
                    let mut pa_2: Option<Arc<Absyn::Path>>;
                    let mut pa_3: Option<Arc<Absyn::Path>>;
                    let mut args_1: Arg;
                    let mut args_2: Arg;
                    let mut args_3: Arg;
                    let mut class_1: Arc<Absyn::Class>;
                    let mut class_2: Arc<Absyn::Class>;
                    let mut class_: Arc<Absyn::Class>;
                    let mut classes_1: Arc<metamodelica::List<Arc<Absyn::Class>>>;
                    (classes_1, pa_3, args_3) = traverseClasses2(classes.clone(), pa.clone(), visitor.clone(), args.clone(), traverse_prot.clone())?;
                    Ok((classes_1.clone(), pa_3.clone(), args_3.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::List::Cons { head: class_, tail: _ }, _, _, _, _) => {
                    let mut pa: Option<Arc<Absyn::Path>>;
                    let mut pa_1: Option<Arc<Absyn::Path>>;
                    let mut pa_2: Option<Arc<Absyn::Path>>;
                    let mut pa_3: Option<Arc<Absyn::Path>>;
                    let mut visitor: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>;
                    let mut args: Arg;
                    let mut args_1: Arg;
                    let mut args_2: Arg;
                    let mut args_3: Arg;
                    let mut class_1: Arc<Absyn::Class>;
                    let mut class_2: Arc<Absyn::Class>;
                    let mut classes_1: Arc<metamodelica::List<Arc<Absyn::Class>>>;
                    let mut classes: Arc<metamodelica::List<Arc<Absyn::Class>>>;
                    let mut traverse_prot: bool;
                    println!("{}", (literal!("-traverse_classes2 failed on class:")).clone());
                    println!("{}", (className(class_.clone())?).clone());
                    println!("{}", (literal!("\\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTpl)
}

fn traverseElementComponents<ArgT: Clone + 'static>(inElement: Arc<Absyn::Element>, inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)> + 'static>, inArg: ArgT) -> (Arc<Absyn::Element>, ArgT, bool) {
    pub type FuncType<ArgT: Clone> = fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)>;

    let mut outElement: Arc<Absyn::Element> = inElement.clone();
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outElement, outArg, outContinue) = (::match_deref::match_deref! { match &(outElement.clone()) {
        Deref @ Absyn::ELEMENT { .. } => {
            let mut spec: Arc<Absyn::ElementSpec>;
            (spec, outArg, outContinue) = traverseElementSpecComponents(var_field!((*outElement).specification, Absyn::Element::ELEMENT).clone(), inFunc.clone(), inArg.clone());
            if !(referenceEq(&spec.clone(),&var_field!((*outElement).specification, Absyn::Element::ELEMENT).clone())) {
                assign_variant_field!(outElement => Absyn::Element::ELEMENT; specification = spec.clone());
            }
            (outElement.clone(), outArg.clone(), outContinue.clone())
        },
        _ => (inElement.clone(), inArg.clone(), true),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outElement, outArg, outContinue)
}

fn traverseElementItem<ArgT: Clone + 'static>(inItem: Arc<Absyn::ElementItem>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)> + 'static>, inArg: ArgT) -> (Arc<Absyn::ElementItem>, ArgT, bool) {
    pub type FuncType<ArgT: Clone> = fn(Arc<Absyn::Element>, ArgT) -> Result<(Arc<Absyn::Element>, ArgT, bool)>;

    let mut outItem: Arc<Absyn::ElementItem>;
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outItem, outArg, outContinue) = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Absyn::ELEMENTITEM { .. } => {
            let mut elem: Arc<Absyn::Element>;
            (elem, outArg, outContinue) = inFunc(var_field!((*inItem).element, Absyn::ElementItem::ELEMENTITEM).clone(), inArg.clone()).unwrap();
            outItem = if (referenceEq(&elem.clone(),&var_field!((*inItem).element, Absyn::ElementItem::ELEMENTITEM).clone())) {inItem.clone()} else {Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elem.clone() })};
            (outItem.clone(), outArg.clone(), outContinue.clone())
        },
        _ => (inItem.clone(), inArg.clone(), true),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outItem, outArg, outContinue)
}

fn traverseElementItemComponents<ArgT: Clone + 'static>(inItem: Arc<Absyn::ElementItem>, inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)> + 'static>, inArg: ArgT) -> (Arc<Absyn::ElementItem>, ArgT, bool) {
    pub type FuncType<ArgT: Clone> = fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)>;

    let mut outItem: Arc<Absyn::ElementItem>;
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outItem, outArg, outContinue) = (::match_deref::match_deref! { match &(inItem.clone()) {
        Deref @ Absyn::ELEMENTITEM { .. } => {
            let mut elem: Arc<Absyn::Element>;
            (elem, outArg, outContinue) = traverseElementComponents(var_field!((*inItem).element, Absyn::ElementItem::ELEMENTITEM).clone(), inFunc.clone(), inArg.clone());
            outItem = if (referenceEq(&elem.clone(),&var_field!((*inItem).element, Absyn::ElementItem::ELEMENTITEM).clone())) {inItem.clone()} else {Arc::new(Absyn::ElementItem::ELEMENTITEM { element: elem.clone() })};
            (outItem.clone(), outArg.clone(), outContinue.clone())
        },
        _ => (inItem.clone(), inArg.clone(), true),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outItem, outArg, outContinue)
}

fn traverseElementSpecComponents<ArgT: Clone + 'static>(inSpec: Arc<Absyn::ElementSpec>, inFunc: Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)> + 'static>, inArg: ArgT) -> (Arc<Absyn::ElementSpec>, ArgT, bool) {
    pub type FuncType<ArgT: Clone> = fn(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT) -> Result<(Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>, ArgT, bool)>;

    let mut outSpec: Arc<Absyn::ElementSpec> = inSpec.clone();
    let mut outArg: ArgT;
    let mut outContinue: bool;
    (outSpec, outArg, outContinue) = (::match_deref::match_deref! { match &(outSpec.clone()) {
        Deref @ Absyn::COMPONENTS { .. } => {
            let mut cls: Arc<Absyn::Class>;
            let mut comps: Arc<metamodelica::List<Arc<Absyn::ComponentItem>>>;
            (comps, outArg, outContinue) = inFunc(var_field!((*outSpec).components, Absyn::ElementSpec::COMPONENTS).clone(), inArg.clone()).unwrap();
            if !(referenceEq(&comps.clone(),&var_field!((*outSpec).components, Absyn::ElementSpec::COMPONENTS).clone())) {
                assign_variant_field!(outSpec => Absyn::ElementSpec::COMPONENTS; components = comps.clone());
            }
            (outSpec.clone(), outArg.clone(), outContinue.clone())
        },
        _ => (inSpec.clone(), inArg.clone(), true),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outSpec, outArg, outContinue)
}

pub fn traverseEquationBidir<Arg: Clone + 'static>(eq: Arc<Absyn::Equation>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::Equation>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut eq: Arc<Absyn::Equation> = eq;
    let mut arg: Arg = arg;
    eq = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::EQ_IF { equationElseItems: eqil2, elseIfBranches: else_branch, equationTrueItems: eqil1, ifExp: e1 } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut eq1: Arc<Absyn::EquationItem>;
            let mut eqil2 = (*eqil2).clone();
            let mut else_branch = (*else_branch).clone();
            let mut eqil1 = (*eqil1).clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (eqil1, arg) = traverseEquationItemListBidir(eqil1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (else_branch, arg) = List::map2FoldCheckReferenceEq(else_branch.clone(), Arc::new(traverseEquationBidirElse), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (eqil2, arg) = traverseEquationItemListBidir(eqil2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Equation::EQ_IF { ifExp: e1.clone(), equationTrueItems: eqil1.clone(), elseIfBranches: else_branch.clone(), equationElseItems: eqil2.clone() })
        },
        Deref @ Absyn::EQ_EQUALS { rightSide: e2, leftSide: e1 } => {
            let mut eqil1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqil2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut eq1: Arc<Absyn::EquationItem>;
            let mut e2 = (*e2).clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            Arc::new(Absyn::Equation::EQ_EQUALS { leftSide: e1.clone(), rightSide: e2.clone() })
        },
        Deref @ Absyn::EQ_PDE { domain: cref1, rightSide: e2, leftSide: e1 } => {
            let mut eqil1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqil2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut eq1: Arc<Absyn::EquationItem>;
            let mut cref1 = (*cref1).clone();
            let mut e2 = (*e2).clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (cref1, _) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            Arc::new(Absyn::Equation::EQ_PDE { leftSide: e1.clone(), rightSide: e2.clone(), domain: cref1.clone() })
        },
        Deref @ Absyn::EQ_CONNECT { connector2: cref2, connector1: cref1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut eqil1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqil2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut eq1: Arc<Absyn::EquationItem>;
            let mut cref2 = (*cref2).clone();
            let mut cref1 = (*cref1).clone();
            (cref1, arg) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (cref2, arg) = traverseExpBidirCref(cref2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            Arc::new(Absyn::Equation::EQ_CONNECT { connector1: cref1.clone(), connector2: cref2.clone() })
        },
        Deref @ Absyn::EQ_FOR { forEquations: eqil1, iterators: iters } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut eqil2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut eq1: Arc<Absyn::EquationItem>;
            let mut eqil1 = (*eqil1).clone();
            let mut iters = (*iters).clone();
            (iters, arg) = List::map2FoldCheckReferenceEq(iters.clone(), Arc::new(traverseExpBidirIterator), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (eqil1, arg) = traverseEquationItemListBidir(eqil1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Equation::EQ_FOR { iterators: iters.clone(), forEquations: eqil1.clone() })
        },
        Deref @ Absyn::EQ_WHEN_E { elseWhenEquations: else_branch, whenEquations: eqil1, whenExp: e1 } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut eqil2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut eq1: Arc<Absyn::EquationItem>;
            let mut else_branch = (*else_branch).clone();
            let mut eqil1 = (*eqil1).clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (eqil1, arg) = traverseEquationItemListBidir(eqil1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (else_branch, arg) = List::map2FoldCheckReferenceEq(else_branch.clone(), Arc::new(traverseEquationBidirElse), enterFunc.clone(), exitFunc.clone(), arg.clone());
            Arc::new(Absyn::Equation::EQ_WHEN_E { whenExp: e1.clone(), whenEquations: eqil1.clone(), elseWhenEquations: else_branch.clone() })
        },
        Deref @ Absyn::EQ_NORETCALL { functionArgs: func_args, functionName: cref1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut eqil1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqil2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut eq1: Arc<Absyn::EquationItem>;
            let mut func_args = (*func_args).clone();
            let mut cref1 = (*cref1).clone();
            (cref1, arg) = traverseExpBidirCref(cref1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (func_args, arg) = traverseExpBidirFunctionArgs(func_args.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            Arc::new(Absyn::Equation::EQ_NORETCALL { functionName: cref1.clone(), functionArgs: func_args.clone() })
        },
        Deref @ Absyn::EQ_FAILURE { equ: eq1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut eqil1: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut eqil2: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
            let mut else_branch: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>)>>;
            let mut cref1: Arc<Absyn::ComponentRef>;
            let mut cref2: Arc<Absyn::ComponentRef>;
            let mut iters: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut func_args: Arc<Absyn::FunctionArgs>;
            let mut eq1 = (*eq1).clone();
            (eq1, arg) = traverseEquationItemBidir(eq1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            Arc::new(Absyn::Equation::EQ_FAILURE { equ: eq1.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((eq, arg))
}

fn traverseEquationBidirElse<Arg: Clone + 'static>(inElse: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<((Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>), Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outElse: (Arc<Absyn::Exp>, Arc<metamodelica::List<Arc<Absyn::EquationItem>>>);
    let mut arg: Arg;
    let mut e: Arc<Absyn::Exp>;
    let mut eqil: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    (e, eqil) = inElse.clone();
    (e, arg) = traverseExpBidir(e.clone(), enterFunc.clone(), exitFunc.clone(), inArg.clone())?;
    (eqil, arg) = traverseEquationItemListBidir(eqil.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
    outElse = (e.clone(), eqil.clone());
    Ok((outElse, arg))
}

fn traverseEquationItemBidir<Arg: Clone + 'static>(equationItem: Arc<Absyn::EquationItem>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::EquationItem>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut equationItem: Arc<Absyn::EquationItem> = equationItem;
    let mut arg: Arg = arg;
    let () = (::match_deref::match_deref! { match &(equationItem.clone()) {
        Deref @ Absyn::EQUATIONITEM { equation_: eq, .. } => {
            let mut eq = (*eq).clone();
            (eq, arg) = traverseEquationBidir(eq.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            assign_variant_field!(equationItem => Absyn::EquationItem::EQUATIONITEM; equation_ = eq.clone());
            ()
        },
        Deref @ Absyn::EQUATIONITEMCOMMENT { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
    Ok((equationItem, arg))
}

pub fn traverseEquationItemListBidir<Arg: Clone + 'static>(inEquationItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> (Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, Arg) {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outEquationItems: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>;
    let mut outArg: Arg;
    (outEquationItems, outArg) = List::map2FoldCheckReferenceEq(inEquationItems.clone(), Arc::new(traverseEquationItemBidir), enterFunc.clone(), exitFunc.clone(), inArg.clone());
    (outEquationItems, outArg)
}

pub fn traverseExp<Arg: Clone + 'static>(inExp: Arc<Absyn::Exp>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<(Arc<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outExp: Arc<Absyn::Exp>;
    let mut outArg: Arg;
    (outExp, outArg) = traverseExpBidir(inExp.clone(), Arc::new(fnptr!(dummyTraverseExp, Arc<Absyn::Exp>, _)), inFunc.clone(), inArg.clone())?;
    Ok((outExp, outArg))
}

pub fn traverseExpBidir<Arg: Clone + 'static>(inExp: Arc<Absyn::Exp>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<(Arc<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut e: Arc<Absyn::Exp>;
    let mut arg: Arg;
    (e, arg) = enterFunc(inExp.clone(), inArg.clone())?;
    (e, arg) = traverseExpBidirSubExps(e.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
    (e, arg) = exitFunc(e.clone(), arg.clone())?;
    Ok((e, arg))
}

pub fn traverseExpBidirCref<Arg: Clone + 'static>(cref: Arc<Absyn::ComponentRef>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::ComponentRef>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut cref: Arc<Absyn::ComponentRef> = cref;
    let mut arg: Arg = arg;
    (cref, arg) = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::CREF_FULLYQUALIFIED { componentRef: cr1 } => {
            let mut name: ArcStr;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut subs1: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut subs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (cr2, arg) = traverseExpBidirCref(cr1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&cr1.clone(),&cr2.clone())) {cref.clone()} else {crefMakeFullyQualified(cr2.clone())}, arg.clone())
        },
        Deref @ Absyn::CREF_QUAL { componentRef: cr1, subscripts: subs1, name } => {
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut subs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (subs2, arg) = traverseExpBidirSubs(subs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (cr2, arg) = traverseExpBidirCref(cr1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&cr1.clone(),&cr2.clone()) && referenceEq(&subs1.clone(),&subs2.clone())) {cref.clone()} else {Arc::new(Absyn::ComponentRef::CREF_QUAL { name: (name.clone()).clone(), subscripts: subs2.clone(), componentRef: cr2.clone() })}, arg.clone())
        },
        Deref @ Absyn::CREF_IDENT { subscripts: subs1, name } => {
            let mut cr1: Arc<Absyn::ComponentRef>;
            let mut cr2: Arc<Absyn::ComponentRef>;
            let mut subs2: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (subs2, arg) = traverseExpBidirSubs(subs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&subs1.clone(),&subs2.clone())) {cref.clone()} else {Arc::new(Absyn::ComponentRef::CREF_IDENT { name: (name.clone()).clone(), subscripts: subs2.clone() })}, arg.clone())
        },
        Deref @ Absyn::ALLWILD => (cref.clone(), arg.clone()),
        Deref @ Absyn::WILD => (cref.clone(), arg.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok((cref, arg))
}

pub fn traverseExpBidirElseIf<Arg: Clone + 'static>(inElseIf: (Arc<Absyn::Exp>, Arc<Absyn::Exp>), enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<((Arc<Absyn::Exp>, Arc<Absyn::Exp>), Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outElseIf: (Arc<Absyn::Exp>, Arc<Absyn::Exp>);
    let mut arg: Arg;
    let mut e1: Arc<Absyn::Exp>;
    let mut e2: Arc<Absyn::Exp>;
    (e1, e2) = inElseIf.clone();
    (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), inArg.clone())?;
    (e2, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
    outElseIf = (e1.clone(), e2.clone());
    Ok((outElseIf, arg))
}

pub fn traverseExpBidirFunctionArgs<Arg: Clone + 'static>(args: Arc<Absyn::FunctionArgs>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::FunctionArgs>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut args: Arc<Absyn::FunctionArgs> = args;
    let mut arg: Arg = arg;
    (args, arg) = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FUNCTIONARGS { argNames: named_args1, args: expl1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut named_args2: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut iters1: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut iters2: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            let mut iterType: Absyn::ReductionIterType;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (named_args2, arg) = List::map2FoldCheckReferenceEq(named_args1.clone(), Arc::new(traverseExpBidirNamedArg), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&expl1.clone(),&expl2.clone()) && referenceEq(&named_args1.clone(),&named_args2.clone())) {args.clone()} else {Arc::new(Absyn::FunctionArgs::FUNCTIONARGS { args: expl2.clone(), argNames: named_args2.clone() })}, arg.clone())
        },
        Deref @ Absyn::FOR_ITER_FARG { exp: e1, iterType, iterators: iters1 } => {
            let mut e2: Arc<Absyn::Exp>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut named_args1: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut named_args2: Arc<metamodelica::List<Arc<Absyn::NamedArg>>>;
            let mut iters2: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>;
            (e2, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (iters2, arg) = List::map2FoldCheckReferenceEq(iters1.clone(), Arc::new(traverseExpBidirIterator), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&e1.clone(),&e2.clone()) && referenceEq(&iters1.clone(),&iters2.clone())) {args.clone()} else {Arc::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: e2.clone(), iterType: iterType.clone(), iterators: iters2.clone() })}, arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((args, arg))
}

pub fn traverseExpBidirIterator<Arg: Clone + 'static>(inIterator: Arc<Absyn::ForIterator>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<(Arc<Absyn::ForIterator>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outIterator: Arc<Absyn::ForIterator>;
    let mut outArg: Arg;
    let mut name: ArcStr;
    let mut guardExp1: Option<Arc<Absyn::Exp>>;
    let mut guardExp2: Option<Arc<Absyn::Exp>>;
    let mut range1: Option<Arc<Absyn::Exp>>;
    let mut range2: Option<Arc<Absyn::Exp>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inIterator.clone()) {
        Deref @ Absyn::ITERATOR { range: __pa0, guardExp: __pa1, name: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    range1 = __pa0.clone();
    guardExp1 = __pa1.clone();
    name = __pa2.clone();
    (guardExp2, outArg) = traverseExpOptBidir(guardExp1.clone(), enterFunc.clone(), exitFunc.clone(), inArg.clone())?;
    (range2, outArg) = traverseExpOptBidir(range1.clone(), enterFunc.clone(), exitFunc.clone(), outArg.clone())?;
    outIterator = if (referenceEq(&guardExp1.clone(),&guardExp2.clone()) && referenceEq(&range1.clone(),&range2.clone())) {inIterator.clone()} else {Arc::new(Absyn::ForIterator { name: (name.clone()).clone(), guardExp: guardExp2.clone(), range: range2.clone() })};
    Ok((outIterator, outArg))
}

pub fn traverseExpBidirNamedArg<Arg: Clone + 'static>(inArg: Arc<Absyn::NamedArg>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inExtra: Arg) -> Result<(Arc<Absyn::NamedArg>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outArg: Arc<Absyn::NamedArg>;
    let mut outExtra: Arg;
    let mut name: ArcStr;
    let mut value1: Arc<Absyn::Exp>;
    let mut value2: Arc<Absyn::Exp>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inArg.clone()) {
        Deref @ Absyn::NAMEDARG { argName: __pa0, argValue: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    value1 = __pa1.clone();
    (value2, outExtra) = traverseExpBidir(value1.clone(), enterFunc.clone(), exitFunc.clone(), inExtra.clone())?;
    outArg = if (referenceEq(&value1.clone(),&value2.clone())) {inArg.clone()} else {Arc::new(Absyn::NamedArg { argName: (name.clone()).clone(), argValue: value2.clone() })};
    Ok((outArg, outExtra))
}

pub fn traverseExpBidirSub<Arg: Clone + 'static>(subscript: Arc<Absyn::Subscript>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::Subscript>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut subscript: Arc<Absyn::Subscript> = subscript;
    let mut arg: Arg = arg;
    (subscript, arg) = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ Absyn::SUBSCRIPT { subscript: e1 } => {
            let mut e2: Arc<Absyn::Exp>;
            (e2, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e2.clone())) {subscript.clone()} else {Arc::new(Absyn::Subscript::SUBSCRIPT { subscript: e2.clone() })}, arg.clone())
        },
        Deref @ Absyn::NOSUB => (subscript.clone(), arg.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok((subscript, arg))
}

fn traverseExpBidirSubExps<Arg: Clone + 'static>(exp: Arc<Absyn::Exp>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut exp: Arc<Absyn::Exp> = exp;
    let mut arg: Arg = arg;
    (exp, arg) = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::INTEGER { .. } => (exp.clone(), arg.clone()),
        Deref @ Absyn::REAL { .. } => (exp.clone(), arg.clone()),
        Deref @ Absyn::STRING { .. } => (exp.clone(), arg.clone()),
        Deref @ Absyn::BOOL { .. } => (exp.clone(), arg.clone()),
        Deref @ Absyn::CREF { componentRef: cref } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (crefm, arg) = traverseExpBidirCref(cref.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&cref.clone(),&crefm.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::CREF { componentRef: crefm.clone() })}, arg.clone())
        },
        Deref @ Absyn::BINARY { exp2: e2, exp1: e1, .. } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone()) && referenceEq(&e2.clone(),&e2m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::BINARY { exp1: e1m.clone(), op: var_field!((*exp).op, Absyn::Exp::BINARY).clone(), exp2: e2m.clone() })}, arg.clone())
        },
        Deref @ Absyn::UNARY { exp: e1, .. } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::UNARY { op: var_field!((*exp).op, Absyn::Exp::UNARY).clone(), exp: e1m.clone() })}, arg.clone())
        },
        Deref @ Absyn::LBINARY { exp2: e2, exp1: e1, .. } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone()) && referenceEq(&e2.clone(),&e2m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::LBINARY { exp1: e1m.clone(), op: var_field!((*exp).op, Absyn::Exp::LBINARY).clone(), exp2: e2m.clone() })}, arg.clone())
        },
        Deref @ Absyn::LUNARY { exp: e1, .. } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::LUNARY { op: var_field!((*exp).op, Absyn::Exp::LUNARY).clone(), exp: e1m.clone() })}, arg.clone())
        },
        Deref @ Absyn::RELATION { exp2: e2, exp1: e1, .. } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone()) && referenceEq(&e2.clone(),&e2m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::RELATION { exp1: e1m.clone(), op: var_field!((*exp).op, Absyn::Exp::RELATION).clone(), exp2: e2m.clone() })}, arg.clone())
        },
        Deref @ Absyn::IFEXP { elseIfBranch: else_ifs1, elseBranch: e3, trueBranch: e2, ifExp: e1 } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e3m, arg) = traverseExpBidir(e3.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (else_ifs2, arg) = List::map2FoldCheckReferenceEq(else_ifs1.clone(), Arc::new(traverseExpBidirElseIf), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&e1.clone(),&e1m.clone()) && referenceEq(&e2.clone(),&e2m.clone()) && referenceEq(&e3.clone(),&e3m.clone()) && referenceEq(&else_ifs1.clone(),&else_ifs2.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::IFEXP { ifExp: e1m.clone(), trueBranch: e2m.clone(), elseBranch: e3m.clone(), elseIfBranch: else_ifs2.clone() })}, arg.clone())
        },
        Deref @ Absyn::CALL { functionArgs: fargs1, function_: cref, .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (fargs2, arg) = traverseExpBidirFunctionArgs(fargs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&fargs1.clone(),&fargs2.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::CALL { function_: cref.clone(), functionArgs: fargs2.clone(), typeVars: var_field!((*exp).typeVars, Absyn::Exp::CALL).clone() })}, arg.clone())
        },
        Deref @ Absyn::PARTEVALFUNCTION { functionArgs: fargs1, function_: cref } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (fargs2, arg) = traverseExpBidirFunctionArgs(fargs1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&fargs1.clone(),&fargs2.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::PARTEVALFUNCTION { function_: cref.clone(), functionArgs: fargs2.clone() })}, arg.clone())
        },
        Deref @ Absyn::ARRAY { arrayExp: expl1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&expl1.clone(),&expl2.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::ARRAY { arrayExp: expl2.clone() })}, arg.clone())
        },
        Deref @ Absyn::MATRIX { matrix: mat_expl } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut mat_expl = (*mat_expl).clone();
            (mat_expl, arg) = List::map2FoldCheckReferenceEq(mat_expl.clone(), Arc::new(fnptr!(traverseExpListBidir, Arc<metamodelica::List<Arc<Absyn::Exp>>>, _, _, _)), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (Arc::new(Absyn::Exp::MATRIX { matrix: mat_expl.clone() }), arg.clone())
        },
        Deref @ Absyn::RANGE { stop: e2, step: oe1, start: e1 } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (oe1m, arg) = traverseExpOptBidir(oe1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone()) && referenceEq(&e2.clone(),&e2m.clone()) && referenceEq(&oe1.clone(),&oe1m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::RANGE { start: e1m.clone(), step: oe1m.clone(), stop: e2m.clone() })}, arg.clone())
        },
        Deref @ Absyn::END => (exp.clone(), arg.clone()),
        Deref @ Absyn::TUPLE { expressions: expl1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&expl1.clone(),&expl2.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::TUPLE { expressions: expl2.clone() })}, arg.clone())
        },
        Deref @ Absyn::AS { exp: e1, id } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::AS { id: (id.clone()).clone(), exp: e1m.clone() })}, arg.clone())
        },
        Deref @ Absyn::CONS { rest: e2, head: e1 } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1m, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2m, arg) = traverseExpBidir(e2.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&e1.clone(),&e1m.clone()) && referenceEq(&e2.clone(),&e2m.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::CONS { head: e1m.clone(), rest: e2m.clone() })}, arg.clone())
        },
        Deref @ Absyn::MATCHEXP { cases: match_cases, inputExp: e1, .. } => {
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            let mut match_cases = (*match_cases).clone();
            let mut e1 = (*e1).clone();
            (e1, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (match_cases, arg) = List::map2FoldCheckReferenceEq(match_cases.clone(), Arc::new(traverseMatchCase), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (Arc::new(Absyn::Exp::MATCHEXP { matchTy: var_field!((*exp).matchTy, Absyn::Exp::MATCHEXP).clone(), inputExp: e1.clone(), localDecls: var_field!((*exp).localDecls, Absyn::Exp::MATCHEXP).clone(), cases: match_cases.clone(), comment: var_field!((*exp).comment, Absyn::Exp::MATCHEXP).clone() }), arg.clone())
        },
        Deref @ Absyn::LIST { exps: expl1 } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (expl2, arg) = traverseExpListBidir(expl1.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&expl1.clone(),&expl2.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::LIST { exps: expl2.clone() })}, arg.clone())
        },
        Deref @ Absyn::CODE { .. } => (exp.clone(), arg.clone()),
        Deref @ Absyn::DOT { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1, arg) = traverseExpBidir(var_field!((*exp).exp, Absyn::Exp::DOT).clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (e2, arg) = traverseExpBidir(var_field!((*exp).index, Absyn::Exp::DOT).clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&var_field!((*exp).exp, Absyn::Exp::DOT).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).index, Absyn::Exp::DOT).clone(),&e2.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::DOT { exp: e1.clone(), index: e2.clone() })}, arg.clone())
        },
        Deref @ Absyn::EXPRESSIONCOMMENT { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1, arg) = traverseExpBidir(var_field!((*exp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (if (referenceEq(&var_field!((*exp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(),&e1.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::EXPRESSIONCOMMENT { commentsBefore: var_field!((*exp).commentsBefore, Absyn::Exp::EXPRESSIONCOMMENT).clone(), exp: e1.clone(), commentsAfter: var_field!((*exp).commentsAfter, Absyn::Exp::EXPRESSIONCOMMENT).clone() })}, arg.clone())
        },
        Deref @ Absyn::SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (e1, arg) = traverseExpBidir(var_field!((*exp).exp, Absyn::Exp::SUBSCRIPTED_EXP).clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (subs, arg) = traverseExpBidirSubs(var_field!((*exp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone(), enterFunc.clone(), exitFunc.clone(), arg.clone());
            (if (referenceEq(&var_field!((*exp).exp, Absyn::Exp::SUBSCRIPTED_EXP).clone(),&e1.clone()) && referenceEq(&var_field!((*exp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone(),&subs.clone())) {exp.clone()} else {Arc::new(Absyn::Exp::SUBSCRIPTED_EXP { exp: e1.clone(), subscripts: subs.clone() })}, arg.clone())
        },
        Deref @ Absyn::BREAK => (exp.clone(), arg.clone()),
        _ => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e1m: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            let mut e2m: Arc<Absyn::Exp>;
            let mut e3: Arc<Absyn::Exp>;
            let mut e3m: Arc<Absyn::Exp>;
            let mut oe1: Option<Arc<Absyn::Exp>>;
            let mut oe1m: Option<Arc<Absyn::Exp>>;
            let mut cref: Arc<Absyn::ComponentRef>;
            let mut crefm: Arc<Absyn::ComponentRef>;
            let mut else_ifs1: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut else_ifs2: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>>;
            let mut expl1: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut expl2: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
            let mut mat_expl: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>>;
            let mut fargs1: Arc<Absyn::FunctionArgs>;
            let mut fargs2: Arc<Absyn::FunctionArgs>;
            let mut error_msg: ArcStr;
            let mut id: ArcStr;
            let mut enterName: ArcStr;
            let mut exitName: ArcStr;
            let mut match_cases: Arc<metamodelica::List<Arc<Absyn::Case>>>;
            let mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
            (_, _, enterName) = System::dladdr(enterFunc.clone());
            (_, _, exitName) = System::dladdr(exitFunc.clone());
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("in traverseExpBidirSubExps(")); __mm_s.push_str(&*enterName.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*exitName.clone()); __mm_s.push_str(&*literal!(") - Unknown expression: ")); ArcStr::from(__mm_s) }).clone();
            error_msg = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*error_msg.clone()); __mm_s.push_str(&*Dump::printExpStr(exp.clone())?); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(error_msg.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, arg))
}

pub fn traverseExpBidirSubs<Arg: Clone + 'static>(subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> (Arc<metamodelica::List<Arc<Absyn::Subscript>>>, Arg) {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut subscripts: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = subscripts;
    let mut arg: Arg = arg;
    (subscripts, arg) = List::map2FoldCheckReferenceEq(subscripts.clone(), Arc::new(traverseExpBidirSub), enterFunc.clone(), exitFunc.clone(), arg.clone());
    (subscripts, arg)
}

pub fn traverseExpList<Arg: Clone + 'static>(inExpList: Arc<metamodelica::List<Arc<Absyn::Exp>>>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> (Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arg) {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outExpList: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    let mut outArg: Arg;
    (outExpList, outArg) = traverseExpListBidir(inExpList.clone(), Arc::new(fnptr!(dummyTraverseExp, Arc<Absyn::Exp>, _)), inFunc.clone(), inArg.clone());
    (outExpList, outArg)
}

pub fn traverseExpListBidir<Arg: Clone + 'static>(inExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> (Arc<metamodelica::List<Arc<Absyn::Exp>>>, Arg) {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outExpl: Arc<metamodelica::List<Arc<Absyn::Exp>>>;
    let mut outArg: Arg;
    (outExpl, outArg) = List::map2FoldCheckReferenceEq(inExpl.clone(), Arc::new(traverseExpBidir), enterFunc.clone(), exitFunc.clone(), inArg.clone());
    (outExpl, outArg)
}

pub fn traverseExpOptBidir<Arg: Clone + 'static>(inExp: Option<Arc<Absyn::Exp>>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<(Option<Arc<Absyn::Exp>>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outExp: Option<Arc<Absyn::Exp>>;
    let mut arg: Arg;
    (outExp, arg) = (match inExp.clone() {
        Some(mut e1) => {
            let mut e2: Arc<Absyn::Exp>;
            (e2, arg) = traverseExpBidir(e1.clone(), enterFunc.clone(), exitFunc.clone(), inArg.clone())?;
            (if (referenceEq(&e1.clone(),&e2.clone())) {inExp.clone()} else {Some(e2.clone())}, arg.clone())
        },
        _ => (inExp.clone(), inArg.clone()),
    });
    Ok((outExp, arg))
}

pub fn traverseExpShallow<ArgT: Clone + 'static>(inExp: Arc<Absyn::Exp>, inArg: ArgT, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<Arc<Absyn::Exp>> {
    pub type FuncT<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>>;

    let mut outExp: Arc<Absyn::Exp> = inExp.clone();
    let _ = (::match_deref::match_deref! { match &(outExp.clone()) {
        Deref @ Absyn::BINARY { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::BINARY;
                exp1 = inFunc(var_field!((*outExp).exp1, Absyn::Exp::BINARY).clone(), inArg.clone())?,
                exp2 = inFunc(var_field!((*outExp).exp2, Absyn::Exp::BINARY).clone(), inArg.clone())?
            );
            ()
        },
        Deref @ Absyn::UNARY { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::UNARY; exp = inFunc(var_field!((*outExp).exp, Absyn::Exp::UNARY).clone(), inArg.clone())?);
            ()
        },
        Deref @ Absyn::LBINARY { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::LBINARY;
                exp1 = inFunc(var_field!((*outExp).exp1, Absyn::Exp::LBINARY).clone(), inArg.clone())?,
                exp2 = inFunc(var_field!((*outExp).exp2, Absyn::Exp::LBINARY).clone(), inArg.clone())?
            );
            ()
        },
        Deref @ Absyn::LUNARY { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::LUNARY; exp = inFunc(var_field!((*outExp).exp, Absyn::Exp::LUNARY).clone(), inArg.clone())?);
            ()
        },
        Deref @ Absyn::RELATION { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::RELATION;
                exp1 = inFunc(var_field!((*outExp).exp1, Absyn::Exp::RELATION).clone(), inArg.clone())?,
                exp2 = inFunc(var_field!((*outExp).exp2, Absyn::Exp::RELATION).clone(), inArg.clone())?
            );
            ()
        },
        Deref @ Absyn::IFEXP { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::IFEXP;
                ifExp = inFunc(var_field!((*outExp).ifExp, Absyn::Exp::IFEXP).clone(), inArg.clone())?,
                trueBranch = inFunc(var_field!((*outExp).trueBranch, Absyn::Exp::IFEXP).clone(), inArg.clone())?,
                elseBranch = inFunc(var_field!((*outExp).elseBranch, Absyn::Exp::IFEXP).clone(), inArg.clone())?,
                elseIfBranch = {
        let mut __acc: Arc<metamodelica::List<(Arc<Absyn::Exp>, Arc<Absyn::Exp>)>> = metamodelica::nil();
        for e in (var_field!((*outExp).elseIfBranch, Absyn::Exp::IFEXP).clone()).into_iter().cloned() {
            let __x = (inFunc(Util::tuple21(e.clone()), inArg.clone())?, inFunc(Util::tuple22(e.clone()), inArg.clone())?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            ()
        },
        Deref @ Absyn::CALL { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::CALL; functionArgs = traverseExpShallowFuncArgs(var_field!((*outExp).functionArgs, Absyn::Exp::CALL).clone(), inArg.clone(), inFunc.clone())?);
            ()
        },
        Deref @ Absyn::PARTEVALFUNCTION { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::PARTEVALFUNCTION; functionArgs = traverseExpShallowFuncArgs(var_field!((*outExp).functionArgs, Absyn::Exp::PARTEVALFUNCTION).clone(), inArg.clone(), inFunc.clone())?);
            ()
        },
        Deref @ Absyn::ARRAY { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::ARRAY; arrayExp = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for e in (var_field!((*outExp).arrayExp, Absyn::Exp::ARRAY).clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::MATRIX { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::MATRIX; matrix = {
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<Arc<Absyn::Exp>>>>> = metamodelica::nil();
        for lst in (var_field!((*outExp).matrix, Absyn::Exp::MATRIX).clone()).into_iter().cloned() {
            let __x = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for e in (lst.clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::RANGE { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::RANGE;
                start = inFunc(var_field!((*outExp).start, Absyn::Exp::RANGE).clone(), inArg.clone())?,
                step = Util::applyOption1(var_field!((*outExp).step, Absyn::Exp::RANGE).clone(), inFunc.clone(), inArg.clone()),
                stop = inFunc(var_field!((*outExp).stop, Absyn::Exp::RANGE).clone(), inArg.clone())?
            );
            ()
        },
        Deref @ Absyn::TUPLE { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::TUPLE; expressions = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for e in (var_field!((*outExp).expressions, Absyn::Exp::TUPLE).clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::AS { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::AS; exp = inFunc(var_field!((*outExp).exp, Absyn::Exp::AS).clone(), inArg.clone())?);
            ()
        },
        Deref @ Absyn::CONS { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::CONS;
                head = inFunc(var_field!((*outExp).head, Absyn::Exp::CONS).clone(), inArg.clone())?,
                rest = inFunc(var_field!((*outExp).rest, Absyn::Exp::CONS).clone(), inArg.clone())?
            );
            ()
        },
        Deref @ Absyn::LIST { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::LIST; exps = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for e in (var_field!((*outExp).exps, Absyn::Exp::LIST).clone()).into_iter().cloned() {
            let __x = inFunc(e.clone(), inArg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            ()
        },
        Deref @ Absyn::DOT { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::DOT;
                exp = inFunc(var_field!((*outExp).exp, Absyn::Exp::DOT).clone(), inArg.clone())?,
                index = inFunc(var_field!((*outExp).index, Absyn::Exp::DOT).clone(), inArg.clone())?
            );
            ()
        },
        Deref @ Absyn::EXPRESSIONCOMMENT { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::EXPRESSIONCOMMENT; exp = inFunc(var_field!((*outExp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), inArg.clone())?);
            ()
        },
        Deref @ Absyn::SUBSCRIPTED_EXP { .. } => {
            let mut e1: Arc<Absyn::Exp>;
            let mut e2: Arc<Absyn::Exp>;
            assign_variant_field!(outExp => Absyn::Exp::SUBSCRIPTED_EXP;
                exp = inFunc(var_field!((*outExp).exp, Absyn::Exp::SUBSCRIPTED_EXP).clone(), inArg.clone())?,
                subscripts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Subscript>>> = metamodelica::nil();
        for s in (var_field!((*outExp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone()).into_iter().cloned() {
            let __x = traverseExpShallowSub(s.clone(), inArg.clone(), inFunc.clone());
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
    Ok(outExp)
}

fn traverseExpShallowFuncArgs<ArgT: Clone + 'static>(inArgs: Arc<Absyn::FunctionArgs>, inArg: ArgT, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<Arc<Absyn::FunctionArgs>> {
    pub type FuncT<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>>;

    let mut outArgs: Arc<Absyn::FunctionArgs> = inArgs.clone();
    outArgs = (::match_deref::match_deref! { match &(outArgs.clone()) {
        Deref @ Absyn::FUNCTIONARGS { .. } => {
            assign_variant_field!(outArgs => Absyn::FunctionArgs::FUNCTIONARGS; args = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::Exp>>> = metamodelica::nil();
        for arg in (var_field!((*outArgs).args, Absyn::FunctionArgs::FUNCTIONARGS).clone()).into_iter().cloned() {
            let __x = inFunc(arg.clone(), inArg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            outArgs.clone()
        },
        Deref @ Absyn::FOR_ITER_FARG { .. } => {
            assign_variant_field!(outArgs => Absyn::FunctionArgs::FOR_ITER_FARG;
                exp = inFunc(var_field!((*outArgs).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), inArg.clone())?,
                iterators = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ForIterator>>> = metamodelica::nil();
        for it in (var_field!((*outArgs).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone()).into_iter().cloned() {
            let __x = traverseExpShallowIterator(it.clone(), inArg.clone(), inFunc.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }
            );
            outArgs.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outArgs)
}

fn traverseExpShallowIterator<ArgT: Clone + 'static>(inIterator: Arc<Absyn::ForIterator>, inArg: ArgT, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>> + 'static>) -> Result<Arc<Absyn::ForIterator>> {
    pub type FuncT<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>>;

    let mut outIterator: Arc<Absyn::ForIterator>;
    let mut name: ArcStr;
    let mut guard_exp: Option<Arc<Absyn::Exp>>;
    let mut range_exp: Option<Arc<Absyn::Exp>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inIterator.clone()) {
        Deref @ Absyn::ITERATOR { name: __pa0, guardExp: __pa1, range: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    name = __pa0.clone();
    guard_exp = __pa1.clone();
    range_exp = __pa2.clone();
    guard_exp = Util::applyOption1(guard_exp.clone(), inFunc.clone(), inArg.clone());
    range_exp = Util::applyOption1(range_exp.clone(), inFunc.clone(), inArg.clone());
    outIterator = Arc::new(Absyn::ForIterator { name: (name.clone()).clone(), guardExp: guard_exp.clone(), range: range_exp.clone() });
    Ok(outIterator)
}

pub fn traverseExpShallowSub<ArgT: Clone + 'static>(sub: Arc<Absyn::Subscript>, inArg: ArgT, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>> + 'static>) -> Arc<Absyn::Subscript> {
    pub type FuncT<ArgT: Clone> = fn(Arc<Absyn::Exp>, ArgT) -> Result<Arc<Absyn::Exp>>;

    let mut sub: Arc<Absyn::Subscript> = sub;
    let () = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => {
            assign_variant_field!(sub => Absyn::Subscript::SUBSCRIPT; subscript = inFunc(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), inArg.clone()).unwrap());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    sub
}

pub fn traverseExpTopDown<Arg: Clone + 'static>(inExp: Arc<Absyn::Exp>, inFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, inArg: Arg) -> Result<(Arc<Absyn::Exp>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut outExp: Arc<Absyn::Exp>;
    let mut outArg: Arg;
    (outExp, outArg) = traverseExpBidir(inExp.clone(), inFunc.clone(), Arc::new(fnptr!(dummyTraverseExp, Arc<Absyn::Exp>, _)), inArg.clone())?;
    Ok((outExp, outArg))
}

fn traverseInnerClass<Arg: Clone + 'static>(inClass: Arc<Absyn::Class>, path: Option<Arc<Absyn::Path>>, visitor: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>, arg: Arg, visitProtected: bool) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> {
    pub type FuncType<Arg: Clone> = fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)>;

    let mut outTpl: (Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg);
    let mut cls: Arc<Absyn::Class> = inClass.clone();
    let mut cdef: Arc<Absyn::ClassDef> = inClass.body.clone();
    let mut pa: Arc<Absyn::Path>;
    let mut opt_pa: Option<Arc<Absyn::Path>>;
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
    let mut args: Arg;
    (cdef, opt_pa, args) = 'mc: {
        let __mc_input = (cdef.clone(), path.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::PARTS { .. }, Some(pa)) => {
                    let mut pa = (*pa).clone();
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut opt_pa: Option<Arc<Absyn::Path>>;
                    let mut args: Arg;
                    let mut cdef: Arc<Absyn::ClassDef>;
                    pa = joinPaths(pa.clone(), Arc::new(Absyn::Path::IDENT { name: (cls.name.clone()).clone() }))?;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), Some(pa.clone()), visitor.clone(), arg.clone(), visitProtected.clone())?;
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    Ok((cdef.clone(), opt_pa.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::PARTS { .. }, None) => {
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut opt_pa: Option<Arc<Absyn::Path>>;
                    let mut args: Arg;
                    let mut cdef: Arc<Absyn::ClassDef>;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), Some(Arc::new(Absyn::Path::IDENT { name: (cls.name.clone()).clone() })), visitor.clone(), arg.clone(), visitProtected.clone())?;
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    Ok((cdef.clone(), opt_pa.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::PARTS { .. }, opt_pa) => {
                    let mut opt_pa = (*opt_pa).clone();
                    let mut args: Arg;
                    let mut cdef: Arc<Absyn::ClassDef>;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone(), opt_pa.clone(), visitor.clone(), arg.clone(), visitProtected.clone())?;
                    assign_variant_field!(cdef => Absyn::ClassDef::PARTS; classParts = parts.clone());
                    Ok((cdef.clone(), opt_pa.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CLASS_EXTENDS { .. }, Some(pa)) => {
                    let mut pa = (*pa).clone();
                    let mut opt_pa: Option<Arc<Absyn::Path>>;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut args: Arg;
                    let mut cdef: Arc<Absyn::ClassDef>;
                    pa = joinPaths(pa.clone(), Arc::new(Absyn::Path::IDENT { name: (cls.name.clone()).clone() }))?;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), Some(pa.clone()), visitor.clone(), arg.clone(), visitProtected.clone())?;
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    Ok((cdef.clone(), opt_pa.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CLASS_EXTENDS { .. }, None) => {
                    let mut args: Arg;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut opt_pa: Option<Arc<Absyn::Path>>;
                    let mut cdef: Arc<Absyn::ClassDef>;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), Some(Arc::new(Absyn::Path::IDENT { name: (cls.name.clone()).clone() })), visitor.clone(), arg.clone(), visitProtected.clone())?;
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    Ok((cdef.clone(), opt_pa.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::CLASS_EXTENDS { .. }, opt_pa) => {
                    let mut opt_pa = (*opt_pa).clone();
                    let mut args: Arg;
                    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>;
                    let mut cdef: Arc<Absyn::ClassDef>;
                    (parts, opt_pa, args) = traverseInnerClassParts(var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone(), opt_pa.clone(), visitor.clone(), arg.clone(), visitProtected.clone())?;
                    assign_variant_field!(cdef => Absyn::ClassDef::CLASS_EXTENDS; parts = parts.clone());
                    Ok((cdef.clone(), opt_pa.clone(), args.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((cdef.clone(), path.clone(), arg.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    assign_field!(cls.body = cdef.clone());
    outTpl = (cls.clone(), opt_pa.clone(), args.clone());
    Ok(outTpl)
}

fn traverseInnerClassElements<Arg: Clone + 'static>(inElements: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, inPath: Option<Arc<Absyn::Path>>, visitor: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>, inArg: Arg, visitProtected: bool) -> Result<(Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, Option<Arc<Absyn::Path>>, Arg)> {
    pub type FuncType<Arg: Clone> = fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)>;

    let mut outTpl: (Arc<metamodelica::List<Arc<Absyn::ElementItem>>>, Option<Arc<Absyn::Path>>, Arg);
    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>> = metamodelica::nil();
    let mut el: Arc<Absyn::Element>;
    let mut arg: Arg = inArg.clone();
    let mut spec: Arc<Absyn::ElementSpec>;
    let mut cl: Arc<Absyn::Class>;
    for e in &*inElements.clone() {
        elts = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ Absyn::ELEMENTITEM { element: el @ Deref @ Absyn::ELEMENT { specification: spec, .. } } => {
            let mut el = (*el).clone();
            let mut spec = (*spec).clone();
            (spec, _, arg) = traverseInnerClassElementspec(spec.clone(), inPath.clone(), visitor.clone(), arg.clone(), visitProtected.clone())?;
            let __owned_variant_specification_0 = Arc::new(spec.clone());
            if let Absyn::Element::ELEMENT { specification, .. } = &mut el {
                *specification = __owned_variant_specification_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Element::ELEMENT"); }
            e.element = el.clone(); // TODO: unhandled field-assign shape
            cons(e.clone(), elts.clone())
        },
        Deref @ Absyn::ELEMENTITEM { element: el @ Deref @ Absyn::ELEMENT { specification: spec @ Deref @ Absyn::CLASSDEF { .. }, .. } } => {
            let mut el = (*el).clone();
            let mut spec = (*spec).clone();
            (cl, _, arg) = traverseInnerClass(spec.class_.clone(), inPath.clone(), visitor.clone(), arg.clone(), visitProtected.clone())?;
            spec.class_ = cl.clone(); // TODO: unhandled field-assign shape
            let __owned_variant_specification_0 = Arc::new(spec.clone());
            if let Absyn::Element::ELEMENT { specification, .. } = &mut el {
                *specification = __owned_variant_specification_0;
            } else { panic!("owned-variant field-assign: value held a different variant than Absyn::Element::ELEMENT"); }
            e.element = el.clone(); // TODO: unhandled field-assign shape
            cons(e.clone(), elts.clone())
        },
        Deref @ Absyn::ELEMENTITEM { element: Deref @ Absyn::ELEMENT { .. } } => elts.clone(),
        _ => cons(e.clone(), elts.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    }
    elts = elts.clone().reverse();
    outTpl = (elts.clone(), inPath.clone(), arg.clone());
    Ok(outTpl)
}

fn traverseInnerClassElementspec<Arg: Clone + 'static>(inElementSpec: Arc<Absyn::ElementSpec>, inPath: Option<Arc<Absyn::Path>>, visitor: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>, inArg: Arg, visitProtected: bool) -> Result<(Arc<Absyn::ElementSpec>, Option<Arc<Absyn::Path>>, Arg)> {
    pub type FuncType<Arg: Clone> = fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)>;

    let mut outTpl: (Arc<Absyn::ElementSpec>, Option<Arc<Absyn::Path>>, Arg);
    outTpl = (::match_deref::match_deref! { match &((inElementSpec.clone(), inPath.clone(), inArg.clone())) {
        (Deref @ Absyn::CLASSDEF { replaceable_: repl, class_: cl }, pa, args) => {
            let mut cl = (*cl).clone();
            let mut pa = (*pa).clone();
            let mut args = (*args).clone();
            (cl, _, args) = visitor((cl.clone(), pa.clone(), args.clone()))?;
            (cl, pa, args) = traverseInnerClass(cl.clone(), pa.clone(), visitor.clone(), args.clone(), visitProtected.clone())?;
            (Arc::new(Absyn::ElementSpec::CLASSDEF { replaceable_: repl.clone(), class_: cl.clone() }), pa.clone(), args.clone())
        },
        (Deref @ Absyn::EXTENDS { .. }, pa, args) => (inElementSpec.clone(), pa.clone(), args.clone()),
        (Deref @ Absyn::IMPORT { .. }, pa, args) => (inElementSpec.clone(), pa.clone(), args.clone()),
        (Deref @ Absyn::COMPONENTS { .. }, pa, args) => (inElementSpec.clone(), pa.clone(), args.clone()),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTpl)
}

fn traverseInnerClassParts<Arg: Clone + 'static>(inClassParts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, inPath: Option<Arc<Absyn::Path>>, visitor: Arc<dyn ::std::ops::Fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)> + 'static>, inArg: Arg, visitProtected: bool) -> Result<(Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, Option<Arc<Absyn::Path>>, Arg)> {
    pub type FuncType<Arg: Clone> = fn((Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)) -> Result<(Arc<Absyn::Class>, Option<Arc<Absyn::Path>>, Arg)>;

    let mut outTpl: (Arc<metamodelica::List<Arc<Absyn::ClassPart>>>, Option<Arc<Absyn::Path>>, Arg);
    let mut parts: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
    let mut elts: Arc<metamodelica::List<Arc<Absyn::ElementItem>>>;
    let mut arg: Arg = inArg.clone();
    parts = {
        let mut __acc: Arc<metamodelica::List<Arc<Absyn::ClassPart>>> = metamodelica::nil();
        for p in (inClassParts.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(p.clone()) {
        Deref @ Absyn::PUBLIC { .. } => {
            (elts, _, arg) = traverseInnerClassElements(var_field!(p.contents, Absyn::ClassPart::PUBLIC).clone(), inPath.clone(), visitor.clone(), arg.clone(), visitProtected.clone())?;
            Arc::new(Absyn::ClassPart::PUBLIC { contents: elts.clone() })
        },
        Deref @ Absyn::PROTECTED { .. } if (visitProtected.clone()) => {
            (elts, _, arg) = traverseInnerClassElements(var_field!(p.contents, Absyn::ClassPart::PROTECTED).clone(), inPath.clone(), visitor.clone(), arg.clone(), true)?;
            Arc::new(Absyn::ClassPart::PROTECTED { contents: elts.clone() })
        },
        _ => p.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    };
    outTpl = (parts.clone(), inPath.clone(), arg.clone());
    Ok(outTpl)
}

fn traverseListGeneric<T: Clone + 'static + PartialEq, ArgT: Clone + 'static>(inList: Arc<metamodelica::List<T>>, inFunc: Arc<dyn ::std::ops::Fn(T, ArgT) -> Result<(T, ArgT, bool)> + 'static>, inArg: ArgT) -> Result<(Arc<metamodelica::List<T>>, ArgT, bool)> {
    pub type FuncType<T: Clone, ArgT: Clone> = fn(T, ArgT) -> Result<(T, ArgT, bool)>;

    let mut outList: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut outArg: ArgT = inArg.clone();
    let mut outContinue: bool = true;
    let mut eq: bool;
    let mut changed: bool = false;
    let mut e: T;
    let mut new_e: T;
    let mut rest_e: Arc<metamodelica::List<T>> = inList.clone();
    while !(rest_e.clone().is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        e = __pa0.clone();
        rest_e = __pa1.clone();
        (new_e, outArg, outContinue) = inFunc(e.clone(), outArg.clone())?;
        eq = referenceEq(&new_e.clone(),&e.clone());
        outList = cons(if (eq.clone()) {e.clone()} else {new_e.clone()}, outList.clone());
        changed = changed.clone() || !(eq.clone());
        if !(outContinue.clone()) {
            break;
        }
    }
    if changed.clone() {
        outList = List::append_reverse(outList.clone(), rest_e.clone());
    } else {
        outList = inList.clone();
    }
    Ok((outList, outArg, outContinue))
}

pub fn traverseMatchCase<Arg: Clone + 'static>(matchCase: Arc<Absyn::Case>, enterFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, exitFunc: Arc<dyn ::std::ops::Fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)> + 'static>, arg: Arg) -> Result<(Arc<Absyn::Case>, Arg)> {
    pub type FuncType<Arg: Clone> = fn(Arc<Absyn::Exp>, Arg) -> Result<(Arc<Absyn::Exp>, Arg)>;

    let mut matchCase: Arc<Absyn::Case> = matchCase;
    let mut arg: Arg = arg;
    (matchCase, arg) = (::match_deref::match_deref! { match &(matchCase.clone()) {
        Deref @ Absyn::CASE { pattern, patternGuard, patternInfo: pinfo, localDecls: ldecls, classPart: cp, result, resultInfo, comment: cmt, info } => {
            let mut pattern = (*pattern).clone();
            let mut patternGuard = (*patternGuard).clone();
            let mut cp = (*cp).clone();
            let mut result = (*result).clone();
            (pattern, arg) = traverseExpBidir(pattern.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (patternGuard, arg) = traverseExpOptBidir(patternGuard.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (cp, arg) = traverseClassPartBidir(cp.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (result, arg) = traverseExpBidir(result.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (Arc::new(Absyn::Case::CASE { pattern: pattern.clone(), patternGuard: patternGuard.clone(), patternInfo: pinfo.clone(), localDecls: ldecls.clone(), classPart: cp.clone(), result: result.clone(), resultInfo: resultInfo.clone(), comment: cmt.clone(), info: info.clone() }), arg.clone())
        },
        Deref @ Absyn::ELSE { localDecls: ldecls, classPart: cp, result, resultInfo, comment: cmt, info } => {
            let mut pattern: Arc<Absyn::Exp>;
            let mut pinfo: SourceInfo;
            let mut patternGuard: Option<Arc<Absyn::Exp>>;
            let mut cp = (*cp).clone();
            let mut result = (*result).clone();
            (cp, arg) = traverseClassPartBidir(cp.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (result, arg) = traverseExpBidir(result.clone(), enterFunc.clone(), exitFunc.clone(), arg.clone())?;
            (Arc::new(Absyn::Case::ELSE { localDecls: ldecls.clone(), classPart: cp.clone(), result: result.clone(), resultInfo: resultInfo.clone(), comment: cmt.clone(), info: info.clone() }), arg.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((matchCase, arg))
}

pub fn typeSpecDimensions(inTypeSpec: Arc<Absyn::TypeSpec>) -> Arc<metamodelica::List<Arc<Absyn::Subscript>>> {
    let mut outDimensions: Arc<metamodelica::List<Arc<Absyn::Subscript>>>;
    outDimensions = (::match_deref::match_deref! { match &(inTypeSpec.clone()) {
        Deref @ Absyn::TPATH { arrayDim: Some(dim), .. } => dim.clone(),
        Deref @ Absyn::TCOMPLEX { arrayDim: Some(dim), .. } => dim.clone(),
        _ => metamodelica::nil(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outDimensions
}

pub fn typeSpecEqual(a: Arc<Absyn::TypeSpec>, b: Arc<Absyn::TypeSpec>) -> bool {
    let mut ob: bool;
    ob = (::match_deref::match_deref! { match &((a.clone(), b.clone())) {
        (Deref @ Absyn::TPATH { .. }, Deref @ Absyn::TPATH { .. }) => pathEqual(var_field!((*a).path, Absyn::TypeSpec::TPATH).clone(), var_field!((*b).path, Absyn::TypeSpec::TPATH).clone()) && optArrayDimEqual(var_field!((*a).arrayDim, Absyn::TypeSpec::TPATH).clone(), var_field!((*b).arrayDim, Absyn::TypeSpec::TPATH).clone()),
        (Deref @ Absyn::TCOMPLEX { .. }, Deref @ Absyn::TCOMPLEX { .. }) => pathEqual(var_field!((*a).path, Absyn::TypeSpec::TCOMPLEX).clone(), var_field!((*b).path, Absyn::TypeSpec::TCOMPLEX).clone()) && List::isEqualOnTrue(var_field!((*a).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone(), var_field!((*b).typeSpecs, Absyn::TypeSpec::TCOMPLEX).clone(), Arc::new(fnptr!(typeSpecEqual, Arc<Absyn::TypeSpec>, Arc<Absyn::TypeSpec>))) && optArrayDimEqual(var_field!((*a).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone(), var_field!((*b).arrayDim, Absyn::TypeSpec::TCOMPLEX).clone()),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    ob
}

pub fn typeSpecPath(tp: Arc<Absyn::TypeSpec>) -> Result<Arc<Absyn::Path>> {
    let mut op: Arc<Absyn::Path>;
    op = (::match_deref::match_deref! { match &(tp.clone()) {
        Deref @ Absyn::TCOMPLEX { .. } => var_field!((*tp).path, Absyn::TypeSpec::TCOMPLEX).clone(),
        Deref @ Absyn::TPATH { .. } => var_field!((*tp).path, Absyn::TypeSpec::TPATH).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(op)
}

pub fn typeSpecPathString(tp: Arc<Absyn::TypeSpec>) -> ArcStr {
    let mut s: ArcStr = pathString(typeSpecPath(tp.clone()).unwrap(), (literal!(".")).clone(), true, false).unwrap();
    s
}

pub fn typeSpecString(inTs: Arc<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = (Dump::unparseTypeSpec(inTs.clone())?).clone();
    Ok(outStr)
}

pub fn typeSpecStringNoQualNoDims(inTs: Arc<Absyn::TypeSpec>) -> Result<ArcStr> {
    let mut outStr: ArcStr;
    outStr = ((::match_deref::match_deref! { match &(inTs.clone()) {
        Deref @ Absyn::TPATH { path, .. } => pathString(makeNotFullyQualified(path.clone()), (literal!(".")).clone(), true, false)?,
        Deref @ Absyn::TCOMPLEX { typeSpecs: typeSpecLst, path, .. } => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = (pathString(makeNotFullyQualified(path.clone()), (literal!(".")).clone(), true, false)?).clone();
            str2 = (typeSpecStringNoQualNoDimsLst(typeSpecLst.clone())?).clone();
            stringAppendList(list![(str1.clone()).clone(), (literal!("<")).clone(), (str2.clone()).clone(), (literal!(">")).clone()])
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outStr)
}

pub fn typeSpecStringNoQualNoDimsLst(inTypeSpecLst: Arc<metamodelica::List<Arc<Absyn::TypeSpec>>>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (List::toString(inTypeSpecLst.clone(), Arc::new(typeSpecStringNoQualNoDims), (literal!("")).clone(), (literal!("")).clone(), (literal!(", ")).clone(), (literal!("")).clone(), false, 0)?).clone();
    Ok(outString)
}

pub fn unqotePathIdents(inPath: Arc<Absyn::Path>) -> Result<Arc<Absyn::Path>> {
    let mut path: Arc<Absyn::Path>;
    path = stringListPath(List::map(pathToStringList(inPath.clone())?, Arc::new(fnptr!(System::unquoteIdentifier, ArcStr))));
    Ok(path)
}

pub fn unqualifyCref(inCref: Arc<Absyn::ComponentRef>) -> Arc<Absyn::ComponentRef> {
    let mut outCref: Arc<Absyn::ComponentRef>;
    outCref = (::match_deref::match_deref! { match &(inCref.clone()) {
        Deref @ Absyn::CREF_FULLYQUALIFIED { .. } => var_field!((*inCref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(),
        _ => inCref.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outCref
}

pub fn withinEqual(within1: Absyn::Within, within2: Absyn::Within) -> bool {
    let mut b: bool;
    b = (match (within1.clone(), within2.clone()) {
        (Absyn::TOP, Absyn::TOP) => true,
        (Absyn::WITHIN { .. }, Absyn::WITHIN { .. }) => pathEqual(var_field!(within1.path, Absyn::Within::WITHIN).clone(), var_field!(within2.path, Absyn::Within::WITHIN).clone()),
        _ => false,
    });
    b
}

pub fn withinEqualCaseInsensitive(within1: Absyn::Within, within2: Absyn::Within) -> bool {
    let mut b: bool;
    b = (match (within1.clone(), within2.clone()) {
        (Absyn::TOP, Absyn::TOP) => true,
        (Absyn::WITHIN { .. }, Absyn::WITHIN { .. }) => pathEqualCaseInsensitive(var_field!(within1.path, Absyn::Within::WITHIN).clone(), var_field!(within2.path, Absyn::Within::WITHIN).clone()),
        _ => false,
    });
    b
}

pub fn withinString(w1: Absyn::Within) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = ((match w1.clone() {
        Absyn::TOP => literal!("within ;"),
        Absyn::WITHIN { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("within ")); __mm_s.push_str(&*pathString(var_field!(w1.path, Absyn::Within::WITHIN).clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

