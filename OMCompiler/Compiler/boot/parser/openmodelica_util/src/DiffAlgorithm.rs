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

use crate::Print;
use crate::System;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Diff {
    Add = 1,
    Delete = 2,
    Equal = 3,
}
impl PartialOrd for Diff {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Diff {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn diff<T: Clone + 'static>(mut seq1: Arc<metamodelica::List<T>>, mut seq2: Arc<metamodelica::List<T>>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = metamodelica::nil();
    let mut start1: i32 = 0;
    let mut end1: i32 = 0;
    let mut start2: i32 = 0;
    let mut end2: i32 = 0;
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    let mut arr1: metamodelica::Array<T>;
    let mut arr2: metamodelica::Array<T>;
    arr1 = metamodelica::arrayFromVec(seq1.clone().into_iter().cloned().collect());
    arr2 = metamodelica::arrayFromVec(seq2.clone().into_iter().cloned().collect());
    start1 = 1;
    start2 = 1;
    end1 = (arr1.clone().borrow().len() as i32);
    end2 = (arr2.clone().borrow().len() as i32);
    out = diffSeq(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), isWhitespaceNotComment.clone(), toString.clone(), start1.clone(), end1.clone(), start2.clone(), end2.clone(), metamodelica::nil(), metamodelica::nil())?;
    Ok(out)
}

pub type partialPrintDiff<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, fn(T) -> Result<ArcStr>) -> Result<ArcStr> + 'static>;

pub fn printDiffTerminalColor<T: Clone + 'static>(mut seq: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    let mut open: ArcStr = arcstr::literal!("");
    let mut close: ArcStr = arcstr::literal!("");
    let mut ts: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut b: bool = false;
    let mut i: i32 = 0;
    // TODO: inherit algorithm from base function via `extends` clause.
    // Requires substituting `replaceable package` constant overrides into the inherited body.
    todo!("function `printDiffTerminalColor` inherits its algorithm via `extends` — not yet implemented")
}

pub fn printDiffXml<T: Clone + 'static>(mut seq: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    let mut open: ArcStr = arcstr::literal!("");
    let mut close: ArcStr = arcstr::literal!("");
    let mut ts: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut b: bool = false;
    let mut i: i32 = 0;
    // TODO: inherit algorithm from base function via `extends` clause.
    // Requires substituting `replaceable package` constant overrides into the inherited body.
    todo!("function `printDiffXml` inherits its algorithm via `extends` — not yet implemented")
}

pub fn printActual<T: Clone + 'static>(mut seq: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> ArcStr {
    let mut res: ArcStr = arcstr::literal!("");
    let mut open: ArcStr = arcstr::literal!("");
    let mut close: ArcStr = arcstr::literal!("");
    let mut ts: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut b: bool = false;
    let mut i: i32 = 0;
    // TODO: inherit algorithm from base function via `extends` clause.
    // Requires substituting `replaceable package` constant overrides into the inherited body.
    todo!("function `printActual` inherits its algorithm via `extends` — not yet implemented")
}

