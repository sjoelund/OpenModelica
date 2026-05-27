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
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_util::BaseAvlSet;

pub type Key = Arc<Absyn::Path>;

pub fn keyStr(mut inKey: Key) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = (AbsynUtil::pathString(inKey.clone(), (literal!(".")).clone(), true, false)?).clone();
    Ok(outString)
}

pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> Result<i32> {
    let mut outResult: i32 = 0;
    outResult = AbsynUtil::pathCompare(inKey1.clone(), inKey2.clone())?;
    Ok(outResult)
}

/// The binary tree data structure.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tree {
    NODE {
        /// The key of the node.
        key: Key,
        /// Height of tree, used for balancing
        height: i32,
        /// Left subtree.
        left: Arc<Tree>,
        /// Right subtree.
        right: Arc<Tree>,
    },
    LEAF {
        /// The key of the node.
        key: Key,
    },
    EMPTY,
}
pub use self::Tree::{NODE,LEAF,EMPTY};

pub type ValueNode = Arc<Absyn::Path>;

pub fn add(mut inTree: Arc<Tree>, mut inKey: Key) -> Result<Arc<Tree>> {
    let mut tree: Arc<Tree> = inTree.clone();
    tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: inKey.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut key_comp: i32 = 0;
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), inKey.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), inKey.clone())?);
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { key } => {
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare(inKey.clone(), key.clone())?;
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: inKey.clone() }), right: Arc::new(crate::AvlSetPath::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: var_field!((*tree).key, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::AvlSetPath::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: inKey.clone() }) });
            } else {
                outTree = tree.clone();
            }
            outTree.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(tree)
}

pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Result<Arc<Tree>> {
    let mut tree: Arc<Tree> = tree;
    for mut key in &*inValues.clone() {
        let mut key = key.clone();
        tree = add(tree.clone(), key.clone())?;
    }
    Ok(tree)
}

fn balance(mut inTree: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut outTree: Arc<Tree> = inTree.clone();
    outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::LEAF { .. } => {
            inTree.clone()
        },
        Deref @ Tree::NODE { .. } => {
            let mut lh: i32 = 0;
            let mut rh: i32 = 0;
            let mut diff: i32 = 0;
            let mut balanced_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
            lh = height(var_field!((*outTree).left, Tree::NODE).clone());
            rh = height(var_field!((*outTree).right, Tree::NODE).clone());
            diff = lh.clone() - rh.clone();
            if diff.clone() < -1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).right, Tree::NODE).clone()) > 0) {rotateLeft(setTreeLeftRight(outTree.clone(), var_field!((*outTree).left, Tree::NODE).clone(), rotateRight(var_field!((*outTree).right, Tree::NODE).clone())?)?)?} else {rotateLeft(outTree.clone())?};
            } else if diff.clone() > 1 {
                balanced_tree = if (calculateBalance(var_field!((*outTree).left, Tree::NODE).clone()) < 0) {rotateRight(setTreeLeftRight(outTree.clone(), rotateLeft(var_field!((*outTree).left, Tree::NODE).clone())?, var_field!((*outTree).right, Tree::NODE).clone())?)?} else {rotateRight(outTree.clone())?};
            } else if var_field!((*outTree).height, Tree::NODE).clone() != std::cmp::max(lh.clone(), rh.clone()) + 1 {
                assign_variant_field!(outTree => Tree::NODE; height = std::cmp::max(lh.clone(), rh.clone()) + 1);
                balanced_tree = outTree.clone();
            } else {
                balanced_tree = outTree.clone();
            }
            balanced_tree.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(outTree)
}

fn calculateBalance(mut inNode: Arc<Tree>) -> i32 {
    let mut outBalance: i32 = 0;
    outBalance = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => height(var_field!((*inNode).left, Tree::NODE).clone()) - height(var_field!((*inNode).right, Tree::NODE).clone()),
        Deref @ Tree::LEAF { .. } => 0,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBalance
}

