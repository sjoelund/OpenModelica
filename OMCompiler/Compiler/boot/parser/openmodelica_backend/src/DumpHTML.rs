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

use crate::BackendDAE;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVariable;
use openmodelica_frontend_types::DAE;
use openmodelica_util::ExpandableArray;
use openmodelica_util::System;
use openmodelica_util_datatypes_basic::List;

// =============================================================================
// types
//
// =============================================================================
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Style {
    pub name: ArcStr,
    pub value: ArcStr,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            name: Default::default(),
            value: Default::default(),
        }
    }
}

pub type STYLE = Style;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tag {
    HEADING {
        stage: i32,
        text: ArcStr,
    },
    HYPERLINK {
        /// #anker or javascript:toggle
        href: ArcStr,
        title: ArcStr,
        text: ArcStr,
    },
    ANKER {
        name: ArcStr,
    },
    LINE {
        text: ArcStr,
    },
    DIVISION {
        id: ArcStr,
        style: Arc<metamodelica::List<Style>>,
        tags: Arc<metamodelica::List<Arc<Tag>>>,
    },
    SCRIPT {
        type_: ArcStr,
        text: ArcStr,
    },
    SCRIPT_BODY {
        type_: ArcStr,
        text: ArcStr,
    },
    CANVAS {
        attr: Arc<metamodelica::List<ArcStr>>,
    },
}
impl Default for Tag {
    fn default() -> Self {
        Self::ANKER {
            name: Default::default(),
        }
    }
}
pub use self::Tag::{HEADING,HYPERLINK,ANKER,LINE,DIVISION,SCRIPT,SCRIPT_BODY,CANVAS};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Document {
    pub docType: ArcStr,
    /// because of performance issues tags in reverse order
    pub head: Arc<metamodelica::List<Arc<Tag>>>,
    /// because of performance issues tags in reverse order
    pub body: Arc<metamodelica::List<Arc<Tag>>>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            docType: Default::default(),
            head: Default::default(),
            body: Default::default(),
        }
    }
}

pub type DOCUMENT = Document;


pub static emptyDocument: std::sync::LazyLock<Document> = std::sync::LazyLock::new(|| { Document { docType: (literal!("")).clone(), head: metamodelica::nil(), body: metamodelica::nil() } });

fn emptyDocumentWithToggleFunktion() -> Result<Document> {
    let mut outDoc: Document;
    outDoc = addScript((literal!("text/Javascript")).clone(), (literal!("function toggle(name) {\n   var element = document.getElementById(name);\n   if (element.style.display == \"none\") {\n      // show the div\n      element.style.display = \"block\";   \n   } else {\n      // hide the div\n      element.style.display = \"none\";\n      // reset element\n      element.reset();\n   }\n}\n\nfunction show(name) {\n   var element = document.getElementById(name);\n   if (element.style.display == \"none\") {\n      // show the div\n      element.style.display = \"block\";   \n   }\n   return true;\n}\n\n    ")).clone(), emptyDocument.clone())?;
    Ok(outDoc)
}

fn addScript(mut type_: ArcStr, mut script: ArcStr, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    outDoc = addHeadTag(Arc::new(Tag::SCRIPT { type_: (type_.clone()).clone(), text: (script.clone()).clone() }), inDoc.clone())?;
    Ok(outDoc)
}

fn addScriptBody(mut type_: ArcStr, mut script: ArcStr, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    outDoc = addBodyTag(Arc::new(Tag::SCRIPT_BODY { type_: (type_.clone()).clone(), text: (script.clone()).clone() }), inDoc.clone())?;
    Ok(outDoc)
}

fn addHeading(mut stage: i32, mut text: ArcStr, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    outDoc = addBodyTag(Arc::new(Tag::HEADING { stage: stage.clone(), text: (text.clone()).clone() }), inDoc.clone())?;
    Ok(outDoc)
}