fn diffSeq<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut inStart1: i32, mut inEnd1: i32, mut inStart2: i32, mut inEnd2: i32, mut inPrefixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut inSuffixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = metamodelica::nil();
    let mut start1: i32 = inStart1.clone();
    let mut end1: i32 = inEnd1.clone();
    let mut start2: i32 = inStart2.clone();
    let mut end2: i32 = inEnd2.clone();
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    let mut prefixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inPrefixes.clone();
    let mut suffixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inSuffixes.clone();
    len1 = end1.clone() - start1.clone() + 1;
    len2 = end2.clone() - start2.clone() + 1;
    if len1.clone() < 1 && len2.clone() < 1 {
        out = List::append_reverse(prefixes.clone(), suffixes.clone());
        return Ok(out);
    } else if len1.clone() < 1 {
        out = List::append_reverse(prefixes.clone(), cons((Diff::Add.clone(), {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (start2.clone()..=end2.clone()).into_iter() {
            let __x = arr2.borrow()[(e.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), suffixes.clone()));
        return Ok(out);
    } else if len2.clone() < 1 {
        out = List::append_reverse(prefixes.clone(), cons((Diff::Delete.clone(), {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (start1.clone()..=end1.clone()).into_iter() {
            let __x = arr1.borrow()[(e.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), suffixes.clone()));
        return Ok(out);
    }
    if if (len1.clone() == len2.clone()) {{
        let mut __acc: Option<bool> = None;
        for e in (1..=len1.clone()).into_iter() {
            let __x = equals(arr1.borrow()[(e.clone() + start1.clone() - 1-1) as usize].clone(), arr2.borrow()[(e.clone() + start2.clone() - 1-1) as usize].clone())?;
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x < __cur { __x } else { __cur } });
        }
        __acc.ok_or_else(|| anyhow::anyhow!("empty min reduction"))?
    }} else {false} {
        out = list![(Diff::Equal.clone(), {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut e in (start1.clone()..=end1.clone()).into_iter() {
            let __x = arr1.borrow()[(e.clone()-1) as usize].clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })];
        return Ok(out);
    }
    (prefixes, start1, start2) = trimCommonPrefix(arr1.clone(), start1.clone(), end1.clone(), arr2.clone(), start2.clone(), end2.clone(), equals.clone(), prefixes.clone(), isWhitespaceNotComment.clone(), toString.clone());
    (suffixes, end1, end2) = trimCommonSuffix(arr1.clone(), start1.clone(), end1.clone(), arr2.clone(), start2.clone(), end2.clone(), equals.clone(), suffixes.clone(), isWhitespaceNotComment.clone());
    if start1.clone() != inStart1.clone() || start2.clone() != inStart2.clone() || end1.clone() != inEnd1.clone() || end2.clone() != inEnd2.clone() {
        out = diffSeq(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), isWhitespaceNotComment.clone(), toString.clone(), start1.clone(), end1.clone(), start2.clone(), end2.clone(), prefixes.clone(), suffixes.clone())?;
        return Ok(out);
    } else {
        out = 'mc: {
        let __mc_input = ();
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok(onlyAdditions(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), toString.clone(), start1.clone(), end1.clone(), start2.clone(), end2.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let () = __mc_input.clone() else { bail!("nomatch") };
            Ok(onlyRemovals(arr1.clone(), arr2.clone(), equals.clone(), isWhitespace.clone(), toString.clone(), start1.clone(), end1.clone(), start2.clone(), end2.clone())?)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(myersGreedyDiff(arr1.clone(), arr2.clone(), equals.clone(), start1.clone(), end1.clone(), start2.clone(), end2.clone())?)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
        out = List::append_reverse(prefixes.clone(), listAppend(out.clone(), suffixes.clone()));
        return Ok(out);
    }
    bail!("fail");
    Ok(out)
}

fn addToList<T: Clone + 'static>(mut inlst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut ind: Diff, mut inacc: Arc<metamodelica::List<T>>, mut newd: Diff, mut t: T) -> (Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, Diff, Arc<metamodelica::List<T>>) {
    let mut lst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inlst.clone();
    let mut d: Diff = newd.clone();
    let mut acc: Arc<metamodelica::List<T>> = inacc.clone();
    if ind.clone() == newd.clone() {
        acc = cons(t.clone(), acc.clone());
    } else {
        if !(inacc.clone().is_empty()) {
            lst = cons((ind.clone(), acc.clone().reverse()), lst.clone());
        }
        acc = list![t.clone()];
    }
    (lst, d, acc)
}

fn endList<T: Clone + 'static>(mut inlst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut ind: Diff, mut inacc: Arc<metamodelica::List<T>>) -> Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> {
    let mut lst: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = inlst.clone();
    if !(inacc.clone().is_empty()) {
        lst = cons((ind.clone(), inacc.clone().reverse()), lst.clone());
    }
    lst
}

fn onlyAdditions<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut start1: i32, mut end1: i32, mut start2: i32, mut end2: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = metamodelica::nil();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d: Diff = Diff::Equal.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    out = metamodelica::nil();
    while start1.clone() + x.clone() <= end1.clone() && start2.clone() + y.clone() <= end2.clone() {
        if equals(arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone(), arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Equal.clone(), arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone());
            x = x.clone() + 1;
            y = y.clone() + 1;
        } else if isWhitespace(arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Delete.clone(), arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone());
            x = x.clone() + 1;
        } else {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Add.clone(), arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone());
            y = y.clone() + 1;
        }
    }
    while start1.clone() + x.clone() <= end1.clone() {
        if isWhitespace(arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Delete.clone(), arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone());
            x = x.clone() + 1;
        } else {
            bail!("fail");
        }
    }
    while start2.clone() + y.clone() <= end2.clone() {
        if isWhitespace(arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Add.clone(), arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone());
            y = y.clone() + 1;
        } else {
            bail!("fail");
        }
    }
    out = endList(out.clone(), d.clone(), lst.clone());
    out = out.clone().reverse();
    Ok(out)
}

