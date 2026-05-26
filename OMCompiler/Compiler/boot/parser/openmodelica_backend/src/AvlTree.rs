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

use openmodelica_util::Error;

pub type FuncTypeKeyToStr<Key: Clone> = fn(Key) -> Result<ArcStr>;

pub type FuncTypeValToStr<Val: Clone> = fn(Val) -> Result<ArcStr>;

pub type FuncTypeItemUpdateCheck<Key: Clone, Val: Clone> = fn(Item<Key, Val>, Item<Key, Val>) -> Result<bool>;

pub type FuncTypeKeyCompare<Key: Clone> = fn(Key, Key) -> Result<i32>;

/// a tree is a node and two optional printing functions
/// a tree is a node and two optional printing functions
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tree<Key: Clone, Val: Clone> {
    pub root: Arc<Node<Key, Val>>,
    /// function to compare keys, should return -1, 0, 1 ONLY!
    pub keyCompareFunc: FuncTypeKeyCompare<Key>,
    /// optional function for printing Key
    pub keyStrFuncOpt: Option<FuncTypeKeyToStr<Key>>,
    /// optional function for printing Val
    pub valStrFuncOpt: Option<FuncTypeValToStr<Val>>,
    /// optional function for reporting error on an update of the same item
    ///       if this function is NONE() then updates of items with the same key is allowed!
    ///       this function gets the new item and the old item for easy reporting,
    ///       and should return:
    ///       - true if update is allowed
    ///       - false if update should not be done
    ///       - should print an error message and fail if it wants to fail the update
    pub updateCheckFuncOpt: Option<FuncTypeItemUpdateCheck<Key, Val>>,
    /// a name for this tree so you know which one it is if you have more
    pub name: ArcStr,
}

pub type TREE<Key, Val> = Tree<Key, Val>;


/// The binary tree data structure
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Node<Key, Val> {
    NODE {
        /// Val
        item: Item<Key, Val>,
        /// height of tree, used for balancing
        height: i32,
        /// left subtree
        left: Arc<Node<Key, Val>>,
        /// right subtree
        right: Arc<Node<Key, Val>>,
    },
    /// no node, empty tree
    NO_NODE,
}
pub use self::Node::{NODE,NO_NODE};

/// Each node in the binary tree can have an item associated with it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Item<Key, Val> {
    ITEM {
        /// Key
        key: Key,
        /// Val
        val: Val,
    },
    /// no item
    NO_ITEM,
}
pub use self::Item::{ITEM,NO_ITEM};

pub fn name<Key: Clone + 'static, Val: Clone + 'static>(mut tree: Tree<Key, Val>) -> Result<ArcStr> {
    let mut name: ArcStr = arcstr::literal!("");
    let Tree { name: __pa0, .. } = (tree.clone()) else { bail!("pattern mismatch") };
    name = __pa0.clone();
    Ok(name)
}

pub fn create<Key: Clone + 'static, Val: Clone + 'static>(mut name: ArcStr, mut inKeyCompareFunc: FuncTypeKeyCompare<Key>, mut inKeyStrFuncOpt: Option<FuncTypeKeyToStr<Key>>, mut inValStrFuncOpt: Option<FuncTypeValToStr<Val>>, mut inUpdateCheckFuncOpt: Option<FuncTypeItemUpdateCheck<Key, Val>>) -> Tree<Key, Val> {
    let mut tree: Tree<Key, Val>;
    tree = Tree { root: Arc::new(Node::NODE { item: crate::AvlTree::Item::NO_ITEM, height: 0, left: Arc::new(crate::AvlTree::Node::NO_NODE), right: Arc::new(crate::AvlTree::Node::NO_NODE) }), keyCompareFunc: inKeyCompareFunc, keyStrFuncOpt: inKeyStrFuncOpt.clone(), valStrFuncOpt: inValStrFuncOpt.clone(), updateCheckFuncOpt: inUpdateCheckFuncOpt.clone(), name: (name.clone()).clone() };
    tree
}

pub fn hasPrintingFunctions<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut tree: Tree<Key, Val>) -> Result<bool> {
    let mut hasPrinting: bool = false;
    let mut kf: Option<FuncTypeKeyToStr<Key>> = None;
    let mut vf: Option<FuncTypeValToStr<Val>> = None;
    let Tree { valStrFuncOpt: __pa0, keyStrFuncOpt: __pa1, .. } = (tree.clone()) else { bail!("pattern mismatch") };
    vf = __pa0.clone();
    kf = __pa1.clone();
    hasPrinting = boolNot(boolOr(None == kf.clone(), None == vf.clone()));
    Ok(hasPrinting)
}