fn addHeadingTag(mut stage: i32, mut text: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    outTags = cons(Arc::new(Tag::HEADING { stage: stage.clone(), text: (text.clone()).clone() }), inTags.clone());
    outTags
}

fn addHyperLink(mut href: ArcStr, mut title: ArcStr, mut text: ArcStr, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    outDoc = addBodyTag(Arc::new(Tag::HYPERLINK { href: (href.clone()).clone(), title: (title.clone()).clone(), text: (text.clone()).clone() }), inDoc.clone())?;
    Ok(outDoc)
}

fn addHyperLinkTag(mut href: ArcStr, mut title: ArcStr, mut text: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    outTags = cons(Arc::new(Tag::HYPERLINK { href: (href.clone()).clone(), title: (title.clone()).clone(), text: (text.clone()).clone() }), inTags.clone());
    outTags
}

fn addAnkerTag(mut name: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    outTags = cons(Arc::new(Tag::ANKER { name: (name.clone()).clone() }), inTags.clone());
    outTags
}

fn addLine(mut text: ArcStr, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    outDoc = addBodyTag(Arc::new(Tag::LINE { text: (text.clone()).clone() }), inDoc.clone())?;
    Ok(outDoc)
}

fn addLineTag(mut text: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    outTags = cons(Arc::new(Tag::LINE { text: (text.clone()).clone() }), inTags.clone());
    outTags
}

fn addDivision(mut id: ArcStr, mut style: Arc<metamodelica::List<Style>>, mut tags: Arc<metamodelica::List<Arc<Tag>>>, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    let mut t: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    t = tags.clone().reverse();
    outDoc = addBodyTag(Arc::new(Tag::DIVISION { id: (id.clone()).clone(), style: style.clone(), tags: t.clone() }), inDoc.clone())?;
    Ok(outDoc)
}

fn addDivisionTag(mut id: ArcStr, mut style: Arc<metamodelica::List<Style>>, mut tags: Arc<metamodelica::List<Arc<Tag>>>, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut t: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    t = tags.clone().reverse();
    outTags = cons(Arc::new(Tag::DIVISION { id: (id.clone()).clone(), style: style.clone(), tags: t.clone() }), inTags.clone());
    outTags
}

fn addBodyTags(mut tags: Arc<metamodelica::List<Arc<Tag>>>, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    let mut docType: ArcStr = arcstr::literal!("");
    let mut head: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut body: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut t: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    t = tags.clone().reverse();
    let Document { body: __pa0, head: __pa1, docType: __pa2 } = (inDoc.clone()) else { bail!("pattern mismatch") };
    body = __pa0.clone();
    head = __pa1.clone();
    docType = __pa2.clone();
    outDoc = Document { docType: (docType.clone()).clone(), head: head.clone(), body: listAppend(body.clone(), t.clone()) };
    Ok(outDoc)
}

