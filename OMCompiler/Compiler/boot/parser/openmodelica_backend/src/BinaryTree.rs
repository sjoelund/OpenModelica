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

use openmodelica_frontend::ComponentReference;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

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

impl Default for BinTree {
    fn default() -> Self {
        Self {
            value: Default::default(),
            leftSubTree: Default::default(),
            rightSubTree: Default::default(),
        }
    }
}

pub type TREENODE = BinTree;


/// Each node in the binary tree can have a value associated with it.
///  - Tree Value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TreeValue {
    /// Key
    pub key: Key,
    pub r#str: ArcStr,
    pub hash: i32,
    /// Value
    pub value: Value,
}

impl Default for TreeValue {
    fn default() -> Self {
        Self {
            key: Default::default(),
            r#str: Default::default(),
            hash: Default::default(),
            value: Default::default(),
        }
    }
}

pub type TREEVALUE = TreeValue;


/// A key is a Component Reference
pub type Key = Arc<DAE::ComponentRef>;

/// - Value
pub type Value = i32;

thread_local! { static __emptyBinTree_TLS: Arc<BinTree> = Arc::new(BinTree { value: None, leftSubTree: None, rightSubTree: None }); }
pub fn emptyBinTree() -> Arc<BinTree> { __emptyBinTree_TLS.with(|__t| __t.clone()) }

/* *************************
  implementation
 **************************/
fn keyCompareNinjaSecretHashTricks(mut lstr: ArcStr, mut lhash: i32, mut rstr: ArcStr, mut rhash: i32) -> i32 {
    let mut cmp: i32 = 0;
    cmp = Util::intSign(lhash.clone() - rhash.clone());
    cmp = if (cmp.clone() == 0) {stringCompare((lstr.clone()).clone(), (rstr.clone()).clone())} else {cmp.clone()};
    cmp
}

pub fn treeGet(mut bt: Arc<BinTree>, mut key: Key) -> Result<Value> {
    let mut v: Value = 0;
    let mut keystr: ArcStr = arcstr::literal!("");
    let mut keyhash: i32 = 0;
    keystr = (ComponentReferenceBasics::printComponentRefStr(key.clone())?).clone();
    keyhash = stringHashDjb2Mod((keystr.clone()).clone(), BaseHashTable::hugeBucketSize.clone());
    v = treeGet3(bt.clone(), (keystr.clone()).clone(), keyhash.clone(), treeGet2(bt.clone(), (keystr.clone()).clone(), keyhash.clone())?)?;
    Ok(v)
}