pub fn hasUpdateCheckFunction<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut tree: Tree<Key, Val>) -> Result<bool> {
    let mut hasUpdateCheck: bool = false;
    let mut uf: Option<FuncTypeItemUpdateCheck<Key, Val>> = None;
    let Tree { updateCheckFuncOpt: __pa0, .. } = (tree.clone()) else { bail!("pattern mismatch") };
    uf = __pa0.clone();
    hasUpdateCheck = boolNot(None == uf.clone());
    Ok(hasUpdateCheck)
}

pub fn getUpdateCheckFunc<Key: Clone + 'static, Val: Clone + 'static>(mut tree: Tree<Key, Val>) -> Result<FuncTypeItemUpdateCheck<Key, Val>> {
    let mut outUpdateCheckFunc: FuncTypeItemUpdateCheck<Key, Val>;
    let Tree { updateCheckFuncOpt: Some(__pa0), .. } = (tree.clone()) else { bail!("pattern mismatch") };
    outUpdateCheckFunc = __pa0.clone();
    Ok(outUpdateCheckFunc)
}

pub fn getKeyCompareFunc<Key: Clone + 'static, Val: Clone + 'static>(mut tree: Tree<Key, Val>) -> Result<FuncTypeKeyCompare<Key>> {
    let mut outKeyCompareFunc: FuncTypeKeyCompare<Key>;
    let Tree { keyCompareFunc: __pa0, .. } = (tree.clone()) else { bail!("pattern mismatch") };
    outKeyCompareFunc = __pa0.clone();
    Ok(outKeyCompareFunc)
}

pub fn getKeyToStrFunc<Key: Clone + 'static, Val: Clone + 'static>(mut tree: Tree<Key, Val>) -> Result<FuncTypeKeyToStr<Key>> {
    let mut outKey2StrFunc: FuncTypeKeyToStr<Key>;
    let Tree { keyStrFuncOpt: Some(__pa0), .. } = (tree.clone()) else { bail!("pattern mismatch") };
    outKey2StrFunc = __pa0.clone();
    Ok(outKey2StrFunc)
}

pub fn getValToStrFunc<Key: Clone + 'static, Val: Clone + 'static>(mut tree: Tree<Key, Val>) -> Result<FuncTypeValToStr<Val>> {
    let mut outVal2StrFunc: FuncTypeValToStr<Val>;
    let Tree { valStrFuncOpt: Some(__pa0), .. } = (tree.clone()) else { bail!("pattern mismatch") };
    outVal2StrFunc = __pa0.clone();
    Ok(outVal2StrFunc)
}

fn newLeafNode<Key: Clone + 'static, Val: Clone + 'static>(mut inItem: Item<Key, Val>, mut height: i32) -> Arc<Node<Key, Val>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = Arc::new(Node::NODE { item: inItem.clone(), height: 1, left: Arc::new(crate::AvlTree::Node::NO_NODE), right: Arc::new(crate::AvlTree::Node::NO_NODE) });
    outNode
}