fn dumpDocument(mut inDoc: Document, mut name: ArcStr) -> Result<()> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut head: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut body: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let Document { body: __pa0, head: __pa1, docType: __pa2 } = (inDoc.clone()) else { bail!("pattern mismatch") };
    body = __pa0.clone();
    head = __pa1.clone();
    r#str = __pa2.clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n<html>\n<head>")); ArcStr::from(__mm_s) }).clone();
    r#str = (List::fold(head.clone().reverse(), Arc::new(dumpTag), (r#str.clone()).clone())).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n</head>")); ArcStr::from(__mm_s) }).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n<body>")); ArcStr::from(__mm_s) }).clone();
    r#str = (List::fold(body.clone().reverse(), Arc::new(dumpTag), (r#str.clone()).clone())).clone();
    r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*literal!("\n</body>\n</html>")); ArcStr::from(__mm_s) }).clone();
    System::writeFile((name.clone()).clone(), (r#str.clone()).clone())?;
    Ok(())
}

fn addHeadTag(mut tag: Arc<Tag>, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    let mut docType: ArcStr = arcstr::literal!("");
    let mut head: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut body: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let Document { body: __pa0, head: __pa1, docType: __pa2 } = (inDoc.clone()) else { bail!("pattern mismatch") };
    body = __pa0.clone();
    head = __pa1.clone();
    docType = __pa2.clone();
    outDoc = Document { docType: (docType.clone()).clone(), head: cons(tag.clone(), head.clone()), body: body.clone() };
    Ok(outDoc)
}

fn addBodyTag(mut tag: Arc<Tag>, mut inDoc: Document) -> Result<Document> {
    let mut outDoc: Document;
    let mut docType: ArcStr = arcstr::literal!("");
    let mut head: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut body: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let Document { body: __pa0, head: __pa1, docType: __pa2 } = (inDoc.clone()) else { bail!("pattern mismatch") };
    body = __pa0.clone();
    head = __pa1.clone();
    docType = __pa2.clone();
    outDoc = Document { docType: (docType.clone()).clone(), head: head.clone(), body: cons(tag.clone(), body.clone()) };
    Ok(outDoc)
}

fn dumpTag(mut tag: Arc<Tag>, mut iBuffer: ArcStr) -> Result<ArcStr> {
    let mut oBuffer: ArcStr = arcstr::literal!("");
    oBuffer = ((::match_deref::match_deref! { match &((tag.clone(), iBuffer.clone())) {
        (Deref @ Tag::HEADING { text: t, stage: i }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n<h")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(">")); __mm_s.push_str(&*t.clone()); __mm_s.push_str(&*literal!("</h")); __mm_s.push_str(&*intString(i.clone())); __mm_s.push_str(&*literal!(">")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ Tag::HYPERLINK { text: t2, title: t1, href: t }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n<a href=\"")); __mm_s.push_str(&*t.clone()); __mm_s.push_str(&*literal!("\" title=\"")); __mm_s.push_str(&*t1.clone()); __mm_s.push_str(&*literal!("\">")); __mm_s.push_str(&*t2.clone()); __mm_s.push_str(&*literal!("</a>")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ Tag::ANKER { name: t }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n<a name=\"")); __mm_s.push_str(&*t.clone()); __mm_s.push_str(&*literal!("\"/>")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ Tag::LINE { text: t }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*t.clone()); __mm_s.push_str(&*literal!("<br>")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ Tag::DIVISION { tags, style, id: t }, _) => {
            let mut t1: ArcStr = arcstr::literal!("");
            let mut t2: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            t1 = stringDelimitList(List::map(style.clone(), Arc::new(dumpStyle)), (literal!("; ")).clone());
            t2 = (List::fold(tags.clone(), Arc::new(dumpTag), (literal!("")).clone())).clone();
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n<div id=\"")); __mm_s.push_str(&*t.clone()); __mm_s.push_str(&*literal!("\" style=\"")); __mm_s.push_str(&*t1.clone()); __mm_s.push_str(&*literal!("\">\n")); __mm_s.push_str(&*t2.clone()); __mm_s.push_str(&*literal!("\n</div>")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ Tag::SCRIPT { text: t2, type_: t1 }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n<script type=\"")); __mm_s.push_str(&*t1.clone()); __mm_s.push_str(&*literal!("\">\n")); __mm_s.push_str(&*t2.clone()); __mm_s.push_str(&*literal!("\n</script>")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ Tag::SCRIPT_BODY { text: t2, type_: t1 }, _) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n<SCRIPT \"")); __mm_s.push_str(&*t1.clone()); __mm_s.push_str(&*literal!("\">\n")); __mm_s.push_str(&*t2.clone()); __mm_s.push_str(&*literal!("\n</SCRIPT>")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        (Deref @ Tag::CANVAS { attr }, _) => {
            let mut t1: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            t1 = stringDelimitList(attr.clone(), (literal!(" ")).clone());
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*iBuffer.clone()); __mm_s.push_str(&*literal!("\n<canvas ")); __mm_s.push_str(&*t1.clone()); __mm_s.push_str(&*literal!("\">\n")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(oBuffer)
}

fn dumpStyle(mut inStyle: Style) -> Result<ArcStr> {
    let mut outBuffer: ArcStr = arcstr::literal!("");
    let mut name: ArcStr = arcstr::literal!("");
    let mut value: ArcStr = arcstr::literal!("");
    let Style { name: __pa0, value: __pa1 } = (inStyle.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    value = __pa1.clone();
    outBuffer = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*name.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*value.clone()); ArcStr::from(__mm_s) }).clone();
    Ok(outBuffer)
}

