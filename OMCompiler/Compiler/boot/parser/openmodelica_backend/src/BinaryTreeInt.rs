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
use openmodelica_util::Util;

/* *************************
  imports
 **************************/
/* *************************
  types
 **************************/
/// Generic Binary tree implementation
///  - Binary Tree
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinTree {
    /// Value
    pub value: Option<TreeValue>,
    /// left subtree
    pub leftSubTree: Option<Arc<BinTree>>,
    /// right subtree
    pub rightSubTree: Option<Arc<BinTree>>,
}

pub type TREENODE = BinTree;


/// Each node in the binary tree can have a value associated with it.
///  - Tree Value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeValue {
    /// Key
    pub key: Key,
    /// Value
    pub value: Value,
}

pub type TREEVALUE = TreeValue;


/// A key is a Integer
pub type Key = i32;

/// - Value
pub type Value = i32;

pub static emptyBinTree: std::sync::LazyLock<Arc<BinTree>> = std::sync::LazyLock::new(|| { Arc::new(BinTree { value: None, leftSubTree: None, rightSubTree: None }) });

/* *************************
  implementation
 **************************/
fn keyCmp(mut keya: Key, mut keyb: Key) -> i32 {
    let mut cmp: i32 = 0;
    cmp = Util::intSign(keya.clone() - keyb.clone());
    cmp
}

pub fn treeGet(mut bt: Arc<BinTree>, mut key: Key) -> Result<Value> {
    let mut v: Value = 0;
    let mut keystr: ArcStr = arcstr::literal!("");
    let mut keyhash: i32 = 0;
    v = treeGet3(bt.clone(), key.clone(), treeGet2(bt.clone(), key.clone())?)?;
    Ok(v)
}