pub fn hasKey(mut inTree: Arc<Tree>, mut inKey: Key) -> Result<bool> {
    let mut comp: bool = false;
    let mut key: Key;
    let mut key_comp: i32 = 0;
    let mut tree: Arc<Tree> = Arc::new(Tree::EMPTY);
    key = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inTree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*inTree).key, Tree::LEAF).clone(),
        Deref @ Tree::EMPTY { .. } => {
            return Ok(comp);
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } });
    key_comp = keyCompare(inKey.clone(), key.clone())?;
    comp = (::match_deref::match_deref! { match &((key_comp.clone(), inTree.clone())) {
        (0, _) => true,
        (1, Deref @ Tree::NODE { right: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        ((-1), Deref @ Tree::NODE { left: tree, .. }) => hasKey(tree.clone(), inKey.clone())?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(comp)
}

fn height(mut inNode: Arc<Tree>) -> i32 {
    let mut outHeight: i32 = 0;
    outHeight = (::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inNode).height, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => 1,
        _ => 0,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outHeight
}

pub fn intersection(mut tree1: Arc<Tree>, mut tree2: Arc<Tree>) -> Result<(Arc<Tree>, Arc<Tree>, Arc<Tree>)> {
    let mut intersect: Arc<Tree> = Arc::new(crate::AvlSetPath::Tree::EMPTY);
    let mut rest1: Arc<Tree> = Arc::new(crate::AvlSetPath::Tree::EMPTY);
    let mut rest2: Arc<Tree> = Arc::new(crate::AvlSetPath::Tree::EMPTY);
    let mut keylist1: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut keylist2: Arc<metamodelica::List<Arc<Absyn::Path>>> = metamodelica::nil();
    let mut k1: Key;
    let mut k2: Key;
    let mut key_comp: i32 = 0;
    if isEmpty(tree1.clone()) {
        rest2 = tree2.clone();
        return Ok((intersect, rest1, rest2));
    }
    if isEmpty(tree2.clone()) {
        rest1 = tree1.clone();
        return Ok((intersect, rest1, rest2));
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(listKeys(tree1.clone(), metamodelica::nil())) {
        Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => bail!("pattern mismatch"),
    } };
    k1 = __pa0.clone();
    keylist1 = __pa1.clone();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(listKeys(tree2.clone(), metamodelica::nil())) {
        Deref @ metamodelica::List::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => bail!("pattern mismatch"),
    } };
    k2 = __pa2.clone();
    keylist2 = __pa3.clone();
    loop {
        key_comp = keyCompare(k1.clone(), k2.clone())?;
        if key_comp.clone() > 0 {
            if true /* isPresent not implemented in Rust */ {
                rest2 = add(rest2.clone(), k2.clone())?;
            }
            if keylist2.clone().is_empty() {
                break;
            }
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(keylist2.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => bail!("pattern mismatch"),
            } };
            k2 = __pa4.clone();
            keylist2 = __pa5.clone();
        } else if key_comp.clone() < 0 {
            if true /* isPresent not implemented in Rust */ {
                rest1 = add(rest1.clone(), k1.clone())?;
            }
            if keylist1.clone().is_empty() {
                break;
            }
            let (__pa6, __pa7) = ::match_deref::match_deref! { match &(keylist1.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
                _ => bail!("pattern mismatch"),
            } };
            k1 = __pa6.clone();
            keylist1 = __pa7.clone();
        } else {
            intersect = add(intersect.clone(), k1.clone())?;
            if keylist1.clone().is_empty() || keylist2.clone().is_empty() {
                break;
            }
            let (__pa8, __pa9) = ::match_deref::match_deref! { match &(keylist1.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa8, tail: __pa9 } => (__pa8.clone(), __pa9.clone()),
                _ => bail!("pattern mismatch"),
            } };
            k1 = __pa8.clone();
            keylist1 = __pa9.clone();
            let (__pa10, __pa11) = ::match_deref::match_deref! { match &(keylist2.clone()) {
                Deref @ metamodelica::List::Cons { head: __pa10, tail: __pa11 } => (__pa10.clone(), __pa11.clone()),
                _ => bail!("pattern mismatch"),
            } };
            k2 = __pa10.clone();
            keylist2 = __pa11.clone();
        }
    }
    if true /* isPresent not implemented in Rust */ && !(keylist1.clone().is_empty()) {
        for mut key in &*keylist1.clone() {
            let mut key = key.clone();
            rest1 = add(rest1.clone(), key.clone())?;
        }
    }
    if true /* isPresent not implemented in Rust */ && !(keylist2.clone().is_empty()) {
        for mut key in &*keylist2.clone() {
            let mut key = key.clone();
            rest2 = add(rest2.clone(), key.clone())?;
        }
    }
    Ok((intersect, rest1, rest2))
}

pub fn isEmpty(mut tree: Arc<Tree>) -> bool {
    let mut isEmpty: bool = false;
    isEmpty = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isEmpty
}

pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut tree: Arc<Tree> = tree;
    tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree.clone(),
        Deref @ Tree::NODE { .. } => {
            tree = add(tree.clone(), var_field!((*treeToJoin).key, Tree::NODE).clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone())?;
            tree.clone()
        },
        Deref @ Tree::LEAF { .. } => add(tree.clone(), var_field!((*treeToJoin).key, Tree::LEAF).clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(tree)
}

pub fn listKeys(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
    lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::LEAF { .. } => cons(var_field!((*inTree).key, Tree::LEAF).clone(), lst.clone()),
        Deref @ Tree::NODE { .. } => {
            lst = listKeys(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(var_field!((*inTree).key, Tree::NODE).clone(), lst.clone());
            lst = listKeys(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>>) -> Arc<metamodelica::List<Arc<Absyn::Path>>> {
    let mut lst: Arc<metamodelica::List<Arc<Absyn::Path>>> = lst;
    lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::LEAF { .. } => cons(var_field!((*inTree).key, Tree::LEAF).clone(), lst.clone()),
        Deref @ Tree::NODE { .. } => {
            lst = listKeysReverse(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst = cons(var_field!((*inTree).key, Tree::NODE).clone(), lst.clone());
            lst = listKeysReverse(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    lst
}

pub fn new() -> Arc<Tree> {
    let mut outTree: Arc<Tree> = Arc::new(crate::AvlSetPath::Tree::EMPTY);
    outTree
}

pub fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => keyStr(var_field!((*inNode).key, Tree::NODE).clone())?,
        Deref @ Tree::LEAF { .. } => keyStr(var_field!((*inNode).key, Tree::LEAF).clone())?,
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

pub fn printTreeStr(mut inTree: Arc<Tree>) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut left: Arc<Tree> = Arc::new(Tree::EMPTY);
    let mut right: Arc<Tree> = Arc::new(Tree::EMPTY);
    outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::EMPTY { .. } => literal!("EMPTY()"),
        Deref @ Tree::LEAF { .. } => printNodeStr(inTree.clone())?,
        Deref @ Tree::NODE { right, left, .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(left.clone(), true, (literal!("")).clone())?); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(right.clone(), false, (literal!("")).clone())?); ArcStr::from(__mm_s) },
        _ => bail!("match: no arm matched"),
    } })).clone();
    Ok(outString)
}