pub fn dumpDAE(mut inDAE: Arc<BackendDAE::BackendDAE>, mut inHeader: ArcStr, mut inFilename: ArcStr) -> Result<()> {
    let mut doc: Document;
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut eqs: Arc<metamodelica::List<Arc<BackendDAE::EqSystem>>> = metamodelica::nil();
    let __pa0 = ::match_deref::match_deref! { match &(inDAE.clone()) {
        Deref @ DAE { eqs: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    eqs = __pa0.clone();
    doc = emptyDocumentWithToggleFunktion()?;
    doc = addHeading(1, (inHeader.clone()).clone(), doc.clone())?;
    r#str = (intString(((System::time()).0 as i32))).clone();
    (doc, _) = List::fold1(eqs.clone(), Arc::new(dumpEqSystem), (r#str.clone()).clone(), (doc.clone(), 1));
    dumpDocument(doc.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*r#str.clone()); __mm_s.push_str(&*inFilename.clone()); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn dumpEqSystem(mut inEqSystem: Arc<BackendDAE::EqSystem>, mut inPrefixIdstr: ArcStr, mut inTpl: (Document, i32)) -> Result<(Document, i32)> {
    let mut outTpl: (Document, i32);
    let mut vars: Arc<metamodelica::List<BackendDAE::Var>> = metamodelica::nil();
    let mut eqnlen: i32 = 0;
    let mut eqnssize: i32 = 0;
    let mut i: i32 = 0;
    let mut varlen_str: ArcStr = arcstr::literal!("");
    let mut eqnlen_str: ArcStr = arcstr::literal!("");
    let mut prefixIdstr: ArcStr = arcstr::literal!("");
    let mut prefixId: ArcStr = arcstr::literal!("");
    let mut eqnsl: Arc<metamodelica::List<Arc<BackendDAE::Equation>>> = metamodelica::nil();
    let mut vars1: BackendDAE::Variables;
    let mut eqns: Arc<ExpandableArray::ExpandableArray<Arc<BackendDAE::Equation>>>;
    let mut m: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> = None;
    let mut mT: Option<metamodelica::Array<Arc<metamodelica::List<i32>>>> = None;
    let mut matching: Arc<BackendDAE::Matching> = Arc::new(BackendDAE::Matching::NO_MATCHING);
    let mut doc: Document;
    let mut tags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(inEqSystem.clone()) {
        Deref @ BackendDAE::EqSystem { matching: __pa0, mT: __pa1, m: __pa2, orderedEqs: __pa3, orderedVars: __pa4, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => bail!("pattern mismatch"),
    } };
    matching = __pa0.clone();
    mT = __pa1.clone();
    m = __pa2.clone();
    eqns = __pa3.clone();
    vars1 = __pa4.clone();
    (doc, i) = inTpl.clone();
    prefixId = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inPrefixIdstr.clone()); __mm_s.push_str(&*literal!("_")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone();
    vars = BackendVariable::varList(vars1.clone())?;
    varlen_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variables (")); __mm_s.push_str(&*intString((vars.clone().len() as i32))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    tags = addHeadingTag(2, (varlen_str.clone()).clone(), metamodelica::nil());
    tags = printVarList(vars.clone(), (prefixId.clone()).clone(), tags.clone());
    eqnsl = BackendEquation::equationList(eqns.clone());
    eqnlen_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Equations (")); __mm_s.push_str(&*intString((eqnsl.clone().len() as i32))); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(BackendEquation::equationArraySize(eqns.clone())?)); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone();
    tags = addHeadingTag(2, (eqnlen_str.clone()).clone(), tags.clone());
    tags = dumpEqns(eqnsl.clone(), (prefixId.clone()).clone(), tags.clone());
    tags = dumpFullMatching(matching.clone(), (prefixId.clone()).clone(), tags.clone())?;
    doc = addLine((literal!("<hr>")).clone(), doc.clone())?;
    doc = addHyperLink(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("javascript:toggle('")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("system')")); ArcStr::from(__mm_s) }).clone(), (literal!("show system")).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("show/hide system ")); __mm_s.push_str(&*intString(i.clone())); ArcStr::from(__mm_s) }).clone(), doc.clone())?;
    doc = addDivision(({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("system")); ArcStr::from(__mm_s) }).clone(), list![Style { name: (literal!("display")).clone(), value: (literal!("none")).clone() }], tags.clone(), doc.clone())?;
    outTpl = (doc.clone(), i.clone() + 1);
    Ok(outTpl)
}

fn printVarList(mut vars: Arc<metamodelica::List<BackendDAE::Var>>, mut prefixId: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut tags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    (tags, _) = List::fold1(vars.clone(), Arc::new(dumpVar), (prefixId.clone()).clone(), (metamodelica::nil(), 1));
    outTags = addHyperLinkTag(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("javascript:toggle('")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("variables')")); ArcStr::from(__mm_s) }).clone(), (literal!("show variables")).clone(), (literal!("show/hide variables")).clone(), inTags.clone());
    outTags = addDivisionTag(({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("variables")); ArcStr::from(__mm_s) }).clone(), list![Style { name: (literal!("background")).clone(), value: (literal!("#FFFFCC")).clone() }, Style { name: (literal!("display")).clone(), value: (literal!("none")).clone() }], tags.clone(), outTags.clone());
    outTags
}