fn onlyRemovals<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut isWhitespace: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>, mut start1: i32, mut end1: i32, mut start2: i32, mut end2: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = metamodelica::nil();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut d: Diff = Diff::Equal.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    out = metamodelica::nil();
    while start1.clone() + x.clone() <= end1.clone() && start2.clone() + y.clone() <= end2.clone() {
        if equals(arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone(), arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Equal.clone(), arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone());
            x = x.clone() + 1;
            y = y.clone() + 1;
        } else if isWhitespace(arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Add.clone(), arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone());
            y = y.clone() + 1;
        } else {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Delete.clone(), arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone());
            x = x.clone() + 1;
        }
    }
    while start1.clone() + x.clone() <= end1.clone() {
        if isWhitespace(arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Delete.clone(), arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone());
            x = x.clone() + 1;
        } else {
            bail!("fail");
        }
    }
    while start2.clone() + y.clone() <= end2.clone() {
        if isWhitespace(arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone())? {
            (out, d, lst) = addToList(out.clone(), d.clone(), lst.clone(), Diff::Add.clone(), arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone());
            y = y.clone() + 1;
        } else {
            bail!("fail");
        }
    }
    out = endList(out.clone(), d.clone(), lst.clone());
    out = out.clone().reverse();
    Ok(out)
}

