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

use crate::InteractiveUtil;
use crate::NFApi;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_util::BaseAvlSet;
use openmodelica_util::BaseAvlTree;
use openmodelica_util::ExecStat;
use openmodelica_util::JSON;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathEntry {
    pub tree: Arc<PathTree::Tree>,
    pub shadowed: bool,
}

impl Default for PathEntry {
    fn default() -> Self {
        Self {
            tree: Default::default(),
            shadowed: Default::default(),
        }
    }
}

pub type ENTRY = PathEntry;


pub mod PathTree {
    use super::*;
    pub type Key = ArcStr;

    pub type Value = Arc<PathEntry>;

    pub fn keyStr(mut inKey: Key) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (inKey.clone()).clone();
        outString
    }

    pub fn valueStr(mut inValue: Value) -> ArcStr {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = (literal!("")).clone();
        outString
    }

    pub fn keyCompare(mut inKey1: Key, mut inKey2: Key) -> i32 {
        let mut outResult: i32 = 0;
        outResult = stringCompare((inKey1.clone()).clone(), (inKey2.clone()).clone());
        outResult
    }

    pub type ConflictFunc = std::sync::Arc<dyn ::std::ops::Fn(Value, Value, Key) -> Result<Value> + 'static>;

    /// The binary tree data structure.
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Tree {
        NODE {
            /// The key of the node.
            key: Key,
            value: Value,
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
            value: Value,
        },
        EMPTY,
    }
    impl Default for Tree {
        fn default() -> Self { Self::EMPTY }
    }
    pub use self::Tree::{NODE,LEAF,EMPTY};

    pub type ValueNode = ArcStr;

    pub fn add(mut inTree: Arc<Tree>, mut inKey: Key, mut inValue: Value, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<PathEntry>, Arc<PathEntry>, ArcStr) -> Result<Arc<PathEntry>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = inTree.clone();
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => {
            Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() })
        },
        Deref @ Tree::NODE { key, .. } => {
            let mut value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
            let mut key_comp: i32 = 0;
            key_comp = keyCompare((inKey.clone()).clone(), (key.clone()).clone());
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = add(var_field!((*tree).left, Tree::NODE).clone(), (inKey.clone()).clone(), inValue.clone(), conflictFunc.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = add(var_field!((*tree).right, Tree::NODE).clone(), (inKey.clone()).clone(), inValue.clone(), conflictFunc.clone())?);
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::NODE).clone(), (key.clone()).clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::NODE).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::NODE; value = value.clone());
                }
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            let mut value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
            let mut key_comp: i32 = 0;
            let mut outTree: Arc<Tree> = Arc::new(Tree::EMPTY);
            key_comp = keyCompare((inKey.clone()).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone());
            if key_comp.clone() == -1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }), right: Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                outTree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (inKey.clone()).clone(), value: inValue.clone() }) });
            } else {
                value = conflictFunc(inValue.clone(), var_field!((*tree).value, Tree::LEAF).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone())?;
                if !(referenceEq(&var_field!((*tree).value, Tree::LEAF).clone(),&value.clone())) {
                    assign_variant_field!(tree => Tree::LEAF; value = value.clone());
                }
                outTree = tree.clone();
            }
            if (key_comp.clone() == 0) {outTree.clone()} else {balance(outTree.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub use addConflictFail as addConflictDefault;

    pub fn addConflictFail(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Result<Value> {
        let mut value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
        bail!("fail");
        Ok(value)
    }

    pub fn addConflictKeep(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = oldValue.clone();
        value
    }

    pub fn addConflictReplace(mut newValue: Value, mut oldValue: Value, mut key: Key) -> Value {
        let mut value: Value = newValue.clone();
        value
    }

    pub fn addList(mut tree: Arc<Tree>, mut inValues: Arc<metamodelica::List<(ArcStr, Arc<PathEntry>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<PathEntry>, Arc<PathEntry>, ArcStr) -> Result<Arc<PathEntry>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn addUpdate(mut tree: Arc<Tree>, mut key: Key, mut r#fn: Arc<dyn ::std::ops::Fn(Option<Arc<PathEntry>>) -> Result<Arc<PathEntry>> + 'static>) -> Result<Arc<Tree>> {
        pub type UpdateFn = std::sync::Arc<dyn ::std::ops::Fn(Option<Arc<PathEntry>>) -> Result<Value> + 'static>;

        let mut tree: Arc<Tree> = tree;
        let mut key_comp: i32 = 0;
        let mut new_tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        tree = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::EMPTY { .. } => Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }),
        Deref @ Tree::NODE { .. } => {
            key_comp = keyCompare((key.clone()).clone(), (var_field!((*tree).key, Tree::NODE).clone()).clone());
            if key_comp.clone() == -1 {
                assign_variant_field!(tree => Tree::NODE; left = addUpdate(var_field!((*tree).left, Tree::NODE).clone(), (key.clone()).clone(), r#fn.clone())?);
            } else if key_comp.clone() == 1 {
                assign_variant_field!(tree => Tree::NODE; right = addUpdate(var_field!((*tree).right, Tree::NODE).clone(), (key.clone()).clone(), r#fn.clone())?);
            } else {
                assign_variant_field!(tree => Tree::NODE; value = r#fn(Some(var_field!((*tree).value, Tree::NODE).clone()))?);
            }
            if (key_comp.clone() == 0) {tree.clone()} else {balance(tree.clone())?}
        },
        Deref @ Tree::LEAF { .. } => {
            key_comp = keyCompare((key.clone()).clone(), (var_field!((*tree).key, Tree::LEAF).clone()).clone());
            if key_comp.clone() == -1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }), right: Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY) });
            } else if key_comp.clone() == 1 {
                new_tree = Arc::new(Tree::NODE { key: (var_field!((*tree).key, Tree::LEAF).clone()).clone(), value: var_field!((*tree).value, Tree::LEAF).clone(), height: 2, left: Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY), right: Arc::new(Tree::LEAF { key: (key.clone()).clone(), value: r#fn(None)? }) });
            } else {
                assign_variant_field!(tree => Tree::LEAF; value = r#fn(Some(var_field!((*tree).value, Tree::LEAF).clone()))?);
                new_tree = tree.clone();
            }
            if (key_comp.clone() == 0) {new_tree.clone()} else {balance(new_tree.clone())?}
        },
        _ => bail!("match: no arm matched"),
    } });
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

    pub fn fold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<PathEntry>, FT) -> Result<FT> + 'static>, mut inStartValue: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<FT> + 'static>;

        let mut outResult: FT = inStartValue.clone();
        outResult = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            outResult = fold(var_field!((*inTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult = inFunc((key.clone()).clone(), value.clone(), outResult.clone()).unwrap();
            outResult = fold(var_field!((*inTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            outResult.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            outResult = inFunc((key.clone()).clone(), value.clone(), outResult.clone()).unwrap();
            outResult.clone()
        },
        _ => {
            outResult.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outResult
    }

    pub fn foldCond<FT: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<PathEntry>, FT) -> Result<(FT, bool)> + 'static>, mut value: FT) -> FT {
        pub type FoldFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<(FT, bool)> + 'static>;

        let mut value: FT = value;
        value = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc((var_field!((*tree).key, Tree::NODE).clone()).clone(), var_field!((*tree).value, Tree::NODE).clone(), value.clone()).unwrap();
            if c.clone() {
                value = foldCond(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), value.clone());
                value = foldCond(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), value.clone());
            }
            value.clone()
        },
        Deref @ Tree::LEAF { .. } => {
            let mut c: bool = false;
            (value, c) = foldFunc((var_field!((*tree).key, Tree::LEAF).clone()).clone(), var_field!((*tree).value, Tree::LEAF).clone(), value.clone()).unwrap();
            value.clone()
        },
        _ => {
            value.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value
    }

    pub fn fold_2<FT1: Clone + 'static, FT2: Clone + 'static>(mut tree: Arc<Tree>, mut foldFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<PathEntry>, FT1, FT2) -> Result<(FT1, FT2)> + 'static>, mut foldArg1: FT1, mut foldArg2: FT2) -> (FT1, FT2) {
        pub type FoldFunc<FT1: Clone + 'static, FT2: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT1, FT2) -> Result<(FT1, FT2)> + 'static>;

        let mut foldArg1: FT1 = foldArg1;
        let mut foldArg2: FT2 = foldArg2;
        let () = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).left, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            (foldArg1, foldArg2) = foldFunc((var_field!((*tree).key, Tree::NODE).clone()).clone(), var_field!((*tree).value, Tree::NODE).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            (foldArg1, foldArg2) = fold_2(var_field!((*tree).right, Tree::NODE).clone(), foldFunc.clone(), foldArg1.clone(), foldArg2.clone());
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            (foldArg1, foldArg2) = foldFunc((var_field!((*tree).key, Tree::LEAF).clone()).clone(), var_field!((*tree).value, Tree::LEAF).clone(), foldArg1.clone(), foldArg2.clone()).unwrap();
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (foldArg1, foldArg2)
    }

    pub fn forEach(mut tree: Arc<Tree>, mut func: Arc<dyn ::std::ops::Fn(ArcStr, Arc<PathEntry>) -> Result<()> + 'static>) -> Result<()> {
        pub type EachFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<()> + 'static>;

        let _ = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => {
            forEach(var_field!((*tree).left, Tree::NODE).clone(), func.clone())?;
            func((var_field!((*tree).key, Tree::NODE).clone()).clone(), var_field!((*tree).value, Tree::NODE).clone())?;
            forEach(var_field!((*tree).right, Tree::NODE).clone(), func.clone())?;
            ()
        },
        Deref @ Tree::LEAF { .. } => {
            func((var_field!((*tree).key, Tree::LEAF).clone()).clone(), var_field!((*tree).value, Tree::LEAF).clone())?;
            ()
        },
        Deref @ Tree::EMPTY { .. } => (),
        _ => bail!("match: no arm matched"),
    } });
        Ok(())
    }

    pub fn fromList(mut inValues: Arc<metamodelica::List<(ArcStr, Arc<PathEntry>)>>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<PathEntry>, Arc<PathEntry>, ArcStr) -> Result<Arc<PathEntry>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY);
        let mut key: Key = arcstr::literal!("");
        let mut value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
        for mut t in &*inValues.clone() {
            let mut t = t.clone();
            (key, value) = t.clone();
            tree = add(tree.clone(), (key.clone()).clone(), value.clone(), conflictFunc.clone())?;
        }
        Ok(tree)
    }

    pub fn get(mut tree: Arc<Tree>, mut key: Key) -> Result<Value> {
        let mut value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
        let mut k: Key = arcstr::literal!("");
        k = ((::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
        value = (::match_deref::match_deref! { match &((keyCompare((key.clone()).clone(), (k.clone()).clone()), tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => var_field!((*tree).value, Tree::LEAF).clone(),
        (0, Deref @ Tree::NODE { .. }) => var_field!((*tree).value, Tree::NODE).clone(),
        (1, Deref @ Tree::NODE { .. }) => get(var_field!((*tree).right, Tree::NODE).clone(), (key.clone()).clone())?,
        ((-1), Deref @ Tree::NODE { .. }) => get(var_field!((*tree).left, Tree::NODE).clone(), (key.clone()).clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(value)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn getOpt(mut tree: Arc<Tree>, mut key: Key) -> Option<Arc<PathEntry>> {
        let mut value: Option<Arc<PathEntry>> = None;
        let mut k: Key = arcstr::literal!("");
        k = ((::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => key.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })).clone();
        value = (::match_deref::match_deref! { match &((keyCompare((key.clone()).clone(), (k.clone()).clone()), tree.clone())) {
        (0, Deref @ Tree::LEAF { .. }) => Some(var_field!((*tree).value, Tree::LEAF).clone()),
        (0, Deref @ Tree::NODE { .. }) => Some(var_field!((*tree).value, Tree::NODE).clone()),
        (1, Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).right, Tree::NODE).clone(), (key.clone()).clone()),
        ((-1), Deref @ Tree::NODE { .. }) => getOpt(var_field!((*tree).left, Tree::NODE).clone(), (key.clone()).clone()),
        _ => None,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        value
    }

    pub fn hasKey(mut inTree: Arc<Tree>, mut inKey: Key) -> Result<bool> {
        let mut comp: bool = false;
        let mut key: Key = arcstr::literal!("");
        let mut key_comp: i32 = 0;
        let mut tree: Arc<Tree> = Arc::new(Tree::EMPTY);
        key = ((::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { .. } => var_field!((*inTree).key, Tree::NODE).clone(),
        Deref @ Tree::LEAF { .. } => var_field!((*inTree).key, Tree::LEAF).clone(),
        Deref @ Tree::EMPTY { .. } => {
            return Ok(comp);
            bail!("fail")
        },
        _ => bail!("match: no arm matched"),
    } })).clone();
        key_comp = keyCompare((inKey.clone()).clone(), (key.clone()).clone());
        comp = (::match_deref::match_deref! { match &((key_comp.clone(), inTree.clone())) {
        (0, _) => true,
        (1, Deref @ Tree::NODE { right: tree, .. }) => hasKey(tree.clone(), (inKey.clone()).clone())?,
        ((-1), Deref @ Tree::NODE { left: tree, .. }) => hasKey(tree.clone(), (inKey.clone()).clone())?,
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

    pub fn intersection() -> Result<()> {
        bail!("fail");
        Ok(())
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

    pub fn join(mut tree: Arc<Tree>, mut treeToJoin: Arc<Tree>, mut conflictFunc: Arc<dyn ::std::ops::Fn(Arc<PathEntry>, Arc<PathEntry>, ArcStr) -> Result<Arc<PathEntry>> + 'static>) -> Result<Arc<Tree>> {
        let mut tree: Arc<Tree> = tree;
        tree = (::match_deref::match_deref! { match &(treeToJoin.clone()) {
        Deref @ Tree::EMPTY { .. } => tree.clone(),
        Deref @ Tree::NODE { .. } => {
            tree = add(tree.clone(), (var_field!((*treeToJoin).key, Tree::NODE).clone()).clone(), var_field!((*treeToJoin).value, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).left, Tree::NODE).clone(), conflictFunc.clone())?;
            tree = join(tree.clone(), var_field!((*treeToJoin).right, Tree::NODE).clone(), conflictFunc.clone())?;
            tree.clone()
        },
        Deref @ Tree::LEAF { .. } => add(tree.clone(), (var_field!((*treeToJoin).key, Tree::LEAF).clone()).clone(), var_field!((*treeToJoin).value, Tree::LEAF).clone(), conflictFunc.clone())?,
        _ => bail!("match: no arm matched"),
    } });
        Ok(tree)
    }

    pub fn listKeys(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
        let mut lst: Arc<metamodelica::List<ArcStr>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { key, .. } => {
            lst = listKeys(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons((key.clone()).clone(), lst.clone());
            lst = listKeys(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { key, .. } => {
            cons((key.clone()).clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listKeysReverse(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<ArcStr>>) -> Arc<metamodelica::List<ArcStr>> {
        let mut lst: Arc<metamodelica::List<ArcStr>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::LEAF { .. } => cons((var_field!((*inTree).key, Tree::LEAF).clone()).clone(), lst.clone()),
        Deref @ Tree::NODE { .. } => {
            lst = listKeysReverse(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst = cons((var_field!((*inTree).key, Tree::NODE).clone()).clone(), lst.clone());
            lst = listKeysReverse(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        _ => lst.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn listValues(mut tree: Arc<Tree>, mut lst: Arc<metamodelica::List<Arc<PathEntry>>>) -> Arc<metamodelica::List<Arc<PathEntry>>> {
        let mut lst: Arc<metamodelica::List<Arc<PathEntry>>> = lst;
        lst = (::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { value, .. } => {
            lst = listValues(var_field!((*tree).right, Tree::NODE).clone(), lst.clone());
            lst = cons(value.clone(), lst.clone());
            lst = listValues(var_field!((*tree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { value, .. } => {
            cons(value.clone(), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn map(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<PathEntry>) -> Result<Arc<PathEntry>> + 'static>) -> Arc<Tree> {
        pub type MapFunc = std::sync::Arc<dyn ::std::ops::Fn(Key, Value) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            new_left = map(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone());
            new_value = inFunc((key.clone()).clone(), value.clone()).unwrap();
            new_right = map(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: (key.clone()).clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
            new_value = inFunc((key.clone()).clone(), value.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        outTree
    }

    pub fn mapFold<FT: Clone + 'static>(mut inTree: Arc<Tree>, mut inFunc: Arc<dyn ::std::ops::Fn(ArcStr, Arc<PathEntry>, FT) -> Result<(Arc<PathEntry>, FT)> + 'static>, mut inStartValue: FT) -> (Arc<Tree>, FT) {
        pub type MapFunc<FT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Key, Value, FT) -> Result<Value> + 'static>;

        let mut outTree: Arc<Tree> = inTree.clone();
        let mut outResult: FT = inStartValue.clone();
        outTree = (::match_deref::match_deref! { match &(outTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            let mut new_value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
            let mut new_left: Arc<Tree> = Arc::new(Tree::EMPTY);
            let mut new_right: Arc<Tree> = Arc::new(Tree::EMPTY);
            (new_left, outResult) = mapFold(var_field!((*outTree).left, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            (new_value, outResult) = inFunc((key.clone()).clone(), value.clone(), outResult.clone()).unwrap();
            (new_right, outResult) = mapFold(var_field!((*outTree).right, Tree::NODE).clone(), inFunc.clone(), outResult.clone());
            if !(referenceEq(&new_left.clone(),&var_field!((*outTree).left, Tree::NODE).clone())) || !(referenceEq(&value.clone(),&new_value.clone())) || !(referenceEq(&new_right.clone(),&var_field!((*outTree).right, Tree::NODE).clone())) {
                outTree = Arc::new(Tree::NODE { key: (key.clone()).clone(), value: new_value.clone(), height: var_field!((*outTree).height, Tree::NODE).clone(), left: new_left.clone(), right: new_right.clone() });
            }
            outTree.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            let mut new_value: Value = Arc::new(<PathEntry as ::std::default::Default>::default());
            (new_value, outResult) = inFunc((key.clone()).clone(), value.clone(), outResult.clone()).unwrap();
            if !(referenceEq(&value.clone(),&new_value.clone())) {
                assign_variant_field!(outTree => Tree::LEAF; value = new_value.clone());
            }
            outTree.clone()
        },
        _ => {
            inTree.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        (outTree, outResult)
    }

    pub fn new() -> Arc<Tree> {
        let mut outTree: Arc<Tree> = Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY);
        outTree
    }

    pub fn printNodeStr(mut inNode: Arc<Tree>) -> Result<ArcStr> {
        let mut outString: ArcStr = arcstr::literal!("");
        outString = ((::match_deref::match_deref! { match &(inNode.clone()) {
        Deref @ Tree::NODE { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr((var_field!((*inNode).key, Tree::NODE).clone()).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::NODE).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
        Deref @ Tree::LEAF { .. } => { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*keyStr((var_field!((*inNode).key, Tree::LEAF).clone()).clone())); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*valueStr(var_field!((*inNode).value, Tree::LEAF).clone())); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) },
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
        let mut val_node: Option<ArcStr> = None;
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
            node = setTreeLeftRight(outNode.clone(), var_field!((*outNode).left, Tree::NODE).clone(), Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY))?;
            setTreeLeftRight(child.clone(), node.clone(), Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY))?
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
            node = setTreeLeftRight(outNode.clone(), Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY), var_field!((*outNode).right, Tree::NODE).clone())?;
            setTreeLeftRight(child.clone(), Arc::new(crate::ReverseLookup::PathTree::Tree::EMPTY), node.clone())?
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
        (Deref @ Tree::NODE { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => Arc::new(Tree::LEAF { key: (var_field!((*orig).key, Tree::NODE).clone()).clone(), value: var_field!((*orig).value, Tree::NODE).clone() }),
        (Deref @ Tree::LEAF { .. }, Deref @ Tree::EMPTY { .. }, Deref @ Tree::EMPTY { .. }) => orig.clone(),
        (Deref @ Tree::NODE { .. }, _, _) => if (referenceEqOrEmpty(var_field!((*orig).left, Tree::NODE).clone(), left.clone()) && referenceEqOrEmpty(var_field!((*orig).right, Tree::NODE).clone(), right.clone())) {orig.clone()} else {Arc::new(Tree::NODE { key: (var_field!((*orig).key, Tree::NODE).clone()).clone(), value: var_field!((*orig).value, Tree::NODE).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() })},
        (Deref @ Tree::LEAF { .. }, _, _) => Arc::new(Tree::NODE { key: (var_field!((*orig).key, Tree::LEAF).clone()).clone(), value: var_field!((*orig).value, Tree::LEAF).clone(), height: std::cmp::max(height(left.clone()), height(right.clone())) + 1, left: left.clone(), right: right.clone() }),
        _ => bail!("match: no arm matched"),
    } });
        Ok(res)
    }

    // NOTE: #[tailcall::tailcall] disabled: function body contains a `match_deref!{…}` match,
    // and the tailcall rewriter cannot see arms hidden behind the macro's `Deref @` patterns.
    pub fn smallestKey(mut tree: Arc<Tree>) -> Result<Key> {
        let mut key: Key = arcstr::literal!("");
        key = ((::match_deref::match_deref! { match &(tree.clone()) {
        Deref @ Tree::NODE { right: Deref @ Tree::EMPTY { .. }, .. } => var_field!((*tree).key, Tree::NODE).clone(),
        Deref @ Tree::NODE { .. } => smallestKey(var_field!((*tree).right, Tree::NODE).clone())?,
        Deref @ Tree::LEAF { .. } => var_field!((*tree).key, Tree::LEAF).clone(),
        _ => bail!("match: no arm matched"),
    } })).clone();
        Ok(key)
    }

    pub fn toList(mut inTree: Arc<Tree>, mut lst: Arc<metamodelica::List<(ArcStr, Arc<PathEntry>)>>) -> Arc<metamodelica::List<(ArcStr, Arc<PathEntry>)>> {
        let mut lst: Arc<metamodelica::List<(ArcStr, Arc<PathEntry>)>> = lst;
        lst = (::match_deref::match_deref! { match &(inTree.clone()) {
        Deref @ Tree::NODE { value, key, .. } => {
            lst = toList(var_field!((*inTree).right, Tree::NODE).clone(), lst.clone());
            lst = cons((key.clone(), value.clone()), lst.clone());
            lst = toList(var_field!((*inTree).left, Tree::NODE).clone(), lst.clone());
            lst.clone()
        },
        Deref @ Tree::LEAF { value, key } => {
            cons((key.clone(), value.clone()), lst.clone())
        },
        _ => {
            lst.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
        lst
    }

    pub fn update(mut tree: Arc<Tree>, mut key: Key, mut value: Value) -> Arc<Tree> {
        let mut outTree: Arc<Tree> = add(tree.clone(), (key.clone()).clone(), value.clone(), (std::sync::Arc::new(fnptr!(addConflictReplace, Arc<PathEntry>, Arc<PathEntry>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<PathEntry>, Arc<PathEntry>, ArcStr) -> Result<Arc<PathEntry>> + 'static>)).unwrap();
        outTree
    }

}

pub mod Paths {
    use super::*;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Paths {
        pub tree: Arc<PathTree::Tree>,
        pub relativePath: Arc<metamodelica::List<ArcStr>>,
        pub currentPath: Arc<metamodelica::List<ArcStr>>,
    }

    impl Default for Paths {
        fn default() -> Self {
            Self {
                tree: Default::default(),
                relativePath: Default::default(),
                currentPath: Default::default(),
            }
        }
    }

    pub type PATHS = Paths;

    pub fn currentPathStr(mut paths: Arc<Paths>) -> ArcStr {
        let mut r#str: ArcStr = stringDelimitList(paths.currentPath.clone().reverse(), (literal!(".")).clone());
        r#str
    }

}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Match {
    pub name: Arc<Absyn::ComponentRef>,
    pub scope: ArcStr,
    pub info: SourceInfo,
}

impl Default for Match {
    fn default() -> Self {
        Self {
            name: Default::default(),
            scope: Default::default(),
            info: Default::default(),
        }
    }
}

pub type MATCH = Match;


pub type Matches = Arc<metamodelica::List<Match>>;

pub fn lookup(mut path: Arc<Absyn::Path>, mut scope: Arc<Absyn::Path>, mut program: Absyn::Program, mut exactMatch: bool, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut result: ArcStr = arcstr::literal!("");
    let mut tree: Arc<PathTree::Tree> = Arc::new(PathTree::Tree::EMPTY);
    let mut matches: Matches = metamodelica::nil();
    let mut paths: Arc<Paths::Paths> = Arc::new(<Paths::Paths as ::std::default::Default>::default());
    let mut cls: Arc<Absyn::Class> = Arc::new(<Absyn::Class as ::std::default::Default>::default());
    let mut opt_path: Option<Arc<Absyn::Path>> = None;
    let mut relative_path: Arc<Absyn::Path>;
    let mut grouped_matches: Arc<metamodelica::List<Arc<metamodelica::List<Match>>>> = metamodelica::nil();
    ExecStat::execStatReset()?;
    if AbsynUtil::pathEqual(scope.clone(), Arc::new(Absyn::Path::IDENT { name: (literal!("AllLoadedClasses")).clone() })) {
        tree = addPath(path.clone(), PathTree::new())?;
        paths = Arc::new(Paths::Paths { tree: tree.clone(), relativePath: AbsynUtil::pathToStringList(path.clone())?, currentPath: metamodelica::nil() });
        matches = lookupInProgram(program.clone(), paths.clone(), exactMatch.clone())?;
    } else {
        opt_path = AbsynUtil::pathStripSamePrefix(path.clone(), scope.clone())?;
        relative_path = Util::getOptionOrDefault(opt_path.clone(), path.clone());
        tree = addPath(relative_path.clone(), PathTree::new())?;
        paths = Arc::new(Paths::Paths { tree: tree.clone(), relativePath: AbsynUtil::pathToStringList(relative_path.clone())?, currentPath: metamodelica::nil() });
        match '__try0: {
            cls = unwrap_break_err!(InteractiveUtil::getPathedClassInProgram(scope.clone(), program.clone(), false, false), '__try0);
            matches = unwrap_break_err!(lookupInClass(cls.clone(), paths.clone(), exactMatch.clone(), metamodelica::nil()), '__try0);
            Ok::<_, anyhow::Error>((matches.clone(),))
        } {
            Ok((__try0_o0,)) => {
                matches = __try0_o0;
            }
            Err(_) => {
                matches = metamodelica::nil();
            }
        }
    }
    grouped_matches = groupMatches(matches.clone())?;
    result = (serializeMatches(grouped_matches.clone(), prettyPrint.clone())?).clone();
    ExecStat::execStat(({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("ReverseLookup.lookup(")); __mm_s.push_str(&*AbsynUtil::pathString(path.clone(), (literal!(".")).clone(), true, false)?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) }).clone())?;
    Ok(result)
}

fn addPath(mut path: Arc<Absyn::Path>, mut tree: Arc<PathTree::Tree>) -> Result<Arc<PathTree::Tree>> {
    let mut tree: Arc<PathTree::Tree> = tree;
    let mut opt_entry: Option<Arc<PathEntry>> = None;
    let mut entry: Arc<PathEntry> = Arc::new(<PathEntry as ::std::default::Default>::default());
    let mut opt_tree: Option<Arc<PathTree::Tree>> = None;
    let mut rest_tree: Arc<PathTree::Tree> = Arc::new(PathTree::Tree::EMPTY);
    tree = (::match_deref::match_deref! { match &(path.clone()) {
        Deref @ Absyn::Path::IDENT { .. } => PathTree::add(tree.clone(), (var_field!((*path).name, Absyn::Path::IDENT).clone()).clone(), Arc::new(PathEntry { tree: PathTree::new(), shadowed: false }), (std::sync::Arc::new(fnptr!(PathTree::addConflictKeep, Arc<PathEntry>, Arc<PathEntry>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<PathEntry>, Arc<PathEntry>, ArcStr) -> Result<Arc<PathEntry>> + 'static>))?,
        Deref @ Absyn::Path::QUALIFIED { .. } => {
            opt_entry = PathTree::getOpt(tree.clone(), (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone());
            if isSome(opt_entry.clone()) {
                entry = Util::getOption(opt_entry.clone())?;
                assign_field!(entry.tree = addPath(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), entry.tree.clone())?);
            } else {
                entry = Arc::new(PathEntry { tree: addPath(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), PathTree::new())?, shadowed: false });
            }
            PathTree::add(tree.clone(), (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone(), entry.clone(), (std::sync::Arc::new(fnptr!(PathTree::addConflictReplace, Arc<PathEntry>, Arc<PathEntry>, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(Arc<PathEntry>, Arc<PathEntry>, ArcStr) -> Result<Arc<PathEntry>> + 'static>))?
        },
        Deref @ Absyn::Path::FULLYQUALIFIED { .. } => addPath(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), tree.clone())?,
        _ => bail!("match: no arm matched"),
    } });
    Ok(tree)
}

fn lookupPath(mut path: Arc<Absyn::Path>, mut paths: Arc<PathTree::Tree>, mut exactMatch: bool, mut fullyQualified: bool) -> Result<bool> {
    let mut found: bool = false;
    let mut entry: Arc<PathEntry> = Arc::new(<PathEntry as ::std::default::Default>::default());
    found = 'mc: {
        let __mc_input = path.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::IDENT { .. } => {
                    let mut entry: Arc<PathEntry> = entry.clone();
                    entry = PathTree::get(paths.clone(), (var_field!((*path).name, Absyn::Path::IDENT).clone()).clone())?;
                    Ok((fullyQualified.clone() || !(entry.shadowed.clone())) && PathTree::isEmpty(entry.tree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::QUALIFIED { .. } => {
                    let mut found: bool = found.clone();
                    let mut entry: Arc<PathEntry> = entry.clone();
                    entry = PathTree::get(paths.clone(), (var_field!((*path).name, Absyn::Path::QUALIFIED).clone()).clone())?;
                    if entry.shadowed.clone() && !(fullyQualified.clone()) {
                        found = false;
                    } else if PathTree::isEmpty(entry.tree.clone()) && !(exactMatch.clone()) {
                        found = true;
                    } else {
                        found = lookupPath(var_field!((*path).path, Absyn::Path::QUALIFIED).clone(), entry.tree.clone(), exactMatch.clone(), fullyQualified.clone())?;
                    }
                    Ok(found.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Path::FULLYQUALIFIED { .. } => {
                    Ok(lookupPath(var_field!((*path).path, Absyn::Path::FULLYQUALIFIED).clone(), paths.clone(), exactMatch.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(found)
}

fn matchPath(mut path: Arc<Absyn::Path>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    if lookupPath(path.clone(), paths.tree.clone(), exactMatch.clone(), false)? {
        matches = cons(Match { name: AbsynUtil::pathToCref(path.clone())?, scope: (Paths::currentPathStr(paths.clone())).clone(), info: info.clone() }, matches.clone());
    }
    Ok(matches)
}

fn lookupCref(mut cref: Arc<Absyn::ComponentRef>, mut paths: Arc<PathTree::Tree>, mut exactMatch: bool, mut fullyQualified: bool) -> Result<bool> {
    let mut found: bool = false;
    let mut entry: Arc<PathEntry> = Arc::new(<PathEntry as ::std::default::Default>::default());
    found = 'mc: {
        let __mc_input = cref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => {
                    let mut entry: Arc<PathEntry> = entry.clone();
                    entry = PathTree::get(paths.clone(), (var_field!((*cref).name, Absyn::ComponentRef::CREF_IDENT).clone()).clone())?;
                    Ok((fullyQualified.clone() || !(entry.shadowed.clone())) && PathTree::isEmpty(entry.tree.clone()))
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
                    let mut entry: Arc<PathEntry> = entry.clone();
                    let mut found: bool = found.clone();
                    entry = PathTree::get(paths.clone(), (var_field!((*cref).name, Absyn::ComponentRef::CREF_QUAL).clone()).clone())?;
                    if entry.shadowed.clone() && !(fullyQualified.clone()) {
                        found = false;
                    } else if PathTree::isEmpty(entry.tree.clone()) && !(exactMatch.clone()) {
                        found = true;
                    } else {
                        found = lookupCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), entry.tree.clone(), exactMatch.clone(), fullyQualified.clone())?;
                    }
                    Ok(found.clone())
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => {
                    Ok(lookupCref(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), paths.clone(), exactMatch.clone(), true)?)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => bail!("nomatch"),
            }}
        })() { break 'mc __v; }
        bail!("matchcontinue: no arm matched")
    };
    Ok(found)
}

fn matchCref(mut cref: Arc<Absyn::ComponentRef>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    if lookupCref(cref.clone(), paths.tree.clone(), exactMatch.clone(), false)? {
        matches = cons(Match { name: cref.clone(), scope: (Paths::currentPathStr(paths.clone())).clone(), info: info.clone() }, matches.clone());
    }
    Ok(matches)
}

fn shadowLocalNames(mut cls: Arc<Absyn::Class>, mut paths: Arc<Paths::Paths>) -> Result<Arc<Paths::Paths>> {
    let mut paths: Arc<Paths::Paths> = paths;
    for mut part in &*AbsynUtil::getClassPartsInClass(cls.clone()) {
        let mut part = part.clone();
        for mut item in &*AbsynUtil::getElementItemsInClass(cls.clone()) {
            let mut item = item.clone();
            paths = shadowLocalNamesInElementItem(item.clone(), paths.clone())?;
        }
    }
    Ok(paths)
}

fn shadowLocalNamesInElementItem(mut item: Arc<Absyn::ElementItem>, mut paths: Arc<Paths::Paths>) -> Result<Arc<Paths::Paths>> {
    let mut paths: Arc<Paths::Paths> = paths;
    let mut spec: Arc<Absyn::ElementSpec>;
    paths = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { element: Deref @ Absyn::Element::ELEMENT { specification: spec, .. } } => shadowLocalNamesInElementSpec(spec.clone(), paths.clone())?,
        _ => paths.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(paths)
}

fn shadowLocalNamesInElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut paths: Arc<Paths::Paths>) -> Result<Arc<Paths::Paths>> {
    let mut paths: Arc<Paths::Paths> = paths;
    paths = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => shadowLocalName((AbsynUtil::className(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone())?).clone(), paths.clone())?,
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            for mut comp in &*var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let mut comp = comp.clone();
                paths = shadowLocalName((AbsynUtil::componentName(comp.clone())?).clone(), paths.clone())?;
            }
            paths.clone()
        },
        _ => paths.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(paths)
}

fn shadowLocalName(mut name: ArcStr, mut paths: Arc<Paths::Paths>) -> Result<Arc<Paths::Paths>> {
    let mut paths: Arc<Paths::Paths> = paths;
    let mut entry: Arc<PathEntry> = Arc::new(<PathEntry as ::std::default::Default>::default());
    if PathTree::hasKey(paths.tree.clone(), (name.clone()).clone())? {
        entry = PathTree::get(paths.tree.clone(), (name.clone()).clone())?;
        if !(entry.shadowed.clone()) {
            assign_field!(entry.shadowed = true);
            assign_field!(paths.tree = PathTree::update(paths.tree.clone(), (name.clone()).clone(), entry.clone()));
        }
    }
    Ok(paths)
}

fn lookupInProgram(mut program: Absyn::Program, mut paths: Arc<Paths::Paths>, mut exactMatch: bool) -> Result<Matches> {
    let mut matches: Matches = metamodelica::nil();
    for mut cls in &*program.classes.clone() {
        let mut cls = cls.clone();
        matches = lookupInClass(cls.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInClass(mut cls: Arc<Absyn::Class>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    let mut relative_path: Arc<metamodelica::List<ArcStr>> = paths.relativePath.clone();
    let mut local_paths: Arc<Paths::Paths> = Arc::new(<Paths::Paths as ::std::default::Default>::default());
    local_paths = shadowLocalNames(cls.clone(), paths.clone())?;
    if !(relative_path.clone().is_empty()) && cls.name.clone() == listHead(relative_path.clone())? {
        relative_path = listRest(relative_path.clone())?;
        assign_field!(local_paths.relativePath = relative_path.clone());
        if !(relative_path.clone().is_empty()) {
            assign_field!(local_paths.tree = addPath(AbsynUtil::stringListPath(relative_path.clone()), local_paths.tree.clone())?);
        }
    }
    matches = lookupInClassDef(cls.body.clone(), (cls.name.clone()).clone(), local_paths.clone(), exactMatch.clone(), cls.info.clone(), matches.clone())?;
    Ok(matches)
}

fn lookupInClassDef(mut cdef: Arc<Absyn::ClassDef>, mut name: ArcStr, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    let mut local_paths: Arc<Paths::Paths> = paths.clone();
    matches = (::match_deref::match_deref! { match &(cdef.clone()) {
        Deref @ Absyn::ClassDef::PARTS { .. } => {
            assign_field!(local_paths.currentPath = cons((name.clone()).clone(), local_paths.currentPath.clone()));
            for mut part in &*var_field!((*cdef).classParts, Absyn::ClassDef::PARTS).clone() {
                let mut part = part.clone();
                matches = lookupInClassPart(part.clone(), local_paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            for mut ann in &*var_field!((*cdef).ann, Absyn::ClassDef::PARTS).clone() {
                let mut ann = ann.clone();
                matches = lookupInAnnotation(ann.clone(), local_paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassDef::DERIVED { .. } => {
            matches = lookupInTypeSpec(var_field!((*cdef).typeSpec, Absyn::ClassDef::DERIVED).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            for mut arg in &*var_field!((*cdef).arguments, Absyn::ClassDef::DERIVED).clone() {
                let mut arg = arg.clone();
                matches = lookupInElementArg(arg.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            lookupInCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::DERIVED).clone(), paths.clone(), exactMatch.clone(), matches.clone())?
        },
        Deref @ Absyn::ClassDef::ENUMERATION { .. } => {
            matches = lookupInEnumDef(var_field!((*cdef).enumLiterals, Absyn::ClassDef::ENUMERATION).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            lookupInCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::ENUMERATION).clone(), paths.clone(), exactMatch.clone(), matches.clone())?
        },
        Deref @ Absyn::ClassDef::OVERLOAD { .. } => lookupInCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::OVERLOAD).clone(), paths.clone(), exactMatch.clone(), matches.clone())?,
        Deref @ Absyn::ClassDef::CLASS_EXTENDS { .. } => {
            assign_field!(local_paths.currentPath = cons((name.clone()).clone(), local_paths.currentPath.clone()));
            for mut arg in &*var_field!((*cdef).modifications, Absyn::ClassDef::CLASS_EXTENDS).clone() {
                let mut arg = arg.clone();
                matches = lookupInElementArg(arg.clone(), local_paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            for mut part in &*var_field!((*cdef).parts, Absyn::ClassDef::CLASS_EXTENDS).clone() {
                let mut part = part.clone();
                matches = lookupInClassPart(part.clone(), local_paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            for mut ann in &*var_field!((*cdef).ann, Absyn::ClassDef::CLASS_EXTENDS).clone() {
                let mut ann = ann.clone();
                matches = lookupInAnnotation(ann.clone(), local_paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassDef::PDER { .. } => {
            matches = matchPath(var_field!((*cdef).functionName, Absyn::ClassDef::PDER).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            lookupInCommentOpt(var_field!((*cdef).comment, Absyn::ClassDef::PDER).clone(), paths.clone(), exactMatch.clone(), matches.clone())?
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInClassPart(mut part: Arc<Absyn::ClassPart>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(part.clone()) {
        Deref @ Absyn::ClassPart::PUBLIC { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PUBLIC).clone() {
                let mut e = e.clone();
                matches = lookupInElementItem(e.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassPart::PROTECTED { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::PROTECTED).clone() {
                let mut e = e.clone();
                matches = lookupInElementItem(e.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassPart::EQUATIONS { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::EQUATIONS).clone() {
                let mut e = e.clone();
                matches = lookupInEquationItem(e.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassPart::INITIALEQUATIONS { .. } => {
            for mut e in &*var_field!((*part).contents, Absyn::ClassPart::INITIALEQUATIONS).clone() {
                let mut e = e.clone();
                matches = lookupInEquationItem(e.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassPart::ALGORITHMS { .. } => {
            for mut alg in &*var_field!((*part).contents, Absyn::ClassPart::ALGORITHMS).clone() {
                let mut alg = alg.clone();
                matches = lookupInAlgorithmItem(alg.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassPart::INITIALALGORITHMS { .. } => {
            for mut alg in &*var_field!((*part).contents, Absyn::ClassPart::INITIALALGORITHMS).clone() {
                let mut alg = alg.clone();
                matches = lookupInAlgorithmItem(alg.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ClassPart::EXTERNAL { .. } => {
            matches = lookupInExternalDecl(var_field!((*part).externalDecl, Absyn::ClassPart::EXTERNAL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            if isSome(var_field!((*part).annotation_, Absyn::ClassPart::EXTERNAL).clone()) {
                matches = lookupInAnnotation(Util::getOption(var_field!((*part).annotation_, Absyn::ClassPart::EXTERNAL).clone())?, paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInEnumDef(mut enumDef: Arc<Absyn::EnumDef>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(enumDef.clone()) {
        Deref @ Absyn::EnumDef::ENUMLITERALS { .. } => {
            for mut lit in &*var_field!((*enumDef).enumLiterals, Absyn::EnumDef::ENUMLITERALS).clone() {
                let mut lit = lit.clone();
                matches = lookupInCommentOpt(lit.comment.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInCommentOpt(mut cmt: Option<Arc<Absyn::Comment>>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    if isSome(cmt.clone()) {
        matches = lookupInComment(Util::getOption(cmt.clone())?, paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInComment(mut cmt: Arc<Absyn::Comment>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    if isSome(cmt.annotation_.clone()) {
        matches = lookupInAnnotation(Util::getOption(cmt.annotation_.clone())?, paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInAnnotation(mut ann: Arc<Absyn::Annotation>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut arg in &*ann.elementArgs.clone() {
        let mut arg = arg.clone();
        matches = lookupInElementArg(arg.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInElementArg(mut arg: Arc<Absyn::ElementArg>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(arg.clone()) {
        Deref @ Absyn::ElementArg::MODIFICATION { .. } => {
            if isSome(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone()) {
                matches = lookupInModification(Util::getOption(var_field!((*arg).modification, Absyn::ElementArg::MODIFICATION).clone())?, paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ElementArg::REDECLARATION { .. } => {
            matches = lookupInElementSpec(var_field!((*arg).elementSpec, Absyn::ElementArg::REDECLARATION).clone(), paths.clone(), exactMatch.clone(), var_field!((*arg).info, Absyn::ElementArg::REDECLARATION).clone(), matches.clone())?;
            if isSome(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone()) {
                matches = lookupInConstrainClass(Util::getOption(var_field!((*arg).constrainClass, Absyn::ElementArg::REDECLARATION).clone())?, paths.clone(), exactMatch.clone(), var_field!((*arg).info, Absyn::ElementArg::REDECLARATION).clone(), matches.clone())?;
            }
            matches.clone()
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInModification(mut r#mod: Arc<Absyn::Modification>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut arg in &*r#mod.elementArgLst.clone() {
        let mut arg = arg.clone();
        matches = lookupInElementArg(arg.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    matches = lookupInEqMod(r#mod.eqMod.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
    Ok(matches)
}

fn lookupInEqMod(mut eqMod: Arc<Absyn::EqMod>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(eqMod.clone()) {
        Deref @ Absyn::EqMod::EQMOD { .. } => lookupInExp(var_field!((*eqMod).exp, Absyn::EqMod::EQMOD).clone(), paths.clone(), exactMatch.clone(), var_field!((*eqMod).info, Absyn::EqMod::EQMOD).clone(), matches.clone())?,
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInExp(mut exp: Arc<Absyn::Exp>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Absyn::Exp::CREF { .. } => matchCref(var_field!((*exp).componentRef, Absyn::Exp::CREF).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Deref @ Absyn::Exp::BINARY { .. } => {
            matches = lookupInExp(var_field!((*exp).exp1, Absyn::Exp::BINARY).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            lookupInExp(var_field!((*exp).exp2, Absyn::Exp::BINARY).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?
        },
        Deref @ Absyn::Exp::UNARY { .. } => lookupInExp(var_field!((*exp).exp, Absyn::Exp::UNARY).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Deref @ Absyn::Exp::LBINARY { .. } => {
            matches = lookupInExp(var_field!((*exp).exp1, Absyn::Exp::LBINARY).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            lookupInExp(var_field!((*exp).exp2, Absyn::Exp::LBINARY).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?
        },
        Deref @ Absyn::Exp::LUNARY { .. } => lookupInExp(var_field!((*exp).exp, Absyn::Exp::LUNARY).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Deref @ Absyn::Exp::IFEXP { .. } => {
            matches = lookupInExp(var_field!((*exp).ifExp, Absyn::Exp::IFEXP).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInExp(var_field!((*exp).trueBranch, Absyn::Exp::IFEXP).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInExp(var_field!((*exp).elseBranch, Absyn::Exp::IFEXP).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            for mut branch in &*var_field!((*exp).elseIfBranch, Absyn::Exp::IFEXP).clone() {
                let mut branch = branch.clone();
                matches = lookupInExp(Util::tuple21(branch.clone()), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
                matches = lookupInExp(Util::tuple22(branch.clone()), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::Exp::CALL { .. } => {
            matches = matchCref(var_field!((*exp).function_, Absyn::Exp::CALL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            lookupInFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::CALL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { .. } => {
            matches = matchCref(var_field!((*exp).function_, Absyn::Exp::PARTEVALFUNCTION).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            lookupInFunctionArgs(var_field!((*exp).functionArgs, Absyn::Exp::PARTEVALFUNCTION).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?
        },
        Deref @ Absyn::Exp::ARRAY { .. } => {
            for mut e in &*var_field!((*exp).arrayExp, Absyn::Exp::ARRAY).clone() {
                let mut e = e.clone();
                matches = lookupInExp(e.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::Exp::MATRIX { .. } => {
            for mut row in &*var_field!((*exp).matrix, Absyn::Exp::MATRIX).clone() {
                let mut row = row.clone();
                for mut e in &*row.clone() {
                    let mut e = e.clone();
                    matches = lookupInExp(e.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
                }
            }
            matches.clone()
        },
        Deref @ Absyn::Exp::RANGE { .. } => {
            matches = lookupInExp(var_field!((*exp).start, Absyn::Exp::RANGE).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            if isSome(var_field!((*exp).step, Absyn::Exp::RANGE).clone()) {
                matches = lookupInExp(Util::getOption(var_field!((*exp).step, Absyn::Exp::RANGE).clone())?, paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            lookupInExp(var_field!((*exp).stop, Absyn::Exp::RANGE).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?
        },
        Deref @ Absyn::Exp::TUPLE { .. } => {
            for mut e in &*var_field!((*exp).expressions, Absyn::Exp::TUPLE).clone() {
                let mut e = e.clone();
                matches = lookupInExp(e.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { .. } => lookupInExp(var_field!((*exp).exp, Absyn::Exp::EXPRESSIONCOMMENT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Deref @ Absyn::Exp::SUBSCRIPTED_EXP { .. } => {
            matches = lookupInExp(var_field!((*exp).exp, Absyn::Exp::SUBSCRIPTED_EXP).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            lookupInSubscripts(var_field!((*exp).subscripts, Absyn::Exp::SUBSCRIPTED_EXP).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInCref(mut cref: Arc<Absyn::ComponentRef>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = matchCref(cref.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    matches = lookupInCrefSubs(cref.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    Ok(matches)
}

fn lookupInCrefSubs(mut cref: Arc<Absyn::ComponentRef>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ Absyn::ComponentRef::CREF_IDENT { .. } => lookupInSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_IDENT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Deref @ Absyn::ComponentRef::CREF_QUAL { .. } => {
            matches = lookupInSubscripts(var_field!((*cref).subscripts, Absyn::ComponentRef::CREF_QUAL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            lookupInCrefSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_QUAL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?
        },
        Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } => lookupInCrefSubs(var_field!((*cref).componentRef, Absyn::ComponentRef::CREF_FULLYQUALIFIED).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInSubscripts(mut subs: Arc<metamodelica::List<Arc<Absyn::Subscript>>>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut sub in &*subs.clone() {
        let mut sub = sub.clone();
        matches = lookupInSubscript(sub.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInSubscript(mut sub: Arc<Absyn::Subscript>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(sub.clone()) {
        Deref @ Absyn::Subscript::SUBSCRIPT { .. } => lookupInExp(var_field!((*sub).subscript, Absyn::Subscript::SUBSCRIPT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInFunctionArgs(mut args: Arc<Absyn::FunctionArgs>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(args.clone()) {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { .. } => {
            for mut arg in &*var_field!((*args).args, Absyn::FunctionArgs::FUNCTIONARGS).clone() {
                let mut arg = arg.clone();
                matches = lookupInExp(arg.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            for mut named_arg in &*var_field!((*args).argNames, Absyn::FunctionArgs::FUNCTIONARGS).clone() {
                let mut named_arg = named_arg.clone();
                matches = lookupInExp(named_arg.argValue.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::FunctionArgs::FOR_ITER_FARG { .. } => {
            matches = lookupInExp(var_field!((*args).exp, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInForIterators(var_field!((*args).iterators, Absyn::FunctionArgs::FOR_ITER_FARG).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(matches)
}

fn lookupInForIterators(mut iterators: Arc<metamodelica::List<Arc<Absyn::ForIterator>>>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut i in &*iterators.clone() {
        let mut i = i.clone();
        if isSome(i.range.clone()) {
            matches = lookupInExp(Util::getOption(i.range.clone())?, paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
        }
    }
    Ok(matches)
}

fn lookupInElementItem(mut item: Arc<Absyn::ElementItem>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::ElementItem::ELEMENTITEM { .. } => lookupInElement(var_field!((*item).element, Absyn::ElementItem::ELEMENTITEM).clone(), paths.clone(), exactMatch.clone(), matches.clone())?,
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInElement(mut element: Arc<Absyn::Element>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ Absyn::Element::ELEMENT { .. } => {
            matches = lookupInElementSpec(var_field!((*element).specification, Absyn::Element::ELEMENT).clone(), paths.clone(), exactMatch.clone(), var_field!((*element).info, Absyn::Element::ELEMENT).clone(), matches.clone())?;
            if isSome(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone()) {
                matches = lookupInConstrainClass(Util::getOption(var_field!((*element).constrainClass, Absyn::Element::ELEMENT).clone())?, paths.clone(), exactMatch.clone(), var_field!((*element).info, Absyn::Element::ELEMENT).clone(), matches.clone())?;
            }
            matches.clone()
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInElementSpec(mut spec: Arc<Absyn::ElementSpec>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(spec.clone()) {
        Deref @ Absyn::ElementSpec::CLASSDEF { .. } => lookupInClass(var_field!((*spec).class_, Absyn::ElementSpec::CLASSDEF).clone(), paths.clone(), exactMatch.clone(), matches.clone())?,
        Deref @ Absyn::ElementSpec::EXTENDS { .. } => {
            matches = matchPath(var_field!((*spec).path, Absyn::ElementSpec::EXTENDS).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            for mut arg in &*var_field!((*spec).elementArg, Absyn::ElementSpec::EXTENDS).clone() {
                let mut arg = arg.clone();
                matches = lookupInElementArg(arg.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            if isSome(var_field!((*spec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone()) {
                matches = lookupInAnnotation(Util::getOption(var_field!((*spec).annotationOpt, Absyn::ElementSpec::EXTENDS).clone())?, paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches.clone()
        },
        Deref @ Absyn::ElementSpec::IMPORT { .. } => {
            matches = lookupInImport(var_field!((*spec).import_, Absyn::ElementSpec::IMPORT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches.clone()
        },
        Deref @ Absyn::ElementSpec::COMPONENTS { .. } => {
            matches = lookupInTypeSpec(var_field!((*spec).typeSpec, Absyn::ElementSpec::COMPONENTS).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            for mut c in &*var_field!((*spec).components, Absyn::ElementSpec::COMPONENTS).clone() {
                let mut c = c.clone();
                matches = lookupInComponentItem(c.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            matches.clone()
        },
        _ => bail!("match: no arm matched"),
    } });
    Ok(matches)
}

fn lookupInConstrainClass(mut constrainClass: Arc<Absyn::ConstrainClass>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = lookupInElementSpec(constrainClass.elementSpec.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    Ok(matches)
}

fn lookupInImport(mut imp: Absyn::Import, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (match imp.clone() {
        Absyn::Import::NAMED_IMPORT { .. } => matchPath(var_field!(imp.path, Absyn::Import::NAMED_IMPORT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Absyn::Import::QUAL_IMPORT { .. } => matchPath(var_field!(imp.path, Absyn::Import::QUAL_IMPORT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Absyn::Import::UNQUAL_IMPORT { .. } => matchPath(var_field!(imp.path, Absyn::Import::UNQUAL_IMPORT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        Absyn::Import::GROUP_IMPORT { .. } => matchPath(var_field!(imp.prefix, Absyn::Import::GROUP_IMPORT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?,
        _ => bail!("match: no arm matched"),
    });
    Ok(matches)
}

fn lookupInComponentItem(mut item: Arc<Absyn::ComponentItem>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = lookupInComponent(item.component.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    if isSome(item.condition.clone()) {
        matches = lookupInExp(Util::getOption(item.condition.clone())?, paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInComponent(mut component: Absyn::Component, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = lookupInSubscripts(component.arrayDim.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    if isSome(component.modification.clone()) {
        matches = lookupInModification(Util::getOption(component.modification.clone())?, paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInTypeSpec(mut typeSpec: Arc<Absyn::TypeSpec>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(typeSpec.clone()) {
        Deref @ Absyn::TypeSpec::TPATH { .. } => {
            matches = matchPath(var_field!((*typeSpec).path, Absyn::TypeSpec::TPATH).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            if isSome(var_field!((*typeSpec).arrayDim, Absyn::TypeSpec::TPATH).clone()) {
                matches = lookupInSubscripts(Util::getOption(var_field!((*typeSpec).arrayDim, Absyn::TypeSpec::TPATH).clone())?, paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            }
            matches.clone()
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInEquationItems(mut items: Arc<metamodelica::List<Arc<Absyn::EquationItem>>>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut item in &*items.clone() {
        let mut item = item.clone();
        matches = lookupInEquationItem(item.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInEquationItem(mut item: Arc<Absyn::EquationItem>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::EquationItem::EQUATIONITEM { .. } => {
            matches = lookupInEquation(var_field!((*item).equation_, Absyn::EquationItem::EQUATIONITEM).clone(), paths.clone(), exactMatch.clone(), var_field!((*item).info, Absyn::EquationItem::EQUATIONITEM).clone(), matches.clone())?;
            lookupInCommentOpt(var_field!((*item).comment, Absyn::EquationItem::EQUATIONITEM).clone(), paths.clone(), exactMatch.clone(), matches.clone())?
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInEquation(mut eq: Arc<Absyn::Equation>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    let () = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Absyn::Equation::EQ_IF { .. } => {
            matches = lookupInExp(var_field!((*eq).ifExp, Absyn::Equation::EQ_IF).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInEquationItems(var_field!((*eq).equationTrueItems, Absyn::Equation::EQ_IF).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            for mut branch in &*var_field!((*eq).elseIfBranches, Absyn::Equation::EQ_IF).clone() {
                let mut branch = branch.clone();
                matches = lookupInExp(Util::tuple21(branch.clone()), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
                matches = lookupInEquationItems(Util::tuple22(branch.clone()), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches = lookupInEquationItems(var_field!((*eq).equationElseItems, Absyn::Equation::EQ_IF).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_EQUALS { .. } => {
            matches = lookupInExp(var_field!((*eq).leftSide, Absyn::Equation::EQ_EQUALS).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_EQUALS).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_PDE { .. } => {
            matches = lookupInExp(var_field!((*eq).leftSide, Absyn::Equation::EQ_PDE).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInExp(var_field!((*eq).rightSide, Absyn::Equation::EQ_PDE).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_CONNECT { .. } => {
            matches = lookupInCref(var_field!((*eq).connector1, Absyn::Equation::EQ_CONNECT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInCref(var_field!((*eq).connector2, Absyn::Equation::EQ_CONNECT).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_FOR { .. } => {
            matches = lookupInForIterators(var_field!((*eq).iterators, Absyn::Equation::EQ_FOR).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInEquationItems(var_field!((*eq).forEquations, Absyn::Equation::EQ_FOR).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Equation::EQ_WHEN_E { .. } => {
            matches = lookupInExp(var_field!((*eq).whenExp, Absyn::Equation::EQ_WHEN_E).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInEquationItems(var_field!((*eq).whenEquations, Absyn::Equation::EQ_WHEN_E).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            for mut branch in &*var_field!((*eq).elseWhenEquations, Absyn::Equation::EQ_WHEN_E).clone() {
                let mut branch = branch.clone();
                matches = lookupInExp(Util::tuple21(branch.clone()), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
                matches = lookupInEquationItems(Util::tuple22(branch.clone()), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            ()
        },
        Deref @ Absyn::Equation::EQ_NORETCALL { .. } => {
            matches = lookupInCref(var_field!((*eq).functionName, Absyn::Equation::EQ_NORETCALL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInFunctionArgs(var_field!((*eq).functionArgs, Absyn::Equation::EQ_NORETCALL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInAlgorithmItems(mut items: Arc<metamodelica::List<Arc<Absyn::AlgorithmItem>>>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut item in &*items.clone() {
        let mut item = item.clone();
        matches = lookupInAlgorithmItem(item.clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn lookupInAlgorithmItem(mut item: Arc<Absyn::AlgorithmItem>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    matches = (::match_deref::match_deref! { match &(item.clone()) {
        Deref @ Absyn::AlgorithmItem::ALGORITHMITEM { .. } => {
            matches = lookupInAlgorithm(var_field!((*item).algorithm_, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), paths.clone(), exactMatch.clone(), var_field!((*item).info, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), matches.clone())?;
            matches = lookupInCommentOpt(var_field!((*item).comment, Absyn::AlgorithmItem::ALGORITHMITEM).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            matches.clone()
        },
        _ => matches.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInAlgorithm(mut alg: Arc<Absyn::Algorithm>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    let () = (::match_deref::match_deref! { match &(alg.clone()) {
        Deref @ Absyn::Algorithm::ALG_ASSIGN { .. } => {
            matches = lookupInExp(var_field!((*alg).assignComponent, Absyn::Algorithm::ALG_ASSIGN).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInExp(var_field!((*alg).value, Absyn::Algorithm::ALG_ASSIGN).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_IF { .. } => {
            matches = lookupInExp(var_field!((*alg).ifExp, Absyn::Algorithm::ALG_IF).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInAlgorithmItems(var_field!((*alg).trueBranch, Absyn::Algorithm::ALG_IF).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            for mut branch in &*var_field!((*alg).elseIfAlgorithmBranch, Absyn::Algorithm::ALG_IF).clone() {
                let mut branch = branch.clone();
                matches = lookupInExp(Util::tuple21(branch.clone()), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
                matches = lookupInAlgorithmItems(Util::tuple22(branch.clone()), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            matches = lookupInAlgorithmItems(var_field!((*alg).elseBranch, Absyn::Algorithm::ALG_IF).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_FOR { .. } => {
            matches = lookupInForIterators(var_field!((*alg).iterators, Absyn::Algorithm::ALG_FOR).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInAlgorithmItems(var_field!((*alg).forBody, Absyn::Algorithm::ALG_FOR).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_PARFOR { .. } => {
            matches = lookupInForIterators(var_field!((*alg).iterators, Absyn::Algorithm::ALG_PARFOR).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInAlgorithmItems(var_field!((*alg).parforBody, Absyn::Algorithm::ALG_PARFOR).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHILE { .. } => {
            matches = lookupInExp(var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHILE).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInAlgorithmItems(var_field!((*alg).whileBody, Absyn::Algorithm::ALG_WHILE).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            ()
        },
        Deref @ Absyn::Algorithm::ALG_WHEN_A { .. } => {
            matches = lookupInExp(var_field!((*alg).boolExpr, Absyn::Algorithm::ALG_WHEN_A).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInAlgorithmItems(var_field!((*alg).whenBody, Absyn::Algorithm::ALG_WHEN_A).clone(), paths.clone(), exactMatch.clone(), matches.clone())?;
            for mut branch in &*var_field!((*alg).elseWhenAlgorithmBranch, Absyn::Algorithm::ALG_WHEN_A).clone() {
                let mut branch = branch.clone();
                matches = lookupInExp(Util::tuple21(branch.clone()), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
                matches = lookupInAlgorithmItems(Util::tuple22(branch.clone()), paths.clone(), exactMatch.clone(), matches.clone())?;
            }
            ()
        },
        Deref @ Absyn::Algorithm::ALG_NORETCALL { .. } => {
            matches = lookupInCref(var_field!((*alg).functionCall, Absyn::Algorithm::ALG_NORETCALL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            matches = lookupInFunctionArgs(var_field!((*alg).functionArgs, Absyn::Algorithm::ALG_NORETCALL).clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(matches)
}

fn lookupInExternalDecl(mut extDecl: Arc<Absyn::ExternalDecl>, mut paths: Arc<Paths::Paths>, mut exactMatch: bool, mut info: SourceInfo, mut matches: Matches) -> Result<Matches> {
    let mut matches: Matches = matches;
    for mut arg in &*extDecl.args.clone() {
        let mut arg = arg.clone();
        matches = lookupInExp(arg.clone(), paths.clone(), exactMatch.clone(), info.clone(), matches.clone())?;
    }
    if isSome(extDecl.annotation_.clone()) {
        matches = lookupInAnnotation(Util::getOption(extDecl.annotation_.clone())?, paths.clone(), exactMatch.clone(), matches.clone())?;
    }
    Ok(matches)
}

fn serializeMatches(mut groupedMatches: Arc<metamodelica::List<Arc<metamodelica::List<Match>>>>, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut r#str: ArcStr = arcstr::literal!("");
    let mut json_groups: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
    let mut json_elems: Arc<metamodelica::List<Arc<JSON::JSON>>> = metamodelica::nil();
    let mut json_group: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut json_elem: Arc<JSON::JSON> = Arc::new(JSON::FALSE);
    let mut first_match: Match = <Match as ::std::default::Default>::default();
    for mut group in &*groupedMatches.clone() {
        let mut group = group.clone();
        first_match = listHead(group.clone())?;
        json_group = JSON::addPair((literal!("filename")).clone(), JSON::makeString(first_match.info.fileName.clone()), JSON::emptyListObject())?;
        json_elems = metamodelica::nil();
        for mut m in &*group.clone() {
            let mut m = m.clone();
            json_elem = NFApi::dumpJSONSourceInfo(m.info.clone(), false)?;
            json_elem = JSON::addPair((literal!("name")).clone(), JSON::makeString((Dump::printComponentRefStr(m.name.clone())?).clone()), json_elem.clone())?;
            json_elem = JSON::addPair((literal!("class")).clone(), JSON::makeString((m.scope.clone()).clone()), json_elem.clone())?;
            json_elems = cons(json_elem.clone(), json_elems.clone());
        }
        json_group = JSON::addPair((literal!("matches")).clone(), JSON::makeArray(json_elems.clone()), json_group.clone())?;
        json_groups = cons(json_group.clone(), json_groups.clone());
    }
    r#str = (JSON::toString(JSON::makeArray(json_groups.clone()), prettyPrint.clone())?).clone();
    Ok(r#str)
}

fn groupMatches(mut matches: Matches) -> Result<Arc<metamodelica::List<Arc<metamodelica::List<Match>>>>> {
    fn add_match(mut oldMatches: Option<Arc<metamodelica::List<Match>>>, mut newMatch: Match) -> Result<Matches> {
        let mut outMatches: Matches = metamodelica::nil();
        if isSome(oldMatches.clone()) {
            let __pa0 = ::match_deref::match_deref! { match &(oldMatches.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => bail!("pattern mismatch"),
            } };
            outMatches = __pa0.clone();
            outMatches = cons(newMatch.clone(), outMatches.clone());
        } else {
            outMatches = list![newMatch.clone()];
        }
        Ok(outMatches)
    }

    let mut outMatches: Arc<metamodelica::List<Arc<metamodelica::List<Match>>>> = metamodelica::nil();
    let mut grouped_matches: Arc<UnorderedMap::UnorderedMap<ArcStr, Arc<metamodelica::List<Match>>>>;
    grouped_matches = UnorderedMap::new((std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>), (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>), 1);
    for mut m in &*matches.clone() {
        let mut m = m.clone();
        UnorderedMap::addUpdate(m.info.fileName.clone(), Arc::new({ let __pe_b1 = m.clone(); move |__pe_a0| add_match(__pe_a0, __pe_b1.clone()) }), grouped_matches.clone())?;
    }
    outMatches = UnorderedMap::valueList(grouped_matches.clone());
    outMatches = {
        let mut __acc: Arc<metamodelica::List<_>> = metamodelica::nil();
        for mut l in (outMatches.clone()).into_iter().cloned() {
            let __x = metamodelica::Dangerous::listReverseInPlace(l.clone());
            __acc = cons(__x, __acc);
        }
        __acc
    };
    Ok(outMatches)
}