fn dumpVar(mut inVar: BackendDAE::Var, mut prefixId: ArcStr, mut inTpl: (Arc<metamodelica::List<Arc<Tag>>>, i32)) -> Result<(Arc<metamodelica::List<Arc<Tag>>>, i32)> {
    let mut oTpl: (Arc<metamodelica::List<Arc<Tag>>>, i32);
    let mut tags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut ln: ArcStr = arcstr::literal!("");
    let mut istr: ArcStr = arcstr::literal!("");
    (tags, i) = inTpl.clone();
    istr = (intString(i.clone())).clone();
    ln = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("varanker")); __mm_s.push_str(&*istr.clone()); ArcStr::from(__mm_s) }).clone();
    tags = addAnkerTag((ln.clone()).clone(), tags.clone());
    ln = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*istr.clone()); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*BackendDump::varString(inVar.clone())?); ArcStr::from(__mm_s) }).clone();
    tags = addLineTag((ln.clone()).clone(), tags.clone());
    oTpl = (tags.clone(), i.clone() + 1);
    Ok(oTpl)
}

fn dumpEqns(mut eqns: Arc<metamodelica::List<Arc<BackendDAE::Equation>>>, mut prefixId: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut tags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    (tags, _) = List::fold1(eqns.clone(), Arc::new(dumpEqn), (prefixId.clone()).clone(), (metamodelica::nil(), 1));
    outTags = addHyperLinkTag(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("javascript:toggle('")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("equations')")); ArcStr::from(__mm_s) }).clone(), (literal!("show equations")).clone(), (literal!("show/hide equations")).clone(), inTags.clone());
    outTags = addDivisionTag(({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("equations")); ArcStr::from(__mm_s) }).clone(), list![Style { name: (literal!("background")).clone(), value: (literal!("#C0C0C0")).clone() }, Style { name: (literal!("display")).clone(), value: (literal!("none")).clone() }], tags.clone(), outTags.clone());
    outTags
}