fn myersGreedyDiff<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut start1: i32, mut end1: i32, mut start2: i32, mut end2: i32) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = metamodelica::nil();
    let mut len1: i32 = 0;
    let mut len2: i32 = 0;
    let mut maxIter: i32 = 0;
    let mut sz: i32 = 0;
    let mut middle: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut V: metamodelica::Array<i32>;
    let mut paths: metamodelica::Array<Arc<metamodelica::List<(i32, i32)>>>;
    let mut prevPath: Arc<metamodelica::List<(i32, i32)>> = metamodelica::nil();
    len1 = end1.clone() - start1.clone() + 1;
    len2 = end2.clone() - start2.clone() + 1;
    maxIter = len1.clone() + len2.clone();
    sz = 2 * maxIter.clone() + 1;
    middle = maxIter.clone() + 1;
    V = arrayCreate(sz.clone(), 0);
    paths = arrayCreate(sz.clone(), metamodelica::nil());
    for mut D in 0..=maxIter.clone() {
        for mut k in (-(D.clone())..=D.clone()).step_by((2) as usize) {
            if k.clone() == -(D.clone()) || k.clone() != D.clone() && V.borrow()[(k.clone() - 1 + middle.clone()-1) as usize].clone() < V.borrow()[(k.clone() + 1 + middle.clone()-1) as usize].clone() {
                x = V.borrow()[(k.clone() + 1 + middle.clone()-1) as usize].clone();
                prevPath = paths.borrow()[(k.clone() + 1 + middle.clone()-1) as usize].clone();
            } else {
                x = V.borrow()[(k.clone() - 1 + middle.clone()-1) as usize].clone() + 1;
                prevPath = paths.borrow()[(k.clone() - 1 + middle.clone()-1) as usize].clone();
            }
            y = x.clone() - k.clone();
            {
                let __cell0 = cons((x.clone(), y.clone()), prevPath.clone());
                paths.clone().borrow_mut()[(k.clone() + middle.clone()-1) as usize] = __cell0;
            }
            while if (x.clone() < len1.clone() && y.clone() < len2.clone()) {equals(arr1.borrow()[(start1.clone() + x.clone()-1) as usize].clone(), arr2.borrow()[(start2.clone() + y.clone()-1) as usize].clone())?} else {false} {
                x = x.clone() + 1;
                y = y.clone() + 1;
                {
                    let __cell1 = cons((x.clone(), y.clone()), paths.borrow()[(k.clone() + middle.clone()-1) as usize].clone());
                    paths.clone().borrow_mut()[(k.clone() + middle.clone()-1) as usize] = __cell1;
                }
            }
            {
                let __cell2 = x.clone();
                V.clone().borrow_mut()[(k.clone() + middle.clone()-1) as usize] = __cell2;
            }
            if x.clone() >= len1.clone() && y.clone() >= len2.clone() {
                out = myersGreedyPathToDiff(arr1.clone(), arr2.clone(), start1.clone(), start2.clone(), paths.borrow()[(k.clone() + middle.clone()-1) as usize].clone())?;
                return Ok(out);
            }
        }
    }
    println!("{}", (literal!("myersDiff: This cannot happen")).clone());
    bail!("fail");
    Ok(out)
}

