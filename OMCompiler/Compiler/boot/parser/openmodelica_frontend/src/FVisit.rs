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

use crate::FCore;
use crate::FNode;
use openmodelica_util::Error;
use openmodelica_util_datatypes_basic::List;

// public imports
// protected imports
pub type Id = i32;

pub type Seq = i32;

pub type Next = i32;

pub type Node = FCore::Node;

pub type Ref = metamodelica::Array<FCore::Node>;

pub type Data = FCore::Data;

pub type Visit = FCore::Visit;

pub type VAvlTree = Arc<FCore::VAvlTree>;

pub type Visited = FCore::Visited;

pub type AvlTree = Arc<FCore::VAvlTree>;

pub type AvlKey = i32;

pub type AvlValue = FCore::Visit;

pub type AvlTreeValue = FCore::VAvlTreeValue;

// TODO: non-Sync, non-const-emittable constant — needs new emission path.
// Type: FCore::Visited
// Expr: Constructor { name: 'FCore.Visited.V', args: [Var { name: 'FCore.emptyVAvlTree', segments: [CrefSegment { name: 'FCore', subscripts: [] }, CrefSegment { name: 'emptyVAvlTree', subscripts: [] }], ty: AliasTo('FCore.VAvlTree') }, Var { name: 'FCore.firstId', segments: [CrefSegment { name: 'FCore', subscripts: [] }, CrefSegment { name: 'firstId', subscripts: [] }], ty: I32 }], named_args: [], ty: RustStruct('FCore.Visited'), field_names: ['tree', 'next'] }
pub fn emptyVisited() -> FCore::Visited { todo!("non-Sync, non-const-emittable constant emptyVisited — extend codegen") }

pub fn new() -> Visited {
    let mut visited: Visited;
    visited = emptyVisited().clone();
    visited
}

pub fn reset(mut inVisited: Visited) -> Visited {
    let mut visited: Visited;
    visited = new();
    visited
}

pub fn next(mut inVisited: Visited) -> Result<(Visited, Next)> {
    let mut outVisited: Visited;
    let mut next: Next = 0;
    let mut v: VAvlTree;
    let mut n: Next = 0;
    let FCore::V { tree: __pa0, next: __pa1 } = (inVisited.clone()) else { bail!("pattern mismatch") };
    v = __pa0.clone();
    n = __pa1.clone();
    next = n.clone();
    n = FCore::next(n.clone());
    outVisited = FCore::Visited { tree: v.clone(), next: n.clone() };
    Ok((outVisited, next))
}