fn dumpEqn(mut inEquation: Arc<BackendDAE::Equation>, mut prefixId: ArcStr, mut inTpl: (Arc<metamodelica::List<Arc<Tag>>>, i32)) -> Result<(Arc<metamodelica::List<Arc<Tag>>>, i32)> {
    let mut oTpl: (Arc<metamodelica::List<Arc<Tag>>>, i32);
    let mut tags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut i: i32 = 0;
    let mut ln: ArcStr = arcstr::literal!("");
    let mut istr: ArcStr = arcstr::literal!("");
    (tags, i) = inTpl.clone();
    istr = (intString(i.clone())).clone();
    ln = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("eqanker")); __mm_s.push_str(&*istr.clone()); ArcStr::from(__mm_s) }).clone();
    tags = addAnkerTag((ln.clone()).clone(), tags.clone());
    ln = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*istr.clone()); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BackendEquation::equationSize(inEquation.clone())?)); __mm_s.push_str(&*literal!("): ")); __mm_s.push_str(&*BackendDump::equationString(inEquation.clone())?); ArcStr::from(__mm_s) }).clone();
    tags = addLineTag((ln.clone()).clone(), tags.clone());
    oTpl = (tags.clone(), i.clone() + 1);
    Ok(oTpl)
}

fn dumpFullMatching(mut inMatch: Arc<BackendDAE::Matching>, mut prefixId: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Result<Arc<metamodelica::List<Arc<Tag>>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    outTags = (::match_deref::match_deref! { match &(inMatch.clone()) {
        Deref @ BackendDAE::Matching::NO_MATCHING => {
            inTags.clone()
        },
        Deref @ BackendDAE::Matching::MATCHING { ass1, ass2: _, comps: _ } => {
            let mut tags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
            tags = dumpMatching(ass1.clone(), (prefixId.clone()).clone(), inTags.clone());
            tags.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTags)
}

fn dumpMatching(mut v: metamodelica::Array<i32>, mut prefixId: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut len: i32 = 0;
    let mut len_str: ArcStr = arcstr::literal!("");
    let mut tags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    outTags = addHeadingTag(2, (literal!("Matching")).clone(), inTags.clone());
    len = (v.clone().borrow().len() as i32);
    len_str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*intString(len.clone())); __mm_s.push_str(&*literal!(" variables and equations\n")); ArcStr::from(__mm_s) }).clone();
    outTags = addLineTag((len_str.clone()).clone(), outTags.clone());
    tags = dumpMatching2(v.clone(), 1, len.clone(), (prefixId.clone()).clone(), metamodelica::nil());
    outTags = addHyperLinkTag(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("javascript:toggle('")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("matching')")); ArcStr::from(__mm_s) }).clone(), (literal!("show matching")).clone(), (literal!("show/hide matching")).clone(), outTags.clone());
    outTags = addDivisionTag(({ let mut __mm_s = String::new(); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("matching")); ArcStr::from(__mm_s) }).clone(), list![Style { name: (literal!("background")).clone(), value: (literal!("#339966")).clone() }, Style { name: (literal!("display")).clone(), value: (literal!("none")).clone() }], tags.clone(), outTags.clone());
    outTags
}

