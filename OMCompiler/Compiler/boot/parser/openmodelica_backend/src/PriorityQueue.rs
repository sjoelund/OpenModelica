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

use crate::SimCode;

/* protected */
/* TODO: Hide when RML is killed */
/* This specific version... */
pub type Priority = i32;

pub type Data = Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>;

/* Replaceable types */
pub fn compareElement(mut el1: Element, mut el2: Element) -> bool {
    let mut b: bool = false;
    let mut p1: Priority = 0;
    let mut p2: Priority = 0;
    (p1, _) = el1.clone();
    (p2, _) = el2.clone();
    b = p1.clone() <= p2.clone();
    b
}

pub type Element = (i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>);

pub type T = Arc<metamodelica::List<Arc<Tree>>>;

thread_local! { static __empty_TLS: Arc<metamodelica::List<Arc<Tree>>> = metamodelica::nil(); }
pub fn empty() -> Arc<metamodelica::List<Arc<Tree>>> { __empty_TLS.with(|__t| __t.clone()) }

/*
function isEmpty = listEmpty;
*/
pub fn isEmpty(mut ts: T) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = ts.clone().is_empty();
    isEmpty
}

pub fn insert(mut elt: Element, mut ts: T) -> Result<T> {
    let mut ots: T = metamodelica::nil();
    ots = ins(Arc::new(Tree { elt: elt.clone(), rank: 0, trees: metamodelica::nil() }), ts.clone())?;
    Ok(ots)
}