pub fn visited(mut inVisited: Visited, mut inRef: Ref) -> Result<bool> {
    let mut b: bool = false;
    b = 'mc: {
        let __mc_input = (inVisited.clone(), inRef.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (FCore::Visited { tree: ref a, .. }, _) = __mc_input.clone() else { bail!("nomatch") };
            let _ = FNode::id(FNode::fromRef(inRef.clone())?)?;
            let _ = avlTreeGet(a.clone(), FNode::id(FNode::fromRef(inRef.clone())?)?)?;
            Ok(true)
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            Ok(false)
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(b)
}

pub fn seq(mut v: Visit) -> Result<Seq> {
    let mut s: Seq = 0;
    let FCore::VN { seq: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    s = __pa0.clone();
    Ok(s)
}

pub fn r#ref(mut v: Visit) -> Result<Ref> {
    let mut r: Ref;
    let FCore::VN { r#ref: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    r = __pa0.clone();
    Ok(r)
}

pub fn tree(mut v: Visited) -> Result<AvlTree> {
    let mut a: AvlTree;
    let FCore::V { tree: __pa0, .. } = (v.clone()) else { bail!("pattern mismatch") };
    a = __pa0.clone();
    Ok(a)
}

pub fn visit(mut inVisited: Visited, mut inRef: Ref) -> Result<Visited> {
    let mut outVisited: Visited;
    outVisited = 'mc: {
        let __mc_input = (inVisited.clone(), inRef.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (_, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut v: Visit;
            let _ = FNode::id(FNode::fromRef(inRef.clone())?)?;
            v = avlTreeGet(tree(inVisited.clone())?, FNode::id(FNode::fromRef(inRef.clone())?)?)?;
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Already visited: ")); __mm_s.push_str(&*FNode::toStr(FNode::fromRef(inRef.clone())?)?); __mm_s.push_str(&*literal!(" seq: ")); __mm_s.push_str(&*intString(seq(v.clone())?)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let (FCore::Visited { tree: ref a, next: _ }, _) = __mc_input.clone() else { bail!("nomatch") };
            let mut s: Seq = 0;
            let mut n: Next = 0;
            let mut id: Id = 0;
            let mut a = a.clone();
            let mut outVisited: FCore::Visited;
            id = FNode::id(FNode::fromRef(inRef.clone())?)?;
            if '__try0: {
                let _ = unwrap_break_err!(avlTreeGet(tree(inVisited.clone())?, id.clone()), '__try0);
                Ok::<(), anyhow::Error>(())
            }.is_ok() { bail!("failure(): body succeeded") }
            let (FCore::V { next: __pa1, .. }, __pa2) = (next(inVisited.clone())?) else { bail!("pattern mismatch") };
            n = __pa1.clone();
            s = __pa2.clone();
            a = avlTreeAdd(a.clone(), id.clone(), FCore::Visit { r#ref: inRef.clone(), seq: s.clone() })?;
            outVisited = FCore::Visited { tree: a.clone(), next: n.clone() };
            Ok(outVisited.clone())
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outVisited)
}

// ************************ AVL Tree implementation ***************************
// ************************ AVL Tree implementation ***************************
// ************************ AVL Tree implementation ***************************
// ************************ AVL Tree implementation ***************************
pub fn keyCompare(mut k1: AvlKey, mut k2: AvlKey) -> i32 {
    let mut i: i32 = 0;
    i = if (intGt(k1.clone(), k2.clone())) {1} else {if (intLt(k1.clone(), k2.clone())) {-1} else {0}};
    i
}

pub fn keyStr(mut k: AvlKey) -> ArcStr {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (intString(k.clone())).clone();
    r#str
}

pub fn valueStr(mut v: AvlValue) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = ((match v.clone() {
        FCore::Visit { seq: mut seq, .. } => {
            intString(seq.clone())
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(r#str)
}

/* Generic Code below */
pub fn avlTreeNew() -> AvlTree {
    let mut tree: AvlTree;
    tree = FCore::emptyVAvlTree().clone();
    tree
}

pub fn avlTreeAdd(mut inAvlTree: AvlTree, mut inKey: AvlKey, mut inValue: AvlValue) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (::match_deref::match_deref! { match &((inAvlTree.clone(), inKey.clone(), inValue.clone())) {
        (Deref @ FCore::VAvlTree { right: None, left: None, value: None, .. }, key, value) => {
            Arc::new(FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: key.clone(), value: value.clone() }), height: 1, left: None, right: None })
        },
        (Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey, .. }), .. }, key, value) => {
            balance(avlTreeAdd2(inAvlTree.clone(), keyCompare(key.clone(), rkey.clone()), key.clone(), value.clone())?)?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("Env.avlTreeAdd failed")).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAvlTree)
}

pub fn avlTreeAdd2(mut inAvlTree: AvlTree, mut keyComp: i32, mut inKey: AvlKey, mut inValue: AvlValue) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (::match_deref::match_deref! { match &((inAvlTree.clone(), keyComp.clone(), inKey.clone(), inValue.clone())) {
        (Deref @ FCore::VAvlTree { right, left, height: h, value: Some(FCore::VAvlTreeValue { key: rkey, .. }) }, 0, _, value) => {
            Arc::new(FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey.clone(), value: value.clone() }), height: h.clone(), left: left.clone(), right: right.clone() })
        },
        (Deref @ FCore::VAvlTree { right, left, height: h, value: oval }, 1, key, value) => {
            let mut t_1: AvlTree;
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(right.clone());
            t_1 = avlTreeAdd(t.clone(), key.clone(), value.clone())?;
            Arc::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: left.clone(), right: Some(t_1.clone()) })
        },
        (Deref @ FCore::VAvlTree { right, left, height: h, value: oval }, (-1), key, value) => {
            let mut t_1: AvlTree;
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(left.clone());
            t_1 = avlTreeAdd(t.clone(), key.clone(), value.clone())?;
            Arc::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: Some(t_1.clone()), right: right.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAvlTree)
}

fn createEmptyAvlIfNone(mut t: Option<Arc<FCore::VAvlTree>>) -> AvlTree {
    let mut outT: AvlTree;
    outT = (::match_deref::match_deref! { match &(t.clone()) {
        None => Arc::new(FCore::VAvlTree { value: None, height: 0, left: None, right: None }),
        Some(outT) => outT.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outT
}

fn nodeValue(mut bt: AvlTree) -> Result<AvlValue> {
    let mut v: AvlValue;
    v = (::match_deref::match_deref! { match &(bt.clone()) {
        Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: _, value: v }), .. } => v.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(v)
}

fn balance(mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (::match_deref::match_deref! { match &(inBt.clone()) {
        bt => {
            let mut d: i32 = 0;
            let mut bt = (*bt).clone();
            d = differenceInHeight(bt.clone())?;
            bt = doBalance(d.clone(), bt.clone())?;
            bt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBt)
}

fn doBalance(mut difference: i32, mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (::match_deref::match_deref! { match &((difference.clone(), inBt.clone())) {
        ((-1), bt) => {
            computeHeight(bt.clone())?
        },
        (0, bt) => {
            computeHeight(bt.clone())?
        },
        (1, bt) => {
            computeHeight(bt.clone())?
        },
        (_, bt) => {
            let mut bt = (*bt).clone();
            bt = doBalance2(difference.clone() < 0, bt.clone())?;
            bt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outBt)
}

fn doBalance2(mut differenceIsNegative: bool, mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (::match_deref::match_deref! { match &((differenceIsNegative.clone(), inBt.clone())) {
        (true, bt) => {
            let mut bt = (*bt).clone();
            bt = doBalance3(bt.clone())?;
            bt = rotateLeft(bt.clone())?;
            bt.clone()
        },
        (false, bt) => {
            let mut bt = (*bt).clone();
            bt = doBalance4(bt.clone())?;
            bt = rotateRight(bt.clone())?;
            bt.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBt)
}

fn doBalance3(mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = 'mc: {
        let __mc_input = inBt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                bt => {
                    let mut rr: AvlTree;
                    let mut bt = (*bt).clone();
                    let true = (differenceInHeight(getOption(rightNode(bt.clone())?)?)? > 0) else { bail!("pattern mismatch") };
                    rr = rotateRight(getOption(rightNode(bt.clone())?)?)?;
                    bt = setRight(bt.clone(), Some(rr.clone()))?;
                    Ok(bt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inBt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBt)
}

fn doBalance4(mut inBt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = 'mc: {
        let __mc_input = inBt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                bt => {
                    let mut rl: AvlTree;
                    let mut bt = (*bt).clone();
                    let true = (differenceInHeight(getOption(leftNode(bt.clone())?)?)? < 0) else { bail!("pattern mismatch") };
                    rl = rotateLeft(getOption(leftNode(bt.clone())?)?)?;
                    bt = setLeft(bt.clone(), Some(rl.clone()))?;
                    Ok(bt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inBt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBt)
}

fn setRight(mut node: AvlTree, mut right: Option<Arc<FCore::VAvlTree>>) -> Result<AvlTree> {
    let mut outNode: AvlTree;
    outNode = (::match_deref::match_deref! { match &((node.clone(), right.clone())) {
        (Deref @ FCore::VAvlTree { value, height, left: l, right: _ }, _) => {
            Arc::new(FCore::VAvlTree { value: value.clone(), height: height.clone(), left: l.clone(), right: right.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn setLeft(mut node: AvlTree, mut left: Option<Arc<FCore::VAvlTree>>) -> Result<AvlTree> {
    let mut outNode: AvlTree;
    outNode = (::match_deref::match_deref! { match &((node.clone(), left.clone())) {
        (Deref @ FCore::VAvlTree { value, height, left: _, right: r }, _) => {
            Arc::new(FCore::VAvlTree { value: value.clone(), height: height.clone(), left: left.clone(), right: r.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn leftNode(mut node: AvlTree) -> Result<Option<Arc<FCore::VAvlTree>>> {
    let mut subNode: Option<Arc<FCore::VAvlTree>> = None;
    subNode = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ FCore::VAvlTree { left: subNode, .. } => subNode.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(subNode)
}

fn rightNode(mut node: AvlTree) -> Result<Option<Arc<FCore::VAvlTree>>> {
    let mut subNode: Option<Arc<FCore::VAvlTree>> = None;
    subNode = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ FCore::VAvlTree { right: subNode, .. } => subNode.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(subNode)
}

fn exchangeLeft(mut inNode: AvlTree, mut inParent: AvlTree) -> Result<AvlTree> {
    let mut outParent: AvlTree;
    outParent = (::match_deref::match_deref! { match &((inNode.clone(), inParent.clone())) {
        (node, parent) => {
            let mut bt: AvlTree;
            let mut node = (*node).clone();
            let mut parent = (*parent).clone();
            parent = setRight(parent.clone(), leftNode(node.clone())?)?;
            parent = balance(parent.clone())?;
            node = setLeft(node.clone(), Some(parent.clone()))?;
            bt = balance(node.clone())?;
            bt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outParent)
}

fn exchangeRight(mut inNode: AvlTree, mut inParent: AvlTree) -> Result<AvlTree> {
    let mut outParent: AvlTree;
    outParent = (::match_deref::match_deref! { match &((inNode.clone(), inParent.clone())) {
        (node, parent) => {
            let mut bt: AvlTree;
            let mut node = (*node).clone();
            let mut parent = (*parent).clone();
            parent = setLeft(parent.clone(), rightNode(node.clone())?)?;
            parent = balance(parent.clone())?;
            node = setRight(node.clone(), Some(parent.clone()))?;
            bt = balance(node.clone())?;
            bt.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outParent)
}

fn rotateLeft(mut node: AvlTree) -> Result<AvlTree> {
    let mut outNode: AvlTree;
    outNode = exchangeLeft(getOption(rightNode(node.clone())?)?, node.clone())?;
    Ok(outNode)
}

fn getOption<T: Clone + 'static>(mut opt: Option<T>) -> Result<T> {
    let mut val: T;
    val = (match opt.clone() {
        Some(mut val) => val.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(val)
}

fn rotateRight(mut node: AvlTree) -> Result<AvlTree> {
    let mut outNode: AvlTree;
    outNode = exchangeRight(getOption(leftNode(node.clone())?)?, node.clone())?;
    Ok(outNode)
}

fn differenceInHeight(mut node: AvlTree) -> Result<i32> {
    let mut diff: i32 = 0;
    diff = (::match_deref::match_deref! { match &(node.clone()) {
        Deref @ FCore::VAvlTree { right: r, left: l, .. } => {
            let mut lh: i32 = 0;
            let mut rh: i32 = 0;
            lh = getHeight(l.clone())?;
            rh = getHeight(r.clone())?;
            lh.clone() - rh.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(diff)
}

pub fn avlTreeGet(mut inAvlTree: AvlTree, mut inKey: AvlKey) -> Result<AvlValue> {
    let mut outValue: AvlValue;
    outValue = (::match_deref::match_deref! { match &((inAvlTree.clone(), inKey.clone())) {
        (Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey, .. }), .. }, key) => {
            avlTreeGet2(inAvlTree.clone(), keyCompare(key.clone(), rkey.clone()), key.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

fn avlTreeGet2(mut inAvlTree: AvlTree, mut keyComp: i32, mut inKey: AvlKey) -> Result<AvlValue> {
    let mut outValue: AvlValue;
    outValue = (::match_deref::match_deref! { match &((inAvlTree.clone(), keyComp.clone(), inKey.clone())) {
        (Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { value: rval, .. }), .. }, 0, _) => {
            rval.clone()
        },
        (Deref @ FCore::VAvlTree { right: Some(right), .. }, 1, key) => {
            avlTreeGet(right.clone(), key.clone())?
        },
        (Deref @ FCore::VAvlTree { left: Some(left), .. }, (-1), key) => {
            avlTreeGet(left.clone(), key.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

fn getOptionStr<Type_a: Clone + 'static>(mut inTypeAOption: Option<Type_a>, mut inFuncTypeTypeAToString: Arc<dyn ::std::ops::Fn(Type_a) -> Result<ArcStr> + 'static>) -> Result<ArcStr> {
    pub type FuncTypeType_aToString<Type_a: Clone> = fn(Type_a) -> Result<ArcStr>;

    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match (inTypeAOption.clone(), inFuncTypeTypeAToString.clone()) {
        (Some(mut a), mut r) => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = r(a.clone())?;
            r#str.clone()
        },
        (None, _) => {
            literal!("")
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

fn printAvlTreeStr(mut inAvlTree: AvlTree) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inAvlTree.clone()) {
        Deref @ FCore::VAvlTree { right: r, left: l, value: Some(FCore::VAvlTreeValue { key: _, value: rval }), .. } => {
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s2 = (getOptionStr(l.clone(), Arc::new(printAvlTreeStr))?).clone();
            s3 = (getOptionStr(r.clone(), Arc::new(printAvlTreeStr))?).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*valueStr(rval.clone())?); __mm_s.push_str(&*literal!(",  ")); __mm_s.push_str(&*if (stringEq((s2.clone()).clone(), (literal!("")).clone())) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(", ")); ArcStr::from(__mm_s) }}); __mm_s.push_str(&*s3.clone()); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        Deref @ FCore::VAvlTree { right: r, left: l, value: None, .. } => {
            let mut s2: ArcStr = arcstr::literal!("");
            let mut s3: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            s2 = (getOptionStr(l.clone(), Arc::new(printAvlTreeStr))?).clone();
            s3 = (getOptionStr(r.clone(), Arc::new(printAvlTreeStr))?).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*if (stringEq((s2.clone()).clone(), (literal!("")).clone())) {literal!("")} else {{ let mut __mm_s = String::new(); __mm_s.push_str(&*s2.clone()); __mm_s.push_str(&*literal!(", ")); ArcStr::from(__mm_s) }}); __mm_s.push_str(&*s3.clone()); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn computeHeight(mut bt: AvlTree) -> Result<AvlTree> {
    let mut outBt: AvlTree;
    outBt = (::match_deref::match_deref! { match &(bt.clone()) {
        Deref @ FCore::VAvlTree { right: r, left: l, value: v @ Some(_), .. } => {
            let mut hl: i32 = 0;
            let mut hr: i32 = 0;
            let mut height: i32 = 0;
            hl = getHeight(l.clone())?;
            hr = getHeight(r.clone())?;
            height = intMax(hl.clone(), hr.clone()) + 1;
            Arc::new(FCore::VAvlTree { value: v.clone(), height: height.clone(), left: l.clone(), right: r.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBt)
}

fn getHeight(mut bt: Option<Arc<FCore::VAvlTree>>) -> Result<i32> {
    let mut height: i32 = 0;
    height = (::match_deref::match_deref! { match &(bt.clone()) {
        None => 0,
        Some(Deref @ FCore::VAvlTree { height, .. }) => height.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(height)
}

pub fn printAvlTreeStrPP(mut inTree: AvlTree) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (printAvlTreeStrPP2(Some(inTree.clone()), (literal!("")).clone())?).clone();
    Ok(outString)
}

fn printAvlTreeStrPP2(mut inTree: Option<Arc<FCore::VAvlTree>>, mut inIndent: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &((inTree.clone(), inIndent.clone())) {
        (None, _) => {
            literal!("")
        },
        (Some(Deref @ FCore::VAvlTree { right: r, left: l, value: Some(FCore::VAvlTreeValue { key: rkey, .. }), .. }), _) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut indent: ArcStr = arcstr::literal!("");
            indent = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone();
            s1 = (printAvlTreeStrPP2(l.clone(), (indent.clone()).clone())?).clone();
            s2 = (printAvlTreeStrPP2(r.clone(), (indent.clone()).clone())?).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*keyStr(rkey.clone())); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        (Some(Deref @ FCore::VAvlTree { right: r, left: l, value: None, .. }), _) => {
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            let mut indent: ArcStr = arcstr::literal!("");
            indent = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone();
            s1 = (printAvlTreeStrPP2(l.clone(), (indent.clone()).clone())?).clone();
            s2 = (printAvlTreeStrPP2(r.clone(), (indent.clone()).clone())?).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn avlTreeReplace(mut inAvlTree: AvlTree, mut inKey: AvlKey, mut inValue: AvlValue) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (::match_deref::match_deref! { match &((inAvlTree.clone(), inKey.clone(), inValue.clone())) {
        (Deref @ FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: rkey, .. }), .. }, key, value) => {
            avlTreeReplace2(inAvlTree.clone(), keyCompare(key.clone(), rkey.clone()), key.clone(), value.clone())?
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("FVisit.avlTreeReplace")); __mm_s.push_str(&*literal!(" failed")); ArcStr::from(__mm_s) }).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outAvlTree)
}

fn avlTreeReplace2(mut inAvlTree: AvlTree, mut inKeyComp: i32, mut inKey: AvlKey, mut inValue: AvlValue) -> Result<AvlTree> {
    let mut outAvlTree: AvlTree;
    outAvlTree = (::match_deref::match_deref! { match &((inAvlTree.clone(), inKeyComp.clone(), inKey.clone(), inValue.clone())) {
        (Deref @ FCore::VAvlTree { right, left, height: h, value: Some(_) }, 0, key, value) => {
            Arc::new(FCore::VAvlTree { value: Some(FCore::VAvlTreeValue { key: key.clone(), value: value.clone() }), height: h.clone(), left: left.clone(), right: right.clone() })
        },
        (Deref @ FCore::VAvlTree { right, left, height: h, value: oval }, 1, key, value) => {
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(right.clone());
            t = avlTreeReplace(t.clone(), key.clone(), value.clone())?;
            Arc::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: left.clone(), right: Some(t.clone()) })
        },
        (Deref @ FCore::VAvlTree { right, left, height: h, value: oval }, (-1), key, value) => {
            let mut t: AvlTree;
            t = createEmptyAvlIfNone(left.clone());
            t = avlTreeReplace(t.clone(), key.clone(), value.clone())?;
            Arc::new(FCore::VAvlTree { value: oval.clone(), height: h.clone(), left: Some(t.clone()), right: right.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outAvlTree)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn getAvlTreeValues(mut tree: Arc<metamodelica::List<Option<Arc<FCore::VAvlTree>>>>, mut acc: Arc<metamodelica::List<FCore::VAvlTreeValue>>) -> Result<Arc<metamodelica::List<FCore::VAvlTreeValue>>> {
    let mut res: Arc<metamodelica::List<FCore::VAvlTreeValue>> = metamodelica::nil();
    res = (::match_deref::match_deref! { match &((tree.clone(), acc.clone())) {
        (Deref @ metamodelica::List::Nil, _) => {
            acc.clone()
        },
        (Deref @ metamodelica::List::Cons { head: Some(Deref @ FCore::VAvlTree { right, left, value, .. }), tail: rest }, _) => {
            getAvlTreeValues(cons(left.clone(), cons(right.clone(), rest.clone())), List::consOption(value.clone(), acc.clone()))?
        },
        (Deref @ metamodelica::List::Cons { head: None, tail: rest }, _) => {
            getAvlTreeValues(rest.clone(), acc.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

pub fn getAvlValue(mut inValue: AvlTreeValue) -> Result<AvlValue> {
    let mut res: AvlValue;
    res = (match inValue.clone() {
        FCore::VAvlTreeValue { value: mut res, .. } => res.clone(),
        _ => bail!("match: no arm matched"),
    });
    Ok(res)
}

// ************************ END AVL Tree implementation ***************************
// ************************ END AVL Tree implementation ***************************
// ************************ END AVL Tree implementation ***************************
// ************************ END AVL Tree implementation ***************************