fn treeGet2(mut inBinTree: Arc<BinTree>, mut ikey: Key) -> Result<i32> {
    let mut compResult: i32 = 0;
    compResult = (::match_deref::match_deref! { match &(inBinTree.clone()) {
        Deref @ BinTree { value: Some(TreeValue { key, .. }), .. } => {
            keyCmp(key.clone(), ikey.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(compResult)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn treeGet3(mut inBinTree: Arc<BinTree>, mut ikey: Key, mut inCompResult: i32) -> Result<Value> {
    let mut outValue: Value = 0;
    outValue = (::match_deref::match_deref! { match &((inBinTree.clone(), inCompResult.clone())) {
        (Deref @ BinTree { value: Some(TreeValue { value: rval, .. }), .. }, 0) => {
            rval.clone()
        },
        (Deref @ BinTree { rightSubTree: Some(right), .. }, 1) => {
            let mut compResult: i32 = 0;
            compResult = treeGet2(right.clone(), ikey.clone())?;
            treeGet3(right.clone(), ikey.clone(), compResult.clone())?
        },
        (Deref @ BinTree { leftSubTree: Some(left), .. }, (-1)) => {
            let mut compResult: i32 = 0;
            compResult = treeGet2(left.clone(), ikey.clone())?;
            treeGet3(left.clone(), ikey.clone(), compResult.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn treeAddList(mut inBinTree: Arc<BinTree>, mut inKeyLst: Arc<metamodelica::List<i32>>) -> Result<Arc<BinTree>> {
    let mut outBinTree: Arc<BinTree>;
    outBinTree = (::match_deref::match_deref! { match &((inBinTree.clone(), inKeyLst.clone())) {
        (bt, Deref @ metamodelica::List::Nil) => {
            bt.clone()
        },
        (bt, Deref @ metamodelica::List::Cons { head: key, tail: res }) => {
            let mut bt_1: Arc<BinTree>;
            let mut bt_2: Arc<BinTree>;
            bt_1 = treeAdd(bt.clone(), key.clone(), 0)?;
            bt_2 = treeAddList(bt_1.clone(), res.clone())?;
            bt_2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinTree)
}

pub fn treeAdd(mut inBinTree: Arc<BinTree>, mut inKey: Key, mut inValue: Value) -> Result<Arc<BinTree>> {
    let mut outBinTree: Arc<BinTree>;
    outBinTree = 'mc: {
        let __mc_input = inBinTree.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: None, leftSubTree: None, value: None } => {
                    Ok(Arc::new(BinTree { value: Some(TreeValue { key: inKey.clone(), value: inValue.clone() }), leftSubTree: None, rightSubTree: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: right, leftSubTree: left, value: Some(TreeValue { key: rkey, value: _ }) } => {
                    let 0 = (keyCmp(rkey.clone(), inKey.clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(BinTree { value: Some(TreeValue { key: rkey.clone(), value: inValue.clone() }), leftSubTree: left.clone(), rightSubTree: right.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: Some(t), leftSubTree: left, value: optVal @ Some(TreeValue { key: rkey, value: _ }) } => {
                    let mut t_1: Arc<BinTree>;
                    let 1 = (keyCmp(rkey.clone(), inKey.clone())) else { bail!("pattern mismatch") };
                    t_1 = treeAdd(t.clone(), inKey.clone(), inValue.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: left.clone(), rightSubTree: Some(t_1.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: None, leftSubTree: left, value: optVal @ Some(TreeValue { key: rkey, value: _ }) } => {
                    let mut right_1: Arc<BinTree>;
                    let 1 = (keyCmp(rkey.clone(), inKey.clone())) else { bail!("pattern mismatch") };
                    right_1 = treeAdd(Arc::new(BinTree { value: None, leftSubTree: None, rightSubTree: None }), inKey.clone(), inValue.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: left.clone(), rightSubTree: Some(right_1.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: right, leftSubTree: Some(t), value: optVal @ Some(TreeValue { key: rkey, value: _ }) } => {
                    let mut t_1: Arc<BinTree>;
                    let (-1) = (keyCmp(rkey.clone(), inKey.clone())) else { bail!("pattern mismatch") };
                    t_1 = treeAdd(t.clone(), inKey.clone(), inValue.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: Some(t_1.clone()), rightSubTree: right.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: right, leftSubTree: None, value: optVal @ Some(TreeValue { key: rkey, value: _ }) } => {
                    let mut left_1: Arc<BinTree>;
                    let (-1) = (keyCmp(rkey.clone(), inKey.clone())) else { bail!("pattern mismatch") };
                    left_1 = treeAdd(Arc::new(BinTree { value: None, leftSubTree: None, rightSubTree: None }), inKey.clone(), inValue.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: Some(left_1.clone()), rightSubTree: right.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- BinaryTreeInt.treeAdd failed\n")).clone()])?;
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(outBinTree)
}

// protected function treeDelete2 "author: PA
//   This function deletes an entry from the BinTree."
//   input BinTree inBinTree;
//   input Integer inKey;
//   output BinTree outBinTree;
// algorithm
//   outBinTree := matchcontinue (inBinTree,inKey)
//     local
//       BinTree bt,right,left,t;
//       Key key,rkey;
//       TreeValue rightmost;
//       Option<BinTree> optRight,optLeft,optTree;
//       Value rval;
//       Option<TreeValue> optVal;
//       Integer rhash;
//
//     case ((bt as TREENODE(value = NONE(),leftSubTree = NONE(),rightSubTree = NONE())),_)
//       then bt;
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rval)),leftSubTree = optLeft,rightSubTree = SOME(right)),_)
//       equation
//         0 = keyCmp(rkey, inKey);
//         (rightmost,right) = treeDeleteRightmostValue(right);
//         optRight = treePruneEmptyNodes(right);
//       then
//         TREENODE(SOME(rightmost),optLeft,optRight);
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rval)),leftSubTree = SOME(left as TREENODE(value=_)),rightSubTree = NONE()),_)
//       equation
//         0 = keyCmp(rkey, inKey);
//       then
//         left;
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rval)),leftSubTree = NONE(),rightSubTree = NONE()),_)
//       equation
//         0 = keyCmp(rkey, inKey);
//       then
//         TREENODE(NONE(),NONE(),NONE());
//
//     case (TREENODE(value = optVal as SOME(TREEVALUE(rkey,rval)),leftSubTree = optLeft,rightSubTree = SOME(t)),_)
//       equation
//         1 = keyCmp(rkey, inKey);
//         t = treeDelete2(t, inKey);
//         optTree = treePruneEmptyNodes(t);
//       then
//         TREENODE(optVal,optLeft,optTree);
//
//     case (TREENODE(value = optVal as SOME(TREEVALUE(rkey,rval)),leftSubTree =  SOME(t),rightSubTree = optRight),_)
//       equation
//         -1 = keyCmp(rkey, inKey);
//         t = treeDelete2(t, inKey);
//         optTree = treePruneEmptyNodes(t);
//       then
//         TREENODE(optVal,optTree,optRight);
//
//     else
//       equation
//         Error.addMessage(Error.INTERNAL_ERROR,{"-BinaryTree.treeDelete failed\n"});
//       then
//         fail();
//   end matchcontinue;
// end treeDelete2;
// protected function treeDeleteRightmostValue "author: PA
//   This function takes a BinTree and deletes the rightmost value of the tree.
//   Tt returns this value and the updated BinTree. This function is used in
//   the binary tree deletion function \'tree_delete\'.
//   inputs:  (BinTree)
//   outputs: (TreeValue, /* deleted value */
//               BinTree    /* updated bintree */)
// "
//   input BinTree inBinTree;
//   output TreeValue outTreeValue;
//   output BinTree outBinTree;
// algorithm
//   (outTreeValue,outBinTree) := matchcontinue (inBinTree)
//     local
//       TreeValue treeVal,value;
//       BinTree left,right,bt;
//       Option<BinTree> optRight, optLeft;
//       Option<TreeValue> optTreeVal;
//
//     case (TREENODE(value = SOME(treeVal),leftSubTree = NONE(),rightSubTree = NONE()))
//       then (treeVal,TREENODE(NONE(),NONE(),NONE()));
//
//     case (TREENODE(value = SOME(treeVal),leftSubTree = SOME(left),rightSubTree = NONE()))
//       then (treeVal,left);
//
//     case (TREENODE(value = optTreeVal,leftSubTree = optLeft,rightSubTree = SOME(right)))
//       equation
//         (value,right) = treeDeleteRightmostValue(right);
//         optRight = treePruneEmptyNodes(right);
//       then
//         (value,TREENODE(optTreeVal,optLeft,optRight));
//
//     case (TREENODE(value = SOME(treeVal),leftSubTree = NONE(),rightSubTree = SOME(right)))
//       equation
//         failure((_,_) = treeDeleteRightmostValue(right));
//         print("- BinaryTree.treeDeleteRightmostValue: right value was empty, left NONE\n");
//       then
//         (treeVal,TREENODE(NONE(),NONE(),NONE()));
//
//     else
//       equation
//         Error.addMessage(Error.INTERNAL_ERROR,{"- BinaryTree.treeDeleteRightmostValue failed\n"});
//       then
//         fail();
//   end matchcontinue;
// end treeDeleteRightmostValue;
// protected function treePruneEmptyNodes "author: PA
//   This function is a helper function to tree_delete
//   It is used to delete empty nodes of the BinTree
//   representation, that might be introduced when deleting nodes."
//   input BinTree inBinTree;
//   output Option<BinTree> outBinTreeOption;
// algorithm
//   outBinTreeOption := matchcontinue (inBinTree)
//     local BinTree bt;
//     case TREENODE(value = NONE(),leftSubTree = NONE(),rightSubTree = NONE()) then NONE();
//     case bt then SOME(bt);
//   end matchcontinue;
// end treePruneEmptyNodes;
// protected function bintreeDepth "author: PA
//   This function calculates the depth of the Binary Tree given
//   as input. It can be used for debugging purposes to investigate
//   how balanced binary trees are."
//   input BinTree inBinTree;
//   output Integer outInteger;
// algorithm
//   outInteger := matchcontinue (inBinTree)
//     local
//       Value ld,rd,res;
//       BinTree left,right;
//
//     case (TREENODE(leftSubTree = NONE(),rightSubTree = NONE())) then 1;
//
//     case (TREENODE(leftSubTree = SOME(left),rightSubTree = SOME(right)))
//       equation
//         ld = bintreeDepth(left);
//         rd = bintreeDepth(right);
//         res = intMax(ld, rd);
//       then
//         res + 1;
//
//     case (TREENODE(leftSubTree = SOME(left),rightSubTree = NONE()))
//       equation
//         ld = bintreeDepth(left);
//       then
//         ld;
//
//     case (TREENODE(leftSubTree = NONE(),rightSubTree = SOME(right)))
//       equation
//         rd = bintreeDepth(right);
//       then
//         rd;
//   end matchcontinue;
// end bintreeDepth;
pub fn bintreeToList(mut inBinTree: Arc<BinTree>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outKeyLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outValueLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outKeyLst, outValueLst) = 'mc: {
        let __mc_input = inBinTree.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                bt => {
                    let mut klst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (klst, vlst) = bintreeToList2(bt.clone(), metamodelica::nil(), metamodelica::nil())?;
                    Ok((klst.clone(), vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    println!("{}", (literal!("- BackendDAEUtil.bintreeToList failed\n")).clone());
                    Ok(bail!("fail"))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outKeyLst, outValueLst))
}

fn bintreeToList2(mut inBinTree: Arc<BinTree>, mut inKeyLst: Arc<metamodelica::List<i32>>, mut inValueLst: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outKeyLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outValueLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outKeyLst, outValueLst) = 'mc: {
        let __mc_input = (inBinTree.clone(), inKeyLst.clone(), inValueLst.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: None, leftSubTree: None, value: None }, klst, vlst) => {
                    Ok((klst.clone(), vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: right, leftSubTree: left, value: Some(TreeValue { value, key }) }, klst, vlst) => {
                    let mut klst = (*klst).clone();
                    let mut vlst = (*vlst).clone();
                    (klst, vlst) = bintreeToListOpt(left.clone(), klst.clone(), vlst.clone())?;
                    (klst, vlst) = bintreeToListOpt(right.clone(), klst.clone(), vlst.clone())?;
                    Ok((cons(key.clone(), klst.clone()), cons(value.clone(), vlst.clone())))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { leftSubTree: left, value: None, .. }, klst, vlst) => {
                    let mut klst = (*klst).clone();
                    let mut vlst = (*vlst).clone();
                    (klst, vlst) = bintreeToListOpt(left.clone(), klst.clone(), vlst.clone())?;
                    (klst, vlst) = bintreeToListOpt(left.clone(), klst.clone(), vlst.clone())?;
                    Ok((klst.clone(), vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok((outKeyLst, outValueLst))
}

fn bintreeToListOpt(mut inBinTreeOption: Option<Arc<BinTree>>, mut inKeyLst: Arc<metamodelica::List<i32>>, mut inValueLst: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<i32>>, Arc<metamodelica::List<i32>>)> {
    let mut outKeyLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut outValueLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outKeyLst, outValueLst) = (::match_deref::match_deref! { match &((inBinTreeOption.clone(), inKeyLst.clone(), inValueLst.clone())) {
        (None, klst, vlst) => {
            (klst.clone(), vlst.clone())
        },
        (Some(bt), klst, vlst) => {
            let mut klst = (*klst).clone();
            let mut vlst = (*vlst).clone();
            (klst, vlst) = bintreeToList2(bt.clone(), klst.clone(), vlst.clone())?;
            (klst.clone(), vlst.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok((outKeyLst, outValueLst))
}

