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

pub type Type = ArcStr;

pub type Ident = ArcStr;

pub type Label = ArcStr;

/// an Attribute is a pair of name an value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attribute {
    /// name
    pub name: ArcStr,
    /// value
    pub value: ArcStr,
}

pub type ATTR = Attribute;


pub type Attributes = Arc<metamodelica::List<Attribute>>;

/// A graphviz Node is a node of the graph.
///    It has a type and attributes and children.
///    It can also have a list of labels, provided by the LNODE
///    constructor.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Node {
    NODE {
        type_: Type,
        attributes: Attributes,
        children: Arc<metamodelica::List<Arc<Node>>>,
    },
    LNODE {
        type_: Type,
        labelLst: Arc<metamodelica::List<ArcStr>>,
        attributes: Attributes,
        children: Arc<metamodelica::List<Arc<Node>>>,
    },
}
pub use self::Node::{NODE,LNODE};

pub type Children = Arc<metamodelica::List<Arc<Node>>>;

pub static r#box: Attribute = Attribute { name: literal!("shape"), value: literal!("box") };

pub fn dump(mut node: Arc<Node>) -> Result<()> {
    let mut nm: Label = arcstr::literal!("");
    println!("{}", (literal!("graph AST {\n")).clone());
    nm = (dumpNode(node.clone())?).clone();
    println!("{}", (literal!("}\n")).clone());
    Ok(())
}

fn dumpNode(mut inNode: Arc<Node>) -> Result<Ident> {
    let mut outIdent: Ident = arcstr::literal!("");
    outIdent = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Node::NODE { children, attributes: attr, type_: typ } => {
            let mut nm: Label = arcstr::literal!("");
            let mut typlbl: Label = arcstr::literal!("");
            let mut out: Label = arcstr::literal!("");
            let mut newattr: Attributes = metamodelica::nil();
            nm = (nodename((typ.clone()).clone())).clone();
            typlbl = (makeLabel(list![(typ.clone()).clone()])?).clone();
            newattr = cons(Attribute { name: (literal!("label")).clone(), value: (typlbl.clone()).clone() }, attr.clone());
            out = (makeNode((nm.clone()).clone(), newattr.clone())?).clone();
            println!("{}", (out.clone()).clone());
            dumpChildren((nm.clone()).clone(), children.clone())?;
            nm.clone()
        },
        Deref @ Node::LNODE { children, attributes: attr, labelLst: lbl, type_: typ } => {
            let mut nm: Label = arcstr::literal!("");
            let mut out: Label = arcstr::literal!("");
            let mut lblstr: Label = arcstr::literal!("");
            let mut newattr: Attributes = metamodelica::nil();
            let mut lbl_1: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
            nm = (nodename((typ.clone()).clone())).clone();
            lbl_1 = cons((typ.clone()).clone(), lbl.clone());
            lblstr = (makeLabel(lbl_1.clone())?).clone();
            newattr = cons(Attribute { name: (literal!("label")).clone(), value: (lblstr.clone()).clone() }, attr.clone());
            out = (makeNode((nm.clone()).clone(), newattr.clone())?).clone();
            println!("{}", (out.clone()).clone());
            dumpChildren((nm.clone()).clone(), children.clone())?;
            nm.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outIdent)
}