fn treeGet2(mut inBinTree: Arc<BinTree>, mut keystr: ArcStr, mut keyhash: i32) -> Result<i32> {
    let mut compResult: i32 = 0;
    compResult = (::match_deref::match_deref! { match &(inBinTree.clone()) {
        Deref @ BinTree { value: Some(TreeValue { hash: rkeyhash, r#str: rkeystr, .. }), .. } => {
            keyCompareNinjaSecretHashTricks((rkeystr.clone()).clone(), rkeyhash.clone(), (keystr.clone()).clone(), keyhash.clone())
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(compResult)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
fn treeGet3(mut inBinTree: Arc<BinTree>, mut keystr: ArcStr, mut keyhash: i32, mut inCompResult: i32) -> Result<Value> {
    let mut outValue: Value = 0;
    outValue = (::match_deref::match_deref! { match &((inBinTree.clone(), inCompResult.clone())) {
        (Deref @ BinTree { value: Some(TreeValue { value: rval, .. }), .. }, 0) => {
            rval.clone()
        },
        (Deref @ BinTree { rightSubTree: Some(right), .. }, 1) => {
            let mut compResult: i32 = 0;
            compResult = treeGet2(right.clone(), (keystr.clone()).clone(), keyhash.clone())?;
            treeGet3(right.clone(), (keystr.clone()).clone(), keyhash.clone(), compResult.clone())?
        },
        (Deref @ BinTree { leftSubTree: Some(left), .. }, (-1)) => {
            let mut compResult: i32 = 0;
            compResult = treeGet2(left.clone(), (keystr.clone()).clone(), keyhash.clone())?;
            treeGet3(left.clone(), (keystr.clone()).clone(), keyhash.clone(), compResult.clone())?
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outValue)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn treeAddList(mut inBinTree: Arc<BinTree>, mut inKeyLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>) -> Result<Arc<BinTree>> {
    let mut outBinTree: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
    outBinTree = (::match_deref::match_deref! { match &((inBinTree.clone(), inKeyLst.clone())) {
        (bt, Deref @ metamodelica::List::Nil) => {
            bt.clone()
        },
        (bt, Deref @ metamodelica::List::Cons { head: key, tail: res }) => {
            let mut bt_1: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
            let mut bt_2: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
            bt_1 = treeAdd(bt.clone(), key.clone(), 0)?;
            bt_2 = treeAddList(bt_1.clone(), res.clone())?;
            bt_2.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outBinTree)
}

pub fn treeAdd(mut inBinTree: Arc<BinTree>, mut inKey: Key, mut inValue: Value) -> Result<Arc<BinTree>> {
    let mut outBinTree: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
    let mut r#str: ArcStr = arcstr::literal!("");
    r#str = (ComponentReferenceBasics::printComponentRefStr(inKey.clone())?).clone();
    outBinTree = treeAdd2(inBinTree.clone(), inKey.clone(), stringHashDjb2Mod((r#str.clone()).clone(), BaseHashTable::hugeBucketSize.clone()), (r#str.clone()).clone(), inValue.clone())?;
    Ok(outBinTree)
}

fn treeAdd2(mut inBinTree: Arc<BinTree>, mut inKey: Key, mut keyhash: i32, mut keystr: ArcStr, mut inValue: Value) -> Result<Arc<BinTree>> {
    let mut outBinTree: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
    outBinTree = 'mc: {
        let __mc_input = (inBinTree.clone(), inKey.clone(), inValue.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: None, leftSubTree: None, value: None }, key, value) => {
                    Ok(Arc::new(BinTree { value: Some(TreeValue { key: key.clone(), r#str: (keystr.clone()).clone(), hash: keyhash.clone(), value: value.clone() }), leftSubTree: None, rightSubTree: None }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: right, leftSubTree: left, value: Some(TreeValue { key: rkey, r#str: rkeystr, hash: rhash, value: _ }) }, _, value) => {
                    let 0 = (keyCompareNinjaSecretHashTricks((rkeystr.clone()).clone(), rhash.clone(), (keystr.clone()).clone(), keyhash.clone())) else { bail!("pattern mismatch") };
                    Ok(Arc::new(BinTree { value: Some(TreeValue { key: rkey.clone(), r#str: (rkeystr.clone()).clone(), hash: rhash.clone(), value: value.clone() }), leftSubTree: left.clone(), rightSubTree: right.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: Some(t), leftSubTree: left, value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }) }, key, value) => {
                    let mut t_1: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
                    let 1 = (keyCompareNinjaSecretHashTricks((rkeystr.clone()).clone(), rhash.clone(), (keystr.clone()).clone(), keyhash.clone())) else { bail!("pattern mismatch") };
                    t_1 = treeAdd2(t.clone(), key.clone(), keyhash.clone(), (keystr.clone()).clone(), value.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: left.clone(), rightSubTree: Some(t_1.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: None, leftSubTree: left, value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }) }, key, value) => {
                    let mut right_1: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
                    let 1 = (keyCompareNinjaSecretHashTricks((rkeystr.clone()).clone(), rhash.clone(), (keystr.clone()).clone(), keyhash.clone())) else { bail!("pattern mismatch") };
                    right_1 = treeAdd2(Arc::new(BinTree { value: None, leftSubTree: None, rightSubTree: None }), key.clone(), keyhash.clone(), (keystr.clone()).clone(), value.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: left.clone(), rightSubTree: Some(right_1.clone()) }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: right, leftSubTree: Some(t), value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }) }, key, value) => {
                    let mut t_1: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
                    let (-1) = (keyCompareNinjaSecretHashTricks((rkeystr.clone()).clone(), rhash.clone(), (keystr.clone()).clone(), keyhash.clone())) else { bail!("pattern mismatch") };
                    t_1 = treeAdd2(t.clone(), key.clone(), keyhash.clone(), (keystr.clone()).clone(), value.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: Some(t_1.clone()), rightSubTree: right.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BinTree { rightSubTree: right, leftSubTree: None, value: optVal @ Some(TreeValue { key: _, r#str: rkeystr, hash: rhash, value: _ }) }, key, value) => {
                    let mut left_1: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
                    let (-1) = (keyCompareNinjaSecretHashTricks((rkeystr.clone()).clone(), rhash.clone(), (keystr.clone()).clone(), keyhash.clone())) else { bail!("pattern mismatch") };
                    left_1 = treeAdd2(Arc::new(BinTree { value: None, leftSubTree: None, rightSubTree: None }), key.clone(), keyhash.clone(), (keystr.clone()).clone(), value.clone())?;
                    Ok(Arc::new(BinTree { value: optVal.clone(), leftSubTree: Some(left_1.clone()), rightSubTree: right.clone() }))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![(literal!("- BinaryTree.treeAdd2 failed\n")).clone()])?;
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
//   input String keystr;
//   input Integer keyhash;
//   output BinTree outBinTree;
// algorithm
//   outBinTree := matchcontinue (inBinTree,keystr,keyhash)
//     local
//       BinTree bt,right,left,t;
//       DAE.ComponentRef key,rkey;
//       String rkeystr;
//       TreeValue rightmost;
//       Option<BinTree> optRight,optLeft,optTree;
//       Value rval;
//       Option<TreeValue> optVal;
//       Integer rhash;
//
//     case ((bt as TREENODE(value = NONE(),leftSubTree = NONE(),rightSubTree = NONE())),_,_)
//       then bt;
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = optLeft,rightSubTree = SOME(right)),_,_)
//       equation
//         0 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//         (rightmost,right) = treeDeleteRightmostValue(right);
//         optRight = treePruneEmptyNodes(right);
//       then
//         TREENODE(SOME(rightmost),optLeft,optRight);
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = SOME(left as TREENODE(value=_)),rightSubTree = NONE()),_,_)
//       equation
//         0 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//       then
//         left;
//
//     case (TREENODE(value = SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = NONE(),rightSubTree = NONE()),_,_)
//       equation
//         0 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//       then
//         TREENODE(NONE(),NONE(),NONE());
//
//     case (TREENODE(value = optVal as SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree = optLeft,rightSubTree = SOME(t)),_,_)
//       equation
//         1 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//         t = treeDelete2(t, keystr, keyhash);
//         optTree = treePruneEmptyNodes(t);
//       then
//         TREENODE(optVal,optLeft,optTree);
//
//     case (TREENODE(value = optVal as SOME(TREEVALUE(rkey,rkeystr,rhash,rval)),leftSubTree =  SOME(t),rightSubTree = optRight),_,_)
//       equation
//         -1 = keyCompareNinjaSecretHashTricks(rkeystr, rhash, keystr, keyhash);
//         t = treeDelete2(t, keystr, keyhash);
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
pub fn bintreeToList(mut inBinTree: Arc<BinTree>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<i32>>)> {
    let mut outKeyLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outValueLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outKeyLst, outValueLst) = 'mc: {
        let __mc_input = inBinTree.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut klst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (klst, vlst) = bintreeToList2(inBinTree.clone(), metamodelica::nil(), metamodelica::nil())?;
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

fn bintreeToList2(mut inBinTree: Arc<BinTree>, mut inKeyLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inValueLst: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<i32>>)> {
    let mut outKeyLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outValueLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outKeyLst, outValueLst) = 'mc: {
        let __mc_input = inBinTree.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: None, leftSubTree: None, value: None } => {
                    Ok((inKeyLst.clone(), inValueLst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { rightSubTree: right, leftSubTree: left, value: Some(TreeValue { value, key, .. }) } => {
                    let mut klst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (klst, vlst) = bintreeToListOpt(left.clone(), cons(key.clone(), inKeyLst.clone()), cons(value.clone(), inValueLst.clone()))?;
                    (klst, vlst) = bintreeToListOpt(right.clone(), klst.clone(), vlst.clone())?;
                    Ok((klst.clone(), vlst.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BinTree { leftSubTree: left, value: None, .. } => {
                    let mut klst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
                    let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
                    (klst, vlst) = bintreeToListOpt(left.clone(), inKeyLst.clone(), inValueLst.clone())?;
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

fn bintreeToListOpt(mut inBinTreeOption: Option<Arc<BinTree>>, mut inKeyLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, mut inValueLst: Arc<metamodelica::List<i32>>) -> Result<(Arc<metamodelica::List<Arc<DAE::ComponentRef>>>, Arc<metamodelica::List<i32>>)> {
    let mut outKeyLst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    let mut outValueLst: Arc<metamodelica::List<i32>> = metamodelica::nil();
    (outKeyLst, outValueLst) = (::match_deref::match_deref! { match &(inBinTreeOption.clone()) {
        None => {
            (inKeyLst.clone(), inValueLst.clone())
        },
        Some(bt) => {
            let mut klst: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
            let mut vlst: Arc<metamodelica::List<i32>> = metamodelica::nil();
            (klst, vlst) = bintreeToList2(bt.clone(), inKeyLst.clone(), inValueLst.clone())?;
            (klst.clone(), vlst.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outKeyLst, outValueLst))
}

pub fn binTreeintersection(mut bt1: Arc<BinTree>, mut bt2: Arc<BinTree>, mut iBt: Arc<BinTree>) -> Result<Arc<BinTree>> {
    let mut oBt: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
    let mut keys: Arc<metamodelica::List<Arc<DAE::ComponentRef>>> = metamodelica::nil();
    (keys, _) = bintreeToList(bt1.clone())?;
    oBt = List::fold1(keys.clone(), (std::sync::Arc::new(binTreeintersection1) as std::sync::Arc<dyn ::std::ops::Fn(Arc<DAE::ComponentRef>, Arc<BinTree>, Arc<BinTree>) -> Result<Arc<BinTree>> + 'static>), bt2.clone(), iBt.clone());
    Ok(oBt)
}

fn binTreeintersection1(mut key: Arc<DAE::ComponentRef>, mut bt2: Arc<BinTree>, mut iBt: Arc<BinTree>) -> Result<Arc<BinTree>> {
    let mut oBt: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
    oBt = 'mc: {
        let __mc_input = iBt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut bt: Arc<BinTree> = Arc::new(<BinTree as ::std::default::Default>::default());
                    treeGet(bt2.clone(), key.clone())?;
                    bt = treeAdd(iBt.clone(), key.clone(), 0)?;
                    Ok(bt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(iBt.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(oBt)
}