pub fn meld(mut its1: T, mut its2: T) -> Result<T> {
    let mut ts: T = metamodelica::nil();
    ts = (::match_deref::match_deref! { match &((its1.clone(), its2.clone())) {
        (ts1, Deref @ metamodelica::List::Nil) => {
            ts1.clone()
        },
        (Deref @ metamodelica::List::Nil, ts2) => {
            ts2.clone()
        },
        (Deref @ metamodelica::List::Cons { head: t1, tail: ts1 }, Deref @ metamodelica::List::Cons { head: t2, tail: ts2 }) => {
            meld2(rank(t1.clone())? < rank(t2.clone())?, rank(t2.clone())? < rank(t1.clone())?, t1.clone(), ts1.clone(), t2.clone(), ts2.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ts)
}

pub fn meld2(mut b1: bool, mut b2: bool, mut t1: Arc<Tree>, mut inTs1: T, mut t2: Arc<Tree>, mut inTs2: T) -> Result<T> {
    let mut ts: T = metamodelica::nil();
    ts = (::match_deref::match_deref! { match &((b1.clone(), b2.clone(), inTs1.clone(), inTs2.clone())) {
        (true, _, ts1, ts2) => {
            ts = meld(ts1.clone(), cons(t2.clone(), ts2.clone()))?;
            cons(t1.clone(), ts.clone())
        },
        (_, true, ts1, ts2) => {
            ts = meld(cons(t1.clone(), ts1.clone()), ts2.clone())?;
            cons(t2.clone(), ts.clone())
        },
        _ => {
            ins(link(t1.clone(), t2.clone())?, meld(inTs1.clone(), inTs2.clone())?)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ts)
}

pub fn findMin(mut inTs: T) -> Result<Element> {
    let mut elt: Element;
    elt = (::match_deref::match_deref! { match &(inTs.clone()) {
        Deref @ metamodelica::List::Cons { head: t, tail: Deref @ metamodelica::List::Nil } => {
            root(t.clone())?
        },
        Deref @ metamodelica::List::Cons { head: t, tail: ts } => {
            let mut x: Element;
            let mut y: Element;
            x = root(t.clone())?;
            y = findMin(ts.clone())?;
            if (compareElement(x.clone(), y.clone())) {x.clone()} else {y.clone()}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(elt)
}

pub fn deleteMin(mut ts: T) -> Result<T> {
    let mut ots: T = metamodelica::nil();
    let mut ts1: T = metamodelica::nil();
    let mut ts2: T = metamodelica::nil();
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getMin(ts.clone())?) {
        (Deref @ Tree { trees: __pa0, .. }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ts1 = __pa0.clone();
    ts2 = __pa1.clone();
    ots = meld(ts1.clone().reverse(), ts2.clone())?;
    Ok(ots)
}

pub fn deleteAndReturnMin(mut ts: T) -> Result<(T, Element)> {
    let mut ots: T = metamodelica::nil();
    let mut elt: Element;
    let mut ts1: T = metamodelica::nil();
    let mut ts2: T = metamodelica::nil();
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(getMin(ts.clone())?) {
        (Deref @ Tree { trees: __pa0, elt: __pa1, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    ts1 = __pa0.clone();
    elt = __pa1.clone();
    ts2 = __pa2.clone();
    ots = meld(ts1.clone().reverse(), ts2.clone())?;
    Ok((ots, elt))
}

pub fn elements(mut ts: T) -> Result<Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)>>> {
    let mut elts: Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)>> = metamodelica::nil();
    elts = elements2(ts.clone(), metamodelica::nil())?;
    Ok(elts)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn elements2(mut its: T, mut acc: Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)>>) -> Result<Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)>>> {
    let mut elts: Arc<metamodelica::List<(i32, Arc<metamodelica::List<Arc<SimCode::SimEqSystem>>>)>> = metamodelica::nil();
    elts = (::match_deref::match_deref! { match &(its.clone()) {
        Deref @ metamodelica::List::Nil => {
            acc.clone().reverse()
        },
        ts => {
            let mut elt: Element;
            let mut ts = (*ts).clone();
            (ts, elt) = deleteAndReturnMin(ts.clone())?;
            elements2(ts.clone(), cons(elt.clone(), acc.clone()))?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elts)
}

/* TODO: Hide from user when we remove RML... */
pub type Rank = i32;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree {
    pub elt: Element,
    pub rank: Rank,
    pub trees: T,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            elt: Default::default(),
            rank: Default::default(),
            trees: Default::default(),
        }
    }
}

pub type NODE = Tree;


fn root(mut tree: Arc<Tree>) -> Result<Element> {
    let mut elt: Element;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree { elt: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    elt = __pa0.clone();
    Ok(elt)
}

fn rank(mut tree: Arc<Tree>) -> Result<Rank> {
    let mut rank: Rank = 0;
    let __pa0 = ::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree { rank: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    rank = __pa0.clone();
    Ok(rank)
}

fn link(mut t1: Arc<Tree>, mut t2: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut t: Arc<Tree> = Arc::new(<Tree as ::std::default::Default>::default());
    t = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ Tree { elt: e1, rank: r1, trees: ts1 }, Deref @ Tree { elt: e2, rank: r2, trees: ts2 }) => {
            let mut r1 = (*r1).clone();
            let mut ts1 = (*ts1).clone();
            let mut r2 = (*r2).clone();
            let mut ts2 = (*ts2).clone();
            r1 = r1.clone() + 1;
            r2 = r2.clone() + 1;
            ts1 = cons(t2.clone(), ts1.clone());
            ts2 = cons(t1.clone(), ts2.clone());
            if (compareElement(root(t1.clone())?, root(t2.clone())?)) {Arc::new(Tree { elt: e1.clone(), rank: r1.clone(), trees: ts1.clone() })} else {Arc::new(Tree { elt: e2.clone(), rank: r2.clone(), trees: ts2.clone() })}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(t)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn ins(mut t: Arc<Tree>, mut its: T) -> Result<T> {
    let mut ots: T = metamodelica::nil();
    ots = (::match_deref::match_deref! { match &((t.clone(), its.clone())) {
        (_, Deref @ metamodelica::List::Nil) => {
            list![t.clone()]
        },
        (t1, Deref @ metamodelica::List::Cons { head: t2, tail: ts }) => {
            if (rank(t1.clone())? < rank(t2.clone())?) {cons(t1.clone(), cons(t2.clone(), ts.clone()))} else {ins(link(t1.clone(), t2.clone())?, ts.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(ots)
}

fn getMin(mut ts: T) -> Result<(Arc<Tree>, T)> {
    let mut min: Arc<Tree> = Arc::new(<Tree as ::std::default::Default>::default());
    let mut ots: T = metamodelica::nil();
    (min, ots) = (::match_deref::match_deref! { match &(ts.clone()) {
        Deref @ metamodelica::List::Cons { head: t, tail: Deref @ metamodelica::List::Nil } => {
            (t.clone(), metamodelica::nil())
        },
        Deref @ metamodelica::List::Cons { head: t1, tail: ts1 } => {
            let mut t2: Arc<Tree> = Arc::new(<Tree as ::std::default::Default>::default());
            let mut ts2: T = metamodelica::nil();
            let mut b: bool = false;
            (t2, ts2) = getMin(ts1.clone())?;
            b = compareElement(root(t1.clone())?, root(t2.clone())?);
            (if (b.clone()) {t1.clone()} else {t2.clone()}, if (b.clone()) {ts1.clone()} else {cons(t1.clone(), ts2.clone())})
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((min, ots))
}