fn makeLabel(mut sl: Arc<metamodelica::List<ArcStr>>) -> Result<ArcStr> {
    let mut s2: ArcStr = arcstr::literal!("");
    let mut s0: Label = arcstr::literal!("");
    let mut s1: Label = arcstr::literal!("");
    s0 = (makeLabelReq(sl.clone(), (literal!("")).clone())?).clone();
    s1 = (stringAppend((literal!("\"")).clone(), (s0.clone()).clone())).clone();
    s2 = (stringAppend((s1.clone()).clone(), (literal!("\"")).clone())).clone();
    Ok(s2)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeLabelReq(mut inStringLst: Arc<metamodelica::List<ArcStr>>, mut inString: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inStringLst.clone()) {
        Deref @ metamodelica::List::Cons { head: s, tail: Deref @ metamodelica::List::Nil } => {
            stringAppend((inString.clone()).clone(), (s.clone()).clone())
        },
        Deref @ metamodelica::List::Cons { head: s1, tail: Deref @ metamodelica::List::Cons { head: s2, tail: Deref @ metamodelica::List::Nil } } => {
            let mut s: Label = arcstr::literal!("");
            s = (stringAppend((inString.clone()).clone(), (s1.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("\\n")).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (s2.clone()).clone())).clone();
            s.clone()
        },
        Deref @ metamodelica::List::Cons { head: s1, tail: rest } => {
            let mut s: Label = arcstr::literal!("");
            s = (stringAppend((inString.clone()).clone(), (s1.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("\\n")).clone())).clone();
            makeLabelReq(rest.clone(), (s.clone()).clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn dumpChildren(mut inIdent: Ident, mut inChildren: Children) -> Result<()> {
    let _ = (::match_deref::match_deref! { match &((inIdent.clone(), inChildren.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            ()
        },
        (parent, Deref @ metamodelica::List::Cons { head: node, tail: rest }) => {
            let mut nm: Label = arcstr::literal!("");
            nm = (dumpNode(node.clone())?).clone();
            printEdge((nm.clone()).clone(), (parent.clone()).clone());
            dumpChildren((parent.clone()).clone(), rest.clone())?;
            ()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(())
}

fn nodename(mut r#str: ArcStr) -> ArcStr {
    let mut s: ArcStr = arcstr::literal!("");
    let mut i: i32 = 0;
    let mut is: Label = arcstr::literal!("");
    i = tick();
    is = (intString(i.clone())).clone();
    s = (stringAppend((literal!("GVNOD")).clone(), (is.clone()).clone())).clone();
    s
}

fn printEdge(mut n1: Ident, mut n2: Ident) -> () {
    let mut r#str: Label = arcstr::literal!("");
    r#str = (makeEdge((n1.clone()).clone(), (n2.clone()).clone())).clone();
    println!("{}", (r#str.clone()).clone());
    println!("{}", (literal!(";\n")).clone());
    ()
}

fn makeEdge(mut n1: Ident, mut n2: Ident) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: Label = arcstr::literal!("");
    s = (stringAppend((n1.clone()).clone(), (literal!(" -- ")).clone())).clone();
    r#str = (stringAppend((s.clone()).clone(), (n2.clone()).clone())).clone();
    r#str
}

fn makeNode(mut nm: Ident, mut attr: Attributes) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut s: Label = arcstr::literal!("");
    let mut s_1: Label = arcstr::literal!("");
    s = (makeAttr(attr.clone())?).clone();
    s_1 = (stringAppend((nm.clone()).clone(), (s.clone()).clone())).clone();
    r#str = (stringAppend((s_1.clone()).clone(), (literal!(";")).clone())).clone();
    Ok(r#str)
}

fn makeAttr(mut l: Arc<metamodelica::List<Attribute>>) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut res: Label = arcstr::literal!("");
    let mut s: Label = arcstr::literal!("");
    res = (makeAttrReq(l.clone(), (literal!("")).clone())?).clone();
    s = (stringAppend((literal!("[")).clone(), (res.clone()).clone())).clone();
    r#str = (stringAppend((s.clone()).clone(), (literal!("]")).clone())).clone();
    Ok(r#str)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn makeAttrReq(mut inAttributeLst: Arc<metamodelica::List<Attribute>>, mut inString: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inAttributeLst.clone()) {
        Deref @ metamodelica::List::Cons { head: Attribute { value: v, name }, tail: Deref @ metamodelica::List::Nil } => {
            let mut s: Label = arcstr::literal!("");
            s = (stringAppend((inString.clone()).clone(), (name.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("=")).clone())).clone();
            stringAppend((s.clone()).clone(), (v.clone()).clone())
        },
        Deref @ metamodelica::List::Cons { head: Attribute { value: v, name }, tail: rest } => {
            let mut s: Label = arcstr::literal!("");
            s = (stringAppend((inString.clone()).clone(), (name.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!("=")).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (v.clone()).clone())).clone();
            s = (stringAppend((s.clone()).clone(), (literal!(",")).clone())).clone();
            makeAttrReq(rest.clone(), (s.clone()).clone())?
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