fn dumpMatching2(mut v: metamodelica::Array<i32>, mut i: i32, mut len: i32, mut prefixId: ArcStr, mut inTags: Arc<metamodelica::List<Arc<Tag>>>) -> Arc<metamodelica::List<Arc<Tag>>> {
    let mut outTags: Arc<metamodelica::List<Arc<Tag>>> = metamodelica::nil();
    let mut eqn: i32 = 0;
    let mut s: ArcStr = arcstr::literal!("");
    let mut s2: ArcStr = arcstr::literal!("");
    match '__try0: {
        let true = (intLe(i.clone(), len.clone())) else { break '__try0 Err::<_, _>(anyhow::anyhow!("pattern mismatch")) };
        s = (intString(i.clone())).clone();
        eqn = v.borrow()[(i.clone()-1) as usize].clone();
        s2 = (intString(eqn.clone())).clone();
        s = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable <a href=\"#")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("varanker")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("\" onclick=\"return show('")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("variables');\">")); __mm_s.push_str(&*s.clone()); __mm_s.push_str(&*literal!("</a> is solved in equation  <a href=\"#")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("eqanker")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("\" onclick=\"return show('")); __mm_s.push_str(&*prefixId.clone()); __mm_s.push_str(&*literal!("equations');\">")); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!("</a>")); ArcStr::from(__mm_s) }).clone();
        outTags = dumpMatching2(v.clone(), i.clone() + 1, len.clone(), (prefixId.clone()).clone(), cons(Arc::new(Tag::LINE { text: (s.clone()).clone() }), inTags.clone()));
        Ok::<_, anyhow::Error>((outTags.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outTags = __try0_o0;
        }
        Err(_) => {
            outTags = inTags.clone();
        }
    }
    outTags
}