pub fn add<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>, mut inKey: Key, mut inVal: Val) -> Result<Tree<Key, Val>> {
    let mut outTree: Tree<Key, Val>;
    outTree = 'mc: {
        let __mc_input = (inTree.clone(), inKey.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (Tree { root: ref node, keyCompareFunc: mut cf, keyStrFuncOpt: mut kf, valStrFuncOpt: mut vf, updateCheckFuncOpt: mut uf, name: mut n }, mut key) = __mc_input.clone() else { bail!("nomatch") };
            let mut node = node.clone();
            node = addNode(inTree.clone(), node.clone(), key.clone(), inVal.clone())?;
            Ok(Tree { root: node.clone(), keyCompareFunc: cf, keyStrFuncOpt: kf.clone(), valStrFuncOpt: vf.clone(), updateCheckFuncOpt: uf.clone(), name: (n.clone()).clone() })
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AvlTree.add name: ")); __mm_s.push_str(&*name(inTree.clone())?); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outTree)
}

fn addNode<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKey: Key, mut inVal: Val) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &((inTree.clone(), inNode.clone(), inKey.clone(), inVal.clone())) {
        (_, Deref @ Node::NO_NODE { .. }, _, _) => {
            let mut n: Arc<Node<Key, Val>>;
            n = newLeafNode(Item::ITEM { key: inKey.clone(), val: inVal.clone() }, 1);
            n.clone()
        },
        (_, Deref @ Node::NODE { right: Deref @ Node::NO_NODE { .. }, left: Deref @ Node::NO_NODE { .. }, item: Item::NO_ITEM { .. }, .. }, key, val) => {
            let mut n: Arc<Node<Key, Val>>;
            n = newLeafNode(Item::ITEM { key: key.clone(), val: val.clone() }, 1);
            n.clone()
        },
        (Tree { keyCompareFunc, .. }, Deref @ Node::NODE { item: Item::ITEM { key: rkey, .. }, .. }, key, val) => {
            let mut n: Arc<Node<Key, Val>>;
            let mut order: i32 = 0;
            order = keyCompareFunc(key.clone(), rkey.clone())?;
            n = balance(addNode_dispatch(inTree.clone(), inNode.clone(), order.clone(), key.clone(), val.clone())?)?;
            n.clone()
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AvlTree.addNode name: ")); __mm_s.push_str(&*name(inTree.clone())?); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

fn addNode_dispatch<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKeyComp: i32, mut inKey: Key, mut inVal: Val) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = 'mc: {
        let __mc_input = (inNode.clone(), inKeyComp.clone(), inKey.clone(), inVal.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: _, height: h, left: l, right: r }, 0, key, val) => {
                    let false = (hasUpdateCheckFunction(inTree.clone())?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(Node::NODE { item: Item::ITEM { key: key.clone(), val: val.clone() }, height: h.clone(), left: l.clone(), right: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: i, height: h, left: l, right: r }, 0, key, val) => {
                    let mut updateCheckFunc: FuncTypeItemUpdateCheck<Key, Val>;
                    let true = (hasUpdateCheckFunction(inTree.clone())?) else { bail!("pattern mismatch") };
                    updateCheckFunc = getUpdateCheckFunc(inTree.clone())?;
                    let true = (updateCheckFunc(i.clone(), Item::ITEM { key: key.clone(), val: val.clone() })?) else { bail!("pattern mismatch") };
                    Ok(Arc::new(Node::NODE { item: Item::ITEM { key: key.clone(), val: val.clone() }, height: h.clone(), left: l.clone(), right: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { item: i, height: _, left: _, right: _ }, 0, key, val) => {
                    let mut updateCheckFunc: FuncTypeItemUpdateCheck<Key, Val>;
                    let true = (hasUpdateCheckFunction(inTree.clone())?) else { bail!("pattern mismatch") };
                    updateCheckFunc = getUpdateCheckFunc(inTree.clone())?;
                    let false = (updateCheckFunc(i.clone(), Item::ITEM { key: key.clone(), val: val.clone() })?) else { bail!("pattern mismatch") };
                    Ok(inNode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { right: r, left: l, height: h, item: i }, 1, key, val) => {
                    let mut n: Arc<Node<Key, Val>>;
                    n = emptyNodeIfNoNode(r.clone())?;
                    n = addNode(inTree.clone(), n.clone(), key.clone(), val.clone())?;
                    Ok(Arc::new(Node::NODE { item: i.clone(), height: h.clone(), left: l.clone(), right: n.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Node::NODE { right: r, left: l, height: h, item: i }, (-1), key, val) => {
                    let mut n: Arc<Node<Key, Val>>;
                    n = emptyNodeIfNoNode(l.clone())?;
                    n = addNode(inTree.clone(), n.clone(), key.clone(), val.clone())?;
                    Ok(Arc::new(Node::NODE { item: i.clone(), height: h.clone(), left: n.clone(), right: r.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

pub fn get<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inKey: Key) -> Result<Val> {
    let mut outVal: Val;
    let mut node: Arc<Node<Key, Val>>;
    let Tree { root: __pa0, .. } = (inTree.clone()) else { bail!("pattern mismatch") };
    node = __pa0.clone();
    outVal = getNode(inTree.clone(), node.clone(), inKey.clone())?;
    Ok(outVal)
}

fn getNode<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKey: Key) -> Result<Val> {
    let mut outVal: Val;
    let mut rkey: Key;
    let mut keyCompareFunc: FuncTypeKeyCompare<Key>;
    let mut order: i32 = 0;
    let __pa0 = ::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Node::NODE { item: Item::ITEM { key: __pa0, .. }, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    rkey = __pa0.clone();
    keyCompareFunc = getKeyCompareFunc(inTree.clone())?;
    order = keyCompareFunc(inKey.clone(), rkey.clone())?;
    outVal = getNode_dispatch(inTree.clone(), inNode.clone(), order.clone(), inKey.clone())?;
    Ok(outVal)
}

fn getNode_dispatch<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKeyComp: i32, mut inKey: Key) -> Result<Val> {
    let mut outVal: Val;
    outVal = (::match_deref::match_deref! { match &((inNode.clone(), inKeyComp.clone())) {
        (Deref @ Node::NODE { item: Item::ITEM { val, .. }, .. }, 0) => {
            val.clone()
        },
        (Deref @ Node::NODE { right: r, .. }, 1) => {
            getNode(inTree.clone(), r.clone(), inKey.clone())?
        },
        (Deref @ Node::NODE { left: l, .. }, (-1)) => {
            getNode(inTree.clone(), l.clone(), inKey.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outVal)
}

pub fn replace<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inKey: Key, mut inVal: Val) -> Result<Tree<Key, Val>> {
    let mut outTree: Tree<Key, Val>;
    outTree = (match (inTree.clone(), inKey.clone(), inVal.clone()) {
        (Tree { root: ref node, keyCompareFunc: mut keyCompareFunc, keyStrFuncOpt: mut kf, valStrFuncOpt: mut vf, updateCheckFuncOpt: mut uf, name: mut n }, mut key, mut val) => {
            let mut node = node.clone();
            node = replaceNode(inTree.clone(), node.clone(), key.clone(), val.clone())?;
            Tree { root: node.clone(), keyCompareFunc: keyCompareFunc, keyStrFuncOpt: kf.clone(), valStrFuncOpt: vf.clone(), updateCheckFuncOpt: uf.clone(), name: (n.clone()).clone() }
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AvlTree.replace name: ")); __mm_s.push_str(&*name(inTree.clone())?); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
    });
    Ok(outTree)
}

pub fn replaceNode<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKey: Key, mut inVal: Val) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &((inTree.clone(), inNode.clone(), inKey.clone(), inVal.clone())) {
        (Tree { keyCompareFunc, .. }, Deref @ Node::NODE { item: Item::ITEM { key: rkey, .. }, .. }, key, val) => {
            let mut n: Arc<Node<Key, Val>>;
            let mut order: i32 = 0;
            order = keyCompareFunc(key.clone(), rkey.clone())?;
            n = replaceNode_dispatch(inTree.clone(), inNode.clone(), order.clone(), key.clone(), val.clone())?;
            n.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn replaceNode_dispatch<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKeyComp: i32, mut inKey: Key, mut inVal: Val) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &((inNode.clone(), inKeyComp.clone(), inKey.clone(), inVal.clone())) {
        (Deref @ Node::NODE { right: r, left: l, height: h, item: Item::ITEM { .. } }, 0, key, val) => {
            Arc::new(Node::NODE { item: Item::ITEM { key: key.clone(), val: val.clone() }, height: h.clone(), left: l.clone(), right: r.clone() })
        },
        (Deref @ Node::NODE { right: r, left: l, height: h, item: i }, 1, key, val) => {
            let mut n: Arc<Node<Key, Val>>;
            n = emptyNodeIfNoNode(r.clone())?;
            n = replaceNode(inTree.clone(), n.clone(), key.clone(), val.clone())?;
            Arc::new(Node::NODE { item: i.clone(), height: h.clone(), left: l.clone(), right: n.clone() })
        },
        (Deref @ Node::NODE { right: r, left: l, height: h, item: i }, (-1), key, val) => {
            let mut n: Arc<Node<Key, Val>>;
            n = emptyNodeIfNoNode(l.clone())?;
            n = replaceNode(inTree.clone(), n.clone(), key.clone(), val.clone())?;
            Arc::new(Node::NODE { item: i.clone(), height: h.clone(), left: n.clone(), right: r.clone() })
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn emptyNodeIfNoNode<Key: Clone + 'static, Val: Clone + 'static>(mut inNode: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Node::NO_NODE { .. } => Arc::new(Node::NODE { item: crate::AvlTree::Item::NO_ITEM, height: 0, left: Arc::new(crate::AvlTree::Node::NO_NODE), right: Arc::new(crate::AvlTree::Node::NO_NODE) }),
        Deref @ Node::NODE { .. } => inNode.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn balance<Key: Clone + 'static, Val: Clone + 'static>(mut inNode: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    let mut d: i32 = 0;
    d = differenceInHeight(inNode.clone())?;
    outNode = doBalance(d.clone(), inNode.clone())?;
    Ok(outNode)
}

fn doBalance<Key: Clone + 'static, Val: Clone + 'static>(mut difference: i32, mut inNode: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = (match difference.clone() {
        (-1) => computeHeight(inNode.clone())?,
        0 => computeHeight(inNode.clone())?,
        1 => computeHeight(inNode.clone())?,
        _ => doBalance2(difference.clone() < 0, inNode.clone())?,
    });
    Ok(outNode)
}

fn doBalance2<Key: Clone + 'static, Val: Clone + 'static>(mut inDiffIsNegative: bool, mut inNode: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = (::match_deref::match_deref! { match &((inDiffIsNegative.clone(), inNode.clone())) {
        (true, n) => {
            let mut n = (*n).clone();
            n = doBalance3(n.clone())?;
            n = rotateLeft(n.clone())?;
            n.clone()
        },
        (false, n) => {
            let mut n = (*n).clone();
            n = doBalance4(n.clone())?;
            n = rotateRight(n.clone())?;
            n.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outNode)
}

fn doBalance3<Key: Clone + 'static, Val: Clone + 'static>(mut inNode: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = 'mc: {
        let __mc_input = inNode.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                n => {
                    let mut rr: Arc<Node<Key, Val>>;
                    let mut rN: Arc<Node<Key, Val>>;
                    let mut n = (*n).clone();
                    rN = rightNode(n.clone())?;
                    let true = (differenceInHeight(rN.clone())? > 0) else { bail!("pattern mismatch") };
                    rr = rotateRight(rN.clone())?;
                    n = setRight(n.clone(), rr.clone())?;
                    Ok(n.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn doBalance4<Key: Clone + 'static, Val: Clone + 'static>(mut inNode: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = 'mc: {
        let __mc_input = inNode.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                n => {
                    let mut rl: Arc<Node<Key, Val>>;
                    let mut lN: Arc<Node<Key, Val>>;
                    let mut n = (*n).clone();
                    lN = leftNode(n.clone())?;
                    let true = (differenceInHeight(lN.clone())? < 0) else { bail!("pattern mismatch") };
                    rl = rotateLeft(lN.clone())?;
                    n = setLeft(n.clone(), rl.clone())?;
                    Ok(n.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inNode.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outNode)
}

fn setRight<Key: Clone + 'static, Val: Clone + 'static>(mut node: Arc<Node<Key, Val>>, mut right: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    let mut item: Item<Key, Val>;
    let mut l: Arc<Node<Key, Val>>;
    let mut height: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(node.clone()) {
        Deref @ Node::NODE { item: __pa0, height: __pa1, left: __pa2, right: _ } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    item = __pa0.clone();
    height = __pa1.clone();
    l = __pa2.clone();
    outNode = Arc::new(Node::NODE { item: item.clone(), height: height.clone(), left: l.clone(), right: right.clone() });
    Ok(outNode)
}

fn setLeft<Key: Clone + 'static, Val: Clone + 'static>(mut node: Arc<Node<Key, Val>>, mut left: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    let mut item: Item<Key, Val>;
    let mut r: Arc<Node<Key, Val>>;
    let mut height: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(node.clone()) {
        Deref @ Node::NODE { item: __pa0, height: __pa1, left: _, right: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    item = __pa0.clone();
    height = __pa1.clone();
    r = __pa2.clone();
    outNode = Arc::new(Node::NODE { item: item.clone(), height: height.clone(), left: left.clone(), right: r.clone() });
    Ok(outNode)
}

fn leftNode<Key: Clone + 'static, Val: Clone + 'static>(mut node: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut subNode: Arc<Node<Key, Val>>;
    let __pa0 = ::match_deref::match_deref! { match &(node.clone()) {
        Deref @ Node::NODE { left: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    subNode = __pa0.clone();
    Ok(subNode)
}

fn rightNode<Key: Clone + 'static, Val: Clone + 'static>(mut node: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut subNode: Arc<Node<Key, Val>>;
    let __pa0 = ::match_deref::match_deref! { match &(node.clone()) {
        Deref @ Node::NODE { right: __pa0, .. } => __pa0.clone(),
        _ => bail!("pattern mismatch"),
    } };
    subNode = __pa0.clone();
    Ok(subNode)
}

fn exchangeLeft<Key: Clone + 'static, Val: Clone + 'static>(mut inNode: Arc<Node<Key, Val>>, mut inParent: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outParent: Arc<Node<Key, Val>>;
    let mut parent: Arc<Node<Key, Val>>;
    let mut node: Arc<Node<Key, Val>>;
    parent = setRight(inParent.clone(), leftNode(inNode.clone())?)?;
    parent = balance(parent.clone())?;
    node = setLeft(inNode.clone(), parent.clone())?;
    outParent = balance(node.clone())?;
    Ok(outParent)
}

fn exchangeRight<Key: Clone + 'static, Val: Clone + 'static>(mut inNode: Arc<Node<Key, Val>>, mut inParent: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outParent: Arc<Node<Key, Val>>;
    let mut parent: Arc<Node<Key, Val>>;
    let mut node: Arc<Node<Key, Val>>;
    parent = setLeft(inParent.clone(), rightNode(inNode.clone())?)?;
    parent = balance(parent.clone())?;
    node = setRight(inNode.clone(), parent.clone())?;
    outParent = balance(node.clone())?;
    Ok(outParent)
}

fn rotateLeft<Key: Clone + 'static, Val: Clone + 'static>(mut node: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = exchangeLeft(rightNode(node.clone())?, node.clone())?;
    Ok(outNode)
}

fn rotateRight<Key: Clone + 'static, Val: Clone + 'static>(mut node: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    outNode = exchangeRight(leftNode(node.clone())?, node.clone())?;
    Ok(outNode)
}

fn differenceInHeight<Key: Clone + 'static, Val: Clone + 'static>(mut node: Arc<Node<Key, Val>>) -> Result<i32> {
    let mut diff: i32 = 0;
    let mut l: Arc<Node<Key, Val>>;
    let mut r: Arc<Node<Key, Val>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(node.clone()) {
        Deref @ Node::NODE { right: __pa0, left: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r = __pa0.clone();
    l = __pa1.clone();
    diff = getHeight(l.clone())? - getHeight(r.clone())?;
    Ok(diff)
}

fn computeHeight<Key: Clone + 'static, Val: Clone + 'static>(mut inNode: Arc<Node<Key, Val>>) -> Result<Arc<Node<Key, Val>>> {
    let mut outNode: Arc<Node<Key, Val>>;
    let mut l: Arc<Node<Key, Val>>;
    let mut r: Arc<Node<Key, Val>>;
    let mut i: Item<Key, Val>;
    let mut val: Val;
    let mut hl: i32 = 0;
    let mut hr: i32 = 0;
    let mut height: i32 = 0;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Node::NODE { right: __pa0, left: __pa1, item: __pa2 @ Item::ITEM { .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => bail!("pattern mismatch"),
    } };
    r = __pa0.clone();
    l = __pa1.clone();
    i = __pa2.clone();
    hl = getHeight(l.clone())?;
    hr = getHeight(r.clone())?;
    height = intMax(hl.clone(), hr.clone()) + 1;
    outNode = Arc::new(Node::NODE { item: i.clone(), height: height.clone(), left: l.clone(), right: r.clone() });
    Ok(outNode)
}

fn getHeight<Key: Clone + 'static, Val: Clone + 'static>(mut bt: Arc<Node<Key, Val>>) -> Result<i32> {
    let mut height: i32 = 0;
    height = (::match_deref::match_deref! { match &(bt.clone()) {
        Deref @ Node::NO_NODE { .. } => 0,
        Deref @ Node::NODE { height, .. } => height.clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(height)
}

pub fn prettyPrintTreeStr<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (prettyPrintTreeStr_dispatch(inTree.clone(), (literal!("")).clone())?).clone();
    Ok(outString)
}

fn prettyPrintTreeStr_dispatch<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>, mut inIndent: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut node: Arc<Node<Key, Val>>;
    if !(hasPrintingFunctions(inTree.clone())?) {
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TreePrintError<NO_PRINTING_FUNCTIONS_ATTACHED> name[")); __mm_s.push_str(&*name(inTree.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        return Ok(outString);
    }
    let Tree { root: __pa0, .. } = (inTree.clone()) else { bail!("pattern mismatch") };
    node = __pa0.clone();
    outString = (prettyPrintNodeStr(inTree.clone(), node.clone(), (inIndent.clone()).clone())?).clone();
    Ok(outString)
}

fn prettyPrintNodeStr<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inIndent: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Node::NO_NODE { .. } => {
            literal!("")
        },
        Deref @ Node::NODE { right: r, left: l, item: Item::NO_ITEM { .. }, .. } => {
            let mut indent: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            indent = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone();
            s1 = (prettyPrintNodeStr(inTree.clone(), l.clone(), (indent.clone()).clone())?).clone();
            s2 = (prettyPrintNodeStr(inTree.clone(), r.clone(), (indent.clone()).clone())?).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        Deref @ Node::NODE { right: r, left: l, item: item @ Item::ITEM { .. }, .. } => {
            let mut indent: ArcStr = arcstr::literal!("");
            let mut s1: ArcStr = arcstr::literal!("");
            let mut s2: ArcStr = arcstr::literal!("");
            let mut res: ArcStr = arcstr::literal!("");
            indent = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*literal!("  ")); ArcStr::from(__mm_s) }).clone();
            s1 = (prettyPrintNodeStr(inTree.clone(), l.clone(), (indent.clone()).clone())?).clone();
            s2 = (prettyPrintNodeStr(inTree.clone(), r.clone(), (indent.clone()).clone())?).clone();
            res = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*printItemStr(inTree.clone(), item.clone())?); __mm_s.push_str(&*s1.clone()); __mm_s.push_str(&*s2.clone()); ArcStr::from(__mm_s) }).clone();
            res.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printTreeStr<Key: Clone + 'static + PartialEq, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut node: Arc<Node<Key, Val>>;
    if !(hasPrintingFunctions(inTree.clone())?) {
        outString = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TreePrintError<NO_PRINTING_FUNCTIONS_ATTACHED> name[")); __mm_s.push_str(&*name(inTree.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
        return Ok(outString);
    }
    let Tree { root: __pa0, .. } = (inTree.clone()) else { bail!("pattern mismatch") };
    node = __pa0.clone();
    outString = (printNodeStr(inTree.clone(), node.clone())?).clone();
    Ok(outString)
}

fn printNodeStr<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Node::NO_NODE { .. } => {
            literal!("")
        },
        Deref @ Node::NODE { item: Item::NO_ITEM { .. }, .. } => {
            literal!("")
        },
        Deref @ Node::NODE { right, left, item: item @ Item::ITEM { .. }, .. } => {
            let mut left_str: ArcStr = arcstr::literal!("");
            let mut right_str: ArcStr = arcstr::literal!("");
            let mut item_str: ArcStr = arcstr::literal!("");
            let mut r#str: ArcStr = arcstr::literal!("");
            left_str = (printNodeStr(inTree.clone(), left.clone())?).clone();
            right_str = (printNodeStr(inTree.clone(), right.clone())?).clone();
            item_str = (printItemStr(inTree.clone(), item.clone())?).clone();
            r#str = stringAppendList(list![(literal!("i: ")).clone(), (item_str.clone()).clone(), (literal!(", l: ")).clone(), (left_str.clone()).clone(), (literal!(", r: ")).clone(), (right_str.clone()).clone()]);
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printItemStr<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inItem: Item<Key, Val>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((match inItem.clone() {
        Item::NO_ITEM { .. } => {
            literal!("[]")
        },
        Item::ITEM { val: mut val, key: mut key } => {
            let mut r#str: ArcStr = arcstr::literal!("");
            let mut keyStr: ArcStr = arcstr::literal!("");
            let mut valStr: ArcStr = arcstr::literal!("");
            let mut key2Str: FuncTypeKeyToStr<Key>;
            let mut val2Str: FuncTypeValToStr<Val>;
            key2Str = getKeyToStrFunc(inTree.clone())?;
            val2Str = getValToStrFunc(inTree.clone())?;
            keyStr = key2Str(key.clone())?;
            valStr = val2Str(val.clone())?;
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*keyStr.clone()); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valStr.clone()); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone();
            r#str.clone()
        },
        _ => bail!("match: no arm matched"),
    })).clone();
    Ok(outString)
}

pub fn getKeyOfVal<Key: Clone + 'static, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>, mut inVal: Val) -> Result<Key> {
    let mut outKey: Key;
    let mut node: Arc<Node<Key, Val>>;
    let mut key: Key;
    let Tree { root: __pa0, .. } = (inTree.clone()) else { bail!("pattern mismatch") };
    node = __pa0.clone();
    outKey = getKeyOfValNode(inTree.clone(), node.clone(), inVal.clone())?;
    Ok(outKey)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn getKeyOfValNode<Key: Clone + 'static, Val: Clone + 'static + PartialEq>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inVal: Val) -> Result<Key> {
    let mut outKey: Key;
    outKey = 'mc: {
        let __mc_input = inNode.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Node::NODE { item: Item::ITEM { key: k, val: v }, .. } => {
                    let true = (v.clone() == inVal.clone()) else { bail!("pattern mismatch") };
                    Ok(k.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Node::NODE { left, item: Item::ITEM { key: _, val: v }, .. } => {
                    let mut k: Key;
                    let false = (v.clone() == inVal.clone()) else { bail!("pattern mismatch") };
                    k = getKeyOfValNode(inTree.clone(), left.clone(), inVal.clone())?;
                    Ok(k.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Node::NODE { right, item: Item::ITEM { key: _, val: v }, .. } => {
                    let mut k: Key;
                    let false = (v.clone() == inVal.clone()) else { bail!("pattern mismatch") };
                    k = getKeyOfValNode(inTree.clone(), right.clone(), inVal.clone())?;
                    Ok(k.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outKey)
}

pub fn addUnique<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inKey: Key, mut inVal: Val) -> Result<(Tree<Key, Val>, Item<Key, Val>)> {
    let mut outTree: Tree<Key, Val>;
    let mut outItem: Item<Key, Val>;
    (outTree, outItem) = 'mc: {
        let __mc_input = (inTree.clone(), inKey.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (Tree { root: ref node, keyCompareFunc: mut cf, keyStrFuncOpt: mut kf, valStrFuncOpt: mut vf, updateCheckFuncOpt: mut uf, name: mut n }, mut key) = __mc_input.clone() else { bail!("nomatch") };
            let mut item: Item<Key, Val>;
            let mut node = node.clone();
            (node, item) = addNodeUnique(inTree.clone(), node.clone(), key.clone(), inVal.clone())?;
            Ok((Tree { root: node.clone(), keyCompareFunc: cf, keyStrFuncOpt: kf.clone(), valStrFuncOpt: vf.clone(), updateCheckFuncOpt: uf.clone(), name: (n.clone()).clone() }, item.clone()))
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { bail!("nomatch") };
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AvlTree.addUnique name: ")); __mm_s.push_str(&*name(inTree.clone())?); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            Ok(bail!("fail"))
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outTree, outItem))
}

fn addNodeUnique<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKey: Key, mut inVal: Val) -> Result<(Arc<Node<Key, Val>>, Item<Key, Val>)> {
    let mut outNode: Arc<Node<Key, Val>>;
    let mut outItem: Item<Key, Val>;
    (outNode, outItem) = (::match_deref::match_deref! { match &((inTree.clone(), inNode.clone(), inKey.clone(), inVal.clone())) {
        (_, Deref @ Node::NO_NODE { .. }, _, _) => {
            let mut item: Item<Key, Val>;
            let mut n: Arc<Node<Key, Val>>;
            item = Item::ITEM { key: inKey.clone(), val: inVal.clone() };
            n = newLeafNode(item.clone(), 1);
            (n.clone(), item.clone())
        },
        (_, Deref @ Node::NODE { right: Deref @ Node::NO_NODE { .. }, left: Deref @ Node::NO_NODE { .. }, item: Item::NO_ITEM { .. }, .. }, key, val) => {
            let mut item: Item<Key, Val>;
            let mut n: Arc<Node<Key, Val>>;
            item = Item::ITEM { key: key.clone(), val: val.clone() };
            n = newLeafNode(item.clone(), 1);
            (n.clone(), item.clone())
        },
        (Tree { keyCompareFunc, .. }, Deref @ Node::NODE { item: Item::ITEM { key: rkey, .. }, .. }, key, val) => {
            let mut item: Item<Key, Val>;
            let mut n: Arc<Node<Key, Val>>;
            let mut order: i32 = 0;
            order = keyCompareFunc(key.clone(), rkey.clone())?;
            (n, item) = addNodeUnique_dispatch(inTree.clone(), inNode.clone(), order.clone(), key.clone(), val.clone())?;
            n = balance(n.clone())?;
            (n.clone(), item.clone())
        },
        _ => {
            let mut r#str: ArcStr = arcstr::literal!("");
            r#str = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("AvlTree.addNodeUnique name: ")); __mm_s.push_str(&*name(inTree.clone())?); __mm_s.push_str(&*literal!(" failed!")); ArcStr::from(__mm_s) }).clone();
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(r#str.clone()).clone()])?;
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outNode, outItem))
}

fn addNodeUnique_dispatch<Key: Clone + 'static, Val: Clone + 'static>(mut inTree: Tree<Key, Val>, mut inNode: Arc<Node<Key, Val>>, mut inKeyComp: i32, mut inKey: Key, mut inVal: Val) -> Result<(Arc<Node<Key, Val>>, Item<Key, Val>)> {
    let mut outNode: Arc<Node<Key, Val>>;
    let mut outItem: Item<Key, Val>;
    (outNode, outItem) = (::match_deref::match_deref! { match &((inNode.clone(), inKeyComp.clone())) {
        (Deref @ Node::NODE { item: i, height: _, left: _, right: _ }, 0) => {
            (inNode.clone(), i.clone())
        },
        (Deref @ Node::NODE { right: r, left: l, height: h, item: i }, 1) => {
            let mut n: Arc<Node<Key, Val>>;
            let mut it: Item<Key, Val>;
            n = emptyNodeIfNoNode(r.clone())?;
            (n, it) = addNodeUnique(inTree.clone(), n.clone(), inKey.clone(), inVal.clone())?;
            (Arc::new(Node::NODE { item: i.clone(), height: h.clone(), left: l.clone(), right: n.clone() }), it.clone())
        },
        (Deref @ Node::NODE { right: r, left: l, height: h, item: i }, (-1)) => {
            let mut n: Arc<Node<Key, Val>>;
            let mut it: Item<Key, Val>;
            n = emptyNodeIfNoNode(l.clone())?;
            (n, it) = addNodeUnique(inTree.clone(), n.clone(), inKey.clone(), inVal.clone())?;
            (Arc::new(Node::NODE { item: i.clone(), height: h.clone(), left: n.clone(), right: r.clone() }), it.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outNode, outItem))
}