fn myersGreedyPathToDiff<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut arr2: metamodelica::Array<T>, mut start1: i32, mut start2: i32, mut paths: Arc<metamodelica::List<(i32, i32)>>) -> Result<Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>> {
    let mut out: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = metamodelica::nil();
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut y1: i32 = 0;
    let mut y2: i32 = 0;
    let mut d1: Diff = Diff::Equal.clone();
    let mut d2: Diff = Diff::Equal.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    let mut t: T;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(paths.clone()) {
        Deref @ metamodelica::List::Cons { head: (__pa0, __pa1), tail: _ } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    x2 = __pa0.clone();
    y2 = __pa1.clone();
    for mut path in &*listRest(paths.clone())? {
        let mut path = path.clone();
        (x1, y1) = path.clone();
        if x2.clone() - x1.clone() == 1 && y2.clone() - y1.clone() == 1 {
            d1 = Diff::Equal.clone();
            t = arr1.borrow()[(start1.clone() + x1.clone()-1) as usize].clone();
        } else if x2.clone() - x1.clone() == 1 && y2.clone() == y1.clone() {
            d1 = Diff::Delete.clone();
            t = arr1.borrow()[(start1.clone() + x1.clone()-1) as usize].clone();
        } else if y2.clone() - y1.clone() == 1 && x2.clone() == x1.clone() {
            d1 = Diff::Add.clone();
            t = arr2.borrow()[(start2.clone() + y1.clone()-1) as usize].clone();
        } else {
            println!("{}", (literal!("myersGreedyPathToDiff: This cannot happen\n")).clone());
            bail!("fail");
        }
        if lst.clone().is_empty() {
            lst = list![t.clone()];
        } else if d1.clone() == d2.clone() {
            lst = cons(t.clone(), lst.clone());
        } else {
            out = cons((d2.clone(), lst.clone()), out.clone());
            lst = list![t.clone()];
        }
        d2 = d1.clone();
        x2 = x1.clone();
        y2 = y1.clone();
    }
    if !(lst.clone().is_empty()) {
        out = cons((d2.clone(), lst.clone()), out.clone());
    }
    Ok(out)
}

fn trimCommonPrefix<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut inStart1: i32, mut end1: i32, mut arr2: metamodelica::Array<T>, mut inStart2: i32, mut end2: i32, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> (Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, i32, i32) {
    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut prefixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = acc.clone();
    let mut start1: i32 = inStart1.clone();
    let mut start2: i32 = inStart2.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    while start1.clone() <= end1.clone() && start2.clone() <= end2.clone() {
        if equals(arr1.borrow()[(start1.clone()-1) as usize].clone(), arr2.borrow()[(start2.clone()-1) as usize].clone()).unwrap() {
            lst = cons(arr1.borrow()[(start1.clone()-1) as usize].clone(), lst.clone());
            start1 = start1.clone() + 1;
            start2 = start2.clone() + 1;
        } else if start2.clone() + 1 <= end2.clone() && isWhitespaceNotComment(arr2.borrow()[(start2.clone()-1) as usize].clone()).unwrap() {
            if !(equals(arr1.borrow()[(start1.clone()-1) as usize].clone(), arr2.borrow()[(start2.clone() + 1-1) as usize].clone()).unwrap()) {
                break;
            }
            start2 = start2.clone() + 1;
        } else {
            break;
        }
    }
    if !(lst.clone().is_empty()) {
        prefixes = cons((Diff::Equal.clone(), lst.clone().reverse()), prefixes.clone());
    }
    (prefixes, start1, start2)
}

fn trimCommonSuffix<T: Clone + 'static>(mut arr1: metamodelica::Array<T>, mut start1: i32, mut inEnd1: i32, mut arr2: metamodelica::Array<T>, mut start2: i32, mut inEnd2: i32, mut equals: Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>, mut acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, mut isWhitespaceNotComment: Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>) -> (Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>>, i32, i32) {
    pub type FunEquals<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T, T) -> Result<bool> + 'static>;

    pub type FunWhitespace<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<bool> + 'static>;

    let mut suffixes: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<T>>)>> = acc.clone();
    let mut end1: i32 = inEnd1.clone();
    let mut end2: i32 = inEnd2.clone();
    let mut lst: Arc<metamodelica::List<T>> = metamodelica::nil();
    while start1.clone() <= end1.clone() && start2.clone() <= end2.clone() {
        if equals(arr1.borrow()[(end1.clone()-1) as usize].clone(), arr2.borrow()[(end2.clone()-1) as usize].clone()).unwrap() {
            lst = cons(arr1.borrow()[(end1.clone()-1) as usize].clone(), lst.clone());
            end1 = end1.clone() - 1;
            end2 = end2.clone() - 1;
        } else if start2.clone() <= end2.clone() - 1 && isWhitespaceNotComment(arr2.borrow()[(end2.clone()-1) as usize].clone()).unwrap() {
            if !(equals(arr1.borrow()[(end1.clone()-1) as usize].clone(), arr2.borrow()[(end2.clone() - 1-1) as usize].clone()).unwrap()) {
                break;
            }
            end2 = end2.clone() - 1;
        } else {
            break;
        }
    }
    if !(lst.clone().is_empty()) {
        suffixes = cons((Diff::Equal.clone(), lst.clone()), suffixes.clone());
    }
    (suffixes, end1, end2)
}

fn printStartToEnd<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut startIndex: i32, mut endIndex: i32, mut toString: Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>) -> Result<ArcStr> {
    pub type ToString<T: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(T) -> Result<ArcStr> + 'static>;

    let mut res: ArcStr = arcstr::literal!("");
    res = stringAppendList({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut index in (startIndex.clone()..=endIndex.clone()).into_iter() {
            let __x = toString(arr.clone().borrow()[(index.clone()-1) as usize].clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