fn printTreeStr2(mut inTree: Arc<Tree>, mut isLeft: bool, mut inIndent: ArcStr) -> Result<ArcStr> {
    let mut outString: ArcStr = arcstr::literal!("");
    let mut val_node: Option<Arc<Absyn::Path>> = None;
    let mut left: Option<Arc<Tree>> = None;
    let mut right: Option<Arc<Tree>> = None;
    let mut left_str: ArcStr = arcstr::literal!("");
    let mut right_str: ArcStr = arcstr::literal!("");
    outString = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).left, Tree::NODE).clone(), true, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!("     ")} else {literal!(" │   ")}); ArcStr::from(__mm_s) }).clone())?); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*printTreeStr2(var_field!((*inTree).right, Tree::NODE).clone(), false, ({ let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" │   ")} else {literal!("     ")}); ArcStr::from(__mm_s) }).clone())?); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*inIndent.clone()); __mm_s.push_str(&*if (isLeft.clone()) {literal!(" ┌")} else {literal!(" └")}); __mm_s.push_str(&*literal!("────")); __mm_s.push_str(&*printNodeStr(inTree.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) },
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
    Ok(outString)
}

fn referenceEqOrEmpty(mut t1: Arc<Tree>, mut t2: Arc<Tree>) -> bool {
    let mut b: bool = false;
    b = (::match_deref::match_deref! { match &((t1.clone(), t2.clone())) {
        (Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => true,
        _ => referenceEq(&t1.clone(),&t2.clone()),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn rotateLeft(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut outNode: Arc<Tree> = inNode.clone();
    outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { right: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), var_field!((**child).left, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), node.clone(), var_field!((**child).right, Tree::NODE).clone())?
        },
        Deref @ Tree::NODE { right: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::AvlSetPath::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::AvlSetPath::Tree::EMPTY))?
        },
        _ => {
            inNode.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

fn rotateRight(mut inNode: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut outNode: Arc<Tree> = inNode.clone();
    outNode = (::match_deref::match_deref! { match &(outNode.clone()) {
        Deref @ Tree::NODE { left: child @ Deref @ Tree::NODE { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), var_field!((**child).right, Tree::NODE).clone(), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), var_field!((**child).left, Tree::NODE).clone(), node.clone())?
        },
        Deref @ Tree::NODE { left: child @ Deref @ Tree::LEAF { .. }, .. } => {
            let mut node: Arc<Tree> = Arc::new(Tree::EMPTY);
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::AvlSetPath::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::AvlSetPath::Tree::EMPTY), node.clone())?
        },
        _ => {
            inNode.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outNode)
}

pub fn setTreeLeftRight(mut orig: Arc<Tree>, mut left: Arc<Tree>, mut right: Arc<Tree>) -> Result<Arc<Tree>> {
    let mut res: Arc<Tree> = Arc::new(Tree::EMPTY);
    res = (::match_deref::match_deref! { match &((orig.clone(), left.clone(), right.clone())) {
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => Arc::new(Tree::LEAF { key: var_field!((*orig).key, Tree::NODE).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig.clone(),
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig.clone()} else {Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::NODE).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() })},
        (Deref @ Tree::LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: var_field!((*orig).key, Tree::LEAF).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() }),
        _ => bail!("match: no arm matched"),
    } });
    Ok(res)
}

// NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
// and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
pub fn smallestKey(mut tree: Arc<Tree>) -> Result<Key> {
    let mut key: Key;
    key = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::NODE { .. } => smallestKey(var_field!((*tree).right, Tree::NODE).clone())?,
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } });
    Ok(key)
}