pub fn dumpMatrixHTML(mut m: metamodelica::Array<Arc<metamodelica::List<i32>>>, mut rowNames: Arc<metamodelica::List<ArcStr>>, mut columNames: Arc<metamodelica::List<ArcStr>>, mut fileName: ArcStr) -> Result<()> {
    let mut size: i32 = 0;
    let mut rowIdx: i32 = 0;
    let mut colIdx: i32 = 0;
    let mut matrixMargin: i32 = 0;
    let mut blockSize: i32 = 0;
    let mut row: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut color: ArcStr = arcstr::literal!("");
    let mut rowLabel: ArcStr = arcstr::literal!("");
    let mut colLabel: ArcStr = arcstr::literal!("");
    let mut blockDraw: ArcStr = arcstr::literal!("");
    let mut rowLabelDraw: ArcStr = arcstr::literal!("");
    let mut colLabelDraw: ArcStr = arcstr::literal!("");
    let mut scripts: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut rowLabelScripts: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut colLabelScripts: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut doc: Document;
    let mut canvas: Arc<Tag>;
    matrixMargin = 100;
    blockSize = 20;
    scripts = metamodelica::nil();
    rowLabelScripts = metamodelica::nil();
    colLabelScripts = metamodelica::nil();
    scripts = cons(literal!("var ctx = document.querySelector('canvas').getContext('2d');\n"), scripts.clone());
    scripts = cons(literal!("ctx.fillStyle = '#001D4B';\n"), scripts.clone());
    scripts = cons(literal!("ctx.font=\"18px Arial\";\n\n"), scripts.clone());
    scripts = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var blockSize = ")); __mm_s.push_str(&*intString(blockSize.clone())); __mm_s.push_str(&*literal!(";\n")); ArcStr::from(__mm_s) }, scripts.clone());
    scripts = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("var matrixMargin = ")); __mm_s.push_str(&*intString(matrixMargin.clone())); __mm_s.push_str(&*literal!(";\n\n")); ArcStr::from(__mm_s) }, scripts.clone());
    scripts = cons(literal!("\nfunction drawRectangle(px, py, blockSize, margin, ctx) {\n   ctx.fillRect(((py-1)*blockSize) + matrixMargin,((px-1)*blockSize) + matrixMargin, blockSize, blockSize);\n   return ctx;\n     }\n\nfunction rowName(name, rowIdx, blockSize, margin, ctx) {\n   ctx.strokeText(name, 0, 18+margin+(rowIdx-1)*blockSize, margin);\n   return ctx;\n     }\n\nfunction colName(name, colIdx, blockSize, margin, ctx) {\n   ctx.strokeText(name, 0, 18+margin+(colIdx-1)*blockSize, margin);\n   return ctx;\n     }\n\nfunction makeLines(blockSize, margin,  n,  ctx) {\n     for (var x = 0; x < n+1; ++x) {\n     ctx.beginPath();\n     ctx.moveTo( x*blockSize + margin, margin);\n     ctx.lineTo( x*blockSize + margin, margin + (n)*blockSize);\n     ctx.stroke();\n     }\n\n\n    for (var x = 0; x < n+1; ++x) {\n     ctx.beginPath();\n     ctx.moveTo(margin, x*blockSize + margin);\n     ctx.lineTo(margin + (n)*blockSize, x*blockSize + margin);\n     ctx.stroke();\n    }\n\n  return ctx;\n  }\n  "), scripts.clone());
    size = (m.clone().borrow().len() as i32);
    for mut rowIdx in 1..=size.clone() {
        row = m.clone().borrow()[(rowIdx.clone()-1) as usize].clone();
        rowLabelDraw = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ctx = rowName(\"eq_")); __mm_s.push_str(&*(rowNames.clone()).get(rowIdx.clone())?); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*intString(rowIdx.clone())); __mm_s.push_str(&*literal!(", blockSize, matrixMargin, ctx);\n")); ArcStr::from(__mm_s) }).clone();
        rowLabelScripts = cons(rowLabelDraw.clone(), rowLabelScripts.clone());
        colLabelDraw = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ctx = colName(\"var_")); __mm_s.push_str(&*(columNames.clone()).get(rowIdx.clone())?); __mm_s.push_str(&*literal!("\", ")); __mm_s.push_str(&*intString(rowIdx.clone())); __mm_s.push_str(&*literal!(", blockSize, matrixMargin, ctx);\n")); ArcStr::from(__mm_s) }).clone();
        colLabelScripts = cons(colLabelDraw.clone(), colLabelScripts.clone());
        for mut colIdx in &*row.clone() {
            let mut colIdx = colIdx.clone();
            if colIdx.clone() > 0 {
                blockDraw = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ctx = drawRectangle(")); __mm_s.push_str(&*intString(rowIdx.clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*intString(colIdx.clone())); __mm_s.push_str(&*literal!(",blockSize, matrixMargin,  ctx);\n")); ArcStr::from(__mm_s) }).clone();
                scripts = cons(blockDraw.clone(), scripts.clone());
            }
        }
    }
    scripts = listAppend(rowLabelScripts.clone(), scripts.clone());
    scripts = cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  ctx.textAlign = 'right';\n\n  ctx = makeLines(blockSize, matrixMargin, ")); __mm_s.push_str(&*intString(size.clone())); __mm_s.push_str(&*literal!(", ctx);\n")); ArcStr::from(__mm_s) }, scripts.clone());
    scripts = cons(literal!("ctx.rotate(-Math.PI / 2);\n"), scripts.clone());
    scripts = listAppend(colLabelScripts.clone(), scripts.clone());
    doc = emptyDocumentWithToggleFunktion()?;
    canvas = Arc::new(Tag::CANVAS { attr: list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("width = \"")); __mm_s.push_str(&*intString(size.clone() * blockSize.clone() + matrixMargin.clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" height = \"")); __mm_s.push_str(&*intString(size.clone() * blockSize.clone() + matrixMargin.clone())); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) }).clone()] });
    doc = addScriptBody((literal!("LANGUAGE=\"JavaScript")).clone(), (List::fold(scripts.clone(), Arc::new(fnptr!(stringAppend, ArcStr, ArcStr)), (literal!("")).clone())).clone(), doc.clone())?;
    doc = addHeadTag(canvas.clone(), doc.clone())?;
    dumpDocument(doc.clone(), ({ let mut __mm_s = String::new(); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!(".html")); ArcStr::from(__mm_s) }).clone())?;
    Ok(())
}

fn intAbsGt(mut i1: i32, mut i2: i32) -> bool {
    let mut out: bool = false;
    out = intGt(intAbs(i1.clone()), intAbs(i2.clone()));
    out
}

